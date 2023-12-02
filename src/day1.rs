use std::collections::HashMap;

const INVALID_ENTRY: i32 = 0;

pub fn part1(input: &str) -> i32 {
    let mut accumulator: i32 = 0;

    for (_line_num, line) in input.lines().enumerate() {
        let mut first: i32 = INVALID_ENTRY;
        let mut last: i32 = INVALID_ENTRY;

        for c in line.char_indices() {
            if c.1.is_numeric() {
                if first == INVALID_ENTRY {
                    first = c.1.to_digit(10).unwrap() as i32;
                }

                last = c.1.to_digit(10).unwrap() as i32;
            }
        }

        // Make an assumption that the number can never exceed 99,
        // so the first number is *10 (10's place).
        let val = first * 10 + last;
        accumulator += val;
    }

    return accumulator;
}

pub fn part2(input: &str) -> i32 {
    let mut accumulator: i32 = 0;

    let num_str_map: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (_line_num, line) in input.lines().enumerate() {
        let mut first: i32 = INVALID_ENTRY;
        let mut last: i32 = INVALID_ENTRY;

        for c in line.char_indices() {
            if c.1.is_numeric() {
                if first == INVALID_ENTRY {
                    first = c.1.to_digit(10).unwrap() as i32;
                }

                last = c.1.to_digit(10).unwrap() as i32;
            } else {
                let num_str = line.split_at(c.0).1;
                for num in num_str_map.keys() {
                    if num_str.starts_with(num) {
                        let val = num_str_map.get(num).unwrap();

                        if first == INVALID_ENTRY {
                            first = val.clone();
                        }

                        last = val.clone();
                    }
                }
            }
        }

        // Make an assumption that the number can never exceed 99,
        // so the first number is *10 (10's place).
        let val = first * 10 + last;
        accumulator += val;
    }

    return accumulator;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../days/1/example_1");
    const EXAMPLE_2: &str = include_str!("../days/1/example_2");
    const INPUT: &str = include_str!("../days/1/input");

    #[test]
    fn day1_part1_example() {
        assert_eq!(part1(EXAMPLE_1), 142);
    }

    #[test]
    fn day1_part1_input() {
        assert_eq!(part1(INPUT), 54081);
    }

    #[test]
    fn day2_part2_example() {
        assert_eq!(part2(EXAMPLE_2), 281)
    }

    #[test]
    fn day2_part2_input() {
        assert_eq!(part2(INPUT), 54649);
    }
}
