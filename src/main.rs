mod cryptogram;
use cryptogram::{Cryptogram, XOR};

fn main() {
    // let xor_cipher = XOR {
    //     key: String::from("caowei")
    // };
    // let message = "Hello, World!";
    // let encoded = xor_cipher.encode(message);
    // let decoded = xor_cipher.decode(&encoded);
    //
    // println!("Original: {}", message);
    // println!("Encoded: {}", encoded);
    // println!("Decoded: {}", decoded);
    let result = (7u64.pow(16) % 12);
    println!("{}", result)
}
