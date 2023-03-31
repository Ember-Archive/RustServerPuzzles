use goldbachweaklib::*;

fn main() {
    let a = [111, 17, 199, 287, 53];
    let k = 10;

    for &number in a.iter() {
        let (prime1, prime2, prime3) = find_primes(number as i64, k);
        println!("{} = {} + {} + {}", number, prime1, prime2, prime3);
    }
}
