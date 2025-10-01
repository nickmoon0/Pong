use bevy::prelude::{ButtonInput, KeyCode, Query, Res, ResMut, Time, Transform, With};
use rand::Rng;
use crate::game::game_state::GameState;
use crate::game::pong::{Ball, Player};

pub enum BallDirection {
    Left,
    Right
}

pub fn start_ball(
    mut game_state: ResMut<GameState>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Space) && !game_state.ball_moving() {
        let direction = random_direction();
        game_state.set_ball_direction(direction);
        game_state.toggle_ball_moving();
    }
}

pub fn move_ball(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut ball_query: Query<&mut Transform, With<Ball>>
) {
    // only run logic below if the ball is currently moving
    if !game_state.ball_moving() { return; }

    // Get ball
    let mut ball_transform = match ball_query.single_mut() {
        Ok(transform) => transform,
        Err(e) => panic!("Could not retrieve ball transform when attempting to move ball. Err: {}", e)
    };

    let movement_speed: f32 = 150.;
    let movement = movement_speed * time.delta_secs();

    match game_state.ball_direction() {
        BallDirection::Left => ball_transform.translation.x -= movement,
        BallDirection::Right => ball_transform.translation.x += movement,
    };
}

pub fn collision_detected(
    mut game_state: ResMut<GameState>,
    ball_query: Query<&Transform, With<Ball>>,
    player_position_query: Query<(&Player, &Transform), With<Player>>
) {
    // Logic only applies if the ball is moving
    if !game_state.ball_moving() { return; }

    let ball_transform = match ball_query.single() {
        Ok(transform) => transform,
        Err(e) => panic!("Could not retrieve ball transform when attempting to move ball. Err: {}", e)
    };

    let x_offset_coefficient = match game_state.ball_direction() {
        BallDirection::Left => -1.0,
        BallDirection::Right => 1.0
    };
    for (player, player_transform) in player_position_query {
        let is_p1 = match player {
            Player::P1 => true,
            Player::P2 => false,
        };

        let paddle_top = player_transform.translation.y + 50.0;
        let paddle_bottom = player_transform.translation.y - 50.0;

        let paddle_x = player_transform.translation.x + 5.0 * -x_offset_coefficient;

        let ball_y = ball_transform.translation.y;
        let ball_x = ball_transform.translation.x + (7.5/2.0) * x_offset_coefficient;

        let y_collision = ball_y <= paddle_top && ball_y >= paddle_bottom;
        let x_collision = match game_state.ball_direction() {
            BallDirection::Left => ball_x <= paddle_x && is_p1,
            BallDirection::Right => ball_x >= paddle_x && !is_p1
        };

        if y_collision && x_collision {
            game_state.toggle_ball_direction();
        }
    }
}

fn random_direction() -> BallDirection {
    let mut rng = rand::rng();
    match rng.random_range(0..=1) {
        0 => BallDirection::Left,
        1 => BallDirection::Right,
        _ => panic!("Random number generated in random_direction was not 0 or 1. Invalid value")
    }
}