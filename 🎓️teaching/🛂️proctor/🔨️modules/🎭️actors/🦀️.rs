//! 🎭️ The quiz lifecycle as framework actors: the roster (one actor holding the handle index,
//! serving `identify-learner`) and one learner actor per learner (runs, answers, submissions,
//! badges). Both wrap the quiz core's pure `decide_*`/`evolve_*` without adding domain rules; the
//! actor state bytes are the serde JSON of the core's `RosterState`/`LearnerState`.
//!
//! **Enrollment.** A learner actor only knows a learner once `learner-registered` is in its own
//! stream, but identification is decided by the roster. [`EnrollmentSaga`] relays every committed
//! roster fact about a learner (`learner-registered`, `learner-recalled`) to that learner's actor as
//! a proctor-internal `quiz.enroll-learner` command, issued as the `proctor` service account and
//! deduplicated by the roster event it relays; the learner actor appends the fact unless it is
//! already registered. The learner stream therefore holds the learner's complete history.
//!
//! **Wire mapping** (design §9a): command kind `quiz.<type>`, version `1`, command id and
//! idempotency key = the quiz command id, tenant and scope = the catalog id, target
//! `quiz-roster/roster` for `identify-learner` and `quiz-learner/<learner>` otherwise; event kind
//! `quiz.<type>` with the quiz event JSON as payload; a quiz rejection is the framework
//! `invalid` rejection whose `detail` is the rejection string.
//!
//! @see ../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/🧾️lifecycle/🦀️.rs — decide/evolve
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs — `Decider`, `Saga`

use std::sync::Arc;

use quiz::{Command, Event, LearnerContext, LearnerState, RosterState};
use semio_framework_dispatch_macros::dyn_enum_close;
use server::authority::{ActorState, Decider, Decision, DecisionContext, Saga};
use server::contract::{ActorKey, CommandEnvelope, CommandId, EventRecord, IdempotencyKey, Principal, Rejection, Scope, TenantId, TraceContext};
use server::__semio_dispatch_Decider;

use crate::catalog::LoadedCatalog;

/// 📇️ The actor kind of the roster.
pub const ROSTER: &str = "quiz-roster";
/// 🧑‍🎓️ The actor kind of a learner.
pub const LEARNER: &str = "quiz-learner";
/// 🆔️ The id of the one roster actor.
pub const ROSTER_ID: &str = "roster";
/// 📨️ The proctor-internal command relaying a roster fact to its learner.
pub const ENROLL: &str = "quiz.enroll-learner";
/// 🤖️ The service account the proctor's own workflows act as.
pub const PROCTOR_SERVICE: &str = "proctor";
/// 🔢️ The wire version of every quiz command and query kind.
pub const WIRE_VERSION: u32 = 1;

/// 🏷️ `quiz.<type>` of a quiz command.
pub fn command_kind(command: &Command) -> String {
    format!("quiz.{}", command.type_name())
}

/// 🏷️ `quiz.<type>` of a quiz event.
pub fn event_kind(event: &Event) -> String {
    format!("quiz.{}", event.type_name())
}

/// 📇️ The roster actor of a catalog.
pub fn roster_key(tenant: &str) -> ActorKey {
    ActorKey { tenant: TenantId(tenant.to_string()), kind: ROSTER.to_string(), id: ROSTER_ID.to_string() }
}

/// 🧑‍🎓️ The actor of one learner of a catalog.
pub fn learner_key(tenant: &str, learner: &str) -> ActorKey {
    ActorKey { tenant: TenantId(tenant.to_string()), kind: LEARNER.to_string(), id: learner.to_string() }
}

/// 🎯️ The actor a quiz command is serialized through.
pub fn command_target(command: &Command, tenant: &str) -> ActorKey {
    match command {
        Command::IdentifyLearner { .. } => roster_key(tenant),
        _ => learner_key(tenant, command.learner()),
    }
}

/// 🚫️ The framework rejection carrying a quiz rejection.
pub fn rejection(rejection: quiz::Rejection) -> Rejection {
    Rejection::Invalid { detail: rejection.as_str().to_string() }
}

//#region 🔖️Deciders
/// 📇️ The roster: identifies learners against the handle index.
pub struct RosterDecider {
    pub tenant: String,
}

/// 🧑‍🎓️ One learner's runs, answers, submissions and badges, decided against the loaded catalog.
pub struct LearnerDecider {
    pub catalog: Arc<LoadedCatalog>,
}

impl Decider for RosterDecider {
    async fn actor_kind(&self) -> &str {
        ROSTER
    }

    async fn decide(&self, state: &ActorState, envelope: &CommandEnvelope, context: &DecisionContext) -> Decision {
        match admitted(envelope, &self.tenant) {
            Ok(command) => emitted(envelope, context, quiz::decide_roster(&decoded(state).unwrap_or_default(), &command, context.now.millis)),
            Err(refusal) => Decision::Reject(refusal),
        }
    }

    async fn evolve(&self, state: &mut ActorState, event: &EventRecord) {
        let Ok(fact) = serde_json::from_slice::<Event>(&event.payload) else { return };
        let mut roster: RosterState = decoded(state).unwrap_or_default();
        quiz::evolve_roster(&mut roster, &fact);
        state.bytes = encoded(&roster);
    }
}

impl Decider for LearnerDecider {
    async fn actor_kind(&self) -> &str {
        LEARNER
    }

    async fn decide(&self, state: &ActorState, envelope: &CommandEnvelope, context: &DecisionContext) -> Decision {
        let learner = learner_state(state, &envelope.target.id);
        if envelope.kind == ENROLL {
            return enroll(&learner, envelope, context, self.catalog.id());
        }
        match admitted(envelope, self.catalog.id()) {
            Ok(command) => {
                let lifecycle = LearnerContext { now: context.now.millis, catalog: &self.catalog.catalog, quizzes: self.catalog.current() };
                emitted(envelope, context, quiz::decide_learner(&learner, &command, &lifecycle))
            }
            Err(refusal) => Decision::Reject(refusal),
        }
    }

    async fn evolve(&self, state: &mut ActorState, event: &EventRecord) {
        let Ok(fact) = serde_json::from_slice::<Event>(&event.payload) else { return };
        let mut learner = learner_state(state, &event.stream.id);
        quiz::evolve_learner(&mut learner, &fact);
        state.bytes = encoded(&learner);
    }
}

dyn_enum_close! {
    /// ⚖️ The proctor's closed decider set.
    pub enum ProctorDeciders: Decider {
        Roster(RosterDecider),
        Learner(LearnerDecider),
    }
}

/// 🧠️ The learner state behind an actor, or the empty state of `learner`.
pub fn learner_state(state: &ActorState, learner: &str) -> LearnerState {
    decoded(state).unwrap_or_else(|| quiz::empty_learner_state(learner))
}

fn decoded<T: serde::de::DeserializeOwned>(state: &ActorState) -> Option<T> {
    serde_json::from_slice(&state.bytes).ok()
}

fn encoded<T: serde::Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

fn mismatch(detail: impl std::fmt::Display) -> Rejection {
    Rejection::Invalid { detail: format!("envelope-mismatch: {detail}") }
}

/// 🛃️ The quiz command inside an envelope, once the envelope agrees with it (design §9a).
pub fn admitted(envelope: &CommandEnvelope, tenant: &str) -> Result<Command, Rejection> {
    if envelope.version != WIRE_VERSION {
        return Err(mismatch(format!("version {} is not {WIRE_VERSION}", envelope.version)));
    }
    if envelope.target.tenant.0 != tenant || envelope.scope.0 != tenant {
        return Err(mismatch(format!("tenant and scope must both be the catalog id {tenant:?}")));
    }
    let command: Command = serde_json::from_slice(&envelope.payload).map_err(|error| Rejection::Invalid { detail: format!("command-malformed: {error}") })?;
    if envelope.kind != command_kind(&command) {
        return Err(mismatch(format!("kind {:?} does not carry a {} command", envelope.kind, command.type_name())));
    }
    if envelope.command_id.0 != *command.id() || envelope.idempotency_key.as_ref().map(|key| &key.0) != Some(command.id()) {
        return Err(mismatch("commandId and idempotencyKey must both be the command id"));
    }
    if envelope.target != command_target(&command, tenant) {
        return Err(mismatch(format!("target must be {}", describe(&command_target(&command, tenant)))));
    }
    Ok(command)
}

fn describe(actor: &ActorKey) -> String {
    format!("{}/{}/{}", actor.tenant.0, actor.kind, actor.id)
}

fn emitted(envelope: &CommandEnvelope, context: &DecisionContext, decision: quiz::Decision) -> Decision {
    match decision {
        quiz::Decision::Events(events) => Decision::Emit { events: events.iter().map(|fact| record(&envelope.target, context, fact)).collect(), effects: Vec::new() },
        quiz::Decision::Rejection(refused) => Decision::Reject(rejection(refused)),
    }
}

fn record(target: &ActorKey, context: &DecisionContext, fact: &Event) -> EventRecord {
    EventRecord { stream: target.clone(), seq: 0, hlc: context.now, kind: event_kind(fact), payload: encoded(fact) }
}

fn enroll(learner: &LearnerState, envelope: &CommandEnvelope, context: &DecisionContext, tenant: &str) -> Decision {
    if envelope.principal != (Principal::ServiceAccount { id: PROCTOR_SERVICE.to_string() }) {
        return Decision::Reject(Rejection::Unauthorized { detail: format!("{ENROLL} is issued by the proctor only") });
    }
    if envelope.target.tenant.0 != tenant || envelope.scope.0 != tenant || envelope.target.kind != LEARNER {
        return Decision::Reject(mismatch(format!("{ENROLL} must address a learner of catalog {tenant:?}")));
    }
    let Ok(fact) = serde_json::from_slice::<Event>(&envelope.payload) else {
        return Decision::Reject(Rejection::Invalid { detail: format!("{ENROLL} carries no quiz event") });
    };
    match &fact {
        Event::LearnerRegistered { learner: id, .. } if *id == envelope.target.id => Decision::Emit { events: if learner.identity.is_some() { Vec::new() } else { vec![record(&envelope.target, context, &fact)] }, effects: Vec::new() },
        Event::LearnerRecalled { learner: id, .. } if *id == envelope.target.id => Decision::Emit { events: vec![record(&envelope.target, context, &fact)], effects: Vec::new() },
        _ => Decision::Reject(mismatch(format!("{ENROLL} relays a registration or recall of {}", envelope.target.id))),
    }
}
//#endregion 🔖️Deciders

//#region 🔖️Saga
/// 🧵️ Relays every committed roster fact about a learner to that learner's actor.
pub struct EnrollmentSaga;

impl Saga for EnrollmentSaga {
    async fn on_event(&self, event: &EventRecord) -> Vec<CommandEnvelope> {
        enrollment(event).into_iter().collect()
    }
}

/// 📨️ The enrollment command relaying one roster event, if it is a fact about a learner.
pub fn enrollment(event: &EventRecord) -> Option<CommandEnvelope> {
    if event.stream.kind != ROSTER {
        return None;
    }
    let fact = serde_json::from_slice::<Event>(&event.payload).ok()?;
    let (Event::LearnerRegistered { learner, .. } | Event::LearnerRecalled { learner, .. }) = &fact else { return None };
    let tenant = &event.stream.tenant.0;
    let key = format!("enroll:{tenant}:{}:{}", event.stream.id, event.seq);
    Some(CommandEnvelope {
        command_id: CommandId(key.clone()),
        kind: ENROLL.to_string(),
        version: WIRE_VERSION,
        target: learner_key(tenant, learner),
        scope: Scope(tenant.clone()),
        principal: Principal::ServiceAccount { id: PROCTOR_SERVICE.to_string() },
        session: None,
        device: None,
        payload: event.payload.clone(),
        causal_frontier: None,
        client_hlc: event.hlc,
        expected_revision: None,
        idempotency_key: Some(IdempotencyKey(key)),
        capability_proof: None,
        trace: TraceContext::default(),
    })
}
//#endregion 🔖️Saga

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
