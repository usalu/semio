//! ✨️ Unit tests of the Rust twin of the effects module: the published words of the hash, every committed vector of the particle-motion case (hashes, sums and bins, births and lives exactly; positions within a billionth like the TypeScript suite, ages exactly; caps and ends exactly), the laws of births, motions and caps on drawn emitters, and the ban on platform transcendentals.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/✨️particle-motion/🔣️.json — numpy's answers (case ✨️particle-motion)
//! @see ../../🟦️.ts — the TypeScript twin, whose suite these tests follow

use super::*;
use crate::randomness::random_unit;
use crate::schema::tests::{entries, fixture, number, typed};
use crate::schema::DRIFTS;
use serde_json::Value;

const SEED: u32 = 20_261_003;
const ORIGIN: Point = Point { x: 412.5, y: 233.25 };
const EMITTERS: u32 = 24;
const TICKS: Ticks = 400;

fn drawn(motion: Drift, counter: u32) -> Emission {
    let draw = |lane: u32| random_unit(&[SEED, lane, counter]);
    Emission { motion, count: 1 + (draw(1) * 32.0).floor() as u32, life: (draw(2) * 128.0 + 1.0).floor() / 64.0, speed: (draw(3) * 200.0).floor(), spread: (draw(4) * 9.0).floor() / 8.0 }
}

fn counted(emission: Emission, births: &[Ticks], until: Option<Ticks>, tick: Ticks) -> Vec<Ticks> {
    let life = life_ticks(emission);
    let ages: Vec<Ticks> = births.iter().filter(|&&born| born <= tick && tick - born < life && until.is_none_or(|until| born < until)).map(|&born| tick - born).collect();
    let keep = ages.len().saturating_sub(swarm_of(emission) as usize);
    ages[keep..].to_vec()
}

fn ages(particles: &[Particle]) -> Vec<Ticks> {
    particles.iter().map(|particle| particle.age).collect()
}

fn ticks_of(value: &Value) -> Vec<Ticks> {
    entries(value).iter().map(|tick| number(tick) as Ticks).collect()
}

fn until_of(value: &Value) -> Option<Ticks> {
    value.as_f64().map(|until| until as Ticks)
}

fn alike(produced: &[Particle], expected: &Value, label: &str) {
    let expected: Vec<Particle> = typed(expected);
    assert_eq!(produced.len(), expected.len(), "{label}: count");
    for (index, (left, right)) in produced.iter().zip(&expected).enumerate() {
        assert_eq!(left.age, right.age, "{label}[{index}].age");
        for (field, a, b) in [("x", left.x, right.x), ("y", left.y, right.y), ("scale", left.scale, right.scale), ("rotation", left.rotation, right.rotation), ("opacity", left.opacity, right.opacity)] {
            assert!((a - b).abs() <= 1e-9, "{label}[{index}].{field}: {a} instead of {b}");
        }
    }
}

fn aged(ages: &[Ticks]) -> Vec<Particle> {
    ages.iter().enumerate().map(|(position, &age)| Particle { x: position as f64, y: 0.0, scale: 1.0, rotation: 0.0, opacity: 1.0, age }).collect()
}

#[test]
fn the_hash_reproduces_the_published_words() {
    assert_eq!([lowbias32(1), lowbias32(0xdead_beef), mix(0, 0), mix(1, 2), mix(mix(7, 3), 5)], [0x6889_90c0, 0xe628_c683, 0xe577_f3aa, 0xb111_e030, 0x5ab3_6a78]);
    assert_eq!((lowbias32(0), mix(0x9e37_79b9, 0xffff_ffff)), (0, 0));
    assert_ne!(mix(0, 0), 0);
}

#[test]
fn the_hash_yields_numpys_words_for_every_committed_word_and_chain() {
    let vectors = fixture("particle-motion");
    assert!(entries(&vectors["hashes"]).len() > 12);
    for vector in entries(&vectors["hashes"]) {
        let word = if vector["low"].is_number() {
            lowbias32(number(&vector["low"]) as u32)
        } else {
            let chain: Vec<u32> = entries(&vector["chain"]).iter().map(|link| number(link) as u32).collect();
            chain[1..].iter().fold(chain[0], |word, &link| mix(word, link))
        };
        assert_eq!(f64::from(word), number(&vector["expected"]), "{}", vector["id"]);
    }
}

#[test]
fn a_word_becomes_a_unit_by_the_randomness_modules_own_division_and_indices_enter_by_their_low_bits() {
    assert_eq!([unit(0), unit(2_147_483_648), unit(4_294_967_295)], [0.0, 0.5, 1.0 - 1.0 / 4_294_967_296.0]);
    assert_eq!(scattered(7, 3, 5), f64::from(0x5ab3_6a78_u32) / 4_294_967_296.0);
    assert_eq!(scattered(7, 3 + (1 << 32), 5), scattered(7, 3, 5));
    assert_eq!(scattered(7, -1, 5), unit(mix(mix(7, u32::MAX), 5)));
    assert_eq!(emitter_key(SEED, 3, 0, 128), mix(mix(mix(SEED, 3), 0), 128));
    assert_eq!(emitter_key(20_261_002, 3, 0, 128), 3_911_377_450);
    assert_eq!(emitter_key(SEED, 3, 0, -1), mix(mix(mix(SEED, 3), 0), u32::MAX));
}

#[test]
fn every_committed_run_sums_and_bins_as_numpy_does() {
    let vectors = fixture("particle-motion");
    for vector in entries(&vectors["uniformity"]) {
        let (key, lane, first, count) = (number(&vector["key"]) as u32, number(&vector["lane"]) as u32, number(&vector["first"]) as i64, number(&vector["count"]) as i64);
        if count > 70_000 {
            continue;
        }
        let mut bins = [0_u64; 16];
        let mut total = 0_u64;
        for index in first..first + count {
            let word = mix(mix(key, index as u32), lane);
            total += u64::from(word);
            bins[(unit(word) * 16.0).floor() as usize] += 1;
        }
        assert_eq!(total as f64, number(&vector["expected"]["total"]), "{}", vector["id"]);
        assert_eq!(bins.to_vec(), entries(&vector["expected"]["bins"]).iter().map(|bin| number(bin) as u64).collect::<Vec<_>>(), "{}", vector["id"]);
    }
}

#[test]
fn lives_swarms_and_periods_are_counted_in_whole_ticks_and_particles() {
    let of = |count: u32, life: f64| Emission { motion: Drift::Fall, count, life, speed: 0.0, spread: 0.0 };
    assert_eq!([of(1, 1.0), of(1, 1.6), of(1, 0.3), of(1, 0.008), of(1, 0.001), of(1, 0.4)].map(life_ticks), [64, 102, 19, 1, 1, 26]);
    assert_eq!([of(1, 1.0), of(32, 1.0), of(40, 1.0), of(0, 1.0)].map(swarm_of), [1, 32, EMITTER_CAP, 0]);
    assert_eq!([of(6, 1.0), of(32, 1.0), of(64, 1.0), of(1, 1.0), of(32, 0.25), of(5, 1.0), of(0, 1.0)].map(period_of), [11, 2, 2, 64, 1, 13, 64]);
}

#[test]
fn every_committed_emitter_is_born_and_lives_as_numpys_tick_by_tick_simulation_says() {
    let vectors = fixture("particle-motion");
    assert!(entries(&vectors["births"]).len() > 15);
    for vector in entries(&vectors["births"]) {
        let emission: Emission = typed(&vector["emitter"]);
        let (since, until, key) = (number(&vector["since"]) as Ticks, until_of(&vector["until"]), number(&vector["key"]) as u32);
        let expected = &vector["expected"];
        assert_eq!((life_ticks(emission), swarm_of(emission), period_of(emission)), (number(&expected["life"]) as Ticks, number(&expected["swarm"]) as u32, number(&expected["period"]) as Ticks), "{}", vector["id"]);
        let born: Vec<Ticks> = (0..number(&vector["births"]) as i64).map(|index| born_at(emission, since, key, index)).collect();
        assert_eq!(born, ticks_of(&expected["born"]), "{}", vector["id"]);
        let first = number(&vector["first"]) as Ticks;
        for (tick, alive) in (first..=number(&vector["last"]) as Ticks).zip(entries(&expected["ages"])) {
            assert_eq!(ages(&particles_of(emission, Point { x: 0.0, y: 0.0 }, Facing::Right, since, until, tick, key)), ticks_of(alive), "{} at {tick}", vector["id"]);
        }
    }
}

#[test]
fn every_birth_has_its_own_slot_in_the_order_of_the_indices() {
    for counter in 0..EMITTERS {
        let emission = drawn(Drift::Rise, counter);
        let period = period_of(emission);
        let mut last = -1;
        for index in 0..200 {
            let born = born_at(emission, 100, counter, index);
            assert!(born >= 100 + index * period && born < 100 + (index + 1) * period && born > last, "emitter {counter}, index {index}");
            last = born;
        }
    }
}

#[test]
fn a_continuous_emitter_shows_exactly_the_particles_a_count_over_every_index_finds_alive() {
    let mut seen = 0;
    for counter in 0..EMITTERS {
        for motion in [Drift::Fall, Drift::Rise, Drift::Drift] {
            let emission = drawn(motion, counter);
            let since = (random_unit(&[SEED, 5, counter]) * 50.0).floor() as Ticks;
            let until = if counter % 4 == 0 { None } else { Some(since + (random_unit(&[SEED, 6, counter]) * TICKS as f64).floor() as Ticks) };
            let births: Vec<Ticks> = (0..TICKS / period_of(emission) + 2).map(|index| born_at(emission, since, counter, index)).collect();
            for tick in since - 3..since + TICKS {
                let shown = ages(&particles_of(emission, ORIGIN, Facing::Right, since, until, tick, counter));
                assert_eq!(shown, counted(emission, &births, until, tick), "{motion:?} {counter} at {tick}");
                seen += shown.len();
            }
        }
    }
    assert!(seen > (EMITTERS as usize) * (TICKS as usize));
}

#[test]
fn nothing_shows_before_the_start_beyond_the_count_or_younger_before_elder() {
    for counter in 0..EMITTERS {
        for motion in DRIFTS {
            let emission = drawn(motion, counter);
            let until = 40 + 3 * Ticks::from(counter);
            assert!(particles_of(emission, ORIGIN, Facing::Right, 20, Some(until), 19, counter).is_empty());
            for tick in (20..20 + TICKS).step_by(3) {
                let particles = particles_of(emission, ORIGIN, Facing::Right, 20, Some(until), tick, counter);
                assert!(particles.len() <= emission.count as usize, "{motion:?} {counter} at {tick}");
                assert!(particles.windows(2).all(|pair| pair[1].age <= pair[0].age), "{motion:?} {counter} at {tick}");
                if matches!(motion, Drift::Fall | Drift::Rise | Drift::Drift) {
                    assert!(particles.iter().all(|particle| tick - particle.age < until && tick - particle.age >= 20), "{motion:?} {counter} at {tick}");
                }
            }
        }
    }
}

#[test]
fn every_committed_particle_lies_where_numpy_puts_it() {
    let vectors = fixture("particle-motion");
    assert!(entries(&vectors["motions"]).len() > 20);
    for vector in entries(&vectors["motions"]) {
        let emission: Emission = typed(&vector["emitter"]);
        let (origin, facing): (Point, Facing) = (typed(&vector["origin"]), typed(&vector["facing"]));
        let (since, until, key) = (number(&vector["since"]) as Ticks, until_of(&vector["until"]), number(&vector["key"]) as u32);
        for (tick, expected) in ticks_of(&vector["ticks"]).into_iter().zip(entries(&vector["expected"])) {
            alike(&particles_of(emission, origin, facing, since, until, tick, key), expected, &format!("{} at {tick}", vector["id"]));
        }
    }
}

#[test]
fn every_particle_is_finite_and_its_opacity_and_scale_stay_in_range() {
    for counter in 0..EMITTERS {
        for motion in DRIFTS {
            let emission = drawn(motion, counter);
            let facing = if counter % 2 == 0 { Facing::Right } else { Facing::Left };
            for tick in (0..TICKS).step_by(2) {
                for particle in particles_of(emission, ORIGIN, facing, 0, Some(100), tick, counter) {
                    assert!((0.0..=1.0).contains(&particle.opacity) && particle.scale > 0.0 && particle.scale <= 1.2, "{motion:?} {counter} at {tick}: {particle:?}");
                    assert!(particle.x.is_finite() && particle.y.is_finite() && particle.rotation.is_finite(), "{motion:?} {counter} at {tick}: {particle:?}");
                }
            }
        }
    }
}

#[test]
fn a_pet_facing_left_mirrors_its_particles_about_the_origin_exactly() {
    for counter in 0..EMITTERS {
        for motion in DRIFTS {
            let emission = drawn(motion, counter);
            let tick = 40 + Ticks::from(counter);
            let right = particles_of(emission, ORIGIN, Facing::Right, 0, Some(90), tick, counter);
            let left = particles_of(emission, ORIGIN, Facing::Left, 0, Some(90), tick, counter);
            assert_eq!(left.len(), right.len());
            for (mirrored, particle) in left.iter().zip(&right) {
                assert!((mirrored.x - ORIGIN.x + (particle.x - ORIGIN.x)).abs() <= 1e-9 && (mirrored.y - particle.y).abs() <= 1e-9 && (mirrored.scale - particle.scale).abs() <= 1e-9, "{motion:?} {counter}");
                assert_eq!((mirrored.opacity, mirrored.age), (particle.opacity, particle.age), "{motion:?} {counter}");
            }
        }
    }
}

#[test]
fn a_burst_throws_its_whole_swarm_at_once_and_an_orbit_fades_out_after_its_end() {
    let burst = Emission { motion: Drift::Burst, count: 12, life: 1.6, speed: 160.0, spread: 0.5 };
    let start = particles_of(burst, ORIGIN, Facing::Right, 30, None, 30, 21);
    assert_eq!(start.len(), 12);
    for (index, particle) in start.iter().enumerate() {
        assert_eq!((particle.x, particle.y, particle.scale, particle.opacity, particle.age), (ORIGIN.x, ORIGIN.y, 1.0, 1.0, 0));
        assert!(particle.rotation >= UP + (index as f64 / 12.0 - 0.5) * 0.5 && particle.rotation < UP + ((index + 1) as f64 / 12.0 - 0.5) * 0.5);
    }
    assert_eq!((particles_of(burst, ORIGIN, Facing::Right, 30, None, 131, 21).len(), particles_of(burst, ORIGIN, Facing::Right, 30, None, 132, 21).len(), particles_of(burst, ORIGIN, Facing::Right, 30, Some(10), 60, 21).len()), (12, 0, 12));
    let orbit = Emission { motion: Drift::Orbit, count: 5, life: 1.0, speed: 94.0, spread: 1.0 };
    assert_eq!(particles_of(orbit, ORIGIN, Facing::Right, 100, None, 100, 3).iter().map(|particle| particle.opacity).collect::<Vec<_>>(), [0.0; 5]);
    assert_eq!(particles_of(orbit, ORIGIN, Facing::Right, 100, Some(400), 404, 3)[0].opacity, 0.5);
    assert_eq!((particles_of(orbit, ORIGIN, Facing::Right, 100, Some(400), 400 + ORBIT_FADE - 1, 3).len(), particles_of(orbit, ORIGIN, Facing::Right, 100, Some(400), 400 + ORBIT_FADE, 3).len()), (5, 0));
}

#[test]
fn every_committed_cap_keeps_the_survivors_numpys_stable_sort_keeps() {
    let vectors = fixture("particle-motion");
    assert!(entries(&vectors["caps"]).len() > 10);
    for vector in entries(&vectors["caps"]) {
        let cap = number(&vector["cap"]).floor();
        let kept = capped(aged(&ticks_of(&vector["ages"])), if cap > 0.0 { cap as usize } else { 0 });
        assert_eq!(kept.iter().map(|particle| particle.x).collect::<Vec<_>>(), entries(&vector["expected"]).iter().map(number).collect::<Vec<_>>(), "{}", vector["id"]);
    }
}

#[test]
fn a_cap_keeps_the_youngest_in_their_order_and_the_earlier_of_two_of_one_age() {
    for counter in 0..400 {
        let size = (random_unit(&[SEED, 7, counter]) * 60.0).floor() as usize;
        let cap = (random_unit(&[SEED, 8, counter]) * 40.0).floor() as usize;
        let particles = aged(&(0..size).map(|index| (random_unit(&[SEED, 9, counter * 64 + index as u32]) * 12.0).floor() as Ticks).collect::<Vec<_>>());
        let mut order: Vec<usize> = (0..size).collect();
        order.sort_by_key(|&position| (particles[position].age, position));
        let mut expected: Vec<usize> = order.into_iter().take(cap).collect();
        expected.sort_unstable();
        assert_eq!(capped(particles, cap).iter().map(|particle| particle.x as usize).collect::<Vec<_>>(), expected, "crowd {counter}");
    }
    let stage: Vec<Particle> = (0..8).flat_map(|index| particles_of(Emission { motion: Drift::Orbit, count: 32, life: 1.0, speed: 60.0, spread: 1.0 }, ORIGIN, Facing::Right, index * 10, None, 200, index as u32)).collect();
    assert_eq!((stage.len(), STAGE_CAP), (256, 160));
    assert_eq!(capped(stage.clone(), STAGE_CAP), stage[96..].to_vec());
}

#[test]
fn every_committed_emitter_ends_at_numpys_tick() {
    let vectors = fixture("particle-motion");
    assert!(entries(&vectors["ends"]).len() > 12);
    for vector in entries(&vectors["ends"]) {
        let emission: Emission = typed(&vector["emitter"]);
        assert_eq!(emitter_ends(emission, number(&vector["since"]) as Ticks, until_of(&vector["until"])), until_of(&vector["expected"]), "{}", vector["id"]);
    }
}

#[test]
fn an_emitter_ends_at_the_first_tick_from_which_nothing_is_alive() {
    for counter in 0..EMITTERS {
        for motion in DRIFTS {
            let emission = drawn(motion, counter);
            let since = 10 + Ticks::from(counter);
            let until = since - 5 + (random_unit(&[SEED, 10, counter]) * 200.0).floor() as Ticks;
            let end = emitter_ends(emission, since, Some(until)).unwrap_or(Ticks::MIN);
            assert!(end >= since && end <= since.max(until) + life_ticks(emission).max(ORBIT_FADE), "{motion:?} {counter}");
            assert!((end..end + 2 * life_ticks(emission) + 3).all(|tick| particles_of(emission, ORIGIN, Facing::Right, since, Some(until), tick, counter).is_empty()), "{motion:?} {counter}");
            assert_eq!(emitter_ends(emission, since, None), if motion == Drift::Burst { Some(since + life_ticks(emission)) } else { None });
        }
    }
}

#[test]
fn the_constants_are_the_twins_and_the_module_calls_no_platform_transcendental() {
    assert_eq!((TURN_RADIANS.to_bits(), EMITTER_CAP, STAGE_CAP, ORBIT_FADE, FALL_FADE_IN), (0x4019_21fb_5444_2d18, 32, 160, 8, 4));
    assert_eq!(
        [AHEAD, DOWN, UP, FALL_SWAY, FALL_SWAY_SPEED, FALL_SWAY_RATE, RISE_WANDER, RISE_WANDER_RATE, RISE_ROCK, BURST_DRAG, BURST_GRAVITY, ORBIT_SQUASH, ORBIT_DEPTH, DRIFT_MEANDER, DRIFT_RATE],
        [0.0, 0.25, 0.75, 3.0, 60.0, 0.5, 24.0, 0.35, 0.1, 0.4, 120.0, 0.35, 0.15, 3.0, 0.5]
    );
    assert_eq!([LANE_BIRTH, LANE_HEADING, LANE_PACE, LANE_PHASE, LANE_LOOK], [0, 1, 2, 3, 4]);
    let source = include_str!("../../🦀️.rs");
    for call in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", ".max(0.0", ".min(1.0", "f64::max", "f64::min", "SystemTime", "Instant", "println!", "static "] {
        assert!(!source.contains(call), "{call}");
    }
}
