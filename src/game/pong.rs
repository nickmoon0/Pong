use std::process::exit;
use bevy::prelude::{App, Update, Plugin, Startup, ColorMaterial, Commands, Camera2d, Component, Transform, Mesh2d, ResMut, Assets, Mesh, Rectangle, MeshMaterial2d, Color, Query, Window, With, Res, Time, ButtonInput, KeyCode, Circle, IntoScheduleConfigs, info};
use bevy::window::PrimaryWindow;
use crate::game::game_state::GameState;
use crate::game::paddle::Paddle;
use crate::game::physics::{start_ball, collision_detected, move_ball, BallDirection};

#[derive(Component)]
pub enum Player {
    P1,
    P2
}

#[derive(Component)]
pub struct Ball;

pub struct Pong;

impl Plugin for Pong {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::new());
        app.add_systems(Startup, (setup_players, setup_ball, setup_board));
        app.add_systems(Update, (
            start_ball,
            (handle_controls, collision_detected, move_ball, player_scored).chain()
        ));
    }
}

fn setup_board(
    mut commands: Commands
) {
    // Spawn camera
    commands.spawn(Camera2d);
}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(7.5))),
        MeshMaterial2d(materials.add(Color::hsl(0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ball
    ));
}

//noinspection DuplicatedCode
fn setup_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    // Get window dimensions for spawning entities in the correct location
    let window = match window_query.single() {
        Ok(window) => window,
        Err(e) => {
            eprintln!("Could not get a single window. Err: {}", e);
            exit(1);
        }
    };
    let width = window.width();

    // Spawn player paddles at left and right screen edges (world coordinates are centered)
    let paddle_width = 10.0;
    let paddle_height = 100.0;
    let margin = 10.0;
    let half_width = width / 2.0;

    let left_x = -half_width + margin + paddle_width / 2.0;
    let right_x = half_width - margin - paddle_width / 2.0;

    commands.spawn(Paddle::new(
        Player::P1,
        Mesh2d(meshes.add(Rectangle::new(paddle_width, paddle_height))),
        MeshMaterial2d(materials.add(Color::hsl(0.0, 0.0, 1.0))),
        Transform::from_xyz(left_x, 0.0, 0.0)
    ));

    commands.spawn(Paddle::new(
        Player::P2,
        Mesh2d(meshes.add(Rectangle::new(paddle_width, paddle_height))),
        MeshMaterial2d(materials.add(Color::hsl(0.0, 0.0, 1.0))),
        Transform::from_xyz(right_x, 0.0, 0.0)
    ));
}

fn player_scored(
    mut game_state: ResMut<GameState>,
    ball_position_query: Query<&mut Transform, With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !game_state.ball_moving() { return; }

    let window = match window_query.single() {
        Ok(window) => window,
        Err(e) => panic!("Failed to get window when determining if player scored. Err: {}", e)
    };

    let ball_transform = match ball_position_query.single() {
        Ok(transform) => transform,
        Err(e) => panic!("Failed to get ball when determining if player scored. Err: {}", e)
    };

    let width = window.width() / 2.0;
    match game_state.ball_direction() {
        BallDirection::Left => {
            let ball_x_offset_pos = ball_transform.translation.x + 5.0;
            let out_of_bounds = ball_x_offset_pos <= -width;
            if out_of_bounds {
                game_state.inc_p2_score();
                info!("P1 Score: {}, P2 Score: {}", game_state.p1_score(), game_state.p2_score());
                reset_game(game_state, ball_position_query);
            }
        },
        BallDirection::Right => {
            let ball_x_offset_pos = ball_transform.translation.x - 5.0;
            let out_of_bounds = ball_x_offset_pos >= width;
            if out_of_bounds {
                game_state.inc_p1_score();
                info!("P1 Score: {}, P2 Score: {}", game_state.p1_score(), game_state.p2_score());
                reset_game(game_state, ball_position_query);
            }
        },
    };
}

fn reset_game(
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<&mut Transform, With<Ball>>
) {
    game_state.toggle_ball_moving();
    let mut ball_transform = match ball_query.single_mut() {
        Ok(transform) => transform,
        Err(e) => panic!("Failed to get ball when resetting game. Err: {}", e)
    };

    ball_transform.translation.x = 0.0;
    ball_transform.translation.y = 0.0;
}

fn handle_controls(
    time: Res<Time>,
    player_position_query: Query<(&Player, &mut Transform), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Set keys for player movement
    let p1_up = KeyCode::KeyW;
    let p1_down = KeyCode::KeyS;
    let p2_up = KeyCode::ArrowUp;
    let p2_down = KeyCode::ArrowDown;

    let movement_speed = 300.;

    // Set window height variables
    let window = match window_query.single() {
        Ok(window) => window,
        Err(e) => {
            eprintln!("Could not get a single window. Err: {}", e);
            exit(1);
        }
    };

    let height = window.height() / 2.0;
    let paddle_height: f32 = 100.0;

    for (player, mut transform) in player_position_query {
        let up: KeyCode;
        let down: KeyCode;

        // Determine which keys should be used for up and down based on the current player
        match player {
            Player::P1 => {
                up = p1_up;
                down = p1_down;
            },
            Player::P2 => {
                up = p2_up;
                down = p2_down;
            }
        };

        let movement = movement_speed * time.delta_secs();
        let can_move_up = transform.translation.y + movement <= height - paddle_height / 2.0;
        let can_move_down = transform.translation.y - movement >= -height + paddle_height / 2.0;

        if keyboard.pressed(up) && can_move_up {
            transform.translation.y += movement;
        } else if keyboard.pressed(down) && can_move_down {
            transform.translation.y -= movement;
        }
    }
}