pub fn float_eq(a: f32, b: f32) -> bool {
    float_eq_eps(a, b, 0.0001)
}

pub fn float_eq_eps(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert!(float_eq(1.0, 1.0));
        assert!(float_eq(1.000000, 1.000001));
    }
}
