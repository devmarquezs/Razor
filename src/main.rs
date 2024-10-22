extern crate ggez;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::event::{self, EventHandler};
use core::graphics::renderer2d::Renderer2D;

struct MainState{
    renderer: Renderer2D,
}

impl MainState{
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState{
            renderer: Renderer2D::new(),
        };
        Ok(state)
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.renderer.draw(ctx)
    }
}

fn main() -> GameResult<()> {
    let (mut ctx, event_loop) = ContextBuilder::new("razor", "Pablo Marques")
    .build()
    .expect("Falha ao criar Contexto");

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
