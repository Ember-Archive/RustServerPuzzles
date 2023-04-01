use rayon::prelude::*;

fn subfactorial(n: u64, memo: &mut Vec<Option<u64>>) -> u64 {
    // base case 0 = 1
   if n == 0 {
       return 1;
   }
   
   // base case 1 = 0
   if n == 1 {
       return 0; 
   }

   // if previously calculated, return the memoised value
   if let Some(val) = memo[n as usize] {
       return val;
   }

   // let's get recursive
   let result = (n - 1) * (subfactorial(n - 1, memo) + subfactorial(n - 2, memo));
   
   // memoise the result
   memo[n as usize] = Some(result);

   result
}

pub fn calc_subfactorials(inputs: &Vec<u64>) -> Vec<(u64, u64)> {
    let results: Vec<(u64, u64)> = inputs
        .par_iter()
        .map(|&n| {
            let mut memo = vec![None; n as usize + 1]; // memoisation vector
            (n, subfactorial(n, &mut memo))
        })
        .collect(); 
    results
}