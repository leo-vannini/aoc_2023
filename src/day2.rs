pub fn part1(input: &str) -> i32 {
    let mut accumulator: i32 = 0;
    for line in input.lines() {
        // split at : for Game ID
        // Game <id>:
        let full: Vec<&str> = line.split(":").collect();
        let begining: Vec<&str> = full[0].split(" ").collect();
        let game_id: i32 = begining[1].parse::<i32>().unwrap();
        let mut valid: bool = true;

        'parse_game: for game in full[1].split(";") {
            let mut cube = Cube {
                red: 12,
                green: 13,
                blue: 14,
            };

            for play in game.split(",") {
                // TODO --> can you destructure cleaner?
                let op: Vec<&str> = play.trim().split(" ").collect();
                let num: i32 = op[0].parse::<i32>().unwrap();
                let color: &str = op[1];

                match color {
                    "green" => {
                        cube.green -= num;
                    }
                    "red" => {
                        cube.red -= num;
                    }
                    "blue" => {
                        cube.blue -= num;
                    }
                    _ => println!("BAD COLOR"),
                }

                if cube.green < 0 || cube.red < 0 || cube.blue < 0 {
                    valid = false;
                    break 'parse_game;
                }
            }
        }

        if valid {
            accumulator += game_id
        }
    }

    return accumulator;
}

struct Cube {
    red: i32,
    blue: i32,
    green: i32,
}

pub fn part2(input: &str) -> i32 {
    let mut accumulator: i32 = 0;

    for line in input.lines() {
        let mut cube = Cube {
            red: 0,
            blue: 0,
            green: 0,
        };

        let v: Vec<&str> = line.split(":").collect();
        for game in v[1].split(";") {
            for play in game.split(",") {
                // TODO --> can you destructure cleaner?
                let op: Vec<&str> = play.trim().split(" ").collect();
                let num: i32 = op[0].parse::<i32>().unwrap();
                let color: &str = op[1];

                match color {
                    "green" => {
                        if num > cube.green {
                            cube.green = num
                        }
                    }
                    "red" => {
                        if num > cube.red {
                            cube.red = num
                        }
                    }
                    "blue" => {
                        if num > cube.blue {
                            cube.blue = num
                        }
                    }
                    _ => println!("BAD COLOR"),
                }
            }
        }

        accumulator += cube.green * cube.red * cube.blue;
    }

    return accumulator;
}

mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../days/2/example");
    const INPUT: &str = include_str!("../days/2/input");

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    #[test]
    pub fn part1_input() {
        assert_eq!(part1(INPUT), 2449)
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2286)
    }

    #[test]
    pub fn part2_input() {
        assert_eq!(part2(INPUT), 63981)
    }
}
