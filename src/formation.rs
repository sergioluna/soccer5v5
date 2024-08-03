use rand::Rng;

use crate::player_position::PlayerPosition;

#[derive(Debug)]
pub enum Formation {
    Diamond,
    Square,
    Pyramid,
}

impl Formation {
    pub fn get_random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Formation::Diamond,
            1 => Formation::Square,
            _ => Formation::Pyramid,
        }
    }

    pub fn get_player_positions(&self) -> Vec<PlayerPosition> {
        match self {
            Formation::Diamond => {
                vec![
                    PlayerPosition::GK,
                    PlayerPosition::CD,
                    PlayerPosition::LM,
                    PlayerPosition::RM,
                    PlayerPosition::FW,
                ]
            }
            Formation::Square => {
                vec![
                    PlayerPosition::GK,
                    PlayerPosition::DF,
                    PlayerPosition::DF,
                    PlayerPosition::FW,
                    PlayerPosition::FW,
                ]
            }
            Formation::Pyramid => {
                vec![
                    PlayerPosition::GK,
                    PlayerPosition::CD,
                    PlayerPosition::CD,
                    PlayerPosition::CM,
                    PlayerPosition::FW,
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {}
