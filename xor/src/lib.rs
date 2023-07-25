pub trait Xor {
    fn xor(&self, b: &[u8]) -> Vec<u8>;
}

impl Xor for Vec<u8> {
    fn xor(&self, b: &[u8]) -> Vec<u8> {
        self.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
    }
}
