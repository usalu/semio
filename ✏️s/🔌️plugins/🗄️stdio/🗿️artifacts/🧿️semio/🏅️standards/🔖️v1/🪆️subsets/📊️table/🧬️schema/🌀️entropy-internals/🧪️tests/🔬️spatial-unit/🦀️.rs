mod tests {
    use super::*;

    #[test]
    fn constant_image_is_rejected() {
        let pixels = vec![5.0; 16];
        let cfg = SpatialConfig::new(SpatialMethod::Global, 4).unwrap();
        assert!(matches!(entropy_2d(&pixels, 4, 4, cfg), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn noisy_image_has_higher_glcm_entropy_than_smooth_gradient() {
        // 🔐️ A checkerboard is NOT a good "high texture" counter-example here: it alternates
        // between exactly two values, so its dx=1 co-occurrence matrix is nearly deterministic
        // (low entropy). Genuine per-pixel noise, which visits many distinct adjacent-value
        // pairs unpredictably, is the correct high-entropy comparison against a smooth gradient.
        let width = 16;
        let height = 16;
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(9);
        let noise: Vec<f64> = (0..width * height).map(|_| rng.next_f64()).collect();
        let gradient: Vec<f64> = (0..width * height).map(|i| (i % width) as f64).collect();
        let cfg = SpatialConfig::new(SpatialMethod::Glcm { dx: 1, dy: 0 }, 4).unwrap();
        let h_noise = entropy_2d(&noise, width, height, cfg).unwrap().value;
        let h_gradient = entropy_2d(&gradient, width, height, cfg).unwrap().value;
        assert!(h_noise > h_gradient, "noise={h_noise} gradient={h_gradient}");
    }

    #[test]
    fn global_entropy_of_uniform_random_image_is_near_max() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let width = 32;
        let height = 32;
        let pixels: Vec<f64> = (0..width * height).map(|_| rng.next_f64()).collect();
        let cfg = SpatialConfig::new(SpatialMethod::Global, 8).unwrap();
        let est = entropy_2d(&pixels, width, height, cfg).unwrap();
        assert!(est.value > 0.8 * 8.0_f64.ln(), "got {}", est.value);
    }

    #[test]
    fn shape_mismatch_is_rejected() {
        let pixels = vec![1.0, 2.0, 3.0];
        let cfg = SpatialConfig::new(SpatialMethod::Global, 4).unwrap();
        assert!(matches!(entropy_2d(&pixels, 2, 2, cfg), Err(EntropyError::ShapeMismatch { .. })));
    }

    #[test]
    fn spatial_config_rejects_small_bins() {
        assert!(SpatialConfig::new(SpatialMethod::Global, 1).is_err());
    }
}
