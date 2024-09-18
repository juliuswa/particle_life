use nannou::prelude::*;
use nannou::rand::{rand, Rng};

use crate::tick_counter::TickCounter;

const PARTICLES: usize = 1600;
const COLORS: usize = 10;
const PARTITIONS: usize = 4;

const NEUTRAL_R: f32 = 40.0;
const MAX_R: f32 = 160.0;

const FRICTION: f32 = 0.8;
const FORCE_FACTOR: f32 = 0.5;

pub struct ParticleSystem {
    x_range: f32,
    x_min: f32,
    x_max: f32,
    y_range: f32,
    y_min: f32,
    y_max: f32,

    x_pos: [f32; PARTICLES],
    y_pos: [f32; PARTICLES],
    x_mov: [f32; PARTICLES],
    y_mov: [f32; PARTICLES],
    color: [usize; PARTICLES],

    pub attraction: [[f32; COLORS]; COLORS],

    partitions: Vec<Vec<Vec<usize>>>,
    part_w: f32,
    part_h: f32,

    tick_counter: TickCounter,
}

impl ParticleSystem {
    pub fn new(range: Rect) -> Self {
        let part_w = (range.right() - range.left()) / PARTITIONS as f32;
        let part_h = (range.top() - range.bottom()) / PARTITIONS as f32;

        if part_w < MAX_R || part_h < MAX_R {
            println!("WARNING: MAX_R is {} and partition size is ({}, {})", MAX_R, part_w, part_h)
        }

        Self {
            x_range: range.right() - range.left(),
            x_min: range.left(),
            x_max: range.right(),
            y_range: range.top() - range.bottom(),
            y_min: range.bottom(),
            y_max: range.top(),

            x_pos: [0.0; PARTICLES],
            y_pos: [0.0; PARTICLES],
            x_mov: [0.0; PARTICLES],
            y_mov: [0.0; PARTICLES],
            color: [0; PARTICLES],

            attraction: [[0.0; COLORS]; COLORS],

            partitions: vec![vec![vec![]; PARTITIONS]; PARTITIONS],
            part_w: part_w,
            part_h: part_h,

            tick_counter: TickCounter::new(),
        }
    }

    pub fn initialize_random(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..PARTICLES {
            self.x_pos[i] = rng.gen_range(self.x_min..self.x_max);
            self.y_pos[i] = rng.gen_range(self.y_min..self.y_max);

            self.x_mov[i] = 0.0;
            self.y_mov[i] = 0.0;

            self.color[i] = rng.gen_range(0..COLORS);

            let part = self.get_partition(self.x_pos[i], self.y_pos[i]);
            self.partitions[part.0][part.1].push(i);
        }

        for i in 0..COLORS {
            for j in 0..COLORS {
                self.attraction[i][j] = rng.gen_range(-1.0..1.0);
            }
        }
    }

    fn get_partition(&self, x: f32, y: f32) -> (usize, usize) {
        let x_part = ((x - self.x_min) / self.part_w).floor() as usize;
        let y_part = ((y - self.y_min) / self.part_h).floor() as usize;

        (x_part, y_part)        
    }

    fn get_neighbor_particles(&self, x: usize, y: usize) -> Vec<usize> {
        let mut particles = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                let part_x = loop_to_int((x + i) as i32 - 1, 0, PARTITIONS as i32) as usize;
                let part_y = loop_to_int((y + j) as i32 - 1, 0, PARTITIONS as i32) as usize;

                particles.extend(self.partitions[part_x][part_y].iter());
            }
        }

        particles
    }

    pub fn update(&mut self) {        
        let half_x_range = self.x_range / 2.0;
        let half_y_range = self.y_range / 2.0;

        let mut x_moves = [0.0; PARTICLES];
        let mut y_moves = [0.0; PARTICLES];

        for i in 0..PARTICLES {
            let part = self.get_partition(self.x_pos[i], self.y_pos[i]);
            let neighbors = self.get_neighbor_particles(part.0, part.1);

            let mut move_x = 0.0;
            let mut move_y = 0.0;

            for n in neighbors {
                if i == n {
                    continue;
                }
                
                let x_dist = loop_float(self.x_pos[n] - self.x_pos[i], -half_x_range, half_x_range);
                let y_dist = loop_float(self.y_pos[n] - self.y_pos[i], -half_y_range, half_y_range);
                let dist = (x_dist * x_dist + y_dist * y_dist).sqrt();

                let attr = self.attraction[self.color[i]][self.color[n]];
                let force = get_force(dist, attr);           

                move_x += x_dist * force / dist;
                move_y += y_dist * force / dist;
            }

            x_moves[i] = self.x_mov[i] * (1.0 - FRICTION) + move_x * FORCE_FACTOR; 
            y_moves[i] = self.y_mov[i] * (1.0 - FRICTION) + move_y * FORCE_FACTOR; 
        }

        for i in 0..PARTICLES {
            let part = self.get_partition(self.x_pos[i], self.y_pos[i]);

            let new_x = loop_float(self.x_pos[i] + x_moves[i], self.x_min, self.x_max);
            let new_y = loop_float(self.y_pos[i] + y_moves[i], self.y_min, self.y_max);

            let new_part = self.get_partition(new_x, new_y);

            if new_part.0 >= PARTITIONS
            || new_part.1 >= PARTITIONS {           
                println!("ERROR x{} y{}", new_x, new_y);
            }

            if part.0 != new_part.0 || part.1 != new_part.1 {
                if let Some(pos) = self.partitions[part.0][part.1].iter().position(|&x| x == i) {
                    self.partitions[part.0][part.1].remove(pos);
                    self.partitions[new_part.0][new_part.1].push(i);
                }
            }

            if new_x < self.x_min
            || new_x > self.x_max 
            || new_y < self.y_min
            || new_y > self.y_max {           
                println!("ERROR");
            }

            self.x_mov[i] = x_moves[i];
            self.y_mov[i] = y_moves[i];

            self.x_pos[i] = new_x;
            self.y_pos[i] = new_y;
        }

        for i in 0..PARTICLES {
            if self.x_pos[i] < self.x_min
            || self.x_pos[i] > self.x_max
            || self.y_pos[i] < self.y_min
            || self.y_pos[i] > self.y_max
            || self.x_mov[i] > 100.0
            || self.y_mov[i] > 100.0 {                
                println!("ERROR");
            }
        }

        self.tick_counter.tick();
    }

    pub fn draw(&self, app: &App, draw: &Draw) {
        for i in 0..PARTITIONS {
            let x = self.x_min + i as f32 * self.part_w;
            let y = self.y_min + i as f32 * self.part_h;

            draw.line()
                .start(vec2(x, self.y_min))
                .end(vec2(x, self.y_max))
                .color(DARKGRAY);

            draw.line()
                .start(vec2(self.x_min, y))
                .end(vec2(self.x_max, y))
                .color(rgb(0.2, 0.2, 0.2));
        }

        for i in 0..PARTICLES {
            draw.ellipse()
                .x_y(self.x_pos[i], self.y_pos[i])
                .w_h(5.0, 5.0)
                .color(hsv(self.color[i] as f32 / COLORS as f32, 1.0, 1.0));
        }

        let win = app.window_rect();
        let win_p = win.pad(25.0);

        let fps_square = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);
        let fps_string = format!("{} fps", self.tick_counter.count_last_second());
        draw.text(&fps_string)
            .xy(fps_square.xy())
            .wh(fps_square.wh())
            .color(PLUM);
    }
}


fn loop_float(n: f32, lower: f32, upper: f32) -> f32
{
    if n < lower {
        let range = upper - lower;
        return n + ((lower - n) / range).ceil() * range;
    }

    if n >= upper {
        let range = upper - lower;
        return n - ((n - upper) / range).ceil() * range;
    }

    n
}

fn loop_to_int(n: i32, lower: i32, upper: i32) -> i32
{
    if n < lower {
        let range = (upper - lower) as f32;
        return n + (((lower - n) as f32 / range).ceil() * range) as i32;
    }

    if n == upper {
        return lower;
    }

    if n > upper {
        let range = (upper - lower) as f32;
        return n - (((n - upper) as f32 / range).ceil() * range) as i32;
    }

    n
}


fn get_force(dist: f32, attr: f32) -> f32 {
    if dist < NEUTRAL_R {
        return dist / NEUTRAL_R - 1.0;
    }

    if dist < MAX_R {
        let nominator = abs(2.0 * dist - MAX_R - NEUTRAL_R);
        let denominator  =  MAX_R - NEUTRAL_R;

        return attr * (1.0 - nominator / denominator)
    }

    0.0
}


