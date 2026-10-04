//! 🧮️ Unit tests of the trigonometry: the bit patterns of the constants, exact landmarks, symmetries and periods, numpy's committed answers and bit patterns, the platform's own sine and cosine as a second reference, the scalar blends, and the ban on platform transcendentals in the module.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/📐️turn-trigonometry/🔣️.json — numpy's answers and the committed bit patterns

use super::*;
use crate::schema::tests::{bits, entries, fixture, number};

const BOUND: f64 = 1e-12;
const ULP: f64 = f64::EPSILON;

fn dyadic(turns: i32) -> Vec<f64> {
    (-turns * 1024..=turns * 1024).map(|step| f64::from(step) / 1024.0).collect()
}

fn positive_zero(value: f64) -> bool {
    value.to_bits() == 0
}

#[test]
fn the_constants_are_the_published_doubles() {
    assert_eq!(TAU.to_bits(), std::f64::consts::TAU.to_bits());
    assert_eq!(bits(TAU), "401921fb54442d18");
    let published = [(SINE_1, "bfc5555555555549"), (SINE_2, "3f8111111110f8a6"), (SINE_3, "bf2a01a019c161d5"), (SINE_4, "3ec71de357b1fe7d"), (SINE_5, "be5ae5e68a2b9ceb"), (SINE_6, "3de5d93a5acfd57c")];
    for (constant, pattern) in published {
        assert_eq!(bits(constant), pattern);
    }
    let published = [(COSINE_1, "3fa555555555554c"), (COSINE_2, "bf56c16c16c15177"), (COSINE_3, "3efa01a019cb1590"), (COSINE_4, "be927e4f809c52ad"), (COSINE_5, "3e21ee9ebdb4b1c4"), (COSINE_6, "bda8fae9be8838d4")];
    for (constant, pattern) in published {
        assert_eq!(bits(constant), pattern);
    }
}

#[test]
fn quarter_turns_give_exactly_zero_one_and_minus_one_and_never_a_negative_zero() {
    for whole in -4..=4 {
        let whole = f64::from(whole);
        assert!(positive_zero(sin_turns(whole)));
        assert_eq!(cos_turns(whole).to_bits(), 1.0f64.to_bits());
        assert!(positive_zero(sin_turns(whole + 0.5)));
        assert_eq!(cos_turns(whole + 0.5).to_bits(), (-1.0f64).to_bits());
        assert!(positive_zero(cos_turns(whole + 0.25)));
        assert!(positive_zero(cos_turns(whole + 0.75)));
    }
    assert_eq!(sin_turns(0.25), 1.0);
    assert_eq!(sin_turns(0.75), -1.0);
    assert_eq!(sin_turns(-0.25), -1.0);
    assert_eq!(sin_turns(-0.75), 1.0);
    assert!(positive_zero(sin_turns(-0.0)));
}

#[test]
fn an_eighth_of_a_turn_is_within_one_unit_in_the_last_place_of_the_root_of_one_half() {
    for eighth in [1.0, 3.0, 5.0, 7.0, -1.0, -3.0, -5.0, -7.0] {
        assert!((sin_turns(eighth / 8.0).abs() - std::f64::consts::FRAC_1_SQRT_2).abs() <= ULP);
        assert!((cos_turns(eighth / 8.0).abs() - std::f64::consts::FRAC_1_SQRT_2).abs() <= ULP);
    }
}

#[test]
fn sine_is_odd_and_cosine_is_even_exactly() {
    for turns in dyadic(2).into_iter().chain([0.1, 0.3, 1.0 / 3.0, 1.0 / 7.0, 20.0 / 360.0, 0.999, 5e-324, 1e-20, 123_456.789, 4_503_599_627_370_497.0]) {
        assert_eq!(sin_turns(-turns) + sin_turns(turns), 0.0, "{turns}");
        assert_eq!(cos_turns(-turns).to_bits(), cos_turns(turns).to_bits(), "{turns}");
    }
}

#[test]
fn whole_turns_repeat_exactly_for_a_non_negative_angle() {
    for turns in dyadic(1).into_iter().filter(|angle| *angle >= 0.0) {
        for whole in [1.0, 2.0, 7.0, 1024.0, 1_073_741_824.0] {
            assert_eq!(sin_turns(turns + whole).to_bits(), sin_turns(turns).to_bits(), "{turns} + {whole}");
            assert_eq!(cos_turns(turns + whole).to_bits(), cos_turns(turns).to_bits(), "{turns} + {whole}");
        }
    }
}

#[test]
fn whole_turns_repeat_within_two_units_in_the_last_place_across_the_sign_change() {
    for turns in dyadic(1) {
        for whole in [-3.0, -1.0, 1.0, 3.0] {
            assert!((sin_turns(turns + whole) - sin_turns(turns)).abs() <= 2.0 * ULP, "{turns} + {whole}");
            assert!((cos_turns(turns + whole) - cos_turns(turns)).abs() <= 2.0 * ULP, "{turns} + {whole}");
        }
    }
}

#[test]
fn sine_and_cosine_stay_on_the_unit_circle() {
    let mut worst = 0.0f64;
    for turns in dyadic(2).into_iter().chain([0.1, 1.0 / 3.0, 1.0 / 7.0, 0.123_456_789, 7.654_321]) {
        worst = worst.max((sin_turns(turns) * sin_turns(turns) + cos_turns(turns) * cos_turns(turns) - 1.0).abs());
    }
    assert!(worst <= 3.0 * ULP, "{worst}");
}

#[test]
fn every_finite_angle_stays_within_the_unit_interval_and_no_other_angle_has_an_answer() {
    for turns in [5e-324, 1e-300, 7.450_580_596_923_828e-9, 1e15 + 0.125, 4_503_599_627_370_496.5, 9_007_199_254_740_992.0, 1e300, f64::MAX] {
        for angle in [turns, -turns] {
            assert!(sin_turns(angle).abs() <= 1.0, "{angle}");
            assert!(cos_turns(angle).abs() <= 1.0, "{angle}");
        }
    }
    assert!(sin_turns(5e-324) > 0.0);
    assert_eq!(cos_turns(5e-324), 1.0);
    for angle in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(sin_turns(angle).is_nan());
        assert!(cos_turns(angle).is_nan());
    }
}

#[test]
fn numpy_agrees_on_every_committed_angle() {
    let vectors = fixture("turn-trigonometry");
    assert!(entries(&vectors["angles"]).len() > 100);
    for vector in entries(&vectors["angles"]) {
        let turns = number(&vector["numerator"]) / number(&vector["denominator"]);
        let slack = if turns.abs() <= 8.0 { 1e-14 } else { 1e-9 };
        assert!((sin_turns(turns) - number(&vector["expected"]["sine"])).abs() <= slack, "{}", vector["id"]);
        assert!((cos_turns(turns) - number(&vector["expected"]["cosine"])).abs() <= slack, "{}", vector["id"]);
    }
}

#[test]
fn every_committed_angle_has_the_committed_bit_pattern() {
    let vectors = fixture("turn-trigonometry");
    let mut compared = 0;
    for vector in entries(&vectors["angles"]) {
        let turns = number(&vector["numerator"]) / number(&vector["denominator"]);
        assert_eq!(Some(bits(sin_turns(turns)).as_str()), vector["bits"]["sine"].as_str(), "{}", vector["id"]);
        assert_eq!(Some(bits(cos_turns(turns)).as_str()), vector["bits"]["cosine"].as_str(), "{}", vector["id"]);
        compared += 2;
    }
    assert!(compared > 200, "{compared}");
}

#[test]
fn numpy_agrees_along_every_committed_sweep() {
    let vectors = fixture("turn-trigonometry");
    let mut worst = 0.0f64;
    let mut compared = 0;
    for vector in entries(&vectors["sweeps"]) {
        let (denominator, first, last) = (number(&vector["denominator"]), number(&vector["first"]) as i64, number(&vector["last"]) as i64);
        let (sines, cosines) = (entries(&vector["expected"]["sines"]), entries(&vector["expected"]["cosines"]));
        assert_eq!(sines.len() as i64, last - first + 1);
        assert_eq!(cosines.len(), sines.len());
        for step in first..=last {
            let turns = step as f64 / denominator;
            worst = worst.max((sin_turns(turns) - number(&sines[(step - first) as usize])).abs());
            worst = worst.max((cos_turns(turns) - number(&cosines[(step - first) as usize])).abs());
            compared += 2;
        }
    }
    assert!(compared > 2000, "{compared}");
    assert!(worst <= BOUND, "{worst}");
}

#[test]
fn clamp_raises_to_the_low_bound_then_lowers_to_the_high_bound() {
    assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
    assert_eq!(clamp(-2.0, 0.0, 1.0), 0.0);
    assert_eq!(clamp(2.0, 0.0, 1.0), 1.0);
    assert_eq!(clamp(0.0, 2.0, -2.0), -2.0);
    assert_eq!(clamp(-5.0, 2.0, -2.0), -2.0);
    assert!(clamp(f64::NAN, 0.0, 1.0).is_nan());
    let vectors = fixture("turn-trigonometry");
    assert!(!entries(&vectors["clamps"]).is_empty());
    for vector in entries(&vectors["clamps"]) {
        assert_eq!(clamp(number(&vector["value"]), number(&vector["low"]), number(&vector["high"])), number(&vector["expected"]), "{}", vector["id"]);
    }
}

#[test]
fn lerp_starts_at_the_start_moves_in_proportion_and_extrapolates() {
    assert_eq!(lerp(2.0, 10.0, 0.0), 2.0);
    assert_eq!(lerp(2.0, 10.0, 1.0), 10.0);
    assert_eq!(lerp(2.0, 10.0, 0.5), 6.0);
    assert_eq!(lerp(2.0, 10.0, -0.5), -2.0);
    assert_eq!(lerp(2.0, 10.0, 1.5), 14.0);
    let vectors = fixture("turn-trigonometry");
    assert!(!entries(&vectors["lerps"]).is_empty());
    for vector in entries(&vectors["lerps"]) {
        assert!((lerp(number(&vector["from"]), number(&vector["to"]), number(&vector["amount"])) - number(&vector["expected"])).abs() <= BOUND, "{}", vector["id"]);
    }
}

#[test]
fn smoothstep_is_flat_at_both_ends_symmetric_about_one_half_and_rising() {
    assert_eq!(smoothstep(-3.0), 0.0);
    assert_eq!(smoothstep(0.0), 0.0);
    assert_eq!(smoothstep(0.5), 0.5);
    assert_eq!(smoothstep(1.0), 1.0);
    assert_eq!(smoothstep(3.0), 1.0);
    for step in 0..256 {
        let amount = f64::from(step) / 256.0;
        assert_eq!(smoothstep(amount) + smoothstep(1.0 - amount), 1.0);
        assert!(smoothstep(f64::from(step + 1) / 256.0) > smoothstep(amount));
    }
    let vectors = fixture("turn-trigonometry");
    assert!(!entries(&vectors["smoothsteps"]).is_empty());
    for vector in entries(&vectors["smoothsteps"]) {
        assert!((smoothstep(number(&vector["amount"])) - number(&vector["expected"])).abs() <= BOUND, "{}", vector["id"]);
    }
}

#[test]
fn the_arctangent_and_decay_constants_are_the_published_decimals() {
    let published = [(ARC_1, "3feffee6fb4c3c19"), (ARC_3, "bfd523a08398a655"), (ARC_5, "3fc70edc3bd59924"), (ARC_7, "bfb5cb46bacf7447"), (ARC_9, "3f9555cbe46f80c1"), (DECAY_2, "3fdeb851eb851eb8"), (DECAY_3, "3fce147ae147ae14")];
    for (constant, pattern) in published {
        assert_eq!(bits(constant), pattern);
    }
}

#[test]
fn the_axes_give_exact_directions_and_no_direction_is_a_negative_zero() {
    assert!(positive_zero(atan_turns(0.0, 0.0)));
    assert!(positive_zero(atan_turns(0.0, 3.0)));
    assert!(positive_zero(atan_turns(-0.0, 3.0)));
    assert!(positive_zero(atan_turns(-1e-300, 1e300)));
    assert_eq!(atan_turns(2.0, 0.0), 0.25);
    assert_eq!(atan_turns(0.0, -5.0), 0.5);
    assert_eq!(atan_turns(-0.0, -5.0), 0.5);
    assert_eq!(atan_turns(-7.0, 0.0), -0.25);
    assert_eq!(atan_turns(-1e-300, -1.0), 0.5);
}

#[test]
fn a_direction_is_odd_in_y_mirrored_in_x_and_blind_to_doubling() {
    for row in -12..=12 {
        for column in -12..=12 {
            let (y, x) = (f64::from(row) * 0.3, f64::from(column) * 0.3);
            let turns = atan_turns(y, x);
            assert!(turns > -0.5 && turns <= 0.5, "{y} {x}");
            assert_eq!(atan_turns(2.0 * y, 2.0 * x).to_bits(), turns.to_bits(), "{y} {x}");
            assert_eq!(atan_turns(y / 1024.0, x / 1024.0).to_bits(), turns.to_bits(), "{y} {x}");
            if turns != 0.5 {
                assert_eq!(atan_turns(-y, x) + turns, 0.0, "{y} {x}");
            }
            if y > 0.0 && x > 0.0 {
                assert_eq!(atan_turns(y, -x).to_bits(), (0.5 - turns).to_bits(), "{y} {x}");
            }
        }
    }
}

#[test]
fn numpy_agrees_on_every_committed_direction_within_the_stated_error() {
    let vectors = fixture("turn-trigonometry");
    assert!(entries(&vectors["arctangents"]).len() > 30);
    for vector in entries(&vectors["arctangents"]) {
        let turns = atan_turns(number(&vector["y"]), number(&vector["x"]));
        let apart = ((turns - number(&vector["reference"]) + 0.5).rem_euclid(1.0) - 0.5).abs();
        assert!(apart <= 2e-6, "{}: {apart}", vector["id"]);
        assert_eq!(Some(bits(turns).as_str()), vector["expected"]["bits"].as_str(), "{}", vector["id"]);
    }
}

#[test]
fn every_committed_lattice_has_the_committed_directions() {
    let vectors = fixture("turn-trigonometry");
    let mut compared = 0;
    for vector in entries(&vectors["arctangentGrids"]) {
        let (span, step) = (number(&vector["span"]) as i32, number(&vector["step"]));
        let expected = entries(&vector["expected"]);
        let mut index = 0;
        for row in -span..=span {
            for column in -span..=span {
                assert_eq!(atan_turns(f64::from(row) * step, f64::from(column) * step).to_bits(), number(&expected[index]).to_bits(), "{} {row} {column}", vector["id"]);
                index += 1;
            }
        }
        assert_eq!(index, expected.len());
        compared += index;
    }
    assert!(compared > 1000, "{compared}");
}

#[test]
fn the_decay_starts_at_one_falls_all_the_way_and_stays_near_the_exponential() {
    assert_eq!(fast_neg_exp(0.0), 1.0);
    assert_eq!(fast_neg_exp(-3.0), 1.0);
    assert_eq!(fast_neg_exp(f64::NAN), 1.0);
    assert_eq!(fast_neg_exp(f64::INFINITY), 0.0);
    let mut before = 1.0;
    for step in 1..=4096 {
        let value = fast_neg_exp(f64::from(step) / 128.0);
        assert!(value < before && value > 0.0, "{step}");
        before = value;
    }
    let vectors = fixture("turn-trigonometry");
    assert!(entries(&vectors["decays"]).len() > 15);
    for vector in entries(&vectors["decays"]) {
        let x = number(&vector["x"]);
        let value = fast_neg_exp(x);
        assert!((value - number(&vector["reference"])).abs() <= if x <= 1.0 { 6e-4 } else { 1.9e-2 }, "{}", vector["id"]);
        assert_eq!(Some(bits(value).as_str()), vector["expected"]["bits"].as_str(), "{}", vector["id"]);
    }
}

#[test]
fn the_module_calls_no_platform_transcendental_and_fuses_no_product() {
    let source = include_str!("../../🦀️.rs");
    for call in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".log", ".mul_add(", ".sin_cos(", "rand", "Instant", "SystemTime"] {
        assert!(!source.contains(call), "{call}");
    }
}

mod quick {
    use super::super::*;

    #[test]
    fn the_platform_sine_and_cosine_agree_for_65_537_angles_over_eight_turns_either_way() {
        let mut worst = 0.0f64;
        for step in -32_768..=32_768 {
            let turns = f64::from(step) / 4096.0 + f64::from(step) / 1_048_576.0;
            let radians = 2.0 * std::f64::consts::PI * turns;
            worst = worst.max((cos_turns(turns) - radians.cos()).abs()).max((sin_turns(turns) - radians.sin()).abs());
        }
        assert!(worst <= 1e-12, "{worst}");
    }

    #[test]
    fn the_platform_arctangent_agrees_within_two_millionths_of_a_turn_on_a_lattice_and_around_the_circle() {
        let mut worst = 0.0f64;
        for row in -200..=200 {
            for column in -200..=200 {
                let (y, x) = (f64::from(row) * 0.37, f64::from(column) * 0.37);
                let apart = (atan_turns(y, x) - y.atan2(x) / std::f64::consts::TAU + 0.5).rem_euclid(1.0) - 0.5;
                worst = worst.max(apart.abs());
            }
        }
        for step in -32_768..=32_768 {
            let turns = f64::from(step) / 65_537.0;
            let back = atan_turns(sin_turns(turns), cos_turns(turns));
            worst = worst.max(((back - turns + 0.5).rem_euclid(1.0) - 0.5).abs());
        }
        assert!(worst <= 2e-6 && worst > 1.5e-6, "{worst}");
    }

    #[test]
    fn the_platform_exponential_stays_within_the_stated_gaps_of_the_decay() {
        let (mut near, mut far) = (0.0f64, 0.0f64);
        for step in 0..=20_000 {
            let x = f64::from(step) / 1000.0;
            let gap = (fast_neg_exp(x) - (-x).exp()).abs();
            if x <= 1.0 {
                near = near.max(gap);
            }
            far = far.max(gap);
        }
        assert!(near <= 6e-4 && far <= 1.9e-2 && far > 1.8e-2, "{near} {far}");
    }
}
