mod tests {
    use super::*;

    #[test]
    fn mi_of_independent_variables_is_near_zero() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let n = 5000;
        let x: Vec<u32> = (0..n).map(|_| rng.next_below(4) as u32).collect();
        let y: Vec<u32> = (0..n).map(|_| rng.next_below(4) as u32).collect();
        let est = mutual_information(&x, &y, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.02, "got {}", est.value);
    }

    #[test]
    fn mi_of_identical_variables_equals_entropy() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(2);
        let x: Vec<u32> = (0..2000).map(|_| rng.next_below(5) as u32).collect();
        let mi = mutual_information(&x, &x, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let counts = Counts::from_symbols(&x, 5).unwrap();
        let h = entropy_discrete(&counts_to_u64(counts.raw()), DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!((mi.value - h.value).abs() < 1e-9);
    }

    #[test]
    fn mi_rejects_length_mismatch() {
        assert!(matches!(mutual_information(&[0, 1], &[0], DiscreteMethod::Plugin, LogBase::Bits), Err(EntropyError::LengthMismatch { .. })));
    }

    #[test]
    fn cmi_zero_when_x_and_y_independent_given_z() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(3);
        let n = 4000;
        let z: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let x: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let y: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let est = conditional_mutual_information(&x, &y, &z, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn ksg1_matches_gaussian_closed_form() {
        // 🔐️ bivariate Gaussian with correlation rho: I(X;Y) = -0.5*ln(1-rho^2).
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(4);
        let rho = 0.6_f64;
        let n = 2000;
        let mut x = Vec::with_capacity(n);
        let mut y = Vec::with_capacity(n);
        for _ in 0..n {
            let z1 = rng.next_gaussian();
            let z2 = rng.next_gaussian();
            x.push(z1);
            y.push(rho * z1 + (1.0 - rho * rho).sqrt() * z2);
        }
        let cfg = KsgConfig::new(5, KsgVariant::Ksg1).unwrap();
        let est = mutual_information_knn(&x, &y, cfg).unwrap();
        let expected = -0.5 * (1.0 - rho * rho).ln();
        assert!((est.value - expected).abs() < 0.05, "got {} expected {}", est.value, expected);
    }

    #[test]
    fn ksg2_matches_gaussian_closed_form() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(5);
        let rho = 0.5_f64;
        let n = 2000;
        let mut x = Vec::with_capacity(n);
        let mut y = Vec::with_capacity(n);
        for _ in 0..n {
            let z1 = rng.next_gaussian();
            let z2 = rng.next_gaussian();
            x.push(z1);
            y.push(rho * z1 + (1.0 - rho * rho).sqrt() * z2);
        }
        let cfg = KsgConfig::new(5, KsgVariant::Ksg2).unwrap();
        let est = mutual_information_knn(&x, &y, cfg).unwrap();
        let expected = -0.5 * (1.0 - rho * rho).ln();
        assert!((est.value - expected).abs() < 0.08, "got {} expected {}", est.value, expected);
    }

    #[test]
    fn ksg_mi_of_independent_gaussians_is_near_zero() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(6);
        let n = 1500;
        let x: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let y: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
        let cfg = KsgConfig::new(5, KsgVariant::Ksg1).unwrap();
        let est = mutual_information_knn(&x, &y, cfg).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn total_correlation_of_independent_variables_is_near_zero() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(7);
        let n = 3000;
        let a: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
        let b: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
        let c: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
        let est = total_correlation(&[&a, &b, &c], &[3, 3, 3], LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn dual_total_correlation_requires_at_least_two_variables() {
        let a = [0u32, 1, 0, 1];
        assert!(matches!(dual_total_correlation(&[&a], &[2], LogBase::Nats), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn o_information_of_redundant_copy_is_positive() {
        // 🔐️ X1=X2=X3 (perfect redundancy): O-information should be strongly positive.
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(8);
        let x: Vec<u32> = (0..2000).map(|_| rng.next_below(4) as u32).collect();
        let est = o_information(&[&x, &x, &x], &[4, 4, 4], LogBase::Nats).unwrap();
        assert!(est.value > 0.5, "got {}", est.value);
    }

    #[test]
    fn pack_symbols_rejects_overflow() {
        let a = [0u32];
        let result = pack_symbols(&[&a, &a], &[u32::MAX as usize, u32::MAX as usize]);
        assert!(result.is_err());
    }
}
