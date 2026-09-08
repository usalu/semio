mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn approx_eq_complex(a: Complex, b: Complex, tol: f64) -> bool {
        (a.re - b.re).abs() < tol && (a.im - b.im).abs() < tol
    }

    #[test]
    fn radix2_fft_matches_naive_dft() {
        for n in [2usize, 4, 8, 16, 32, 64] {
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(n as u64);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-9), "n={n}");
            }
        }
    }

    #[test]
    fn bluestein_fft_matches_naive_dft_for_arbitrary_lengths() {
        for n in [1usize, 3, 5, 6, 7, 11, 13, 17, 100, 101, 257] {
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(n as u64 + 1);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6), "n={n} a={a:?} b={b:?}");
            }
        }
    }

    #[test]
    fn forward_then_inverse_roundtrips() {
        for n in [8usize, 15, 32, 100] {
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(n as u64 + 99);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64(), rng.next_f64())).collect();
            let plan = Fft::new(n);
            let forward = plan.forward(&input);
            let back = plan.inverse(&forward);
            for (a, b) in input.iter().zip(back.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-8), "n={n}");
            }
        }
    }

    #[test]
    fn parseval_theorem_holds() {
        let n = 32;
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(3);
        let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, 0.0)).collect();
        let spectrum = Fft::new(n).forward(&input);
        let time_energy: f64 = input.iter().map(|c| c.norm_sq()).sum();
        let freq_energy: f64 = spectrum.iter().map(|c| c.norm_sq()).sum::<f64>() / n as f64;
        assert!((time_energy - freq_energy).abs() < 1e-9);
    }

    #[test]
    fn real_fft_returns_one_sided_spectrum_length() {
        let signal: Vec<f64> = (0..16).map(|i| (i as f64).sin()).collect();
        let spectrum = real_fft(&signal);
        assert_eq!(spectrum.len(), 16 / 2 + 1);
    }

    #[test]
    fn window_functions_produce_expected_length_and_endpoints() {
        for kind in [WindowKind::Rectangular, WindowKind::Hann, WindowKind::Hamming, WindowKind::Blackman, WindowKind::BlackmanHarris, WindowKind::Kaiser(8.0), WindowKind::Tukey(0.5)] {
            let w = window(kind, 64);
            assert_eq!(w.len(), 64);
            assert!(w.iter().all(|&v| v.is_finite() && v >= -1e-9));
        }
    }

    #[test]
    fn hann_window_endpoints_are_zero() {
        let w = window(WindowKind::Hann, 32);
        assert!(w[0].abs() < 1e-9);
        assert!(w[31].abs() < 1e-9);
    }

    #[test]
    fn rectangular_window_is_all_ones() {
        let w = window(WindowKind::Rectangular, 10);
        assert!(w.iter().all(|&v| (v - 1.0).abs() < 1e-12));
    }

    mod quick {
        use super::*;

        #[test]
        fn bluestein_matches_radix2_on_power_of_two_length() {
            let n = 64;
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(77);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64(), rng.next_f64())).collect();
            let radix2 = Fft::new(n).forward(&input);
            let bluestein = fft_bluestein(&input, false);
            for (a, b) in radix2.iter().zip(bluestein.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6));
            }
        }

        #[test]
        fn large_prime_length_dft_matches_naive() {
            let n = 101; // prime, not near a power of two
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(4242);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, 0.0)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6));
            }
        }
    }
}
