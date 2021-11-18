use minirpg::Player;

fn execute_action(action: &str, player: &Player) {
    match action.to_lowercase().trim() {
        "advance" => minirpg::advance(),
        "shop" => minirpg::shop(player),
        _ => println!("Invalid option!")
    }
}

fn main() {

    let mut player = Player::new();
    
    loop {
        if player.get_health() == 0 { break }

        let user_input = minirpg::get_user_input().unwrap();
        execute_action(&user_input, &player);

    }
}
