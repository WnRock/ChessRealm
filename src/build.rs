extern crate embed_resource;

fn main() {
    embed_resource::compile("assets/windows_icon.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
