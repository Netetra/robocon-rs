pub fn sized_slice<const SIZE: usize>(slice: &[u8]) -> Option<&[u8; SIZE]> {
    let sized_slice = slice.try_into().ok()?;
    Some(sized_slice)
}
