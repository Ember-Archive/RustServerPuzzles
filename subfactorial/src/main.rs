use subfactoriallib::*;

fn main() {
    let numbers = vec![6, 9, 14, 20];

    let results = calc_subfactorials(&numbers);

    for (n, sf) in results {
        println!("!{} = {}", n, sf);
    }
}