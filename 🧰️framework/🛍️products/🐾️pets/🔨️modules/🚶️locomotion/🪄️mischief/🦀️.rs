//! 🪄️ Mischief: reaching a post on foot or with gear, slipping to a distant post and pushing from it. See [`super`].

use super::*;
use crate::schedule::Post;
use semio_framework_pets_contract::schema::Arrival;
use semio_framework_pets_contract::schema::Facing;
use crate::draft::{index_of, turn};
use crate::spacing::{bodies_of, extent_at, obstacles_for, wall_lean_of};
use semio_framework_pets_clearance::{free_among, grown, meets};
use semio_framework_pets_interaction::mischief::LIFT_TICKS;
use semio_framework_pets_traversal::climbing::{GRIP_HANG, cling_of};
use semio_framework_pets_contract::schema::{Foothold, Trip};

//#region 🔖️Mischief
/// 📬️ Reach a post on foot or with gear, then use a guarded poof only as the last resort.
pub fn post_to(draft: &mut Draft<'_>, index: usize, post: &Post, side: Facing, span: Ticks, slip: bool, now: Ticks) -> Option<Ticks> {
    let body = &draft.stage.actors[index];
    match post {
        Post::Wall { pitch, y } => {
            for outing in outings_to(draft, index, &Venture::Wall { pitch: pitch.clone(), y: Some(*y) }, &[0.0, 0.0, 1.0], now) {
                let arrives = outing.trip.from + outing.trip.steps.len() as Ticks - 1;
                if outing.trip.grip >= GRIP_HANG * span as f64 && embark(draft, index, outing, now) { return Some(arrives); }
            }
        }
        Post::Perch { perch, x } if body.footing == Footing::Perch && body.perch.as_deref() == Some(&perch.surface) => {
            let stand = Stand { perch: perch.clone(), x: body.x, y: body.y, facing: body.facing, grip: body.grip };
            if let Some(steps) = walked(&stand, draft.kinds[index], *x, Activity::Walk) {
                if steps.is_empty() { return Some(now); }
                let trip = Trip { owner: body.species.clone(), from: now + 1, steps, pitches: Vec::new(), ladder: None, ending: Arrival::Perch, landing: perch.surface.clone(), grip: body.grip };
                let arrives = trip.from + trip.steps.len() as Ticks - 1;
                if embark(draft, index, Outing { trip, raise: None, stands: 0, rope: None }, now) { return Some(arrives); }
            }
        }
        _ => {}
    }
    (slip && pop_to(draft, index, post, side, span, now)).then_some(now)
}

/// 💫️ Slip to a distant post only where the entire body and comfortable gap are free.
fn pop_to(draft: &mut Draft<'_>, index: usize, post: &Post, side: Facing, span: Ticks, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let (feet, pitch) = match post {
        Post::Wall { pitch, y } => (Foothold { x: cling_of(pitch, kind.size), y: *y, footing: Footing::Wall, perch: None, activity: Activity::Climb, facing: side, hold: 0 }, Some(pitch)),
        Post::Perch { perch, x } => (Foothold { x: *x, y: perch.y - hover_of(kind), footing: Footing::Perch, perch: Some(perch.surface.clone()), activity: Activity::Idle, facing: side, hold: -1 }, None),
    };
    let extent = extent_at(&body.species, kind, Point { x: feet.x, y: feet.y }, wall_lean_of(kind, feet.footing, pitch, feet.x), None);
    let room = grown(extent, COMFORT_GAP);
    let half = kind.size.width / 2.0;
    if feet.x - half < 0.0 || feet.x + half > draft.stage.width || feet.y - kind.size.height < 0.0 || feet.y > draft.stage.height || !free_among(extent, &obstacles_for(draft, index, now)) || bodies_of(&draft.stage.actors, &draft.kinds).iter().any(|other| other.owner != body.species && meets(room, other.extent)) { return false; }
    dusted(draft, index, now);
    release(draft, index, now);
    ungear(draft, index, now);
    let body = &mut draft.stage.actors[index];
    body.x = feet.x;
    body.y = feet.y;
    body.goal = feet.x;
    body.vx = 0.0;
    body.vy = 0.0;
    body.tilt = 0.0;
    body.opacity = 0.0;
    body.facing = side;
    body.faced = now;
    body.host = None;
    if let Post::Wall { pitch, .. } = post {
        body.footing = Footing::Wall;
        body.perch = None;
        body.pitch = Some(pitch.clone());
        rest(draft, index, span, now);
        draft.stage.actors[index].until = now + span;
    } else {
        body.footing = Footing::Perch;
        body.perch = feet.perch;
        settle(draft, index, now);
    }
    true
}

/// 🤜️ A ready actor pushes from its post while retaining enough grip to finish.
pub fn push_at(draft: &mut Draft<'_>, index: usize, side: Facing, until: Ticks, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    if body.leaving || body.partner.is_some() || trip_of(&draft.stage.trips, &body.species).is_some() { return false; }
    let clinging = body.footing == Footing::Wall && body.activity == Activity::Climb && body.pitch.is_some();
    if !clinging && !(body.footing == Footing::Perch && body.activity == Activity::Idle) { return false; }
    if clinging { let grip = grip_now(body, now); if grip < GRIP_HANG * (until - now) as f64 { return false; } draft.stage.actors[index].grip = grip; }
    shift(draft, index, Activity::Push, now);
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(draft.kinds[index], Activity::Push, 0.0);
    body.until = until;
    body.goal = body.x;
    body.vx = 0.0;
    body.vy = 0.0;
    turn(body, side, now);
    true
}

/// 🫷️ A stopped pusher returns to its perch rest or a short wall rest.
pub fn unpush(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    if body.activity != Activity::Push { return; }
    if body.footing == Footing::Wall && body.pitch.is_some() { rest(draft, index, 1, now); } else { settle(draft, index, now); }
}

/// 🧯️ End an unreclaimed prank quietly and start its cooldown.
pub fn unlifted(draft: &mut Draft<'_>, now: Ticks) {
    let Some(lift) = &draft.stage.lift else { return; };
    if now - lift.since > LIFT_TICKS { return; }
    let pusher = lift.pusher.clone();
    draft.stage.lift = None;
    draft.stage.rested = now;
    if let Some(index) = index_of(&draft.stage.actors, &pusher) { unpush(draft, index, now); }
}
//#endregion 🔖️Mischief
