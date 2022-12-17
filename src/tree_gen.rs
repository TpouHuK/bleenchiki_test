use glam::Vec2;
use speedy2d::color::Color;
use rand::{Rng, thread_rng};
use core::f32;
use std::f32::consts::PI;
use speedy2d::Graphics2D;

use crate::verlet_physics::*;


struct Node {
    width: f32,
    angle: f32,
    length: f32,
    level: u32,
    parent: Option<usize>,
    pos: Vec2,
}

impl Node {
    fn derive_from(&self, self_id: usize) -> Self {
        let width = self.width * WIDTH_COEFFICIENT;
        let angle = self.angle + get_angle_deviation();
        let length = self.length * LENGTH_COEFFICIENT;

        let pos = self.pos + Vec2::from_angle(angle)*length;
        Node {
            width,
            angle,
            length,
            level: self.level,
            pos,
            parent: Some(self_id),
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>
}

impl Tree {
    fn new () -> Self {
        Tree { nodes: Vec::new() }
    }

    fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len()-1
    }

    pub fn display(&self, graphics: &mut Graphics2D) {
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];

            if let Some(parent) = node.parent {
                let parent = &self.nodes[parent];
                const LINE_COLOR: Color = Color::from_rgb(237.0/255.0, 198.0/255.0, 114.0/255.0);
                graphics.draw_line::<(f32, f32), (f32, f32)>(parent.pos.into(), node.pos.into(), node.width, LINE_COLOR);
            }

        }
    }
}

const ROOT_WIDTH: f32 = 10.0;
const ROOT_ANGLE: f32 = -PI/2.0; // Up
const ROOT_LENGTH: f32 = 40.0;

const ANGLE_DEVIATION: f32 = 10.0; // Degrees
                                   //
const SPLIT_ANGLE_DEVIATION: f32 = 10.0;
const SPLIT_ANGLE_DEFAULT: f32 = 30.0;


const WIDTH_COEFFICIENT: f32 = 0.90;
const SPLIT_WIDTH_COEFFICIENT: f32 = 0.75;
const LENGTH_COEFFICIENT: f32 = 0.99;

const WIDTH_THRESHOLD: f32 = 1.0;

fn get_angle_deviation() -> f32 {
    thread_rng().gen_range(-ANGLE_DEVIATION..ANGLE_DEVIATION).to_radians()
}

fn get_split_angle() -> f32 {
    SPLIT_ANGLE_DEFAULT + thread_rng().gen_range(-SPLIT_ANGLE_DEVIATION..SPLIT_ANGLE_DEVIATION) * if thread_rng().gen_bool(0.5) {1.0 } else { -1.0 }
}

fn move_forward(tree: &mut Tree, ref_root: usize) -> usize {
    let ancestor = &tree.nodes[ref_root];
    let new_node = ancestor.derive_from(ref_root);
    tree.add_node(new_node)
}

fn make_split(tree: &mut Tree, ref_root: usize) -> usize {
    let ancestor = &tree.nodes[ref_root];
    let mut new_node = ancestor.derive_from(ref_root);
    new_node.angle += get_split_angle();
    new_node.width *= SPLIT_WIDTH_COEFFICIENT;
    tree.add_node(new_node)
}

fn recursive_gen(tree: &mut Tree, mut previous: usize) {
    loop {
        previous = move_forward(tree, previous);
        if previous % 3 == 0 && previous != 0 {
            let split = make_split(tree, previous);
            recursive_gen(tree, split);
        }
        if tree.nodes[previous].width < WIDTH_THRESHOLD {
            break;
        }
    }
}

pub fn generate_tree() -> Tree {
    let mut tree = Tree::new();
    
    // Generate root
    let root = Node {
        width: ROOT_WIDTH,
        angle: ROOT_ANGLE,
        length: ROOT_LENGTH,
        level: 0,
        pos: Vec2::new(1280.0 / 2.0, 700.0),
        parent: None,
    };

    let ref_root = tree.add_node(root);

    // Grow tree
    recursive_gen(&mut tree, ref_root);

    tree
}
