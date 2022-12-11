use rand::{Rng, thread_rng, random};
use std::f32::consts::PI;

struct ParticleState {
    x: f32,
    y: f32,
    angle: f32, size: f32, opacity: f32,
}

pub struct SmokeParticle {
    time_alive: f32,
    time_to_live: f32,
    start_state: ParticleState,
    end_state: ParticleState,
}

pub struct SmokeGenerator {
    pub x: f32,
    pub y: f32,
    pub particles: Vec<SmokeParticle>,
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
    pub fn get_pos(&self) -> (f32, f32) {
        let t = 1f32 - (self.time_to_live - self.time_alive) / self.time_to_live;
        (
            smoothstop(self.start_state.x, self.end_state.x, t), 
            smoothstop(self.start_state.y, self.end_state.y, t)
        )
    }

    fn get_t(&self) -> f32 {
        1f32 - (self.time_to_live - self.time_alive) / self.time_to_live
    }

    pub fn get_size(&self) -> f32 {
        let t = self.get_t();
        smoothstop(self.start_state.size, self.end_state.size, t)
    }

    pub fn get_opacity(&self) -> f32 {
        let t = self.get_t();
        smoothstop(self.start_state.opacity, self.end_state.opacity, t)
    }

    pub fn get_angle(&self) -> f32 {
        let t = self.get_t();
        smoothstop(self.start_state.angle, self.end_state.angle, t)
    }


    pub fn alive(&self) -> bool {
        self.time_to_live > self.time_alive
    }
}

fn gen_range_float(a: f32, b: f32) -> f32 {
    a + random::<f32>() % (b - a)
}

impl SmokeGenerator {
    pub fn spawn_particle(&mut self, target_x: f32, target_y: f32) {
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

    pub fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.time_alive += dt;
        }
        self.particles.retain(|x| { x.alive() });
    }
}


/*
 * this is commented out code for drawing particles
        for particle in &generator.particles {
            let (cx, cy) = particle.get_pos();
            let p_size: f32 = particle.get_size();
            let opacity = particle.get_opacity();
            let angle = particle.get_angle();

            let quad = get_rotated_quad(cx, cy, p_size, p_size, angle);
            let color = Color::from_rgba(1.0, 1.0, 1.0, opacity);
            let colors: [Color; 4] = [color; 4];
            let image_cords = get_rotated_quad(0.0, 0.0, 2.0, 2.0, 0.0);
            graphics.draw_quad_image_tinted_four_color(quad, colors, image_cords, smoke_texture);
        }

fn get_smoke_texture(graphics: &mut Graphics2D) {
        let image = graphics.create_image_from_file_path(None, speedy2d::image::ImageSmoothingMode::NearestNeighbor, Path::new("./smoke_particle.png")).unwrap();
}
*/
