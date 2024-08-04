use crate::player_position::PlayerPosition;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    name: String,
    number: u8,
    position: Option<PlayerPosition>,
}

impl Player {
    pub fn new(name: String, number: u8) -> Self {
        Self {
            name,
            number,
            position: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn number(&self) -> u8 {
        self.number
    }

    pub fn position(&self) -> Option<PlayerPosition> {
        self.position.clone()
    }

    pub fn set_position(&mut self, position: PlayerPosition) {
        self.position = Some(position);
    }
}

#[cfg(test)]
mod tests {}
