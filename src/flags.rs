use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Flag<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub args: &'a [&'a str],
    pub default: bool,
}

impl<'a> Debug for Flag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result<> {
        write!(f, "{}\t", self.name,)?;
        for arg in self.args {
            write!(f, " {}", arg)?;
        }
        write!(f, "\t{}", self.desc)
    }
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