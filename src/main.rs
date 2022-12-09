use speedy2d::Window;
use speedy2d::dimen::{Vector2, Vec2};
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::Graphics2D;
use std::path::Path;
use std::time;
use std::f32::consts::PI;

use rand::{Rng, thread_rng, random};

struct ParticleState {
    x: f32,
    y: f32,
    angle: f32,
    size: f32,
    opacity: f32,
}

struct SmokeParticle {
    time_alive: f32,
    time_to_live: f32,
    start_state: ParticleState,
    end_state: ParticleState,
}

struct SmokeGenerator {
    x: f32,
    y: f32,
    particles: Vec<SmokeParticle>,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

fn dumbpow(num: f32, pow: i32) -> f32 {
    let mut pownum = num;
    for _ in 1..pow { pownum *= num }
    pownum
}

fn smoothstop(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * (1.0 - dumbpow(1.0-t, 2))
}

impl SmokeParticle {
    fn get_pos(&self) -> (f32, f32) {
        let t = 1f32 - (self.time_to_live - self.time_alive) / self.time_to_live;
        (
            smoothstop(self.start_state.x, self.end_state.x, t), 
            smoothstop(self.start_state.y, self.end_state.y, t)
        )
    }

    fn get_t(&self) -> f32 {
        1f32 - (self.time_to_live - self.time_alive) / self.time_to_live
    }

    fn get_size(&self) -> f32 {
        let t = self.get_t();
        smoothstop(self.start_state.size, self.end_state.size, t)
    }

    fn get_opacity(&self) -> f32 {
        let t = self.get_t();
        smoothstop(self.start_state.opacity, self.end_state.opacity, t)
    }

    fn get_angle(&self) -> f32 {
        let t = self.get_t();
        smoothstop(self.start_state.angle, self.end_state.angle, t)
    }


    fn alive(&self) -> bool {
        self.time_to_live > self.time_alive
    }
}

fn gen_range_float(a: f32, b: f32) -> f32 {
    a + random::<f32>() % (b - a)
}

impl SmokeGenerator {
    fn spawn_particle(&mut self, target_x: f32, target_y: f32) {
        let time_to_live = 3f32;

        const MIN_END_SIZE: f32 = 200.0;
        const MAX_END_SIZE: f32 = 600.0;
        const MIN_START_SIZE: f32 = 50.0;
        const MAX_START_SIZE: f32 = 100.0;

        const START_OPACITY: f32 = 0.1;
        const END_OPACITY: f32 = 0.0;
        

        let start_size = gen_range_float(MIN_START_SIZE, MAX_START_SIZE);
        let start_angle = gen_range_float(-PI, PI);
        let start_state = ParticleState {
            x: self.x,
            y: self.y,
            angle: start_angle,
            size: start_size,
            opacity: START_OPACITY,
        };
        
        const DIST: i32 = 200;
        let target_x = target_x + thread_rng().gen_range(-DIST..=DIST) as f32;
        let target_y = target_y + thread_rng().gen_range(-DIST..=DIST) as f32;

        let end_angle = gen_range_float(-PI, PI);
        let end_size:f32 = gen_range_float(MIN_END_SIZE, MAX_END_SIZE);
        let end_state = ParticleState {
            x: target_x,
            y: target_y,
            angle: end_angle,
            size: end_size,
            opacity: END_OPACITY,
        };

        self.particles.push( SmokeParticle { time_alive: 0f32, time_to_live, start_state, end_state} );
    }

    fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.time_alive += dt;
        }
    }
}


#[derive(Default)]
struct MyWindowHandler {
    mouse_pos: (f32, f32),
    last_frame: Option<time::Instant>, dt: f32,
    generator: Option<SmokeGenerator>,
    smoke_texture: Option<speedy2d::image::ImageHandle>,
    ticks: f32,
}

impl MyWindowHandler {
    fn calc_fps(&mut self) {
        let now = time::Instant::now();
        let elapsed = now.duration_since(self.last_frame.unwrap()).as_secs_f32();
        self.last_frame = Some(now);
        let fps = 1f32 / elapsed ;
        self.dt = elapsed;
        println!("FPS: {}", fps);
    }
}

fn get_rotated_quad(ix: f32, iy: f32, w: f32, h: f32, ang: f32) -> [Vec2; 4] {
    let cos = ang.cos();
    let sin = ang.sin();

    let rot_xy = |x, y| { (ix + x * cos - y * sin, iy + x * sin + y * cos) };

    let half_width = w / 2.0;
    let half_height = h / 2.0;

    let (x1, y1) = rot_xy(-half_width, -half_height);
    let (x2, y2) = rot_xy(half_width, -half_height);
    let (x3, y3) = rot_xy(half_width, half_height);
    let (x4, y4) = rot_xy(-half_width, half_height);

    [Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), Vec2::new(x4, y4)]
}

/*
fn rgb_from_hsv(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if (s == 0.0) { return (v, v, v) }
    let mut i = (h*6.) as i32;
    let f = (h*6.) - i as f32;
    let (p, q, t) = (v*(1.0-s), v*(1.0-s*f), v*(1.0-s*(1.0-f)));
    i %= 6;
    match i {
    }
    if i == 0:
} */

impl WindowHandler for MyWindowHandler
{
    fn on_start(&mut self, _helper: &mut WindowHelper, _: WindowStartupInfo)
    {
        self.last_frame = Some(time::Instant::now());

        self.generator = Some(
            SmokeGenerator{
                x: 100f32,
                y: 100f32,
                particles: Vec::new(),
            }
            );
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        if self.smoke_texture.is_none() {
            let image = graphics.create_image_from_file_path(None, speedy2d::image::ImageSmoothingMode::NearestNeighbor, Path::new("./smoke_particle.png")).unwrap();
            self.smoke_texture = Some(image);
        }

        graphics.clear_screen(Color::from_rgba(0.0, 0.0, 0.0, 0.0));

        self.calc_fps();
        let generator = self.generator.as_mut().unwrap();
        
        for _ in 0..20 {
            generator.spawn_particle(self.mouse_pos.0, self.mouse_pos.1);
        }
        generator.update_particles(self.dt);

        let smoke_texture = self.smoke_texture.as_ref().unwrap();
        for particle in &generator.particles {
            let (cx, cy) = particle.get_pos();
            let p_size: f32 = particle.get_size();
            let opacity = particle.get_opacity();
            let angle = particle.get_angle();

            let quad = get_rotated_quad(cx, cy, p_size, p_size, angle);
            // graphics.draw_quad(quad, color);
            // let color = Color::from_rgba(1.0, 1.0, 1.0, opacity);
            let color = Color::from_rgba(1.0, 1.0, 1.0, opacity);
            let colors: [Color; 4] = [color; 4];
            let image_cords = get_rotated_quad(0.0, 0.0, 2.0, 2.0, 0.0);
            graphics.draw_quad_image_tinted_four_color(quad, colors, image_cords, smoke_texture);
        }

        generator.particles.retain(|x| { x.alive() });
        helper.request_redraw();
    }


    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, mouse_pos: Vector2<f32>){
        self.mouse_pos = (mouse_pos.x, mouse_pos.y);
    }

}

fn main() {
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    window.run_loop::<MyWindowHandler>(Default::default());
}
