use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, InstanceArray, Mesh};
use ggez::{Context, GameError};
use palette::{FromColor, Hsv, Srgb};

use crate::particle_sytem::{ParticleSystem, HEIGHT, COLORS};
use crate::runtime::{Runtime, WINDOW_SEC};

pub struct Gui {
    hue_rgb_map: [Color; 360],
}

impl Gui {
    pub fn new() -> Self {
        Self {
            hue_rgb_map: construct_hue_rgb_map(),
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        particle_system: &ParticleSystem,
        runtime: &mut Runtime,
    ) -> Result<(), GameError> {

        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        runtime.start("draw_particles");
        self.draw_particles(ctx, &mut canvas, particle_system)?;
        runtime.stop();

        runtime.start("draw_runtime");
        self.draw_runtime(&mut canvas, runtime);
        runtime.stop();

        runtime.start("present");
        canvas.finish(ctx)?;
        runtime.stop();

        Ok(())
    }

    fn draw_particles(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        particle_system: &ParticleSystem
    ) -> Result<(), GameError> {
        let (width, height) = ctx.gfx.size();
        let scaling = height / HEIGHT;

        let particle_mesh = Mesh::new_circle(ctx, DrawMode::fill(), [0.0, 0.0], 3.0, 0.1, Color::WHITE)?;        
        let mut instances = InstanceArray::new(ctx, None);

        //runtime.start("draw");
        for i in 0..particle_system.x_pos.len() {
            let x = particle_system.x_pos[i] * scaling + (width  / 2.0);
            let y = particle_system.y_pos[i] * scaling + (height / 2.0);
            let color = self.get_color_by_ratio(particle_system.color[i] as f32, COLORS as f32);

            let draw_params = graphics::DrawParam::default().dest([x, y]).color(color);

            instances.push(draw_params);
        }
        //runtime.stop();

        canvas.draw_instanced_mesh(particle_mesh, &instances, DrawParam::default());

        Ok(())
    }

    fn draw_runtime(&self, canvas: &mut Canvas, runtime: &mut Runtime){
        let mut pos = (10.0, 10.0);

        let analysis = runtime.get_analysis();

        let performance_string: String = format!("Performance Statistics ({WINDOW_SEC} sec):");
        draw_text(canvas, &performance_string, pos);
        pos = (pos.0, pos.1 + 40.0);

        for i in 0..analysis.names.len() {
            let analysis_string = format!(
                "|{}| cnt: {} ttl: {}ms av: {}ms",
                analysis.names[i],
                analysis.counts[i],
                analysis.totals[i],
                analysis.averages[i].round()
            );

            draw_text(canvas, &analysis_string, pos);
            pos = (pos.0, pos.1 + 40.0);
        }
    }

    fn get_color_by_ratio(&self, nom: f32, den: f32) -> Color {
        self.hue_rgb_map[(360.0 * nom / den) as usize]
    }
}

fn draw_text(canvas: &mut Canvas, text: &str, pos: (f32, f32)) {
    let text: graphics::Text = graphics::Text::new(text);

    let draw_params = graphics::DrawParam::default()
        .scale([2.0, 2.0])
        .dest([pos.0, pos.1]);

    canvas.draw(&text, draw_params)
}

fn construct_hue_rgb_map() -> [Color; 360] {
    let mut map = [Color::new(0.0, 0.0, 0.0, 0.0); 360];

    for (i, color) in map.iter_mut().enumerate(){
        let hsv_color = Hsv::new(i as f32, 1.0, 1.0);
        let rgb_color = Srgb::from_color(hsv_color);
        color.r = rgb_color.red;
        color.g = rgb_color.green;
        color.b = rgb_color.blue;
        color.a = 1.0;
    }

    map
}
