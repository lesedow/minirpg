pub struct Weapon {
    pub name: String,
    pub price: u32,
    pub damage: i32
}

impl Weapon {
    pub fn new(name: &str, price: u32, damage: i32) -> Weapon {
        Weapon {
            name: name.to_string(), 
            price,
            damage
        }
    }

}
