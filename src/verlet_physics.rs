use rand::random;
use speedy2d::Graphics2D;
use speedy2d::color::Color;

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

fn get_distance(a: &PhysicsParticle, b: &PhysicsParticle) -> f32 {
    (sqr(a.x - b.x) + sqr(a.y - b.y)).sqrt()
}

fn get_angle(a: &PhysicsParticle, b: &PhysicsParticle) -> f32 {
    let dy = b.y - a.y;
    let dx = b.x - a.x;
    dy.atan2(dx)
}

pub struct PhysicsParticle {
    x: f32,
    y: f32,
    old_x: f32,
    old_y: f32,
    acc_x: f32,
    acc_y: f32,
    mass: f32,
    angle: f32,
    fixed: bool,
    radius: f32,
    color: Color,
}

fn sqr(x: f32) -> f32 {
    x*x
}

impl PhysicsParticle {
    pub fn display(&self, graphics: &mut Graphics2D) {
        let position = (self.x, self.y);
        graphics.draw_circle(position, self.radius, self.color);
    }
    
    pub fn physics_step(&mut self) {
        if self.fixed { return } ;

        const AIR_FRICTION: f32 = 0.990;
        let vel_x = (self.x - self.old_x) * AIR_FRICTION;
        let vel_y = (self.y - self.old_y) * AIR_FRICTION;
        self.old_x = self.x;
        self.old_y = self.y;
        self.x = self.x + vel_x + self.acc_x * DELTA_TIME * DELTA_TIME;
        self.y = self.y + vel_y + self.acc_y * DELTA_TIME * DELTA_TIME;
        self.acc_x = 0.0;
        self.acc_y = 0.0;
    }

    pub fn accelerate(&mut self, acc_x: f32, acc_y: f32) {
        self.acc_x += acc_x;
        self.acc_y += acc_y;
    }

    pub fn constrain_circle(&mut self, cx: f32, cy: f32, r: f32) {
        let to_obj_x = self.x - cx;
        let to_obj_y = self.y - cy;

        let dist = (to_obj_x*to_obj_x) + (to_obj_y*to_obj_y);
        if dist > sqr(r - self.radius) {
            let dist = dist.sqrt();
            let n_x = to_obj_x / dist;
            let n_y = to_obj_y / dist;
            self.x = cx + n_x * (r - self.radius);
            self.y = cy + n_y * (r - self.radius);
        }
    }

    pub fn constrain_rect(&mut self, x: f32, y: f32, w: f32, h: f32){
        if self.x < x+self.radius { self.x = x+self.radius }
        if self.y < y+self.radius { self.y = y+self.radius }
        if self.x > x+w-self.radius { self.x = x+w-self.radius }
        if self.y > y+h-self.radius { self.y = y+h-self.radius }
    }

    pub fn solve_collision(&mut self, other: &mut PhysicsParticle) {
        let coll_x = self.x - other.x;
        let coll_y = self.y - other.y;

        let dist = (coll_x*coll_x) + (coll_y*coll_y);
        let trig_dist = self.radius + other.radius;
        if dist < sqr(trig_dist) {
            let dist = dist.sqrt();
            let n_x = coll_x / dist;
            let n_y = coll_y / dist;
            let delta = (self.radius + other.radius) - dist;

            self.x += (self.radius/trig_dist) * delta * n_x;
            self.y += (self.radius/trig_dist) * delta * n_y;
            other.x -= (other.radius/trig_dist) * delta * n_x;
            other.y -= (other.radius/trig_dist) * delta * n_y;
        }
    }
}


pub struct Branch {
    particle_a: usize,
    particle_b: usize,
    length: f32,
    target_angle: f32,
}

pub struct Tree ();

impl Tree {
    pub fn new(simulation: &mut ParticleSimulation) -> Self {
        let (root_x, root_y) = (1280.0/2.0, 620.0);
        let root_r = 3.0;
        let root = simulation.new_particle(root_x, root_y, root_r, 1.0,  true);
        simulation.particles[root].angle = (-90f32).to_radians();

        let mut x = root_x; let mut y = root_y;
        let mut last = root;
        let mut mass = 100.0;
        for _ in 0..40 {
            x += random::<f32>()*30.0 - 15.0;
            y -= 10.0;
            mass -= 2.0;
            let r = 4.0;
            let new = simulation.new_particle(x, y, r, mass, false);
            let angle =  (random::<f32>()*10.0).to_radians();
            simulation.new_branch(last, new, 10.0, angle);
            last = new;
        }

        Tree()
    }
    
    pub fn test_new(simulation: &mut ParticleSimulation) -> Self {
        let (root_x, root_y) = (1280.0/2.0, 620.0);
        let root_r = 3.0;
        let root = simulation.new_particle(root_x, root_y, root_r, 1.0,  true);


        let d90 = (90f32).to_radians();
        let stem_point = simulation.new_particle(root_x - 100.0, root_y - 100.0, root_r*1.0, 1.0, false);
        simulation.new_branch(root, stem_point, 100.0, d90/2.0);

        let another_point = simulation.new_particle(root_x + 200.0, root_y - 100.0, root_r, 1.0, false);
        simulation.new_branch(stem_point, another_point, 100.0, d90/2.0);

        let another_point2 = simulation.new_particle(root_x + 200.0, root_y - 100.0, root_r*1.0, 1.0, false);
        simulation.new_branch(another_point, another_point2, 100.0, 0.0);

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
            //self.solve_collisions(); 
            self.solve_branches();
        }

        for particle in &mut self.particles {
            //particle.accelerate(0.0, 150.0); // Applying gravity
            particle.physics_step();
        }

    }

    fn solve_branches(&mut self) {
        for branch in &self.branches {
            let [a, b] = self.particles.get_many_mut([branch.particle_a, branch.particle_b]).unwrap();
            let dist = get_distance(a, b);
            let angle = get_angle(a, b);
            println!("a: {}, b: {}, angle: {}", branch.particle_a, branch.particle_b, a.angle.to_degrees());
            // Forcing distance
            let (c_sin, c_cos) = angle.sin_cos();
            let (set_x, set_y) = (a.x + branch.length*c_cos, a.y + branch.length*c_sin);
            b.x = set_x;
            b.y = set_y;
            let (t_sin, t_cos) = (a.angle + branch.target_angle).sin_cos();
            let (tgt_x, tgt_y) = (a.x + branch.length*t_cos, a.y + branch.length*t_sin);

            const POWER: f32 = 1.0;
            let sum = a.mass + b.mass;
            let b_power = POWER/(b.mass/sum);
            let a_power = POWER/(a.mass/sum);

            let bdx = (tgt_x - b.x) * b_power;
            let bdy = (tgt_y - b.y) * b_power;
            b.accelerate(bdx, bdy);
            b.angle = angle;

            //if !a.fixed {
               // let adx = -(tgt_x - b.x) * a_power;
              //  let ady = -(tgt_y - b.y) * a_power;
             //   a.accelerate(adx, ady);
            //}
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
            target_angle,
        };
        self.branches.push(branch);
    }

    pub fn new_particle(&mut self, x: f32, y: f32, r: f32, mass: f32, fixed: bool) -> usize {
        let particle = PhysicsParticle {
            x,
            y,
            old_x: x,
            old_y: y,
            acc_x: 0.0,
            acc_y: 0.0,
            angle: 0.0,
            mass,
            fixed,
            radius: r,
            color: Color::from_rgb(1.0, 1.0, 1.0),
        };
        self.particles.push(particle);
        self.particles.len() - 1
    }
}
