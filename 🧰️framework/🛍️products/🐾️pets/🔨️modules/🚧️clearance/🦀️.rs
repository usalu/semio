//! 🚧️ Clearance — the Rust twin of `🟦️.ts`: the machinery that keeps the bodies of pets apart, so that at the end of every tick no two of them overlap.
//!
//! A **body** is the solid box of an actor ([`body_of`]): its size box, the hover beneath a floater, what its
//! posture adds (the lean of a held pet, an open canopy) and a margin on every side. The hard invariant is
//! `overlaps(bodies) = []` ([`overlaps`]): two bodies overlap when their open boxes intersect, so bodies that touch
//! are apart. Everything that *places* a body ([`slot_in`], [`guarded_stride`], [`seat_of`], [`pushed_out`],
//! [`lift_onto`]) leaves a [`SEAM`] of 1/64 px beside its neighbour; the predicates ([`meets`], [`free_at`],
//! [`claim_clear`]) are exact.
//!
//! A **claim** is the swept corridor of a planned motion ([`claim_of`]): the box of the body for every range of ticks
//! of the plan, and the place where it rests afterwards. Whoever flies, falls, glides, climbs or reels plans first,
//! has the plan vetted ([`claim_clear`]) and then follows it; everybody else respects the claim.
//!
//! Every function evaluates the expressions of the TypeScript twin in the same order: only `+ − × ÷`, `abs`, `floor`,
//! JavaScript's `Math.min`/`Math.max` (`smaller`/`larger` of the terrain twin, which answer NaN and the signed zeros
//! as JavaScript does) and comparisons; lists are walked in their given order and every sort is a stable insertion,
//! so both twins answer every input bit for bit alike. Where the TypeScript twin floors a number the type says it:
//! the span of a claim is whole ticks, the rounds of a push are a count, a side is a [`Facing`] and a tally counts
//! whole ticks.
//!
//! @see ./🟦️.ts — the TypeScript twin
//! @see ../🏞️terrain/🦀️.rs — perches, falls and hops whose paths are claimed here; JavaScript's extremes
//! @see ../../🧬️schema/🦀️.rs — `Claim`, `Extent`, `Slice`, `Perch`, `Point`, `Size`, `Facing`, `Ticks`
//! @see <https://en.wikipedia.org/wiki/Isotonic_regression> — pool-adjacent-violators, the seating of [`seat_of`]

use crate::schema::{Claim, Extent, Facing, Perch, Point, Size, Slice, Slug, Ticks, TICKS_PER_SECOND};
use crate::terrain::{larger, smaller};
use std::cmp::max;

//#region 🔖️Constants
/// 🫧️ The margin of a body on every side in pixels: two bodies that touch keep the comfortable gap of 8 px between their size boxes.
pub const MARGIN: f64 = 4.0;

/// 🧵️ The hair every placement leaves beside a neighbour, in pixels (1/64, exact in binary): far above the rounding of any sum of coordinates, far below anything visible.
pub const SEAM: f64 = 0.015625;

/// ♾️ The last tick there is (2³² − 1): the end of a body that stays, in the time line of [`claim_clear`].
pub const FOREVER: Ticks = 4_294_967_295;

/// 🧭️ The air control a fall may try, in pixels per second added to its horizontal speed, in the order tried: straight first, then the nearest ones.
pub const STEERINGS: [f64; 13] = [0.0, 30.0, -30.0, 60.0, -60.0, 90.0, -90.0, 130.0, -130.0, 180.0, -180.0, 240.0, -240.0];

/// 🤲️ How many rounds [`pushed_out`] gets to free a held body.
pub const PUSHES: usize = 4;

/// ⏳️ How many ticks a blocked walker waits before it gives its goal up for a new one.
pub const PATIENCE: Ticks = 40;

/// ⏱️ How many ticks a plan may be put off (the pet stays where it is, as long as that place is free) before [`must_poof`] takes the stay away.
pub const DELAY: Ticks = 32;

/// 🛝️ The speed at which a pet starts to slide off a head, in pixels per second.
pub const SLIDE_OFF_SPEED: f64 = 32.0;

/// 📈️ What a slide gains per tick, in pixels per second.
pub const SLIDE_OFF_GAIN: f64 = 2.0;

/// 🚀️ The fastest slide, in pixels per second.
pub const SLIDE_OFF_LIMIT: f64 = 128.0;

/// 🙇️ How far a body that hangs at full tilt reaches out on either side, as a share of its height.
pub const LEAN: f64 = 0.25;

/// 🛫️ How far above its own altitude the high lane of a floater lies, as a share of its height plus the margin.
pub const LANE_LIFT: f64 = 0.9;

/// 🏃️ How much faster than it walks a pet scoots to its seat.
pub const SCOOT_HASTE: f64 = 1.5;
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🐾️ The solid box of one actor, named by the slug of its species.
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub owner: Slug,
    pub extent: Extent,
}

/// 🧍️ What a posture adds to the size box of a species, in pixels: to the left, to the right and above.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Posture {
    pub left: f64,
    pub right: f64,
    pub above: f64,
}

/// 👥️ Two owners whose bodies are too close, in the order of the list they were found in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pair {
    pub first: Slug,
    pub second: Slug,
}

/// 🪑️ How far a seated body moves along its perch, in pixels (negative: to the left).
#[derive(Clone, Debug, PartialEq)]
pub struct Seat {
    pub owner: Slug,
    pub shift: f64,
}

/// 🛋️ A seating of a perch: the seats from left to right, and who has to leave because the perch cannot hold them, the lowest priority first.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Seating {
    pub seats: Vec<Seat>,
    pub leavers: Vec<Slug>,
}

/// ⛸️ One tick of sliding off a head: how far the rider moves, and the side it slides to from now on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Slide {
    pub stride: f64,
    pub side: Facing,
}

/// 🧮️ What a run of the invariant has counted: its ticks, its actor-ticks (bodies summed over the ticks), the ticks with an overlap (must stay 0), the ticks with a near miss, the poofs and the ticks somebody waited.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub ticks: u64,
    pub actors: u64,
    pub overlaps: u64,
    pub nears: u64,
    pub poofs: u64,
    pub waits: u64,
}
//#endregion 🔖️Types

//#region 🔖️Bodies
/// 🧘️ The posture that adds nothing: standing, walking, sitting.
pub const UPRIGHT: Posture = Posture { left: 0.0, right: 0.0, above: 0.0 };

/// 🤸️ The posture of a body that hangs or tumbles tilted: `LEAN × height × |sine|` on either side, `sine` being the sine of its tilt (1 for a held pet, which may swing all the way).
pub fn leaning(height: f64, sine: f64) -> Posture {
    let reach = LEAN * height * sine.abs();
    Posture { left: reach, right: reach, above: 0.0 }
}

/// 🪂️ The posture of a body under an open canopy: the canopy sits centred on top of the size box, so it adds its height above and half of what it is wider on either side.
pub fn canopied(size: Size, canopy: Size) -> Posture {
    let reach = larger((canopy.width - size.width) / 2.0, 0.0);
    Posture { left: reach, right: reach, above: larger(canopy.height, 0.0) }
}

/// 📐️ The body of an actor whose feet are at `feet`: the size box centred above the feet, the `hover` beneath them down to the perch, what the posture adds, and `margin` on every side.
pub fn body_of(owner: &str, feet: Point, size: Size, hover: f64, posture: Posture, margin: f64) -> Body {
    let half = size.width / 2.0;
    Body { owner: owner.to_string(), extent: Extent { x0: feet.x - half - posture.left - margin, y0: feet.y - size.height - posture.above - margin, x1: feet.x + half + posture.right + margin, y1: feet.y + hover + margin } }
}

/// ➡️ A box moved by `dx` and `dy`.
pub fn shifted(extent: Extent, dx: f64, dy: f64) -> Extent {
    Extent { x0: extent.x0 + dx, y0: extent.y0 + dy, x1: extent.x1 + dx, y1: extent.y1 + dy }
}

/// 🎈️ A box grown by `margin` on every side.
pub fn grown(extent: Extent, margin: f64) -> Extent {
    Extent { x0: extent.x0 - margin, y0: extent.y0 - margin, x1: extent.x1 + margin, y1: extent.y1 + margin }
}

/// 🔗️ The smallest box that holds both boxes.
pub fn united(one: Extent, other: Extent) -> Extent {
    Extent { x0: smaller(one.x0, other.x0), y0: smaller(one.y0, other.y0), x1: larger(one.x1, other.x1), y1: larger(one.y1, other.y1) }
}

/// 💥️ Whether two boxes overlap: their open interiors intersect, so boxes that only touch do not.
pub fn meets(one: Extent, other: Extent) -> bool {
    one.x0 < other.x1 && other.x0 < one.x1 && one.y0 < other.y1 && other.y0 < one.y1
}

/// 🚨️ The executable invariant: every pair of bodies that overlap, by ascending position of the first and then of the second in the list. It must be empty at the end of every tick.
pub fn overlaps(bodies: &[Body]) -> Vec<Pair> {
    let mut pairs = Vec::new();
    for (first, one) in bodies.iter().enumerate() {
        for other in &bodies[first + 1..] {
            if meets(one.extent, other.extent) {
                pairs.push(Pair { first: one.owner.clone(), second: other.owner.clone() });
            }
        }
    }
    pairs
}

/// 😬️ Every pair of bodies that do not overlap but are closer than `margin` to each other, in the order of [`overlaps`].
pub fn near_misses(bodies: &[Body], margin: f64) -> Vec<Pair> {
    let mut pairs = Vec::new();
    for (first, one) in bodies.iter().enumerate() {
        for other in &bodies[first + 1..] {
            if !meets(one.extent, other.extent) && meets(grown(one.extent, margin), other.extent) {
                pairs.push(Pair { first: one.owner.clone(), second: other.owner.clone() });
            }
        }
    }
    pairs
}
//#endregion 🔖️Bodies

//#region 🔖️Free places
/// 🧱️ Everything `owner` has to stay clear of from `tick` on: the bodies of all others in list order, then for every claim of another owner in list order its slices that are not over yet (`until ≥ tick`) and the place where it comes to rest.
pub fn obstacles_of(owner: &str, bodies: &[Body], claims: &[Claim], tick: Ticks) -> Vec<Extent> {
    let mut obstacles = Vec::new();
    for body in bodies {
        if body.owner != owner {
            obstacles.push(body.extent);
        }
    }
    for claim in claims {
        if claim.owner == owner {
            continue;
        }
        for slice in &claim.slices {
            if slice.until >= tick {
                obstacles.push(slice.extent);
            }
        }
        if let Some(rest) = claim.rest {
            obstacles.push(rest);
        }
    }
    obstacles
}

/// 🆓️ Whether a box overlaps none of the obstacles.
pub fn free_among(extent: Extent, obstacles: &[Extent]) -> bool {
    for obstacle in obstacles {
        if meets(extent, *obstacle) {
            return false;
        }
    }
    true
}

/// ✅️ Whether `owner` may be at `extent` from `tick` on: the box overlaps no other body, no corridor that is left of another owner's claim and no place where such a claim comes to rest.
pub fn free_at(extent: Extent, owner: &str, bodies: &[Body], claims: &[Claim], tick: Ticks) -> bool {
    free_among(extent, &obstacles_of(owner, bodies, claims, tick))
}

/// 🪜️ Whether two boxes share some of their height (open intervals), the first condition of every obstacle that stands in the way sideways.
fn level(one: Extent, other: Extent) -> bool {
    one.y0 < other.y1 && other.y0 < one.y1
}

/// ✂️ `stretches` (pairs of ends, ascending) without the open interval `(low, high)`; an end that is only touched stays, as a single point if need be.
fn carved(stretches: &[(f64, f64)], low: f64, high: f64) -> Vec<(f64, f64)> {
    let mut kept = Vec::new();
    for &(x0, x1) in stretches {
        if high <= x0 || low >= x1 {
            kept.push((x0, x1));
        } else {
            if low >= x0 {
                kept.push((x0, low));
            }
            if high <= x1 {
                kept.push((high, x1));
            }
        }
    }
    kept
}

/// 🎯️ The place nearest to `x` between `low` and `high` (ends included) at which a body can be: `extent` is its box when its feet are at `x`, and at the answered feet position the box, moved along, keeps a [`SEAM`] from every obstacle whose height it shares. Among two places equally near the left one; `None` when there is none.
pub fn slot_in(x: f64, extent: Extent, low: f64, high: f64, obstacles: &[Extent]) -> Option<f64> {
    let ordered = low <= high;
    if !ordered {
        return None;
    }
    let mut stretches = vec![(low, high)];
    for obstacle in obstacles {
        if !level(extent, *obstacle) {
            continue;
        }
        stretches = carved(&stretches, x + (obstacle.x0 - extent.x1 - SEAM), x + (obstacle.x1 - extent.x0 + SEAM));
    }
    let mut nearest = None;
    let mut least = f64::INFINITY;
    for &(x0, x1) in &stretches {
        let place = smaller(larger(x, x0), x1);
        let distance = (place - x).abs();
        if distance < least {
            nearest = Some(place);
            least = distance;
        }
    }
    nearest
}

/// 📍️ The free spot on a perch nearest to `x` for a body whose box is `extent` when its feet are at `x`: its footprint (`foot` px to either side of the feet) stays on the perch. `None` when the perch has no room.
pub fn spot_on(perch: &Perch, x: f64, extent: Extent, foot: f64, obstacles: &[Extent]) -> Option<f64> {
    slot_in(x, extent, perch.x0 + foot, perch.x1 - foot, obstacles)
}

/// 🏛️ The free column over a perch nearest to `x` for a landing: the place at which the body can come straight down by `drop` px (from `extent`, its box with the feet at `x`, to the perch) without touching anybody on the way, its footprint on the perch. `None` when there is none.
pub fn column_over(perch: &Perch, x: f64, extent: Extent, drop: f64, foot: f64, obstacles: &[Extent]) -> Option<f64> {
    slot_in(x, Extent { x0: extent.x0, y0: extent.y0, x1: extent.x1, y1: extent.y1 + larger(drop, 0.0) }, perch.x0 + foot, perch.x1 - foot, obstacles)
}
//#endregion 🔖️Free places

//#region 🔖️Claims
/// 🛤️ The claim of a planned motion: `extents[k]` is the box of the body at the end of tick `from + k`, a slice holds `span` ticks (at least 1) and is the smallest box around their boxes, and `rest` is where the owner stays afterwards. With `span = 1` the claim is the path itself; a wider span costs fewer boxes and claims a little more room. The TypeScript twin floors a fractional span; here the type says it.
pub fn claim_of(owner: &str, from: Ticks, extents: &[Extent], span: Ticks, rest: Option<Extent>) -> Claim {
    let width = max(span, 1) as usize;
    let mut slices = Vec::new();
    for (index, chunk) in extents.chunks(width).enumerate() {
        let start = index * width;
        let mut hull = chunk[0];
        for extent in &chunk[1..] {
            hull = united(hull, *extent);
        }
        slices.push(Slice { from: from + start as Ticks, until: from + (start + chunk.len()) as Ticks - 1, extent: hull });
    }
    Claim { owner: owner.to_string(), slices, rest }
}

/// 🔎️ Where a claim has its owner at `tick`: the slice that holds the tick, the rest after the last slice, and `None` before the first slice (the owner is still where its body is) or after a plan that ends off stage.
pub fn slice_at(claim: &Claim, tick: Ticks) -> Option<Extent> {
    let first = claim.slices.first()?;
    if tick < first.from {
        return None;
    }
    for slice in &claim.slices {
        if tick <= slice.until {
            return Some(slice.extent);
        }
    }
    claim.rest
}

/// 🕰️ The time line of one owner as slices without a gap: its body until its claim begins (for ever without a claim), the slices of the claim, then its rest for ever.
fn spans_of(body: Option<Extent>, claim: Option<&Claim>) -> Vec<Slice> {
    let mut spans = Vec::new();
    let Some(claim) = claim.filter(|claim| !claim.slices.is_empty()) else {
        if let Some(body) = body {
            spans.push(Slice { from: 0, until: FOREVER, extent: body });
        }
        return spans;
    };
    let first = claim.slices[0].from;
    let last = claim.slices[claim.slices.len() - 1].until;
    if let Some(body) = body.filter(|_| first > 0) {
        spans.push(Slice { from: 0, until: first - 1, extent: body });
    }
    spans.extend_from_slice(&claim.slices);
    if let Some(rest) = claim.rest.filter(|_| last < FOREVER) {
        spans.push(Slice { from: last + 1, until: FOREVER, extent: rest });
    }
    spans
}

/// ⚔️ Whether two time lines hold boxes that overlap at one and the same tick.
fn collide(mine: &[Slice], theirs: &[Slice]) -> bool {
    for one in mine {
        for other in theirs {
            if one.from <= other.until && other.from <= one.until && meets(one.extent, other.extent) {
                return true;
            }
        }
    }
    false
}

/// 🚦️ Whether a plan may be taken: from its first tick on, the box of its slice — and after its last slice its rest, for ever — overlaps nobody at the same tick. Another owner is where its body is until its own claim begins (for ever when it has none), then inside the slices of that claim, then at the rest of that claim; so a plan may cross a corridor before or after its owner passes, but may never come to rest where somebody arrives later. One claim per owner is expected (the first one in the list is the time line of its body); the owner's own body and claims are ignored.
pub fn claim_clear(claim: &Claim, bodies: &[Body], claims: &[Claim]) -> bool {
    let mine = spans_of(None, Some(claim));
    for body in bodies {
        if body.owner == claim.owner {
            continue;
        }
        let theirs = claims.iter().find(|other| other.owner == body.owner);
        if collide(&mine, &spans_of(Some(body.extent), theirs)) {
            return false;
        }
    }
    for (index, other) in claims.iter().enumerate() {
        if other.owner == claim.owner {
            continue;
        }
        let leading = !claims[..index].iter().any(|before| before.owner == other.owner);
        let embodied = bodies.iter().any(|body| body.owner == other.owner);
        if leading && embodied {
            continue;
        }
        if collide(&mine, &spans_of(None, Some(other))) {
            return false;
        }
    }
    true
}

/// 🔓️ The claims without those of `owner`: its plan has ended, was given up, or its owner vanished.
pub fn released(claims: &[Claim], owner: &str) -> Vec<Claim> {
    claims.iter().filter(|claim| claim.owner != owner).cloned().collect()
}

/// 🧹️ The claims as they stand at `tick`: slices that are over (`until < tick`) are dropped, and with them every claim that has no slice left — its owner has arrived and its body now holds the place.
pub fn pruned(claims: &[Claim], tick: Ticks) -> Vec<Claim> {
    let mut kept = Vec::new();
    for claim in claims {
        let slices: Vec<Slice> = claim.slices.iter().filter(|slice| slice.until >= tick).copied().collect();
        if !slices.is_empty() {
            kept.push(Claim { owner: claim.owner.clone(), slices, rest: claim.rest });
        }
    }
    kept
}
//#endregion 🔖️Claims

//#region 🔖️Order on a perch
/// 👣️ The farthest a body may move sideways this tick: `stride` (negative: to the left) cut short so that the box stops a [`SEAM`] before the first obstacle that shares its height and lies ahead (its middle beyond the middle of the box). An obstacle the box already overlaps therefore holds it back only on the way further in, never on the way out. Because nobody passes anybody whose height it shares, the order along a perch never changes by walking.
pub fn guarded_stride(extent: Extent, stride: f64, obstacles: &[Extent]) -> f64 {
    let middle = extent.x0 + extent.x1;
    if stride > 0.0 {
        let mut reach = stride;
        for obstacle in obstacles {
            if !level(extent, *obstacle) {
                continue;
            }
            let ahead = obstacle.x0 + obstacle.x1 > middle;
            if !ahead {
                continue;
            }
            reach = smaller(reach, larger(obstacle.x0 - extent.x1 - SEAM, 0.0));
        }
        return reach;
    }
    if stride < 0.0 {
        let mut reach = 0.0 - stride;
        for obstacle in obstacles {
            if !level(extent, *obstacle) {
                continue;
            }
            let ahead = obstacle.x0 + obstacle.x1 < middle;
            if !ahead {
                continue;
            }
            reach = smaller(reach, larger(extent.x0 - obstacle.x1 - SEAM, 0.0));
        }
        return 0.0 - reach;
    }
    0.0
}

/// 🦘️ The first condition of a swap by a hop: a hop that rises `rise` px from where `hopper` stands lifts its box over `hurdle`, a seam to spare. The swap itself is a plan like any other: the hop is taken only when [`claim_clear`] holds for its claim, which also keeps the landing beyond the neighbour free; otherwise the lower priority turns round or waits ([`PATIENCE`]).
pub fn vaults(hopper: Extent, rise: f64, hurdle: Extent) -> bool {
    hopper.y1 - rise <= hurdle.y0 - SEAM
}

/// 📶️ The positions of the bodies from left to right by the middle of their boxes, a stable insertion: equals keep their order in the list.
fn ranked(bodies: &[Body]) -> Vec<usize> {
    let mut ranks: Vec<usize> = Vec::new();
    for (index, body) in bodies.iter().enumerate() {
        let middle = body.extent.x0 + body.extent.x1;
        let mut at = ranks.len();
        while at > 0 && bodies[ranks[at - 1]].extent.x0 + bodies[ranks[at - 1]].extent.x1 > middle {
            at -= 1;
        }
        ranks.insert(at, index);
    }
    ranks
}

/// 🔢️ The owners of the bodies from left to right by the middle of their boxes; equals keep their order in the list.
pub fn order_of(bodies: &[Body]) -> Vec<Slug> {
    ranked(bodies).into_iter().map(|index| bodies[index].owner.clone()).collect()
}

/// 🧷️ Whether an order was kept: the owners that are in both lists follow each other in `after` as they did in `before`.
pub fn order_kept(before: &[Slug], after: &[Slug]) -> bool {
    let mut last: Option<usize> = None;
    for owner in before {
        let Some(at) = after.iter().position(|other| other == owner) else {
            continue;
        };
        if last.is_some_and(|last| at < last) {
            return false;
        }
        last = Some(at);
    }
    true
}
//#endregion 🔖️Order on a perch

//#region 🔖️Seating
/// 💺️ The seating of a perch after a survey: every body moves as little as possible (least squares) while the order from left to right is kept, neighbours keep a [`SEAM`] between their boxes and every box without its `margin` lies on the perch.
///
/// `bodies` are the bodies that stand on the perch, the highest priority first. While they do not fit side by side
/// the last one in the list leaves. For the others, sorted by the middle of their boxes, let `o₁ = 0` and
/// `oᵢ₊₁ = oᵢ + widthᵢ + SEAM`; with `qᵢ = x0ᵢ − oᵢ` "apart and in order" reads "`q` never decreases" and "on the
/// perch" reads `perch.x0 − margin ≤ q ≤ perch.x1 + margin − widthₙ − oₙ`. The nearest such `q` is the isotonic
/// regression of `q` (pool adjacent violators: a block is the mean of its members, two neighbouring blocks merge
/// while the left mean is larger than the right one), clamped into the bounds. A seating that is valid already is
/// answered with shifts of exactly 0.
pub fn seat_of(bodies: &[Body], perch: &Perch, margin: f64) -> Seating {
    let mut members = ranked(bodies);
    let mut leavers = Vec::new();
    let low = perch.x0 - margin;
    let mut offsets: Vec<f64> = Vec::new();
    let mut high = low;
    while !members.is_empty() {
        offsets = vec![0.0];
        for &member in &members[..members.len() - 1] {
            let before = bodies[member].extent;
            offsets.push(offsets[offsets.len() - 1] + (before.x1 - before.x0) + SEAM);
        }
        let last = bodies[members[members.len() - 1]].extent;
        high = perch.x1 + margin - (last.x1 - last.x0) - offsets[members.len() - 1];
        if low <= high {
            break;
        }
        let mut worst = 0;
        for (at, &member) in members.iter().enumerate().skip(1) {
            if member > members[worst] {
                worst = at;
            }
        }
        leavers.push(bodies[members[worst]].owner.clone());
        members.remove(worst);
    }
    let mut sums: Vec<f64> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();
    for (at, &member) in members.iter().enumerate() {
        sums.push(bodies[member].extent.x0 - offsets[at]);
        counts.push(1);
        while sums.len() > 1 && sums[sums.len() - 2] / counts[counts.len() - 2] as f64 > sums[sums.len() - 1] / counts[counts.len() - 1] as f64 {
            let (Some(sum), Some(count)) = (sums.pop(), counts.pop()) else {
                break;
            };
            let merged = sums.len() - 1;
            sums[merged] += sum;
            counts[merged] += count;
        }
    }
    let mut means = Vec::with_capacity(members.len());
    for (&sum, &count) in sums.iter().zip(&counts) {
        means.extend(std::iter::repeat_n(smaller(larger(sum / count as f64, low), high), count));
    }
    let seats = members.iter().zip(&means).enumerate().map(|(at, (&member, &mean))| Seat { owner: bodies[member].owner.clone(), shift: mean - (bodies[member].extent.x0 - offsets[at]) }).collect();
    Seating { seats, leavers }
}

/// 🎚️ The share of its way every member of a group covers this tick when the one with the longest way moves `stride` px: `stride ÷ max |shift|`, at most 1 (and 1 when nobody has a way left). Because being apart, in order and on the perch are linear conditions, every mixture of two valid seatings is valid: a group that moves by one common share never overlaps within itself.
pub fn scoot_fraction(shifts: &[f64], stride: f64) -> f64 {
    let mut farthest = 0.0;
    for shift in shifts {
        farthest = larger(farthest, shift.abs());
    }
    let moving = farthest > 0.0;
    if !moving {
        return 1.0;
    }
    if farthest > stride {
        larger(stride, 0.0) / farthest
    } else {
        1.0
    }
}

/// 🚶️ Where a scoot from `from` towards `to` stands after covering `fraction` of the way: `to` itself once the fraction reaches 1.
pub fn scooted(from: f64, to: f64, fraction: f64) -> f64 {
    if fraction >= 1.0 {
        to
    } else {
        from + (to - from) * fraction
    }
}
//#endregion 🔖️Seating

//#region 🔖️Heads
/// 🎩️ The body on whose top a descending body lands, like a perch crossing: between `before` and `after` (the box of the faller at the end of the last tick and of this one) its lower edge comes down onto the upper edge of a body whose width it shares at `after`. The highest such body, the first one among equals; `None` when it lands on nobody — never while rising, and never on a body it was beside already.
pub fn head_under<'a>(before: Extent, after: Extent, owner: &str, bodies: &'a [Body]) -> Option<&'a Body> {
    let mut host: Option<&Body> = None;
    for body in bodies {
        if body.owner == owner {
            continue;
        }
        let top = body.extent.y0;
        let crossing = before.y1 <= top && after.y1 > top - SEAM && after.y1 >= before.y1;
        if !crossing {
            continue;
        }
        let across = after.x0 < body.extent.x1 && body.extent.x0 < after.x1;
        if !across {
            continue;
        }
        if host.is_none_or(|host| top < host.extent.y0) {
            host = Some(body);
        }
    }
    host
}

/// 🛗️ How far a box has to move down (negative: up) to stand on a head: its lower edge a [`SEAM`] above the upper edge of `host`.
pub fn lift_onto(extent: Extent, host: Extent) -> f64 {
    host.y0 - SEAM - extent.y1
}

/// 🪨️ Whether `rider` still stands on `host`: it shares its width and its lower edge lies within two seams above the upper edge of the host. Checked every tick; a rider whose host walked off, fell or vanished falls again.
pub fn rests_on(rider: Extent, host: Extent) -> bool {
    rider.x0 < host.x1 && host.x0 < rider.x1 && rider.y1 <= host.y0 && host.y0 - rider.y1 <= 2.0 * SEAM
}

/// ↔️ The side a rider slides off to: right when the middle of its box is at or beyond the middle of the host, else left.
pub fn slide_side(rider: Extent, host: Extent) -> Facing {
    if rider.x0 + rider.x1 >= host.x0 + host.x1 {
        Facing::Right
    } else {
        Facing::Left
    }
}

/// 🎿️ How far a slide moves in its tick number `ticks` (0 for the first), in pixels: it starts at [`SLIDE_OFF_SPEED`], gains [`SLIDE_OFF_GAIN`] per tick and never exceeds [`SLIDE_OFF_LIMIT`].
pub fn slide_stride(ticks: Ticks) -> f64 {
    smaller(SLIDE_OFF_SPEED + SLIDE_OFF_GAIN * ticks as f64, SLIDE_OFF_LIMIT) / TICKS_PER_SECOND as f64
}

/// 🛷️ One tick of sliding off a head towards `side`: the stride of [`slide_stride`], kept between the edges `low` and `high` of the stage and guarded against the obstacles. A rider that cannot move at all turns round and tries the other side from the next tick on. Once it no longer [`rests_on`] its host it falls.
pub fn slide_of(rider: Extent, side: Facing, ticks: Ticks, low: f64, high: f64, obstacles: &[Extent]) -> Slide {
    let wanted = slide_stride(ticks);
    let bounded = match side {
        Facing::Right => smaller(wanted, larger(high - rider.x1, 0.0)),
        Facing::Left => 0.0 - smaller(wanted, larger(rider.x0 - low, 0.0)),
    };
    let stride = guarded_stride(rider, bounded, obstacles);
    if stride == 0.0 {
        Slide { stride: 0.0, side: side.reversed() }
    } else {
        Slide { stride, side }
    }
}
//#endregion 🔖️Heads

//#region 🔖️Held
/// 🫸️ How a held body gets out of the obstacles it was dragged into: the sum of the shortest ways out (left, right, up or down, the first one among equals, a [`SEAM`] to spare), taken obstacle by obstacle in list order for at most `iterations` rounds. `None` when it still overlaps something afterwards: then the body keeps the place it had. Without an overlap the answer is no move at all.
pub fn pushed_out(extent: Extent, obstacles: &[Extent], iterations: usize) -> Option<Point> {
    let mut dx = 0.0;
    let mut dy = 0.0;
    for _ in 0..iterations {
        let mut clean = true;
        for obstacle in obstacles {
            let x0 = extent.x0 + dx;
            let y0 = extent.y0 + dy;
            let x1 = extent.x1 + dx;
            let y1 = extent.y1 + dy;
            let inside = x0 < obstacle.x1 && obstacle.x0 < x1 && y0 < obstacle.y1 && obstacle.y0 < y1;
            if !inside {
                continue;
            }
            clean = false;
            let left = x1 - obstacle.x0 + SEAM;
            let right = obstacle.x1 - x0 + SEAM;
            let up = y1 - obstacle.y0 + SEAM;
            let down = obstacle.y1 - y0 + SEAM;
            let least = smaller(smaller(smaller(left, right), up), down);
            if least == left {
                dx -= left;
            } else if least == right {
                dx += right;
            } else if least == up {
                dy -= up;
            } else {
                dy += down;
            }
        }
        if clean {
            return Some(Point { x: dx, y: dy });
        }
    }
    free_among(shifted(extent, dx, dy), obstacles).then_some(Point { x: dx, y: dy })
}
//#endregion 🔖️Held

//#region 🔖️Last resort
/// 💨️ Whether a pet has no legal continuation and must poof (vanish at once, arrive anew at a free place): it may not stay — `stay` is the box it would keep, `None` when staying is no option (its patience of [`DELAY`] ticks is used up, or nothing holds it there) — and none of its `plans` is clear.
pub fn must_poof(owner: &str, stay: Option<Extent>, plans: &[Claim], bodies: &[Body], claims: &[Claim], tick: Ticks) -> bool {
    if stay.is_some_and(|stay| free_at(stay, owner, bodies, claims, tick)) {
        return false;
    }
    for plan in plans {
        if claim_clear(plan, bodies, claims) {
            return false;
        }
    }
    true
}

/// 🚪️ Who has to poof after a change from outside (a surface that moved its riders), so that the others are apart again: `bodies` in the order of their right to stay — whoever did not move first. A body leaves when it overlaps a body before it that stays, or, having no plan of its own, a corridor that is left or a rest of a claim whose owner stays. The claims of those who leave count no longer.
pub fn evicted(bodies: &[Body], claims: &[Claim], tick: Ticks) -> Vec<Slug> {
    let mut gone: Vec<Slug> = Vec::new();
    for (index, body) in bodies.iter().enumerate() {
        let mut blocked = bodies[..index].iter().any(|other| !gone.contains(&other.owner) && meets(body.extent, other.extent));
        let planned = claims.iter().any(|claim| claim.owner == body.owner);
        if !blocked && !planned {
            blocked = claims.iter().any(|claim| !gone.contains(&claim.owner) && (claim.slices.iter().any(|slice| slice.until >= tick && meets(body.extent, slice.extent)) || claim.rest.is_some_and(|rest| meets(body.extent, rest))));
        }
        if blocked {
            gone.push(body.owner.clone());
        }
    }
    gone
}
//#endregion 🔖️Last resort

//#region 🔖️Lanes
/// 🛩️ How far above its own altitude a floater of this height glides in its high lane, which it tries when the claim of its glide at its own altitude is not clear: [`LANE_LIFT`] × (height + margin).
pub fn lane_lift(height: f64, margin: f64) -> f64 {
    LANE_LIFT * (height + margin)
}
//#endregion 🔖️Lanes

//#region 🔖️Metrics
/// 🥚️ The tally of a run that has not begun.
pub const TALLY: Tally = Tally { ticks: 0, actors: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0 };

/// 📊️ The tally after one more tick: the bodies as they are at its end, counted once for [`overlaps`] and once for [`near_misses`] within `margin`, plus the poofs and the waiting actors of the tick.
pub fn tallied(tally: Tally, bodies: &[Body], margin: f64, poofs: u64, waits: u64) -> Tally {
    Tally {
        ticks: tally.ticks + 1,
        actors: tally.actors + bodies.len() as u64,
        overlaps: tally.overlaps + u64::from(!overlaps(bodies).is_empty()),
        nears: tally.nears + u64::from(!near_misses(bodies, margin).is_empty()),
        poofs: tally.poofs + poofs,
        waits: tally.waits + waits,
    }
}

/// 💯️ A count per million actor-ticks; 0 for a run without any.
pub fn per_million(count: u64, actors: u64) -> f64 {
    if actors > 0 {
        (count as f64 * 1_000_000.0) / actors as f64
    } else {
        0.0
    }
}
//#endregion 🔖️Metrics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
