use serialize::DeSerialize;
use xor::Xor;

const CIPHER: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn is_control(c: u8) -> bool {
    // https://utf8-chartable.de/unicode-utf8-table.pl?utf8=0x
    // '\n' is an exception
    c == 0x7F || (c < 0x20 && c != b'\n')
}

fn compare_score(key: &u8) -> u32 {
    let plain_text = CIPHER.from_hex().xor(&[*key; CIPHER.len()]);

    // If plain_text not ASCII, the key is not correct.
    if !plain_text.is_ascii() {
        return u32::MAX;
    }

    if plain_text.iter().any(|&c| is_control(c)) {
        return u32::MAX;
    }

    // Count occurrences and compare it to the expected frequency.

    0
}

fn break_encryption() -> u8 {
    (0u8..=255).min_by_key(compare_score).unwrap()
}

fn main() {
    break_encryption();
}
