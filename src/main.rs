extern crate ggez;
use ggez::{graphics, Context, ContextBuilder, GameError, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::Color;
use razor::core::graphics::renderer2d::Renderer2D;
use razor::ecs::entity::Entity;
use razor::ecs::component::{Position, Velocity, Sprite};
use razor::ecs::system::System;

struct MainState {
    renderer: Renderer2D,
    ecs_system: System,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let renderer = Renderer2D::new(ctx)?;
        let mut ecs_system = System::new();
        
        // Adicionando uma entidade com posição, velocidade e sprite
        let entity = Entity::new(1);
        let position = Position { x: 100.0, y: 100.0 };
        let velocity = Velocity { dx: 1.0, dy: 1.0 };
        let sprite = Sprite { texture: "/images/welcome.png".to_string() };
        ecs_system.add_entity(entity, position, velocity, sprite);
        
        Ok(MainState {
            renderer,
            ecs_system,
        })
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        self.ecs_system.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, Color::WHITE);
        self.ecs_system.render(ctx)?;
        self.renderer.draw(ctx)
    }
}

fn main() -> GameResult<()> {
    let (mut ctx, event_loop) = ContextBuilder::new("razor", "Author Name")
        .add_resource_path("assets")
        .build()
        .expect("Falha ao criar o contexto");

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
