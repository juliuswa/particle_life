mod particle_sytem;
mod gui;
mod particle;
mod runtime;
mod tick_counter;

use particle_sytem::ParticleSystem;
use gui::Gui;
use nannou::prelude::*;
use nannou::rand::{rand, Rng};
use particle::Particle;
use runtime::Runtime;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    window: window::Id,
    gui: Gui,
    grid: ParticleSystem,
    runtime: Runtime
}


fn model(app: &App) -> Model {
    let window = app.new_window().view(view).build().unwrap();
    let gui = Gui::new();

    let mut particle_system = ParticleSystem::new(app.window_rect());
    particle_system.initialize_random();

    //particle_system.attraction = [[0.5, 0.5, -1.0],[0.0, 0.5, 0.5],[0.5, 0.0, 0.5]];

    Model { 
        window,
        gui,
        grid: particle_system,
        runtime: Runtime::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.grid.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgb(0.1, 0.1, 0.1));
    model.grid.draw(&app, &draw);

    draw.to_frame(app, &frame).unwrap();
}