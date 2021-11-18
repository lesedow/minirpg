use std::io;
use std::collections::HashMap;

pub struct Player {
    health: u32,
    weapon: HashMap<String, u32>,
    money: u32,
}

impl Player {
    pub fn new() -> Player {
        let mut weapon = HashMap::new();
        weapon.insert(String::from("Sword"), 2);
        Player { health: 100, weapon, money: 0 }
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn add_money(&mut self, value: u32) {
        self.money += value;
    }

    pub fn subtract_money(&mut self, value: u32) {
        self.money -= value;
    }

    pub fn set_weapon(&mut self, name: &str, damage: u32) {
        let mut new_weapon = HashMap::new();
        new_weapon.insert(String::from(name), damage);
        self.weapon = new_weapon;

        println!("You got a new sword! {}", name);
    }

    pub fn hurt_player(&mut self, damage: u32) {
        self.health -= damage;
        println!("Player got hurt! Health left: {}", self.health);
    }
} 

pub fn get_user_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input) 
}

pub fn advance() {}

pub fn shop(player: &Player) {
    let shop_stock = vec![
        ("Cutlass", 5),
        ("Katana", 10),
        ("Pistol", 15),
        ("AK-47", 20),
        ("Magic Staff", 25),
        ("Fireball", 30),
        ("Banana", 35),
        ("Juan", 40),
        ("Super Idol", 100),
    ];

    println!("Welcome to the shop adventurer!"); 

    for (i, (item, val)) in shop_stock.iter().enumerate() {
        println!("{}) {} - ${}", i, item, val);
    }

    let option = get_user_input().unwrap();

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_buy() {
        let player = Player::new();
    }
}