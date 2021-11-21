use std::io;
use std::fs;
use std::error::Error;
use crate::weapon::Weapon;

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
        let damage: u32 = separate_words[2].parse().unwrap();

        final_list.push(Weapon::new(separate_words[0], price, damage));
    }

    Ok(final_list)
} 