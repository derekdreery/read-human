//! Getting data from a human, from stdin.
//!
//! This library provides methods for getting information from a human. They all work on buffered
//! lines of input (this is how terminals work unless you put them in a different mode). They are
//! useful for building simple interactive command line apps (for example how *pacman* gets
//! confirmantion during a system upgrade on arch linux). They are also useful for learning, when
//! you want to be able to get data easily.
use std::{
    io::{self, Write},
    str::FromStr,
};

/// Get a line of text from the user.
///
/// The question is displayed first. This method converts empty text into `None`. Any whitespace
/// around the input, including the line ending, will be trimmed.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::io;
/// let name = read_human::read_string("Please enter your name: ")?;
/// if let Some(name) = name {
///     // An empty string would have been converted into `None`
///     assert!(name != "");
/// }
/// # Ok::<(), io::Error>(())
/// ```
pub fn read_string(question: &str) -> io::Result<Option<String>> {
    print!("{}: ", question);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(if buf.trim() == "" {
        None
    } else {
        Some(buf.trim().to_owned())
    })
}

/// Get a line of non-empty text from the user.
///
/// The question is displayed first. This method converts empty text into `None`. Any whitespace
/// around the input, including the line ending, will be trimmed.
///
/// This function will keep asking for input until it gets a non-empty string.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::io;
/// let name = read_human::read_string_nonempty("Please enter your name: ")?;
/// // The function will not return until we have a non-empty string
/// assert!(name != "");
/// # Ok::<(), io::Error>(())
/// ```
pub fn read_string_nonempty(question: &str) -> io::Result<String> {
    loop {
        match read_string(question)? {
            Some(s) => return Ok(s),
            None => println!("Input must not be empty."),
        };
    }
}

/// Get a line of from the user without displaying a question first.
///
/// This method converts empty text into `None`. Any whitespace
/// around the input, including the line ending, will be trimmed.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::io;
/// let name = read_human::read_string_noquestion()?;
/// if let Some(name) = name {
///     // An empty string would have been converted into `None`
///     assert!(name != "");
/// }
/// # Ok::<(), io::Error>(())
/// ```
pub fn read_string_noquestion() -> io::Result<Option<String>> {
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(if buf.trim() == "" {
        None
    } else {
        Some(buf.trim().to_owned())
    })
}

/// Allow the user to choose between a set of choices, and optionally give them a default choice.
///
/// The default will be selected if the user just hits enter. If the user writes something that's
/// not one of the choices, the function will ask for another line of input.
///
/// # Panics
///
/// This function will panic if the default value is >= the number of options.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::io;
/// let choice = read_human::read_choice("What is your gender?", &["male", "female"], None)?;
/// // Must be 0 (male) or 1 (female)
/// assert!(choice <= 2);
/// # Ok::<(), io::Error>(())
/// ```
pub fn read_choice(
    question: &str,
    options: &[impl AsRef<str>],
    default: Option<usize>,
) -> io::Result<usize> {
    assert!(
        if let Some(val) = default {
            val < options.len()
        } else {
            true
        },
        "default index must be in the options slice"
    );
    loop {
        print!("{} [", question);
        let mut options_iter = options.iter().enumerate();
        if let Some((_, opt)) = options_iter.next() {
            print!(r#"1: "{}""#, opt.as_ref());
        }
        for (idx, option) in options_iter {
            print!(r#", {}: "{}""#, idx + 1, option.as_ref());
        }
        if let Some(d) = default {
            print!(" (default: {})", d + 1);
        }
        print!("]: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let ans = buf.trim();
        if let Some(val) = default {
            if ans == "" {
                return Ok(val);
            }
        }
        match ans.parse::<usize>() {
            Ok(a) => {
                let ans = a - 1;
                if ans < options.len() {
                    return Ok(ans);
                } else {
                    println!("{} is not a valid option (too big)", ans);
                }
            }
            Err(_) => {
                println!("{} is not a valid option", ans);
            }
        }
    }
}

/// Read in any type that implementd `FromStr` from stdio.
///
/// If the text was empty, or couldn't be converted, then the user will be asked for more input.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::io;
/// let number: u32 = read_human::read_custom_nonempty("How old are you")?;
/// # Ok::<(), io::Error>(())
/// ```
pub fn read_custom_nonempty<T: FromStr>(question: &str) -> io::Result<T> {
    loop {
        let raw = read_string_nonempty(question)?;
        match raw.parse::<T>() {
            Ok(t) => return Ok(t),
            Err(_) => println!("{} is not valid", raw),
        }
    }
}

/// Read in any type that implementd `FromStr` from stdio.
///
/// If the text couldn't be converted, then the user will be asked for more input.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::io;
/// let number: Option<u32> = read_human::read_custom(
///     "Let us know how many times you've visited, if applicable")?;
/// # Ok::<(), io::Error>(())
/// ```
pub fn read_custom<T: FromStr>(question: &str) -> io::Result<Option<T>> {
    loop {
        let raw = match read_string(question)? {
            Some(s) => s,
            None => return Ok(None),
        };
        match raw.parse::<T>() {
            Ok(t) => return Ok(Some(t)),
            Err(_) => println!("{} is not valid", raw),
        }
    }
}

/// Read in any type that implementd `FromStr` from stdio.
///
/// If the text couldn't be converted, then the user will be asked for more input.
///
/// # Examples
///
/// ```rust,no_run
/// # use std::io;
/// let number: Option<u32> = read_human::read_custom_noquestion()?;
/// # Ok::<(), io::Error>(())
/// ```
pub fn read_custom_noquestion<T: FromStr>() -> io::Result<Option<T>> {
    loop {
        let raw = match read_string_noquestion()? {
            Some(s) => s,
            None => return Ok(None),
        };
        match raw.parse::<T>() {
            Ok(t) => return Ok(Some(t)),
            Err(_) => println!("{} is not valid", raw),
        }
    }
}
