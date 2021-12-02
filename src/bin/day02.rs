use std::fs;

fn part1(input : &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for entry in input.lines() {
        let parts = entry.split_whitespace().collect::<Vec<&str>>();
        let distance = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => horizontal += distance,
            "up"      => depth -= distance,
            "down"    => depth += distance,
            _         => ()
        }
    }
    horizontal * depth
}

fn part2(input : &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for entry in input.lines() {
        let parts = entry.split(' ').collect::<Vec<&str>>();
        let distance = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => {
                horizontal += distance;
                depth += aim * distance;
            },
            "up"   => aim -= distance,
            "down" => aim += distance,
            _      => ()
        }
    }
    horizontal * depth
}

fn main() {
    let data = fs::read_to_string("inputs/input02.txt").expect("Unable to read file");
    println!("First task: {:?}", part1(&data));
    println!("Second task: {:?}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = fs::read_to_string("inputs/input02.test.txt").expect("Unable to read file");
        assert_eq!(part1(&data), 150);
    }

    #[test]
    fn test_part2() {
        let data = fs::read_to_string("inputs/input02.test.txt").expect("Unable to read file");
        assert_eq!(part2(&data), 900);
    }
}


