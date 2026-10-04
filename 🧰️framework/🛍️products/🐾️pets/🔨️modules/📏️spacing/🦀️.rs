//! 📏️ The spacing of grounded actors: actors of one surface never stand in each other. Where an actor can stand a comfortable gap away from everybody else (`vacancy`, `rooms_for`, `quarters`), how far it can walk (`clearway`), whether somebody is in the way of its next strides (`hindered`) and whether the arc of a hop keeps clear of what must stay free (`soars`) — with the gaps the whole stage keeps: `COMFORT_GAP`, `MEET_GAP` and `CONTACT`.
//!
//! Everything here measures along one surface: feet on the same perch, distances in x.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../📏️spacing/🟦️.ts — the TypeScript twin

use crate::draft::{carve, crowd_on, shoulders, Draft, Room, RATE};
use crate::schema::{Activity, Perch, Point, Species, Ticks};
use crate::terrain::{hop_step, larger, perch_at, smaller, Flight, Hop};
use crate::trigonometry::clamp;

//#region 🔖️Constants
pub(crate) const MEET_GAP: f64 = 6.0;
pub(crate) const COMFORT_GAP: f64 = 8.0;
const CONTACT: f64 = 0.0078125;
//#endregion 🔖️Constants

//#region 🔖️Spacing
/// 🪑️ The place nearest to `x` on a perch where an actor stands a comfortable gap (8 px between the bodies) away from everybody else on that surface, or `None` when the perch has no such place.
pub(crate) fn vacancy(draft: &Draft<'_>, index: usize, perch: &Perch, x: f64) -> Option<f64> {
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    if perch.x1 - half < perch.x0 + half {
        return None;
    }
    let mut stretches = vec![[perch.x0 + half, perch.x1 - half]];
    for (other, neighbour) in draft.stage.actors.iter().enumerate() {
        if other == index || neighbour.perch.as_deref() != Some(perch.surface.as_str()) {
            continue;
        }
        let room = shoulders(draft.kinds[other], kind) + COMFORT_GAP;
        stretches = carve(&stretches, neighbour.x - room, neighbour.x + room);
    }
    let mut nearest = None;
    let mut least = f64::INFINITY;
    for &[low, high] in &stretches {
        let place = clamp(x, low, high);
        let distance = (place - x).abs();
        if distance < least {
            nearest = Some(place);
            least = distance;
        }
    }
    nearest
}

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
pub(crate) fn clearway(draft: &Draft<'_>, index: usize, gap: f64, except: Option<usize>) -> [f64; 2] {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let Some(perch) = body.perch.as_deref().and_then(|surface| perch_at(&draft.stage.perches, surface, body.x)) else {
        return [body.x, body.x];
    };
    let mut low = perch.x0 + kind.size.width / 2.0;
    let mut high = perch.x1 - kind.size.width / 2.0;
    for (other, neighbour) in draft.stage.actors.iter().enumerate() {
        if other == index || Some(other) == except || neighbour.perch != body.perch {
            continue;
        }
        let room = shoulders(draft.kinds[other], kind) + gap;
        let walks = neighbour.activity == Activity::Walk;
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

/// 🚧️ Whether somebody stands in the way of the next `ticks` strides of a walker: another actor on its surface, ahead of it, whose body its own would come closer to than 6 px (bodies that are that close already count as touching within 1/128 px).
pub(crate) fn hindered(draft: &Draft<'_>, index: usize, ticks: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let way = if body.goal > body.x { 1.0 } else { -1.0 };
    let far = (larger(kind.locomotion.speed, 0.0) * ticks as f64) / RATE;
    let ahead = clamp(body.goal, body.x - far, body.x + far);
    for (other, neighbour) in draft.stage.actors.iter().enumerate() {
        if other == index || neighbour.perch != body.perch || (neighbour.x - body.x) * way <= 0.0 {
            continue;
        }
        if (neighbour.x - ahead) * way < shoulders(draft.kinds[other], kind) + MEET_GAP - CONTACT {
            return true;
        }
    }
    false
}

/// 🛏️ Where a species could arrive: per perch that carries it the stretches it can stand on a comfortable gap away from the actors already there (half of both widths plus 8 px), and how many stand there. A perch without such a stretch is full.
pub(crate) fn rooms_for(draft: &Draft<'_>, kind: &Species) -> Vec<Room> {
    let mut rooms = Vec::new();
    let half = kind.size.width / 2.0;
    for (at, perch) in draft.stage.perches.iter().enumerate() {
        if perch.x1 - half < perch.x0 + half {
            continue;
        }
        let mut stretches = vec![[perch.x0 + half, perch.x1 - half]];
        for (other, neighbour) in draft.stage.actors.iter().enumerate() {
            if neighbour.perch.as_deref() != Some(perch.surface.as_str()) {
                continue;
            }
            let gap = shoulders(draft.kinds[other], kind) + COMFORT_GAP;
            stretches = carve(&stretches, neighbour.x - gap, neighbour.x + gap);
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

/// 🏘️ The rooms a newcomer chooses among, so that a company spreads out over what the stage offers: those that hold the fewest actors, and of these the ones on the ground — the lowest perches of the stage — only when no higher one is as empty. Pets stand on things before they stand beneath them.
pub(crate) fn quarters(draft: &Draft<'_>, rooms: Vec<Room>) -> Vec<Room> {
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
