pub fn sum_offset<T>(data: &[u8], start: usize, offset: usize) -> T
where
    T: std::iter::Sum + From<u8>,
{
    data[start..start + offset]
        .iter()
        .copied()
        .map(From::from)
        .sum()
}
