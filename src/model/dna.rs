use bevy::{
    prelude::{Component, Vec3},
    render::mesh::Mesh,
};

use crate::model::gene::Gene;

#[derive(Component, Debug)]
pub struct Dna;

impl Dna {
    pub fn new(twist_degree: f32) -> Vec<(Gene, Mesh)> {
        let mut dna: Vec<(Gene, Mesh)> = Vec::new();
        for i in 0..25 {
            let g1 = Gene::new(
                1.0,
                10.0,
                20,
                20,
                0.6,
                10.0,
                10,
                4,
                twist_degree,
                Vec3::new(0.0, -120.0 + i as f32 * 10.0, 0.0),
                0.0,
            );
            let g2 = Gene::new(
                1.0,
                10.0,
                20,
                20,
                0.6,
                10.0,
                10,
                4,
                twist_degree,
                Vec3::new(0.0, -120.0 + i as f32 * 10.0, 0.0),
                180.0,
            );
            dna.push((g1.clone(), g1.into()));
            dna.push((g2.clone(), g2.into()));
        }
        dna
    }
}
