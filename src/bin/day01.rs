use std::fs;

fn prepare_input(s: String) -> Vec<i32> {
    s.lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(input : &Vec<i32>) -> i32 {
    let mut previous = &input[0];
    let mut count = 0;
    for i in input {
        if i > previous { count +=1 }
        previous = i;
    }
    count
}

fn part2(input : &Vec<i32>) -> i32 {
    let windows = input.windows(3).map(|x| x.iter().sum()).collect::<Vec<i32>>();
    part1(&windows)
}

fn main() {
    let data = fs::read_to_string("inputs/input01.txt").expect("Unable to read file");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = fs::read_to_string("inputs/input01.test.txt").expect("Unable to read file");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let data = fs::read_to_string("inputs/input01.test.txt").expect("Unable to read file");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 5);
    }
}