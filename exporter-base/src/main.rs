use std::{env, path::PathBuf};
mod exporter;
mod video_scene;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: exporter-base <output_path> [fps]");
        std::process::exit(1);
    }

    let output = PathBuf::from(&args[1]);
    let fps: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30);

    let mut scene = video_scene::build();

    exporter::export_scene(&mut scene, output, fps, true).unwrap_or_else(|e| {
        eprintln!("Export failed: {e}");
        std::process::exit(1);
    });
}
