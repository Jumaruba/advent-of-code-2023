use std::collections::HashSet;


pub fn problem_b(my_numbers: &HashSet<&str>, winning_numbers: &HashSet<&str>, sum: &mut Vec<usize>, pos: usize) {
    let mut matching_numbers = 0;
    for winning in winning_numbers {
        if my_numbers.contains(winning) {
            matching_numbers += 1; 
        }
    }
    for i in 1..=matching_numbers {
        if pos + i < sum.len() {
            sum[pos+i] += sum[pos];
        }
    }
}
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


pub fn parse_card(line: &str, pos: usize) -> HashSet<&str> {
    let mut set = line
        .trim()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>()[pos]      // Get the winning side
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
        let mut vec: Vec<usize> = vec![1; lines.len()];
        let mut total = 0; 
        let mut i = 0; 
        for line in lines.iter() {
            if is_problem_a {
                total += problem_a(&parse_card(line, 1), &parse_card(line, 0));
            } else {
                problem_b(&parse_card(line, 1), &parse_card(line, 0), &mut vec, i);
            }
            i += 1; 
        }
        if is_problem_a {
            return total;
        } 
        else {
            return vec.iter().sum();
        }
    }

    #[test]
    fn test_a1() {
        assert_eq!(run("test/day_4/input1.txt", true), 13);
    }

    #[test]
    fn get_answer_a() {
        println!("{:?}", run("test/day_4/input.txt", true));
    }

    #[test]
    fn test_b1() {
        assert_eq!(run("test/day_4/input1.txt", false), 30);
    }

    #[test]
    fn get_answer_b() {
        println!("{:?}", run("test/day_4/input.txt", false));
    }
}