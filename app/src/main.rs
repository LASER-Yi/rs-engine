use engine::RsEngine;
use std::process;

fn main() {
    let engine = RsEngine::new();

    if let Err(e) = engine.run() {
        eprintln!("The app exits with error: {}", e);
        process::exit(1);
    }
}
