pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn gcd(a: u64, b: u64) -> u64{
    if a < b{
        return gcd(b, a)
    }
    if b == 0{
        return a
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        let result = gcd(1, 50);
        assert_eq!(result, 1);

        let result = gcd(25, 5);
        assert_eq!(result, 5);
    }
}
