//! 🎰️ Unit tests of the counter-based randomness: numpy's committed seed-sequence words, the laws of a pure keyed draw, the statistics of units and weighted picks, and the ban on platform randomness in the module.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/🎲️counter-randomness/🔣️.json — numpy's answers (case 🎲️counter-randomness)

use super::*;
use crate::schema::tests::{entries, fixture, number};
use serde_json::Value;

const SEED: u32 = 20_261_002;

fn key_of(value: &Value) -> Vec<u32> {
    entries(value).iter().map(word).collect()
}

fn word(value: &Value) -> u32 {
    value.as_u64().and_then(|word| u32::try_from(word).ok()).unwrap_or_else(|| panic!("not an unsigned 32-bit word: {value}"))
}

fn numbers(value: &Value) -> Vec<f64> {
    entries(value).iter().map(number).collect()
}

fn index_of(pick: Option<usize>) -> f64 {
    pick.map_or(-1.0, |index| index as f64)
}

#[test]
fn numpy_generates_the_same_words_for_every_committed_key() {
    let vectors = fixture("counter-randomness");
    assert!(entries(&vectors["words"]).len() > 15);
    for vector in entries(&vectors["words"]) {
        assert_eq!(random_words(&key_of(&vector["key"]), word(&vector["count"]) as usize), key_of(&vector["expected"]), "{}", vector["id"]);
    }
}

#[test]
fn the_key_of_one_zero_starts_with_the_documented_words_of_numpy() {
    assert_eq!(random_words(&[0], 4), [2_968_811_710, 3_677_149_159, 745_650_761, 2_884_920_346]);
}

#[test]
fn a_read_is_as_long_as_asked_and_a_longer_read_extends_a_shorter_one() {
    assert!(random_words(&[SEED, 1, 2], 0).is_empty());
    assert_eq!(random_words(&[SEED, 1, 2], 257).len(), 257);
    let key = [SEED, 3, 99];
    let again = vec![SEED, 3, 99];
    assert_eq!(random_words(&key, 12), random_words(&again, 12));
    assert_eq!(random_words(&key, 12)[..5], random_words(&key, 5)[..]);
}

#[test]
fn missing_words_count_as_zeros_up_to_the_pool_of_four_and_no_further() {
    assert_eq!(random_words(&[5], 8), random_words(&[5, 0], 8));
    assert_eq!(random_words(&[5], 8), random_words(&[5, 0, 0, 0], 8));
    assert_ne!(random_words(&[5], 8), random_words(&[5, 0, 0, 0, 0], 8));
    assert_eq!(random_words(&[], 8), random_words(&[0, 0, 0, 0], 8));
}

#[test]
fn one_flipped_key_bit_changes_about_half_of_the_output_bits() {
    let (mut flipped, mut trials) = (0u32, 0u32);
    for position in 0..3 {
        for bit in 0..32 {
            let mut key = [SEED, 7, 41];
            let before = random_words(&key, 4);
            key[position] ^= 1 << bit;
            let after = random_words(&key, 4);
            flipped += before.iter().zip(&after).map(|(left, right)| (left ^ right).count_ones()).sum::<u32>();
            trials += 4 * 32;
        }
    }
    let share = f64::from(flipped) / f64::from(trials);
    assert!(share > 0.47 && share < 0.53, "{share}");
}

#[test]
fn neighbouring_counters_and_streams_draw_unrelated_words() {
    let mut seen = std::collections::BTreeSet::new();
    for stream in 0..20 {
        for counter in 0..200 {
            seen.insert(random_words(&[SEED, stream, counter], 1)[0]);
        }
    }
    assert_eq!(seen.len(), 4000);
}

#[test]
fn a_unit_is_the_first_word_divided_by_two_to_the_thirty_second() {
    let vectors = fixture("counter-randomness");
    assert!(!entries(&vectors["units"]).is_empty());
    for vector in entries(&vectors["units"]) {
        let key = key_of(&vector["key"]);
        assert_eq!(random_unit(&key).to_bits(), number(&vector["expected"]).to_bits(), "{}", vector["id"]);
        assert_eq!(random_unit(&key), f64::from(random_words(&key, 1)[0]) / 4_294_967_296.0, "{}", vector["id"]);
    }
    assert!(!entries(&vectors["streams"]).is_empty());
    for vector in entries(&vectors["streams"]) {
        let units: Vec<f64> = (0..word(&vector["count"])).map(|counter| random_unit(&[word(&vector["seed"]), word(&vector["stream"]), counter])).collect();
        assert_eq!(units, numbers(&vector["expected"]), "{}", vector["id"]);
    }
}

#[test]
fn a_range_scales_the_unit_between_its_bounds() {
    let vectors = fixture("counter-randomness");
    assert!(!entries(&vectors["ranges"]).is_empty());
    for vector in entries(&vectors["ranges"]) {
        let (key, low, high) = (key_of(&vector["key"]), number(&vector["low"]), number(&vector["high"]));
        assert_eq!(random_between(&key, low, high).to_bits(), number(&vector["expected"]).to_bits(), "{}", vector["id"]);
        assert_eq!(random_between(&key, low, high), low + (high - low) * random_unit(&key), "{}", vector["id"]);
    }
    for counter in 0..2000 {
        let ascending = random_between(&[SEED, 1, counter], 2.0, 6.0);
        let descending = random_between(&[SEED, 1, counter], 6.0, 2.0);
        assert!((2.0..6.0).contains(&ascending));
        assert!(descending > 2.0 && descending <= 6.0);
        assert_eq!(ascending + descending, 8.0);
        assert_eq!(random_between(&[SEED, 1, counter], 5.0, 5.0), 5.0);
    }
}

#[test]
fn a_pick_follows_the_cumulative_weights_of_numpy_for_every_committed_stream() {
    let vectors = fixture("counter-randomness");
    assert!(entries(&vectors["picks"]).len() > 5);
    for vector in entries(&vectors["picks"]) {
        let weights = numbers(&vector["weights"]);
        let picks: Vec<f64> = (0..word(&vector["count"])).map(|counter| index_of(random_pick(&[word(&vector["seed"]), word(&vector["stream"]), counter], &weights))).collect();
        assert_eq!(picks, numbers(&vector["expected"]), "{}", vector["id"]);
    }
}

#[test]
fn no_positive_weight_means_no_pick_and_a_weight_that_is_not_positive_is_never_picked() {
    assert_eq!(random_pick(&[SEED], &[]), None);
    assert_eq!(random_pick(&[SEED], &[0.0, 0.0, 0.0]), None);
    assert_eq!(random_pick(&[SEED], &[-1.0, -2.0]), None);
    assert_eq!(random_pick(&[SEED], &[0.0, f64::NAN, -0.0]), None);
    for counter in 0..2000 {
        assert!(matches!(random_pick(&[SEED, 2, counter], &[0.0, 2.0, -5.0, 1.0, 0.0]), Some(1 | 3)));
        assert_eq!(random_pick(&[SEED, 2, counter], &[0.0, 0.0, 7.0]), Some(2));
    }
    for counter in 0..500 {
        assert_eq!(random_pick(&[SEED, 4, counter], &[1.0, 2.0, 4.0, 1.0]), random_pick(&[SEED, 4, counter], &[8.0, 16.0, 32.0, 8.0]));
    }
}

#[test]
fn a_pick_is_the_one_weighted_index_of_the_product_at_the_unit_of_the_key() {
    let weights = [0.5, 0.0, 1.25, 3.0, 0.0, 0.125];
    for counter in 0..500 {
        let key = [SEED, 5, counter];
        assert_eq!(random_pick(&key, &weights), weighted_index(&weights, random_unit(&key)));
        assert_eq!(random_unit(&key).to_bits(), unit_of(random_words(&key, 1)[0]).to_bits());
    }
    let even = [1.0, 1.0, 2.0];
    assert_eq!([weighted_index(&even, 0.0), weighted_index(&even, 0.25), weighted_index(&even, 0.5), weighted_index(&even, unit_of(u32::MAX))], [Some(0), Some(1), Some(2), Some(2)]);
    assert_eq!([weighted_index(&[], 0.5), weighted_index(&[0.0, -1.0], 0.5)], [None, None]);
}

#[test]
fn a_word_becomes_a_unit_by_one_exact_division_and_three_streams_are_reserved_at_the_top() {
    assert_eq!([unit_of(0), unit_of(2_147_483_648), unit_of(u32::MAX)], [0.0, 0.5, 1.0 - 1.0 / 4_294_967_296.0]);
    assert_eq!([STAGE_STREAM, CAST_STREAM, ROTATION_STREAM, CHEMISTRY_STREAM, GEAR_STREAM, MISCHIEF_STREAM], [0xffff_ffff, 0xffff_fffe, 0xffff_fffd, 0xffff_fffc, 0xffff_fffb, 0xffff_fffa]);
    let mut words: Vec<u32> = [STAGE_STREAM, CAST_STREAM, ROTATION_STREAM, CHEMISTRY_STREAM, GEAR_STREAM, MISCHIEF_STREAM, 0, 1, 2].iter().map(|&stream| random_words(&[SEED, stream, 0], 1)[0]).collect();
    words.sort_unstable();
    words.dedup();
    assert_eq!(words.len(), 9);
}

#[test]
fn the_module_calls_no_platform_random_source_clock_or_transcendental() {
    let source = include_str!("../../🦀️.rs");
    for call in [".sin(", ".cos(", ".tan(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", "rand::", "RandomState", "Instant", "SystemTime", "thread_local", "static mut"] {
        assert!(!source.contains(call), "{call}");
    }
}

mod quick {
    use super::super::*;

    const SEED: u32 = 20_261_002;
    const DRAWS: u32 = 20_000;

    #[test]
    fn units_lie_in_the_unit_interval_and_are_uniform_over_a_stream() {
        let mut sum = 0.0;
        let mut tenths = [0u32; 10];
        for counter in 0..DRAWS {
            let unit = random_unit(&[SEED, 0, counter]);
            assert!((0.0..1.0).contains(&unit));
            sum += unit;
            tenths[(unit * 10.0).floor() as usize] += 1;
        }
        assert!((sum / f64::from(DRAWS) - 0.5).abs() < 0.01);
        for count in tenths {
            assert!((f64::from(count) / f64::from(DRAWS) - 0.1).abs() < 0.01);
        }
    }

    #[test]
    fn picks_are_in_proportion_to_the_weights() {
        let weights = [1.0, 2.0, 3.0, 4.0];
        let mut counts = [0u32; 4];
        for counter in 0..DRAWS {
            counts[random_pick(&[SEED, 3, counter], &weights).unwrap_or_else(|| panic!("no pick at {counter}"))] += 1;
        }
        for (count, weight) in counts.iter().zip(weights) {
            assert!((f64::from(*count) / f64::from(DRAWS) - weight / 10.0).abs() < 0.015);
        }
    }
}
