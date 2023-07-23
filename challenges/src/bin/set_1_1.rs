use serialize::{DeSerialize, Serialize};

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let actual = hex.from_hex().to_base64();

    assert_eq!(actual, expected);

    println!("Convert hex to base64: OK!");
}
