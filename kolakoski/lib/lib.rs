use rayon::prelude::*;

pub fn calc_kolakoski(inputs: Vec<usize>) {
    inputs
        .par_iter()
        .for_each(|&n| {
            let result = kolakoski_ratio_nilsson(n);
            println!("Input {}: {}", n, result);
        });
}

pub fn kolakoski_ratio_nilsson(n: usize) -> String {
    let (mut count_1, mut count_2) = (1, 1); // Initialize counts of 1s and 2s
    let (mut k, mut i) = (vec![1, 2], 0); // Initialize the Kolakoski sequence with [1, 2] and a starting index of 0

    // Continue until the total count (count_1 + count_2) is equal to or greater than n
    while (count_1 + count_2) < n {
        let curr = k[i]; // Get the current value from the Kolakoski sequence
        let next_val = 3 - k[k.len() - 1]; // Calculate the next value to add to the Kolakoski sequence (3 - last value)

        // Check if adding the current value to the total count will not exceed n
        if (count_1 + count_2 + curr) <= n {
            if next_val == 1 {
                count_1 += curr; // If the next value is 1, increment count_1 by the current value
            } else {
                count_2 += curr; // If the next value is 2, increment count_2 by the current value
            }
        } else {
            // If adding the current value to the total count will exceed n, calculate the remaining difference
            let diff = n - (count_1 + count_2);
            if next_val == 1 {
                count_1 += diff; // If the next value is 1, increment count_1 by the remaining difference
            } else {
                count_2 += diff; // If the next value is 2, increment count_2 by the remaining difference
            }
            break;
        }

        // Append the next value to the Kolakoski sequence 'curr' number of times
        for _ in 0..curr {
            k.push(next_val);
        }
        i += 1; // Increment the index
    }

    format!("{}:{}", count_1, count_2) // Return the ratio of 1s and 2s as a formatted string
}