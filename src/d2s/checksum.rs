pub fn calculate_checksum(bytes: &Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    for byte in bytes {
        sum = sum.rotate_left(1);
        sum += Into::<u32>::into(*byte);
    }
    sum
}
