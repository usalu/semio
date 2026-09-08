mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn binary_string(s: &str) -> Vec<u32> {
        s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect()
    }

    #[test]
    fn lz76_matches_canonical_test_string() {
        // 🔐️ See `exhaustive::lz76_canonical_value_matches_verified_reference` for the full
        // cross-validation story behind this specific number.
        let s = binary_string("0001101001000101");
        assert_eq!(lz76_complexity(&s), 6);
    }

    #[test]
    fn lz76_constant_sequence_is_minimally_complex() {
        let s = vec![0u32; 200];
        assert!(lz76_complexity(&s) <= 3);
    }

    #[test]
    fn lz76_repetitive_much_lower_than_random_of_same_length() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(7);
        let n = 500;
        let repetitive: Vec<u32> = (0..n).map(|i| (i % 3) as u32).collect();
        let random: Vec<u32> = (0..n).map(|_| rng.next_below(8) as u32).collect();
        let c_rep = lz76_complexity(&repetitive);
        let c_rand = lz76_complexity(&random);
        assert!(c_rep * 3 < c_rand, "c_rep={c_rep} c_rand={c_rand}");
    }

    #[test]
    fn lempel_ziv_complexity_rejects_empty() {
        assert!(matches!(lempel_ziv_complexity(&[], false), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn lempel_ziv_complexity_raw_matches_lz76() {
        let s = binary_string("0001101001000101");
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert_eq!(est.value, 6.0);
        assert_eq!(est.n, s.len());
        assert_eq!(est.diagnostics[1], ("raw_complexity", 6.0));
    }

    #[test]
    fn lempel_ziv_complexity_normalized_is_zero_for_single_symbol_alphabet() {
        let s = vec![0u32; 50];
        let est = lempel_ziv_complexity(&s, true).unwrap();
        assert_eq!(est.value, 0.0);
        assert_eq!(est.diagnostics[0], ("alphabet_size", 1.0));
    }

    #[test]
    fn lempel_ziv_complexity_small_sample_warns() {
        let s = binary_string("0001101001000101");
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })));
    }

    #[test]
    fn lempel_ziv_complexity_large_sample_does_not_warn() {
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(3);
        let s: Vec<u32> = (0..200).map(|_| rng.next_below(4) as u32).collect();
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert!(est.warnings.is_empty());
    }

    #[test]
    fn lz78_empty_input_compresses_to_zero() {
        assert_eq!(Lz78Compressor.compressed_len(&[]), 0);
    }

    #[test]
    fn lz78_repetitive_input_compresses_shorter_than_random() {
        let repetitive = b"abababababababab".to_vec();
        let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(42);
        let random: Vec<u8> = (0..repetitive.len()).map(|_| rng.next_below(256) as u8).collect();
        let comp = Lz78Compressor;
        assert!(comp.compressed_len(&repetitive) <= comp.compressed_len(&random));
    }

    #[test]
    fn ncd_rejects_empty_inputs() {
        let comp = Lz78Compressor;
        assert!(matches!(ncd(&[], b"x", &comp), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(ncd(b"x", &[], &comp), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn ncd_of_a_string_with_itself_is_well_below_unrelated_random_strings() {
        let comp = Lz78Compressor;
        let text: Vec<u8> = b"the quick brown fox jumps over the lazy dog ".repeat(8);
        let d_self = ncd(&text, &text, &comp).unwrap();

        let mut rng_a = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(1234);
        let a: Vec<u8> = (0..text.len()).map(|_| rng_a.next_below(256) as u8).collect();
        let mut rng_b = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(999_999);
        let b: Vec<u8> = (0..text.len()).map(|_| rng_b.next_below(256) as u8).collect();
        let d_diff = ncd(&a, &b, &comp).unwrap();

        assert!(d_self < d_diff, "d_self={d_self} d_diff={d_diff}");
        assert!(d_diff > 0.5, "expected two unrelated random byte strings to be far apart: {d_diff}");
    }

    #[test]
    fn ncd_is_bounded_below_by_zero() {
        let comp = Lz78Compressor;
        let x = b"identical payload identical payload".to_vec();
        let d = ncd(&x, &x, &comp).unwrap();
        assert!(d >= 0.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn lz76_is_non_decreasing_in_sequence_length_for_a_growing_random_stream() {
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(11);
            let full: Vec<u32> = (0..300).map(|_| rng.next_below(5) as u32).collect();
            let mut prev = lz76_complexity(&full[..1]);
            for len in [10, 50, 100, 200, 300] {
                let c = lz76_complexity(&full[..len]);
                assert!(c >= prev, "len={len} c={c} prev={prev}");
                prev = c;
            }
        }

        #[test]
        fn lz78_concatenation_never_shrinks_relative_to_either_half() {
            let comp = Lz78Compressor;
            let mut rng = crate::standards::v1::subsets::table::schema::entropy_internals::numeric::Xorshift64::new(55);
            for _ in 0..20 {
                let n = 20 + rng.next_below(80);
                let x: Vec<u8> = (0..n).map(|_| rng.next_below(256) as u8).collect();
                let m = 20 + rng.next_below(80);
                let y: Vec<u8> = (0..m).map(|_| rng.next_below(256) as u8).collect();
                let mut xy = x.clone();
                xy.extend_from_slice(&y);
                let cx = comp.compressed_len(&x);
                let cy = comp.compressed_len(&y);
                let cxy = comp.compressed_len(&xy);
                assert!(cxy >= cx.max(cy), "cxy={cxy} cx={cx} cy={cy}");
            }
        }
    }

    // #region 🔖️Exhaustive
    /// 🔐️ Brute-force validation of [`lz76_complexity`] against an independent definitional
    /// oracle, over every binary string of length `1..=12` (`4094` strings). This measure is
    /// well known to be easy to get off-by-one wrong, so the incremental (fast) implementation is
    /// checked against a slow, obviously-correct-by-construction parser rather than trusted on
    /// its own.
    mod exhaustive {
        use super::*;

        /// 🔐️ Naive contiguous-substring test, `O(n*m)`, used only by the brute-force oracle
        /// below (never on a hot path).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn contains(haystack: &[u32], needle: &[u32]) -> bool {
            if needle.is_empty() {
                return true;
            }
            if needle.len() > haystack.len() {
                return false;
            }
            (0..=haystack.len() - needle.len()).any(|start| &haystack[start..start + needle.len()] == needle)
        }

        /// 🔐️ Definitional LZ76 phrase count: repeatedly takes the shortest prefix-extension of
        /// the unparsed remainder that is not already a substring of (parsed history + candidate
        /// minus its own last symbol), i.e. the shortest prefix of the remainder not found
        /// anywhere in the string up to (and including) the position just before the prefix's
        /// last symbol. Deliberately independent of [`lz76_complexity`]'s control flow.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn brute_force_lz76(s: &[u32]) -> usize {
            let n = s.len();
            if n == 0 {
                return 0;
            }
            let mut phrases = 0usize;
            let mut pos = 0usize;
            while pos < n {
                let mut len = 1usize;
                loop {
                    if pos + len > n {
                        len = n - pos;
                        break;
                    }
                    let candidate = &s[pos..pos + len];
                    let haystack = &s[0..pos + len - 1];
                    if !contains(haystack, candidate) {
                        break;
                    }
                    len += 1;
                }
                phrases += 1;
                pos += len;
            }
            phrases
        }

        #[test]
        fn lz76_matches_definitional_brute_force_for_every_short_binary_string() {
            let mut checked = 0usize;
            for len in 1..=12usize {
                for v in 0..(1u32 << len) {
                    let s: Vec<u32> = (0..len as u32).map(|b| (v >> b) & 1).collect();
                    let incremental = lz76_complexity(&s);
                    let brute = brute_force_lz76(&s);
                    assert_eq!(incremental, brute, "len={len} v={v} s={s:?}");
                    checked += 1;
                }
            }
            assert_eq!(checked, 8190); // 2^1 + 2^2 + ... + 2^12
        }

        #[test]
        fn lz76_canonical_value_matches_verified_reference() {
            // 🔐️ The literal incremental-parsing pseudocode, cross-checked against
            // `brute_force_lz76` above on all 8190 binary strings of length 1..=12 with zero
            // mismatches, computes `c = 6` for this string (not the `8` sometimes quoted for it
            // in secondary sources — that figure does not reproduce under this definition and
            // was rejected in favor of the exhaustively cross-validated result).
            let s = binary_string("0001101001000101");
            assert_eq!(lz76_complexity(&s), 6);
            assert_eq!(brute_force_lz76(&s), 6);
        }
    }
    // #endregion 🔖️Exhaustive
}
