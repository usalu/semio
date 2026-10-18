//! ⚔️ Conflict records: the conflict record, the command touch, the history transition codes, the recent touch memory and the concurrent write grading. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Conflict
/// ⚔️ One detected write/write intersection between the currently-executing operation and
/// an earlier operation it did not declare as a `dependency`. Detected directly off
/// `db_state::TouchedRegion::path_intersects` (see `DocumentState::apply_entries`). Resolution
/// policy is last-writer-wins (the conflicting write still applies) — recorded for the caller's
/// visibility only, never rejected, since every commit in a single-document-actor pipeline is
/// already fully serialized by the time this runs. Kept a local, path-granular shape (rather than
/// `db_conflict::ConflictRecord`, which reports per conflicting COMMAND PAIR) because it is
/// `db_engine`'s own frozen `CommandReceipt.conflicts` element type — see module doc.
/// `preview_conflicts` (below) is the real, additive `db_conflict::ConflictDetector` integration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictRecord {
    pub command_id: protocol::MutationId,
    pub conflicting_with: protocol::MutationId,
    pub path: String,
}

/// ⚔️ Builds the `db_conflict::CommandTouch` `envelope` would produce, without applying it —
/// shared by `submit`'s outcome-step gate and `preview_conflicts`'s advisory `db_conflict::
/// ConflictDetector` use. No per-operation conflict-declaration tag anymore (C10 deleted the CRDT-era
/// vocabulary): a `CommandTouch` carries only what `db_conflict::ConflictDetector` actually needs to detect —
/// identity, kind, timestamp, and the regions it touched.
// 🚫️async: E1 pure accessor consumed by a sync Iterator::map closure — see R9
pub fn command_touch(envelope: &protocol::MutationEnvelope, touched: &db_state::TouchedSet) -> db_conflict::CommandTouch {
    let kind = if supersedes_inputs(envelope) { SUPERSEDE_TOUCH_KIND } else { envelope.diff.schema.0.as_str() };
    let touch = db_conflict::CommandTouch::new(envelope.mutation_id.clone(), envelope.actor.clone(), db_conflict::CommandKind::from(kind), envelope.timestamp);
    touched.regions.iter().fold(touch, |touch, region| touch.touch(region.clone()))
}

/// ✏️ Whether `envelope` is a `Supersede` — the one history transition that writes document fields: it replaces the
/// effective input of committed operations, so it is graded by its declared `target` like any other write
/// (📋️ `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §2, §9.1).
fn supersedes_inputs(envelope: &protocol::MutationEnvelope) -> bool {
    matches!(protocol::history_transition_from_envelope(envelope), Ok(Some(protocol::HistoryTransition::Supersede(_))))
}

/// ⚖️ Grades one `db_conflict::ConflictRecord` into the `protocol::MutationMessage` the
/// outcome step judges against `options.policy` (contract §C9: "region intersection = `Warning`;
/// constraint violation = `Fatal`") — `db_conflict` deliberately never grades its own findings (see
/// its module doc), so `db_artifact`, the first crate below it that actually decides what to DO
/// about a conflict, is where that grading belongs. `TouchedRegion` still lands (last-writer-wins,
/// see `🔖️Conflict`'s doc) so it reads as an adjusted-but-applied write (`mutation.clamped`); a
/// violated `Constraint` reads as a broken structural invariant (`mutation.invariant`) — the two
/// `Warning`/`Fatal` codes from the frozen nine that fit each shape.
async fn grade_conflict_record(record: &db_conflict::ConflictRecord) -> protocol::MutationMessage {
    match &record.kind {
        db_conflict::ConflictKind::TouchedRegion(regions) => {
            let target: Vec<String> = regions.iter().map(|region| region.path.clone()).collect();
            protocol::MutationMessage::warning("mutation.clamped", format!("command {} touches region(s) also touched by concurrent command {}", record.command_id.0, record.conflicting_with.0)).at(target)
        }
        db_conflict::ConflictKind::Constraint(description) => {
            protocol::MutationMessage::fatal("mutation.invariant", format!("command {} violates constraint '{description}' held by concurrent command {}", record.command_id.0, record.conflicting_with.0)).at([description.clone()])
        }
    }
}

/// 🏷️ `CommandTouch` kind of a durable group decision's marker in the recent commit window: it names
/// the decision's edit so a later write can say it observed it, and it is never itself graded.
const DURABLE_GROUP_TOUCH_KIND: &str = "db.durable-group";

/// 🏷️ `CommandTouch` kind of a committed `Supersede` ([`supersedes_inputs`]): unlike every other history transition it
/// writes the fields its declared target names, so later writes are graded against it and it against them.
const SUPERSEDE_TOUCH_KIND: &str = "db.supersede";

/// 🪪️ The code of the refusal the event log answers a `Revert`/`Reinstate` naming an operation another actor authored
/// (`ArtifactEngine::submit`); a client localizes the refusal by this code, never by the message prose.
pub const FOREIGN_HISTORY_TRANSITION_CODE: &str = "history.foreign-transition";

/// 🗣️ The message of that refusal (developer detail; the UI text is the localized label of its code).
pub const FOREIGN_HISTORY_TRANSITION_MESSAGE: &str = "an undo or redo may only name operations its own author wrote";

/// 🧩️ The code of the refusal a history transition gets when its payload does not decode, or when a `Supersede` does
/// not declare exactly its targets as its dependencies: every replica would refuse it, so the log never holds it.
pub const MALFORMED_HISTORY_TRANSITION_CODE: &str = "history.malformed-transition";

/// 🗣️ The message of that refusal.
pub const MALFORMED_HISTORY_TRANSITION_MESSAGE: &str = "a history transition must decode and a supersession must depend on exactly its targets";

/// 👻️ The code of the refusal a `Supersede` gets when it names an operation neither committed, nor earlier in its batch,
/// nor folded by a durable group decision — the event log's twin of the fold's unknown-target error.
pub const UNKNOWN_SUPERSEDE_TARGET_CODE: &str = "history.unknown-target";

/// 🗣️ The message of that refusal.
pub const UNKNOWN_SUPERSEDE_TARGET_MESSAGE: &str = "a supersession may only name operations the document's log holds";

/// 🧷️ The recent-window marker of one committed durable group decision (see [`DURABLE_GROUP_TOUCH_KIND`]).
pub fn durable_group_touch(edit_id: &str) -> db_conflict::CommandTouch {
    db_conflict::CommandTouch::new(protocol::MutationId(edit_id.to_string()), protocol::ActorId(String::new()), db_conflict::CommandKind::from(DURABLE_GROUP_TOUCH_KIND), protocol::HybridLogicalTimestamp::new(0, 0))
}

/// 🪟️ Appends one committed touch to the bounded recent commit window, oldest out first.
pub fn remember_recent_touch(window: &mut VecDeque<db_conflict::CommandTouch>, touch: db_conflict::CommandTouch) {
    if window.len() >= MAX_RECENT_TOUCHES {
        window.pop_front();
    }
    window.push_back(touch);
}

/// 👁️ Whether `observed` names `touch`: its command itself, or — for a durable group decision's marker —
/// one of the operations its edit folded (`<edit>#<position>`), which reach a replica inside that edit.
fn touch_is_named(touch: &db_conflict::CommandTouch, observed: &str) -> bool {
    observed == touch.command_id.0 || (touch.kind.0 == DURABLE_GROUP_TOUCH_KIND && observed.strip_prefix(touch.command_id.0.as_str()).is_some_and(|position| position.starts_with('#')))
}

/// ✍️ Whether `touch` wrote document fields: every operation and every `Supersede` did; the other history
/// transitions (undo, redo, checkpoints) and durable group markers never did.
fn touch_writes_fields(touch: &db_conflict::CommandTouch) -> bool {
    touch.kind.0 != DURABLE_GROUP_TOUCH_KIND && touch.kind.0 != protocol::HISTORY_TRANSITION_SCHEMA
}

/// 🕰️ The writes `envelope` (whose own touch is `written`) was authored without seeing: in commit order (the recent
/// commit window, then the batch's earlier envelopes), every write after the one it names as `observed` — all of them
/// when it observed nothing in the window — by another actor. A replica stamps each operation it authors with the
/// newest foreign operation it had applied, and replicas receive commits in commit order, so everything up to that one
/// was seen; the author's own writes are its own history. Only a field write is graded: a `Supersede` is, the other
/// history transitions never are.
pub fn unseen_concurrent_writes<'a>(recent: &'a VecDeque<db_conflict::CommandTouch>, batch: &'a [db_conflict::CommandTouch], written: &db_conflict::CommandTouch, envelope: &protocol::MutationEnvelope) -> Vec<&'a db_conflict::CommandTouch> {
    if !touch_writes_fields(written) {
        return Vec::new();
    }
    let window: Vec<&db_conflict::CommandTouch> = recent.iter().chain(batch.iter()).collect();
    let seen = envelope.observed.as_ref().and_then(|observed| window.iter().rposition(|touch| touch_is_named(touch, &observed.0))).map_or(0, |index| index + 1);
    window.into_iter().skip(seen).filter(|touch| touch.actor != envelope.actor && touch_writes_fields(touch)).collect()
}

/// 🎯️ Whether two declared targets (outermost segment first) can address the same part: an empty target is
/// the whole artifact; otherwise they overlap when they share a segment — conservative for path-shaped targets
/// (`[collection, id]`) and exact for id-set targets.
fn targets_overlap(a: &[String], b: &[String]) -> bool {
    a.is_empty() || b.is_empty() || a.iter().any(|segment| b.contains(segment))
}

/// ⚖️ Grades `written` — authored without seeing the concurrent write `unseen` — into `mutation.clamped`
/// (contract §C9: region intersection = `Warning`): fields both touch when the database can read both diffs, else
/// their declared targets overlapping. Disjoint parts are no conflict. `Normal` accepts a clamped write with its
/// message, `Vigilant` refuses it (ticket 26/09/23 C10: a vigilant hub accepted a same-field write authored
/// during a 12 s cut, because no plugin diff is readable here and no client said what it had seen).
pub async fn grade_concurrent_write(written: &db_conflict::CommandTouch, written_target: &[String], unseen: &db_conflict::CommandTouch, unseen_target: &[String]) -> Vec<protocol::MutationMessage> {
    if !written.touched.regions.is_empty() && !unseen.touched.regions.is_empty() {
        let mut messages = Vec::new();
        for record in db_conflict::ConflictDetector::new().detect(&[unseen.clone(), written.clone()]) {
            messages.push(grade_conflict_record(&record).await);
        }
        return messages;
    }
    if !targets_overlap(written_target, unseen_target) {
        return Vec::new();
    }
    vec![protocol::MutationMessage::warning("mutation.clamped", format!("command {} was authored without seeing concurrent command {}, which writes the same part of the document", written.command_id.0, unseen.command_id.0)).at(written_target.to_vec())]
}
//#endregion 🔖️Conflict
//#endregion 🔖️Ids
