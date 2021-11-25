#[derive(Debug)]
pub struct Monster {
    name: String,
    damage: u32,
    health: u32,
    money_value: u32,
}

impl Monster {
    pub fn new(
        name: &str,
        damage: u32,
        health: u32,
        money_value: u32
    ) -> Monster {
        Monster { name: name.to_owned(), damage, health, money_value}
    }
}