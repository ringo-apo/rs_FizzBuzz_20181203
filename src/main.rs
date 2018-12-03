pub struct Fizz {
    values: [Option<&'static str>; 3],
}

impl Fizz {
    pub fn new() -> Fizz {
        Fizz { values: [Some("Fizz"), None, None] }
    }

    fn get(&self, i: usize) -> Option<&'static str> {
        self.values[i%3]
    }
}

pub struct Buzz {
    values: [Option<&'static str>; 5],
}

impl Buzz {
    pub fn new() -> Buzz {
        Buzz { values: [Some("Buzz"), None, None, None, None] }
    }
    fn get(&self, i: usize) -> Option<&'static str> {
        self.values[i%5]
    }
}

fn main() {
    let fizz = Fizz::new();
    let buzz = Buzz::new();

    for i in 1..50 {
        match (fizz.get(i), buzz.get(i)) {
            (None, None) => println!("{}", i),
            (fizz, buzz) => println!("{}{}", fizz.unwrap_or_default(), buzz.unwrap_or_default()),
        }
    }
}