//! 🌍️ The reference world of the clearance unit suite: the rules of the module, and nothing else, keep six to ten pets apart while they walk, hop, glide, fall, land on heads, are dragged and thrown by a hand and lose, shrink and move their perches.
//!
//! @see ../../🦀️.rs — the twin under test
//! @see ../../../🏞️terrain/🦀️.rs — the perches, falls and hops the world moves by
//! @see ../../../🎲️randomness/🦀️.rs — the draws of the world

use super::*;

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
