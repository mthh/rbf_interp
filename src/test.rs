#[cfg(test)]
mod test {
    use ::*;
    use num_traits::{Float, Zero};
    type Pt = PtValue<f64>;

    pub fn almost_equal<T>(a: T, b: T, epsilon: T) -> bool
        where T: Float
    {
        let diff = (a - b).abs();
        if a == b {
            true
        } else if a == Zero::zero() || b == Zero::zero() || diff < T::min_positive_value() {
            diff < (epsilon * T::min_positive_value())
        } else {
            (diff / T::min(a.abs() + b.abs(), T::max_value())) < epsilon
        }
    }

    #[test]
    fn test_radial_basis_func_linear() {
        let obs_pts = vec![Pt::new(0.0, 0.0, 0.0),
                           Pt::new(0.0, 100.0, 6.0),
                           Pt::new(75.0, 25.0, 3.1),
                           Pt::new(100.0, 75.0, 7.4)];
        let rbf = Rbf::new(&obs_pts, "linear", None);
        assert_eq!(true,
                   almost_equal(2.843937337, rbf.interp_point((0.0, 50.0)), 0.0000001));
        assert_eq!(true,
                   almost_equal(0.754167644, rbf.interp_point((12.0, 12.0)), 0.0000001));
    }

    #[test]
    fn test_radial_basis_func_cubic() {
        let obs_pts = vec![Pt::new(0.0, 0.0, 0.0),
                           Pt::new(0.0, 100.0, 6.0),
                           Pt::new(75.0, 25.0, 3.1),
                           Pt::new(100.0, 75.0, 7.4)];
        let rbf = Rbf::new(&obs_pts, "cubic", None);
        assert_eq!(true,
                   almost_equal(0.554789362, rbf.interp_point((0.0, 50.0)), 0.0000001));
        assert_eq!(true,
                   almost_equal(-0.181785359, rbf.interp_point((12.0, 12.0)), 0.0000001));
    }

    #[test]
    fn test_radial_basis_func_gaussian() {
        let obs_pts = vec![Pt::new(0.0, 0.0, 0.0),
                           Pt::new(0.0, 100.0, 6.0),
                           Pt::new(75.0, 25.0, 3.1),
                           Pt::new(100.0, 75.0, 7.4)];
        let rbf = Rbf::new(&obs_pts, "gaussian", None);
        assert_eq!(true,
                   almost_equal(3.494929342, rbf.interp_point((0.0, 50.0)), 0.0000001));
        assert_eq!(true,
                   almost_equal(0.777143813, rbf.interp_point((12.0, 12.0)), 0.0000001));
    }
}
