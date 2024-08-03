use crate::formation::Formation;
use crate::player_pool::PlayerPool;

#[derive(Debug)]
pub struct Team {
    player_pool: PlayerPool,
    formation: Formation,
}

impl From<PlayerPool> for Team {
    fn from(player_pool: PlayerPool) -> Self {
        Team {
            player_pool,
            formation: Formation::get_random(),
        }
    }
}

impl Team {
    pub fn player_pool(&self) -> &PlayerPool {
        &self.player_pool
    }
}

#[cfg(test)]
mod tests {}
