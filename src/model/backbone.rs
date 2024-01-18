use bevy::prelude::{shape, Mesh};

pub struct Backbone;

impl Backbone {
    pub fn new(radius: f32, height: f32, resolution: u32, segments: u32) -> Mesh {
        shape::Cylinder {
            radius,
            height,
            resolution,
            segments,
        }
        .into()
    }
}
