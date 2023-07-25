use serialize::DeSerialize;
use xor::Xor;

fn main() {
    let a = "1c0111001f010100061a024b53535009181c".from_hex();
    let b = "686974207468652062756c6c277320657965".from_hex();

    let result = a.xor(&b);

    assert!(result == "746865206b696420646f6e277420706c6179".from_hex());

    println!("Fixed XOR: OK!");
}
