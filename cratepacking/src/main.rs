use rayon::prelude::*;

fn calc_boxes(container_x: u32, container_y: u32, x: u32, y: u32) -> u32 {
    let fit_wide = container_x / x;
    let fit_tall = container_y / y;
    let total = fit_tall * fit_wide;
    return total;
}

fn main() {
    let inputs = vec![
        (25, 18, 6, 5), 
        (10, 10, 1, 1), 
        (12, 34, 5, 6), 
        (12345, 678910, 1112, 1314), 
        (5, 100, 6, 1)
    ];

    let results: Vec<u32> = inputs
        .par_iter()
        .map(|(container_x, container_y, x, y)| {
            calc_boxes(*container_x, *container_y, *x, *y)
        })
        .collect();

    // Print the results
    for ((container_x, container_y, x, y), answer) in inputs.iter().zip(results.iter()) {
        println!("{}x{} container, {}x{} boxes: {}", container_x, container_y, x, y, answer);
    }
}