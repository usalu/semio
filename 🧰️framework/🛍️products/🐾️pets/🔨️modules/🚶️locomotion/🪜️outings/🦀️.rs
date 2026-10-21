//! 🪜️ Outings: planning a gear trip over walls, ladders and grappling ropes and committing it only when its corridor is clear. See [`super`].

use super::*;
use semio_framework_pets_traversal::climbing::Handwork;
use semio_framework_pets_traversal::climbing::Leg;
use semio_framework_pets_traversal::climbing::Entry;
use semio_framework_pets_contract::schema::Facing;
use semio_framework_pets_contract::schema::Arrival;
use semio_framework_pets_contract::schema::Gear;
use semio_framework_pets_contract::schema::Trip;
use semio_framework_pets_traversal::climbing::LadderStand;
use semio_framework_pets_contract::schema::Rope;
use semio_framework_pets_contract::schema::Pitch;
use semio_framework_pets_contract::schema::Foothold;
use semio_framework_pets_traversal::climbing::WallHold;
use semio_framework_pets_contract::schema::Ladder;
use semio_framework_pets_contract::schema::Shot;
use crate::draft::{TURN_TICKS, facing_to, turn};
use crate::spacing::{bodies_of, extent_at, wall_lean_of};
use semio_framework_pets_clearance::{claim_clear, claim_of, grown, meets, released};
use semio_framework_pets_kinematics::trigonometry::smoothstep;
use semio_framework_pets_traversal::climbing::{GRIP_BUDGET, GRIP_CLIMB, GRIP_HANG, HOOK_RETURN, HOOK_SPEED, LADDER_DISMOUNT_TICKS, LADDER_IDLE, LADDER_LIFE, LADDER_MOUNT_TICKS, LADDER_RAISE_TICKS, ROPE_AIM_TICKS, ROPE_HOIST_TICKS, ROPE_MARGIN, ROPE_MISS_CHANCE, ROPE_SHRUG_TICKS, ROPE_TUG_TICKS, WALL_GRAB_TICKS, chain_of, cling_of, clings, foot_of, grip_for, haul_of, haul_step, hoist_path, hook_ticks, ladder_at, ladder_exit, ladder_landing, ladder_length, ladder_step, ladder_to, landing_for, ledge_of, miss_of, rim_for, rim_of, route_of, shot_for, sighted, slide_step, wall_cost, wall_path};
use semio_framework_pets_traversal::swing::REEL_LEAST;

//#region 🔖️Gear
/// ⛺️ A planned gear trip and the ladder or rope it installs.
#[derive(Clone)]
pub struct Outing { pub(super) trip: Trip, pub(super) raise: Option<LadderStand>, pub(super) stands: Ticks, pub(super) rope: Option<Rope> }

/// 🧿️ A destination on a perch or a chosen height along a wall.
#[derive(Clone)]
pub enum Venture { Perch(Perch), Wall { pitch: Pitch, y: Option<f64> } }

/// 🧩️ The stance where a gear leg begins.
pub(super) struct Stand { pub(super) perch: Perch, pub(super) x: f64, pub(super) y: f64, pub(super) facing: Facing, pub(super) grip: f64 }

/// 🎒️ Gear moves non-floating species along walls, ladders and grappling ropes.
pub fn geared(kind: &Species) -> bool {
    kind.locomotion.gait != Gait::Float && kind.gear.iter().any(|gear| matches!(gear, Gear::Climb | Gear::Ladder | Gear::Grapple))
}

/// 🗻️ Find a trip by its owning species.
pub fn trip_of<'a>(trips: &'a [Trip], owner: &str) -> Option<&'a Trip> { trips.iter().find(|trip| trip.owner == owner) }

/// 🪺️ The interval where the complete footprint fits on a perch.
fn room_on(perch: &Perch, half: f64) -> Option<[f64; 2]> { (perch.x0 + half <= perch.x1 - half).then_some([perch.x0 + half, perch.x1 - half]) }

/// 🧲️ Face towards the wall, opposite its outward side.
fn wallward(side: Facing) -> Facing { if side == Facing::Right { Facing::Left } else { Facing::Right } }

/// 💸️ Hanging and climbing consume their own grip budget per wall foothold.
pub(super) fn effort_of(step: &Foothold) -> f64 { if step.footing != Footing::Wall { 0.0 } else if step.activity == Activity::Slide { GRIP_HANG } else { GRIP_CLIMB } }

/// 🔋️ Hanging consumes the grip anchored at the beginning of the rest.
pub(super) fn grip_now(body: &semio_framework_pets_contract::schema::Actor, now: Ticks) -> f64 { larger(body.grip - GRIP_HANG * (now - body.since) as f64, 0.0) }

/// 🚸️ Plan the walk to gear, including the initial turn and exact final stride.
pub(super) fn walked(stand: &Stand, kind: &Species, to: f64, activity: Activity) -> Option<Vec<Foothold>> {
    let mut steps = Vec::new();
    if to == stand.x { return Some(steps); }
    if kind.locomotion.speed <= 0.0 { return None; }
    let facing = facing_to(to, stand.x);
    if facing != stand.facing { for _ in 0..TURN_TICKS { steps.push(Foothold { x: stand.x, y: stand.y, footing: Footing::Perch, perch: Some(stand.perch.surface.clone()), activity, facing, hold: -1 }); } }
    let mut x = stand.x;
    for _ in 0..4096 {
        x = stride_to(x, to, kind.locomotion.speed);
        steps.push(Foothold { x, y: stand.y, footing: Footing::Perch, perch: Some(stand.perch.surface.clone()), activity, facing, hold: -1 });
        if x == to { return Some(steps); }
    }
    None
}

/// 🪵️ Append the exact eased movement along a ladder's rails.
fn rungs(stand: &LadderStand, travel: f64, goal: f64, steps: &mut Vec<Foothold>) {
    let mut along = travel;
    for tick in 0..4096 {
        if along == goal { break; }
        along = ladder_step(along, goal, tick);
        let feet = ladder_at(stand, along);
        steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing: wallward(stand.side), hold: -1 });
    }
}

/// 🧱️ Plan a wall line with its initial grip, mantle or foot exit and exact grip cost.
pub(super) fn scaling(draft: &Draft<'_>, index: usize, lead: &[Foothold], chain: &[&Pitch], start: usize, entry: Entry, x: f64, y: f64, end: usize, goal: f64, mantle: bool, foot: Option<&Perch>, grip: f64, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let path = wall_path(chain, start, entry, x, y, end, goal, mantle, kind.size);
    let mut steps = lead.to_vec();
    for clamber in &path { steps.push(Foothold { x: clamber.x, y: clamber.y, footing: Footing::Wall, perch: None, activity: if clamber.work == Handwork::Mantle { Activity::Mantle } else { Activity::Climb }, facing: wallward(chain[clamber.hold].side), hold: clamber.hold as i64 }); }
    let (mut ending, mut landing) = (Arrival::Wall, String::new());
    let mut cost = wall_cost(&path);
    if mantle {
        let top = rim_for(chain[end], &draft.stage.perches, kind.size)?;
        let room = room_on(top, kind.size.width / 2.0)?;
        let ledge = ledge_of(chain[end], kind.size);
        if ledge < room[0] || ledge > room[1] { return None; }
        ending = Arrival::Perch;
        landing = top.surface.clone();
    } else if let Some(foot) = foot {
        let room = room_on(foot, kind.size.width / 2.0)?;
        let off = cling_of(chain[end], kind.size);
        let to = clamp(off, room[0], room[1]);
        if to != off {
            for tick in 1..=WALL_GRAB_TICKS { steps.push(Foothold { x: if tick == WALL_GRAB_TICKS { to } else { off + (to - off) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64) }, y: foot.y, footing: Footing::Wall, perch: None, activity: Activity::Climb, facing: wallward(chain[end].side), hold: end as i64 }); }
            cost += WALL_GRAB_TICKS as f64 * GRIP_CLIMB;
        }
        ending = Arrival::Perch;
        landing = foot.surface.clone();
    }
    if steps.is_empty() || cost > grip { return None; }
    Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: chain.iter().map(|pitch| (*pitch).clone()).collect(), ladder: None, ending, landing, grip: grip - cost }, raise: None, stands: 0, rope: None })
}

/// 🏔️ Walk to a wall hold and continue along its planned wall line.
fn from_perch(draft: &Draft<'_>, index: usize, stand: &Stand, chain: &[&Pitch], start: usize, hold: WallHold, end: usize, goal: f64, mantle: bool, foot: Option<&Perch>, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let room = room_on(&stand.perch, kind.size.width / 2.0)?;
    let at = if hold.over { ledge_of(chain[start], kind.size) } else { clamp(hold.x, room[0], room[1]) };
    if at < room[0] || at > room[1] { return None; }
    let lead = walked(stand, kind, at, Activity::Walk)?;
    scaling(draft, index, &lead, chain, start, if hold.over { Entry::Hang } else { Entry::Grab }, at, stand.y, end, goal, mantle, foot, stand.grip, from)
}

/// 🏗️ Carry, raise, mount and climb a ladder through its exit.
fn ladder_lead(draft: &Draft<'_>, index: usize, stand: &Stand, ladder: &LadderStand, raise: bool, from: Ticks) -> Option<(Vec<Foothold>, Ticks)> {
    let kind = draft.kinds[index];
    let room = room_on(&stand.perch, kind.size.width / 2.0)?;
    let at = clamp(ladder.foot.x, room[0], room[1]);
    let mut steps = walked(stand, kind, at, if raise { Activity::Carry } else { Activity::Walk })?;
    let facing = wallward(ladder.side);
    let mut stands = 0;
    if raise {
        for _ in 0..LADDER_RAISE_TICKS { steps.push(Foothold { x: at, y: stand.y, footing: Footing::Perch, perch: Some(stand.perch.surface.clone()), activity: Activity::Carry, facing, hold: -1 }); }
        stands = from + steps.len() as Ticks;
    }
    for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { ladder.foot.x } else { at + (ladder.foot.x - at) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
    rungs(ladder, 0.0, ladder_exit(ladder, kind.size), &mut steps);
    Some((steps, stands))
}

/// 🪤️ Raise a ladder as the first leg of a climb onto its wall line.
fn lean_outing(draft: &Draft<'_>, index: usize, stand: &Stand, chain: &[&Pitch], start: usize, lean: &LadderStand, end: usize, goal: f64, mantle: bool, foot: Option<&Perch>, from: Ticks) -> Option<Outing> {
    let (steps, stands) = ladder_lead(draft, index, stand, lean, true, from)?;
    let exit = ladder_at(lean, ladder_exit(lean, draft.kinds[index].size));
    let mut outing = scaling(draft, index, &steps, chain, start, Entry::Grab, exit.x, exit.y, end, goal, mantle, foot, stand.grip, from)?;
    outing.trip.ladder = Some(draft.stage.actors[index].species.clone());
    outing.raise = Some(lean.clone());
    outing.stands = stands;
    Some(outing)
}

/// 🪜️ Plan an ascent or descent over a standing or newly raised ladder.
fn ladder_outing(draft: &Draft<'_>, index: usize, stand: &Stand, ladder: &LadderStand, owner: &str, up: bool, to: &Perch, raise: bool, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let start = room_on(&stand.perch, kind.size.width / 2.0)?;
    let goal = room_on(to, kind.size.width / 2.0)?;
    let exit = ladder_exit(ladder, kind.size);
    let landing = ladder_landing(ladder, kind.size);
    let top = ladder_at(ladder, exit);
    let facing = wallward(ladder.side);
    let mut steps;
    let mut stands = 0;
    if up {
        if landing.x < goal[0] || landing.x > goal[1] { return None; }
        (steps, stands) = ladder_lead(draft, index, stand, ladder, raise, from)?;
        for tick in 1..=LADDER_DISMOUNT_TICKS { let feet = hoist_path(top, landing, kind.size.height, tick as f64 / LADDER_DISMOUNT_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Mantle, facing, hold: -1 }); }
    } else {
        if landing.x < start[0] || landing.x > start[1] { return None; }
        steps = walked(stand, kind, landing.x, Activity::Walk)?;
        for tick in 1..=LADDER_DISMOUNT_TICKS { let feet = hoist_path(top, landing, kind.size.height, 1.0 - tick as f64 / LADDER_DISMOUNT_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
        rungs(ladder, exit, 0.0, &mut steps);
        let off = clamp(ladder.foot.x, goal[0], goal[1]);
        if off != ladder.foot.x { for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { off } else { ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); } }
    }
    Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: Vec::new(), ladder: Some(owner.to_owned()), ending: Arrival::Perch, landing: to.surface.clone(), grip: stand.grip }, raise: raise.then(|| ladder.clone()), stands, rope: None })
}

/// 🛤️ Reach a ladder from a wall, descend it and step onto its ground.
fn ladder_down(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, x: f64, y: f64, grip: f64, ladder: &Ladder, from: Ticks) -> Option<Outing> {
    let size = draft.kinds[index].size;
    let stand = LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top };
    let at = chain.iter().position(|pitch| pitch.wall == ladder.wall && pitch.side == ladder.side && pitch.y0 <= ladder.top.y && ladder.top.y <= pitch.y1)?;
    let exit = ladder_exit(&stand, size);
    let door = ladder_at(&stand, exit);
    if !clings(chain[at], door.y, size) { return None; }
    let ground = draft.stage.perches.iter().find(|perch| perch.surface == ladder.surface && perch.y == ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1)?;
    let below = room_on(ground, size.width / 2.0)?;
    let along = scaling(draft, index, &[], chain, start, Entry::Cling, x, y, at, door.y, false, None, grip, from)?;
    let facing = wallward(ladder.side);
    let mut steps = along.trip.steps;
    let off = cling_of(chain[at], size);
    for tick in 1..=WALL_GRAB_TICKS { steps.push(Foothold { x: if tick == WALL_GRAB_TICKS { door.x } else { off + (door.x - off) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64) }, y: door.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
    rungs(&stand, exit, 0.0, &mut steps);
    let foot = clamp(ladder.foot.x, below[0], below[1]);
    if foot != ladder.foot.x { for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { foot } else { ladder.foot.x + (foot - ladder.foot.x) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); } }
    Some(Outing { trip: Trip { owner: along.trip.owner, from, steps, pitches: chain.iter().map(|pitch| (*pitch).clone()).collect(), ladder: Some(ladder.owner.clone()), ending: Arrival::Perch, landing: ground.surface.clone(), grip: along.trip.grip }, raise: None, stands: 0, rope: None })
}

/// 🛗️ Find exits from a wall line, cheapest first with stable ties.
pub(super) fn ways_off(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, x: f64, y: f64, grip: f64, now: Ticks) -> Vec<Outing> {
    let kind = draft.kinds[index];
    let mut ways = Vec::new();
    for at in start..chain.len() {
        for perch in &draft.stage.perches {
            let Some(hold) = grip_for(perch, chain[at], kind.size) else { continue; };
            if hold.over { continue; }
            if let Some(way) = scaling(draft, index, &[], chain, start, Entry::Cling, x, y, at, perch.y, false, Some(perch), grip, now + 1) { ways.push(way); }
        }
    }
    for at in (0..=start).rev() {
        if rim_for(chain[at], &draft.stage.perches, kind.size).is_none() { continue; }
        if let Some(way) = scaling(draft, index, &[], chain, start, Entry::Cling, x, y, at, rim_of(chain[at], kind.size), true, None, grip, now + 1) { ways.push(way); }
    }
    for ladder in &draft.stage.ladders {
        if ladder.since > now || ladder.until <= now { continue; }
        if let Some(way) = ladder_down(draft, index, chain, start, x, y, grip, ladder, now + 1) { ways.push(way); }
    }
    ways.sort_by(|one, other| other.trip.grip.total_cmp(&one.trip.grip));
    ways
}

/// 🪫️ Reserve the grip needed for the cheapest available exit from a wall.
pub(super) fn reserve_of(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, x: f64, y: f64, now: Ticks) -> f64 {
    ways_off(draft, index, chain, start, x, y, GRIP_BUDGET, now).first().map_or(0.0, |way| GRIP_BUDGET - way.trip.grip)
}

/// 🦺️ The complete body at a gear foothold includes its leaning wall posture.
fn body_at(kind: &Species, trip: &Trip, step: &Foothold) -> semio_framework_pets_contract::schema::Extent {
    extent_at(&trip.owner, kind, Point { x: step.x, y: step.y }, wall_lean_of(kind, step.footing, (step.hold >= 0).then(|| &trip.pitches[step.hold as usize]), step.x), None)
}

/// ⚖️ A wall destination must leave enough grip for a rest and its exit.
pub(super) fn lasting(draft: &Draft<'_>, index: usize, outing: &Outing, now: Ticks) -> bool {
    let trip = &outing.trip;
    let last = trip.steps.last().unwrap();
    let pitch = &trip.pitches[last.hold as usize];
    let chain = chain_of(pitch, &draft.stage.pitches, draft.kinds[index].size);
    let start = chain.iter().position(|candidate| *candidate == pitch).unwrap();
    trip.grip >= reserve_of(draft, index, &chain, start, last.x, last.y, now) + GRIP_HANG * 128.0
}

/// 🛏️ Wall destinations preserve the comfortable spacing used on perches.
pub(super) fn spaced(draft: &Draft<'_>, index: usize, outing: &Outing) -> bool {
    let room = grown(body_at(draft.kinds[index], &outing.trip, outing.trip.steps.last().unwrap()), COMFORT_GAP);
    bodies_of(&draft.stage.actors, &draft.kinds).iter().all(|other| other.owner == outing.trip.owner || !meets(room, other.extent))
}

/// 🏕️ Find wall rests from direct holds and ladder exits, halving unreachable rests.
fn rest_spots(draft: &Draft<'_>, index: usize, stand: &Stand, pitch: &Pitch, y: Option<f64>, place: f64, height: f64, from: Ticks, now: Ticks) -> Vec<Outing> {
    let kind = draft.kinds[index];
    let chain = chain_of(pitch, &draft.stage.pitches, kind.size);
    let leans = kind.gear.contains(&Gear::Ladder) && kind.gear.contains(&Gear::Climb);
    let mut outings = Vec::new();
    for start in 0..chain.len() {
        let mut holds = Vec::new();
        if let Some(hold) = grip_for(&stand.perch, chain[start], kind.size) { holds.push((hold, None)); }
        if let Some(lean) = leans.then(|| ladder_to(&stand.perch, chain[start], &draft.stage.keepouts, kind.size)).flatten() {
            holds.push((WallHold { x: cling_of(chain[start], kind.size), y: ladder_at(&lean, ladder_exit(&lean, kind.size)).y, over: false }, Some(lean)));
        }
        for (hold, lean) in holds {
            let over = hold.over;
            let entry = if over { rim_of(chain[start], kind.size) } else { hold.y };
            let mut spots = Vec::new();
            if let Some(y) = y { spots.push((chain.iter().position(|candidate| *candidate == pitch).unwrap(), y)); }
            else {
                let ahead = if over { chain.len() - start } else { start + 1 };
                let offset = std::cmp::min((place * ahead as f64).floor() as usize, ahead - 1);
                let at = if over { start + offset } else { start - offset };
                let near = if at == start { entry } else if over { rim_of(chain[at], kind.size) } else { foot_of(chain[at], kind.size) };
                let far = if over { foot_of(chain[at], kind.size) } else { rim_of(chain[at], kind.size) };
                spots.push((at, near + (far - near) * height));
                let end = if over { foot_of(chain[start], kind.size) } else { rim_of(chain[start], kind.size) };
                let mut share = 1.0;
                for _ in 0..3 { share /= 2.0; spots.push((start, entry + (end - entry) * share)); }
            }
            for (at, spot) in spots {
                let outing = if let Some(lean) = &lean { lean_outing(draft, index, stand, &chain, start, lean, at, spot, false, None, from) } else { from_perch(draft, index, stand, &chain, start, hold, at, spot, false, None, from) };
                if let Some(outing) = outing.filter(|outing| lasting(draft, index, outing, now) && spaced(draft, index, outing)) { outings.push(outing); break; }
            }
        }
    }
    outings
}

/// 🪝️ Plan a grappling shot, miss and return or a complete haul and mantle.
fn rope_outing(draft: &Draft<'_>, index: usize, stand: &Stand, shot: &Shot, to: &Perch, at: f64, miss: bool, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let goal = room_on(to, kind.size.width / 2.0)?;
    let mut steps = walked(stand, kind, at, Activity::Walk)?;
    let perch = stand.perch.surface.clone();
    let facing = shot.facing;
    for _ in 0..ROPE_AIM_TICKS { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
    let fire = from + steps.len() as Ticks;
    let point = miss_of(shot, to);
    let under = Point { x: shot.hook.x, y: to.y };
    let missing = miss && point.x >= 0.0 && point.x <= draft.stage.width && sighted(shot.muzzle, point, &draft.stage.keepouts, ROPE_MARGIN, under, under);
    let target = if missing { point } else { shot.hook };
    let out = hook_ticks(shot.muzzle, target, HOOK_SPEED);
    for _ in 0..out { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
    let (dx, dy) = (target.x - shot.muzzle.x, target.y - shot.muzzle.y);
    let rope = Rope { shot: if missing { Shot { hook: target, ..shot.clone() } } else { shot.clone() }, since: fire, length: (dx * dx + dy * dy).sqrt(), hand: shot.muzzle, before: shot.muzzle, caught: !missing };
    if missing {
        for _ in 0..hook_ticks(target, shot.muzzle, HOOK_RETURN) { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
        for _ in 0..ROPE_SHRUG_TICKS { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Shrug, facing, hold: -1 }); }
        return Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: Vec::new(), ladder: None, ending: Arrival::Perch, landing: perch, grip: stand.grip }, raise: None, stands: 0, rope: Some(rope) });
    }
    for _ in 0..ROPE_TUG_TICKS { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
    let mut haul = haul_of(shot, shot.length, kind.size);
    for tick in 0..4096 {
        if haul.rope <= REEL_LEAST * kind.size.height { break; }
        haul = haul_step(shot, haul, tick, kind.size);
        steps.push(Foothold { x: haul.x, y: haul.y, footing: Footing::Rope, perch: None, activity: Activity::Reel, facing, hold: -1 });
    }
    let landed = landing_for(shot, to, kind.size);
    let spot = Point { x: clamp(landed.x, goal[0], goal[1]), y: landed.y };
    for tick in 1..=ROPE_HOIST_TICKS { let feet = hoist_path(Point { x: haul.x, y: haul.y }, spot, kind.size.height, tick as f64 / ROPE_HOIST_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Rope, perch: None, activity: Activity::Mantle, facing, hold: -1 }); }
    Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: Vec::new(), ladder: None, ending: Arrival::Perch, landing: to.surface.clone(), grip: stand.grip }, raise: None, stands: 0, rope: Some(rope) })
}

/// 📶️ Rank outings by duration while preserving equal-duration order.
pub(super) fn ranked(outings: &mut Vec<Outing>, outing: Outing) {
    let mut at = outings.len();
    while at > 0 && outings[at - 1].trip.steps.len() > outing.trip.steps.len() { at -= 1; }
    outings.insert(at, outing);
}

/// 🔗️ Join two complete legs when they share at most one ladder and one rope.
fn joined(first: &Outing, second: Outing) -> Option<Outing> {
    if (first.trip.ladder.is_some() && second.trip.ladder.is_some()) || (first.rope.is_some() && second.rope.is_some()) { return None; }
    let offset = first.trip.pitches.len() as i64;
    let mut steps = first.trip.steps.clone();
    steps.extend(second.trip.steps.into_iter().map(|mut step| { if step.hold >= 0 { step.hold += offset; } step }));
    let mut pitches = first.trip.pitches.clone();
    pitches.extend(second.trip.pitches);
    Some(Outing { trip: Trip { owner: first.trip.owner.clone(), from: first.trip.from, steps, pitches, ladder: first.trip.ladder.clone().or(second.trip.ladder), ending: second.trip.ending, landing: second.trip.landing, grip: second.trip.grip }, raise: first.raise.clone().or(second.raise), stands: if first.raise.is_some() { first.stands } else { second.stands }, rope: first.rope.clone().or(second.rope) })
}

/// 🧷️ Plan all direct legs to a perch or wall rest in stable quickest-first order.
fn legs_to(draft: &Draft<'_>, index: usize, stand: &Stand, venture: &Venture, units: &[f64], miss: bool, from: Ticks, now: Ticks) -> Vec<Outing> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let to = match venture { Venture::Wall { pitch, y } => return rest_spots(draft, index, stand, pitch, *y, units[0], units[1], from, now), Venture::Perch(perch) => perch };
    let standing: Vec<_> = draft.stage.ladders.iter().filter(|ladder| ladder.rider.is_none() && ladder.since <= now && ladder.until > now).collect();
    let ladders: Vec<_> = standing.iter().map(|ladder| LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top }).collect();
    let legs = route_of(stand.x, &stand.perch, to, &kind.gear, kind.size, stand.grip, &draft.stage.pitches, &ladders, &draft.stage.keepouts).unwrap_or_default();
    let mut outings = Vec::new();
    for leg in legs {
        let outing = match leg {
            Leg::Ladder { ladder, up, .. } => ladders.iter().position(|candidate| std::ptr::eq(candidate, ladder)).and_then(|at| ladder_outing(draft, index, stand, ladder, &standing[at].owner, up, to, false, from)),
            Leg::Wall { pitch, hold, exit, goal, .. } => { let chain = chain_of(pitch, &draft.stage.pitches, kind.size); from_perch(draft, index, stand, &chain, chain.iter().position(|candidate| *candidate == pitch).unwrap(), hold, chain.iter().position(|candidate| *candidate == exit).unwrap(), goal, !hold.over, hold.over.then_some(to), from) },
            Leg::Raise { ladder, .. } => if draft.stage.ladders.len() < 2 && !draft.stage.ladders.iter().any(|ladder| ladder.owner == body.species) { ladder_outing(draft, index, stand, &ladder, &body.species, true, to, true, from) } else { None },
            Leg::Grapple { at, shot } => {
                room_on(&stand.perch, kind.size.width / 2.0).and_then(|room| {
                    let place = clamp(at, room[0], room[1]);
                    let shot = if place == at { Some(shot) } else { shot_for(Point { x: place, y: stand.perch.y }, std::slice::from_ref(to), &draft.stage.keepouts, kind.size) }?;
                    rope_outing(draft, index, stand, &shot, to, place, miss, from)
                })
            }
        };
        if let Some(outing) = outing { ranked(&mut outings, outing); }
    }
    outings
}

/// 🗺️ Plan direct outings, then two-leg outings only when no direct way exists.
pub(super) fn outings_to(draft: &Draft<'_>, index: usize, venture: &Venture, units: &[f64], now: Ticks) -> Vec<Outing> {
    let body = &draft.stage.actors[index];
    if !geared(draft.kinds[index]) || body.footing != Footing::Perch { return Vec::new(); }
    let Some(from) = body.perch.as_deref().and_then(|surface| perch_at(&draft.stage.perches, surface, body.x)) else { return Vec::new(); };
    let stand = Stand { perch: from.clone(), x: body.x, y: body.y, facing: body.facing, grip: body.grip };
    let direct = legs_to(draft, index, &stand, venture, units, units[2] < ROPE_MISS_CHANCE, now + 1, now);
    if !direct.is_empty() { return direct; }
    let mut outings = Vec::new();
    for between in &draft.stage.perches {
        if between == from || matches!(venture, Venture::Perch(perch) if perch == between) { continue; }
        for first in legs_to(draft, index, &stand, &Venture::Perch(between.clone()), units, false, now + 1, now) {
            let last = first.trip.steps.last().unwrap();
            let arrival = Stand { perch: between.clone(), x: last.x, y: last.y, facing: last.facing, grip: first.trip.grip };
            for second in legs_to(draft, index, &arrival, venture, units, false, first.trip.from + first.trip.steps.len() as Ticks, now) { if let Some(both) = joined(&first, second) { ranked(&mut outings, both); } }
        }
    }
    outings
}

/// 🌉️ Gear reaches a destination directly or through one intermediate perch.
pub fn reaches(draft: &Draft<'_>, kind: &Species, x: f64, from: &Perch, to: &Perch, now: Ticks) -> bool {
    let ladders: Vec<_> = draft.stage.ladders.iter().filter(|ladder| ladder.rider.is_none() && ladder.since <= now && ladder.until > now).map(|ladder| LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top }).collect();
    let route = |x, from, to| route_of(x, from, to, &kind.gear, kind.size, GRIP_BUDGET, &draft.stage.pitches, &ladders, &draft.stage.keepouts).is_some();
    if route(x, from, to) { return true; }
    draft.stage.perches.iter().any(|between| between != from && between != to && route(x, from, between) && route((between.x0 + between.x1) / 2.0, between, to))
}

/// 🎋️ Plan both exits from a ladder, trying the nearer one first.
pub(super) fn ladder_ways(draft: &Draft<'_>, index: usize, ladder: &Ladder, now: Ticks) -> Vec<Outing> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let stand = LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top };
    let travel = clamp((body.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0.0, 1.0) * ladder_length(&stand);
    let exit = ladder_exit(&stand, kind.size);
    let landing = ladder_landing(&stand, kind.size);
    let facing = wallward(ladder.side);
    let mut ways = Vec::new();
    let crown = draft.stage.pitches.iter().filter(|pitch| pitch.wall == ladder.wall && pitch.side == ladder.side).find_map(|pitch| rim_for(pitch, &draft.stage.perches, kind.size));
    if let Some(crown) = crown {
        if room_on(crown, kind.size.width / 2.0).is_some_and(|above| landing.x >= above[0] && landing.x <= above[1]) {
            let mut steps = Vec::new();
            rungs(&stand, travel, exit, &mut steps);
            let top = ladder_at(&stand, exit);
            for tick in 1..=LADDER_DISMOUNT_TICKS { let feet = hoist_path(top, landing, kind.size.height, tick as f64 / LADDER_DISMOUNT_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Mantle, facing, hold: -1 }); }
            ways.push(Outing { trip: Trip { owner: body.species.clone(), from: now + 1, steps, pitches: Vec::new(), ladder: Some(ladder.owner.clone()), ending: Arrival::Perch, landing: crown.surface.clone(), grip: body.grip }, raise: None, stands: 0, rope: None });
        }
    }
    if let Some(ground) = draft.stage.perches.iter().find(|perch| perch.surface == ladder.surface && perch.y == ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1) {
        if let Some(below) = room_on(ground, kind.size.width / 2.0) {
            let mut steps = Vec::new();
            rungs(&stand, travel, 0.0, &mut steps);
            let off = clamp(ladder.foot.x, below[0], below[1]);
            if off != ladder.foot.x { for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { off } else { ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); } }
            if steps.is_empty() { steps.push(Foothold { x: body.x, y: body.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
            let way = Outing { trip: Trip { owner: body.species.clone(), from: now + 1, steps, pitches: Vec::new(), ladder: Some(ladder.owner.clone()), ending: Arrival::Perch, landing: ground.surface.clone(), grip: body.grip }, raise: None, stands: 0, rope: None };
            if !ways.is_empty() && travel < exit - travel { ways.insert(0, way); } else { ways.push(way); }
        }
    }
    ways
}

/// 🛝️ Plan a wall slide to the first lower perch or the end of the wall.
pub(super) fn slide_outing(draft: &Draft<'_>, index: usize, grip: f64, now: Ticks) -> Option<Outing> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let pitch = body.pitch.as_ref()?;
    let bottom = foot_of(pitch, kind.size);
    let below = landing_of(&draft.stage.perches, body.x, body.y, bottom);
    let floor = below.map_or(bottom, |below| below.y);
    let mut steps = Vec::new();
    let (mut y, mut vy) = (body.y, 0.0);
    let facing = wallward(pitch.side);
    for _ in 0..4096 {
        if y == floor { break; }
        let slide = slide_step(y, vy, floor);
        (y, vy) = (slide.y, slide.vy);
        steps.push(Foothold { x: body.x, y, footing: Footing::Wall, perch: None, activity: Activity::Slide, facing, hold: 0 });
    }
    let (mut ending, mut landing) = (Arrival::Air, String::new());
    if let Some((below, room)) = below.and_then(|below| room_on(below, kind.size.width / 2.0).map(|room| (below, room))) {
        let to = clamp(body.x, room[0], room[1]);
        if to != body.x { for tick in 1..=WALL_GRAB_TICKS { steps.push(Foothold { x: if tick == WALL_GRAB_TICKS { to } else { body.x + (to - body.x) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64) }, y: below.y, footing: Footing::Wall, perch: None, activity: Activity::Slide, facing, hold: 0 }); } }
        ending = Arrival::Perch;
        landing = below.surface.clone();
    }
    if steps.is_empty() { return None; }
    let cost: f64 = steps.iter().map(effort_of).sum();
    Some(Outing { trip: Trip { owner: body.species.clone(), from: now + 1, steps, pitches: vec![pitch.clone()], ladder: None, ending, landing, grip: larger(grip - cost, 0.0) }, raise: None, stands: 0, rope: None })
}

/// 👟️ A trip's walk or carry aims at the final consecutive ground foothold.
pub(super) fn walk_end(trip: &Trip, at: usize) -> f64 {
    let mut goal = trip.steps[at].x;
    for step in &trip.steps[at + 1..] { if step.footing != Footing::Perch || !matches!(step.activity, Activity::Walk | Activity::Carry) { break; } goal = step.x; }
    goal
}

/// ⛵️ Commit a gear outing only after its entire claimed corridor is clear.
pub(super) fn embark(draft: &mut Draft<'_>, index: usize, outing: Outing, now: Ticks) -> bool {
    let kind = draft.kinds[index];
    let trip = &outing.trip;
    let owner = draft.stage.actors[index].species.clone();
    let half = kind.size.width / 2.0;
    if trip.steps.iter().any(|step| step.x - half < 0.0 || step.x + half > draft.stage.width || step.y - kind.size.height < 0.0 || step.y > draft.stage.height) { return false; }
    let end = trip.from + trip.steps.len() as Ticks - 1;
    if outing.raise.is_some() {
        if draft.stage.ladders.len() >= 2 || draft.stage.ladders.iter().any(|ladder| ladder.owner == owner) { return false; }
    } else if let Some(ladder) = &trip.ladder {
        if !draft.stage.ladders.iter().any(|entry| &entry.owner == ladder && (entry.rider.is_none() || entry.rider.as_deref() == Some(&owner)) && entry.since <= now && entry.since + LADDER_LIFE >= end + LADDER_IDLE) { return false; }
    }
    let extents: Vec<_> = trip.steps.iter().map(|step| body_at(kind, trip, step)).collect();
    let claim = claim_of(&owner, trip.from, &extents, CLAIM_SPAN, if trip.ending == Arrival::Air { None } else { extents.last().copied() });
    if !claim_clear(&claim, &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims) { return false; }
    release(draft, index, now);
    if draft.stage.actors[index].footing == Footing::Wall { draft.stage.actors[index].grip = grip_now(&draft.stage.actors[index], now); }
    if let Some(raise) = outing.raise { draft.stage.ladders.push(Ladder { owner: owner.clone(), wall: raise.wall, surface: raise.surface, side: raise.side, foot: raise.foot, top: raise.top, since: outing.stands, until: std::cmp::min(outing.stands + LADDER_LIFE, end + LADDER_IDLE), rider: Some(owner.clone()) }); }
    else if let Some(ladder) = &trip.ladder { for entry in &mut draft.stage.ladders { if &entry.owner == ladder { entry.until = std::cmp::min(entry.since + LADDER_LIFE, std::cmp::max(entry.until, end + LADDER_IDLE)); entry.rider = Some(owner.clone()); } } }
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.claims.push(claim);
    draft.stage.courses.retain(|course| course.owner != owner);
    draft.stage.trips.retain(|trip| trip.owner != owner);
    let first = &trip.steps[0];
    let goal = if first.footing == Footing::Perch && matches!(first.activity, Activity::Walk | Activity::Carry) { walk_end(trip, 0) } else { draft.stage.actors[index].x };
    let body = &mut draft.stage.actors[index];
    body.rope = outing.rope;
    body.until = end;
    body.goal = goal;
    turn(body, first.facing, now);
    draft.stage.trips.push(outing.trip);
    true
}

/// 🚩️ Try every venture from a drawn first choice, taking its first legal outing.
pub fn set_out(draft: &mut Draft<'_>, index: usize, ventures: &[Venture], pick: f64, rest: &[f64], now: Ticks) -> bool {
    if ventures.is_empty() { return false; }
    let first = std::cmp::min((pick * ventures.len() as f64).floor() as usize, ventures.len() - 1);
    for turn in 0..ventures.len() { for outing in outings_to(draft, index, &ventures[(first + turn) % ventures.len()], rest, now) { if embark(draft, index, outing, now) { return true; } } }
    false
}
//#endregion 🔖️Gear
