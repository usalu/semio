//! 👥️ Population: who is on stage and where the ground is — the survey (perches cut anew, grounded actors riding their surfaces and seated apart again), arrivals and spreading out over new ground, the summons, and the liveliness (freezing and thawing).
//!
//! What a survey brings and what it does to the company belongs here.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../👥️population/🟦️.ts — the TypeScript twin

use crate::behavior::{dwell_of, needs_of, MODE_LIMITS};
use crate::draft::{actor_key, blink_at, clip_at, crowd_on, hover_of, index_of, release, remove, settle, shift, shoulders, stage_key, surface_of, Draft};
use crate::feeling::at_rest;
use crate::gesture::{no_hover, COLD};
use crate::locomotion::{crowd_out, drop, leave, vanish};
use crate::randomness::{random_words, unit_of};
use crate::schema::{Activity, Actor, Facing, Footing, Gaze, Menagerie, Perch, PetMode, Slug, Species, Surface, Surveyed, Ticks};
use crate::spacing::{quarters, rooms_for, COMFORT_GAP};
use crate::terrain::{landing_of, larger, perches_of, smaller};
use crate::trigonometry::clamp;

//#region 🔖️Constants
pub(crate) const CENTRED: Gaze = Gaze { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0 };
const GRIP_BUDGET: f64 = 384.0;
//#endregion 🔖️Constants

//#region 🔖️Riding
/// 📐️ The perches of the stage for everyone who is on it or wanted: clearance = the tallest of them plus its hover, minimum = the widest, so that every perch carries any of them.
fn measure(menagerie: &Menagerie, draft: &mut Draft<'_>) {
    let mut tallest = 0.0;
    let mut widest = 0.0;
    for kind in &menagerie.species {
        if !draft.stage.wanted.contains(&kind.id) && index_of(&draft.stage.actors, &kind.id).is_none() {
            continue;
        }
        tallest = larger(tallest, kind.size.height + hover_of(kind));
        widest = larger(widest, kind.size.width);
    }
    draft.stage.perches = perches_of(&draft.stage.surfaces, &draft.stage.keepouts, draft.stage.width, draft.stage.height, tallest, widest);
}

/// 🚚️ How far the surface an actor stands on moved its left end from where the surfaces were `before` (`None` when they did not change): 0 for an actor in the air and for a surface that is new or gone.
fn haul(draft: &Draft<'_>, index: usize, before: Option<&[Surface]>) -> f64 {
    let Some(surface) = draft.stage.actors[index].perch.as_deref() else {
        return 0.0;
    };
    match (surface_of(before.unwrap_or(&draft.stage.surfaces), surface), surface_of(&draft.stage.surfaces, surface)) {
        (Some(old), Some(fresh)) => fresh.x0 - old.x0,
        _ => 0.0,
    }
}

/// 🚡️ A grounded actor rides its surface: it moves with the surface's left end and height, and is held inside the nearest perch of that surface (fading in anew when that carries it farther than its own width). When that surface has no perch left it stays on a perch of another surface that lies exactly under its feet (an element that was replaced by its like); without one it falls — unless the survey also brought new ground (`uprooted`: the scenery changed, and a fall would pass in front of whatever is there now) or the stage is still: then it is gone with its ground and arrives anew. `before` are the surfaces as they were, `None` when they did not change. Answers how far the perch pulled the actor from where its surface carried it (0 for an actor in the air or falling), or `None` when the actor is gone.
fn carry(draft: &mut Draft<'_>, index: usize, before: Option<&[Surface]>, uprooted: bool, now: Ticks) -> Option<f64> {
    let kind = draft.kinds[index];
    let moved = haul(draft, index, before);
    let body = &draft.stage.actors[index];
    let Some(surface) = body.perch.as_deref() else {
        return Some(0.0);
    };
    let x = body.x + moved;
    let half = kind.size.width / 2.0;
    let mut best: Option<&Perch> = None;
    let mut least = f64::INFINITY;
    for perch in &draft.stage.perches {
        if perch.surface != surface {
            continue;
        }
        let low = perch.x0 + half;
        let high = perch.x1 - half;
        let gap = if x < low {
            low - x
        } else if x > high {
            x - high
        } else {
            0.0
        };
        if gap < least {
            best = Some(perch);
            least = gap;
        }
    }
    if best.is_none() {
        best = landing_of(&draft.stage.perches, body.x, body.y + hover_of(kind), body.y + hover_of(kind));
        least = 0.0;
    }
    let Some(best) = best else {
        if uprooted || draft.stage.mode == PetMode::Still {
            vanish(draft, index, now);
            return None;
        }
        drop(draft, index, now);
        return Some(0.0);
    };
    let elsewhere = (best.surface != surface).then(|| best.surface.clone());
    let (low, high, y) = (best.x0 + half, best.x1 - half, best.y - hover_of(kind));
    let fades = least > kind.size.width && draft.stage.mode != PetMode::Still;
    let body = &mut draft.stage.actors[index];
    if elsewhere.is_some() {
        body.perch = elsewhere;
    }
    body.x = clamp(x, low, high);
    body.y = y;
    body.goal = clamp(body.goal + moved, low, high);
    if fades && !body.leaving {
        body.opacity = 0.0;
    }
    Some(least)
}

/// 💺️ Nobody stands in anybody after a ride. Per perch, the actors it holds (leavers aside) must fit with a comfortable gap between their bodies (the sum of their widths plus 8 px per neighbour pair within the width of the perch; one actor always fits): while they do not, the one its perch pulled farthest (`strains`) — among equals the last in `menagerie.species` order — is crowded out where it stood (`places`), as visible as it was (`shown`), and arrives anew once a perch has room. The others are then set apart, each moved as little as possible, left to right and back: the feet of neighbours end at least as far apart as before the ride, held between half of both widths (bodies that touch) and 8 px more. On a still stage whoever is crowded out is gone at once.
fn seat(draft: &mut Draft<'_>, places: &[f64], strains: &[f64], shown: &[f64]) {
    let now = draft.stage.tick;
    let mut gone: Vec<usize> = Vec::new();
    for at in 0..draft.stage.perches.len() {
        let perch = &draft.stage.perches[at];
        let (x0, x1) = (perch.x0, perch.x1);
        let mut members: Vec<usize> = Vec::new();
        for (index, body) in draft.stage.actors.iter().enumerate() {
            if body.leaving || body.perch.as_deref() != Some(perch.surface.as_str()) || body.x < x0 || body.x > x1 {
                continue;
            }
            let mut slot = members.len();
            while slot > 0 && places[members[slot - 1]] > places[index] {
                slot -= 1;
            }
            members.insert(slot, index);
        }
        while members.len() > 1 {
            let mut need = (members.len() - 1) as f64 * COMFORT_GAP;
            for &member in &members {
                need += draft.kinds[member].size.width;
            }
            if need <= x1 - x0 {
                break;
            }
            let mut worst = 0;
            for slot in 1..members.len() {
                let strain = strains[members[slot]];
                let most = strains[members[worst]];
                if strain > most || (strain == most && members[slot] > members[worst]) {
                    worst = slot;
                }
            }
            let index = members.remove(worst);
            draft.stage.actors[index].x = places[index];
            draft.stage.actors[index].opacity = shown[index];
            crowd_out(draft, index, now);
            gone.push(index);
        }
        for slot in 1..members.len() {
            let (before, member) = (members[slot - 1], members[slot]);
            let hard = shoulders(draft.kinds[before], draft.kinds[member]);
            let apart = clamp(places[member] - places[before], hard, hard + COMFORT_GAP);
            let held = draft.stage.actors[before].x;
            if draft.stage.actors[member].x - held < apart {
                draft.stage.actors[member].x = held + apart;
            }
        }
        for slot in (0..members.len()).rev() {
            let member = members[slot];
            let high = x1 - draft.kinds[member].size.width / 2.0;
            if draft.stage.actors[member].x > high {
                draft.stage.actors[member].x = high;
            }
            if slot < members.len() - 1 {
                let after = members[slot + 1];
                let hard = shoulders(draft.kinds[after], draft.kinds[member]);
                let apart = clamp(places[after] - places[member], hard, hard + COMFORT_GAP);
                let held = draft.stage.actors[after].x;
                if held - draft.stage.actors[member].x < apart {
                    draft.stage.actors[member].x = held - apart;
                }
            }
            let body = &mut draft.stage.actors[member];
            if body.activity != Activity::Walk {
                body.goal = body.x;
            }
        }
    }
    if draft.stage.mode != PetMode::Still {
        return;
    }
    gone.sort_unstable_by(|one, other| other.cmp(one));
    for index in gone {
        remove(draft, index);
    }
}

/// 🆕️ Whether the surfaces of the stage hold one that was not there `before`: new ground.
fn widened(draft: &Draft<'_>, before: &[Surface]) -> bool {
    draft.stage.surfaces.iter().any(|surface| surface_of(before, &surface.id).is_none())
}

/// 🎠️ Every grounded actor rides its surface from where the surfaces were `before` (`uprooted`: new ground came with them), and is then seated so that nobody stands in anybody.
fn ride(draft: &mut Draft<'_>, before: Option<&[Surface]>, uprooted: bool) {
    let now = draft.stage.tick;
    let mut places: Vec<f64> = Vec::with_capacity(draft.stage.actors.len());
    let mut strains: Vec<f64> = Vec::with_capacity(draft.stage.actors.len());
    let mut shown: Vec<f64> = Vec::with_capacity(draft.stage.actors.len());
    let mut index = 0;
    while index < draft.stage.actors.len() {
        let place = draft.stage.actors[index].x + haul(draft, index, before);
        let opacity = draft.stage.actors[index].opacity;
        let Some(strain) = carry(draft, index, before, uprooted, now) else {
            continue;
        };
        places.push(place);
        strains.push(strain);
        shown.push(opacity);
        index += 1;
    }
    seat(draft, &places, &strains, &shown);
}
//#endregion 🔖️Riding

//#region 🔖️Arrival
/// 🌟️ A wanted species arrives: one stage draw picks one of its [`quarters`] (uniformly), a place on it a comfortable gap away from everybody there and the way it faces; one actor draw its first idle dwell, idle clip and blink. It fades in (on a still stage it is simply there), standing on its perch, feeling the resting mood of its species at rest, in the first state of its species, untouched and empty-handed. While no perch has room it waits off stage: nothing is drawn, and it is tried again with every survey, every summons and every whole second.
fn arrive<'a>(draft: &mut Draft<'a>, kind: &'a Species, stream: u32) {
    let rooms = quarters(draft, rooms_for(draft, kind));
    if rooms.is_empty() {
        return;
    }
    let words = random_words(&stage_key(draft), 3);
    let chosen = (unit_of(words[0]) * rooms.len() as f64).floor();
    let room = &rooms[if chosen < rooms.len() as f64 { chosen as usize } else { rooms.len() - 1 }];
    let mut along = unit_of(words[1]) * room.span;
    let mut x = room.stretches[0][0];
    for &[x0, x1] in &room.stretches {
        let length = x1 - x0;
        x = x0 + smaller(along, length);
        if along <= length {
            break;
        }
        along -= length;
    }
    let now = draft.stage.tick;
    let counter = now as u32;
    let draws = random_words(&[draft.stage.seed, stream, counter], 3);
    let perch = &draft.stage.perches[room.perch];
    let mode = draft.stage.mode;
    let body = Actor {
        species: kind.id.clone(),
        perch: Some(perch.surface.clone()),
        host: None,
        pitch: None,
        grip: GRIP_BUDGET,
        footing: Footing::Perch,
        x,
        y: perch.y - hover_of(kind),
        vx: 0.0,
        vy: 0.0,
        tilt: 0.0,
        facing: if unit_of(words[2]) < 0.5 { Facing::Right } else { Facing::Left },
        faced: now,
        activity: Activity::Idle,
        since: now,
        until: now + dwell_of(Activity::Idle, mode, unit_of(draws[0])),
        goal: x,
        partner: None,
        clip: clip_at(kind, Activity::Idle, unit_of(draws[1])),
        gaze: CENTRED,
        blink: blink_at(now, unit_of(draws[2])),
        needs: needs_of(kind.temperament),
        opacity: if mode == PetMode::Still { 1.0 } else { 0.0 },
        leaving: false,
        draws: counter.wrapping_add(1),
        feeling: at_rest(kind.mood, now),
        state: kind.states.first().map_or_else(String::new, |state| state.id.clone()),
        state_since: now,
        former: kind.states.first().map_or_else(String::new, |state| state.id.clone()),
        trick: None,
        warmth: COLD,
        hover: no_hover(0),
        hang: None,
        chute: None,
        rope: None,
        emitters: Vec::new(),
    };
    let at = draft.streams.iter().take_while(|&&earlier| earlier < stream).count();
    draft.stage.actors.insert(at, body);
    draft.kinds.insert(at, kind);
    draft.streams.insert(at, stream);
}

/// 📣️ Every wanted species that is not on stage arrives, in `menagerie.species` order — as long as the company on stage, leavers included, is smaller than the wanted one: a newcomer waits until whoever it replaces is gone, so a stage never holds more actors than were summoned.
pub(crate) fn spawn<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>) {
    if draft.stage.perches.is_empty() {
        return;
    }
    for (stream, kind) in menagerie.species.iter().enumerate() {
        if draft.stage.actors.len() >= draft.stage.wanted.len() {
            break;
        }
        if draft.stage.wanted.contains(&kind.id) && index_of(&draft.stage.actors, &kind.id).is_none() {
            arrive(draft, kind, stream as u32);
        }
    }
}

/// 🌬️ The company spreads out over new ground (a survey brought a surface that was not there before). Every perch that holds more than one actor gives up all but the first of them (in `menagerie.species` order), as far as perches that hold nobody can carry them — each such perch takes one: whoever stands idle by itself on the shared perch leaves (on a still stage it is gone at once) and, still wanted, arrives anew where nobody stands. So a company that gathered on the only edge a screen offered — the footer line under an introduction — does not stay parked there when the next screen brings cards. A time of concentration leaves everybody where they are.
fn spread(draft: &mut Draft<'_>) {
    if draft.stage.quiet {
        return;
    }
    let now = draft.stage.tick;
    let mut vacant: Vec<usize> = Vec::new();
    for (at, perch) in draft.stage.perches.iter().enumerate() {
        if crowd_on(draft, perch) == 0 {
            vacant.push(at);
        }
    }
    let mut gone: Vec<usize> = Vec::new();
    for shared in 0..draft.stage.perches.len() {
        let mut first = true;
        let mut index = 0;
        while index < draft.stage.actors.len() && !vacant.is_empty() {
            let at = index;
            index += 1;
            let perch = &draft.stage.perches[shared];
            let body = &draft.stage.actors[at];
            if body.leaving || body.perch.as_deref() != Some(perch.surface.as_str()) || body.x < perch.x0 || body.x > perch.x1 {
                continue;
            }
            if first {
                first = false;
                continue;
            }
            if body.activity != Activity::Idle || body.partner.is_some() {
                continue;
            }
            let width = draft.kinds[at].size.width;
            let Some(home) = vacant.iter().position(|&free| draft.stage.perches[free].x1 - draft.stage.perches[free].x0 >= width) else {
                continue;
            };
            vacant.remove(home);
            if draft.stage.mode == PetMode::Still {
                gone.push(at);
            } else {
                leave(draft, at, now);
            }
        }
    }
    gone.sort_unstable_by(|one, other| other.cmp(one));
    for index in gone {
        if index < draft.stage.actors.len() {
            remove(draft, index);
        }
    }
}
//#endregion 🔖️Arrival

//#region 🔖️Events
/// 🗺️ The stage was measured anew: perches are cut again, grounded actors ride their surfaces, fall or — when the survey brought new ground — are gone with ground that vanished, and whoever waits for a perch arrives; over new ground the company then spreads out ([`spread`]), and on a still stage whoever it moved arrives at once. The walls and the fixtures of the survey are kept as they come.
pub(crate) fn survey<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, event: &Surveyed) {
    let before = std::mem::replace(&mut draft.stage.surfaces, event.surfaces.clone());
    draft.stage.width = event.width;
    draft.stage.height = event.height;
    draft.stage.keepouts.clone_from(&event.keepouts);
    draft.stage.walls.clone_from(&event.walls);
    draft.stage.fixtures.clone_from(&event.fixtures);
    let uprooted = widened(draft, &before);
    measure(menagerie, draft);
    ride(draft, Some(&before), uprooted);
    spawn(menagerie, draft);
    if !uprooted {
        return;
    }
    spread(draft);
    spawn(menagerie, draft);
}

/// 🎟️ The species that belong on stage changed: whoever is no longer wanted leaves (on a still stage it is gone at once), a leaver that is wanted again stays when it still stands on a perch (one that was crowded out goes on fading and arrives anew), perches are cut for the new company and the newly wanted arrive.
pub(crate) fn summon<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, wanted: &[Slug]) {
    let now = draft.stage.tick;
    draft.stage.wanted = wanted.to_vec();
    let mut index = 0;
    while index < draft.stage.actors.len() {
        let body = &mut draft.stage.actors[index];
        let stays = wanted.contains(&body.species);
        if stays && body.leaving && body.perch.is_some() {
            body.leaving = false;
            settle(draft, index, now);
        }
        if !stays && !draft.stage.actors[index].leaving {
            if draft.stage.mode == PetMode::Still {
                remove(draft, index);
                continue;
            }
            leave(draft, index, now);
        }
        index += 1;
    }
    measure(menagerie, draft);
    ride(draft, None, false);
    spawn(menagerie, draft);
}

/// 🧊️ The stage turns still: leavers and whoever is in the air are gone (the wanted among them arrive anew at once, on a perch with room), and everyone else is idle at rest, whole, without a partner, with centred pupils, calm (the resting mood of its species at rest), cold and with no gesture under way.
fn freeze(draft: &mut Draft<'_>, now: Ticks) {
    let mut index = 0;
    while index < draft.stage.actors.len() {
        if draft.stage.actors[index].leaving || draft.stage.actors[index].perch.is_none() {
            remove(draft, index);
            continue;
        }
        shift(draft, index, Activity::Idle, now);
        let body = &mut draft.stage.actors[index];
        body.faced = now;
        body.until = now;
        body.clip = None;
        body.partner = None;
        body.vx = 0.0;
        body.vy = 0.0;
        body.goal = body.x;
        body.gaze = CENTRED;
        body.feeling = at_rest(draft.kinds[index].mood, now);
        body.warmth = COLD;
        body.hover = no_hover(now);
        body.opacity = 1.0;
        index += 1;
    }
}

/// 🎚️ The liveliness changed. Still freezes the stage; leaving still gives every actor a fresh idle dwell, idle clip and blink; a mode that allows fewer movers lets the walkers beyond its limit come to rest (hops in flight count first).
pub(crate) fn tune<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>, mode: PetMode) {
    if mode == draft.stage.mode {
        return;
    }
    let before = draft.stage.mode;
    let now = draft.stage.tick;
    draft.stage.mode = mode;
    if mode == PetMode::Still {
        freeze(draft, now);
        spawn(menagerie, draft);
        return;
    }
    if before == PetMode::Still {
        for index in 0..draft.stage.actors.len() {
            let words = random_words(&actor_key(draft, index), 3);
            let clip = clip_at(draft.kinds[index], Activity::Idle, unit_of(words[1]));
            let body = &mut draft.stage.actors[index];
            body.since = now;
            body.until = now + dwell_of(Activity::Idle, mode, unit_of(words[0]));
            body.clip = clip;
            body.blink = blink_at(now, unit_of(words[2]));
        }
        return;
    }
    let mut movers = draft.stage.actors.iter().filter(|body| !body.leaving && body.activity == Activity::Hop).count();
    for index in 0..draft.stage.actors.len() {
        let body = &draft.stage.actors[index];
        if body.leaving || body.activity != Activity::Walk {
            continue;
        }
        movers += 1;
        if movers <= MODE_LIMITS[mode].movers {
            continue;
        }
        release(draft, index, now);
        settle(draft, index, now);
    }
}
//#endregion 🔖️Events

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
