use bevy::prelude::{ButtonInput, KeyCode, Query, Res, ResMut, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
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
    let x_movement = movement_speed * time.delta_secs() * game_state.ball_speed_coefficient();
    let y_movement = game_state.ball_angle() * time.delta_secs() * game_state.ball_speed_coefficient();

    match game_state.ball_direction() {
        BallDirection::Left => {
            ball_transform.translation.x -= x_movement;
            ball_transform.translation.y += y_movement;
        },
        BallDirection::Right => {
            ball_transform.translation.x += x_movement;
            ball_transform.translation.y += y_movement;
        },
    };
}

pub fn collision_detected(
    mut game_state: ResMut<GameState>,
    ball_query: Query<&Transform, With<Ball>>,
    player_position_query: Query<(&Player, &Transform), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    // Logic only applies if the ball is moving
    if !game_state.ball_moving() { return; }

    let window = match window_query.single() {
        Ok(window) => window,
        Err(e) => panic!("Could not retrieve window when attempting to check for collisions. Err: {}", e)
    };

    let ball_transform = match ball_query.single() {
        Ok(transform) => transform,
        Err(e) => panic!("Could not retrieve ball transform when attempting to check for collisions. Err: {}", e)
    };

    let x_offset_coefficient = match game_state.ball_direction() {
        BallDirection::Left => -1.0,
        BallDirection::Right => 1.0
    };

    let window_height = window.height() / 2.0;
    let ball_radius = 7.5;

    let ball_y = ball_transform.translation.y;
    let ball_x = ball_transform.translation.x + ball_radius * x_offset_coefficient;

    // Check for top/bottom collision
    let top_collision = ball_y + ball_radius >= window_height && game_state.ball_angle() > 0.0;
    let bottom_collision = ball_y - ball_radius <= -window_height && game_state.ball_angle() < 0.0;
    if top_collision || bottom_collision {
        let inverted_angle = -game_state.ball_angle();
        game_state.set_ball_angle(inverted_angle);
    }

    // Check for paddle collision
    for (player, player_transform) in player_position_query {
        let is_p1 = match player {
            Player::P1 => true,
            Player::P2 => false,
        };

        let paddle_top = player_transform.translation.y + 50.0;
        let paddle_bottom = player_transform.translation.y - 50.0;

        let paddle_x = player_transform.translation.x + 5.0 * -x_offset_coefficient;

        let y_collision = ball_y <= paddle_top && ball_y >= paddle_bottom;
        let x_collision = match game_state.ball_direction() {
            BallDirection::Left => ball_x <= paddle_x && is_p1,
            BallDirection::Right => ball_x >= paddle_x && !is_p1
        };

        let y_center = (paddle_top + paddle_bottom) / 2.0;

        if y_collision && x_collision {
            game_state.toggle_ball_direction();
            if ball_y > y_center {
                game_state.set_ball_angle(45.0);
            } else {
                game_state.set_ball_angle(-45.0);
            }
            game_state.bump_speed_coefficient();
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