use rand::prelude::*;
use std::env;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Exclude {
    Low,
    High,
    None,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Roll {
    num: u32,
    die: u32,
    bonus: i32,
    exclude: Exclude,
}

/// Generates a random number in the range specified by the die.
fn roll_die(die: u32, mut rng: impl Rng) -> u32 {
    rng.gen_range(1, die + 1)
}

impl Roll {
    /// Simulates the `Roll`.
    fn roll(&self, mut rng: impl Rng) {
        print!("[{}d{}", self.num, self.die);
        if self.bonus != 0 {
            print!("{:+}", self.bonus);
        }
        match self.exclude {
            Exclude::Low => print!(" - low"),
            Exclude::High => print!(" - high"),
            Exclude::None => {}
        }
        print!("] ");
        match self.num {
            1 => {
                let roll = rng.gen_range(1, self.die + 1) as i32 + self.bonus;
                println!("{}", roll);
            }
            n if n > 1 => {
                let mut rolls = vec![];
                let mut exclude = match self.exclude {
                    Exclude::Low => std::u32::MAX,
                    Exclude::High => 0,
                    Exclude::None => 0,
                };
                for _ in 0..n {
                    let roll = roll_die(self.die, &mut rng);
                    match self.exclude {
                        Exclude::Low => {
                            if exclude > roll {
                                exclude = roll;
                            }
                        }
                        Exclude::High => {
                            if exclude < roll {
                                exclude = roll;
                            }
                        }
                        Exclude::None => {}
                    }
                    rolls.push(roll);
                }
                let mut excluded_shown = false;

                for &roll in rolls.iter() {
                    if roll == exclude && !excluded_shown {
                        excluded_shown = true;
                        print!("({}), ", roll);
                    } else {
                        print!("{}, ", roll);
                    }
                }

                let total = rolls.iter().sum::<u32>() as i32 + self.bonus;
                let exclude = exclude as i32;
                println!("final = {}", total - exclude);
            }
            _ => println!("no dice rolled"),
        }
    }
}

/// Attempts to parse the specified argument. An argument is expected in the
/// form of either <number>d<die> or d<die>. In the latter case, the number of
/// dice is inferred to be 1. If the argument cannot be parsed, `None` is
/// returned instead.
fn parse_arg(arg: &str) -> Option<Roll> {
    let idx = arg.find('d')?;
    let (num, die) = arg.split_at(idx);
    let num: u32 = num.parse().unwrap_or_else(|_| 1);
    let die = die.trim_start_matches("d");

    // Check if we have a flat bonus
    let (die, bonus) = if let Some(bonus_idx) = die.find('+') {
        let (die, bonus) = die.split_at(bonus_idx);
        let bonus: i32 = bonus.parse().ok()?;
        let die: u32 = die.parse().ok()?;
        (die, bonus)
    } else if let Some(bonus_idx) = die.find('-') {
        let (die, bonus) = die.split_at(bonus_idx);
        let bonus: i32 = bonus.parse().ok()?;
        let die: u32 = die.parse().ok()?;
        (die, bonus)
    } else {
        let die: u32 = die.parse().ok()?;
        (die, 0)
    };

    Some(Roll {
        num,
        die,
        bonus,
        exclude: Exclude::None,
    })
}

trait ParseArgs {
    /// Attempts to parse the arguments into an `impl Iterator<Item=Roll`.
    fn parse_args(self) -> Option<Vec<Roll>>;
}

impl<T> ParseArgs for T
where
    T: Iterator<Item = String>,
{
    fn parse_args(self) -> Option<Vec<Roll>> {
        self.flat_map(|arg| match arg.as_str() {
            "adv" | "advantage" => vec![Some(Roll {
                num: 2,
                die: 20,
                bonus: 0,
                exclude: Exclude::Low,
            })],
            "dis" | "disadvantage" => vec![Some(Roll {
                num: 2,
                die: 20,
                bonus: 0,
                exclude: Exclude::High,
            })],
            "chaos" | "chaos_bolt" => vec![
                Some(Roll {
                    num: 2,
                    die: 8,
                    bonus: 0,
                    exclude: Exclude::None,
                }),
                Some(Roll {
                    num: 1,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::None,
                }),
            ],
            "stats" | "char" | "character" => vec![
                Some(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Some(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Some(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Some(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Some(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Some(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
            ],
            arg => vec![parse_arg(arg)],
        })
        .collect()
    }
}

fn main() {
    let rolls = env::args().skip(1).parse_args();
    match rolls {
        Some(rolls) => match rolls.len() {
            0 => println!("[Error] No dice specified"),
            _ => {
                for roll in rolls {
                    let mut rng = thread_rng();
                    roll.roll(&mut rng);
                }
            }
        },
        None => println!("[Error] Failed to parse input"),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_single() {
//         let parsed = parse_arg("4d6");
//         let expected = Some(Roll { num: 4, die: 6 });
//         assert_eq!(parsed, expected);

//         let parsed = parse_arg("d10");
//         let expected = Some(Roll { num: 1, die: 10 });
//         assert_eq!(parsed, expected);

//         let parsed = parse_arg("5");
//         let expected = None;
//         assert_eq!(parsed, expected);
//     }

//     #[test]
//     fn parse_multiple() {
//         let args = ["adv", "dis", "chaos", "4d6", "d20"]
//             .into_iter()
//             .map(ToString::to_string);
//         let parsed: Vec<_> = parse_args(args).collect();
//         let expected = vec![
//             Roll { num: 1, die: 20 },
//             Roll { num: 1, die: 20 },
//             Roll { num: 1, die: 20 },
//             Roll { num: 1, die: 20 },
//             Roll { num: 2, die: 8 },
//             Roll { num: 1, die: 6 },
//             Roll { num: 4, die: 6 },
//             Roll { num: 1, die: 20 },
//         ];
//         assert_eq!(parsed, expected);
//     }
// }
