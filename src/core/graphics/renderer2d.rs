extern crate ggez;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

pub struct Renderer2D;

impl Renderer2D {
    pub fn new() -> Self {
        Renderer2D
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        //funções de desenho
        graphics::present(ctx)?;
        Ok(())
    }
}