use crate::ecs::entity::Entity;
use crate::ecs::component::{Position, Velocity, Sprite};
use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Image};

pub struct System {
    entities: Vec<Entity>,
    positions: Vec<Position>,
    velocities: Vec<Velocity>,
    sprites: Vec<Sprite>,
}

impl System {
    pub fn new() -> Self {
        System {
            entities: Vec::new(),
            positions: Vec::new(),
            velocities: Vec::new(),
            sprites: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity, position: Position, velocity: Velocity, sprite: Sprite) {
        self.entities.push(entity);
        self.positions.push(position);
        self.velocities.push(velocity);
        self.sprites.push(sprite);
    }

    pub fn update(&mut self) {
        for (position, velocity) in self.positions.iter_mut().zip(self.velocities.iter()) {
            position.x += velocity.dx;
            position.y += velocity.dy;
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        for (position, sprite) in self.positions.iter().zip(self.sprites.iter()) {
            let image = Image::new(ctx, &sprite.texture)?;
            graphics::draw(ctx, &image, DrawParam::default().dest([position.x, position.y]))?;
        }
        Ok(())
    }
}



