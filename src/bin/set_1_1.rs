const BASE64_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub trait Serialize {
    fn to_base64(&self) -> String;
}

pub trait DeSerialize {
    fn from_hex(&self) -> Vec<u8>;
}

impl DeSerialize for &str {
    fn from_hex(&self) -> Vec<u8> {
        self.chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|byte| byte.iter().collect::<String>())
            .map(|byte| u8::from_str_radix(&byte, 16).unwrap())
            .collect()
    }
}

impl Serialize for [u8] {
    fn to_base64(&self) -> String {
        let bytes = self.clone();
        let mut base64 = String::with_capacity(bytes.len() * 4 / 3);

        for (i, byte) in bytes.iter().enumerate() {
            match i % 3 {
                0 => {
                    let index = (*byte >> 2) as usize; //                           1111_1100 -> 0011_1111
                    base64.push(BASE64_ALPHABET[index] as char);
                }
                1 => {
                    let previous_2bits = (bytes[i - 1] & 0b0000_0011) << 4; //      0000_0011 -> 0011_0000
                    let first_4bits = byte >> 4; //                                 1111_0000 -> 0000_1111
                    let first_char_index = previous_2bits | first_4bits; //         0011_0000  | 0000_1111 = 0011_1111
                    base64.push(BASE64_ALPHABET[first_char_index as usize] as char);

                    let last_4bits = (byte & 0b0000_1111) << 2; //                  0000_1111 -> 0011_1100
                    let next_2bits = bytes[i + 1] >> 6; //                          1100_0000 -> 0000_0011
                    let second_char_index = last_4bits | next_2bits; //             0011_1100 | 0000_0011 = 0011_1111
                    base64.push(BASE64_ALPHABET[second_char_index as usize] as char);
                }
                2 => {
                    let index = byte & 0b0011_1111;
                    base64.push(BASE64_ALPHABET[index as usize] as char);
                }
                _ => {}
            }
        }

        base64
    }
}

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let actual = hex.from_hex().to_base64();

    assert_eq!(actual, expected);

    println!("Convert hex to base64: OK!");
}
