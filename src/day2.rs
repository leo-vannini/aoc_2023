use std::collections::HashMap;

const DEBUG: bool = false;

pub fn part1(input: &str) -> i32 {
    let mut accumulator: i32 = 0;
    for line in input.lines() {
        // split at : for Game ID
        // Game <id>:
        let full: Vec<&str> = line.split(":").collect();
        let begining: Vec<&str> = full[0].split(" ").collect();
        let game_id: i32 = begining[1].parse::<i32>().unwrap();

        // parse until ; OR newline
        if DEBUG {
            println!("{game_id}");
        }

        let mut valid: bool = true;

        'parse_game: for game in full[1].split(";") {
            if DEBUG {
                println!("Game string --> {game}");
            }

            for play in game.split(",") {
                let mut cfg: HashMap<&str, i32> =
                    HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

                if DEBUG {
                    println!("Play --> {play}");
                }

                // TODO --> can you destructure cleaner?
                let op: Vec<&str> = play.trim().split(" ").collect();
                let num: i32 = op[0].parse::<i32>().unwrap();
                let color: &str = op[1];
                if DEBUG {
                    println!("Num --> {}", num);
                    println!("Color --> {}", color);
                }

                let current: i32 = cfg.get(color).copied().unwrap();
                let possible: i32 = current - num;
                if possible >= 0 {
                    cfg.insert(color, current - num);
                } else {
                    valid = false;
                    break 'parse_game;
                }
            }
        }

        if valid {
            accumulator += game_id
        } else {
            println!("{game_id} --> IMPOSSIBLE!")
        }
    }

    return accumulator;
}

mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../days/2/example_1");
    const INPUT: &str = include_str!("../days/2/input");

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 8);
    }

    #[test]
    pub fn part1_input() {
        assert_eq!(part1(INPUT), 2449)
    }
}
