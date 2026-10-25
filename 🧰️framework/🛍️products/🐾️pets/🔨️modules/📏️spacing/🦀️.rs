//! 📏️ The spacing of grounded actors: actors of one surface never stand in each other. Where an actor can stand a comfortable gap away from everybody else (`rooms_for`, `quarters`), how far it can walk (`clearway`) and whether the arc of a hop keeps clear of what must stay free (`soars`) — with the gaps the whole stage keeps: `COMFORT_GAP`, `MEET_GAP`.
//!
//! Everything here measures along one surface: feet on the same perch, distances in x.
//! A part of the stage, not of the package: the items the parts of the stage in `pets-staging` call are `pub`, the rest stays private to this crate.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../📏️spacing/🟦️.ts — the TypeScript twin

use crate::draft::{carve, crowd_on, shoulders, Draft, Room};
use semio_framework_pets_clearance::Body;
use semio_framework_pets_contract::schema::{Activity, Cast, Menagerie, Perch, Point, Rect, Slug, Species, Surface, Ticks};
use semio_framework_pets_kinematics::terrain::{hop_step, larger, perch_at, perches_of, smaller, Flight, Hop};
use semio_framework_pets_kinematics::trigonometry::clamp;
use semio_framework_pets_contract::schema::Footing;
use semio_framework_pets_contract::schema::Actor;
use semio_framework_pets_contract::schema::Extent;
use crate::draft::hover_of;
use semio_framework_pets_clearance::{MARGIN, SEAM, UPRIGHT, body_of, obstacles_of, united};
use semio_framework_pets_kinematics::trigonometry::{cos_turns, sin_turns};
use semio_framework_pets_traversal::climbing::cling_of;
use semio_framework_pets_traversal::swing::CHUTE_ROD;

//#region 🔖️Constants
pub const MEET_GAP: f64 = 6.0;
pub const COMFORT_GAP: f64 = 8.0;
//#endregion 🔖️Constants

/// 🦎️ The body's lean towards its wall, given up as its feet leave the line they cling to.
pub fn wall_lean_of(kind: &Species, footing: Footing, pitch: Option<&semio_framework_pets_contract::schema::Pitch>, x: f64) -> f64 {
    let Some(pitch) = pitch.filter(|_| footing == Footing::Wall) else { return 0.0; };
    let share = clamp(1.0 - (x - cling_of(pitch, kind.size)).abs() / (kind.size.width / 2.0), 0.0, 1.0);
    (if pitch.side.sign() > 0.0 { -1.0 } else { 1.0 }) * 0.04 * share
}

/// 🙃️ The turned size box, hover and margin about the scruff.
fn turned(feet: Point, sine: f64, cosine: f64, kind: &Species) -> Extent {
    let grip = kind.grip;
    let half = kind.size.width / 2.0;
    let pivot_x = feet.x + grip * sine;
    let pivot_y = feet.y - grip * cosine;
    let top = grip - kind.size.height;
    let bottom = grip + hover_of(kind);
    let across = half * cosine.abs();
    let down = half * sine.abs();
    let left_top = 0.0 - sine * top;
    let left_bottom = 0.0 - sine * bottom;
    let high_top = cosine * top;
    let high_bottom = cosine * bottom;
    Extent { x0: pivot_x + smaller(left_top, left_bottom) - across - MARGIN, y0: pivot_y + smaller(high_top, high_bottom) - down - MARGIN, x1: pivot_x + larger(left_top, left_bottom) + across + MARGIN, y1: pivot_y + larger(high_top, high_bottom) + down + MARGIN }
}

/// ☂️ The canopy's box, leaning with its cords above the scruff.
fn canopy_extent(feet: Point, canopy: Point, kind: &Species) -> Extent {
    let height = kind.size.height;
    let cords = CHUTE_ROD * height;
    let sine = (canopy.x - feet.x) / cords;
    let cosine = (feet.y - canopy.y) / cords;
    let pivot_y = feet.y - kind.grip;
    let span = 0.75 * kind.size.width;
    let rim = 0.0 - 0.7 * height;
    let dome = rim - 0.5625 * height;
    let across = span * cosine.abs();
    let down = span * sine.abs();
    let left_rim = 0.0 - sine * rim;
    let left_dome = 0.0 - sine * dome;
    let high_rim = cosine * rim;
    let high_dome = cosine * dome;
    Extent { x0: feet.x + smaller(left_rim, left_dome) - across - MARGIN, y0: pivot_y + smaller(high_rim, high_dome) - down - MARGIN, x1: feet.x + larger(left_rim, left_dome) + across + MARGIN, y1: pivot_y + larger(high_rim, high_dome) + down + MARGIN }
}

/// 📐️ The body at its feet, tilt and optional canopy, including the drawing's complete extent.
pub fn extent_at(owner: &str, kind: &Species, feet: Point, tilt: f64, canopy: Option<Point>) -> Extent {
    let body = if tilt == 0.0 { body_of(owner, feet, kind.size, hover_of(kind), UPRIGHT, MARGIN).extent } else { turned(feet, sin_turns(tilt), cos_turns(tilt), kind) };
    canopy.map_or(body, |canopy| united(body, canopy_extent(feet, canopy, kind)))
}

/// 🧸️ The actual actor's body, wall lean and canopy.
pub fn extent_of(actor: &Actor, kind: &Species) -> Extent {
    extent_at(&actor.species, kind, Point { x: actor.x, y: actor.y }, actor.tilt + wall_lean_of(kind, actor.footing, actor.pitch.as_ref(), actor.x), actor.chute.as_ref().map(|chute| Point { x: chute.canopy.x, y: chute.canopy.y }))
}

/// 👪️ Every actor's body in stage order.
pub fn bodies_of(actors: &[Actor], kinds: &[&Species]) -> Vec<Body> {
    actors.iter().zip(kinds).map(|(actor, kind)| Body { owner: actor.species.clone(), extent: extent_of(actor, kind) }).collect()
}

/// 🧱️ Every other body, claimed corridor and planned resting place at this tick.
pub(crate) fn obstacles_for(draft: &Draft<'_>, index: usize, now: Ticks) -> Vec<Extent> {
    obstacles_of(&draft.stage.actors[index].species, &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims, now)
}

//#region 🔖️Spacing
/// 🪂️ Whether the arc of a hop keeps clear of what must stay free: on every tick on which the feet are above both ends of the hop (lower down the body is beside the things it hops between), the box of the body touches no keep-out and stays below the top of the stage.
pub(crate) fn soars(draft: &Draft<'_>, kind: &Species, from: Point, to: Point, hop: Hop) -> bool {
    let half = kind.size.width / 2.0;
    let ridge = smaller(from.y, to.y);
    let mut flight = Flight { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
    for left in (2..=hop.ticks).rev() {
        flight = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
        if flight.y >= ridge {
            continue;
        }
        let top = flight.y - kind.size.height;
        if top < 0.0 {
            return false;
        }
        for keepout in &draft.stage.keepouts {
            if keepout.width > 0.0 && keepout.height > 0.0 && larger(flight.x - half, keepout.x) < smaller(flight.x + half, keepout.x + keepout.width) && larger(top, keepout.y) < smaller(flight.y, keepout.y + keepout.height) {
                return false;
            }
        }
    }
    true
}

/// 🛣️ The stretch `[low, high]` of its perch an actor can walk on without its body coming closer than `gap` to anybody else on that surface (`except` aside; a walker counts for the whole way it still has to go). The stretch always holds the place where the actor stands, and is that place alone for an actor without a perch.
pub fn clearway(draft: &Draft<'_>, index: usize, gap: f64, except: Option<usize>) -> [f64; 2] {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let Some(perch) = (body.footing == Footing::Perch).then_some(body.perch.as_deref()).flatten().and_then(|surface| perch_at(&draft.stage.perches, surface, body.x)) else {
        return [body.x, body.x];
    };
    let mut low = perch.x0 + kind.size.width / 2.0;
    let mut high = perch.x1 - kind.size.width / 2.0;
    for (other, neighbour) in draft.stage.actors.iter().enumerate() {
        if other == index || Some(other) == except || neighbour.footing != Footing::Perch || neighbour.perch != body.perch {
            continue;
        }
        let room = shoulders(draft.kinds[other], kind) + gap + SEAM;
        let walks = matches!(neighbour.activity, Activity::Walk | Activity::Scoot);
        let left = if walks && neighbour.goal < neighbour.x { neighbour.goal } else { neighbour.x };
        let right = if walks && neighbour.goal > neighbour.x { neighbour.goal } else { neighbour.x };
        if right <= body.x {
            low = larger(low, right + room);
        } else if left >= body.x {
            high = smaller(high, left - room);
        } else {
            low = body.x;
            high = body.x;
        }
    }
    [smaller(low, body.x), larger(high, body.x)]
}

/// 🛏️ Where a species could arrive: per perch that carries it the stretches it can stand on a comfortable gap away from the actors already there (half of both widths plus 8 px), and how many stand there. A perch without such a stretch is full.
pub fn rooms_for(draft: &Draft<'_>, kind: &Species) -> Vec<Room> {
    let mut rooms = Vec::new();
    let half = kind.size.width / 2.0;
    let hover = hover_of(kind);
    let obstacles = obstacles_of(&kind.id, &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims, draft.stage.tick);
    for (at, perch) in draft.stage.perches.iter().enumerate() {
        if perch.x1 - half < perch.x0 + half {
            continue;
        }
        let mut stretches = vec![[perch.x0 + half, perch.x1 - half]];
        let box_at = body_of(&kind.id, Point { x: 0.0, y: perch.y - hover }, kind.size, hover, UPRIGHT, MARGIN).extent;
        for obstacle in &obstacles {
            if !(box_at.y0 < obstacle.y1 && obstacle.y0 < box_at.y1) { continue; }
            stretches = carve(&stretches, obstacle.x0 - box_at.x1 - SEAM, obstacle.x1 - box_at.x0 + SEAM);
        }
        if stretches.is_empty() {
            continue;
        }
        let mut span = 0.0;
        for &[x0, x1] in &stretches {
            span += x1 - x0;
        }
        rooms.push(Room { perch: at, stretches, span, crowd: crowd_on(draft, perch) });
    }
    rooms
}

/// 🌇️ The ground a survey offers: what the perches of a company are cut from.
#[derive(Clone, Copy)]
pub struct Ground<'a> {
    pub surfaces: &'a [Surface],
    pub keepouts: &'a [Rect],
    pub width: f64,
    pub height: f64,
}

/// 🏙️ The perches of a company on `ground`: clearance = the tallest of them plus its hover, minimum = the widest, so that every perch carries any of them; for nobody, the perches of nobody (every surface).
pub fn perches_for(ground: Ground<'_>, company: &[&Species]) -> Vec<Perch> {
    let mut tallest = 0.0;
    let mut widest = 0.0;
    for kind in company {
        tallest = larger(tallest, kind.size.height + hover_of(kind));
        widest = larger(widest, kind.size.width);
    }
    perches_of(ground.surfaces, ground.keepouts, ground.width, ground.height, tallest, widest)
}

/// 🗿️ Who of `species` (in the order given, a species the menagerie does not know left out) can stand on `ground` together: each one in its turn joins when the perches cut for it and everyone who joined before it still exist. Whoever does not fit is left out and the others do not wait for it, so one species too wide or too tall for every ground never takes the ground from those that fit; the first of them that fits alone always joins.
pub fn standing<'m>(menagerie: &'m Menagerie, ground: Ground<'_>, species: &[Slug]) -> Vec<&'m Species> {
    let mut company: Vec<&Species> = Vec::new();
    for id in species {
        let Some(kind) = menagerie.species.iter().find(|known| &known.id == id) else { continue; };
        if company.iter().any(|joined| joined.id == kind.id) {
            continue;
        }
        company.push(kind);
        if perches_for(ground, &company).is_empty() {
            company.pop();
        }
    }
    company
}

/// 🎑️ The company of a stage: who stands on it already (its actors, in their order) and who is wanted (in `menagerie.species` order), as far as they can stand on its ground together ([`standing`]). Its perches are cut for them, and of the wanted only they arrive.
pub fn company_of<'m>(menagerie: &'m Menagerie, draft: &Draft<'_>) -> Vec<&'m Species> {
    let mut names: Vec<Slug> = draft.stage.actors.iter().map(|actor| actor.species.clone()).collect();
    for kind in &menagerie.species {
        if draft.stage.wanted.contains(&kind.id) && !names.contains(&kind.id) {
            names.push(kind.id.clone());
        }
    }
    standing(menagerie, Ground { surfaces: &draft.stage.surfaces, keepouts: &draft.stage.keepouts, width: draft.stage.width, height: draft.stage.height }, &names)
}

/// 🏰️ The troupe a scene casts, as far as `ground` carries it: of those who stood on the stage the turn before (`holding`) and of the `troupe` (in this order, as many as the troupe has seats), whoever stands on the ground even alone stays; every other seat goes to a species of the scene's core and rotation (in this order, once) that does, the `epoch`th of them first and the next ones round and round. A species to `avoid` (one the host has found does not stay on the ground it is shown on: its own presence takes the room it stood on) does not fit. A troupe that stands on the ground is cast as it is, and whoever held a seat keeps it while it fits, so a ground that changes under the pets recasts nobody who can stay and a page whose layout follows the pets cannot make the cast swing back and forth; when none of the scene's species can stand, nobody is cast.
pub fn cast_on(menagerie: &Menagerie, ground: Ground<'_>, cast: &Cast, troupe: &[Slug], epoch: i64, holding: &[Slug], avoid: &[Slug]) -> Vec<Slug> {
    let fits = |id: &Slug| !avoid.contains(id) && !standing(menagerie, ground, std::slice::from_ref(id)).is_empty();
    let mut kept: Vec<Slug> = Vec::new();
    for id in holding.iter().chain(troupe) {
        if kept.len() < troupe.len() && !kept.contains(id) && fits(id) {
            kept.push(id.clone());
        }
    }
    if kept.len() == troupe.len() {
        return kept;
    }
    let mut spare: Vec<Slug> = Vec::new();
    for id in cast.core.iter().chain(&cast.rotation) {
        if !spare.contains(id) && !kept.contains(id) && !troupe.contains(id) && fits(id) {
            spare.push(id.clone());
        }
    }
    let first = if spare.is_empty() { 0 } else { epoch.rem_euclid(spare.len() as i64) as usize };
    let missing = troupe.len() - kept.len();
    let open = if missing < spare.len() { missing } else { spare.len() };
    for seat in 0..open {
        kept.push(spare[(first + seat) % spare.len()].clone());
    }
    kept
}

/// 🏘️ The rooms a newcomer chooses among, so that a company spreads out over what the stage offers: those that hold the fewest actors, and of these the ones on the ground — the lowest perches of the stage — only when no higher one is as empty. Pets stand on things before they stand beneath them.
pub fn quarters(draft: &Draft<'_>, rooms: Vec<Room>) -> Vec<Room> {
    let mut fewest = usize::MAX;
    for room in &rooms {
        if room.crowd < fewest {
            fewest = room.crowd;
        }
    }
    let mut ground = f64::NEG_INFINITY;
    for perch in &draft.stage.perches {
        if perch.y > ground {
            ground = perch.y;
        }
    }
    let raised = |room: &Room| draft.stage.perches[room.perch].y < ground;
    let mut emptiest: Vec<Room> = rooms.into_iter().filter(|room| room.crowd == fewest).collect();
    if emptiest.iter().any(raised) {
        emptiest.retain(raised);
    }
    emptiest
}
//#endregion 🔖️Spacing
