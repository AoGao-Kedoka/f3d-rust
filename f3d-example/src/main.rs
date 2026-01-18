use f3d::{Engine, Options, Scene, Interactor, Window};

fn main() {
    let engine = Engine::new();
    let window = engine.window();
    window.set_title("Hello from rust");

    let options = engine.options();
    options.set_bool("render.effect.display_depth", true);
    options.set_bool("model.scivis.enable", true);

    let scene = engine.scene();
    scene.add("../f3d/testing/data/dragon.vtu");

    let interactor = engine.interactor();
    interactor.start(1.0 / 30.0);
}

