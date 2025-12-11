use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// Represents a UCI chess engine process.
pub struct UciEngine {
    process: Child,
    stdin: ChildStdin,
    receiver: Receiver<String>,
}

/// Errors that can occur when working with the UCI engine.
#[derive(Debug)]
pub enum EngineError {
    SpawnFailed(std::io::Error),
    WriteFailed(std::io::Error),
    EngineNotReady,
    EngineClosed,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::SpawnFailed(e) => write!(f, "Failed to spawn engine: {}", e),
            EngineError::WriteFailed(e) => write!(f, "Failed to write to engine: {}", e),
            EngineError::EngineNotReady => write!(f, "Engine is not ready"),
            EngineError::EngineClosed => write!(f, "Engine has closed"),
        }
    }
}

impl std::error::Error for EngineError {}

impl UciEngine {
    /// Spawns a new UCI engine process from the given executable path.
    pub fn new(engine_path: &str) -> Result<Self, EngineError> {
        let mut process = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(EngineError::SpawnFailed)?;

        let stdin = process.stdin.take().ok_or(EngineError::EngineNotReady)?;
        let stdout = process.stdout.take().ok_or(EngineError::EngineNotReady)?;

        let receiver = Self::spawn_reader_thread(stdout);

        Ok(Self {
            process,
            stdin,
            receiver,
        })
    }

    /// Spawns a background thread to read engine output lines.
    fn spawn_reader_thread(stdout: ChildStdout) -> Receiver<String> {
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(text) => {
                        if sender.send(text).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        receiver
    }

    /// Sends a raw command string to the engine (appends newline automatically).
    pub fn send_command(&mut self, command: &str) -> Result<(), EngineError> {
        writeln!(self.stdin, "{}", command).map_err(EngineError::WriteFailed)?;
        self.stdin.flush().map_err(EngineError::WriteFailed)?;
        Ok(())
    }

    /// Reads lines from the engine with a timeout.
    /// Returns all lines read, or an error if timeout is reached before the predicate matches.
    pub fn read_until_timeout<F>(
        &self,
        predicate: F,
        timeout: std::time::Duration,
    ) -> Result<Vec<String>, EngineError>
    where
        F: Fn(&str) -> bool,
    {
        let mut lines = Vec::new();
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(EngineError::EngineNotReady);
            }

            match self.receiver.recv_timeout(timeout - start.elapsed()) {
                Ok(line) => {
                    let matched = predicate(&line);
                    lines.push(line);
                    if matched {
                        return Ok(lines);
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    return Err(EngineError::EngineNotReady);
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(EngineError::EngineClosed);
                }
            }
        }
    }

    /// Sends the "quit" command and waits for the engine to exit.
    pub fn quit(&mut self) -> Result<(), EngineError> {
        let _ = self.send_command("quit");
        let _ = self.process.wait();
        Ok(())
    }

    /// Sends "uci" command and waits for "uciok" response.
    pub fn init_uci(&mut self) -> Result<Vec<String>, EngineError> {
        self.send_command("uci")?;
        self.read_until_timeout(|line| line == "uciok", std::time::Duration::from_secs(5))
    }

    /// Sends "isready" command and waits for "readyok" response.
    pub fn wait_ready(&mut self) -> Result<(), EngineError> {
        self.send_command("isready")?;
        self.read_until_timeout(|line| line == "readyok", std::time::Duration::from_secs(5))?;
        Ok(())
    }

    /// Sends "ucinewgame" command to reset the engine for a new game.
    pub fn new_game(&mut self) -> Result<(), EngineError> {
        self.send_command("ucinewgame")?;
        self.wait_ready()
    }

    /// Sends a setoption command to configure the engine.
    pub fn set_option(&mut self, name: &str, value: &str) -> Result<(), EngineError> {
        let cmd = format!("setoption name {} value {}", name, value);
        self.send_command(&cmd)
    }

    /// Sends the position command starting from initial position with moves.
    pub fn set_position_startpos(&mut self, moves: &str) -> Result<(), EngineError> {
        let cmd = if moves.is_empty() {
            "position startpos".to_string()
        } else {
            format!("position startpos moves {}", moves)
        };
        self.send_command(&cmd)
    }

    /// Sends "go" command and waits for "bestmove" response.
    /// Returns the best move in UCI format (e.g., "e2e4").
    pub fn go(
        &mut self,
        depth: Option<u32>,
        movetime_ms: Option<u64>,
    ) -> Result<String, EngineError> {
        let mut cmd = "go".to_string();
        if let Some(d) = depth {
            cmd.push_str(&format!(" depth {}", d));
        }
        if let Some(t) = movetime_ms {
            cmd.push_str(&format!(" movetime {}", t));
        }
        self.send_command(&cmd)?;

        let lines = self.read_until_timeout(
            |line| line.starts_with("bestmove"),
            std::time::Duration::from_secs(30),
        )?;

        for line in lines.iter().rev() {
            if line.starts_with("bestmove") {
                return Self::parse_bestmove(line);
            }
        }

        Err(EngineError::EngineNotReady)
    }

    /// Parses the bestmove line and extracts the move.
    fn parse_bestmove(line: &str) -> Result<String, EngineError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "bestmove" {
            Ok(parts[1].to_string())
        } else {
            Err(EngineError::EngineNotReady)
        }
    }
}

/// Request sent to the engine thread.
pub struct MoveRequest {
    pub moves_uci: String,
    pub depth: Option<u32>,
    pub movetime_ms: Option<u64>,
    pub elo: Option<u32>,
}

/// A handle to communicate with a running engine in a background thread.
pub struct EngineHandle {
    request_sender: Sender<MoveRequest>,
    result_receiver: Receiver<Result<String, EngineError>>,
}

impl EngineHandle {
    /// Spawns a new engine in a background thread.
    /// The engine is initialized (uci, isready) before this function returns.
    pub fn new(engine_path: &str) -> Result<Self, EngineError> {
        let (request_sender, request_receiver) = mpsc::channel::<MoveRequest>();
        let (result_sender, result_receiver) = mpsc::channel::<Result<String, EngineError>>();

        let path = engine_path.to_string();

        let (init_sender, init_receiver) = mpsc::channel::<Result<(), EngineError>>();

        thread::spawn(move || {
            let engine_result = (|| {
                let mut engine = UciEngine::new(&path)?;
                engine.init_uci()?;
                engine.wait_ready()?;
                Ok::<UciEngine, EngineError>(engine)
            })();

            let mut engine = match engine_result {
                Ok(e) => {
                    let _ = init_sender.send(Ok(()));
                    e
                }
                Err(e) => {
                    let _ = init_sender.send(Err(e));
                    return;
                }
            };

            while let Ok(request) = request_receiver.recv() {
                let result = (|| {
                    engine.new_game()?;
                    if let Some(elo) = request.elo {
                        engine.set_option("UCI_LimitStrength", "true")?;
                        engine.set_option("UCI_Elo", &elo.to_string())?;
                    } else {
                        engine.set_option("UCI_LimitStrength", "false")?;
                    }
                    engine.set_position_startpos(&request.moves_uci)?;
                    engine.go(request.depth, request.movetime_ms)
                })();
                if result_sender.send(result).is_err() {
                    break;
                }
            }
        });

        init_receiver
            .recv()
            .map_err(|_| EngineError::EngineClosed)??;

        Ok(Self {
            request_sender,
            result_receiver,
        })
    }

    /// Sends a move request to the engine (non-blocking).
    pub fn request_move(
        &self,
        moves_uci: String,
        depth: Option<u32>,
        movetime_ms: Option<u64>,
        elo: Option<u32>,
    ) {
        let _ = self.request_sender.send(MoveRequest {
            moves_uci,
            depth,
            movetime_ms,
            elo,
        });
    }

    /// Tries to receive a move result (non-blocking).
    /// Returns None if no result is available yet.
    pub fn try_recv_move(&self) -> Option<Result<String, EngineError>> {
        self.result_receiver.try_recv().ok()
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        let _ = self.quit();
    }
}
