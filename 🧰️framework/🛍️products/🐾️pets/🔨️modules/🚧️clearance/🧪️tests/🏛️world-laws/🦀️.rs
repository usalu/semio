//! 🏛️ Unit suite of the clearance twin, laws of the world: lanes and metrics, exact operations without side effects, and the reference world that keeps every pair of bodies apart, with the `quick` and `exhaustive` levels.
//!
//! @see ../🌍️reference-world/🦀️.rs — the world under test
//! @see ../🧰️clearance-tools/🦀️.rs — the sweeps and the ablations

use super::*;

#[test]
fn the_high_lane_of_a_floater_lies_nine_tenths_of_its_height_and_margin_higher() {
    assert_eq!(lane_lift(48.0, 4.0), LANE_LIFT * 52.0);
    assert_eq!(lane_lift(40.0, 0.0), 36.0);
}

#[test]
fn a_run_is_tallied_tick_by_tick() {
    let apart = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 20.0, 0.0, 30.0, 10.0)];
    let near = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 11.0, 0.0, 30.0, 10.0)];
    let over = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 9.0, 0.0, 30.0, 10.0), body("c", 9.5, 0.0, 12.0, 10.0)];
    let mut tally = TALLY;
    tally = tallied(tally, &apart, 2.0, 0, 1);
    assert_eq!(tally, Tally { ticks: 1, actors: 2, overlaps: 0, nears: 0, poofs: 0, waits: 1 });
    tally = tallied(tally, &near, 2.0, 1, 0);
    assert_eq!(tally, Tally { ticks: 2, actors: 4, overlaps: 0, nears: 1, poofs: 1, waits: 1 });
    tally = tallied(tally, &over, 2.0, 2, 3);
    assert_eq!(tally, Tally { ticks: 3, actors: 7, overlaps: 1, nears: 1, poofs: 3, waits: 4 });
    assert_eq!(per_million(3, 7), 3_000_000.0 / 7.0);
    assert_eq!(per_million(3, 0), 0.0);
}

#[test]
fn the_twin_uses_nothing_but_exact_operations_and_javascripts_extremes() {
    const SOURCE: &str = include_str!("../../🦀️.rs");
    for banned in [
        ".sin(",
        ".cos(",
        ".tan(",
        ".atan2(",
        ".exp(",
        ".ln(",
        ".log",
        ".powf(",
        ".powi(",
        ".mul_add(",
        ".hypot(",
        ".sqrt(",
        ".max(",
        ".min(",
        ".clamp(",
        "f64::max",
        "f64::min",
        "static ",
        "thread_local!",
        "HashMap",
        "BTreeMap",
        "HashSet",
        "std::time",
        "println!",
        "eprintln!",
        "dbg!",
    ] {
        assert!(!SOURCE.contains(banned), "{banned}");
    }
}

#[test]
fn the_reference_world_keeps_every_pair_of_bodies_apart_at_the_end_of_every_tick_and_the_order_on_every_perch() {
    kept_apart(2, 1500);
}

#[test]
fn the_reference_world_holds_with_claims_cut_into_slices_of_four_ticks_as_well() {
    kept_apart_in_slices(1, 1500);
}

#[test]
fn the_reference_world_is_judged_alike_by_an_overlap_test_that_measures_the_area_of_the_intersection() {
    judged_alike(600);
}

#[test]
fn the_reference_world_is_a_pure_function_of_its_seed_and_rules() {
    reproduced(800);
}

#[test]
fn the_reference_world_lets_bodies_overlap_as_soon_as_a_guarding_rule_is_switched_off() {
    bitten(0, 0);
}

#[test]
fn the_reference_world_needs_the_poof_where_planning_fails() {
    poofs_needed(1, 600);
}

#[test]
fn the_reference_world_keeps_bodies_apart_without_the_quality_rules_too() {
    quality_optional(1, 1000);
}

mod quick {
    use super::*;

    #[test]
    fn generated_cases_agree_with_brute_force() {
        overlaps_agree_with_areas(400);
        slots_agree_with_the_lattice(400);
        claims_agree_with_time_lines(400);
        strides_agree_with_the_lattice(400);
        seatings_agree_with_every_pooling(400);
        scoots_stay_apart(400);
        pushes_free_or_nothing(400);
        walkers_kept_in_order(4000);
    }

    #[test]
    fn the_reference_world_holds_in_more_and_longer_runs() {
        kept_apart(6, 5000);
        kept_apart_in_slices(3, 5000);
        judged_alike(3000);
        reproduced(5000);
        poofs_needed(6, 6000);
        quality_optional(6, 6000);
    }
}

mod exhaustive {
    use super::*;

    #[test]
    fn generated_cases_agree_with_brute_force() {
        overlaps_agree_with_areas(4000);
        slots_agree_with_the_lattice(4000);
        claims_agree_with_time_lines(4000);
        strides_agree_with_the_lattice(4000);
        seatings_agree_with_every_pooling(4000);
        scoots_stay_apart(4000);
        pushes_free_or_nothing(4000);
        walkers_kept_in_order(40000);
    }

    #[test]
    fn the_reference_world_holds_in_the_runs_of_the_research_prototype() {
        kept_apart(24, 30000);
        kept_apart_in_slices(6, 30000);
        judged_alike(30000);
        reproduced(30000);
        bitten(12, 15000);
        poofs_needed(12, 15000);
        quality_optional(12, 15000);
    }
}
