use std::io;
use std::fs;
use std::error::Error;
use rand::Rng;
use std::convert::TryInto;
use crate::weapon::Weapon;
use crate::monster::Monster;

pub fn random_number(value: i32) -> usize {
    rand::thread_rng().gen_range(0..value).try_into().unwrap()
}

pub fn random_i32(value: i32) -> i32 {
    rand::thread_rng().gen_range(0..value)
}

pub fn random_monster() -> Monster {
    const MONSTERS: &[(&str, i32, i32, u32)] = &[
        ("Skeleton", 2, 6, 2),
        ("Goblin", 5, 10, 4),
        ("Crabman", 8, 14, 6),
        ("Witch", 11, 18, 8),
        ("Iron Skeleton", 14, 22, 10),
        ("Goblin King", 17, 26, 12),
        ("Slayer", 20, 30, 14),
        ("Foolface", 23, 34, 20)
    ];

    let (name, health, damage, money_value) = MONSTERS[random_number(MONSTERS.len().try_into().unwrap())];
    Monster::new(name, health, damage, money_value)
}

pub fn get_as_usize(value: &str) -> Option<usize>{
    if let Ok(num) = value.trim().parse::<usize>() {
        return Some(num);
    }
    None
}

pub fn get_user_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input) 
}

pub fn get_stock() -> Result<Vec<Weapon>, Box<dyn Error>>{
    let contents = fs::read_to_string("stock.txt")?;
    let mut final_list = Vec::new();

    for line in contents.lines() {
        let separate_words: Vec<&str> = line.split(' ').collect();
        let price: u32 = separate_words[1].parse().unwrap();
        let damage: i32 = separate_words[2].parse().unwrap();

        final_list.push(Weapon::new(separate_words[0], price, damage));
    }

    Ok(final_list)
} 