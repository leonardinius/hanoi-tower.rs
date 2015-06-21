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
        let max_weight : usize = 
            self.rods.iter()
                .map(|r| r.stack.iter().map(|s| s.weight).max().unwrap_or(0))
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
                    Some(tower) => format!("{1}[{0}|{0}]{1}", dup("#", tower.weight), dup(" ", max_weight - tower.weight)),
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
    let desk1 = Desk {
        rods : vec![
            Rod { stack:  vec![ Tower {weight: 4, }, Tower {weight: 3, }, ], },
            Rod { stack:  vec![ Tower {weight: 2, }, ], },
            Rod { stack:  vec![ Tower {weight: 1, }, ], },
        ],
    };
    println!("{0}", desk1);

    let desk2 = Desk {
        rods : vec![
            Rod { stack:  vec![ Tower {weight: 4, }, Tower {weight: 2, }, ], },
            Rod { stack:  vec![ Tower {weight: 3, }, ], },
            Rod { stack:  vec![ Tower {weight: 1, }, ], },
        ],
    };
    println!("{0}", desk2);

    let desk3 = Desk {
        rods : vec![
            Rod { stack:  vec![ Tower {weight: 4, }, Tower {weight: 2, }, ], },
            Rod { stack:  vec![ Tower {weight: 3, }, Tower {weight: 1 }, ], },
        ],
    };
    println!("{0}", desk3);
}
