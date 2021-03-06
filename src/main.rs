#![feature(slice_patterns)]

use std::fmt;
use std::fmt::Display;
use std::iter;
use std::io;

#[macro_use]
extern crate log;


#[derive(Debug)]
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

#[derive(Debug)]
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
        match other.stack.last() {
            None => false,
            Some(ref disk) => self.can_move_disk(disk)
        }
    }

    pub fn take_from(& mut self, other: &mut Rod) -> bool {
        if self.can_move(other) {
            trace!("Will take {0:?} <- {1:?}", self, other);
            self.stack.push(other.stack.pop().expect("can_move returned true"));
            true
        } else {
            false
        }
    }

}

struct Desk {
    rods : Vec<Rod>,
}

impl Desk {
    pub fn new(rods: &[&[usize]]) -> Desk {
        Desk { rods : rods.iter().map(|x| Rod::new(*x) ).collect::<Vec<_>>() }
    }

    pub fn new_default(count : usize) -> Desk {
        let tmp = (1 .. count+1).rev().collect::<Vec<usize>>();

        let first: &[usize] = &tmp[ .. ];
        Desk::new(&[
            &first,
            &[],
            &[]
        ])
    }

    pub fn is_done(&self) -> bool {
        self.rods.iter().take(self.rods.len() -1).all(|r| r.stack.is_empty())
    }

    pub fn move_disk(&mut self, from: usize, to: usize) -> bool {
        let from_rod;
        let to_rod;

        if from < to && to < self.rods.len() {
            let (x, y) = self.rods.split_at_mut(to);

            from_rod = x.get_mut(from);
            to_rod = y.get_mut(0);
        } else if to < from && from < self.rods.len() {
            let (x, y) = self.rods.split_at_mut(from);

            from_rod = y.get_mut(0);
            to_rod = x.get_mut(to);
        } else {
            from_rod = None;
            to_rod = None;
        }

        match (to_rod, from_rod) {
            (Some(a), Some(b)) => a.take_from(b),
            _ => false,
        }
    }
}

impl Display for Desk {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let max_weight : usize =
            self.rods.iter()
                .map(|r| r.stack.iter().map(|s| s.weight()).max().unwrap_or(0))
                .max().unwrap_or(0);
        let height = max_weight + 2;

        let dup = |s: &str, n: usize| iter::repeat(s).take(n).collect::<String>();
        let mut display : Vec<String> = Vec::new();

        for i in 0..height {
            let mut s : String = "  ".to_string();

            for rod_index in (0..self.rods.len()) {
                let rod = &self.rods[rod_index];
                let tmp = match rod.stack.get(i) {
                    None => format!("{0}{1}{0}", dup(" ", max_weight + 1), if i == height -1 {(rod_index+1).to_string()} else {"|".to_string()}),
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
        display.push(dup("-", 2 + 2 + (self.rods.len() +1) * max_weight * 2) + "\n");
        f.write_fmt(format_args!("{0}", display.connect("")))
    }
}

fn read_move() ->(usize, usize) {
    let buffer = &mut String::with_capacity(32);
    let _ = io::stdin().read_line(buffer)
        .ok()
        .expect("Failed to read line");

    let splits = buffer.trim().split(' ').collect::<Vec<&str>>();
    match &splits[ .. ] {
        [ from, to ] => (
                from.trim().parse::<usize>().unwrap_or(1),
                to.trim().parse::<usize>().unwrap_or(1)
            ),
        _ => (1, 1),
        //[] | [_] | [_, _, ..]=> (1, 1),
    }
}

fn main() {
    let desk = &mut Desk::new_default(10);

    while !desk.is_done() {
        println!("{0}", desk);

        println!("Your move [from] [to]: ");
        let (from, to) = read_move();
        println!("Moving from {0} -> {1}", from, to);

        match desk.move_disk(from -1, to -1) {
            false => println!("Could not perfrom such move"),
            true => (),
        }
    }

    println!("Game over, you win");
}
