use anyhow::anyhow;
use malachite::{
    Integer,
    base::num::{
        basic::traits::{One, Two, Zero},
        conversion::traits::{FromSciString, FromStringBase},
    },
};
use std::{io, io::Write};

fn input(prompt: &str) -> anyhow::Result<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    Ok(n.trim().to_string())
}

fn starts_with(string: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|&prefix| string.starts_with(prefix))
}

fn bigint(n: String) -> anyhow::Result<Integer> {
    let parsed = if starts_with(&n, &["0x", "0X"]) {
        Integer::from_string_base(16, &n[2..])
    } else {
        Integer::from_sci_string(&n)
    };

    parsed.ok_or_else(|| anyhow!("Invalid integer format"))
}

const FOUR: Integer = Integer::const_from_unsigned(4);

struct Catalan {
    n: Integer,
    c: Integer,
}

impl Catalan {
    fn new() -> Self {
        Self {
            n: Integer::ZERO,
            c: Integer::ONE,
        }
    }

    fn next(&mut self) -> &Integer {
        self.n += Integer::ONE;
        self.c = &self.c * (FOUR * &self.n - Integer::TWO) / (&self.n + Integer::ONE);

        &self.c
    }
}

fn main() -> anyhow::Result<()> {
    println!("This program finds the first n Catalan numbers.");

    let n = bigint(input("Enter n :\t")?)
        .map_err(|_| anyhow!("Unable to parse integer"))?;

    let mut c = Catalan::new();
    while c.n < n {
        println!("Cat({})\t= {}", c.n, c.c);
        c.next();
    }

    Ok(())
}
