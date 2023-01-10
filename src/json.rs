use std::{io::prelude::*, fs};

pub fn save_test(name: &str) -> std::io::Result<()> {
    let list_obj = json::object!{
        name: name
    };
    let content = fs::read_to_string("List.json")
        .expect("Should be able to read the file!");
    let mut list = json::parse(&content).unwrap();
    list.push(list_obj)
        .expect("Should be able to append to file!");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open("List.json")
        .expect("Can't open the file!");
    write!(file, "{}", json::stringify_pretty(list, 2))?;
    Ok(())
}
