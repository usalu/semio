//! 🧰️ Tools of the cases of the clearance unit suite: the ablations and the cheapest runs that bite per rule, the builders of boxes, bodies and sweeps, the runs of the reference world, and the generated cases compared with brute force.
//!
//! @see ../🌍️reference-world/🦀️.rs — the world the runs play

use super::*;

//#region 🔖️Tools of the cases
/// ✂️ A rule set with one rule switched off.
type Ablation = fn(Rules) -> Rules;

/// 🦷️ Per core rule the cheapest run in which the world without that rule lets bodies overlap (the TypeScript suite's `BITES`, found with `clearance_world.ts --teeth 40 --ticks 2500`): the rule switched off, the seed and a number of ticks a little beyond the first overlap.
const BITES: [(Ablation, u32, Ticks); 6] = [
    (|rules| Rules { guard: false, ..rules }, 12, 40),
    (|rules| Rules { corridors: false, ..rules }, 9, 120),
    (|rules| Rules { vetting: false, ..rules }, 23, 40),
    (|rules| Rules { rests: false, ..rules }, 21, 110),
    (|rules| Rules { projection: false, ..rules }, 6, 20),
    (|rules| Rules { eviction: false, ..rules }, 17, 40),
];

/// 🔲️ A box by its four edges.
pub(super) fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Extent {
    Extent { x0, y0, x1, y1 }
}

/// 🏷️ A body by its owner and the four edges of its box.
pub(super) fn body(owner: &str, x0: f64, y0: f64, x1: f64, y1: f64) -> Body {
    Body { owner: owner.to_string(), extent: rect(x0, y0, x1, y1) }
}

/// 👥️ A pair by its two owners.
pub(super) fn pair(first: &str, second: &str) -> Pair {
    Pair { first: first.to_string(), second: second.to_string() }
}

/// 🔤️ Owned slugs of a list of names.
pub(super) fn slugs(names: &[&str]) -> Vec<Slug> {
    names.iter().map(|name| (*name).to_string()).collect()
}

/// 🎰️ A deterministic stream of whole numbers in `[0, bound)`, the TypeScript suite's 32-bit linear congruential generator, so both suites draw the same cases.
struct Stream(u32);

impl Stream {
    /// ➡️ The next whole number below `bound`.
    fn next(&mut self, bound: f64) -> f64 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        ((f64::from(self.0) / 4_294_967_296.0) * bound).floor()
    }
}

/// 🟰️ Whether two boxes share an area, decided by the size of their intersection instead of by comparisons of edges.
fn sharing(one: Extent, other: Extent) -> bool {
    one.x1.min(other.x1) - one.x0.max(other.x0) > 0.0 && one.y1.min(other.y1) - one.y0.max(other.y0) > 0.0
}

/// 🧱️ A few boxes on the quarter-pixel lattice around a body that is 40 wide and 50 tall at (100, 250).
fn scattered(stream: &mut Stream, count: f64) -> Vec<Extent> {
    let mut boxes = Vec::new();
    for _ in 0..count as usize {
        let x0 = stream.next(1200.0) / 4.0 - 50.0;
        let y0 = 150.0 + stream.next(800.0) / 4.0;
        let x1 = x0 + 4.0 + stream.next(240.0) / 4.0;
        let y1 = y0 + 4.0 + stream.next(240.0) / 4.0;
        boxes.push(rect(x0, y0, x1, y1));
    }
    boxes
}

/// 🛤️ The boxes of a 40 × 50 body that moves by a fixed step per tick.
pub(super) fn marching(x: f64, y: f64, dx: f64, dy: f64, ticks: usize) -> Vec<Extent> {
    (0..ticks).map(|tick| tick as f64).map(|tick| rect(x + dx * tick, y + dy * tick, x + dx * tick + 40.0, y + dy * tick + 50.0)).collect()
}

/// 🕹️ The point the hop or fall of a vector plans at the end of a tick.
pub(super) fn feet(x: f64, y: f64) -> Point {
    Point { x, y }
}

/// 🌐️ Holds every world of the seeds to the invariant and the order, for a number of ticks each, and that the world was busy.
pub(super) fn kept_apart(worlds: u32, ticks: Ticks) {
    let mut sum = Counts::default();
    let mut actors = 0;
    for seed in 1..=worlds {
        let world = run_world(seed, ticks, RULES, 1);
        assert_eq!(world.tally.ticks, ticks as u64);
        assert_eq!(world.tally.overlaps, 0, "seed {seed}");
        assert_eq!(world.counts.disorders, 0, "seed {seed}");
        assert!((6..=10).contains(&world.pets.len()));
        for pet in &world.pets {
            assert!((28.0..=58.0).contains(&pet.size.width) && (40.0..=56.0).contains(&pet.size.height));
        }
        actors += world.tally.actors;
        for (total, count) in [&mut sum.falls, &mut sum.heads, &mut sum.steered, &mut sum.chutes, &mut sum.hops, &mut sum.glides, &mut sum.grabs, &mut sum.surveys, &mut sum.seated].into_iter().zip([
            world.counts.falls,
            world.counts.heads,
            world.counts.steered,
            world.counts.chutes,
            world.counts.hops,
            world.counts.glides,
            world.counts.grabs,
            world.counts.surveys,
            world.counts.seated,
        ]) {
            *total += count;
        }
    }
    assert!(actors > u64::from(worlds) * ticks as u64 * 5);
    for (name, total) in [("falls", sum.falls), ("heads", sum.heads), ("hops", sum.hops), ("glides", sum.glides), ("grabs", sum.grabs), ("surveys", sum.surveys), ("chutes", sum.chutes)] {
        assert!(total > 0, "{name}");
    }
}

/// 🍰️ Holds the worlds of the seeds with claims cut into slices of four ticks.
pub(super) fn kept_apart_in_slices(worlds: u32, ticks: Ticks) {
    for seed in 1..=worlds {
        let world = run_world(seed, ticks, RULES, 4);
        assert_eq!([world.tally.overlaps, world.counts.disorders], [0, 0], "seed {seed}");
    }
}

/// 🔬️ Holds the invariant to a second overlap test, the area of the intersection, tick by tick, in a world with every rule and in one without the guard.
pub(super) fn judged_alike(ticks: Ticks) {
    let mut world = open_world(3, RULES, 1);
    let mut bare = open_world(3, Rules { guard: false, ..RULES }, 1);
    let mut intersecting = 0;
    for _ in 0..ticks {
        tick_world(&mut world);
        tick_world(&mut bare);
        for (run, counted) in [(&world, false), (&bare, true)] {
            let bodies = bodies_of(run);
            let mut pairs = 0;
            for (first, one) in bodies.iter().enumerate() {
                pairs += bodies[first + 1..].iter().filter(|other| sharing(one.extent, other.extent)).count();
            }
            assert_eq!(overlaps(&bodies).len(), pairs);
            if counted {
                intersecting += pairs;
            }
        }
    }
    assert_eq!(world.tally.overlaps, 0);
    assert!(intersecting > 0);
    assert!(bare.tally.overlaps > 0);
}

/// 🪞️ Holds that a world is a pure function of its seed and rules.
pub(super) fn reproduced(ticks: Ticks) {
    let one = run_world(5, ticks, RULES, 1);
    let other = run_world(5, ticks, RULES, 1);
    assert_eq!(other.tally, one.tally);
    assert_eq!(other.counts.listed(), one.counts.listed());
    assert_eq!(bodies_of(&other), bodies_of(&one));
    assert_ne!(bodies_of(&run_world(6, ticks, RULES, 1)), bodies_of(&one));
}

/// 💀️ Holds that every core rule bites: without it bodies overlap in its cheapest run (and, given `ablations`, in the sweep of the research prototype).
pub(super) fn bitten(ablations: u32, ablation_ticks: Ticks) {
    for (without, seed, ticks) in BITES {
        assert_eq!(run_world(seed, ticks, RULES, 1).tally.overlaps, 0, "seed {seed}");
        assert!(run_world(seed, ticks, without(RULES), 1).tally.overlaps > 0, "seed {seed}");
        let overlapping: u64 = (1..=ablations).map(|other| run_world(other, ablation_ticks, without(RULES), 1).tally.overlaps).sum();
        if ablations > 0 {
            assert!(overlapping > 0, "seed {seed}");
        }
    }
}

/// 🫥️ Holds that the poof is what keeps pets apart where planning fails: without heads and air control they poof instead of overlapping, and without the poof as well they overlap.
pub(super) fn poofs_needed(seeds: u32, ticks: Ticks) {
    let (mut kept, mut poofs, mut plain, mut bare) = (0, 0, 0, 0);
    for seed in 1..=seeds {
        let without = run_world(seed, ticks, Rules { heads: false, steering: false, ..RULES }, 1);
        kept += without.tally.overlaps;
        poofs += without.tally.poofs;
        plain += run_world(seed, ticks, RULES, 1).tally.poofs;
        bare += run_world(seed, ticks, Rules { heads: false, steering: false, poof: false, ..RULES }, 1).tally.overlaps;
    }
    assert_eq!(kept, 0);
    assert!(poofs > plain);
    assert!(bare > 0);
}

/// 🎖️ Holds that the quality rules are not needed for the invariant, only for fewer poofs and spills.
pub(super) fn quality_optional(seeds: u32, ticks: Ticks) {
    let (mut poofs, mut headless_poofs, mut unsteered_poofs, mut spilled, mut unseated_spilled) = (0, 0, 0, 0, 0);
    for seed in 1..=seeds {
        let plain = run_world(seed, ticks, RULES, 1);
        let headless = run_world(seed, ticks, Rules { heads: false, ..RULES }, 1);
        let unsteered = run_world(seed, ticks, Rules { steering: false, ..RULES }, 1);
        let unseated = run_world(seed, ticks, Rules { seating: false, ..RULES }, 1);
        for world in [&plain, &headless, &unsteered, &unseated] {
            assert_eq!([world.tally.overlaps, world.counts.disorders], [0, 0], "seed {seed}");
        }
        assert_eq!(headless.counts.heads, 0);
        assert_eq!(unsteered.counts.steered, 0);
        assert_eq!(unseated.counts.seated, 0);
        poofs += plain.tally.poofs;
        headless_poofs += headless.tally.poofs;
        unsteered_poofs += unsteered.tally.poofs;
        spilled += plain.counts.spilled;
        unseated_spilled += unseated.counts.spilled;
    }
    if seeds > 1 {
        assert!(headless_poofs > poofs);
        assert!(unsteered_poofs > poofs);
        assert!(unseated_spilled > spilled);
    }
}
//#endregion 🔖️Tools of the cases

//#region 🔖️Generated cases
/// 📏️ Overlaps agree with the area of the intersection on generated bodies.
pub(super) fn overlaps_agree_with_areas(cases: usize) {
    let mut stream = Stream(11);
    for _ in 0..cases {
        let bodies: Vec<Body> = scattered(&mut stream, 8.0).into_iter().enumerate().map(|(index, extent)| Body { owner: format!("b{index}"), extent }).collect();
        let mut expected = Vec::new();
        for (first, one) in bodies.iter().enumerate() {
            for other in &bodies[first + 1..] {
                if sharing(one.extent, other.extent) {
                    expected.push(Pair { first: one.owner.clone(), second: other.owner.clone() });
                }
            }
        }
        assert_eq!(overlaps(&bodies), expected);
    }
}

/// 🔭️ `slot_in` agrees with a search over every place of a 1/64 px lattice.
pub(super) fn slots_agree_with_the_lattice(cases: usize) {
    let mut stream = Stream(23);
    for round in 0..cases {
        let x = 100.0 + stream.next(400.0) / 4.0;
        let half = 10.0 + stream.next(80.0) / 4.0;
        let own = rect(x - half, 250.0, x + half, 300.0);
        let count = 1.0 + stream.next(5.0);
        let obstacles = scattered(&mut stream, count);
        let low = stream.next(400.0) / 4.0;
        let high = low + stream.next(800.0) / 4.0;
        let mut best: Option<f64> = None;
        for step in 0..=((high - low) * 64.0) as i64 {
            let place = low + step as f64 / 64.0;
            let there = shifted(own, place - x, 0.0);
            if obstacles.iter().any(|obstacle| sharing(rect(there.x0 - SEAM, there.y0, there.x1 + SEAM, there.y1), *obstacle)) {
                continue;
            }
            if best.is_none_or(|best| (place - x).abs() < (best - x).abs()) {
                best = Some(place);
            }
        }
        assert_eq!(slot_in(x, own, low, high, &obstacles), best, "round {round}");
    }
}

/// 🧵️ `claim_clear` agrees with a comparison of every tick of every time line.
pub(super) fn claims_agree_with_time_lines(cases: usize) {
    let mut stream = Stream(37);
    let horizon: Ticks = 60;
    let line = |own: Option<Extent>, claim: Option<&Claim>| -> Vec<Option<Extent>> {
        (0..=horizon)
            .map(|tick| match claim {
                Some(claim) if !claim.slices.is_empty() && tick >= claim.slices[0].from => slice_at(claim, tick),
                _ => own,
            })
            .collect()
    };
    let mut clear = 0;
    for round in 0..cases {
        let others = 1 + stream.next(4.0) as usize;
        let mut bodies = Vec::new();
        let mut claims = Vec::new();
        for index in 0..others {
            let owner = format!("o{index}");
            let x = stream.next(60.0) * 4.0;
            let y = stream.next(40.0) * 4.0;
            if stream.next(4.0) > 0.0 {
                bodies.push(body(&owner, x, y, x + 40.0, y + 50.0));
            }
            if stream.next(2.0) > 0.0 {
                let (dx, dy) = (stream.next(9.0) - 4.0, stream.next(9.0) - 4.0);
                let path = marching(x, y, dx, dy, 1 + stream.next(30.0) as usize);
                let from = stream.next(20.0) as Ticks;
                let span = 1 + stream.next(4.0) as Ticks;
                let rest = (stream.next(3.0) > 0.0).then(|| path[path.len() - 1]);
                claims.push(claim_of(&owner, from, &path, span, rest));
            }
        }
        let from = stream.next(20.0) as Ticks;
        let (x, y) = (stream.next(60.0) * 4.0, stream.next(40.0) * 4.0);
        let (dx, dy) = (stream.next(9.0) - 4.0, stream.next(9.0) - 4.0);
        let path = marching(x, y, dx, dy, 1 + stream.next(30.0) as usize);
        let span = 1 + stream.next(4.0) as Ticks;
        let rest = (stream.next(3.0) > 0.0).then(|| path[path.len() - 1]);
        let plan = claim_of("me", from, &path, span, rest);
        let mine = line(None, Some(&plan));
        let mut expected = true;
        for index in 0..others {
            let owner = format!("o{index}");
            let theirs = line(bodies.iter().find(|entry: &&Body| entry.owner == owner).map(|entry| entry.extent), claims.iter().find(|entry: &&Claim| entry.owner == owner));
            for tick in from..=horizon {
                if let (Some(one), Some(other)) = (mine[tick as usize], theirs[tick as usize]) {
                    if sharing(one, other) {
                        expected = false;
                    }
                }
            }
        }
        if expected {
            clear += 1;
        }
        assert_eq!(claim_clear(&plan, &bodies, &claims), expected, "round {round}");
    }
    assert!(clear * 20 > cases);
    assert!(clear * 20 < cases * 19);
}

/// 🦿️ `guarded_stride` agrees with a walk along a 1/64 px lattice that stops at the first place too close to an obstacle.
pub(super) fn strides_agree_with_the_lattice(cases: usize) {
    let mut stream = Stream(41);
    let mut hindered = 0;
    for round in 0..cases {
        let x = 150.0 + stream.next(200.0) / 4.0;
        let own = rect(x - 20.0, 250.0, x + 20.0, 300.0);
        let stride = (stream.next(161.0) - 80.0) / 4.0;
        let side = if stride < 0.0 { -1.0 } else { 1.0 };
        let near = x + 20.0 + stream.next(80.0) / 4.0;
        let far = x - 20.0 - stream.next(80.0) / 4.0;
        let flank = if side > 0.0 {
            let top = 240.0 + stream.next(40.0);
            rect(near, top, near + 10.0 + stream.next(80.0) / 4.0, 310.0)
        } else {
            let left = far - 10.0 - stream.next(80.0) / 4.0;
            rect(left, 240.0 + stream.next(40.0), far, 310.0)
        };
        let count = 1.0 + stream.next(5.0);
        let mut obstacles = scattered(&mut stream, count);
        obstacles.push(flank);
        obstacles.retain(|obstacle| !sharing(rect(own.x0 - SEAM, own.y0, own.x1 + SEAM, own.y1), *obstacle));
        let mut reach = 0.0;
        for step in 1..=(stride.abs() * 64.0) as i64 {
            let there = shifted(own, (side * step as f64) / 64.0, 0.0);
            if obstacles.iter().any(|obstacle| sharing(rect(there.x0 - SEAM, there.y0, there.x1 + SEAM, there.y1), *obstacle)) {
                break;
            }
            reach = step as f64 / 64.0;
        }
        if reach < stride.abs() {
            hindered += 1;
        }
        assert_eq!(guarded_stride(own, stride, &obstacles), if reach == 0.0 { 0.0 } else { side * reach }, "round {round}");
    }
    assert!(hindered * 5 > cases);
}

/// 🚶️ Walkers on one line stay apart and in their order, which nothing but the guard does.
pub(super) fn walkers_kept_in_order(ticks: usize) {
    for guarded in [true, false] {
        let mut stream = Stream(5);
        let mut walkers = Vec::new();
        let mut x = 0.0;
        for index in 0..7 {
            let width = 20.0 + stream.next(21.0);
            walkers.push(body(&format!("w{index}"), x, 0.0, x + width, 40.0));
            x = x + width + SEAM + stream.next(40.0) / 8.0;
        }
        let order = order_of(&walkers);
        let (mut collisions, mut swaps) = (0, 0);
        for tick in 0..ticks {
            for turn in 0..walkers.len() {
                let index = (tick + turn) % walkers.len();
                let wanted = (stream.next(49.0) - 24.0) / 8.0;
                let stride = if guarded { guarded_stride(walkers[index].extent, wanted, &obstacles_of(&walkers[index].owner, &walkers, &[], tick as Ticks)) } else { wanted };
                walkers[index].extent = shifted(walkers[index].extent, stride, 0.0);
            }
            if !overlaps(&walkers).is_empty() {
                collisions += 1;
            }
            if !order_kept(&order, &order_of(&walkers)) {
                swaps += 1;
            }
        }
        if guarded {
            assert_eq!([collisions, swaps], [0, 0]);
        } else {
            assert!(collisions * 10 > ticks);
            assert!(swaps > 0);
        }
    }
}

/// 🧮️ `seat_of` is the nearest valid seating: it agrees with a search over every way to pool neighbours.
pub(super) fn seatings_agree_with_every_pooling(cases: usize) {
    let perch = Perch { surface: "s".to_string(), x0: 100.0, x1: 400.0, y: 300.0 };
    let mut stream = Stream(53);
    let mut moved = 0;
    for round in 0..cases {
        let count = 1 + stream.next(5.0) as usize;
        let mut bodies = Vec::new();
        for index in 0..count {
            let x0 = 80.0 + stream.next(1000.0) / 4.0;
            let x1 = x0 + 30.0 + stream.next(30.0);
            bodies.push(body(&format!("s{index}"), x0, 248.0, x1, 304.0));
        }
        let seating = seat_of(&bodies, &perch, MARGIN);
        if !seating.leavers.is_empty() {
            continue;
        }
        let order: Vec<&Body> = order_of(&bodies).iter().map(|owner| bodies.iter().find(|entry| &entry.owner == owner).expect("a body")).collect();
        assert_eq!(seating.seats.iter().map(|seat| seat.owner.clone()).collect::<Vec<_>>(), order.iter().map(|entry| entry.owner.clone()).collect::<Vec<_>>());
        let mut offsets = vec![0.0];
        for at in 1..count {
            offsets.push(offsets[at - 1] + (order[at - 1].extent.x1 - order[at - 1].extent.x0) + SEAM);
        }
        let wanted: Vec<f64> = order.iter().zip(&offsets).map(|(entry, offset)| entry.extent.x0 - offset).collect();
        let low = perch.x0 - MARGIN;
        let high = perch.x1 + MARGIN - (order[count - 1].extent.x1 - order[count - 1].extent.x0) - offsets[count - 1];
        let mut best: Vec<f64> = Vec::new();
        let mut least = f64::INFINITY;
        for cuts in 0..1_usize << (count - 1) {
            let mut values = Vec::new();
            let mut start = 0;
            for at in 0..count {
                if at < count - 1 && cuts & (1 << at) == 0 {
                    continue;
                }
                let block = &wanted[start..=at];
                let mean = (block.iter().sum::<f64>() / block.len() as f64).max(low).min(high);
                values.extend(std::iter::repeat_n(mean, at + 1 - start));
                start = at + 1;
            }
            if values.windows(2).any(|pair| pair[1] < pair[0]) {
                continue;
            }
            let cost: f64 = values.iter().zip(&wanted).map(|(value, want)| (value - want) * (value - want)).sum();
            if cost < least - 1e-9 {
                least = cost;
                best = values;
            }
        }
        for (at, seat) in seating.seats.iter().enumerate() {
            assert!((seat.shift - (best[at] - wanted[at])).abs() < 1e-9, "round {round} seat {at}");
        }
        if seating.seats.iter().any(|seat| seat.shift != 0.0) {
            moved += 1;
        }
        let after: Vec<Body> = seating.seats.iter().map(|seat| Body { owner: seat.owner.clone(), extent: shifted(bodies.iter().find(|entry| entry.owner == seat.owner).expect("a body").extent, seat.shift, 0.0) }).collect();
        assert_eq!(overlaps(&after), Vec::new(), "round {round}");
        for neighbours in after.windows(2) {
            assert!(neighbours[1].extent.x0 - neighbours[0].extent.x1 > SEAM - 1e-9, "round {round}");
        }
        assert!(after[0].extent.x0 > low - 1e-9);
        assert!(after[count - 1].extent.x1 < perch.x1 + MARGIN + 1e-9);
        for seat in seat_of(&after, &perch, MARGIN).seats {
            assert!(seat.shift.abs() < 1e-9, "round {round}");
        }
    }
    assert!(moved * 4 > cases);
}

/// 🛺️ Generated groups scoot to their seats by one common share of the way, tick by tick apart.
pub(super) fn scoots_stay_apart(cases: usize) {
    let mut stream = Stream(67);
    for round in 0..cases {
        let mut bodies = Vec::new();
        let mut x = 100.0 + stream.next(200.0) / 4.0;
        let mut index = 0;
        while (index as f64) < 2.0 + stream.next(4.0) {
            let width = 30.0 + stream.next(30.0);
            bodies.push(body(&format!("s{index}"), x, 248.0, x + width, 304.0));
            x = x + width + SEAM + stream.next(160.0) / 4.0;
            index += 1;
        }
        let x0 = 100.0 + stream.next(400.0) / 4.0;
        let shrunk = Perch { surface: "s".to_string(), x0, x1: 500.0 + stream.next(800.0) / 4.0, y: 300.0 };
        let seating = seat_of(&bodies, &shrunk, MARGIN);
        let members: Vec<&Body> = seating.seats.iter().map(|seat| bodies.iter().find(|entry| entry.owner == seat.owner).expect("a body")).collect();
        let goals: Vec<f64> = seating.seats.iter().zip(&members).map(|(seat, member)| member.extent.x0 + seat.shift).collect();
        let mut places: Vec<f64> = members.iter().map(|member| member.extent.x0).collect();
        let mut fraction = 0.0;
        let mut tick = 0;
        while tick < 2000 && fraction < 1.0 {
            let shifts: Vec<f64> = places.iter().zip(&goals).map(|(place, goal)| goal - place).collect();
            fraction = scoot_fraction(&shifts, 1.125);
            places = places.iter().zip(&goals).map(|(&place, &goal)| scooted(place, goal, fraction)).collect();
            let there: Vec<Body> = members.iter().zip(&places).map(|(member, place)| Body { owner: member.owner.clone(), extent: shifted(member.extent, place - member.extent.x0, 0.0) }).collect();
            assert_eq!(overlaps(&there), Vec::new(), "round {round} tick {tick}");
            tick += 1;
        }
        assert_eq!(places, goals);
    }
}

/// 🫸️ `pushed_out` answers a free place or nothing on generated crowds, out of a single obstacle by its shortest way.
pub(super) fn pushes_free_or_nothing(cases: usize) {
    let mut stream = Stream(71);
    let (mut freed, mut stuck) = (0, 0);
    for round in 0..cases {
        let own = rect(100.0, 250.0, 140.0, 300.0);
        let count = 1.0 + stream.next(4.0);
        let obstacles = scattered(&mut stream, count);
        let Some(push) = pushed_out(own, &obstacles, PUSHES) else {
            stuck += 1;
            continue;
        };
        assert!(free_among(shifted(own, push.x, push.y), &obstacles), "round {round}");
        if free_among(own, &obstacles) {
            assert_eq!(push, Point { x: 0.0, y: 0.0 });
        } else {
            freed += 1;
        }
        if obstacles.len() == 1 && !free_among(own, &obstacles) {
            let only = obstacles[0];
            let least = [own.x1 - only.x0, only.x1 - own.x0, own.y1 - only.y0, only.y1 - own.y0].into_iter().fold(f64::INFINITY, f64::min);
            assert_eq!(push.x.abs() + push.y.abs(), least + SEAM, "round {round}");
        }
    }
    assert!(freed * 10 > cases);
    assert!(stuck * 2 < cases);
}
//#endregion 🔖️Generated cases
