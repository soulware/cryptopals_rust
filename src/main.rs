extern crate rustc_serialize as serialize;

use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;


fn main() {
    set1_challenge1();
}

fn set1_challenge1() {
    println!("Set 1, Challenge 1 - ");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = input.from_hex().unwrap();
    let result = bytes.to_base64(base64::STANDARD);
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    if result == expected {
        println!("Success! {}", result);
    } else {
        println!("Failed??? {}", result);
    }
}
