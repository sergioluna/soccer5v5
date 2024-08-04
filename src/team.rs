use crate::formation::Formation;
use crate::player_pool::PlayerPool;

#[derive(Debug)]
pub struct Team {
    player_pool: PlayerPool,
    formation: Formation,
}

impl From<PlayerPool> for Team {
    fn from(mut player_pool: PlayerPool) -> Self {
        let formation = Formation::get_random();
        let positions = formation.get_player_positions();
        for (i, position) in positions.iter().enumerate() {
            player_pool.mut_players()[i].set_position(position.clone())
        }
        Team {
            player_pool,
            formation,
        }
    }
}

impl Team {
    pub fn player_pool(&self) -> &PlayerPool {
        &self.player_pool
    }

    pub fn formation(&self) -> &Formation {
        &self.formation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    fn get_valid_player_pool() -> PlayerPool {
        let players = vec![
            Player::new(String::from("Washington"), 1),
            Player::new(String::from("Adams"), 2),
            Player::new(String::from("Jefferson"), 3),
            Player::new(String::from("Madison"), 4),
            Player::new(String::from("Monroe"), 5),
            Player::new(String::from("Adams"), 6),
            Player::new(String::from("Jackson"), 7),
            Player::new(String::from("Buren"), 8),
            Player::new(String::from("Harrison"), 9),
            Player::new(String::from("Tyler"), 10),
        ];
        PlayerPool::from(players)
    }

    #[test]
    fn assigns_roles_to_players() {
        let player_pool = get_valid_player_pool();
        let split = player_pool.get_random_split();
        let teams = vec![Team::from(split.0), Team::from(split.1)];
        for team in teams {
            for player in team.player_pool().players() {
                assert_ne!(player.position(), None);
            }
        }
    }
}
