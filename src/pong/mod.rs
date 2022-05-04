pub mod entity;

use ggez::{
    event::{EventHandler, KeyCode},
    graphics::{self, Color, DrawMode, DrawParam, Rect, Mesh},
    input::keyboard,
    GameError,
};

use self::entity::Entity;
use crate::{ENTITY_SPEED, WINDOW_HEIGHT, ENTITY_HEIGHT, BALL_SPEED, BALL_SIZE};

pub struct Pong {
    pub player: Entity,
    pub enemy: Entity,
    pub ball: Entity,
    pub ball_derect: (f32, f32),
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        // keu code player
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player.y -= ENTITY_SPEED;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player.y += ENTITY_SPEED;
        }

        // IA for enemy 
        self.enemy.y += (self.ball.y - self.enemy.y - (BALL_SIZE / 2.0)) * 0.18;

        collition_window(&mut self.player);
        collition_window(&mut self.enemy);

        collition_ball_window(&self.ball, &mut self.ball_derect);
        self.ball.x += self.ball_derect.0 * BALL_SPEED;
        self.ball.y += self.ball_derect.1 * BALL_SPEED;


        if self.ball.x + BALL_SIZE >= self.enemy.x {
            self.ball_derect.0 *= -1.0;
        } else if self.ball.x < self.player.x + self.player.width {
            self.ball_derect.0 *= -1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        graphics::clear(ctx, Color::from_rgb(172, 23, 199));
        let rect_player = draw_rect(ctx, &mut self.player)?;
        let rect_enemy = draw_rect(ctx, &mut self.enemy)?;
        let rect_ball = draw_rect(ctx, &mut self.ball)?;

        graphics::draw(ctx, &rect_player, DrawParam::default())?;
        graphics::draw(ctx, &rect_enemy, DrawParam::default())?;
        graphics::draw(ctx, &rect_ball, DrawParam::default())?;

        graphics::present(ctx)
    }
}

fn draw_rect(ctx: &mut ggez::Context, entity: &mut Entity) -> Result<Mesh, GameError>{
    graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(
            entity.x,
            entity.y,
            entity.width,
            entity.height,
        ),
        Color::BLACK,
    )
}

fn collition_window(entity: &mut Entity) {
    if entity.y + entity.height > WINDOW_HEIGHT {
        entity.y = WINDOW_HEIGHT - ENTITY_HEIGHT;
    }
    if entity.y < 0.0 {
        entity.y = 0.0;
    }
}

fn collition_ball_window(entity: &Entity, direct: &mut (f32, f32)) {
    if entity.y + (direct.1 * BALL_SPEED) + entity.height >= WINDOW_HEIGHT {
        direct.1 *= -1.0
    }
    if entity.y + (direct.1 * BALL_SPEED) + entity.height < 0.0 {
        direct.1 *= -1.0
    }
}