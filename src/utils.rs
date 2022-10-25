pub const EPSILON: f64 = 0.00001;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

// Tests
#[cfg(test)]
mod tests {
    use super::equal;

    #[test]
    fn equal_with_equal_values() {
        assert!(equal(1.0, 1.0));
    }

    #[test]
    fn equal_with_unequal_values() {
        assert!(!equal(1.0, 2.0));
    }

    #[test]
    fn equal_with_close_values() {
        assert!(equal(1.00002, 1.00001));
    }
}
