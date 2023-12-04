use std::collections::HashSet;


pub fn problem_a(my_numbers: &HashSet<&str>, winning_numbers: &HashSet<&str>) -> usize {
    let mut total = 0; 
    for winning in winning_numbers {
        if my_numbers.contains(winning) {
            if total == 0 {
                total = 1;
            } else {
                total *= 2; 
            }
        }
    }
    total
}

pub fn get_card_number(line: &str) -> usize {
    return line
        .trim()
        .split(":")
        .collect::<Vec<&str>>()[0]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
}

pub fn get_winning_numbers(line: &str) -> HashSet<&str> {
    let mut set = line
        .trim()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>()[0]      // Get the winning side
        .trim()
        .split(" ")
        .collect::<HashSet<&str>>();
    set.remove("");
    return set;
}

pub fn get_my_numbers(line: &str) -> HashSet<&str> {
    let mut set = line
        .trim()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>()[1]      // Get my side
        .trim()
        .split(" ")
        .collect::<HashSet<&str>>();
    set.remove("");
    return set;
}

#[cfg(test)]
mod test {
    use crate::day_4::*; 

    pub fn run(filename: &str, is_problem_a: bool) -> usize {
        let input = std::fs::read_to_string(filename).unwrap(); 
        let lines: Vec<&str> = input.split("\n").collect();
        let mut total = 0; 
        for line in lines.iter() {
            total += problem_a(&get_my_numbers(line), &get_winning_numbers(line));
        }
        return total;
    }

    #[test]
    fn test_a1() {
        assert_eq!(run("test/day_4/input1.txt", true), 13);
    }

    #[test]
    fn get_answer_a() {
        println!("{:?}", run("test/day_4/input.txt", true));
    }
}