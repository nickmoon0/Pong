use bevy::prelude::{Bundle, ColorMaterial, Mesh2d, MeshMaterial2d, Transform};
use crate::game::pong::Player;

#[derive(Bundle)]
pub struct Paddle {
    player: Player,
    mesh: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

impl Paddle {
    pub fn new(
        player: Player,
        mesh: Mesh2d,
        mesh_material: MeshMaterial2d<ColorMaterial>,
        transform: Transform,
    ) -> Paddle {
        Paddle {
            player,
            mesh,
            mesh_material,
            transform,
        }
    }
}