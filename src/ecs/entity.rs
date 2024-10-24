pub struct Entity {
    id: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Entity { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}