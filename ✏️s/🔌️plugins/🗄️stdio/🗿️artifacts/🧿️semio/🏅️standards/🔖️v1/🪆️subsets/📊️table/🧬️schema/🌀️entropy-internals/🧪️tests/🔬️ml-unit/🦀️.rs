mod tests {
    use super::*;

    #[test]
    fn predictive_entropy_of_confident_prediction_is_zero() {
        let probs = [1.0, 0.0, 0.0];
        let est = predictive_entropy(&probs, 3, LogBase::Bits).unwrap();
        assert!(est[0].value.abs() < 1e-9);
    }

    #[test]
    fn predictive_entropy_of_uniform_is_max() {
        let probs = [0.25, 0.25, 0.25, 0.25];
        let est = predictive_entropy(&probs, 4, LogBase::Bits).unwrap();
        assert!((est[0].value - 2.0).abs() < 1e-9);
    }

    #[test]
    fn predictive_entropy_batch_shape() {
        let probs = [0.5, 0.5, 0.9, 0.1, 0.25, 0.75];
        let est = predictive_entropy(&probs, 2, LogBase::Bits).unwrap();
        assert_eq!(est.len(), 3);
    }

    #[test]
    fn bald_of_unanimous_ensemble_is_near_zero() {
        // 🔐️ every member gives the same confident prediction: no epistemic disagreement.
        let ensemble = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        let est = bald_mutual_information(&ensemble, 3, 2, LogBase::Bits).unwrap();
        assert!(est[0].value.abs() < 1e-9);
    }

    #[test]
    fn bald_of_disagreeing_ensemble_is_positive() {
        // 🔐️ members confidently disagree with each other but each is individually confident.
        let ensemble = [1.0, 0.0, 0.0, 1.0];
        let est = bald_mutual_information(&ensemble, 2, 2, LogBase::Bits).unwrap();
        assert!(est[0].value > 0.9, "got {}", est[0].value);
    }

    #[test]
    fn ece_of_perfectly_calibrated_predictions_is_zero() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1);
        let n = 5000;
        let confidences: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let correct: Vec<bool> = confidences.iter().map(|&c| rng.next_f64() < c).collect();
        let ece = expected_calibration_error(&confidences, &correct, 10).unwrap();
        assert!(ece < 0.05, "got {ece}");
    }

    #[test]
    fn ece_of_badly_miscalibrated_predictions_is_large() {
        let confidences = vec![0.95; 100];
        let correct = vec![false; 100];
        let ece = expected_calibration_error(&confidences, &correct, 10).unwrap();
        assert!(ece > 0.8, "got {ece}");
    }

    #[test]
    fn ece_rejects_length_mismatch() {
        assert!(expected_calibration_error(&[0.5], &[], 5).is_err());
    }

    #[test]
    fn ece_rejects_out_of_range_confidence() {
        assert!(expected_calibration_error(&[1.5], &[true], 5).is_err());
    }
}
