use std::{f32::consts::E, cmp::max};

const RED: u32= 12; 
const GREEN: u32 = 13;
const BLUE: u32 = 14;

pub fn problem_b(color: &str, count: u32, red_green_blue: &mut Vec<u32>) {
    match color {
        "red" => {red_green_blue[0] = max(red_green_blue[0], count) },
        "blue" => {red_green_blue[1] = max(red_green_blue[1], count) },
        "green" => {red_green_blue[2] = max(red_green_blue[2], count) }
        _ => {}
    }
}
pub fn problem_a(color: &str, count: u32) -> bool{
    match color {
        "red" => {
            if count > RED {
                return false; 
            }
        },
        "blue" => {
            if count > BLUE {
                return false; 
            }
        },
        "green" => {
            if count > GREEN {
                return false; 
            }
        }
        _ => {true;}
    }
    true
}

pub fn solution(line: &str, is_problem_a: bool) -> u32{
    let elements: Vec<&str> = line.split(":").collect();
    let reveals = elements[1].trim().split(";");
    // PROBLEM A 
    let game_number: u32 = elements[0].trim().split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
    // PROBLEM B
    let mut red_green_blue: Vec<u32> = vec![0,0,0];

    // PROCESS 
    // Check each time the colors were revealed.
    for reveal in reveals {
        let colors_count = reveal.split(","); 
        // Check each color count. 
        for color_count in colors_count {
            let color_count_splited: Vec<&str> = color_count.trim().split(" ").collect(); 
            let count = color_count_splited[0].parse::<u32>().unwrap();
            // PROBLEM_A
            if is_problem_a && !problem_a(color_count_splited[1], count){
                return 0;
            } 
            // PROBLEM_B
            else {
                problem_b(color_count_splited[1], count, &mut red_green_blue); 
            }
        }
    }

    if is_problem_a {
        return game_number; 
    } 
    return red_green_blue[0] * red_green_blue[1] * red_green_blue[2];
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(filename: &str, is_problem_a: bool) -> u32 {
        // Read file
        let mut total: u32 = 0; 
        let input = std::fs::read_to_string(filename).unwrap();
        let lines: Vec<&str> = input.split("\n").collect();
        for line in lines {
            total += solution(&line, is_problem_a);
        }
        return total;
    }

    #[test]
    fn test_a() {
        assert_eq!(run("test/day_2/input1.txt", true), 8);
    }

    #[test]
    fn test_b() {
        assert_eq!(run("test/day_2/input1.txt", false), 2286);
    }


    #[test]
    fn get_answer_a(){
        println!("{:?}",run("test/day_2/input.txt", true));
    }

    #[test]
    fn get_answer_b(){
        println!("{:?}",run("test/day_2/input.txt", false));
    }
}
