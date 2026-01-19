use f3d::Engine;

fn main() {
    let engine = Engine::new();
    engine.auto_load_plugins();

    let window = engine.window();
    window.set_title("Hello from Rust");
    
    let camera = engine.window().camera();
    let options = engine.options();

    let scene = engine.scene();
    scene.add("../f3d/testing/data/dragon.vtu");

    let interactor = engine.interactor();
    interactor.start(1.0 / 30.0);

}

