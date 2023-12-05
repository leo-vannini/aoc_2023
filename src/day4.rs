use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> i32 {
    let mut winners: HashSet<i32> = HashSet::new();
    let mut total: i32 = 0;

    for line in input.lines() {
        let vec: Vec<&str> = line.split("|").collect();
        let start: Vec<&str> = vec[0].split(":").collect();
        let mut rd_total: i32 = 0;

        let win_nums: Vec<&str> = start[1]
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();
        println!("winners --> {:?}", win_nums);
        for num in win_nums {
            let val: i32 = num.parse::<i32>().unwrap();
            winners.insert(val);
        }

        let nums: Vec<&str> = vec[1].trim().split(" ").filter(|x| !x.is_empty()).collect();
        println!("nums --> {:?}", nums);
        for num in nums {
            let num: i32 = num.parse::<i32>().unwrap();
            if winners.contains(&num) {
                rd_total = if rd_total == 0 { 1 } else { rd_total * 2 };
                println!("WINNER --> {num} | {rd_total}");
            }
        }

        println!("rd_total --> {rd_total}");
        total += rd_total;
        winners.clear();
    }

    return total;
}

fn part2(input: &str) -> i32 {
    let mut winners: HashSet<i32> = HashSet::new();
    let mut total: i32 = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let vec: Vec<&str> = line.split("|").collect();
        let start: Vec<&str> = vec[0].split(":").collect();
        let mut rd_total: i32 = 0;

        let card_id: Vec<&str> = start[0]
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();
        let card_id: i32 = card_id[1].parse::<i32>().unwrap();
        println!("card_id --> {card_id}");

        // Process the original
        let count = map.get(&card_id);
        if count == None {
            map.insert(card_id, 1);
        } else {
            map.insert(card_id, count.unwrap() + 1);
        }

        let win_nums: Vec<&str> = start[1]
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect();
        for num in win_nums {
            let val: i32 = num.parse::<i32>().unwrap();
            winners.insert(val);
        }

        let nums: Vec<&str> = vec[1].trim().split(" ").filter(|x| !x.is_empty()).collect();
        for num in nums {
            let num: i32 = num.parse::<i32>().unwrap();
            if winners.contains(&num) {
                rd_total += 1;
            }
        }

        println!("winning_nums --> {rd_total}");

        let multiplier: i32 = map.get(&card_id).unwrap().clone();
        total += multiplier;
        println!("multiplier --> {multiplier}");
        for i in 1..rd_total + 1 {
            let i: i32 = card_id + i;
            println!("i --> {i}");
            let count = map.get(&i);
            if count == None {
                map.insert(i, 1 * multiplier);
            } else {
                map.insert(i, count.unwrap() + multiplier);
            }
        }

        println!("map --> {:?}", map);
        winners.clear();
    }

    return total;
}

mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../days/4/example");
    const INPUT: &str = include_str!("../days/4/input");

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    pub fn part1_input() {
        assert_eq!(part1(INPUT), 21158)
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(EXAMPLE), 30)
    }

    #[test]
    pub fn part2_input() {
        assert_eq!(part2(INPUT), 6050769)
    }
}
