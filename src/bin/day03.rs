use std::fs;

/// Convert input to a vec of vec of i32.
fn prepare_input(s : &String) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for line in s.lines() {
        let next : Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        result.append(&mut vec![next])
    }
    result
}

/// Find most common bit ( 0 or 1) in a column.
/// Ties are broken: than 1 is said to be the most common bit.
/// Returns either 0 or 1.
fn most_common_in_column(v : &Vec<Vec<u8>>, column : usize) -> u8 {
    let mut counter: i32 = 0;
    for line in v {
        match line[column] {
            0 => counter -=1,
            1 => counter +=1,
            _ => println!("Fail1 {:?}", line[column])
        }
    }
    // counter == 0 means there are equally many 0's and 1's
    // in this case we say 1 is the most common bit
    counter += 1;
    if counter > 0 { 1 } else { 0 }
}

/// Solve part 1.
/// Returns the power consumption (= product of gamma and epsilon).
fn part1(input : &Vec<Vec<u8>>) -> u32 {
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for i in 0..input[0].len(){
        let most_common = most_common_in_column(input, i);
        if most_common > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    binary_string2decimal(&gamma) * binary_string2decimal(&epsilon)
}

/// Assemble a binary number from a vec of 0's and 1's, convert this binary number to decimal.
fn vec_of_binary_to_decimal(vec_of_binary : &Vec<u8>) -> u32 {
    binary_string2decimal(&vec_of_binary.iter().map(|int| int.to_string()).collect::<Vec<String>>().join(""))
}

/// Convert a string of a binary number to a decimal number.
fn binary_string2decimal(s: &String) -> u32 {
    isize::from_str_radix(s, 2).unwrap() as u32
}

/// Solve part 2.
/// Return the live support rating (= product of oxygen generator rating and CO2 scrubber rating).
fn part2(input : &Vec<Vec<u8>>) -> u32 {
    let mut oxygen = input.clone();
    let mut co2 = input.clone();
    for i in 0..input[0].len() {
        let most_common_oxy = most_common_in_column(&oxygen, i);
        oxygen = oxygen.clone().into_iter().filter(|l| l[i] == most_common_oxy).collect();
        if oxygen.len() == 1 {break;}
    }
    for i in 0..input[0].len() {
        // Derive least common bit from most common bit.
        // In case of ties most_common_in_column deems 1 to be the most common bit.
        // By inverting the choice of the most common bit,
        // this respects the rules for how to split ties for the CO2 scrubber rating:
        // "If 0 and 1 are equally common, keep values with a 0 in the position being considered."
        let most_common_co2 = (most_common_in_column(&co2, i) +1 ) %2;
        co2 = co2.clone().into_iter().filter(|l| l[i] == most_common_co2).collect();
        if co2.len() == 1 {break;}
    }
    vec_of_binary_to_decimal(&oxygen[0]) * vec_of_binary_to_decimal(&co2[0])
}


fn main() {
    let data = fs::read_to_string("inputs/input03.txt").expect("Unable to read file");
    let input = prepare_input(&data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = fs::read_to_string("inputs/input03.test.txt").expect("Unable to read file");
        let input = prepare_input(&data);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn test_part2() {
        let data = fs::read_to_string("inputs/input03.test.txt").expect("Unable to read file");
        let input = prepare_input(&data);
        assert_eq!(part2(&input), 230);
    }
}