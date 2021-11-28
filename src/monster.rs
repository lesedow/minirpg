#[derive(Debug)]
pub struct Monster {
    name: String,
    damage: i32,
    health: i32,
    money_value: u32,
}

impl Monster {
    pub fn new(
        name: &str,
        damage: i32,
        health: i32,
        money_value: u32
    ) -> Monster {
        Monster { name: name.to_owned(), damage, health, money_value}
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!("{} took {} damage and has {} health left", self.name, damage, self.health);
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }

    pub fn money_value(&self) -> u32 {
        self.money_value
    }

    pub fn health(&self) -> i32 {
        self.health
    }
}