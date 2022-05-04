mod pong;

use ggez::{conf::WindowMode, event, ContextBuilder};
use pong::{entity::Entity, Pong};
use rand::random;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 500.0;
pub const ENTITY_WIDTH: f32 = 15.0;
pub const ENTITY_HEIGHT: f32 = 75.0;
pub const ENTITY_SPEED: f32 = 5.0;
pub const BALL_SIZE: f32 = 10.0;
pub const BALL_SPEED: f32 = 3.0;

fn main() {
    let x = 15.0;
    let y = (WINDOW_HEIGHT - ENTITY_HEIGHT) / 2.0;

    run_app(Pong {
        player: Entity {
            x,
            y,
            width: ENTITY_WIDTH,
            height: ENTITY_HEIGHT,
        },
        enemy: Entity {
            x: WINDOW_WIDTH - ENTITY_WIDTH - x,
            y,
            width: ENTITY_WIDTH,
            height: ENTITY_HEIGHT,
        },

        ball: Entity {
            x: (WINDOW_WIDTH - BALL_SIZE) / 2.0,
            y: (WINDOW_HEIGHT - BALL_SIZE) / 2.0,
            width: BALL_SIZE,
            height: BALL_SIZE,
        },
        ball_derect: (random::<f32>(), random::<f32>()),
    });
}

fn run_app(pong: Pong) {
    let (ctx, event_loop) = ContextBuilder::new("pong", "Werick Santana")
        .window_mode(WindowMode {
            height: WINDOW_HEIGHT,
            width: WINDOW_WIDTH,
            ..Default::default()
        })
        .build()
        .unwrap();

    event::run(ctx, event_loop, pong)
}
