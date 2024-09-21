mod gui;
mod particle_sytem;
mod runtime;

use ggez::event::{self, EventHandler};
use ggez::{Context, ContextBuilder, GameResult};
use gui::Gui;
use particle_sytem::ParticleSystem;
use runtime::Runtime;

fn main() -> GameResult {
    let window_mode = ggez::conf::WindowMode::default()
        .dimensions(1920.0, 1080.0);

    let (ctx, event_loop) = ContextBuilder::new("particle life", "julius wachlin")
        .window_mode(window_mode)
        .build()
        .expect("Failed to build ggez context");

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}


struct MainState {
    gui: Gui,
    particle_system: ParticleSystem,
    runtime: Runtime,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut particle_system = ParticleSystem::new();
        particle_system.initialize_random();

        /*
        particle_system.attraction = [
            [1.0, 0.5, 0.0, -0.3],
            [-0.3, 1.0, 0.5, 0.0],
            [0.0, -0.3, 1.0, 0.5],
            [0.5, 0.0, -0.3, 1.0]
            ];
        
        particle_system.attraction = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
            ];
        */

        Ok(MainState {
            gui: Gui::new(),
            particle_system,
            runtime: Runtime::new(),
        })
    }
}

impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.runtime.start("update");

        self.particle_system.update();
        self.runtime.cleanup();

        self.runtime.stop();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //self.runtime.start("draw");
        self.gui.draw(ctx, &self.particle_system, &mut self.runtime)?;

        //self.runtime.stop();
        Ok(())
    }
}
