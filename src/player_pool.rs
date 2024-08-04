use rand::seq::SliceRandom;

use crate::player::Player;

#[derive(Debug)]
pub struct PlayerPool {
    players: Vec<Player>,
}

impl From<Vec<Player>> for PlayerPool {
    fn from(players: Vec<Player>) -> Self {
        if players.len() != 10 {
            panic!("players length must be 10");
        }
        Self { players }
    }
}

impl PlayerPool {
    pub fn get_random_split(&self) -> (Self, Self) {
        let mut rng = rand::thread_rng();

        let mut players_copy = self.players.clone();
        players_copy.shuffle(&mut rng);

        let mut group1 = players_copy;
        let group2 = group1.split_off(5);

        (Self { players: group1 }, Self { players: group2 })
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn mut_players(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "players length must be 10")]
    fn player_pool_too_small() {
        let players = vec![Player::new(String::from("Washington"), 1)];
        let _ = PlayerPool::from(players);
    }

    #[test]
    #[should_panic(expected = "players length must be 10")]
    fn player_pool_too_large() {
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
            Player::new(String::from("Polk"), 11),
        ];
        let _ = PlayerPool::from(players);
    }

    #[test]
    fn create_player_pool_from_player_vec() {
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
        let player_pool = PlayerPool::from(players);
        assert_eq!(player_pool.players().len(), 10);
    }

    #[test]
    fn get_random_split() {
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
        let pool = PlayerPool::from(players);
        let pool_split = pool.get_random_split();
        assert_eq!(pool_split.0.players().len(), 5);
        assert_eq!(pool_split.1.players().len(), 5);
        assert_eq!(pool_split.0.players().len(), pool_split.1.players().len());
    }
}
