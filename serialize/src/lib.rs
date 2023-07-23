const BASE64_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE16_ALPHABET: &[u8] = b"0123456789abcdef";

pub trait Serialize {
    fn to_hex(&self) -> String;
    fn to_base64(&self) -> String;
}

impl Serialize for [u8] {
    fn to_hex(&self) -> String {
        self.iter()
            .fold(String::with_capacity(self.len() * 2), |mut string, byte| {
                let first = byte >> 4;
                let second = byte & 0b0000_1111;

                string.push(BASE16_ALPHABET[first as usize] as char);
                string.push(BASE16_ALPHABET[second as usize] as char);
                string
            })
    }
    fn to_base64(&self) -> String {
        self.iter().enumerate().fold(
            String::with_capacity(self.len() * 4 / 3),
            |mut base64, (i, byte)| {
                match i % 3 {
                    0 => {
                        let index = (*byte >> 2) as usize; //                           1111_1100 -> 0011_1111
                        base64.push(BASE64_ALPHABET[index] as char);
                    }
                    1 => {
                        let previous_2bits = (self[i - 1] & 0b0000_0011) << 4; //      0000_0011 -> 0011_0000
                        let first_4bits = byte >> 4; //                                 1111_0000 -> 0000_1111
                        let first_char_index = previous_2bits | first_4bits; //         0011_0000  | 0000_1111 = 0011_1111
                        base64.push(BASE64_ALPHABET[first_char_index as usize] as char);

                        let last_4bits = (byte & 0b0000_1111) << 2; //                  0000_1111 -> 0011_1100
                        let next_2bits = self[i + 1] >> 6; //                          1100_0000 -> 0000_0011
                        let second_char_index = last_4bits | next_2bits; //             0011_1100 | 0000_0011 = 0011_1111
                        base64.push(BASE64_ALPHABET[second_char_index as usize] as char);
                    }
                    2 => {
                        let index = byte & 0b0011_1111;
                        base64.push(BASE64_ALPHABET[index as usize] as char);
                    }
                    _ => {}
                };

                base64
            },
        )
    }
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
