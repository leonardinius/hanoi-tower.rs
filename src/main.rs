use std::fmt;
use std::fmt::Display;
use std::iter;

struct Tower {
    weight : usize,
}

impl Display for Tower {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("[{0}]", iter::repeat("#").take(self.weight).collect::<String>()))
    }
}

struct Rod {
    stack : Vec<Tower>,
}

struct Desk {
    rods : Vec<Rod>,
}

impl Display for Desk {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let max_weight = 
            self.rods.iter()
                .map(|r| r.stack.iter().map(|s| s.weight).max().unwrap_or(0))
                .max().unwrap_or(0);
        let height = max_weight + 2usize;

        let width = 2 + ((max_weight + 2) * self.rods.len()) + 2;
        let mut display : Vec<String> = Vec::new();

        display.reverse();
        f.write_fmt(format_args!("{0}", display.connect("")))
    }
}

fn main() {
    let t  =  Tower { weight : 10,  };
    println!("Hello, world! {0}", t);
}
