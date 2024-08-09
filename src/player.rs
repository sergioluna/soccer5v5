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
mod tests {
    use super::*;

    #[test]
    fn get_name() {
        let player = Player::new(String::from("Washington"), 1);
        assert_eq!(*player.name(), String::from("Washington"));
    }

    #[test]
    fn get_number() {
        let player = Player::new(String::from("Washington"), 1);
        assert_eq!(player.number(), 1);
    }

    #[test]
    fn get_position() {
        let player = Player::new(String::from("Washington"), 1);
        assert_eq!(player.position(), None);
    }

    #[test]
    fn set_position() {
        let mut player = Player::new(String::from("Washington"), 1);
        player.set_position(PlayerPosition::GK);
        assert_eq!(player.position(), Some(PlayerPosition::GK));
    }
}
