use bevy::prelude::*;

#[derive(Component)]
pub struct BearScore {
    pub value: i32,
    pub name: String,
}

impl BearScore {
    pub fn new(name: String) -> Self {
        Self {
            value: 10,  // Starting score
            name,
        }
    }
} 