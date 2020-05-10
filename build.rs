use flatc_rust;

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=schemas/*.fbs");

    flatc_rust::run(flatc_rust::Args {
        inputs: &[
            Path::new("schemas/render_commands.fbs"),
            Path::new("schemas/execution_commands.fbs"),
            Path::new("schemas/request_commands.fbs"),
            // Path::new("schemas/monster.fbs"),
        ],
        out_dir: Path::new("target/schemas/"),
        ..Default::default()
    })
    .expect("flatc");
}
