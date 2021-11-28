use crate::weapon::Weapon;

pub struct Player {
    health: i32,
    pub weapon: Weapon,
    money: u32,
}

impl Player {
    pub fn new() -> Player {
        let weapon = Weapon::new("Sword", 0, 2);
        Player { health: 100, weapon, money: 0 }
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn reset_health(&mut self) {
        self.health = 100;
    }

    pub fn add_money(&mut self, value: u32) {
        self.money += value;
    }

    pub fn get_money(&self) -> u32 {
        self.money
    }

    fn subtract_money(&mut self, value: u32) {
        self.money -= value;
    }

    fn has_enough_money(&self, value: u32) -> bool {
        self.money >= value
    }

    pub fn set_weapon(&mut self, weapon: Weapon) {
        if self.has_enough_money(weapon.price) {
            self.subtract_money(weapon.price);
            println!("You got a new sword! {}", weapon.name);
            self.weapon = weapon;

            return;
        }
        
        println!("You dont have enough money!");
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!("You have {} health left", self.health);
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy() {
        let mut test_player = Player::new();
        test_player.add_money(100);

        let test_weapon = Weapon::new("Bababoey", 100, 100);
        test_player.set_weapon(test_weapon);

        assert_eq!(test_player.weapon.name, "Bababoey");
    }

    #[test]
    fn not_enough_money() {
        let mut test_player = Player::new();

        let test_weapon = Weapon::new("Bababoey", 100, 100);
        test_player.set_weapon(test_weapon);

        assert_eq!(test_player.weapon.name, "Sword");
    }
}
