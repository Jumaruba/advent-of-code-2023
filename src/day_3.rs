use std::collections::{HashMap, HashSet};


pub fn problem_b(matrix: &Vec<Vec<char>>) -> usize {
    let mut adjacent_stars: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let total_cols = matrix[0].len();
    for i in 0..matrix.len() {
        let mut start_col: i64 = -1; 
        let mut end_col: i64 = -1; 
        for j in 0..matrix[0].len() {
            // Is not a number. Process the previous number and continue; 
            if !matrix[i][j].is_digit(10) {
                start_col = -1; 
                end_col = -1; 
                continue; 
            } 
            // Is the first digit of the number. 
            else if start_col == -1 && matrix[i][j].is_digit(10) {
                start_col = j as i64; 
            } 
            // Is the last digit. Try add the number to the hash. 
            if start_col != -1 && matrix[i][j].is_digit(10) && (j +1 >= total_cols || !matrix[i][j+1].is_digit(10)){
                end_col = j as i64; 
                try_add_to_hash(&matrix, start_col as usize, end_col as usize, i, &mut adjacent_stars);
            }
        }
    }

    // compute hash.
    let mut total: usize = 0; 
    for vec in adjacent_stars.values() {
        if vec.len() != 2 {
            continue;
        }
        for i in 0..vec.len(){ 
            for j in i+1..vec.len() {
                total += vec[i] * vec[j]; 
            }
        }
    } 
    println!("{:?}", adjacent_stars);
    return total;


}
/// Check if a position has an adjacent symbol.
/// Returns the position of the number.
pub fn has_symbol_B(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> HashSet<(usize, usize)> {
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let i: i64 = i.try_into().unwrap();
    let j: i64 = j.try_into().unwrap();
    let total_lines: i64 = matrix.len().try_into().unwrap();
    let total_columns: i64 = matrix[0].len().try_into().unwrap();
    let range: Vec<i64> = vec![-1, 0, 1];
    for m in range.iter() {
        for n in range.iter() {
            let line = n + i; 
            let col = m + j;
            if line >= 0 && col >= 0 && line < total_lines && col < total_columns {
                if let Some(row) = matrix.get(line as usize){
                    if let Some(element) = row.get(col as usize){
                        if is_star(*element) {
                            set.insert((line as usize, col as usize));
                        }
                    }
                }
            }
        }
    }   
    return set;
}

fn try_add_to_hash(matrix: &Vec<Vec<char>>, start_col: usize, end_col: usize, line: usize, map: &mut HashMap<(usize, usize), Vec<usize>>) {
    let mut star_positions: HashSet<(usize, usize)> = HashSet::new();
    // Get all '*' positions.
    for col in start_col..end_col+1 {
        let set = has_symbol_B(matrix, line, col); 
        star_positions = star_positions.union(&set).cloned().collect();
        
    }

    // if it is not empty, then add to a hash.
    let number = get_number(matrix, line, start_col, end_col);
    if star_positions.len() != 0 {
        for star_position in star_positions {
            if None == map.get(&star_position) {
                map.insert(star_position, vec![number]);
            } else {
                map.get_mut(&star_position).unwrap().push(number);
            }
        }
    }
}

fn is_star(symbol: char) -> bool {
    return symbol == '*';
}



/// PROBLEM A ======================================================================================

pub fn problem_a(matrix: &Vec<Vec<char>>) -> usize {
    let mut total: usize = 0; 
    let total_cols = matrix[0].len();
    for i in 0..matrix.len() {
        let mut start_col: i64 = -1; 
        let mut end_col: i64 = -1; 
        for j in 0..matrix[0].len() {
            // Is not a number. Process the previous number and continue; 
            if !matrix[i][j].is_digit(10) {
                start_col = -1; 
                end_col = -1; 
                continue; 
            } else if start_col == -1 && matrix[i][j].is_digit(10) {
                start_col = j as i64; 
            } if start_col != -1 && matrix[i][j].is_digit(10) && (j +1 >= total_cols || !matrix[i][j+1].is_digit(10)){
                end_col = j as i64; 
                if let Some(number) = get_number_if_valid_A(matrix, i, start_col as usize, end_col as usize) {
                    total += number; 
                    println!("Number: {}", number);
                }
            }
        }
    }
    return total;
}

/// Checks if the number has an adjacent symbol. 
/// If it has, then return the number itself. Otherwise, return None.
fn get_number_if_valid_A(matrix: &Vec<Vec<char>>, i: usize, start_col: usize, end_col: usize) -> Option<usize> {
    for j in start_col..end_col+1 {
        if has_symbol_A(matrix, i, j){
            return Some(get_number(matrix, i, start_col, end_col) as usize);
        }
    }
    None 
}



fn is_symbol(symbol: char) -> bool {
    return symbol != '.' && !symbol.is_digit(10); 
}

/// Check if a position has an adjacent symbol.
/// Returns the position of the number.
pub fn has_symbol_A(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let i: i64 = i.try_into().unwrap();
    let j: i64 = j.try_into().unwrap();
    let total_lines: i64 = matrix.len().try_into().unwrap();
    let total_columns: i64 = matrix[0].len().try_into().unwrap();
    let range: Vec<i64> = vec![-1, 0, 1];
    for m in range.iter() {
        for n in range.iter() {
            let line = n + i; 
            let col = m + j;
            if line >= 0 && col >= 0 && line < total_lines && col < total_columns {
                if let Some(row) = matrix.get(line as usize){
                    if let Some(element) = row.get(col as usize){
                        if is_symbol(*element) {
                            return true;
                        }
                    }
                }
            }
        }
    }   
    return false; 
}



// BOTH ======================================================================================
/// Get a number in a matrix position.
fn get_number(matrix: &Vec<Vec<char>>, i: usize, start_col: usize, end_col: usize) -> usize {
    let slice = matrix[i][start_col..end_col+1].iter().collect::<String>();
    return slice.parse::<usize>().unwrap();
}



#[cfg(test)]
mod test{
    use super::*;

    fn run(filename: &str, is_problem_a: bool) -> usize {
        // Allocate in the heap.
        let lines: String = std::fs::read_to_string(filename).unwrap();
        // Now that lines are allocated, we can get the reference and store in the Vec.
        let lines: Vec<&str> = lines.split("\n").collect();
        let matrix = crate::utils::build_matrix(&lines);
        if is_problem_a {
            return problem_a(&matrix);
        } else {
            return problem_b(&matrix);
        }
    }

    #[test]
    fn test_a1() {
        assert_eq!(run("test/day_3/input1.txt", true), 4361);
    }

    

    #[test]
    fn test_a2() {
        assert_eq!(run("test/day_3/input2.txt", true), 4483);
    }

    #[test]
    fn test_a3() {
        assert_eq!(run("test/day_3/input3.txt", true), 413);
    }


    
   #[test]
    fn get_answer_a(){
        println!("{:?}",run("test/day_3/input.txt", true));
    }

    #[test]
    fn test_b1() {
        assert_eq!(run("test/day_3/input1.txt", false), 467835);
    }

    #[test]
    fn test_b3(){
        assert_eq!(run("test/day_3/input3.txt", false), 6756);
    }

   #[test]
    fn get_answer_b(){
        println!("{:?}",run("test/day_3/input.txt", false));
    }





}