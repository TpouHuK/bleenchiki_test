#![feature(slice_split_at_unchecked, get_many_mut)]

#![allow(dead_code, unused_variables, unused_imports)]

use rand::random;
use speedy2d::Window;
use speedy2d::dimen::{Vector2, Vec2};
use speedy2d::color::Color;
use speedy2d::shape::Rect;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use std::time;
use std::thread::sleep;

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
    fn new(simulation: ParticleSimulation) -> Self {
        let last_frame = time::Instant::now();
        let mouse_pos = (0.0, 0.0);
        let dt = 0.0;
        let ticks = 0.0;
        MyWindowHandler { mouse_pos, last_frame, dt, ticks, simulation }
    }

    fn calc_fps(&mut self) {
        let now = time::Instant::now();
        let elapsed = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        let fps = 1f32 / elapsed ;
        self.dt = elapsed;
        self.ticks += self.dt;
        println!("fps: {fps}");
        //sleep(time::Duration::from_millis(3000));
    }

    fn _draw(&mut self, graphics: &mut Graphics2D) {
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
    let window = Window::new_centered("Hello testing", (1280, 720)).unwrap();
    let mut simulation = ParticleSimulation::new();
    Tree::new(&mut simulation);
    let window_handler = MyWindowHandler::new(simulation);
    window.run_loop::<MyWindowHandler>(window_handler);
}
