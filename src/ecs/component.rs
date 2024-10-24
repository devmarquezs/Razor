pub trait Component {
    fn update(&self);
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    fn update(&self) {
        // Atualize a posição
    }
}

pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Component for Velocity {
    fn update(&self) {
        // Atualize a velocidade
    }
}

pub struct Sprite {
    pub texture: String,
}

impl Component for Sprite {
    fn update(&self) {
        // Atualize a sprite
    }
}

