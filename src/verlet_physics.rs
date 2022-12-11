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

pub struct PhysicsParticle {
    x: f32,
    y: f32,
    old_x: f32,
    old_y: f32,
    acc_x: f32,
    acc_y: f32,
    radius: f32,
    heat: f32,
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
        let vel_x = self.x - self.old_x;
        let vel_y = self.y - self.old_y;
        self.old_x = self.x;
        self.old_y = self.y;
        self.x = self.x + vel_x + self.acc_x * DELTA_TIME * DELTA_TIME;
        self.y = self.y + vel_y + self.acc_y * DELTA_TIME * DELTA_TIME;
        self.acc_x = 0.0;
        self.acc_y = 0.0;
    }

    pub fn update_heat(&mut self) {
        if self.y > 690.0 {
            self.heat += 0.04;
        } else {
            self.heat *= 0.99;
        }
        if self.y < 300.0 {
            if self.heat > 0.0 {
            self.heat -= 0.1;
            }
        }
        if self.heat > 0.0 {
            self.accelerate(0.0, -self.heat*400.0);
        }
        self.radius = 1.0 + self.heat * 5.0
    }

    pub fn update_color(&mut self) {
        let (r, g, b) = hsv_to_rgb(0.05, 1.0, self.heat);
        self.color = Color::from_rgb(r, g, b);
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
            let combined_heat = (self.heat + other.heat) / 2.0;

            self.heat += (combined_heat - self.heat) * 0.03;
            other.heat += (combined_heat - other.heat) * 0.03;

        }
    }
}

pub struct ParticleSimulation {
    particles: Vec<PhysicsParticle>,
}

pub fn get_pair_mut<T>(
        slice: &mut [T],
        i: usize,
        j: usize,
) -> Option<(&mut T, &mut T)> {
    let (first, second) = (i.min(j), i.max(j));
    
    if i == j || second >= slice.len() {
        return None;
    }
    
    let (_, tmp) = slice.split_at_mut(first);
    let (x, rest) = tmp.split_at_mut(1);
    let (_, y) = rest.split_at_mut(second - first - 1);
    let pair = if i < j { 
        (&mut x[0], &mut y[0])
    } else {
        (&mut y[0], &mut x[0])
    };
    
    Some(pair)
}

impl ParticleSimulation {
    pub fn new() -> Self {
        ParticleSimulation{ particles: Vec::new() }
    }

    pub fn update(&mut self) {
        for _ in 0..12 {
            self.solve_collisions(); 
        }
        for particle in &mut self.particles {
            particle.accelerate(0.0, 150.0); // Applying gravity
            particle.physics_step();
            particle.update_heat();
            particle.update_color();
            particle.constrain_rect(0.0, 0.0, 700.0, 700.0);
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

    pub fn new_particle(&mut self, x: f32, y: f32, r: f32) {
        let particle = PhysicsParticle {
            x,
            y,
            old_x: x,
            old_y: y,
            acc_x: 0.0,
            acc_y: 0.0,
            radius: r,
            heat: 0.0,
            color: Color::from_rgb(1.0, 1.0, 1.0),
        };
        self.particles.push(particle);
    }
}
