use crate::player_pool::PlayerPool;
use crate::team::Team;

#[derive(Debug)]
pub struct Game {
    player_pool: PlayerPool,
    teams: (Team, Team),
}

impl From<PlayerPool> for Game {
    fn from(player_pool: PlayerPool) -> Self {
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

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Team A:")?;
        writeln!(f, "{} Formation", self.teams.0.formation())?;
        for player in self.teams.0.player_pool().players().iter() {
            let position_string = match player.position() {
                Some(p) => format!("{}", p),
                None => format!("N/A"),
            };
            writeln!(
                f,
                "{} ({}): {}",
                player.name(),
                player.number(),
                position_string,
            )?;
        }
        writeln!(f, "")?;
        writeln!(f, "Team B:")?;
        writeln!(f, "{} Formation", self.teams.1.formation())?;
        for player in self.teams.1.player_pool().players().iter() {
            let position_string = match player.position() {
                Some(p) => format!("{}", p),
                None => format!("N/A"),
            };
            writeln!(
                f,
                "{} ({}): {}",
                player.name(),
                player.number(),
                position_string,
            )?;
        }

        Ok(())
    }
}

impl Game {
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
    fn create_game_from_player_pool() {
        let player_pool = get_valid_player_pool();
        let game = Game::from(player_pool);
        assert_eq!(game.player_pool().players().len(), 10);
        assert_eq!(game.teams().0.player_pool().players().len(), 5);
        assert_eq!(game.teams().1.player_pool().players().len(), 5);
    }
}
