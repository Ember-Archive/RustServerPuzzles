use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;
use num_complex::Complex;

fn fill<F>(well: &[i32], width: usize, target_value: i32, directions_list: F) -> (HashMap<Complex<i32>, i32>, HashMap<Complex<i32>, i32>)
where
    F: Fn(Complex<i32>) -> Vec<Complex<i32>>, {
    let mut squares: HashMap<Complex<i32>, i32> = HashMap::new(); // initialise a HashMap to store the values of each position in the grid
    let mut start_pos: Complex<i32> = Complex::new(0, 0); // initialise the starting position
    let mut target_pos: Complex<i32> = Complex::new(0, 0); // initialise the target position

    for (index, &value) in well.iter().enumerate() { // iterate through the grid
        let position: Complex<i32> = Complex::new((index % width) as i32, (index / width) as i32); // calculate position from 1D vector
        squares.insert(position, value); // add the position and value to the squares HashMap

        if value == 1 { // if the current value is the starting value
            start_pos = position; // set the starting position to the current position
        } else if value == target_value { // if the current value is the target value
            target_pos = position; // set the target position to the current position
        }
    }

    (squares.clone(), fill_step(squares, target_pos, start_pos, directions_list)) // return a tuple containing a copy of the original squares and the result of calling fill_step with the squares HashMap, target position, and directions
}

fn fill_step<F>(mut squares: HashMap<Complex<i32>, i32>, target_pos: Complex<i32>, start_pos: Complex<i32>, directions_list: F) -> HashMap<Complex<i32>, i32>
where
    F: Fn(Complex<i32>) -> Vec<Complex<i32>>, {
    let mut filled: HashSet<Complex<i32>> = vec![start_pos].into_iter().collect::<HashSet<_>>(); // initialize a HashSet with the starting position
    let mut level: i32 = squares[&start_pos];  // set the starting level to the value of the starting position in the squares HashMap
    loop { // repeat until the target cell is reached
        level += 1; // increment the current level
        for pos in &filled { // for each filled position, update the squares HashMap with the current level
            squares.insert(*pos, level);
        }
        if filled.contains(&target_pos) { // if the target cell is in the filled positions, return the squares HashMap
            return squares;
        }
        loop { // repeat until no more boundary cells are found
            let boundary: HashSet<_> = filled // generate the boundary cells by applying the directions to each filled cell and filtering the resulting positions
                .iter()
                .flat_map(|&position| directions_list(position))
                .filter(|&position| squares.contains_key(&position) && !filled.contains(&position) && squares[&position] <= level)
                .collect();
            if !boundary.is_empty() { // if there are boundary cells
                if boundary.iter().any(|&position| squares[&position] < level) { // if there is a boundary cell with a lower level than the current level
                    let lowest: Complex<i32> = boundary.into_iter().min_by_key(|&position| squares[&position]).unwrap(); // find the boundary cell with the lowest level
                    return fill_step(squares.clone(), target_pos, lowest, directions_list); // recursively call fill_step with the lowest boundary cell as the new starting position
                }
                filled.extend(boundary); // add the boundary cells to the filled set
            } else {
                break; // no more boundary cells, exit the loop
            }
        }
    }
}

fn parse_ints(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .map(|s| s.parse().unwrap()) // turn the string into a vec of ints
        .collect()
}

fn build_wells(text: &str) -> (Vec<i32>, usize, i32) {
    let mut numbers: Vec<i32> = parse_ints(text); // parse the input text as a vector of i32s

    let _rows: i32 = numbers.remove(0); // we don't need the rows but it's part of the spec ¯\_(ツ)_/
    let cols: i32 = numbers.remove(0); // get the number of columns
    let target: i32 = numbers.pop().unwrap(); // get the target value off the end

    let width = cols as usize; 

    (numbers, width, target) // return a tuple containing the numbers, width (number of columns) and target value
}


fn main() {  
    let inputs: Vec<&str> = vec![ // too lazy to do user input lmao
        "3 3 1 9 6 2 8 5 3 7 4 4",
        "3 3 8 3 2 7 1 5 4 9 6 4",
        "7 7 38 33 11 48 19 45 22 47 30 24 15 46 28 3 14 13 2 34 8 21 17 10 9 5 16 27 36 39 18 32 20 1 35 49 12 43 29 4 41 26 31 37 25 6 23 44 7 42 40 35",
        "7 7 15 16 46 1 38 43 44 25 10 7 6 34 42 14 8 19 9 21 13 23 22 32 11 29 36 3 5 47 31 33 45 24 12 18 28 40 41 20 26 39 48 2 49 35 27 4 37 30 17 26",
    ];

    let four_dirs = |point: Complex<i32>| -> Vec<Complex<i32>> {
        vec![
            point + Complex::new(1, 0),
            point + Complex::new(-1, 0),
            point + Complex::new(0, 1),
            point + Complex::new(0, -1),
        ]
    };
    
    let eight_dirs = |point: Complex<i32>| -> Vec<Complex<i32>> {
        vec![
            point + Complex::new(1, 0),
            point + Complex::new(-1, 0),
            point + Complex::new(0, 1),
            point + Complex::new(0, -1),
            point + Complex::new(1, 1),
            point + Complex::new(-1, 1),
            point + Complex::new(1, -1),
            point + Complex::new(-1, -1),
        ]
    };

    inputs.par_iter().for_each(|input| { // iterate through the inputs
        let (well, cols, target): (Vec<i32>, usize, i32) = build_wells(input);

        let (original_squares, filled_squares): (HashMap<Complex<i32>, i32>, HashMap<Complex<i32>, i32>) =
            fill(&well, cols, target, &four_dirs);

        let diff: HashMap<Complex<i32>, i32> = original_squares
            .iter()
            .map(|(&position, &value)| (position, filled_squares[&position] - value))
            .collect();

        
        println!("Answers for 4-directional:");
        let total: i32 = diff.values().sum::<i32>();
        println!("{}", total);

        
        let (original_squares, filled_squares): (HashMap<Complex<i32>, i32>, HashMap<Complex<i32>, i32>) =
            fill(&well, cols, target, &eight_dirs);
        let diff: HashMap<Complex<i32>, i32> = original_squares
            .iter()
            .map(|(&position, &value)| (position, filled_squares[&position] - value))
            .collect();

        println!("Answers for 8-directional:");
        let total: i32 = diff.values().sum::<i32>();
        println!("{}", total);
    });
}