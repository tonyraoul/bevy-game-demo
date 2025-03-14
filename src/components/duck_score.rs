use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct DuckScore {
    pub name: String,
    pub value: i32,
}

impl DuckScore {
    pub fn new(name: String) -> Self {
        DuckScore {
            name,
            value: 0,
        }
    }
}