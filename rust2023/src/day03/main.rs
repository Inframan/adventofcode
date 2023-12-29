
fn main() {

    let engine_schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    validate_engine(engine_schematic);

    let matrix: Vec<&str> = engine_schematic.split("\n").collect(); 

    let mut sum = 0;
    let mut current_number = 0;
    let mut adjacent = false;
    for (i, str) in matrix.iter().enumerate() {
        // If it's a new line, reset everything - set current number to 0 and adjacent to false
        // If it was adjancent, that means the number is over and it should be added to the total sum
        if adjacent {
            sum += current_number;
        }
        current_number = 0;
        adjacent = false;
        for (j, char) in str.chars().enumerate() {
            // If it's a ., reset everything - set current number to 0 and adjacent to false
            // If it was adjancent, that means the number is over and it should be added to the total sum
            if char == '.' {
                if adjacent {
                    sum += current_number;
                }
                current_number = 0;
                adjacent = false;
            } else if char.is_ascii_digit() { // If digit, 
                current_number = current_number*10 + char.to_digit(10).unwrap();
                adjacent = adjacent || is_symbol_adjacent(i as i32, j as i32, &matrix);
            }

        }
    }

    print!("The current sum is {}", sum);
}

// Checks if it is adjacent to any symbol
fn is_symbol_adjacent(i: i32, j: i32, matrix: &Vec<&str>) -> bool {

    for n in -1..=1 {
        for m in -1..=1 {
            if is_symbol(i + n, j + m, matrix) {
                return true;
            }
        }

    }
    return false;
}

// Checks if the char at a given position is a symbol
fn is_symbol(i: i32, j: i32, matrix: &Vec<&str>) -> bool {

    let m = usize::try_from(i);
    let n = usize::try_from(j);

    return match (m, n) {
        (Ok(k), Ok(l)) => match matrix.get(k) {
            Some(s) => match s.chars().nth(l) {
                Some(c) => if c.is_ascii_digit() || c == '.' {
                    false
                } else {
                    true
                },
                None => false,
            },
            None => false,
        }
        _ => false,
    }
}

// Check if every line has the same size, otherwise abort
fn validate_engine(schematic: &str) {

    let sizes: Vec<usize> = schematic.lines().map(|s| s.bytes().len()).collect();

    let max = sizes.iter().max().expect("Couldn't get max string");
    let min = sizes.iter().min().expect("Couldn't get min string");

    if max != min {
        panic!("Lines have different sizes. Max: {} Min: {}", max, min);
    }
}