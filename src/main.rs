use minirpg::player::Player;
use minirpg::weapon::Weapon;
use minirpg::utils::*;

pub fn advance(player: &mut Player) {
    let mut monster = random_monster();
    let mut defense_damage = 0;
    let mut defend_limit = 2;
    println!("You stumble upon a {}", monster.name());

    loop {
        if player.get_health() <= 0 { 
            println!("You've been slayed by {}, no reward for you.", monster.name());
            return;
        }

        if monster.health() <= 0 {
            println!("You've slayed {}, you won ${}", monster.name(), monster.money_value());
            player.add_money(monster.money_value());
            player.reset_health();
            return;
        }

        let option = get_user_input().unwrap();

        match option.to_lowercase().trim() {
            "attack" => {
                println!("You have attacked {}", monster.name());
                monster.take_damage(player.weapon.damage);
            },
            "defend" => {
                if defend_limit == 0 {
                    println!("You can only increase your defence twice!");
                } else {
                    println!("You have defended!");
                    while defense_damage == 0 { defense_damage = random_i32(monster.damage()) }
                    println!(
                        "Damage taken from {} will be reduced by {} for now", 
                        monster.name(), 
                        defense_damage
                    ); 
                    defend_limit -= 1;
                }
            }
            _ => continue
        }

        println!("{} attacks you!", monster.name());
        player.take_damage(monster.damage() - defense_damage);

    }

}

pub fn shop(player: &mut Player) {
    let shop_stock = get_stock().unwrap();

    println!("Welcome to the shop adventurer!"); 

    for (i, val) in shop_stock.iter().enumerate() {
        println!("{}) {} - ${}", i, val.name, val.price);
    }

    println!("Your currency is: {}", player.get_money());

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

        if option.trim().to_lowercase() == "back" { return }
    }

}

fn execute_action(action: &str, player: &mut Player) {
    match action.to_lowercase().trim() {
        "advance" => advance(player),
        "shop" => shop(player),
        _ => println!("Invalid option!")
    }
}

fn main() {
    let mut player = Player::new();
    loop {
        let user_input = get_user_input().unwrap();
        execute_action(&user_input, &mut player);
    }
}
