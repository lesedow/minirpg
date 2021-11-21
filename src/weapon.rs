pub struct Weapon {
    pub name: String,
    pub price: u32,
    pub damage: u32
}

impl Weapon {
    pub fn new(name: &str, price: u32, damage: u32) -> Weapon {
        Weapon {
            name: name.to_string(), 
            price,
            damage
        }
    }

}
