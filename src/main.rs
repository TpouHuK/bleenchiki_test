use speedy2d::Window;
use speedy2d::dimen::{Vector2, Vec2};
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::Graphics2D;
use std::path::Path;
use std::time;

mod smoke;
use smoke::*;

#[derive(Default)]
struct MyWindowHandler {
    mouse_pos: (f32, f32),
    last_frame: Option<time::Instant>, dt: f32,
    generator: Option<SmokeGenerator>,
    smoke_texture: Option<speedy2d::image::ImageHandle>,
    ticks: f32,
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
    fn calc_fps(&mut self) {
        let now = time::Instant::now();
        let elapsed = now.duration_since(self.last_frame.unwrap()).as_secs_f32();
        self.last_frame = Some(now);
        let fps = 1f32 / elapsed ;
        self.dt = elapsed;
        self.ticks += self.dt;
        println!("FPS: {}", fps);
    }
}

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
