use std::io;

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u16,
    gender: Gender,
}

fn main() -> Result<(), io::Error> {
    let name = read_human::read_string_nonempty("What is your name")?;
    let age = read_human::read_custom_nonempty("What is your age")?;
    let gender =
        match read_human::read_choice("What is your gender", &["male", "female", "other"], None)? {
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::Other,
            _ => unreachable!(),
        };
    let person = Person { name, age, gender };
    println!("{:?}", person);
    Ok(())
}
