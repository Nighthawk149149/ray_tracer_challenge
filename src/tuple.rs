pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        self.w
    }
}

impl std::ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        Tuple::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
            self.w() * rhs,
        )
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        Tuple::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
            self.w() / rhs,
        )
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::vector::Vector;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 1.0);
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 0.0);
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Point::new(4.0, -4.0, 3.0);
        assert_eq!(p.x(), 4.0);
        assert_eq!(p.y(), -4.0);
        assert_eq!(p.z(), 3.0);
        assert_eq!(p.w(), 1.0);
    }

    #[test]
    fn vector_creates_tuples_with_w_0() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(v.x(), 4.0);
        assert_eq!(v.y(), -4.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 0.0);
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let a3 = a1 + a2;
        assert_eq!(a3.x(), 1.0);
        assert_eq!(a3.y(), 1.0);
        assert_eq!(a3.z(), 6.0);
        assert_eq!(a3.w(), 1.0);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let v = p1 - p2;
        assert_eq!(v.x(), -2.0);
        assert_eq!(v.y(), -4.0);
        assert_eq!(v.z(), -6.0);
        assert_eq!(v.w(), 0.0);
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let p2 = p - v;
        assert_eq!(p2.x(), -2.0);
        assert_eq!(p2.y(), -4.0);
        assert_eq!(p2.z(), -6.0);
        assert_eq!(p2.w(), 1.0);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -2.0);
        assert_eq!(v3.y(), -4.0);
        assert_eq!(v3.z(), -6.0);
        assert_eq!(v3.w(), 0.0);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::zero();
        let v = Vector::new(1.0, -2.0, 3.0);
        let v2 = zero - v;
        assert_eq!(v2.x(), -1.0);
        assert_eq!(v2.y(), 2.0);
        assert_eq!(v2.z(), -3.0);
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let a2 = -a;
        assert_eq!(a2.x(), -1.0);
        assert_eq!(a2.y(), 2.0);
        assert_eq!(a2.z(), -3.0);
        assert_eq!(a2.w(), 4.0);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let a2 = a * 3.5;
        assert_eq!(a2.x(), 3.5);
        assert_eq!(a2.y(), -7.0);
        assert_eq!(a2.z(), 10.5);
        assert_eq!(a2.w(), -14.0);
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let a2 = a * 0.5;
        assert_eq!(a2.x(), 0.5);
        assert_eq!(a2.y(), -1.0);
        assert_eq!(a2.z(), 1.5);
        assert_eq!(a2.w(), -2.0);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let a2 = a / 2.0;
        assert_eq!(a2.x(), 0.5);
        assert_eq!(a2.y(), -1.0);
        assert_eq!(a2.z(), 1.5);
        assert_eq!(a2.w(), -2.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_1_0() {
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_0_1() {
        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn computing_the_magnitude_of_vector_neg1_neg2_neg3() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Vector::new(4.0, 0.0, 0.0);
        let v2 = v.normalize();
        assert_eq!(v2.x(), 1.0);
        assert_eq!(v2.y(), 0.0);
        assert_eq!(v2.z(), 0.0);
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v2 = v.normalize();
        assert_eq!(v2.x(), 1 as f64 / (14 as f64).sqrt());
        assert_eq!(v2.y(), 2 as f64 / (14 as f64).sqrt());
        assert_eq!(v2.z(), 3 as f64 / (14 as f64).sqrt());
        assert_eq!(v2.w(), 0.0);
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v2 = v.normalize();
        assert_eq!(v2.magnitude(), 1.0);
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        let c = a.cross(&b);
        assert_eq!(c.x(), -1.0);
        assert_eq!(c.y(), 2.0);
        assert_eq!(c.z(), -1.0);
        assert_eq!(c.w(), 0.0);
        let d = b.cross(&a);
        assert_eq!(d.x(), 1.0);
        assert_eq!(d.y(), -2.0);
        assert_eq!(d.z(), 1.0);
        assert_eq!(d.w(), 0.0);
    }
}
