use crate::player_pool::PlayerPool;
use crate::team::Team;

#[derive(Debug)]
pub struct Game<'a> {
    player_pool: &'a PlayerPool,
    teams: (Team, Team),
}

impl<'a> From<&'a PlayerPool> for Game<'a> {
    fn from(player_pool: &'a PlayerPool) -> Self {
        if player_pool.players().len() != 10 {
            panic!("Game players length must be 10");
        }

        let groups = player_pool.get_random_split();

        Self {
            player_pool,
            teams: (Team::from(groups.0), Team::from(groups.1)),
        }
    }
}

impl<'a> std::fmt::Display for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Team A:")?;
        for player in self.teams.0.player_pool().players().iter() {
            writeln!(f, "{} ({})", player.name(), player.number())?;
        }
        writeln!(f, "")?;
        writeln!(f, "Team B:")?;
        for player in self.teams.1.player_pool().players().iter() {
            writeln!(f, "{} ({})", player.name(), player.number())?;
        }

        Ok(())
    }
}

impl<'a> Game<'a> {
    pub fn player_pool(&self) -> &PlayerPool {
        &self.player_pool
    }

    pub fn teams(&self) -> &(Team, Team) {
        &self.teams
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn create_game_from_player_pool() {
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
        let game = Game::from(&player_pool);
        assert_eq!(game.player_pool().players().len(), 10);
        assert_eq!(game.teams().0.player_pool().players().len(), 5);
        assert_eq!(game.teams().1.player_pool().players().len(), 5);
    }

    #[test]
    fn game_assigns_players_to_teams() {
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
        let game = Game::from(&player_pool);
        assert_eq!(game.teams().0.player_pool().players().len(), 5);
        assert_eq!(game.teams().1.player_pool().players().len(), 5);
    }

    #[test]
    #[ignore]
    fn game_display() {
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
        let game = Game::from(&player_pool);
    }
}
