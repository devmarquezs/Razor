#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(1);
        assert_eq!(entity.id(), 1);
    }

    #[test]
    fn test_position_component() {
        let position = Position { x: 1.0, y: 2.0 };
        position.update();
        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 2.0);
    }

    #[test]
    fn test_velocity_component() {
        let velocity = Velocity { dx: 1.0, dy: 2.0 };
        velocity.update();
        assert_eq!(velocity.dx, 1.0);
        assert_eq!(velocity.dy, 2.0);
    }

    #[test]
    fn test_system_update() {
        let mut system = System::new();
        let entity = Entity::new(1);
        let position = Position { x: 0.0, y: 0.0 };
        let velocity = Velocity { dx: 1.0, dy: 1.0 };
        system.add_entity(entity, position, velocity);
        system.update();

        assert_eq!(system.positions[0].x, 1.0);
        assert_eq!(system.positions[0].y, 1.0);
    }
}
