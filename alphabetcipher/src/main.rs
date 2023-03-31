/* function to perform a simple substitution cipher on a message using a secret key
 * and return the result as a string
 * 'secret' is the key to use for the cipher, which should be a lowercase alphanumeric string
 * 'message' is the message to be encoded or decoded, which should be a lowercase alphanumeric string
 * 'decode' is a boolean flag indicating whether the function should encode or decode the message
 */
fn alphabet_cipher(secret: &str, message: &str, decode: bool) -> String {
    // calculate the shift factor based on whether the function should encode or decode
    let shift_factor = if decode { -1 } else { 1 };
    
    // process each character in the message along with its index
    message.chars()
        .enumerate()
        .map(|(i, c)| {
            // calculate the shift amount based on the secret key and index
            let shift = secret.chars().nth(i % secret.len()).unwrap() as i32 - 97;
            
            // apply the shift to the character, wrapping around if necessary
            ((c as i32 - 97 + shift_factor * shift + 26) % 26 + 97) as u8 as char
        })
        .collect()
}

fn main() {
    let inputs = vec![
        ("snitch", "thepackagehasbeendelivered"),
        ("bond", "theredfoxtrotsquietlyatmidnight"),
        ("train", "murderontheorientexpress"),
        ("garden", "themolessnuckintothegardenlastnight"),
    ];

    for &(secret, message) in &inputs {
        let encoded = alphabet_cipher(secret, message, false);
        println!("Encoded: {}", encoded);
        println!("Decoded: {}", alphabet_cipher(secret, &encoded, true));
        println!();
    }
}
