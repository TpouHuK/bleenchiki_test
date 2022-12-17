use rand::random;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use glam::f32::Vec2;

const DELTA_TIME: f32 = 1.0 / 60.0;

pub struct PhysicsParticle {
    pub pos: Vec2,
    last_pos: Vec2,
    acc: Vec2,
    mass: f32,
    
    color: Color,
    radius: f32,
    fixed: bool,
}

impl PhysicsParticle {
    pub fn display(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle::<(f32, f32)>(self.pos.into(), self.radius, self.color);

    }
    
    pub fn physics_step(&mut self) {
        if self.fixed { return; }
        let velocity = self.pos - self.last_pos;
        self.last_pos = self.pos;
        self.pos += velocity + self.acc * DELTA_TIME.powi(2);
        self.acc = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc += acc;
    }

    pub fn constrain_circle(&mut self, circle_pos: Vec2, r: f32) {
        let diff = self.pos - circle_pos;
        
        let len_sq = diff.length_squared();
        let max_dist = r-self.radius;  
        if len_sq > max_dist.powi(2) {
            let to_move = diff.normalize();
            self.pos = circle_pos + diff.normalize() * max_dist;
        }
    }
    
    pub fn constrain_rect(&mut self, x: f32, y: f32, w: f32, h: f32){
        self.pos.x = self.pos.x.clamp(x, x+w);
        self.pos.y = self.pos.y.clamp(y, y+h);
    }

    pub fn solve_collision(&mut self, other: &mut PhysicsParticle) {
    }
}


pub struct DistanceConstraint {
    particle_a: usize,
    particle_b: usize,
    length: f32,
}

pub struct AngleConstraint {
    particle_a: usize,
    particle_b: usize,
    particle_c: usize,
    angle: f32,
}


pub fn init_test_simulation(sim: &mut ParticleSimulation) {
        const SCREEN_MIDDLE: Vec2 = Vec2::new(1280.0/2.0, 720.0/2.0);
        const DEFAULT_MASS: f32 = 1.0;
        const DYNAMIC: bool = false;
        const FIXED: bool = true;

        let a = sim.new_particle(SCREEN_MIDDLE + Vec2::new(100.0, 0.0), 10.0, DEFAULT_MASS, FIXED);
        let b = sim.new_particle(SCREEN_MIDDLE - Vec2::new(100.0, 0.0), 10.0, DEFAULT_MASS, DYNAMIC);
        sim.new_distance_constrain(a, b, 100.0);

}

pub struct ParticleSimulation {
    pub particles: Vec<PhysicsParticle>,
    distance_constrains: Vec<DistanceConstraint>,
    angle_constrains: Vec<AngleConstraint>,
}

impl ParticleSimulation {
    pub fn new() -> Self {
        ParticleSimulation{
            particles: Vec::new(),
            distance_constrains: Vec::new(),
            angle_constrains: Vec::new(),
        }
    }

    pub fn physics_step(&mut self) {

        for _ in 0..100 {
            //self.solve_collisions(); 
            self.solve_distance_constrains();
            self.solve_angle_constrains();
        }

        const GRAVITY: Vec2 = Vec2::new(0.0, 0.0);
        for particle in &mut self.particles {
            particle.accelerate(GRAVITY); // Applying gravity
            particle.physics_step();
            const SCREEN_MIDDLE: Vec2 = Vec2::new(1280.0/2.0, 720.0/2.0);
            //particle.constrain_circle(SCREEN_MIDDLE, 300.0);
        }


    }

    fn solve_distance_constrains(&mut self) {
        for constrain in &self.distance_constrains {
            let [a, b] = self.particles.get_many_mut([constrain.particle_a, constrain.particle_b]).unwrap();
            let dist = b.pos - a.pos;
            let ab = dist.normalize_or_zero();
            let adjust_amount = (dist.length() - constrain.length) * ab;
            let mass_sum = a.mass + b.mass;
            let adjust_a = adjust_amount * b.mass / mass_sum;
            let adjust_b = adjust_amount * a.mass / mass_sum;
            if !a.fixed { a.pos += adjust_a }
            if !b.fixed { b.pos -= adjust_b }
        }
    }

    fn solve_angle_constrains(&mut self) {
        for constrain in &self.angle_constrains {
            let init_angle = self.get_hack_angle(constrain.particle_a, constrain.particle_b);
            let angle = init_angle + self.get_angle_between_particles(constrain.particle_a, constrain.particle_b, constrain.particle_c) / 2.0;
            let [a, b, c] = self.particles.get_many_mut([constrain.particle_a, constrain.particle_b, constrain.particle_c]).unwrap();
        }
    }

    fn solve_collisions(&mut self) {
        let l = self.particles.len();
        for i in 0..l {
            for j in (i+1)..l {
                unsafe {
                    let (left, right) = self.particles.split_at_mut_unchecked(j);
                    let a = left.get_unchecked_mut(i);
                    let b = right.get_unchecked_mut(0);
                    a.solve_collision(b);
                }
            }
        }
    }

    pub fn display(&mut self, graphics: &mut Graphics2D) {
        for constrain in &self.distance_constrains {
            let [a, b] = self.particles.get_many_mut([constrain.particle_a, constrain.particle_b]).unwrap();
            Self::display_distance_constrain(a, b, graphics);
        }

        for constrain in &self.angle_constrains {
            // Hack
            let init_angle = self.get_hack_angle(constrain.particle_a, constrain.particle_b);
            let sub_angle = self.get_hack_angle(constrain.particle_c, constrain.particle_b);
            let angle = init_angle - self.get_angle_between_particles(constrain.particle_a, constrain.particle_b, constrain.particle_c) / 2.0;

            let [a, b, c] = self.particles.get_many_mut([constrain.particle_a, constrain.particle_b, constrain.particle_c]).unwrap();

            const LINE_THICKNESS: f32 = 1.0;
            const LINE_COLOR: Color = Color::from_rgb(1.0, 0.0, 0.0);
            const LINE_COLOR2: Color = Color::from_rgb(0.0, 1.0, 0.0);
            const LINE_COLOR3: Color = Color::from_rgb(0.0, 0.0, 1.0);
            graphics.draw_line::<(f32, f32), (f32, f32)>(b.pos.into(), (b.pos + Vec2::from_angle(init_angle)*100.0).into(), LINE_THICKNESS, LINE_COLOR2);
            graphics.draw_line::<(f32, f32), (f32, f32)>(b.pos.into(), (b.pos + Vec2::from_angle(angle)*100.0).into(), LINE_THICKNESS, LINE_COLOR);
            graphics.draw_line::<(f32, f32), (f32, f32)>(b.pos.into(), (b.pos + Vec2::from_angle(sub_angle)*100.0).into(), LINE_THICKNESS, LINE_COLOR3);
        }

        for particle in &self.particles {
            particle.display(graphics);
        }
    }

    fn display_distance_constrain(a: &PhysicsParticle, b: &PhysicsParticle, graphics: &mut Graphics2D){
        const LINE_THICKNESS: f32 = 1.0;
        const LINE_COLOR: Color = Color::from_rgb(237.0/255.0, 198.0/255.0, 114.0/255.0);

        graphics.draw_line::<(f32, f32), (f32, f32)>(a.pos.into(), b.pos.into(), LINE_THICKNESS, LINE_COLOR);
    }

    pub fn new_particle(&mut self, pos: Vec2, r: f32, mass: f32, fixed: bool) -> usize {
        let particle = PhysicsParticle {
            pos,
            last_pos: pos,
            acc: Vec2::ZERO,
            mass,
            color: Color::WHITE,
            radius: r,
            fixed,
        };
        self.particles.push(particle);
        self.particles.len() - 1
    }

    pub fn new_distance_constrain(&mut self, particle_a: usize, particle_b: usize, length: f32) {
        self.distance_constrains.push(DistanceConstraint{ particle_a, particle_b, length });
    }

    pub fn get_distance_between_particles(&mut self, particle_a: usize, particle_b: usize) -> f32 {
            let [a, b] = self.particles.get_many_mut([particle_a, particle_b]).unwrap();
            a.pos.distance(b.pos)
    }

    pub fn get_hack_angle(&self, particle_a: usize, particle_b: usize) -> f32 {
            let a = &self.particles[particle_a];
            let b = &self.particles[particle_b];
            let (x, y) = (b.pos - a.pos).normalize().into();
            y.atan2(x)
    }

    pub fn get_angle_between_particles(&self, particle_a: usize, particle_b: usize, particle_c: usize) -> f32 {
            let a = &self.particles[particle_a];
            let b = &self.particles[particle_b];
            let c = &self.particles[particle_c];
            let ba = (a.pos - b.pos).normalize();
            let bc = (c.pos - b.pos).normalize();
            (f32::atan2(ba.y, ba.x) - f32::atan2(bc.y, bc.x)) % std::f32::consts::PI / 2.0
    }
    
    pub fn new_distance_constrain_in_place(&mut self, particle_a: usize, particle_b: usize) {
        let length = self.get_distance_between_particles(particle_a, particle_b);
        self.distance_constrains.push( DistanceConstraint { particle_a, particle_b, length })
    }

    pub fn new_angle_constrain_in_place(&mut self, particle_a: usize, particle_b: usize, particle_c: usize) {
        let angle = self.get_angle_between_particles(particle_a, particle_b, particle_c);
        self.angle_constrains.push( AngleConstraint { particle_a, particle_b, particle_c, angle })
    }

    pub fn select_point(&self, pos: Vec2) -> Option<usize> {
        const EPSILON: f32 = 10.0;
        for i in 0..self.particles.len() {
            if self.particles[i].pos.distance_squared(pos) < EPSILON {
                return Some(i)
            }
        }
        None
    }

}
