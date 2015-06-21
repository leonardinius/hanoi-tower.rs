use std::fmt;
use std::fmt::Display;
use std::iter;

struct Disk {
    weight : usize,
}

impl Disk {
    pub fn new(weight: usize) -> Disk {
        Disk { weight : weight }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}

impl Display for Disk {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("[{0}]", iter::repeat("#").take(self.weight()).collect::<String>()))
    }
}

struct Rod {
    stack : Vec<Disk>,
}

impl Rod {
    pub fn new(sizes : &[usize]) -> Rod {
        Rod { stack : sizes.iter().map(|x| Disk::new(*x) ).collect::<Vec<_>>() }
    }

    fn can_move_disk(&self, disk : &Disk) -> bool {
        match self.stack.last() {
            None => true,
            Some(t) => t.weight() > disk.weight(),
        }
    }

    pub fn can_move(&self, other: &Rod) -> bool {
        false // self.can_take_disk(&other.stack.last())
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
                let tmp = match rod.stack.get(i) {
                    None => format!("{0}|{0}", dup(" ", max_weight + 1)),
                    Some(disk) => format!("{1}[{0}|{0}]{1}", dup("#", disk.weight()), dup(" ", max_weight - disk.weight())),
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
