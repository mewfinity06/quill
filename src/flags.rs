use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub struct Flag<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub args: &'a [&'a str],
    pub default: bool,
}

#[allow(dead_code)]
impl<'a> Flag<'a> {
    /// Creates a new Flag instance
    pub fn new(name: &'a str, desc: &'a str, args: &'a [&'a str], default: bool) -> Self {
        Flag {
            name,
            desc,
            args,
            default,
        }
    }

    /// Checks if the given string is one of the flag's arguments
    pub fn is_in(&self, s: &str) -> bool {
        self.args.iter().any(|&arg| arg == s)
    }

    /// Gets the name of the flag
    pub fn get_name(self) -> &'a str {
        self.name
    }
}

/// Prints all flags in the provided slice
pub fn print_flags(flags: &[Flag]) {
    for flag in flags {
        println!("    {:?}", flag);
    }
}
