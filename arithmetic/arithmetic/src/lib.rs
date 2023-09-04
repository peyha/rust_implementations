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

pub fn extended_gcd(a: u64, b: u64) -> (i64, i64, i64){
    if a < b{
        let (d, u, v): (i64, i64, i64) = extended_gcd(b, a);
        return (d, v, u);
    }
    if b == 0{
        return (a as i64, 1, 0);
    }
    let (d, u, v): (i64, i64, i64) = extended_gcd(b, a % b);
    return (d, v, u-(a as i64)/(b as i64)*v);
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

    #[test]
    fn extended_gcd_test(){
        let (res_d, res_u, res_v) = extended_gcd(42, 150);
        let potential_d = 42 * res_u + 150 * res_v;
        assert_eq!(potential_d, res_d);
    }
}
