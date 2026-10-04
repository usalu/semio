//! 🚧️ Unit suite of the clearance twin: every function against the cases of the TypeScript suite, against brute force on a lattice (the same generated cases, drawn from the same 32-bit generator), and the reference world of the TypeScript suite, in which the rules of the module, and nothing else, keep six to ten pets apart while they walk, hop, glide, fall, land on heads, are dragged and thrown by a hand and lose, shrink and move their perches.
//!
//! The world is the TypeScript suite's, expression for expression, so a run of it is a pure function of its seed and
//! rules in both languages (`d2_check_world.rs` of ticket 2026/10/02/QUIZ-PETS compares their bodies tick by tick).
//! Levels: the tests outside a submodule are fundamental, `quick` and `exhaustive` run more cases and the research
//! prototype's 24 worlds of 30 000 ticks.
//!
//! @see ../../🦀️.rs — the twin under test
//! @see ../../🟦️.ts, ./🟦️.ts — the TypeScript twin and its suite, the reference world
//! @see ../../../🏞️terrain/🦀️.rs — the perches, falls and hops the world moves by
//! @see ../../../🎲️randomness/🦀️.rs — the draws of the world

use super::*;
use crate::randomness::{random_words, unit_of};
use crate::schema::Ending;
use crate::terrain::{fall_step, hop_landing, hop_of, hop_step, landing_of, stride_to, Flight};

//#region 🔖️Reference world
/// 📜️ The rules of the world, each one a switch: the core rules `guard`, `corridors`, `vetting`, `rests`, `projection`, `poof` and `eviction`, without any of which bodies overlap, and the quality rules `heads`, `steering` and `seating`, without which pets poof or fall more often.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Rules {
    pub(crate) guard: bool,
    pub(crate) corridors: bool,
    pub(crate) vetting: bool,
    pub(crate) rests: bool,
    pub(crate) projection: bool,
    pub(crate) poof: bool,
    pub(crate) eviction: bool,
    pub(crate) heads: bool,
    pub(crate) steering: bool,
    pub(crate) seating: bool,
}

/// 🟢️ Every rule in force.
pub(crate) const RULES: Rules = Rules { guard: true, corridors: true, vetting: true, rests: true, projection: true, poof: true, eviction: true, heads: true, steering: true, seating: true };

/// 🦶️ One tick of a planned path: the feet, the velocity in pixels per second and the posture at the end of that tick.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Step {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    posture: Posture,
}

/// 🗺️ A plan: its steps (the first one is the place it starts from), its claim, and what it ends on: the surface of a perch or the owner of a head.
#[derive(Clone, Debug, PartialEq)]
struct Plan {
    steps: Vec<Step>,
    claim: Claim,
    ending: Ending,
    landing: String,
    steer: f64,
}

/// 🎭️ What a pet of the world is doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Stand,
    Walk,
    Plan,
    Drop,
    Held,
    Ride,
    Scoot,
    Gone,
}

/// 🐶️ A pet of the world.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Pet {
    pub(crate) slug: String,
    pub(crate) size: Size,
    hover: f64,
    speed: f64,
    chute: bool,
    pub(crate) x: f64,
    pub(crate) y: f64,
    vx: f64,
    vy: f64,
    posture: Posture,
    pub(crate) mode: Mode,
    perch: Option<String>,
    goal: f64,
    until: Ticks,
    waited: Ticks,
    steps: Vec<Step>,
    from: Ticks,
    ending: Ending,
    landing: String,
    side: Facing,
    slid: Ticks,
}

/// ✋️ The simulated hand: whom it holds (by position in the pets), where it is and was a tick ago, where it is heading, how fast and for how long.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Hand {
    pet: usize,
    x: f64,
    y: f64,
    px: f64,
    py: f64,
    tx: f64,
    ty: f64,
    pace: f64,
    left: i64,
}

/// 🔢️ What happened in a world, counted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Counts {
    pub(crate) surveys: u64,
    pub(crate) grabs: u64,
    pub(crate) hops: u64,
    pub(crate) glides: u64,
    pub(crate) lanes: u64,
    pub(crate) falls: u64,
    pub(crate) heads: u64,
    pub(crate) steered: u64,
    pub(crate) chutes: u64,
    pub(crate) refusals: u64,
    pub(crate) delays: u64,
    pub(crate) stalls: u64,
    pub(crate) timeouts: u64,
    pub(crate) evictions: u64,
    pub(crate) crowded: u64,
    pub(crate) seated: u64,
    pub(crate) spilled: u64,
    pub(crate) disorders: u64,
    pub(crate) brushes: u64,
}

impl Counts {
    /// 📋️ Every count in the order of the TypeScript world's `counts` object.
    pub(crate) fn listed(&self) -> [u64; 19] {
        [
            self.surveys,
            self.grabs,
            self.hops,
            self.glides,
            self.lanes,
            self.falls,
            self.heads,
            self.steered,
            self.chutes,
            self.refusals,
            self.delays,
            self.stalls,
            self.timeouts,
            self.evictions,
            self.crowded,
            self.seated,
            self.spilled,
            self.disorders,
            self.brushes,
        ]
    }
}

/// 🌍️ The reference world; the order of the pets on every perch at the end of the last tick is a list of pairs (surface, owners).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct World {
    seed: u32,
    rules: Rules,
    span: Ticks,
    pub(crate) tick: Ticks,
    draws: u32,
    perches: Vec<Perch>,
    spare: Vec<Perch>,
    pub(crate) pets: Vec<Pet>,
    claims: Vec<Claim>,
    hand: Option<Hand>,
    pub(crate) tally: Tally,
    poofs: u64,
    waits: u64,
    orders: Vec<(String, Vec<Slug>)>,
    pub(crate) counts: Counts,
}

const WIDTH: f64 = 960.0;
const FLOOR: f64 = 520.0;
const NEAR: f64 = 2.0;
const STREAM: u32 = 0x0c1e_a4a4;
const FALL_LIMIT: Ticks = 900;
const CHUTE_SPEED: f64 = 480.0;
const CHUTE_DECAY: f64 = 0.916_855_355_73;

/// 🎲️ The next draw of a world in [0, 1): counter-based, so a world is a pure function of its seed and rules.
fn draw(world: &mut World) -> f64 {
    let unit = unit_of(random_words(&[world.seed, STREAM, world.draws], 1)[0]);
    world.draws += 1;
    unit
}

/// 🗜️ A value held between two bounds.
fn held(value: f64, low: f64, high: f64) -> f64 {
    smaller(larger(value, low), high)
}

/// 🧊️ The box of a pet at a place in a posture.
fn extent_at(pet: &Pet, x: f64, y: f64, posture: Posture) -> Extent {
    body_of(&pet.slug, Point { x, y }, pet.size, pet.hover, posture, MARGIN).extent
}

/// 🧸️ The box of a pet as it is.
fn extent_of(pet: &Pet) -> Extent {
    extent_at(pet, pet.x, pet.y, pet.posture)
}

/// 👪️ The bodies of a world: every pet that has not vanished, in the order of the pets.
pub(crate) fn bodies_of(world: &World) -> Vec<Body> {
    world.pets.iter().filter(|pet| pet.mode != Mode::Gone).map(|pet| Body { owner: pet.slug.clone(), extent: extent_of(pet) }).collect()
}

/// 🪵️ The perch of a surface.
fn perch_of(world: &World, surface: Option<&str>) -> Option<Perch> {
    world.perches.iter().find(|perch| Some(perch.surface.as_str()) == surface).cloned()
}

/// 🩳️ The perches as far as the feet of a body may go on them whose footprint reaches `half` to either side.
fn narrowed(perches: &[Perch], half: f64) -> Vec<Perch> {
    perches.iter().filter(|perch| perch.x0 + half <= perch.x1 - half).map(|perch| Perch { surface: perch.surface.clone(), x0: perch.x0 + half, x1: perch.x1 - half, y: perch.y }).collect()
}

/// 🦵️ Whether a pet stands on a perch (and not in the air, in the hand or on a head).
fn grounded(pet: &Pet) -> bool {
    matches!(pet.mode, Mode::Stand | Mode::Walk | Mode::Scoot)
}

/// 🔍️ The position of the visible pet of a slug.
fn visible(world: &World, slug: &str) -> Option<usize> {
    world.pets.iter().position(|pet| pet.slug == slug && pet.mode != Mode::Gone)
}

/// 😴️ A pet comes to rest where it is and dwells for a while.
fn settle(world: &mut World, index: usize, now: Ticks) {
    let dwell = (draw(world) * 100.0).floor() as Ticks;
    let pet = &mut world.pets[index];
    pet.mode = Mode::Stand;
    pet.goal = pet.x;
    pet.waited = 0;
    pet.until = now + 20 + dwell;
}

/// 🕳️ A pet loses its footing: it needs a plan for the way down, starting with the given velocity.
fn unfoot(pet: &mut Pet, vx: f64, vy: f64) {
    pet.mode = Mode::Drop;
    pet.perch = None;
    pet.posture = UPRIGHT;
    pet.vx = vx;
    pet.vy = vy;
    pet.waited = 0;
}

/// 💨️ A pet vanishes at once; it arrives anew a second later.
fn poof(world: &mut World, index: usize, now: Ticks) {
    world.claims = released(&world.claims, &world.pets[index].slug);
    let pet = &mut world.pets[index];
    pet.mode = Mode::Gone;
    pet.perch = None;
    pet.posture = UPRIGHT;
    pet.until = now + 64;
    world.poofs += 1;
}

/// 🚪️ A vanished pet arrives at a free spot of a perch it draws; `false` when that perch has none.
fn arrive(world: &mut World, index: usize, now: Ticks) -> bool {
    let chosen = (draw(world) * world.perches.len() as f64).floor() as usize;
    let perch = world.perches[chosen].clone();
    let half = world.pets[index].size.width / 2.0;
    let x = perch.x0 + half + draw(world) * larger(perch.x1 - perch.x0 - world.pets[index].size.width, 0.0);
    let pet = &world.pets[index];
    let feet = perch.y - pet.hover;
    let Some(spot) = spot_on(&perch, x, extent_at(pet, x, feet, UPRIGHT), half, &obstacles_of(&pet.slug, &bodies_of(world), &world.claims, now)) else {
        return false;
    };
    let pet = &mut world.pets[index];
    pet.x = spot;
    pet.y = feet;
    pet.perch = Some(perch.surface);
    pet.posture = UPRIGHT;
    settle(world, index, now);
    true
}

/// 🌱️ A world for a seed: a floor, six shelves, six to ten pets of widths 28…58 and heights 40…56, a fifth of them floaters, half of them with a parachute. `span` is how many ticks a slice of its claims holds (1: the path itself).
pub(crate) fn open_world(seed: u32, rules: Rules, span: Ticks) -> World {
    let mut world = World {
        seed,
        rules,
        span,
        tick: 0,
        draws: 0,
        perches: vec![Perch { surface: "floor".to_string(), x0: 0.0, x1: WIDTH, y: FLOOR }],
        spare: Vec::new(),
        pets: Vec::new(),
        claims: Vec::new(),
        hand: None,
        tally: TALLY,
        poofs: 0,
        waits: 0,
        orders: Vec::new(),
        counts: Counts::default(),
    };
    for shelf in 0..6 {
        let x0 = (draw(&mut world) * 660.0).floor();
        let width = 140.0 + (draw(&mut world) * 240.0).floor();
        let y = FLOOR - 64.0 * (1.0 + (draw(&mut world) * 5.0).floor());
        world.perches.push(Perch { surface: format!("shelf-{shelf}"), x0, x1: smaller(x0 + width, WIDTH), y });
    }
    let count = 6 + (draw(&mut world) * 5.0).floor() as usize;
    for index in 0..count {
        let width = 28.0 + (draw(&mut world) * 31.0).floor();
        let height = 40.0 + (draw(&mut world) * 17.0).floor();
        let speed = 24.0 + (draw(&mut world) * 49.0).floor();
        let hover = if draw(&mut world) < 0.2 { 6.0 + (draw(&mut world) * 13.0).floor() } else { 0.0 };
        let chute = draw(&mut world) < 0.5;
        world.pets.push(Pet {
            slug: format!("pet-{index}"),
            size: Size { width, height },
            hover,
            speed,
            chute,
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            posture: UPRIGHT,
            mode: Mode::Gone,
            perch: None,
            goal: 0.0,
            until: 0,
            waited: 0,
            steps: Vec::new(),
            from: 0,
            ending: Ending::Perch,
            landing: String::new(),
            side: Facing::Right,
            slid: 0,
        });
        if !arrive(&mut world, index, 0) {
            world.pets[index].until = 64;
        }
    }
    world
}

/// 🧾️ The claim of a path, tick by tick; without the rule of rests it names no place where its owner stays.
fn claimed(world: &World, pet: &Pet, steps: &[Step], now: Ticks) -> Claim {
    let extents: Vec<Extent> = steps.iter().map(|step| extent_at(pet, step.x, step.y, step.posture)).collect();
    claim_of(&pet.slug, now, &extents, world.span, if world.rules.rests { extents.last().copied() } else { None })
}

/// 🍂️ The way down from where a pet is with a velocity (`steer` px/s of it being air control): tick by tick `fall_step` and a sideways drift, held inside the stage; a parachute opens once it falls fast; it ends on the first perch its footprint comes down on or on the first head, whichever is higher. `None` when it ends on nothing within the limit.
fn fall_plan(world: &World, pet: &Pet, speed: Point, steer: f64, statics: &[Body], perches: &[Perch], now: Ticks) -> Option<Plan> {
    let half = pet.size.width / 2.0;
    let canopy = canopied(pet.size, Size { width: pet.size.width * 1.6, height: pet.size.height * 0.6 });
    let mut steps = vec![Step { x: pet.x, y: pet.y, vx: speed.x, vy: speed.y, posture: UPRIGHT }];
    let (mut x, mut y, mut vx, mut vy) = (pet.x, pet.y, speed.x, speed.y);
    let mut posture = UPRIGHT;
    let mut open = false;
    for _ in 1..=FALL_LIMIT {
        let before = extent_at(pet, x, y, posture);
        if pet.chute && !open && vy >= CHUTE_SPEED {
            open = true;
            posture = canopy;
        }
        let fallen = fall_step(y, vy);
        let sink = if open { pet.size.height * 2.0 + (vy - pet.size.height * 2.0) * CHUTE_DECAY } else { fallen.vy };
        let next_y = if open { y + sink / 64.0 } else { fallen.y };
        let mut next_x = x + vx / 64.0;
        if next_x < half || next_x > WIDTH - half {
            next_x = held(next_x, half, WIDTH - half);
            vx = 0.0;
        }
        let perch = landing_of(perches, next_x, y + pet.hover, next_y + pet.hover);
        let after = extent_at(pet, next_x, next_y, posture);
        let host = if world.rules.heads { head_under(before, after, &pet.slug, statics) } else { None };
        let on_perch = perch.map_or(f64::INFINITY, |perch| perch.y - pet.hover);
        let on_head = host.map_or(f64::INFINITY, |host| next_y + lift_onto(after, host.extent));
        if perch.is_some() || host.is_some() {
            let head = host.is_some() && on_head <= on_perch;
            steps.push(Step { x: next_x, y: if head { on_head } else { on_perch }, vx: 0.0, vy: 0.0, posture: UPRIGHT });
            let landing = match (head, host, perch) {
                (true, Some(host), _) => host.owner.clone(),
                (_, _, Some(perch)) => perch.surface.clone(),
                _ => String::new(),
            };
            return Some(Plan { claim: claimed(world, pet, &steps, now), steps, ending: if head { Ending::Head } else { Ending::Perch }, landing, steer });
        }
        steps.push(Step { x: next_x, y: next_y, vx, vy: sink, posture });
        x = next_x;
        y = next_y;
        vy = sink;
    }
    None
}

/// 🛫️ A pet takes a plan: from now on it follows its steps, and its claim is on the table.
fn install(world: &mut World, index: usize, plan: Plan, now: Ticks) {
    world.claims = released(&world.claims, &world.pets[index].slug);
    world.claims.push(plan.claim);
    let pet = &mut world.pets[index];
    pet.mode = Mode::Plan;
    pet.perch = None;
    pet.posture = UPRIGHT;
    pet.steps = plan.steps;
    pet.from = now;
    pet.ending = plan.ending;
    pet.landing = plan.landing;
}

/// 🪂️ A pet without footing looks for its way down, in the order tried: as it flies, towards the free column over the perch it would land on, with the air control of `STEERINGS`, and, when it was thrown, bounced back at half its speed or simply let go (again with the air control); the first plan that is clear is taken. While none is, it stays where it is for at most `DELAY` ticks, as long as that place is free; when nothing legal is left it poofs.
fn plunge(world: &mut World, index: usize, now: Ticks) {
    let bodies = bodies_of(world);
    let pet = &world.pets[index];
    let statics: Vec<Body> = bodies.iter().filter(|body| !world.claims.iter().any(|claim| claim.owner == body.owner) && body.owner != pet.slug).cloned().collect();
    let half = pet.size.width / 2.0;
    let perches = narrowed(&world.perches, half);
    let mut plans: Vec<Plan> = Vec::new();
    let mut chosen: Option<Plan> = None;
    let straight = fall_plan(world, pet, Point { x: pet.vx, y: pet.vy }, 0.0, &statics, &perches, now);
    if let Some(straight) = &straight {
        plans.push(straight.clone());
        if !world.rules.vetting || claim_clear(&straight.claim, &bodies, &world.claims) {
            chosen = Some(straight.clone());
        }
    }
    if chosen.is_none() && world.rules.steering {
        let mut tries: Vec<(Point, f64)> = Vec::new();
        if let Some(straight) = straight.as_ref().filter(|straight| straight.ending == Ending::Perch) {
            let perch = perch_of(world, Some(straight.landing.as_str())).expect("the perch a plan lands on");
            let column = column_over(&perch, pet.x, extent_at(pet, pet.x, pet.y, UPRIGHT), perch.y - pet.hover - pet.y, half, &obstacles_of(&pet.slug, &bodies, &world.claims, now));
            let steer = column.map_or(0.0, |column| held(((column - pet.x) * 64.0) / (straight.steps.len() - 1) as f64, -240.0, 240.0));
            if steer != 0.0 {
                tries.push((Point { x: pet.vx + steer, y: pet.vy }, steer));
            }
        }
        for &steer in &STEERINGS[1..] {
            tries.push((Point { x: pet.vx + steer, y: pet.vy }, steer));
        }
        if pet.vx != 0.0 || pet.vy != 0.0 {
            tries.push((Point { x: 0.0 - pet.vx / 2.0, y: pet.vy }, 0.0 - pet.vx / 2.0 - pet.vx));
            for steer in STEERINGS {
                tries.push((Point { x: steer, y: 0.0 }, steer - pet.vx));
            }
        }
        for (speed, steer) in tries {
            let Some(steered) = fall_plan(world, pet, speed, steer, &statics, &perches, now) else {
                continue;
            };
            plans.push(steered.clone());
            if claim_clear(&steered.claim, &bodies, &world.claims) {
                chosen = Some(steered);
                break;
            }
        }
    }
    let chosen = match chosen {
        Some(chosen) => chosen,
        None => {
            let claims: Vec<Claim> = plans.iter().map(|plan| plan.claim.clone()).collect();
            let stay = (pet.waited < DELAY).then(|| extent_of(pet));
            if !must_poof(&pet.slug, stay, &claims, &bodies, &world.claims, now) {
                world.pets[index].waited += 1;
                world.waits += 1;
                world.counts.delays += 1;
                return;
            }
            if world.rules.poof || plans.is_empty() {
                world.counts.timeouts += 1;
                poof(world, index, now);
                return;
            }
            plans.swap_remove(0)
        }
    };
    world.counts.falls += 1;
    if chosen.steer != 0.0 {
        world.counts.steered += 1;
    }
    if chosen.ending == Ending::Head {
        world.counts.heads += 1;
    }
    if chosen.steps.iter().any(|step| step.posture != UPRIGHT) {
        world.counts.chutes += 1;
    }
    install(world, index, chosen, now);
}

/// 🕊️ A planned pet moves on by one step; on the last one it lands: on its perch, or on a head it slides off from.
fn fly(world: &mut World, index: usize, now: Ticks) {
    let pet = &mut world.pets[index];
    let at = (now - pet.from) as usize;
    let step = pet.steps[at];
    pet.x = step.x;
    pet.y = step.y;
    pet.vx = step.vx;
    pet.vy = step.vy;
    pet.posture = step.posture;
    if at < pet.steps.len() - 1 {
        return;
    }
    world.claims = released(&world.claims, &world.pets[index].slug);
    world.pets[index].posture = UPRIGHT;
    if world.pets[index].ending == Ending::Perch {
        world.pets[index].perch = Some(world.pets[index].landing.clone());
        settle(world, index, now);
        return;
    }
    world.pets[index].mode = Mode::Ride;
    world.pets[index].slid = 0;
    let side = visible(world, &world.pets[index].landing).map_or(Facing::Right, |host| slide_side(extent_of(&world.pets[index]), extent_of(&world.pets[host])));
    world.pets[index].side = side;
}

/// 🛷️ A pet on a head: it falls when its host no longer carries it, and slides off otherwise.
fn slide(world: &mut World, index: usize, now: Ticks) {
    let pet = &world.pets[index];
    let extent = extent_of(pet);
    let carried = visible(world, &pet.landing).is_some_and(|host| rests_on(extent, extent_of(&world.pets[host])));
    if !carried {
        unfoot(&mut world.pets[index], 0.0, 0.0);
        return;
    }
    let slid = slide_of(extent, pet.side, pet.slid, 0.0 - MARGIN, WIDTH + MARGIN, &obstacles_of(&pet.slug, &bodies_of(world), &world.claims, now));
    let pet = &mut world.pets[index];
    pet.x += slid.stride;
    if slid.side == pet.side {
        pet.slid += 1;
    } else {
        pet.side = slid.side;
        pet.slid = 0;
    }
}

/// 🦘️ The hop of a walker to a spot: the ballistic arc of the terrain, when it really ends on that perch.
fn hop_plans(world: &World, pet: &Pet, target: &Perch, spot: f64, now: Ticks) -> Vec<Plan> {
    let from = Point { x: pet.x, y: pet.y };
    let to = Point { x: spot, y: target.y };
    let Some(hop) = hop_of(from, to) else {
        return Vec::new();
    };
    let perches = narrowed(&world.perches, pet.size.width / 2.0);
    if hop_landing(&perches, from, to, hop).is_none_or(|landing| landing.surface != target.surface) {
        return Vec::new();
    }
    let mut steps = vec![Step { x: pet.x, y: pet.y, vx: hop.vx, vy: hop.vy, posture: UPRIGHT }];
    let mut flight = Flight { x: pet.x, y: pet.y, vx: hop.vx, vy: hop.vy };
    for left in (1..=hop.ticks).rev() {
        flight = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
        steps.push(Step { x: flight.x, y: flight.y, vx: flight.vx, vy: flight.vy, posture: UPRIGHT });
    }
    vec![Plan { claim: claimed(world, pet, &steps, now), steps, ending: Ending::Perch, landing: target.surface.clone(), steer: 0.0 }]
}

/// 🎈️ The glides of a floater to a spot: a straight line at twice its speed, at its own altitude and in its high lane.
fn glide_plans(world: &World, pet: &Pet, target: &Perch, spot: f64, now: Ticks) -> Vec<Plan> {
    let dx = spot - pet.x;
    let dy = target.y - pet.hover - pet.y;
    if dx.abs() > 300.0 || dy.abs() > 200.0 {
        return Vec::new();
    }
    let ticks = larger((larger(dx.abs(), dy.abs()) / ((2.0 * pet.speed) / 64.0)).ceil(), 1.0);
    if ticks > 256.0 {
        return Vec::new();
    }
    let mut plans = Vec::new();
    for lane in [0.0, lane_lift(pet.size.height, MARGIN)] {
        let mut steps = Vec::new();
        for tick in 0..=(ticks as i64) {
            let tick = tick as f64;
            let ramp = smaller(smaller(1.0, tick / 8.0), (ticks - tick) / 8.0);
            steps.push(if tick == ticks {
                Step { x: spot, y: target.y - pet.hover, vx: 0.0, vy: 0.0, posture: UPRIGHT }
            } else {
                Step { x: pet.x + (dx * tick) / ticks, y: pet.y + (dy * tick) / ticks - lane * ramp, vx: 0.0, vy: 0.0, posture: UPRIGHT }
            });
        }
        plans.push(Plan { claim: claimed(world, pet, &steps, now), steps, ending: Ending::Perch, landing: target.surface.clone(), steer: 0.0 });
    }
    plans
}

/// 🧳️ A pet tries to travel to another perch: it draws a place near itself on every other perch, takes the free spots nearest to those places, and tries the plans to them (a hop or, floating, its glides) beginning with a perch it draws; only a plan that is clear is taken. `false` when it stays.
fn travel(world: &mut World, index: usize, now: Ticks) -> bool {
    let first = (draw(world) * world.perches.len() as f64).floor() as usize;
    let reach = (draw(world) - 0.5) * 240.0;
    let pet = world.pets[index].clone();
    let half = pet.size.width / 2.0;
    let bodies = bodies_of(world);
    let obstacles = obstacles_of(&pet.slug, &bodies, &world.claims, now);
    for turn in 0..world.perches.len() {
        let target = world.perches[(first + turn) % world.perches.len()].clone();
        if pet.perch.as_deref() == Some(target.surface.as_str()) || target.x1 - target.x0 < pet.size.width {
            continue;
        }
        let aim = held(pet.x + reach, target.x0 + half, target.x1 - half);
        let Some(spot) = spot_on(&target, aim, extent_at(&pet, aim, target.y - pet.hover, UPRIGHT), half, &obstacles) else {
            continue;
        };
        let plans = if pet.hover > 0.0 { glide_plans(world, &pet, &target, spot, now) } else { hop_plans(world, &pet, &target, spot, now) };
        for (number, plan) in plans.into_iter().enumerate() {
            if world.rules.vetting && !claim_clear(&plan.claim, &bodies, &world.claims) {
                continue;
            }
            if pet.hover > 0.0 {
                world.counts.glides += 1;
            } else {
                world.counts.hops += 1;
            }
            if number > 0 {
                world.counts.lanes += 1;
            }
            install(world, index, plan, now);
            return true;
        }
    }
    false
}

/// 🤔️ A pet that has dwelt long enough travels or sets out for a goal on its perch.
fn decide(world: &mut World, index: usize, now: Ticks) {
    let Some(perch) = perch_of(world, world.pets[index].perch.as_deref()) else {
        unfoot(&mut world.pets[index], 0.0, 0.0);
        return;
    };
    if draw(world) < 0.35 {
        if travel(world, index, now) {
            return;
        }
        world.counts.refusals += 1;
        let dwell = (draw(world) * 32.0).floor() as Ticks;
        world.pets[index].until = now + 16 + dwell;
        return;
    }
    let pet = &world.pets[index];
    let half = pet.size.width / 2.0;
    let (x, width) = (pet.x, pet.size.width);
    let goal = held(perch.x0 + half + draw(world) * (perch.x1 - perch.x0 - width), smaller(perch.x0 + half, x), larger(perch.x1 - half, x));
    let pet = &world.pets[index];
    let corridors: &[Claim] = if world.rules.corridors { &world.claims } else { &[] };
    let aim = if world.rules.guard { pet.x + guarded_stride(extent_of(pet), goal - pet.x, &obstacles_of(&pet.slug, &bodies_of(world), corridors, now)) } else { goal };
    let pet = &mut world.pets[index];
    pet.goal = aim;
    pet.mode = Mode::Walk;
    pet.waited = 0;
}

/// 👣️ A walker takes its guarded stride; hindered, it waits, and gives its goal up when its patience is over.
fn stride(world: &mut World, index: usize, now: Ticks) {
    let pet = &world.pets[index];
    let next = stride_to(pet.x, pet.goal, pet.speed);
    let wanted = next - pet.x;
    let corridors: &[Claim] = if world.rules.corridors { &world.claims } else { &[] };
    let allowed = if world.rules.guard { guarded_stride(extent_of(pet), wanted, &obstacles_of(&pet.slug, &bodies_of(world), corridors, now)) } else { wanted };
    let pet = &mut world.pets[index];
    if allowed == wanted {
        pet.x = next;
        if pet.x == pet.goal {
            settle(world, index, now);
        }
        return;
    }
    pet.x += allowed;
    pet.waited += 1;
    let patient = pet.waited <= PATIENCE;
    world.waits += 1;
    world.counts.stalls += 1;
    if !patient {
        settle(world, index, now);
    }
}

/// 🤏️ A held pet follows the hand, leaning by how fast it is dragged; it is pushed out of whatever it is dragged into, and keeps its place when that fails or would leave the stage.
fn follow(world: &mut World, index: usize, now: Ticks) {
    let Some(hand) = world.hand else {
        return;
    };
    let pet = &world.pets[index];
    let posture = leaning(pet.size.height, smaller((hand.x - hand.px).abs() / 8.0, 1.0));
    let mut x = hand.x;
    let mut y = hand.y;
    if world.rules.projection {
        let Some(push) = pushed_out(extent_at(pet, x, y, posture), &obstacles_of(&pet.slug, &bodies_of(world), &world.claims, now), PUSHES) else {
            return;
        };
        x += push.x;
        y += push.y;
        if x < pet.size.width / 2.0 || x > WIDTH - pet.size.width / 2.0 || y > FLOOR - pet.hover {
            return;
        }
    }
    let pet = &mut world.pets[index];
    pet.x = x;
    pet.y = y;
    pet.posture = posture;
}

/// 🧲️ The hand heads for another pet (to drag its pet into it) or for any point of the stage.
fn retarget(world: &mut World, hand: &mut Hand) {
    let others: Vec<(f64, f64)> = world.pets.iter().enumerate().filter(|(at, pet)| pet.mode != Mode::Gone && *at != hand.pet).map(|(_, pet)| (pet.x, pet.y)).collect();
    if !others.is_empty() && draw(world) < 0.5 {
        let other = others[(draw(world) * others.len() as f64).floor() as usize];
        hand.tx = other.0;
        hand.ty = other.1;
        return;
    }
    hand.tx = 40.0 + draw(world) * 880.0;
    hand.ty = 60.0 + draw(world) * 440.0;
}

/// ✊️ The hand of the world: now and then it picks any pet up, wherever it is, drags it about for a while and lets it go, thrown with the speed of the hand or just dropped.
fn handle(world: &mut World) {
    let Some(mut hand) = world.hand else {
        let grab = draw(world) < 1.0 / 120.0;
        if !grab {
            return;
        }
        let shown: Vec<usize> = (0..world.pets.len()).filter(|&at| world.pets[at].mode != Mode::Gone).collect();
        if shown.is_empty() {
            return;
        }
        let index = shown[(draw(world) * shown.len() as f64).floor() as usize];
        world.claims = released(&world.claims, &world.pets[index].slug);
        world.pets[index].mode = Mode::Held;
        world.pets[index].perch = None;
        world.counts.grabs += 1;
        let pace = 4.0 + draw(world) * 10.0;
        let left = 20 + (draw(world) * 100.0).floor() as i64;
        let pet = &world.pets[index];
        let mut hand = Hand { pet: index, x: pet.x, y: pet.y, px: pet.x, py: pet.y, tx: pet.x, ty: pet.y, pace, left };
        retarget(world, &mut hand);
        world.hand = Some(hand);
        return;
    };
    hand.left -= 1;
    if hand.left <= 0 {
        let thrown = draw(world) < 0.5;
        unfoot(&mut world.pets[hand.pet], if thrown { held((hand.x - hand.px) * 64.0, -640.0, 640.0) } else { 0.0 }, if thrown { held((hand.y - hand.py) * 64.0, -520.0, 640.0) } else { 0.0 });
        world.hand = None;
        return;
    }
    hand.px = hand.x;
    hand.py = hand.y;
    let dx = hand.tx - hand.x;
    let dy = hand.ty - hand.y;
    let far = larger(dx.abs(), dy.abs());
    if far <= hand.pace {
        hand.x = hand.tx;
        hand.y = hand.ty;
        retarget(world, &mut hand);
    } else {
        hand.x += (dx * hand.pace) / far;
        hand.y += (dy * hand.pace) / far;
    }
    let pet = &world.pets[hand.pet];
    hand.y = smaller(hand.y, FLOOR - pet.hover);
    hand.x = held(hand.x, pet.size.width / 2.0, WIDTH - pet.size.width / 2.0);
    world.hand = Some(hand);
}

/// 🏗️ A survey now and then: a shelf vanishes, shrinks, jumps by up to 60 px or appears. Riders ride a shelf that moved, and whoever it carried into somebody poofs; riders of a shelf that vanished fall; whoever is in the air plans anew; the riders of the shelf are seated again and scoot to their seats.
fn survey(world: &mut World, now: Ticks) {
    let happens = draw(world) < 1.0 / 300.0;
    if !happens {
        return;
    }
    world.counts.surveys += 1;
    let kind = draw(world);
    for index in 0..world.pets.len() {
        if world.pets[index].mode != Mode::Plan {
            continue;
        }
        world.claims = released(&world.claims, &world.pets[index].slug);
        let (vx, vy) = (world.pets[index].vx, world.pets[index].vy);
        unfoot(&mut world.pets[index], vx, vy);
    }
    if world.perches.len() == 1 || (kind < 0.25 && !world.spare.is_empty()) {
        if let Some(spare) = world.spare.pop() {
            world.perches.push(spare);
        }
        return;
    }
    let index = 1 + (draw(world) * (world.perches.len() - 1) as f64).floor() as usize;
    let old = world.perches[index].clone();
    let riders: Vec<usize> = (0..world.pets.len()).filter(|&at| grounded(&world.pets[at]) && world.pets[at].perch.as_deref() == Some(old.surface.as_str())).collect();
    if kind < 0.5 {
        world.perches.remove(index);
        world.spare.push(old);
        for at in riders {
            unfoot(&mut world.pets[at], 0.0, 0.0);
        }
        return;
    }
    let fresh = if kind < 0.75 {
        let width = (0.4 + 0.4 * draw(world)) * (old.x1 - old.x0);
        let x0 = old.x0 + draw(world) * (old.x1 - old.x0 - width);
        Perch { surface: old.surface.clone(), x0, x1: x0 + width, y: old.y }
    } else {
        let x0 = held(old.x0 + (draw(world) - 0.5) * 120.0, 0.0, WIDTH - (old.x1 - old.x0));
        let y = held(old.y + (draw(world) - 0.5) * 120.0, 100.0, 480.0);
        let fresh = Perch { surface: old.surface.clone(), x0, x1: x0 + (old.x1 - old.x0), y };
        let mut moved = riders;
        let mut at = 0;
        while at < moved.len() {
            for candidate in 0..world.pets.len() {
                if world.pets[candidate].mode == Mode::Ride && world.pets[candidate].landing == world.pets[moved[at]].slug && !moved.contains(&candidate) {
                    moved.push(candidate);
                }
            }
            at += 1;
        }
        for &rider in &moved {
            let pet = &mut world.pets[rider];
            pet.x += fresh.x0 - old.x0;
            pet.y += fresh.y - old.y;
            pet.goal += fresh.x0 - old.x0;
        }
        if world.rules.eviction {
            let bodies = bodies_of(world);
            let carried = |body: &Body| moved.iter().any(|&rider| world.pets[rider].slug == body.owner);
            let ranked: Vec<Body> = bodies.iter().filter(|body| !carried(body)).chain(bodies.iter().filter(|body| carried(body))).cloned().collect();
            for owner in evicted(&ranked, &world.claims, now) {
                world.counts.evictions += 1;
                let gone = world.pets.iter().position(|pet| pet.slug == owner).expect("an evicted pet");
                poof(world, gone, now);
            }
        }
        fresh
    };
    world.perches[index] = fresh.clone();
    let members: Vec<usize> = (0..world.pets.len()).filter(|&at| grounded(&world.pets[at]) && world.pets[at].perch.as_deref() == Some(fresh.surface.as_str())).collect();
    if !world.rules.seating {
        for at in members {
            let pet = &world.pets[at];
            if pet.x >= fresh.x0 + pet.size.width / 2.0 && pet.x <= fresh.x1 - pet.size.width / 2.0 {
                continue;
            }
            world.counts.spilled += 1;
            unfoot(&mut world.pets[at], 0.0, 0.0);
        }
        return;
    }
    let bodies: Vec<Body> = members.iter().map(|&at| Body { owner: world.pets[at].slug.clone(), extent: extent_of(&world.pets[at]) }).collect();
    let seating = seat_of(&bodies, &fresh, MARGIN);
    let member = |world: &World, owner: &str| members.iter().copied().find(|&at| world.pets[at].slug == owner).expect("a seated member");
    for owner in &seating.leavers {
        let at = member(world, owner);
        let pet = &world.pets[at];
        if pet.x < fresh.x0 + pet.size.width / 2.0 || pet.x > fresh.x1 - pet.size.width / 2.0 {
            world.counts.spilled += 1;
            unfoot(&mut world.pets[at], 0.0, 0.0);
        } else {
            world.counts.crowded += 1;
            poof(world, at, now);
        }
    }
    if !seating.seats.iter().any(|seat| seat.shift != 0.0) {
        return;
    }
    for seat in &seating.seats {
        let at = member(world, &seat.owner);
        if seat.shift != 0.0 {
            world.counts.seated += 1;
        }
        let pet = &mut world.pets[at];
        pet.mode = Mode::Scoot;
        pet.goal = pet.x + seat.shift;
        pet.waited = 0;
    }
}

/// 🚌️ Everybody who scoots to a seat moves by the one common share of the way, perch by perch, and only when every place on the way is free; a group that is held up too long gives up: whoever is on the perch stays where it is, whoever is not falls.
fn scoot(world: &mut World, now: Ticks) {
    for perch in world.perches.clone() {
        let members: Vec<usize> = (0..world.pets.len()).filter(|&at| world.pets[at].mode == Mode::Scoot && world.pets[at].perch.as_deref() == Some(perch.surface.as_str())).collect();
        if members.is_empty() {
            continue;
        }
        let shifts: Vec<f64> = members.iter().map(|&at| world.pets[at].goal - world.pets[at].x).collect();
        let fraction = scoot_fraction(&shifts, (SCOOT_HASTE * 48.0) / 64.0);
        let others: Vec<Body> = bodies_of(world).into_iter().filter(|body| !members.iter().any(|&at| world.pets[at].slug == body.owner)).collect();
        let obstacles = obstacles_of("", &others, &world.claims, now);
        let places: Vec<f64> = members.iter().map(|&at| scooted(world.pets[at].x, world.pets[at].goal, fraction)).collect();
        if members.iter().zip(&places).all(|(&at, &place)| free_among(extent_at(&world.pets[at], place, world.pets[at].y, UPRIGHT), &obstacles)) {
            for (&at, &place) in members.iter().zip(&places) {
                world.pets[at].x = place;
                if fraction >= 1.0 {
                    settle(world, at, now);
                }
            }
            continue;
        }
        world.waits += members.len() as u64;
        for &at in &members {
            world.pets[at].waited += 1;
            if world.pets[at].waited <= PATIENCE {
                continue;
            }
            if world.pets[at].x < perch.x0 || world.pets[at].x > perch.x1 {
                world.counts.spilled += 1;
                unfoot(&mut world.pets[at], 0.0, 0.0);
            } else {
                settle(world, at, now);
            }
        }
    }
}

/// 🎬️ The turn of one pet.
fn act(world: &mut World, index: usize, now: Ticks) {
    match world.pets[index].mode {
        Mode::Gone => {
            if now >= world.pets[index].until && !arrive(world, index, now) {
                world.pets[index].until = now + 64;
            }
        }
        Mode::Held => follow(world, index, now),
        Mode::Plan => fly(world, index, now),
        Mode::Drop => plunge(world, index, now),
        Mode::Ride => slide(world, index, now),
        Mode::Walk => stride(world, index, now),
        Mode::Stand if now >= world.pets[index].until => decide(world, index, now),
        Mode::Stand | Mode::Scoot => {}
    }
}

/// ⏭️ One tick of a world: the claims are pruned, the survey and the hand act, the seated scoot, every pet takes its turn (the first one rotates), and the tick is tallied: overlaps, near misses, poofs, waits, and whether the order on every perch was kept.
pub(crate) fn tick_world(world: &mut World) {
    world.tick += 1;
    let now = world.tick;
    let poofs = world.poofs;
    let waits = world.waits;
    world.claims = pruned(&world.claims, now);
    survey(world, now);
    handle(world);
    scoot(world, now);
    for turn in 0..world.pets.len() {
        act(world, (now as usize + turn) % world.pets.len(), now);
    }
    let bodies = bodies_of(world);
    world.tally = tallied(world.tally, &bodies, NEAR, world.poofs - poofs, world.waits - waits);
    if near_misses(&bodies, NEAR).iter().any(|pair| world.pets.iter().any(|pet| pet.mode == Mode::Plan && (pet.slug == pair.first || pet.slug == pair.second))) {
        world.counts.brushes += 1;
    }
    let mut orders = Vec::new();
    for perch in &world.perches {
        let seated: Vec<Body> = world.pets.iter().filter(|pet| grounded(pet) && pet.perch.as_deref() == Some(perch.surface.as_str())).map(|pet| Body { owner: pet.slug.clone(), extent: extent_of(pet) }).collect();
        let order = order_of(&seated);
        if let Some((_, before)) = world.orders.iter().find(|(surface, _)| *surface == perch.surface) {
            if !order_kept(before, &order) {
                world.counts.disorders += 1;
            }
        }
        orders.push((perch.surface.clone(), order));
    }
    world.orders = orders;
}

/// 🏁️ A world run for a number of ticks.
pub(crate) fn run_world(seed: u32, ticks: Ticks, rules: Rules, span: Ticks) -> World {
    let mut world = open_world(seed, rules, span);
    for _ in 0..ticks {
        tick_world(&mut world);
    }
    world
}
//#endregion 🔖️Reference world

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

/// 🧰️ A box by its four edges.
fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Extent {
    Extent { x0, y0, x1, y1 }
}

/// 🏷️ A body by its owner and the four edges of its box.
fn body(owner: &str, x0: f64, y0: f64, x1: f64, y1: f64) -> Body {
    Body { owner: owner.to_string(), extent: rect(x0, y0, x1, y1) }
}

/// 👥️ A pair by its two owners.
fn pair(first: &str, second: &str) -> Pair {
    Pair { first: first.to_string(), second: second.to_string() }
}

/// 🔤️ Owned slugs of a list of names.
fn slugs(names: &[&str]) -> Vec<Slug> {
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
fn marching(x: f64, y: f64, dx: f64, dy: f64, ticks: usize) -> Vec<Extent> {
    (0..ticks).map(|tick| tick as f64).map(|tick| rect(x + dx * tick, y + dy * tick, x + dx * tick + 40.0, y + dy * tick + 50.0)).collect()
}

/// 🕹️ The point the hop or fall of a vector plans at the end of a tick.
fn feet(x: f64, y: f64) -> Point {
    Point { x, y }
}

/// 🌐️ Holds every world of the seeds to the invariant and the order, for a number of ticks each, and that the world was busy.
fn kept_apart(worlds: u32, ticks: Ticks) {
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
fn kept_apart_in_slices(worlds: u32, ticks: Ticks) {
    for seed in 1..=worlds {
        let world = run_world(seed, ticks, RULES, 4);
        assert_eq!([world.tally.overlaps, world.counts.disorders], [0, 0], "seed {seed}");
    }
}

/// 🔬️ Holds the invariant to a second overlap test, the area of the intersection, tick by tick, in a world with every rule and in one without the guard.
fn judged_alike(ticks: Ticks) {
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
fn reproduced(ticks: Ticks) {
    let one = run_world(5, ticks, RULES, 1);
    let other = run_world(5, ticks, RULES, 1);
    assert_eq!(other.tally, one.tally);
    assert_eq!(other.counts.listed(), one.counts.listed());
    assert_eq!(bodies_of(&other), bodies_of(&one));
    assert_ne!(bodies_of(&run_world(6, ticks, RULES, 1)), bodies_of(&one));
}

/// 💀️ Holds that every core rule bites: without it bodies overlap in its cheapest run (and, given `ablations`, in the sweep of the research prototype).
fn bitten(ablations: u32, ablation_ticks: Ticks) {
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
fn poofs_needed(seeds: u32, ticks: Ticks) {
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
fn quality_optional(seeds: u32, ticks: Ticks) {
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
fn overlaps_agree_with_areas(cases: usize) {
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
fn slots_agree_with_the_lattice(cases: usize) {
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
fn claims_agree_with_time_lines(cases: usize) {
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
fn strides_agree_with_the_lattice(cases: usize) {
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
fn walkers_kept_in_order(ticks: usize) {
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
fn seatings_agree_with_every_pooling(cases: usize) {
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
fn scoots_stay_apart(cases: usize) {
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
fn pushes_free_or_nothing(cases: usize) {
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

#[test]
fn constants_are_the_starting_values_of_the_design() {
    assert_eq!([MARGIN, SEAM], [4.0, 1.0 / 64.0]);
    assert_eq!(FOREVER, (1_i64 << 32) - 1);
    assert_eq!((PUSHES, PATIENCE, DELAY), (4, 40, 32));
    assert_eq!([SLIDE_OFF_SPEED, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, LEAN, LANE_LIFT, SCOOT_HASTE], [32.0, 2.0, 128.0, 0.25, 0.9, 1.5]);
    assert_eq!(STEERINGS, [0.0, 30.0, -30.0, 60.0, -60.0, 90.0, -90.0, 130.0, -130.0, 180.0, -180.0, 240.0, -240.0]);
    assert_eq!(TALLY, Tally::default());
}

#[test]
fn the_box_is_built_around_the_feet_from_size_hover_posture_and_margin() {
    let size = Size { width: 40.0, height: 48.0 };
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, UPRIGHT, 4.0), body("a", 76.0, 248.0, 124.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 290.0), size, 10.0, UPRIGHT, 4.0).extent, rect(76.0, 238.0, 124.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, leaning(48.0, 1.0), 4.0).extent, rect(64.0, 248.0, 136.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, canopied(size, Size { width: 64.0, height: 28.0 }), 4.0).extent, rect(64.0, 220.0, 136.0, 304.0));
    assert_eq!(body_of("a", feet(100.0, 300.0), size, 0.0, Posture { left: 3.0, right: 14.0, above: 0.0 }, 0.0).extent, rect(77.0, 252.0, 134.0, 300.0));
}

#[test]
fn a_leaning_body_widens_by_a_quarter_of_its_height_times_the_sine_and_a_canopied_one_by_the_overhang() {
    let size = Size { width: 40.0, height: 48.0 };
    assert_eq!(leaning(48.0, 1.0), Posture { left: 12.0, right: 12.0, above: 0.0 });
    assert_eq!(leaning(48.0, -0.5), Posture { left: 6.0, right: 6.0, above: 0.0 });
    assert_eq!(leaning(48.0, 0.0), UPRIGHT);
    assert_eq!(canopied(size, Size { width: 64.0, height: 28.0 }), Posture { left: 12.0, right: 12.0, above: 28.0 });
    assert_eq!(canopied(size, Size { width: 30.0, height: 20.0 }), Posture { left: 0.0, right: 0.0, above: 20.0 });
    assert_eq!(leaning(48.0, 1.0).left, LEAN * 48.0);
}

#[test]
fn neighbours_with_the_comfortable_gap_touch_without_overlapping() {
    let size = Size { width: 40.0, height: 48.0 };
    let small = Size { width: 30.0, height: 40.0 };
    let one = body_of("a", feet(100.0, 300.0), size, 0.0, UPRIGHT, MARGIN);
    let apart = body_of("b", feet(100.0 + (40.0 + 30.0) / 2.0 + 2.0 * MARGIN, 300.0), small, 0.0, UPRIGHT, MARGIN);
    let closer = body_of("b", feet(100.0 + (40.0 + 30.0) / 2.0 + 2.0 * MARGIN - 1.0 / 1024.0, 300.0), small, 0.0, UPRIGHT, MARGIN);
    assert_eq!(apart.extent.x0, one.extent.x1);
    assert_eq!(overlaps(&[one.clone(), apart]), Vec::new());
    assert_eq!(overlaps(&[one, closer]), vec![pair("a", "b")]);
}

#[test]
fn boxes_meet_only_where_open_boxes_intersect() {
    let one = rect(0.0, 0.0, 10.0, 10.0);
    assert!(!meets(one, rect(10.0, 0.0, 20.0, 10.0)));
    assert!(!meets(one, rect(0.0, 10.0, 10.0, 20.0)));
    assert!(meets(one, rect(9.999, 9.999, 20.0, 20.0)));
    assert!(meets(one, rect(2.0, 2.0, 3.0, 3.0)));
    assert!(meets(rect(2.0, 2.0, 3.0, 3.0), one));
    assert!(meets(one, rect(-5.0, 4.0, 15.0, 6.0)));
    assert!(!meets(one, rect(11.0, 0.0, 20.0, 10.0)));
    assert!(meets(one, one));
}

#[test]
fn boxes_move_grow_and_unite() {
    assert_eq!(shifted(rect(1.0, 2.0, 3.0, 4.0), 10.0, -1.0), rect(11.0, 1.0, 13.0, 3.0));
    assert_eq!(grown(rect(1.0, 2.0, 3.0, 4.0), 0.5), rect(0.5, 1.5, 3.5, 4.5));
    assert_eq!(united(rect(1.0, 2.0, 3.0, 4.0), rect(-1.0, 3.0, 2.0, 9.0)), rect(-1.0, 2.0, 3.0, 9.0));
}

#[test]
fn every_overlapping_pair_is_named_by_the_first_and_then_the_second_in_the_list() {
    let bodies = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 5.0, 5.0, 15.0, 15.0), body("c", 8.0, 8.0, 20.0, 20.0), body("d", 100.0, 100.0, 110.0, 110.0)];
    assert_eq!(overlaps(&bodies), vec![pair("a", "b"), pair("a", "c"), pair("b", "c")]);
    assert_eq!(overlaps(&[bodies[3].clone(), bodies[2].clone(), bodies[0].clone()]), vec![pair("c", "a")]);
    assert_eq!(overlaps(&[]), Vec::new());
    assert_eq!(overlaps(&bodies[..1]), Vec::new());
}

#[test]
fn near_misses_are_the_pairs_that_are_apart_but_closer_than_the_margin() {
    let bodies = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 11.0, 0.0, 20.0, 10.0), body("c", 5.0, 5.0, 8.0, 8.0), body("e", 11.0, 11.0, 20.0, 20.0)];
    assert_eq!(near_misses(&bodies, 2.0), vec![pair("a", "b"), pair("a", "e"), pair("b", "e")]);
    assert_eq!(near_misses(&bodies, 1.5), near_misses(&bodies, 2.0));
    assert_eq!(near_misses(&bodies, 1.0), Vec::new());
    assert_eq!(near_misses(&bodies, 0.0), Vec::new());
}

#[test]
fn overlaps_agree_with_the_area_of_the_intersection_on_generated_bodies() {
    overlaps_agree_with_areas(40);
}

/// 🧩️ The bodies and claims of the cases of free places.
fn free_scene() -> ([Body; 2], [Claim; 2]) {
    let bodies = [body("a", 0.0, 0.0, 10.0, 10.0), body("b", 20.0, 0.0, 30.0, 10.0)];
    let claims = [
        Claim { owner: "c".to_string(), slices: vec![Slice { from: 5, until: 5, extent: rect(40.0, 0.0, 50.0, 10.0) }, Slice { from: 6, until: 9, extent: rect(60.0, 0.0, 70.0, 10.0) }], rest: Some(rect(80.0, 0.0, 90.0, 10.0)) },
        Claim { owner: "a".to_string(), slices: vec![Slice { from: 5, until: 9, extent: rect(200.0, 0.0, 210.0, 10.0) }], rest: Some(rect(220.0, 0.0, 230.0, 10.0)) },
    ];
    (bodies, claims)
}

#[test]
fn obstacles_are_the_bodies_of_the_others_the_corridors_that_are_left_and_the_rests() {
    let (bodies, claims) = free_scene();
    assert_eq!(obstacles_of("a", &bodies, &claims, 5), vec![rect(20.0, 0.0, 30.0, 10.0), rect(40.0, 0.0, 50.0, 10.0), rect(60.0, 0.0, 70.0, 10.0), rect(80.0, 0.0, 90.0, 10.0)]);
    assert_eq!(obstacles_of("a", &bodies, &claims, 6), vec![rect(20.0, 0.0, 30.0, 10.0), rect(60.0, 0.0, 70.0, 10.0), rect(80.0, 0.0, 90.0, 10.0)]);
    assert_eq!(obstacles_of("a", &bodies, &claims, 10), vec![rect(20.0, 0.0, 30.0, 10.0), rect(80.0, 0.0, 90.0, 10.0)]);
    assert_eq!(obstacles_of("c", &bodies, &claims, 10), vec![rect(0.0, 0.0, 10.0, 10.0), rect(20.0, 0.0, 30.0, 10.0), rect(220.0, 0.0, 230.0, 10.0)]);
    assert_eq!(obstacles_of("nobody", &[], &[], 0), Vec::new());
}

#[test]
fn a_place_is_free_when_no_body_no_corridor_that_is_left_and_no_rest_of_another_owner_overlaps_it() {
    let (bodies, claims) = free_scene();
    assert!(free_at(rect(10.0, 0.0, 20.0, 10.0), "x", &bodies, &claims, 5));
    assert!(!free_at(rect(5.0, 0.0, 15.0, 10.0), "x", &bodies, &claims, 5));
    assert!(free_at(rect(5.0, 0.0, 15.0, 10.0), "a", &bodies, &claims, 5));
    assert!(!free_at(rect(45.0, 0.0, 55.0, 10.0), "x", &bodies, &claims, 5));
    assert!(free_at(rect(45.0, 0.0, 55.0, 10.0), "x", &bodies, &claims, 6));
    assert!(!free_at(rect(85.0, 0.0, 95.0, 10.0), "x", &bodies, &claims, 1000));
    assert!(free_at(rect(85.0, 0.0, 95.0, 10.0), "c", &bodies, &claims, 1000));
    assert!(free_among(rect(10.0, 0.0, 20.0, 10.0), &[rect(0.0, 0.0, 10.0, 10.0), rect(20.0, 0.0, 30.0, 10.0)]));
    assert!(!free_among(rect(10.0, 0.0, 20.5, 10.0), &[rect(0.0, 0.0, 10.0, 10.0), rect(20.0, 0.0, 30.0, 10.0)]));
}

#[test]
fn the_nearest_place_keeps_a_seam_from_every_obstacle_of_its_height() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 260.0, 150.0, 300.0)]), Some(90.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(60.0, 260.0, 90.0, 300.0)]), Some(110.0 + SEAM));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(90.0, 260.0, 110.0, 300.0)]), Some(70.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 100.0, 150.0, 200.0)]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 200.0, 150.0, 250.0)]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 300.0, 150.0, 350.0)]), Some(100.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 400.0, &[rect(110.0, 200.0, 150.0, 250.25)]), Some(90.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 150.0, 400.0, &[]), Some(150.0));
    assert_eq!(slot_in(100.0, extent, 20.0, 60.0, &[]), Some(60.0));
}

#[test]
fn a_single_point_is_answered_when_that_is_all_there_is_and_nothing_without_room() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let obstacle = [rect(110.0, 260.0, 150.0, 300.0)];
    assert_eq!(slot_in(100.0, extent, 90.0 - SEAM, 160.0, &obstacle), Some(90.0 - SEAM));
    assert_eq!(slot_in(100.0, extent, 90.0, 110.0, &obstacle), None);
    assert_eq!(slot_in(100.0, extent, 90.0, 170.0, &obstacle), None);
    assert_eq!(slot_in(100.0, extent, 90.0, 170.0 + SEAM, &obstacle), Some(170.0 + SEAM));
    assert_eq!(slot_in(100.0, extent, 120.0, 110.0, &[]), None);
    assert_eq!(slot_in(100.0, extent, f64::NAN, 110.0, &[]), None);
}

#[test]
fn the_footprint_stays_on_the_perch_and_a_landing_looks_down_the_whole_column() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let perch = Perch { surface: "s".to_string(), x0: 50.0, x1: 300.0, y: 300.0 };
    assert_eq!(spot_on(&perch, 100.0, extent, 16.0, &[]), Some(100.0));
    assert_eq!(spot_on(&perch, 40.0, shifted(extent, -60.0, 0.0), 16.0, &[]), Some(66.0));
    assert_eq!(spot_on(&perch, 400.0, shifted(extent, 300.0, 0.0), 16.0, &[]), Some(284.0));
    assert_eq!(spot_on(&Perch { surface: "s".to_string(), x0: 50.0, x1: 70.0, y: 300.0 }, 60.0, shifted(extent, -40.0, 0.0), 16.0, &[]), None);
    assert_eq!(spot_on(&perch, 100.0, extent, 16.0, &[rect(110.0, 260.0, 150.0, 300.0)]), Some(90.0 - SEAM));
    let high = rect(80.0, 100.0, 120.0, 150.0);
    let below = [rect(110.0, 200.0, 150.0, 250.0)];
    assert_eq!(spot_on(&perch, 100.0, high, 16.0, &below), Some(100.0));
    assert_eq!(column_over(&perch, 100.0, high, 150.0, 16.0, &below), Some(90.0 - SEAM));
    assert_eq!(column_over(&perch, 100.0, high, 50.0, 16.0, &below), Some(100.0));
    assert_eq!(column_over(&perch, 100.0, high, -30.0, 16.0, &below), Some(100.0));
}

#[test]
fn free_places_agree_with_a_search_over_every_place_of_a_lattice() {
    slots_agree_with_the_lattice(40);
}

#[test]
fn a_path_is_cut_into_slices_of_a_span_each_the_smallest_box_around_its_ticks() {
    let path = marching(0.0, 0.0, 10.0, 1.0, 10);
    let exact = claim_of("a", 100, &path, 1, Some(path[9]));
    assert_eq!(exact.owner, "a");
    assert_eq!(exact.slices.len(), 10);
    assert_eq!(exact.slices[3], Slice { from: 103, until: 103, extent: path[3] });
    assert_eq!(exact.rest, Some(path[9]));
    let coarse = claim_of("a", 100, &path, 4, None);
    assert_eq!(coarse.slices, vec![Slice { from: 100, until: 103, extent: rect(0.0, 0.0, 70.0, 53.0) }, Slice { from: 104, until: 107, extent: rect(40.0, 4.0, 110.0, 57.0) }, Slice { from: 108, until: 109, extent: rect(80.0, 8.0, 130.0, 59.0) }]);
    assert_eq!(coarse.rest, None);
    assert_eq!(claim_of("a", 7, &path, 0, None).slices.len(), 10);
    assert_eq!(claim_of("a", 7, &path, -3, None).slices.len(), 10);
    assert_eq!(claim_of("a", 7, &path, 2, None).slices.len(), 5);
    assert_eq!(claim_of("a", 7, &[], 3, None).slices, Vec::new());
}

#[test]
fn a_claim_answers_where_it_has_its_owner_at_a_tick() {
    let path = marching(0.0, 0.0, 10.0, 0.0, 6);
    let claim = claim_of("a", 10, &path, 2, Some(rect(500.0, 0.0, 540.0, 50.0)));
    assert_eq!(slice_at(&claim, 9), None);
    assert_eq!(slice_at(&claim, 10), Some(rect(0.0, 0.0, 50.0, 50.0)));
    assert_eq!(slice_at(&claim, 13), Some(rect(20.0, 0.0, 70.0, 50.0)));
    assert_eq!(slice_at(&claim, 15), Some(rect(40.0, 0.0, 90.0, 50.0)));
    assert_eq!(slice_at(&claim, 16), Some(rect(500.0, 0.0, 540.0, 50.0)));
    assert_eq!(slice_at(&claim_of("a", 10, &path, 2, None), 16), None);
    assert_eq!(slice_at(&claim_of("a", 10, &[], 2, Some(rect(0.0, 0.0, 1.0, 1.0))), 16), None);
}

#[test]
fn a_plan_that_runs_into_a_body_that_stays_is_refused_and_the_owners_own_body_ignored() {
    let path = marching(0.0, 0.0, 10.0, 0.0, 20);
    let plan = claim_of("a", 0, &path, 1, Some(path[19]));
    assert!(claim_clear(&plan, &[body("a", 0.0, 0.0, 40.0, 50.0)], &[]));
    assert!(claim_clear(&plan, &[body("a", 0.0, 0.0, 40.0, 50.0), body("b", 100.0, 60.0, 140.0, 110.0)], &[]));
    assert!(!claim_clear(&plan, &[body("a", 0.0, 0.0, 40.0, 50.0), body("b", 100.0, 40.0, 140.0, 90.0)], &[]));
    assert!(claim_clear(&plan, &[body("b", 230.0, 0.0, 270.0, 50.0)], &[]));
    assert!(!claim_clear(&plan, &[body("b", 229.0, 0.0, 269.0, 50.0)], &[]));
}

#[test]
fn two_plans_may_cross_one_place_at_different_ticks_but_not_at_the_same_tick() {
    let across = claim_of("a", 20, &marching(0.0, 100.0, 10.0, 0.0, 21), 1, Some(rect(200.0, 100.0, 240.0, 150.0)));
    let down = |from: Ticks| claim_of("b", from, &marching(100.0, -100.0, 0.0, 10.0, 31), 1, Some(rect(100.0, 200.0, 140.0, 250.0)));
    let crossing = std::slice::from_ref(&across);
    assert!(claim_clear(&down(0), &[], crossing));
    assert!(!claim_clear(&down(10), &[], crossing));
    assert!(claim_clear(&down(30), &[], crossing));
    assert!(claim_clear(&across, &[], &[down(0)]));
    assert!(!claim_clear(&across, &[], &[down(10)]));
    assert!(claim_clear(&across, &[], &[down(30)]));
}

#[test]
fn a_rest_where_somebody_arrives_later_and_a_path_through_somebodys_rest_are_refused() {
    let late = claim_of("a", 0, &marching(0.0, 0.0, 10.0, 0.0, 51), 1, Some(rect(500.0, 0.0, 540.0, 50.0)));
    let early = claim_of("b", 0, &marching(500.0, -300.0, 0.0, 10.0, 31), 1, Some(rect(500.0, 0.0, 540.0, 50.0)));
    let arriving = std::slice::from_ref(&late);
    assert!(!claim_clear(&early, &[], arriving));
    assert!(claim_clear(&claim_of("b", 0, &marching(500.0, -300.0, 0.0, 10.0, 31), 1, None), &[], arriving));
    assert!(claim_clear(&claim_of("b", 0, &marching(560.0, -300.0, 0.0, 10.0, 31), 1, Some(rect(560.0, 0.0, 600.0, 50.0))), &[], arriving));
    let through = claim_of("b", 100, &marching(500.0, -300.0, 0.0, 10.0, 61), 1, Some(rect(500.0, 300.0, 540.0, 350.0)));
    assert!(!claim_clear(&through, &[], arriving));
    assert!(claim_clear(&through, &[], &[claim_of("a", 0, &marching(0.0, 0.0, 10.0, 0.0, 51), 1, None)]));
}

#[test]
fn a_planned_body_is_where_its_claim_says_and_where_it_stands_until_that_claim_begins() {
    let leaving = claim_of("a", 0, &marching(100.0, 0.0, 10.0, 0.0, 31), 1, Some(rect(400.0, 0.0, 440.0, 50.0)));
    let landing = claim_of("b", 0, &marching(100.0, -300.0, 0.0, 10.0, 31), 1, Some(rect(100.0, 0.0, 140.0, 50.0)));
    let standing = [body("a", 100.0, 0.0, 140.0, 50.0)];
    assert!(claim_clear(&landing, &standing, std::slice::from_ref(&leaving)));
    assert!(!claim_clear(&landing, &standing, &[]));
    let later = claim_of("a", 40, &marching(300.0, 0.0, 10.0, 0.0, 11), 1, Some(rect(400.0, 0.0, 440.0, 50.0)));
    assert!(!claim_clear(&landing, &standing, std::slice::from_ref(&later)));
    assert!(claim_clear(&landing, &[], std::slice::from_ref(&later)));
    assert!(claim_clear(&landing, &standing, &[claim_of("c", 0, &marching(900.0, 0.0, 0.0, 0.0, 5), 1, None), leaving.clone()]));
    assert!(!claim_clear(&landing, &standing, &[leaving, claim_of("a", 0, &marching(100.0, 0.0, 0.0, 0.0, 40), 1, None)]));
}

#[test]
fn claims_agree_with_a_comparison_of_every_tick_of_every_time_line() {
    claims_agree_with_time_lines(40);
}

#[test]
fn the_claims_of_an_owner_are_released_and_what_is_over_is_pruned() {
    let one = claim_of("a", 0, &marching(0.0, 0.0, 1.0, 0.0, 10), 5, Some(rect(9.0, 0.0, 49.0, 50.0)));
    let two = claim_of("b", 3, &marching(0.0, 0.0, 1.0, 0.0, 4), 2, None);
    let both = [one.clone(), two.clone()];
    assert_eq!(released(&both, "a"), vec![two.clone()]);
    assert_eq!(released(&both, "c"), both.to_vec());
    assert_eq!(pruned(&both, 0), both.to_vec());
    assert_eq!(pruned(&both, 5), vec![Claim { owner: "a".to_string(), slices: vec![one.slices[1]], rest: one.rest }, Claim { owner: "b".to_string(), slices: vec![two.slices[1]], rest: None }]);
    assert_eq!(pruned(&both, 7), vec![Claim { owner: "a".to_string(), slices: vec![one.slices[1]], rest: one.rest }]);
    assert_eq!(pruned(&both, 10), Vec::new());
}

#[test]
fn a_stride_is_cut_short_a_seam_before_the_first_obstacle_ahead_that_shares_its_height() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    assert_eq!(guarded_stride(extent, 5.0, &[]), 5.0);
    assert_eq!(guarded_stride(extent, -5.0, &[]), -5.0);
    assert_eq!(guarded_stride(extent, 0.0, &[rect(0.0, 0.0, 1000.0, 1000.0)]), 0.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(123.0, 260.0, 160.0, 300.0)]), 3.0 - SEAM);
    assert_eq!(guarded_stride(extent, 2.0, &[rect(123.0, 260.0, 160.0, 300.0)]), 2.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(120.01, 260.0, 160.0, 300.0)]), 0.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(40.0, 260.0, 77.0, 300.0)]), SEAM - 3.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(123.0, 260.0, 160.0, 300.0)]), -5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(40.0, 260.0, 77.0, 300.0)]), 5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(123.0, 100.0, 160.0, 250.0)]), 5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(123.0, 300.0, 160.0, 350.0)]), 5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(200.0, 260.0, 240.0, 300.0), rect(123.0, 260.0, 160.0, 300.0), rect(122.0, 100.0, 160.0, 200.0)]), 3.0 - SEAM);
}

#[test]
fn a_body_is_let_out_of_an_obstacle_it_overlaps_never_further_in() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let inside = [rect(110.0, 260.0, 150.0, 300.0)];
    assert_eq!(guarded_stride(extent, 5.0, &inside), 0.0);
    assert_eq!(guarded_stride(extent, -5.0, &inside), -5.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(60.0, 260.0, 100.0, 300.0)]), 5.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(60.0, 260.0, 100.0, 300.0)]), 0.0);
    assert_eq!(guarded_stride(extent, 5.0, &[rect(60.0, 260.0, 140.0, 300.0)]), 5.0);
    assert_eq!(guarded_stride(extent, -5.0, &[rect(60.0, 260.0, 140.0, 300.0)]), -5.0);
}

#[test]
fn a_stride_stops_before_the_corridors_and_the_rests_of_claims_as_before_bodies() {
    let extent = rect(80.0, 250.0, 120.0, 300.0);
    let claims = [Claim { owner: "c".to_string(), slices: vec![Slice { from: 5, until: 5, extent: rect(122.0, 0.0, 160.0, 300.0) }, Slice { from: 6, until: 9, extent: rect(124.0, 0.0, 160.0, 300.0) }], rest: Some(rect(126.0, 250.0, 160.0, 300.0)) }];
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("a", &[], &claims, 5)), 2.0 - SEAM);
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("a", &[], &claims, 6)), 4.0 - SEAM);
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("a", &[], &claims, 10)), 6.0 - SEAM);
    assert_eq!(guarded_stride(extent, 9.0, &obstacles_of("c", &[], &claims, 5)), 9.0);
}

#[test]
fn strides_agree_with_a_walk_along_a_lattice_that_stops_at_the_first_place_too_close() {
    strides_agree_with_the_lattice(40);
}

#[test]
fn walkers_on_one_line_stay_apart_and_in_their_order_which_nothing_but_the_guard_does() {
    walkers_kept_in_order(400);
}

#[test]
fn bodies_are_ranked_from_left_to_right_and_an_order_is_kept_or_not() {
    let bodies = [body("c", 50.0, 0.0, 60.0, 10.0), body("a", 0.0, 0.0, 10.0, 10.0), body("b", 20.0, 0.0, 40.0, 10.0), body("d", 25.0, 0.0, 35.0, 10.0)];
    assert_eq!(order_of(&bodies), slugs(&["a", "b", "d", "c"]));
    assert_eq!(order_of(&[]), Vec::<Slug>::new());
    assert!(order_kept(&slugs(&["a", "b", "c"]), &slugs(&["a", "b", "c"])));
    assert!(order_kept(&slugs(&["a", "b", "c"]), &slugs(&["a", "x", "c"])));
    assert!(!order_kept(&slugs(&["a", "b", "c"]), &slugs(&["c", "y", "a"])));
    assert!(!order_kept(&slugs(&["a", "b", "c"]), &slugs(&["b", "a"])));
    assert!(order_kept(&slugs(&["a", "b", "c"]), &[]));
    assert!(order_kept(&[], &slugs(&["a"])));
}

#[test]
fn a_swap_by_a_hop_needs_a_rise_that_lifts_the_hopper_over_its_neighbour() {
    let hopper = rect(80.0, 250.0, 120.0, 304.0);
    let hurdle = rect(130.0, 244.0, 170.0, 304.0);
    assert!(vaults(hopper, 60.0 + SEAM, hurdle));
    assert!(!vaults(hopper, 60.0, hurdle));
    assert!(!vaults(hopper, 84.0, rect(130.0, 219.0, 170.0, 304.0)));
    assert!(vaults(hopper, 84.0, rect(130.0, 221.0, 170.0, 304.0)));
}

#[test]
fn no_hop_over_a_neighbour_is_clear_whose_rise_does_not_vault_it() {
    let size = Size { width: 30.0, height: 40.0 };
    let from = feet(100.0, 300.0);
    let (mut vaulted, mut refused) = (0, 0);
    for dx in (60..=160).step_by(5).map(f64::from) {
        let to = feet(from.x + dx, 300.0);
        let Some(hop) = hop_of(from, to) else {
            continue;
        };
        let mut path = vec![body_of("h", from, size, 0.0, UPRIGHT, MARGIN).extent];
        let mut flight = Flight { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
        for left in (1..=hop.ticks).rev() {
            flight = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
            path.push(body_of("h", feet(flight.x, flight.y), size, 0.0, UPRIGHT, MARGIN).extent);
        }
        let plan = claim_of("h", 0, &path, 1, Some(path[path.len() - 1]));
        let rise = path[0].y1 - path.iter().map(|extent| extent.y1).fold(f64::INFINITY, f64::min);
        for height in (4..=60).step_by(4).map(f64::from) {
            for width in (4..=40).step_by(12).map(f64::from) {
                let hurdle = body_of("n", feet(from.x + dx / 2.0, 300.0), Size { width, height }, 0.0, UPRIGHT, MARGIN);
                if claim_clear(&plan, std::slice::from_ref(&hurdle), &[]) {
                    vaulted += 1;
                    assert!(vaults(path[0], rise, hurdle.extent), "dx {dx} height {height} width {width}");
                } else {
                    refused += 1;
                }
            }
        }
    }
    assert!(vaulted > 20);
    assert!(refused > 20);
}

/// 🪑️ The perch of the cases of seating.
fn seating_perch() -> Perch {
    Perch { surface: "s".to_string(), x0: 100.0, x1: 400.0, y: 300.0 }
}

/// 💺️ The boxes after a seating, in the order of its seats.
fn seated(bodies: &[Body], seating: &Seating) -> Vec<Body> {
    seating.seats.iter().map(|seat| Body { owner: seat.owner.clone(), extent: shifted(bodies.iter().find(|entry| entry.owner == seat.owner).expect("a body").extent, seat.shift, 0.0) }).collect()
}

/// 🪧️ A seat by its owner and shift.
fn seat(owner: &str, shift: f64) -> Seat {
    Seat { owner: owner.to_string(), shift }
}

#[test]
fn a_valid_seating_is_left_exactly_as_it_is() {
    let bodies = [body("b", 220.0, 248.0, 268.0, 304.0), body("a", 150.0, 248.0, 198.0, 304.0), body("c", 268.0 + SEAM, 248.0, 300.0, 304.0)];
    assert_eq!(seat_of(&bodies, &seating_perch(), MARGIN), Seating { seats: vec![seat("a", 0.0), seat("b", 0.0), seat("c", 0.0)], leavers: Vec::new() });
    assert_eq!(seat_of(&[], &seating_perch(), MARGIN), Seating::default());
}

#[test]
fn two_bodies_that_overlap_move_apart_by_the_same_amount_a_seam_between_them() {
    let seating = seat_of(&[body("a", 150.0, 248.0, 198.0, 304.0), body("b", 190.0, 248.0, 238.0, 304.0)], &seating_perch(), MARGIN);
    assert_eq!(seating, Seating { seats: vec![seat("a", -(4.0 + SEAM / 2.0)), seat("b", 4.0 + SEAM / 2.0)], leavers: Vec::new() });
}

#[test]
fn a_whole_run_of_violators_is_pooled_into_one_block() {
    let bodies = [body("a", 200.0, 248.0, 240.0, 304.0), body("b", 210.0, 248.0, 250.0, 304.0), body("c", 220.0, 248.0, 260.0, 304.0), body("d", 300.0, 248.0, 340.0, 304.0)];
    let after = seated(&bodies, &seat_of(&bodies, &seating_perch(), MARGIN));
    assert_eq!(after.iter().map(|entry| entry.extent.x0).collect::<Vec<_>>(), vec![170.0 - SEAM, 210.0, 250.0 + SEAM, 300.0]);
}

#[test]
fn bodies_are_brought_back_onto_the_perch_their_margin_beyond_its_ends() {
    assert_eq!(seat_of(&[body("a", 60.0, 248.0, 108.0, 304.0)], &seating_perch(), MARGIN).seats, vec![seat("a", 36.0)]);
    assert_eq!(seat_of(&[body("a", 380.0, 248.0, 428.0, 304.0)], &seating_perch(), MARGIN).seats, vec![seat("a", -24.0)]);
    let bodies = [body("a", 300.0, 248.0, 348.0, 304.0), body("b", 352.0, 248.0, 400.0, 304.0)];
    let short = Perch { surface: "s".to_string(), x0: 100.0, x1: 380.0, y: 300.0 };
    assert_eq!(seat_of(&bodies, &short, MARGIN).seats, vec![seat("a", -(12.0 + SEAM)), seat("b", -16.0)]);
    assert_eq!(seated(&bodies, &seat_of(&bodies, &short, MARGIN))[1].extent.x1, short.x1 + MARGIN);
}

#[test]
fn whoever_does_not_fit_is_named_for_leaving_the_last_in_the_list_first() {
    let narrow = Perch { surface: "s".to_string(), x0: 100.0, x1: 200.0, y: 300.0 };
    let bodies = [body("c", 180.0, 248.0, 228.0, 304.0), body("a", 90.0, 248.0, 138.0, 304.0), body("b", 130.0, 248.0, 178.0, 304.0)];
    let seating = seat_of(&bodies, &narrow, MARGIN);
    assert_eq!(seating.leavers, slugs(&["b"]));
    assert_eq!(seating.seats.iter().map(|seat| seat.owner.clone()).collect::<Vec<_>>(), slugs(&["a", "c"]));
    assert_eq!(overlaps(&seated(&bodies, &seating)), Vec::new());
    assert_eq!(seat_of(&[body("a", 90.0, 248.0, 210.0, 304.0)], &narrow, MARGIN), Seating { seats: Vec::new(), leavers: slugs(&["a"]) });
    assert_eq!(seat_of(&[body("a", 90.0, 248.0, 198.0, 304.0)], &narrow, MARGIN).leavers, Vec::<Slug>::new());
    let crowd = [body("a", 0.0, 248.0, 60.0, 304.0), body("b", 0.0, 248.0, 60.0, 304.0), body("c", 0.0, 248.0, 60.0, 304.0), body("d", 0.0, 248.0, 60.0, 304.0)];
    assert_eq!(seat_of(&crowd, &narrow, MARGIN).leavers, slugs(&["d", "c", "b"]));
}

#[test]
fn a_seating_is_the_nearest_valid_one_found_by_a_search_over_every_way_to_pool_neighbours() {
    seatings_agree_with_every_pooling(40);
}

#[test]
fn a_group_scoots_by_one_and_the_same_share_of_the_way_which_never_overlaps_within_the_group() {
    assert_eq!(scoot_fraction(&[3.0, -6.0, 0.0], 1.5), 0.25);
    assert_eq!(scoot_fraction(&[3.0, -6.0, 0.0], 10.0), 1.0);
    assert_eq!(scoot_fraction(&[3.0, -6.0, 0.0], 6.0), 1.0);
    assert_eq!(scoot_fraction(&[0.0, 0.0], 1.5), 1.0);
    assert_eq!(scoot_fraction(&[], 1.5), 1.0);
    assert_eq!(scoot_fraction(&[3.0], 0.0), 0.0);
    assert_eq!(scoot_fraction(&[3.0], -2.0), 0.0);
    assert_eq!(scoot_fraction(&[0.0], -2.0), 1.0);
    assert_eq!(scooted(10.0, 20.0, 0.25), 12.5);
    assert_eq!(scooted(10.0, 20.0, 0.0), 10.0);
    assert_eq!(scooted(10.0, 20.0, 1.0), 20.0);
    assert_eq!(scooted(10.0, 20.0, 1.5), 20.0);
    scoots_stay_apart(40);
}

/// 🎩️ The host of the cases of heads.
fn host() -> Body {
    body("h", 100.0, 240.0, 150.0, 304.0)
}

#[test]
fn a_descending_body_lands_on_the_top_it_comes_down_onto_like_a_perch_crossing() {
    let host = host();
    let hosts = std::slice::from_ref(&host);
    let before = rect(90.0, 180.0, 130.0, 236.0);
    let after = rect(90.0, 190.0, 130.0, 246.0);
    assert_eq!(head_under(before, after, "f", hosts), Some(&host));
    assert_eq!(lift_onto(after, host.extent), -(6.0 + SEAM));
    assert_eq!(shifted(after, 0.0, lift_onto(after, host.extent)).y1, 240.0 - SEAM);
    assert_eq!(head_under(before, after, "h", hosts), None);
    assert_eq!(head_under(shifted(before, -50.0, 0.0), shifted(after, -50.0, 0.0), "f", hosts), None);
    assert_eq!(head_under(shifted(before, -30.0, 0.0), shifted(after, -29.75, 0.0), "f", hosts), Some(&host));
    assert_eq!(head_under(shifted(before, -30.0, 0.0), shifted(after, -30.0, 0.0), "f", hosts), None);
    assert_eq!(head_under(after, before, "f", hosts), None);
    assert_eq!(head_under(shifted(before, 0.0, 14.0), shifted(after, 0.0, 14.0), "f", hosts), None);
    assert_eq!(head_under(before, shifted(before, 0.0, 3.0), "f", hosts), None);
    assert_eq!(head_under(before, shifted(before, 0.0, 4.0), "f", hosts), Some(&host));
    assert_eq!(head_under(shifted(before, 0.0, 4.0), shifted(before, 0.0, 4.25), "f", hosts), Some(&host));
    assert_eq!(head_under(before, after, "f", &[]), None);
}

#[test]
fn the_highest_head_is_chosen_and_the_first_one_among_equals() {
    let before = rect(90.0, 100.0, 180.0, 200.0);
    let after = rect(90.0, 160.0, 180.0, 260.0);
    let taller = body("t", 150.0, 220.0, 200.0, 304.0);
    let twin = body("w", 60.0, 220.0, 95.0, 304.0);
    assert_eq!(head_under(before, after, "f", &[host(), taller.clone()]), Some(&taller));
    assert_eq!(head_under(before, after, "f", &[taller.clone(), host()]), Some(&taller));
    assert_eq!(head_under(before, after, "f", &[host(), twin.clone(), taller]), Some(&twin));
}

#[test]
fn a_rider_still_stands_on_its_host_or_not() {
    let host = host().extent;
    let rider = rect(90.0, 180.0, 130.0, 240.0 - SEAM);
    assert!(rests_on(rider, host));
    assert!(rests_on(shifted(rider, 0.0, SEAM), host));
    assert!(rests_on(shifted(rider, 0.0, -SEAM), host));
    assert!(!rests_on(shifted(rider, 0.0, -2.0 * SEAM), host));
    assert!(!rests_on(shifted(rider, 0.0, 2.0 * SEAM), host));
    assert!(!rests_on(shifted(rider, -30.0, 0.0), host));
    assert!(rests_on(shifted(rider, -29.75, 0.0), host));
    assert!(!rests_on(shifted(rider, 60.0, 0.0), host));
}

#[test]
fn a_rider_slides_off_to_the_nearer_side_faster_and_faster_up_to_a_limit() {
    let host = host().extent;
    assert_eq!(slide_side(rect(110.0, 0.0, 150.0, 10.0), host), Facing::Right);
    assert_eq!(slide_side(rect(105.0, 0.0, 145.0, 10.0), host), Facing::Right);
    assert_eq!(slide_side(rect(104.0, 0.0, 145.0, 10.0), host), Facing::Left);
    assert_eq!(slide_stride(0), 0.5);
    assert_eq!(slide_stride(10), 0.8125);
    assert_eq!(slide_stride(48), 2.0);
    assert_eq!(slide_stride(1000), 2.0);
    for tick in 1..100 {
        assert!(slide_stride(tick) >= slide_stride(tick - 1));
    }
}

#[test]
fn a_slide_is_guarded_kept_on_the_stage_and_turned_round_where_it_cannot_move() {
    let rider = rect(90.0, 180.0, 130.0, 240.0 - SEAM);
    let right = Facing::Right;
    let left = Facing::Left;
    assert_eq!(slide_of(rider, right, 0, 0.0, 960.0, &[host().extent]), Slide { stride: 0.5, side: right });
    assert_eq!(slide_of(rider, left, 10, 0.0, 960.0, &[host().extent]), Slide { stride: -0.8125, side: left });
    assert_eq!(slide_of(rider, right, 0, 0.0, 960.0, &[rect(130.25, 100.0, 200.0, 235.0)]), Slide { stride: 0.25 - SEAM, side: right });
    assert_eq!(slide_of(rider, right, 0, 0.0, 960.0, &[rect(130.0 + SEAM, 100.0, 200.0, 235.0)]), Slide { stride: 0.0, side: left });
    assert_eq!(slide_of(rider, left, 0, 0.0, 960.0, &[rect(40.0, 100.0, 90.0, 235.0)]), Slide { stride: 0.0, side: right });
    assert_eq!(slide_of(rider, right, 0, 0.0, 130.0, &[]), Slide { stride: 0.0, side: left });
    assert_eq!(slide_of(rider, right, 0, 0.0, 130.25, &[]), Slide { stride: 0.25, side: right });
    assert_eq!(slide_of(rider, left, 0, 90.0, 960.0, &[]), Slide { stride: 0.0, side: right });
    assert_eq!(slide_of(rider, left, 0, 89.75, 960.0, &[]), Slide { stride: -0.25, side: left });
}

#[test]
fn a_held_body_takes_the_shortest_way_out_of_an_obstacle_a_seam_to_spare() {
    let extent = rect(100.0, 100.0, 140.0, 150.0);
    let at = |x: f64, y: f64| Some(Point { x, y });
    assert_eq!(pushed_out(extent, &[], PUSHES), at(0.0, 0.0));
    assert_eq!(pushed_out(extent, &[rect(140.0, 100.0, 200.0, 150.0), rect(0.0, 150.0, 300.0, 200.0)], PUSHES), at(0.0, 0.0));
    assert_eq!(pushed_out(extent, &[rect(130.0, 90.0, 200.0, 160.0)], PUSHES), at(-(10.0 + SEAM), 0.0));
    assert_eq!(pushed_out(extent, &[rect(40.0, 90.0, 108.0, 160.0)], PUSHES), at(8.0 + SEAM, 0.0));
    assert_eq!(pushed_out(extent, &[rect(90.0, 140.0, 150.0, 200.0)], PUSHES), at(0.0, -(10.0 + SEAM)));
    assert_eq!(pushed_out(extent, &[rect(90.0, 60.0, 150.0, 105.0)], PUSHES), at(0.0, 5.0 + SEAM));
    assert_eq!(pushed_out(extent, &[rect(110.0, 0.0, 130.0, 300.0)], PUSHES), at(-(30.0 + SEAM), 0.0));
    assert_eq!(pushed_out(extent, &[rect(0.0, 115.0, 300.0, 135.0)], PUSHES), at(0.0, -(35.0 + SEAM)));
}

#[test]
fn a_held_body_goes_on_from_obstacle_to_obstacle_and_gives_up_when_its_rounds_are_over() {
    let extent = rect(100.0, 100.0, 140.0, 150.0);
    let pair = [rect(130.0, 90.0, 200.0, 160.0), rect(80.0, 60.0, 125.0, 105.0)];
    let freed = Some(Point { x: -(10.0 + SEAM), y: 5.0 + SEAM });
    assert_eq!(pushed_out(extent, &pair, PUSHES), freed);
    assert_eq!(pushed_out(extent, &pair, 1), freed);
    assert_eq!(pushed_out(extent, &[pair[1], pair[0]], 1), freed);
    assert_eq!(pushed_out(extent, &[rect(130.0, 90.0, 200.0, 160.0), rect(60.0, 90.0, 95.0, 160.0)], PUSHES), None);
    let walls = [rect(0.0, 0.0, 105.0, 300.0), rect(135.0, 0.0, 300.0, 300.0)];
    assert_eq!(pushed_out(extent, &walls, PUSHES), None);
    assert_eq!(pushed_out(extent, &walls, 100), None);
    assert_eq!(pushed_out(extent, &[rect(130.0, 90.0, 200.0, 160.0)], 0), None);
    assert_eq!(pushed_out(extent, &[rect(140.0, 90.0, 200.0, 160.0)], 0), Some(Point { x: 0.0, y: 0.0 }));
}

#[test]
fn a_push_answers_a_free_place_or_nothing_on_generated_crowds() {
    pushes_free_or_nothing(40);
}

/// 🧯️ The bodies and plans of the cases of the last resort.
fn resort_scene() -> ([Body; 2], Claim, Claim) {
    let bodies = [body("a", 0.0, 0.0, 40.0, 50.0), body("b", 100.0, 0.0, 140.0, 50.0)];
    let clear = claim_of("a", 0, &marching(0.0, 0.0, 0.0, 10.0, 10), 1, Some(rect(0.0, 90.0, 40.0, 140.0)));
    let blocked = claim_of("a", 0, &marching(0.0, 0.0, 10.0, 0.0, 10), 1, Some(rect(90.0, 0.0, 130.0, 50.0)));
    (bodies, clear, blocked)
}

#[test]
fn a_pet_may_be_as_long_as_it_may_stay_or_one_of_its_plans_is_clear() {
    let (bodies, clear, blocked) = resort_scene();
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[], &bodies, &[], 0));
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), std::slice::from_ref(&blocked), &bodies, &[], 0));
    assert!(!must_poof("a", None, &[blocked.clone(), clear.clone()], &bodies, &[], 0));
    assert!(!must_poof("a", Some(rect(70.0, 0.0, 110.0, 50.0)), &[blocked, clear], &bodies, &[], 0));
}

#[test]
fn a_pet_that_may_not_stay_and_has_no_clear_plan_poofs() {
    let (bodies, clear, blocked) = resort_scene();
    assert!(must_poof("a", None, &[], &bodies, &[], 0));
    assert!(must_poof("a", None, std::slice::from_ref(&blocked), &bodies, &[], 0));
    assert!(must_poof("a", Some(rect(70.0, 0.0, 110.0, 50.0)), std::slice::from_ref(&blocked), &bodies, &[], 0));
    let corridor = [claim_of("c", 5, &marching(0.0, -100.0, 0.0, 20.0, 6), 1, None)];
    assert!(must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[], &bodies, &corridor, 5));
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[], &bodies, &corridor, 11));
    assert!(must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), std::slice::from_ref(&blocked), &bodies, &corridor, 5));
    assert!(!must_poof("a", Some(rect(0.0, 0.0, 40.0, 50.0)), &[blocked, clear], &bodies, &corridor, 5));
}

#[test]
fn whoever_a_change_from_outside_carried_into_somebody_who_stays_is_evicted() {
    let (bodies, _, _) = resort_scene();
    let with = |extra: &[Body]| bodies.iter().cloned().chain(extra.iter().cloned()).collect::<Vec<_>>();
    assert_eq!(evicted(&bodies, &[], 0), Vec::<Slug>::new());
    assert_eq!(evicted(&with(&[body("c", 30.0, 0.0, 70.0, 50.0)]), &[], 0), slugs(&["c"]));
    assert_eq!(evicted(&[body("c", 30.0, 0.0, 70.0, 50.0), bodies[0].clone(), bodies[1].clone()], &[], 0), slugs(&["a"]));
    assert_eq!(evicted(&with(&[body("c", 30.0, 0.0, 70.0, 50.0), body("d", 60.0, 0.0, 90.0, 50.0)]), &[], 0), slugs(&["c"]));
    assert_eq!(evicted(&with(&[body("c", 30.0, 0.0, 110.0, 50.0), body("d", 60.0, 0.0, 90.0, 50.0)]), &[], 0), slugs(&["c"]));
    assert_eq!(evicted(&with(&[body("d", 60.0, 0.0, 90.0, 50.0), body("c", 30.0, 0.0, 110.0, 50.0)]), &[], 0), slugs(&["c"]));
}

#[test]
fn a_body_without_a_plan_is_evicted_from_the_corridors_and_rests_of_those_who_stay() {
    let (bodies, _, _) = resort_scene();
    let plan = claim_of("b", 0, &marching(100.0, 0.0, 20.0, 0.0, 10), 1, Some(rect(300.0, 0.0, 340.0, 50.0)));
    let rider = body("c", 200.0, 0.0, 240.0, 50.0);
    let with = |extra: Body| bodies.iter().cloned().chain([extra]).collect::<Vec<_>>();
    assert_eq!(evicted(&with(rider.clone()), std::slice::from_ref(&plan), 0), slugs(&["c"]));
    assert_eq!(evicted(&with(rider.clone()), std::slice::from_ref(&plan), 8), Vec::<Slug>::new());
    assert_eq!(evicted(&with(body("c", 310.0, 0.0, 350.0, 50.0)), std::slice::from_ref(&plan), 100), slugs(&["c"]));
    assert_eq!(evicted(&with(rider.clone()), &[plan.clone(), claim_of("c", 0, &marching(200.0, 0.0, 0.0, 0.0, 3), 1, None)], 0), Vec::<Slug>::new());
    assert_eq!(evicted(&[body("a", 0.0, 0.0, 40.0, 50.0), body("b", 30.0, 0.0, 70.0, 50.0), rider], &[plan], 0), slugs(&["b"]));
}

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
