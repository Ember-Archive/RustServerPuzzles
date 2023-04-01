use rayon::prelude::*;

pub fn calc_kolakoski(inputs: Vec<usize>) {
    inputs
        .par_iter() // Use parallel iterator from Rayon
        .for_each(|&n| {
            let result = kolakoski_ratio_nilsson(n);
            println!("Input {}: {}", n, result);
        });
}

fn kolakoski_ratio_nilsson(n: usize) -> String {
    let mut sequence: Vec<usize> = Vec::with_capacity(n); // Create a vector with the capacity of `n`
    sequence.push(1); // Initialize the sequence with the first element 1
    sequence.push(2); // Initialize the sequence with the second element 2

    let mut i = 0;
    // Loop until the sequence has `n` elements
    while sequence.len() < n {
        let last = *sequence.last().unwrap(); // Get the last element of the sequence
        let curr = sequence[i]; // Get the current element at index `i`
        // Push (3 - last) to the sequence `curr` times
        for _ in 0..curr {
            sequence.push(3 - last);
        }
        i += 1; // Increment the index `i`
    }

    sequence.truncate(n); // Truncate the sequence to have exactly `n` elements

    // Count the occurrences of 1 and 2 in the sequence
    let count_1 = sequence.iter().filter(|&x| *x == 1).count();
    let count_2 = sequence.iter().filter(|&x| *x == 2).count();

    // Return the counts as a formatted string
    format!("{}:{}", count_1, count_2)
}