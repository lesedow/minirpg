use minirpg::player::Player;
use minirpg::weapon::Weapon;
use minirpg::utils::*;

pub fn advance() {}

pub fn shop(player: &mut Player) {
    let shop_stock = get_stock().unwrap();

    println!("Welcome to the shop adventurer!"); 

    for (i, val) in shop_stock.iter().enumerate() {
        println!("{}) {} - ${}", i, val.name, val.price);
    }

    loop {
        let option = get_user_input().unwrap();
        if let Some(num) = get_as_usize(&option) {
            if num >= shop_stock.len() { 
                println!("Invalid shop item!");
                continue; 
            }

            let weapon = &shop_stock[num];
            player.set_weapon(Weapon::new(&weapon.name, weapon.price, weapon.damage));

        }
    }

}

fn execute_action(action: &str, player: &mut Player) {
    match action.to_lowercase().trim() {
        "advance" => advance(),
        "shop" => shop(player),
        _ => println!("Invalid option!")
    }
}

fn main() {

    let mut player = Player::new();
    
    loop {
        if player.get_health() == 0 { break }

        let user_input = get_user_input().unwrap();
        execute_action(&user_input, &mut player);

    }
}
