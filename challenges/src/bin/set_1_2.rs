use serialize::DeSerialize;

pub trait Xorable {
    fn xor(&self, b: &Vec<u8>) -> Vec<u8>;
}

impl Xorable for Vec<u8> {
    fn xor(&self, b: &Vec<u8>) -> Vec<u8> {
        self.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
    }
}

fn main() {
    let a = "1c0111001f010100061a024b53535009181c".from_hex();
    let b = "686974207468652062756c6c277320657965".from_hex();

    let result = a.xor(&b);

    assert!(result == "746865206b696420646f6e277420706c6179".from_hex());

    println!("Fixed XOR: OK!");
}
