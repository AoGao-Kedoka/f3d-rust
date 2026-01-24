use std::env;
use f3d::engine::{Engine, EngineBackend, EngineContext};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let mesh_path = &args[1];

    let engine = Engine::new(
        EngineBackend::Auto { offscreen: false },
        EngineContext::Internal,
    );
    Engine::auto_load_plugins();

    let window = engine.window();
    window.set_title("Hello from Rust");

    let options = engine.options();
    options.set_bool("render.grid.enable", true);

    let scene = engine.scene();
    scene.add(mesh_path).unwrap();

    let interactor = engine.interactor();
    interactor.start(1.0 / 30.0);
}
