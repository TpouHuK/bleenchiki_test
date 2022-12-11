#![feature(slice_split_at_unchecked)]
#![allow(dead_code, unused_variables)]
use rand::random;
use speedy2d::Window;
use speedy2d::dimen::{Vector2, Vec2};
use speedy2d::color::Color;
use speedy2d::shape::Rect;
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::Graphics2D;
use std::path::Path;
use std::time;

mod smoke;
use smoke::*;

mod verlet_physics;
use verlet_physics::*;

struct MyWindowHandler {
    mouse_pos: (f32, f32),
    last_frame: time::Instant,
    dt: f32,
    ticks: f32,
    simulation: ParticleSimulation,
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


impl MyWindowHandler {
    fn new() -> Self {
        let last_frame = time::Instant::now();
        let mouse_pos = (0.0, 0.0);
        let simulation = ParticleSimulation::new();
        let dt = 0.0;
        let ticks = 0.0;
        let mut win_h = MyWindowHandler { mouse_pos, last_frame, dt, ticks, simulation };
        win_h._init_simulation();
        win_h

    }

    fn _init_simulation(&mut self) {
        let max_r = 3.0;
        for y in 0..20 {
        for x in 0..40*2 {
                let radius = 1.0 + random::<f32>() * 5.0;
                self.simulation.new_particle(10.0 + x as f32*8.0, 300.0 + y as f32 * 8.0, radius);
            }
        }
    }

    fn calc_fps(&mut self) {
        let now = time::Instant::now();
        let elapsed = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        let fps = 1f32 / elapsed ;
        self.dt = elapsed;
        self.ticks += self.dt;
        println!("fps: {}", fps);
    }

    fn _draw(&mut self, graphics: &mut Graphics2D) {
        let color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let position = (400.0, 400.0);
        let rect = Rect::new((0.0, 0.0).into(), (700.0, 700.0).into());
        graphics.draw_rectangle(rect, color);
        self.simulation.display(graphics);
        self.simulation.update();
    }
}


impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgba(0.0, 0.0, 0.0, 1.0));
        self.calc_fps();
        self._draw(graphics);
        helper.request_redraw();
    }


    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, mouse_pos: Vector2<f32>){
        self.mouse_pos = (mouse_pos.x, mouse_pos.y);
    }

}

fn main() {
    let window = Window::new_centered("Title", (1200, 1200)).unwrap();
    let window_handler = MyWindowHandler::new();
    window.run_loop::<MyWindowHandler>(window_handler);
}
