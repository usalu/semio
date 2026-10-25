//! 👥️ Population: who is on stage and where the ground is — the survey (perches cut anew, grounded actors riding their surfaces and seated apart again), arrivals and spreading out over new ground, the summons, and the liveliness (freezing and thawing).
//!
//! What a survey brings and what it does to the company belongs here.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../👥️population/🟦️.ts — the TypeScript twin

use semio_framework_pets_staging_draft::draft::{actor_key, blink_at, clip_at, crowd_on, hover_of, index_of, release, remove, settle, shift, stage_key, surface_of, Draft};
use semio_framework_pets_staging_draft::locomotion::{drop, leave, vanish};
use semio_framework_pets_staging_draft::spacing::{company_of, perches_for, quarters, rooms_for, Ground};
use semio_framework_pets_character::behavior::{dwell_of, needs_of, MODE_LIMITS};
use semio_framework_pets_character::feeling::at_rest;
use semio_framework_pets_contract::schema::{Activity, Actor, Facing, Footing, Gaze, Menagerie, PetMode, Slug, Species, Surface, Surveyed, Ticks};
use semio_framework_pets_interaction::gesture::{no_hover, COLD};
use semio_framework_pets_kinematics::randomness::{random_words, unit_of};
use semio_framework_pets_kinematics::terrain::{landing_of, larger, smaller};
use semio_framework_pets_kinematics::trigonometry::clamp;
use semio_framework_pets_character::feeling::Occasion;
use semio_framework_pets_contract::schema::Wall;
use semio_framework_pets_clearance::Body as Solid;
use semio_framework_pets_clearance::{MARGIN, evicted, free_at, seat_of, shifted};
use semio_framework_pets_contract::schema::Plume;
use semio_framework_pets_interaction::gesture::{IDLE, circled};
use semio_framework_pets_kinematics::terrain::walls_of;
use semio_framework_pets_staging_draft::draft::feel;
use semio_framework_pets_staging_draft::locomotion::{course_of, halt, poof, trip_of, unfoot, unlifted};
use semio_framework_pets_staging_draft::spacing::{bodies_of, extent_of};
use semio_framework_pets_traversal::climbing::{GRIP_BUDGET, LadderStand, cling_of, ladder_holds, slip_of, spill_of, wall_holds};

//#region 🔖️Constants
pub(crate) const CENTRED: Gaze = Gaze { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0 };
//#endregion 🔖️Constants

//#region 🔖️Riding
/// 📐️ The perches and the pitches of the stage for its company ([`company_of`]: everyone who is on it or wanted and can stand on its ground together): clearance = the tallest of them plus its hover, minimum = the widest, so that every perch carries any of them; none when nobody that is there or wanted can stand on the ground at all.
fn measure(menagerie: &Menagerie, draft: &mut Draft<'_>) {
    let company = company_of(menagerie, draft);
    let mut widest = 0.0;
    let mut smallest = f64::INFINITY;
    for kind in &company {
        widest = larger(widest, kind.size.width);
        smallest = smaller(smallest, kind.size.height);
    }
    let wishes = !draft.stage.actors.is_empty() || draft.stage.wanted.iter().any(|id| menagerie.species.iter().any(|kind| &kind.id == id));
    let perches = if wishes && company.is_empty() { Vec::new() } else { perches_for(Ground { surfaces: &draft.stage.surfaces, keepouts: &draft.stage.keepouts, width: draft.stage.width, height: draft.stage.height }, &company) };
    draft.stage.perches = perches;
    draft.stage.pitches = if smallest.is_infinite() { Vec::new() } else { walls_of(&draft.stage.walls, &draft.stage.keepouts, draft.stage.width, draft.stage.height, widest + 8.0, 0.5 * smallest) };
}

/// 🚡️ Grounded actors ride rigidly with their surface before being seated.
fn carry(draft: &mut Draft<'_>, index: usize, before: &[Surface], uprooted: bool, now: Ticks) -> bool {
    let kind = draft.kinds[index];
    let body = &draft.stage.actors[index];
    let Some(surface) = body.perch.as_deref().filter(|_| body.footing == Footing::Perch) else { return true; };
    let old = surface_of(before, surface);
    let fresh = surface_of(&draft.stage.surfaces, surface);
    let held = draft.stage.perches.iter().any(|perch| perch.surface == surface);
    if let Some(fresh) = fresh.filter(|_| held) {
        let moved = old.map_or(0.0, |old| fresh.x0 - old.x0);
        let y = fresh.y - hover_of(kind);
        let body = &mut draft.stage.actors[index];
        body.x += moved;
        body.goal += moved;
        body.y = y;
        return true;
    }
    if let Some(like) = landing_of(&draft.stage.perches, body.x, body.y + hover_of(kind), body.y + hover_of(kind)) {
        draft.stage.actors[index].perch = Some(like.surface.clone());
        return true;
    }
    if uprooted || draft.stage.mode == PetMode::Still { return vanish(draft, index, now); }
    drop(draft, index, now);
    true
}

/// 🪑️ Seat each perch's actors in priority order without crossing bodies or corridors.
fn seat(draft: &mut Draft<'_>) {
    let now = draft.stage.tick;
    let stranded: Vec<_> = draft.stage.actors.iter().enumerate().filter_map(|(index, body)| {
        if body.footing != Footing::Perch || body.perch.is_none() { return None; }
        let half = draft.kinds[index].size.width / 2.0;
        let least = draft.stage.perches.iter().filter(|perch| Some(&perch.surface) == body.perch.as_ref()).map(|perch| larger(larger(perch.x0 + half - body.x, body.x - (perch.x1 - half)), 0.0)).fold(f64::INFINITY, smaller);
        (least > 2.0 * draft.kinds[index].size.width).then(|| body.species.clone())
    }).collect();
    for species in stranded {
        if let Some(index) = index_of(&draft.stage.actors, &species) { if draft.stage.mode == PetMode::Still { vanish(draft, index, now); } else { drop(draft, index, now); } }
    }
    for at in 0..draft.stage.perches.len() {
        let perch = draft.stage.perches[at].clone();
        let mut members = Vec::new();
        let mut strains = Vec::new();
        for (index, body) in draft.stage.actors.iter().enumerate() {
            if body.footing != Footing::Perch || body.perch.as_deref() != Some(&perch.surface) { continue; }
            let half = draft.kinds[index].size.width / 2.0;
            let mut nearest = None;
            let mut least = f64::INFINITY;
            for (candidate_at, candidate) in draft.stage.perches.iter().enumerate() {
                if candidate.surface != perch.surface { continue; }
                let gap = larger(larger(candidate.x0 + half - body.x, body.x - (candidate.x1 - half)), 0.0);
                if gap < least { nearest = Some(candidate_at); least = gap; }
            }
            if nearest != Some(at) { continue; }
            let strain = if body.leaving { f64::INFINITY } else { least };
            let mut slot = members.len();
            while slot > 0 && strains[slot - 1] > strain { slot -= 1; }
            members.insert(slot, index);
            strains.insert(slot, strain);
        }
        if members.is_empty() { continue; }
        for &member in &members {
            let half = draft.kinds[member].size.width / 2.0;
            let body = &mut draft.stage.actors[member];
            if body.activity == Activity::Walk { body.goal = clamp(body.goal, perch.x0 + half, perch.x1 - half); }
        }
        let solids: Vec<_> = members.iter().map(|&member| Solid { owner: draft.stage.actors[member].species.clone(), extent: extent_of(&draft.stage.actors[member], draft.kinds[member]) }).collect();
        let seating = seat_of(&solids, &perch, MARGIN);
        for owner in seating.leavers {
            let Some(index) = index_of(&draft.stage.actors, &owner) else { continue; };
            let body = &draft.stage.actors[index];
            if body.x < perch.x0 || body.x > perch.x1 { drop(draft, index, now); } else { poof(draft, index, now); }
        }
        for place in seating.seats {
            if place.shift == 0.0 { continue; }
            let Some(index) = index_of(&draft.stage.actors, &place.owner) else { continue; };
            let half = draft.kinds[index].size.width / 2.0;
            let target = clamp(draft.stage.actors[index].x + place.shift, perch.x0 + half, perch.x1 - half);
            if draft.stage.mode == PetMode::Still {
                let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
                let seated = shifted(bodies[index].extent, target - draft.stage.actors[index].x, 0.0);
                if free_at(seated, &place.owner, &bodies, &draft.stage.claims, now) { draft.stage.actors[index].x = target; draft.stage.actors[index].goal = target; } else { vanish(draft, index, now); }
                continue;
            }
            release(draft, index, now);
            if draft.stage.actors[index].activity != Activity::Scoot { shift(draft, index, Activity::Scoot, now); }
            let body = &mut draft.stage.actors[index];
            body.clip = clip_at(draft.kinds[index], Activity::Scoot, 0.0);
            body.goal = target;
            body.until = now + dwell_of(Activity::Scoot, draft.stage.mode, 0.0);
        }
    }
}

/// 🆕️ Detect newly added ground.
fn widened(draft: &Draft<'_>, before: &[Surface]) -> bool {
    draft.stage.surfaces.iter().any(|surface| surface_of(before, &surface.id).is_none())
}

/// 🧗️ Moving terrain carries ladder riders and wall climbers or throws them clear.
fn hold_on(menagerie: &Menagerie, draft: &mut Draft<'_>, walls: &[Wall], now: Ticks) {
    let mut kept = Vec::new();
    for ladder in draft.stage.ladders.clone() {
        let stand = LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top };
        let owner = menagerie.species.iter().find(|kind| kind.id == ladder.owner);
        let next = owner.and_then(|owner| ladder_holds(&stand, &draft.stage.perches, &draft.stage.pitches, &draft.stage.keepouts, owner.size));
        let rider = ladder.rider.as_deref().and_then(|rider| index_of(&draft.stage.actors, rider)).filter(|&rider| draft.stage.actors[rider].footing == Footing::Ladder);
        if let Some(next) = next {
            if let Some(rider) = rider {
                let body = &mut draft.stage.actors[rider];
                let share = clamp((body.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0.0, 1.0);
                body.x = next.foot.x + (next.top.x - next.foot.x) * share;
                body.y = next.foot.y + (next.top.y - next.foot.y) * share;
            }
            kept.push(semio_framework_pets_contract::schema::Ladder { wall: next.wall, surface: next.surface, side: next.side, foot: next.foot, top: next.top, ..ladder });
        } else if let Some(rider) = rider {
            let toss = spill_of(&stand, draft.stage.actors[rider].y, draft.kinds[rider].size);
            feel(draft, rider, Occasion::Startled, now);
            unfoot(draft, rider, toss.map_or(0.0, |toss| toss.vx), toss.map_or(0.0, |toss| toss.vy), Activity::Fall, now);
        }
    }
    draft.stage.ladders = kept;
    for index in 0..draft.stage.actors.len() {
        let body = &draft.stage.actors[index];
        let Some(pitch) = body.pitch.as_ref().filter(|_| body.footing == Footing::Wall) else { continue; };
        let size = draft.kinds[index].size;
        let old = walls.iter().find(|wall| wall.id == pitch.wall);
        let fresh = draft.stage.walls.iter().find(|wall| wall.id == pitch.wall);
        let y = old.zip(fresh).map_or(body.y, |(old, fresh)| body.y + (fresh.y0 - old.y0));
        if let Some(next) = wall_holds(pitch, &draft.stage.pitches, y, size) {
            let x = cling_of(next, size);
            let pitch = next.clone();
            let body = &mut draft.stage.actors[index];
            body.pitch = Some(pitch);
            body.x = x;
            body.y = y;
        } else {
            let toss = slip_of(pitch);
            feel(draft, index, Occasion::Startled, now);
            unfoot(draft, index, toss.vx, toss.vy, Activity::Fall, now);
        }
    }
}

/// 🎠️ Move surfaces, rank unmoved bodies first, then reserve legal seats.
fn ride(menagerie: &Menagerie, draft: &mut Draft<'_>, before: &[Surface], walls: &[Wall], uprooted: bool, reshaped: bool) {
    let now = draft.stage.tick;
    if reshaped {
        for index in 0..draft.stage.actors.len() {
            let body = &draft.stage.actors[index];
            if trip_of(&draft.stage.trips, &body.species).is_none() { continue; }
            let perched = body.footing == Footing::Perch;
            halt(draft, index, now);
            if perched { settle(draft, index, now); }
        }
    }
    for index in 0..draft.stage.actors.len() {
        let body = &draft.stage.actors[index];
        if course_of(&draft.stage.courses, &body.species).is_some() { let (vx, vy, activity) = (body.vx, body.vy, body.activity); unfoot(draft, index, vx, vy, activity, now); }
    }
    let mut index = 0;
    while index < draft.stage.actors.len() {
        let body = &draft.stage.actors[index];
        if body.footing != Footing::Hand && (body.x < 0.0 || body.x > draft.stage.width) { vanish(draft, index, now); } else { index += 1; }
    }
    let was: Vec<_> = draft.stage.actors.iter().map(|body| (body.species.clone(), body.x, body.y)).collect();
    let mut index = 0;
    while index < draft.stage.actors.len() { if carry(draft, index, before, uprooted, now) { index += 1; } }
    if reshaped { hold_on(menagerie, draft, walls, now); }
    for index in 0..draft.stage.actors.len() {
        let body = &draft.stage.actors[index];
        if body.footing != Footing::Head { continue; }
        let Some(host) = body.host.as_deref().and_then(|host| index_of(&draft.stage.actors, host)) else { continue; };
        let Some((_, x, y)) = was.iter().find(|(species, _, _)| species == &draft.stage.actors[host].species) else { continue; };
        let (dx, dy) = (draft.stage.actors[host].x - x, draft.stage.actors[host].y - y);
        draft.stage.actors[index].x += dx;
        draft.stage.actors[index].y += dy;
    }
    for (index, body) in draft.stage.actors.iter_mut().enumerate() {
        let half = draft.kinds[index].size.width / 2.0;
        if body.footing != Footing::Hand && half <= draft.stage.width - half { body.x = clamp(body.x, half, draft.stage.width - half); }
    }
    let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
    let mut ranked = Vec::new();
    let mut moved = Vec::new();
    for (index, body) in draft.stage.actors.iter().enumerate() {
        if was.iter().any(|(species, x, y)| species == &body.species && *x == body.x && *y == body.y) { ranked.push(bodies[index].clone()); } else { moved.push(bodies[index].clone()); }
    }
    ranked.extend(moved);
    for owner in evicted(&ranked, &draft.stage.claims, now) { if let Some(index) = index_of(&draft.stage.actors, &owner) { poof(draft, index, now); } }
    seat(draft);
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
        emitters: kind.states.first().and_then(|state| state.emitter.as_ref()).map_or_else(Vec::new, |emitter| vec![Plume { emitter: emitter.clone(), since: now, until: None }]),
    };
    let at = draft.streams.iter().take_while(|&&earlier| earlier < stream).count();
    draft.stage.actors.insert(at, body);
    draft.kinds.insert(at, kind);
    draft.streams.insert(at, stream);
}

/// 📣️ Every wanted species that is not on stage arrives, in `menagerie.species` order — as long as the company on stage, leavers included, is smaller than the wanted one: a newcomer waits until whoever it replaces is gone, so a stage never holds more actors than were summoned. A wanted species that cannot stand on the ground together with the company ([`company_of`]) waits off stage and does not hold the others back.
pub(crate) fn spawn<'a>(menagerie: &'a Menagerie, draft: &mut Draft<'a>) {
    if draft.stage.perches.is_empty() {
        return;
    }
    let company: Vec<Slug> = company_of(menagerie, draft).into_iter().map(|kind| kind.id.clone()).collect();
    for (stream, kind) in menagerie.species.iter().enumerate() {
        if draft.stage.actors.len() >= draft.stage.wanted.len() {
            break;
        }
        if draft.stage.wanted.contains(&kind.id) && index_of(&draft.stage.actors, &kind.id).is_none() && company.contains(&kind.id) {
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
            if body.activity != Activity::Idle || body.partner.is_some() || circled(&body.hover, now) {
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
    if let Some(lift) = &draft.stage.lift {
        let was = draft.stage.fixtures.iter().find(|fixture| fixture.id == lift.fixture);
        let fresh = event.fixtures.iter().find(|fixture| fixture.id == lift.fixture);
        if was.zip(fresh).is_none_or(|(was, fresh)| (fresh.x - was.x).abs() > 0.5 || (fresh.y - was.y).abs() > 0.5 || (fresh.x + fresh.width - was.x - was.width).abs() > 0.5 || (fresh.y + fresh.height - was.y - was.height).abs() > 0.5) { unlifted(draft, draft.stage.tick); }
    }
    let before = std::mem::replace(&mut draft.stage.surfaces, event.surfaces.clone());
    let walls = std::mem::replace(&mut draft.stage.walls, event.walls.clone());
    let perches = draft.stage.perches.clone();
    let pitches = draft.stage.pitches.clone();
    let keepouts = draft.stage.keepouts.clone();
    draft.stage.width = event.width;
    draft.stage.height = event.height;
    draft.stage.keepouts.clone_from(&event.keepouts);
    draft.stage.fixtures.clone_from(&event.fixtures);
    let uprooted = widened(draft, &before);
    measure(menagerie, draft);
    let reshaped = perches != draft.stage.perches || pitches != draft.stage.pitches || keepouts != draft.stage.keepouts;
    ride(menagerie, draft, &before, &walls, uprooted, reshaped);
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
        if stays && body.leaving && body.footing == Footing::Perch {
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
    let perches = draft.stage.perches.clone();
    let pitches = draft.stage.pitches.clone();
    measure(menagerie, draft);
    let surfaces = draft.stage.surfaces.clone();
    let walls = draft.stage.walls.clone();
    let reshaped = perches != draft.stage.perches || pitches != draft.stage.pitches;
    ride(menagerie, draft, &surfaces, &walls, false, reshaped);
    spawn(menagerie, draft);
}

/// 🧊️ The stage turns still: leavers and whoever is in the air are gone (the wanted among them arrive anew at once, on a perch with room), and everyone else is idle at rest, whole, without a partner, with centred pupils, calm (the resting mood of its species at rest), cold and with no gesture under way.
fn freeze(draft: &mut Draft<'_>, now: Ticks) {
    unlifted(draft, now);
    draft.stage.lift = None;
    draft.stage.press = IDLE;
    draft.stage.touched = None;
    draft.stage.trail.clear();
    draft.stage.origin = None;
    let mut index = 0;
    while index < draft.stage.actors.len() {
        if draft.stage.actors[index].leaving || draft.stage.actors[index].footing != Footing::Perch {
            remove(draft, index);
            continue;
        }
        halt(draft, index, now);
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
    let mut movers = draft.stage.actors.iter().filter(|body| !body.leaving && (body.activity == Activity::Hop || (body.footing != Footing::Perch && trip_of(&draft.stage.trips, &body.species).is_some()))).count();
    for index in 0..draft.stage.actors.len() {
        let body = &draft.stage.actors[index];
        let setting = body.footing == Footing::Perch && trip_of(&draft.stage.trips, &body.species).is_some();
        if body.leaving || (body.activity != Activity::Walk && !setting) {
            continue;
        }
        movers += 1;
        if movers <= MODE_LIMITS[mode].movers {
            continue;
        }
        halt(draft, index, now);
        release(draft, index, now);
        settle(draft, index, now);
    }
}
//#endregion 🔖️Events

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
