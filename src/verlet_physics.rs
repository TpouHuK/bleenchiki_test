use rand::random;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use glam::f32::Vec2;

const DELTA_TIME: f32 = 1.0 / 60.0;

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 { return (v, v, v) };
    let i = (h*6.0).trunc();
    let f = (h*6.0)-i;
    let (p, q, t) = (v*(1.0 - s), v*(1.0-s*f), v*(1.0-s*(1.0-f)));
    match i as i32 % 6 {
        0 => { (v, t, p) },
        1 => { (q, v, p) },
        2 => { (p, v, t) },
        3 => { (p, q, v) },
        4 => { (t, p, v) },
        5 => { (v, p, q) },
        _ => { unreachable!() },
    }
}

pub struct PhysicsParticle {
    pos: Vec2,
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
        let velocity = self.pos - self.last_pos;
        self.last_pos = self.pos;
        self.pos += velocity + self.acc * DELTA_TIME.powi(2);
        self.acc = Vec2::ZERO;
        //todo!();
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc += acc;
    }

    pub fn constrain_circle(&mut self, cx: f32, cy: f32, r: f32) {
        //todo!();
    }

    pub fn constrain_rect(&mut self, x: f32, y: f32, w: f32, h: f32){
        //todo!();
    }

    pub fn solve_collision(&mut self, other: &mut PhysicsParticle) {
        //todo!();
    }
}


pub struct Branch {
    particle_a: usize,
    particle_b: usize,
    length: f32,
}

pub struct Tree ();

impl Tree {
    pub fn new(simulation: &mut ParticleSimulation) -> Self {
        const SCREEN_MIDDLE: Vec2 = Vec2::new(1280.0/2.0, 720.0/2.0);
        const DEFAULT_MASS: f32 = 1.0;
        const DYNAMIC: bool = false;

        simulation.new_particle(SCREEN_MIDDLE, 10.0, DEFAULT_MASS, DYNAMIC);
        Tree()
    }
    
    pub fn test_new(simulation: &mut ParticleSimulation) -> Self {
        Tree()
    }
}

pub struct ParticleSimulation {
    particles: Vec<PhysicsParticle>,
    branches: Vec<Branch>,
}

impl ParticleSimulation {
    pub fn new() -> Self {
        ParticleSimulation{ particles: Vec::new() , branches: Vec::new() }
    }

    pub fn update(&mut self) {
        for _ in 0..1 {
            self.solve_collisions(); 
            self.solve_branches();
        }

        const GRAVITY: Vec2 = Vec2::new(0.0, 150.0);
        for particle in &mut self.particles {
            particle.accelerate(GRAVITY); // Applying gravity
            particle.physics_step();
        }

    }

    fn solve_branches(&mut self) {
        for branch in &self.branches {
            let [a, b] = self.particles.get_many_mut([branch.particle_a, branch.particle_b]).unwrap();
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

    pub fn display(&self, graphics: &mut Graphics2D) {
        for particle in &self.particles {
            particle.display(graphics);
        }
    }

    pub fn new_branch(&mut self, particle_a: usize, particle_b: usize, length: f32,target_angle: f32){
        let branch = Branch {
            particle_a,
            particle_b,
            length,
        };
        self.branches.push(branch);
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
}
