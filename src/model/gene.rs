use crate::model::backbone::Backbone;
use crate::model::base::Base;
use bevy::{
    prelude::{shape::Quad, Component, Mesh, Quat, Transform, Vec3},
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use std::f32::consts::PI;

#[derive(Component, Debug, Clone)]
pub struct Gene {
    pub backbone_radius: f32,
    pub backbone_height: f32,
    pub backbone_resolution: u32,
    pub backbone_segments: u32,
    pub backbone_position_len: u32,
    pub base_radius: f32,
    pub base_height: f32,
    pub base_resolution: u32,
    pub base_segments: u32,
    pub twist_degree: f32,
    pub transform: Vec3,
    pub rotate_y_degree: f32,
}

impl Gene {
    pub fn new(
        backbone_radius: f32,
        backbone_height: f32,
        backbone_resolution: u32,
        backbone_segments: u32,
        base_radius: f32,
        base_height: f32,
        base_resolution: u32,
        base_segments: u32,
        twist_degree: f32,
        transform: Vec3,
        rotate_y_degree: f32,
    ) -> Self {
        Self {
            backbone_radius,
            backbone_height,
            backbone_resolution,
            backbone_segments,
            backbone_position_len: (backbone_resolution + 1) * (backbone_segments + 1)
                + backbone_resolution * 2,
            base_radius,
            base_height,
            base_resolution,
            base_segments,
            twist_degree,
            transform,
            rotate_y_degree,
        }
    }
    pub fn twist_around_y(&self, position: [f32; 3]) -> [f32; 3] {
        let mut twisted_position: [f32; 3] = [0.0; 3];
        let step_degree = self.twist_degree / self.backbone_segments as f32;
        let theta = PI / 180 as f32
            * step_degree
            * (position[1] / self.backbone_height * self.backbone_segments as f32).ceil();
        let (sin, cos) = theta.sin_cos();
        let rotate_metrix = [[cos, 0.0, sin], [0.0, 1.0, 0.0], [-sin, 0.0, cos]];
        for i in 0..3 {
            for j in 0..3 {
                twisted_position[i] += rotate_metrix[i][j] * position[j]
            }
        }
        twisted_position
    }
    pub fn reverse_twist_around_y(&self, position: [f32; 3]) -> [f32; 3] {
        let mut twisted_position: [f32; 3] = [0.0; 3];
        let step_degree = -self.twist_degree / self.backbone_segments as f32;
        let theta = PI / 180 as f32
            * step_degree
            * (position[1] / self.backbone_height * self.backbone_segments as f32).ceil();
        let (sin, cos) = theta.sin_cos();
        let rotate_metrix = [[cos, 0.0, sin], [0.0, 1.0, 0.0], [-sin, 0.0, cos]];
        for i in 0..3 {
            for j in 0..3 {
                twisted_position[i] += rotate_metrix[i][j] * position[j]
            }
        }
        twisted_position
    }
}

impl From<Gene> for Mesh {
    fn from(v: Gene) -> Self {
        let backbone = Backbone::new(
            v.backbone_radius,
            v.backbone_height,
            v.backbone_resolution,
            v.backbone_segments,
        );
        let base = Base::new(
            v.base_radius,
            v.base_height,
            v.base_resolution,
            v.base_segments,
        );
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut base_indices_offset = 0;

        if let Some(VertexAttributeValues::Float32x3(backbone_position)) =
            backbone.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            base_indices_offset = backbone_position.len() as u32;
            let backbone_transform =
                Transform::from_rotation(Quat::from_rotation_y(PI / 180.0 * v.rotate_y_degree))
                    .mul_transform(Transform::from_xyz(
                        v.transform.x,
                        v.transform.y,
                        v.transform.z,
                    ));
            let mat = backbone_transform.compute_matrix();
            for p in backbone_position {
                positions.push(
                    mat.transform_point3(Vec3::new(p[0] + v.base_height, p[1], p[2]))
                        .into(),
                )
            }
        } else {
            panic!("Do not have backbone position")
        }
        if let Some(VertexAttributeValues::Float32x3(base_position)) =
            base.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let base_transform = Transform::from_rotation(Quat::from_rotation_z(-PI / 2.0))
                .mul_transform(Transform::from_rotation(Quat::from_rotation_x(-PI / 180.0 * v.rotate_y_degree)))
                .mul_transform(Transform::from_xyz(
                    -v.transform.y,
                    v.base_height / 2.0 + v.transform.x,
                    v.transform.z,
                ));
            let mat = base_transform.compute_matrix();
            for p in base_position {
                positions.push(mat.transform_point3(Vec3::from(*p)).into())
            }
        } else {
            panic!("Do not have base position")
        }

        if let Some(VertexAttributeValues::Float32x3(backbone_normals)) =
            backbone.attribute(Mesh::ATTRIBUTE_NORMAL)
        {
            for n in backbone_normals {
                normals.push([n[0], n[1], n[2]]);
            }
        } else {
            panic!("Do not have backbone normals")
        }
        if let Some(VertexAttributeValues::Float32x3(base_normals)) =
            base.attribute(Mesh::ATTRIBUTE_NORMAL)
        {
            for n in base_normals {
                normals.push([n[0], n[1], n[2]]);
            }
        } else {
            panic!("Do not have base normals")
        }

        if let Some(Indices::U32(backbone_indices)) = backbone.indices() {
            for i in backbone_indices {
                indices.push(*i)
            }
        } else {
            panic!("Do not have backbone indices")
        }
        if let Some(Indices::U32(base_indices)) = base.indices() {
            for i in base_indices {
                indices.push(*i + base_indices_offset)
            }
        } else {
            panic!("Do not have base indices")
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
