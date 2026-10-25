//! ⚓️ Footing: a complete course and its reserved corridor, installing a validated course, vanishing, dropping and releasing a footing before planning the way down. See [`super`].

use super::*;
use semio_framework_pets_contract::schema::Ending;
use semio_framework_pets_contract::schema::Course;
use semio_framework_pets_contract::schema::Claim;
use crate::spacing::extent_at;
use semio_framework_pets_clearance::{DELAY, claim_of, released};

//#region 🔖️Footing
/// 🎟️ A complete course and its reserved corridor.
#[derive(Clone)]
pub struct Plan { pub(super) course: Course, pub(super) claim: Claim }

/// 🧾️ Find an owner's planned flight.
pub fn course_of<'a>(courses: &'a [Course], owner: &str) -> Option<&'a Course> {
    courses.iter().find(|course| course.owner == owner)
}

/// 🎫️ Claim every course waypoint and its resting body in four-tick slices.
pub(super) fn plan_of(draft: &Draft<'_>, index: usize, course: Course) -> Plan {
    let kind = draft.kinds[index];
    let extents: Vec<_> = course.steps.iter().map(|step| extent_at(&course.owner, kind, Point { x: step.x, y: step.y }, step.tilt, step.canopy.map(|canopy| Point { x: canopy.x, y: canopy.y }))).collect();
    let rest = if course.ending == Ending::Away { None } else { extents.last().copied() };
    let claim = claim_of(&course.owner, course.from, &extents, CLAIM_SPAN, rest);
    Plan { course, claim }
}

/// 🛫️ Installing a validated course reserves its corridor and commits the flight.
pub(super) fn install(draft: &mut Draft<'_>, index: usize, plan: Plan, activity: Activity, now: Ticks) {
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.claims.push(plan.claim);
    draft.stage.courses.retain(|course| course.owner != owner);
    let until = plan.course.from + plan.course.steps.len() as Ticks - 1;
    let goal = plan.course.steps.last().unwrap().x;
    draft.stage.courses.push(plan.course);
    if draft.stage.actors[index].activity != activity { shift(draft, index, activity, now); draft.stage.actors[index].clip = clip_at(draft.kinds[index], activity, 0.0); }
    let body = &mut draft.stage.actors[index];
    body.footing = if body.chute.is_none() { Footing::Air } else { Footing::Chute };
    body.perch = None;
    body.host = None;
    body.hang = None;
    body.until = until;
    body.goal = goal;
}

/// 🏡️ An actor that has nowhere left to be is gone at once (always `false`): it lets go of its partner and arrives anew, spaced from the others, once it is wanted and a perch has room.
pub fn vanish(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    release(draft, index, now);
    remove(draft, index);
    false
}

/// 🕳️ An actor loses its perch and starts to fall.
pub fn drop(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    unfoot(draft, index, 0.0, 0.0, Activity::Fall, now);
}

/// 🔓️ Release a footing, corridor and course before planning the way down.
pub fn unfoot(draft: &mut Draft<'_>, index: usize, vx: f64, vy: f64, activity: Activity, now: Ticks) {
    release(draft, index, now);
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.courses.retain(|course| course.owner != owner);
    ungear(draft, index, now);
    if draft.stage.actors[index].activity != activity {
        shift(draft, index, activity, now);
        draft.stage.actors[index].clip = clip_at(draft.kinds[index], activity, 0.0);
    }
    let body = &mut draft.stage.actors[index];
    body.perch = None;
    body.footing = if body.chute.is_none() { Footing::Air } else { Footing::Chute };
    body.host = None;
    body.hang = None;
    body.vx = vx;
    body.vy = vy;
    body.goal = body.x;
    body.until = now + DELAY;
}
//#endregion 🔖️Footing
