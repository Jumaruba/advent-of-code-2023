pub fn build_matrix(lines: &Vec<&str>) -> Vec<Vec<char>>{
    let mut matrix: Vec<Vec<char>> = Vec::new(); 
    for line in lines {
        let vec: Vec<char> = line.chars().collect(); 
        matrix.push(vec); 
    }
    matrix
}