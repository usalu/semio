//! 🚪️ Leaving: an actor that has nowhere left to be walks off its perch. See [`super`].

use super::*;

//#region 🔖️Leaving
/// 👋️ An actor starts to leave: it lets go of its partner and walks to the nearer end of its perch when that is within three of its widths and nobody stands in the way, then fades; farther away, hemmed in or in the air, it fades where it is.
pub(crate) fn leave(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    release(draft, index, now);
    draft.stage.actors[index].leaving = true;
    if draft.stage.actors[index].footing != Footing::Perch || draft.stage.actors[index].perch.is_none() { return; }
    halt(draft, index, now);
    let body = &draft.stage.actors[index];
    let Some(surface) = body.perch.as_deref() else {
        return;
    };
    let perch = perch_at(&draft.stage.perches, surface, body.x);
    let half = kind.size.width / 2.0;
    let low = perch.map_or(body.x, |perch| perch.x0 + half);
    let high = perch.map_or(body.x, |perch| perch.x1 - half);
    let end = if body.x - low <= high - body.x { low } else { high };
    let [clear_low, clear_high] = clearway(draft, index, MEET_GAP, None);
    let clip = clip_at(kind, Activity::Walk, 0.0);
    let goal = paced(kind, clip.as_deref(), body.x, end);
    if goal != body.x && end >= clear_low && end <= clear_high && (end - body.x).abs() <= LEAVE_REACH * kind.size.width {
        stroll(draft, index, goal, clip, now);
        return;
    }
    shift(draft, index, Activity::Idle, now);
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(kind, Activity::Idle, 0.0);
    body.goal = body.x;
}
//#endregion 🔖️Leaving
