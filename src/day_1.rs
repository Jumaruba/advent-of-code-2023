const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


fn get_number_part_a(chars: &Vec<char>, pos: usize) -> Option<i32> {
    let c = chars[pos];
    if c >= '0' && c <= '9' {
        return Some(c.to_digit(10).unwrap() as i32);
    }
    None
}

fn get_number_part_b(chars: &Vec<char>, pos: usize) -> Option<i32> {
    for j in 0..NUMBERS.len() {
        let num = chars.get(pos..pos+NUMBERS[j].len());
        if num.is_none() {
            continue;
        }
        let num_str: String = num.unwrap().iter().collect();
        if num_str == NUMBERS[j] {
            return Some(j as i32);
        }
    }
    None
}
fn solution(line: &str) -> [i32;2] {
    let mut frt = -1;
    let mut sec = -1;

    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() {
        let mut res = get_number_part_a(&chars, i); 
        if res.is_none(){
            res = get_number_part_b(&chars, i);
        }
        
        if res.is_some() {
            if frt == -1 {
                frt = res.unwrap();
            } sec = res.unwrap();
            
        }
    }

    [frt, sec]
}
    

#[cfg(test)]
mod test {
    use crate::day_1::solution;

    fn run(name: &str) -> i32 {
        // Read the file
        let input = std::fs::read_to_string(name).unwrap();
        let lines: Vec<&str> = input.split("\n").collect();
        let mut total = 0; 

        for line in lines {
            let res = solution(&line); 
            total += res[1] + res[0]*10; 
        }
        total
    }
    #[test]
    fn test_a(){
        assert_eq!(run("test/day_1/input1.txt"), 142);
    }

    #[test]
    fn test_b(){
        assert_eq!(run("test/day_1/input2.txt"), 281);
    }

    #[test]
    fn get_answer(){
        println!("{:?}",run("test/day_1/input.txt"));
    }
}

