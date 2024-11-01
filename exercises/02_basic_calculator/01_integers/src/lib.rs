fn compute(a: u32, b: u32) -> u32 {
    // removing the u8 suffix would've also worked, bad test case
    a + b * 4u32
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
