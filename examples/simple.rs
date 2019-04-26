use error_gen::ErrorGen; // fast unit errors
use std::{io, str::FromStr};

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Gender {
    pub fn from_cmd() -> io::Result<Self> {
        Ok(
            match read_human::read_choice(
                "What is your gender",
                &["male", "female", "other"],
                None,
            )? {
                0 => Gender::Male,
                1 => Gender::Female,
                2 => Gender::Other,
                _ => unreachable!(),
            },
        )
    }
}

#[derive(Debug, ErrorGen)]
pub struct InvalidName;

#[derive(Debug)]
pub struct Name {
    given: String,
    family: String,
}

impl FromStr for Name {
    type Err = InvalidName;
    // This could be implemented better to handle locales where the family name doesn't come last.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        let mut iter = input
            .char_indices()
            .skip_while(|(_, ch)| !ch.is_whitespace());
        let idx = iter.next().ok_or(InvalidName)?.0;
        let next_idx = iter.next().ok_or(InvalidName)?.0;
        let first = &input[..idx];
        let rest = &input[next_idx..].trim();
        if rest.is_empty() {
            return Err(InvalidName);
        }
        Ok(Name {
            given: first.to_string(),
            family: rest.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct Person {
    name: Name,
    age: u16,
    gender: Gender,
}

fn main() -> Result<(), io::Error> {
    let name = read_human::read_custom_nonempty("What is your name")?;
    let age = read_human::read_custom_nonempty("What is your age")?;
    let gender = Gender::from_cmd()?; // this could have used `FromStr` and `read_custom` instead.
    let person = Person { name, age, gender };
    println!("{:#?}", person);
    Ok(())
}
