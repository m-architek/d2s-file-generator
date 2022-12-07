pub trait ArrayOverwrite {
    fn overwrite_with(&mut self, other: &[u8], from_index: usize);
}

impl ArrayOverwrite for [u8] {

    fn overwrite_with(&mut self, other: &[u8], from_index: usize) {
        for (i, byte) in other.iter().enumerate() {
            self[i + from_index] = *byte;
        }
    }
}
