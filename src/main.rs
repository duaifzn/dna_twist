use bevy::{input::mouse::MouseWheel, prelude::*, render::mesh::VertexAttributeValues};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;
pub mod model;
use crate::model::dna::Dna;
use crate::model::gene::Gene;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (twist, scroll_dna, border))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dna = Dna::new(30.0);
    for (g, m) in dna {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(m),
                material: materials.add(Color::rgb(0.94, 0.97, 1.0).into()),
                ..default()
            },
            g,
        ));
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(50.0).into()),
        material: materials.add(Color::rgba(0.9, 0.2, 0.3, 0.3).into()),
        transform: Transform::from_xyz(0.0, 50.0, 0.0)
            .with_rotation(Quat::from_rotation_y(PI / 180.0 * 45.0)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(50.0).into()),
        material: materials.add(Color::rgba(0.9, 0.2, 0.3, 0.3).into()),
        transform: Transform::from_xyz(0.0, -50.0, 0.0)
            .with_rotation(Quat::from_rotation_y(PI / 180.0 * 45.0)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(35., 0., 100.),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn twist(
    mut query: Query<(&Handle<Mesh>, &Gene), With<Gene>>,
    mut meshes: ResMut<Assets<Mesh>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::A) {
        for (mesh_handle, gene) in &mut query {
            let mesh = meshes.get_mut(mesh_handle).unwrap();
            if let Some(VertexAttributeValues::Float32x3(position)) =
                mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            {
                for i in 0..position.len() {
                    let new_p =
                        gene.twist_around_y([position[i][0], position[i][1], position[i][2]]);
                    position[i][0] = new_p[0];
                    position[i][1] = new_p[1];
                    position[i][2] = new_p[2];
                }
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::S) {
        for (mesh_handle, gene) in &mut query {
            let mesh = meshes.get_mut(mesh_handle).unwrap();
            if let Some(VertexAttributeValues::Float32x3(position)) =
                mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            {
                for i in 0..position.len() {
                    let new_p = gene.reverse_twist_around_y([
                        position[i][0],
                        position[i][1],
                        position[i][2],
                    ]);
                    position[i][0] = new_p[0];
                    position[i][1] = new_p[1];
                    position[i][2] = new_p[2];
                }
            }
        }
    }
}

fn scroll_dna(mut query: Query<&mut Transform, With<Gene>>, mut scroll: EventReader<MouseWheel>) {
    for s in scroll.read() {
        match s.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                for mut aa in &mut query {
                    aa.translation.y += s.y;
                }
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {}
        }
    }
}
fn border(mut query: Query<&mut Transform, With<Gene>>) {
    for mut a in &mut query {
        if a.translation.y >= 50.0 {
            a.translation.y -= 50.0;
        }
        if a.translation.y <= -50.0 {
            a.translation.y += 50.0;
        }
    }
}
