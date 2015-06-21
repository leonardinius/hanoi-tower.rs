use std::fmt;
use std::fmt::Display;
use std::iter;

struct Tower {
    weight : usize,
}

impl Tower {
    pub fn new(weight: usize) -> Tower {
        Tower { weight : weight }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}

impl Display for Tower {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("[{0}]", iter::repeat("#").take(self.weight()).collect::<String>()))
    }
}

struct Rod {
    stack : Vec<Tower>,
}

impl Rod {
    pub fn new(sizes : &[usize]) -> Rod {
        Rod { stack : sizes.iter().map(|x| Tower::new(*x) ).collect::<Vec<_>>() }
    }

    fn can_take_tower(&self, tower : &Tower) -> bool {
        match self.stack.last() {
            None => true,
            Some(t) => t.weight() > tower.weight(),
        }
    }

    pub fn can_take(&self, other: &Rod) -> bool {
        false // self.can_take_tower(&other.stack.last())
    }

    pub fn swap(& mut self, other: &mut Rod) -> &mut Self {
        self
    }

}

struct Desk {
    rods : Vec<Rod>,
}

impl Desk {
    pub fn new(rods: &[&[usize]]) -> Desk {
        Desk { rods : rods.iter().map(|x| Rod::new(*x) ).collect::<Vec<_>>() }
    }
}

impl Display for Desk {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let max_weight : usize =
            self.rods.iter()
                .map(|r| r.stack.iter().map(|s| s.weight()).max().unwrap_or(0))
                .max().unwrap_or(0);
        let height = max_weight + 1;

        let dup = |s: &str, n: usize| iter::repeat(s).take(n).collect::<String>();
        let mut display : Vec<String> = Vec::new();

        for i in 0..height {
            let mut s : String = "  ".to_string();

            for rod in &self.rods {
                let stower: Option<&Tower> = rod.stack.get(i);
                let tmp = match stower {
                    None => format!("{0}|{0}", dup(" ", max_weight + 1)),
                    Some(tower) => format!("{1}[{0}|{0}]{1}", dup("#", tower.weight()), dup(" ", max_weight - tower.weight())),
                };
                s.push_str(" ");
                s.push_str(&tmp);
            }

            s.push_str("  ");
            s.push_str("\n");
            display.push(s);
        }

        display.reverse();
        f.write_fmt(format_args!("{0}", display.connect("")))
    }
}

fn main() {
    let desk1 = Desk::new(&[
        &[4, 3],
        &[2, 1]
    ]);
    println!("{0}", desk1);

}
