use bevy::prelude::Resource;
use crate::game::physics::BallDirection;

#[derive(Resource)]
pub struct GameState {
    p1_score: u16,
    p2_score: u16,
    ball_moving: bool,
    ball_direction: BallDirection,
    ball_angle: f32,
    ball_speed_coefficient: f32
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            p1_score: 0,
            p2_score: 0,
            ball_moving: false,
            ball_direction: BallDirection::Left, // default value, it will be overwritten on actual start
            ball_angle: 0.0,
            ball_speed_coefficient: 1.0
        }
    }

    pub fn p1_score(&self) -> u16 {
        self.p1_score
    }

    pub fn p2_score(&self) -> u16 {
        self.p2_score
    }

    pub fn ball_moving(&self) -> bool {
        self.ball_moving
    }

    pub fn ball_direction(&self) -> &BallDirection {
        &self.ball_direction
    }

    pub fn ball_angle(&self) -> f32 {
        self.ball_angle
    }

    pub fn ball_speed_coefficient(&self) -> f32 {
        self.ball_speed_coefficient
    }

    pub fn inc_p1_score(&mut self) {
        self.p1_score += 1;
    }

    pub fn inc_p2_score(&mut self) {
        self.p2_score += 1;
    }

    pub fn toggle_ball_moving(&mut self) {
        self.ball_moving = !self.ball_moving;
    }

    pub fn toggle_ball_direction(&mut self) {
        match self.ball_direction {
            BallDirection::Right => self.ball_direction = BallDirection::Left,
            BallDirection::Left => self.ball_direction = BallDirection::Right
        };
    }

    pub fn set_ball_direction(&mut self, direction: BallDirection) {
        self.ball_direction = direction;
    }

    pub fn set_ball_angle(&mut self, angle: f32) {
        self.ball_angle = angle;
    }

    pub fn bump_speed_coefficient(&mut self) {
        self.ball_speed_coefficient += 0.2;
    }
}