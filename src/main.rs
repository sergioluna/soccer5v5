use soccer5v5::{Game, Player, PlayerPool};

fn main() {
    let players = PlayerPool::from(vec![
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
    ]);

    let game = Game::from(players);

    println!("{}", game);
}
