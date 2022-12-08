pub trait ArrayOverwrite<T> {
    fn overwrite_with(&mut self, other: &[T], from_index: usize);
}

impl ArrayOverwrite<u8> for [u8] {

    fn overwrite_with(&mut self, other: &[u8], from_index: usize) {
        for (i, byte) in other.iter().enumerate() {
            self[i + from_index] = *byte;
        }
    }
}

pub trait WithPadding {
    fn with_padding<const COUNT: usize>(&self) -> [u8; COUNT];
}

impl WithPadding for String {

    fn with_padding<const COUNT: usize>(&self) -> [u8; COUNT] {
        let mut bytes: [u8; COUNT] = [0; COUNT];
        bytes.overwrite_with(self.as_bytes(), 0);
        bytes
    }
}
