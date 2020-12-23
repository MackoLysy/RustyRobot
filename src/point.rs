use std::f64::consts::PI;

pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x_org: i64, y_org: i64) -> Point {
        Point { x: x_org, y: y_org }
    }

    pub fn caluclate_point(&mut self, distance: i64, angle: i64) {
        let rad = self.angle_to_radias(angle);
        self.x = (rad.cos() * distance as f64) as i64 + self.x;
        self.y = (rad.sin() * distance as f64) as i64 + self.y;
    }

    fn angle_to_radias(&self, angle: i64) -> f64 {
        (angle as f64 * PI) / 180.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    fn gen_point() -> Point {
        Point::new(0, 0)
    }
    #[test]
    fn simple() {
        let point = gen_point();
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);
    }

    #[test]
    fn calulcate_radias() {
        let point = gen_point();
        let rad = point.angle_to_radias(180);
        assert_approx_eq!(rad, 3.141, 2f64);
    }
    #[test]
    fn calulcate_radias_sin() {
        let point = gen_point();
        let rad = point.angle_to_radias(30);
        let sin = rad.sin();
        assert_approx_eq!(sin, 0.5, 2f64);
    }
    #[test]
    fn calulcate_radias_cos() {
        let point = gen_point();
        let rad = point.angle_to_radias(30);
        let sin = rad.cos();
        assert_approx_eq!(sin, 0.8660254, 2f64);
    }

    #[test]
    fn calucalte_point_simple() {
        let mut point = gen_point();
        point.caluclate_point(10, 0);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 0);
    }

    #[test]
    fn calucalte_point_simple1() {
        let mut point = gen_point();
        point.caluclate_point(10, 90);
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 10);
    }
}
