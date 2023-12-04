struct LineBuffer {
    write: bool,
    buf: String,
}

pub fn part1(input: &str) -> i32 {
    let mut vec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        vec.push(line.chars().collect());
    }

    let mut total: i32 = 0;
    let mut upper: LineBuffer = LineBuffer {
        write: false,
        buf: String::new(),
    };
    let mut middle: LineBuffer = LineBuffer {
        write: false,
        buf: String::new(),
    };
    let mut lower: LineBuffer = LineBuffer {
        write: false,
        buf: String::new(),
    };

    let mut saw_symbol: bool = false;

    for i in 0..vec.len() + 1 {
        if i < 1 || i + 1 >= vec.len() {
            continue;
        } else {
            println!("{}", i + 1);
        }

        if false {
            println!("")
        }

        for j in 0..vec[i].len() {
            if vec[i - 1][j].is_numeric() {
                upper.buf.push(vec[i - 1][j]);
                if saw_symbol {
                    upper.write = true;
                }
            }

            if vec[i][j].is_numeric() {
                middle.buf.push(vec[i][j]);
                if saw_symbol {
                    middle.write = true;
                }
            }

            if vec[i + 1][j].is_numeric() {
                lower.buf.push(vec[i + 1][j]);

                if saw_symbol {
                    lower.write = true;
                }
            }

            // symbol check
            let c: char = vec[i][j];
            if c != '.' && !c.is_alphanumeric() {
                if upper.buf.len() > 0 {
                    upper.write = true;
                }

                if middle.buf.len() > 0 {
                    middle.write = true;
                }

                if lower.buf.len() > 0 {
                    lower.write = true;
                }

                saw_symbol = true;
            } else {
                saw_symbol = false;
            }

            // UPPER
            if j == vec[i].len() - 1 || !vec[i - 1][j].is_numeric() {
                if upper.write == true && upper.buf.len() > 0 {
                    let val: i32 = upper.buf.parse::<i32>().unwrap();
                    println!("UPPER write --> {val}");
                    total += val;
                    upper.buf.clear();
                    upper.write = false;
                } else {
                    upper.buf.clear();
                }
            }

            // MIDDLE
            if j == vec[i].len() - 1 || !vec[i][j].is_numeric() {
                if middle.write == true && middle.buf.len() > 0 {
                    let val: i32 = middle.buf.parse::<i32>().unwrap();
                    println!("MIDDLE write --> {val}");
                    total += val;
                    middle.buf.clear();
                    middle.write = false;
                } else {
                    middle.buf.clear();
                }
            }

            // LOWER
            if j == vec[i].len() - 1 || !vec[i + 1][j].is_numeric() {
                if lower.write == true && lower.buf.len() > 0 {
                    let val: i32 = lower.buf.parse::<i32>().unwrap();
                    println!("LOWER write --> {val}");
                    total += val;
                    lower.buf.clear();
                    lower.write = false;
                } else {
                    lower.buf.clear();
                }
            }
        }
    }

    return total;
}

mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../days/3/example");
    const INPUT: &str = include_str!("../days/3/input");

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(EXAMPLE), 4361);
    }

    #[test]
    pub fn part1_input() {
        assert_eq!(part1(INPUT), 527446);
    }
}
