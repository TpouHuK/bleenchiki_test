use glam::Vec2;
use rand::{Rng, thread_rng};
use std::f32::consts::PI;

use crate::verlet_physics::*;


pub fn create_new_tree(sim: &mut ParticleSimulation) {
    const ROOT_POS: Vec2 = Vec2::new(1280.0/2.0, 720.0 - 100.0);
    const ANGLE_VARIATION: f32 = 10.0f32 / 180.0 * PI;
    const BRANCH_PROB: f32 = 1.0 / 4.0;
    const BRANCH_SCALING: f32 = 1.4;
    const BRANCH_ANGLE: f32 = 30.0f32 / 180.0 * PI;
    const MAX_DEPTH: u32 = 7;


    fn gen_smth(sim: &mut ParticleSimulation, mut prev_particle: usize, mut prev_prev_particle: usize, mut prev_pos: Vec2, scale: f32, mut angle: f32, depth: u32) {
        let step = scale / 10.0;
        let mut rng = thread_rng();
        let stem_cnt = rng.gen_range(4..=10);
        for cnt in 0..stem_cnt {
            let angle_variation = rng.gen_range(-ANGLE_VARIATION..ANGLE_VARIATION);
            angle += angle_variation;

            prev_pos += Vec2::new(0.0, -step).rotate(Vec2::from_angle(angle));
            let new_particle = sim.new_particle(prev_pos, 3.0, 1.0, false);
            sim.new_distance_constrain(prev_particle, new_particle, step);
            //let keep_angle = sim.get_angle(prev_prev_particle, prev_particle, new_particle);
            let keep_angle = 0.0;
            sim.new_angle_constrain(prev_prev_particle, prev_particle, new_particle, keep_angle);
            prev_prev_particle = prev_particle;
            prev_particle = new_particle;

            if rng.gen_bool(cnt as f64 / stem_cnt as f64) && depth < MAX_DEPTH{
                gen_smth(sim, prev_particle, prev_prev_particle, prev_pos, scale / BRANCH_SCALING, angle + BRANCH_ANGLE, depth+1);
                gen_smth(sim, prev_particle, prev_prev_particle, prev_pos, scale / BRANCH_SCALING, angle - BRANCH_ANGLE,  depth+1);
                break;
            }
        };
    }
    
    let pre_root = sim.new_particle(ROOT_POS - Vec2::new(-100.0, 0.0), 3.0, 1.0, true);
    let root = sim.new_particle(ROOT_POS, 3.0, 1.0, true);
    const UP_ANGLE: f32 = 0.0;
    gen_smth(sim, root, pre_root, ROOT_POS, 300.0, UP_ANGLE, 0);
}
