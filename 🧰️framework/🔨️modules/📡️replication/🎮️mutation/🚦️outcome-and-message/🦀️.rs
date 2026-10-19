//! 🚦️ Mutation outcomes and messages: the outcome codes, the message record and the mutation outcome with their value conversions. See [`super`].

use super::*;

//#region 🔖️Message
/// 🪙️ One code of the frozen outcome vocabulary. The only code type [`MutationOutcome::refuse`] accepts, so a guard
/// that picks its code at runtime can never name a code outside the table or at a level the table does not fix.
/// Language-agnostic twin: `🧪️tests/🧫️fixtures/🧫️outcome-code/🔣️.json` (schema `🧬️schema/🔣️outcome-code`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OutcomeCode {
    TargetMissing,
    TargetReferenced,
    TargetMismatch,
    NoOp,
    Partial,
    Clamped,
    PreconditionDrifted,
    DuplicateId,
    Invariant,
    InverseRefused,
    Cascade,
}

impl OutcomeCode {
    /// 📚️ Every vocabulary code, in the document's order.
    pub const ALL: [OutcomeCode; 11] = [Self::TargetMissing, Self::TargetReferenced, Self::TargetMismatch, Self::NoOp, Self::Partial, Self::Clamped, Self::PreconditionDrifted, Self::DuplicateId, Self::Invariant, Self::InverseRefused, Self::Cascade];

    /// 🔤️ The wire spelling of the code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetMissing => "mutation.target-missing",
            Self::TargetReferenced => "mutation.target-referenced",
            Self::TargetMismatch => "mutation.target-mismatch",
            Self::NoOp => "mutation.no-op",
            Self::Partial => "mutation.partial",
            Self::Clamped => "mutation.clamped",
            Self::PreconditionDrifted => "mutation.precondition-drifted",
            Self::DuplicateId => "mutation.duplicate-id",
            Self::Invariant => "mutation.invariant",
            Self::InverseRefused => "mutation.inverse-refused",
            Self::Cascade => "mutation.cascade",
        }
    }

    /// 🎚️ The one level the vocabulary fixes for the code.
    pub const fn level(self) -> semio_framework_diagnostic::Severity {
        match self {
            Self::TargetMissing | Self::TargetReferenced | Self::TargetMismatch => semio_framework_diagnostic::Severity::Error,
            Self::NoOp | Self::Partial | Self::Clamped | Self::PreconditionDrifted => semio_framework_diagnostic::Severity::Warning,
            Self::DuplicateId | Self::Invariant | Self::InverseRefused => semio_framework_diagnostic::Severity::Fatal,
            Self::Cascade => semio_framework_diagnostic::Severity::Info,
        }
    }
}

/// 🔡️ A vocabulary code converts to its wire spelling wherever a bare code string travels (validator errors, faults).
impl From<OutcomeCode> for &'static str {
    fn from(code: OutcomeCode) -> Self {
        code.as_str()
    }
}

/// 🎟️ A vocabulary code is a message code: any builder taking a code takes the typed one.
impl From<OutcomeCode> for semio_framework_diagnostic::FaultCode {
    fn from(code: OutcomeCode) -> Self {
        Self::from(code.as_str())
    }
}

/// 📖️ The frozen outcome-code vocabulary and the one level each code fixes, derived from [`OutcomeCode`] — the table
/// persistence validates against (`🏪️store` `expected_mutation_message_level`). Closed: no per-plugin codes.
pub const OUTCOME_CODES: [(&str, semio_framework_diagnostic::Severity); 11] = {
    let mut table = [("", semio_framework_diagnostic::Severity::Info); 11];
    let mut index = 0;
    while index < OutcomeCode::ALL.len() {
        table[index] = (OutcomeCode::ALL[index].as_str(), OutcomeCode::ALL[index].level());
        index += 1;
    }
    table
};

/// 🧱️ Prefix of the apply-time rejection family `mutation.apply.<kebab-detail>`, always `Fatal`.
pub const APPLY_OUTCOME_CODE_PREFIX: &str = "mutation.apply.";

/// ⚖️ The level the frozen vocabulary fixes for `code`; `None` for any code outside it.
pub fn outcome_code_level(code: &str) -> Option<semio_framework_diagnostic::Severity> {
    OUTCOME_CODES.iter().find(|(known, _)| *known == code).map(|(_, level)| *level).or_else(|| {
        code.strip_prefix(APPLY_OUTCOME_CODE_PREFIX)
            .filter(|detail| detail.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())))
            .map(|_| semio_framework_diagnostic::Severity::Fatal)
    })
}

/// 📨️ One outcome-carried diagnostic from a `Mutation`/`MutationKind::diff` — the level
/// vocabulary is [`crate::diagnostic::Severity`] (`Info < Warning < Error < Fatal`, that declaration
/// order IS the level order via `derive(Ord)`); `code` is one of the frozen `mutation.*`
/// codes (`.🧬semio/🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/
/// 📋️contract-freeze.md` §C2, extended 2026-09-30 by the state-dependent `Error` codes `mutation.target-referenced` and
/// `mutation.target-mismatch`, 2026-10-05 by the `Warning` `mutation.precondition-drifted` and the `Fatal`
/// `mutation.inverse-refused` — closed set, no per-plugin codes, ever); `message` is English prose
/// (UI localizes by `code`, never by parsing `message`); `target` is the address of the offending
/// element (outermost segment first, matching [`MutationKind::target`]'s convention); `op_index` is
/// stamped by a batch replay ([`MutationOutcome::stamp_op_index`]) once this message's originating
/// op's position within the batch is known.
/// 🌱️ serde's derives are carried ALONGSIDE the hand-written `ToValue`/`FromValue` twin below —
/// the transitional state the serde-fanout playbook prescribes ("add alongside, do not blind-swap").
/// Attributes are restored verbatim from 67fb4216b2 so the serde wire shape stays byte-identical to
/// the twin. Drop them once every consumer in `🔗️causal`/`📡️wire`/`⚔️conflict` moves to `ToValue`.
#[derive(Clone, Debug, PartialEq)]
pub struct MutationMessage {
    pub level: semio_framework_diagnostic::Severity,
    pub code: semio_framework_diagnostic::FaultCode,
    pub message: String,
    pub target: Vec<String>,
    pub op_index: Option<u32>,
}

/// 🌉️ Hand-written, not derived: mirrors the `#[serde(default, skip_serializing_if = "…")]`
/// sparse-emission on `target`/`op_index` (only emitted when non-default), matching the
/// pre-existing serde wire shape byte-for-byte in field presence.
impl semio_framework_value::ToValue for MutationMessage {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("level".to_string(), semio_framework_value::ToValue::to_value(&self.level)),
            ("code".to_string(), semio_framework_value::ToValue::to_value(&self.code)),
            ("message".to_string(), semio_framework_value::ToValue::to_value(&self.message)),
        ];
        if !self.target.is_empty() {
            entries.push(("target".to_string(), semio_framework_value::ToValue::to_value(&self.target)));
        }
        if self.op_index.is_some() {
            entries.push(("opIndex".to_string(), semio_framework_value::ToValue::to_value(&self.op_index)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for MutationMessage {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationMessage, found {value:?}")));
        };
        let mut level = None;
        let mut code = None;
        let mut message = None;
        let mut target = Vec::new();
        let mut op_index = None;
        for (key, entry) in fields {
            match key.as_str() {
                "level" => level = Some(<semio_framework_diagnostic::Severity as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("level"))?),
                "code" => code = Some(<semio_framework_diagnostic::FaultCode as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("code"))?),
                "message" => message = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("message"))?),
                "target" => target = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                "opIndex" => op_index = <Option<u32> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("opIndex"))?,
                _ => {}
            }
        }
        Ok(MutationMessage {
            level: level.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMessage missing level"))?,
            code: code.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMessage missing code"))?,
            message: message.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMessage missing message"))?,
            target,
            op_index,
        })
    }
}

impl MutationMessage {
    fn at_level(level: semio_framework_diagnostic::Severity, code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self { level, code: code.into(), message: message.into(), target: Vec::new(), op_index: None }
    }

    pub fn info(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Info, code, message)
    }
    pub fn warning(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Warning, code, message)
    }
    pub fn error(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Error, code, message)
    }
    pub fn fatal(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Fatal, code, message)
    }

    /// 🎯️ Attaches the target address (outermost segment first).
    pub fn at(mut self, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.target = target.into_iter().map(Into::into).collect();
        self
    }

    /// 🔢️ Stamps which op (within a batch) produced this message.
    pub fn at_op(mut self, op_index: u32) -> Self {
        self.op_index = Some(op_index);
        self
    }
}

/// 🚦️ The worst (highest) [`crate::diagnostic::Severity`] across `messages`, or `None` if empty.
pub fn worst_level(messages: &[MutationMessage]) -> Option<semio_framework_diagnostic::Severity> {
    messages.iter().map(|message| message.level).max()
}

/// 🗂️ A `Mutation`/`MutationKind::diff`'s full result: the diff plus every
/// [`MutationMessage`] it raised. LAWS (`📋️contract-freeze.md` §C2): (1) a `Fatal` message ⇒
/// `diff == D::default()`; (2) an `Error` message ⇒ `diff` carries no change for the named target;
/// (3) deterministic — equal `(op, base)` ⇒ equal `messages`. Laws 1/2 are upheld by every `diff`
/// leaf's own construction (via [`MutationOutcome::fatal`]/[`MutationOutcome::error`], or by simply
/// never writing to a rejected target before calling [`MutationOutcome::info`]/
/// [`MutationOutcome::warning`]/[`MutationOutcome::absorb_messages`]), not enforced by this type
/// itself — see `🧪️testkit`'s `assert_fatal_never_applies`/`assert_missing_target_is_error`.
///
/// 🎯️ Naming note: the frozen contract's prose also lists chainable `.error(..).await`/`.fatal(..).await`
/// instance builders alongside the static `::error(code,msg,target).await`/`::fatal(code,msg,target).await`
/// whole-outcome shortcuts — those two forms cannot share a name on the same inherent type (Rust
/// E0592: an instance method and an associated function of the same name conflict across impl
/// blocks whenever their `D` bounds overlap). This crate keeps `fatal`/`error` as the static,
/// whole-outcome-rejecting shortcuts (the common case for a simple non-batch verb) and `info`/`warning`
/// as the 2-arg chainable instance builders (the fan-out recipe's own `.info("mutation.cascade", ..).await`
/// example); a leaf needing a TARGETED error/warning alongside a non-empty diff (the batch-verb case).await
/// builds the message directly (`MutationMessage::error(code, msg).await.at(target).await`) and attaches it via
/// [`MutationOutcome::absorb_messages`].
#[derive(Clone, Debug, PartialEq)]
pub struct MutationOutcome<D> {
    pub(super) diff: D,
    pub(super) messages: Vec<MutationMessage>,
}

/// 🛂️ One semantic decision owns the original delta and diagnostics before inverse or application.
pub enum MutationAdmission<D> {
    Admitted(MutationOutcome<D>),
    Refused(MutationOutcome<D>),
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin above
/// documents. Mirrors `#[serde(rename_all = "camelCase", default, skip_serializing_if =
/// "Vec::is_empty")]` byte-for-byte.
impl<D: semio_framework_value::ToValue> semio_framework_value::ToValue for MutationOutcome<D> {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![("diff".to_string(), semio_framework_value::ToValue::to_value(&self.diff))];
        if !self.messages.is_empty() {
            entries.push(("messages".to_string(), semio_framework_value::ToValue::to_value(&self.messages)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl<D: semio_framework_value::FromValue> semio_framework_value::FromValue for MutationOutcome<D> {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationOutcome, found {value:?}")));
        };
        let mut diff = None;
        let mut messages = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "diff" => diff = Some(<D as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("diff"))?),
                "messages" => messages = <Vec<MutationMessage> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?,
                _ => {}
            }
        }
        Ok(MutationOutcome { diff: diff.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationOutcome missing diff"))?, messages })
    }
}

impl<D: Default> MutationOutcome<D> {
    /// 🕳️ No change, no messages — the algebra's identity element.
    pub fn empty() -> Self {
        Self { diff: D::default(), messages: Vec::new() }
    }

    /// 🚨️ Forces `diff = D::default()` (LAW 1) with one `Fatal` message.
    pub fn fatal(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::fatal(code, message).at(target)] }
    }

    /// 🚫️ Empty diff with one `Error` message — the caller's `diff` leaf never touched `target`.
    pub fn error(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::error(code, message).at(target)] }
    }

    /// 🧭️ Empty diff with one message at the level the vocabulary fixes for `code` — for a guard that picks its
    /// vocabulary code at runtime; [`OutcomeCode`] admits no other code.
    pub fn refuse(code: OutcomeCode, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::at_level(code.level(), code.as_str(), message).at(target)] }
    }
}

impl<D> MutationOutcome<D> {
        /// ⚖️ Error and Fatal refuse the whole semantic operation before any executable delta is used.
    pub fn admit(self) -> MutationAdmission<D> {
        if self.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
            MutationAdmission::Refused(self)
        } else {
            MutationAdmission::Admitted(self)
        }
    }

    /// 🛂️ Gates cold inverse candidates on semantic admission and actual participation, retiring the evaluated diff once.
    pub fn admit_inverse<P>(self, foreign_steps: impl FnOnce() -> Result<bool, semio_framework_value::ValueError>) -> Result<bool, semio_framework_value::ValueError>
    where
        D: MutationDiff<P>,
    {
        let (outcome, admitted) = match self.admit() {
            MutationAdmission::Admitted(outcome) => (outcome, true),
            MutationAdmission::Refused(outcome) => (outcome, false),
        };
        let executable = if !admitted { Ok(false) } else if !outcome.diff().is_empty() { Ok(true) } else { foreign_steps() };
        let (diff, _) = outcome.into_parts();
        diff.retire_cold();
        executable
    }

/// ✅️ A successful diff, no messages.
    pub fn new(diff: D) -> Self {
        Self { diff, messages: Vec::new() }
    }

    pub fn diff(&self) -> &D {
        &self.diff
    }

    pub fn messages(&self) -> &[MutationMessage] {
        &self.messages
    }

    /// ➡️ Consumes `self` into its raw `(diff, messages)` parts.
    pub fn into_parts(self) -> (D, Vec<MutationMessage>) {
        (self.diff, self.messages)
    }

    /// ➕️ Appends one `Info`-level message (e.g. a cascade note).
    pub fn info(mut self, code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::info(code, message));
        self
    }

    /// ➕️ Appends one `Warning`-level message.
    pub fn warning(mut self, code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::warning(code, message));
        self
    }

    /// ➕️ Appends every message in `messages`, in order — the general escape hatch for a targeted
    /// message that the 2-arg `info`/`warning` builders can't express (see the naming note above).
    pub fn absorb_messages(mut self, messages: impl IntoIterator<Item = MutationMessage>) -> Self {
        self.messages.extend(messages);
        self
    }

    /// 🔢️ Stamps `op_index` onto every message currently carried — called once per op during a
    /// batch replay, right after that op's `diff` is computed.
    pub fn stamp_op_index(mut self, op_index: u32) -> Self {
        for message in &mut self.messages {
            message.op_index = Some(op_index);
        }
        self
    }

    /// 🚦️ The worst level across `self.messages`, or `None` if there are none.
    pub fn worst_level(&self) -> Option<semio_framework_diagnostic::Severity> {
        worst_level(&self.messages)
    }

    /// ✅️ Whether `policy` would accept this outcome (i.e. does NOT reject its worst level). An
    /// outcome with no messages is always applicable.
    pub fn is_applicable(&self, policy: semio_framework_replication_wire_core::MergePolicy) -> bool {
        match self.worst_level() {
            Some(level) => !policy.rejects(level),
            None => true,
        }
    }

    /// 🔀️ Maps the diff, keeping every message unchanged.
    pub fn map<D2>(self, f: impl FnOnce(D) -> D2) -> MutationOutcome<D2> {
        MutationOutcome { diff: f(self.diff), messages: self.messages }
    }
}
#[cfg(test)]
#[path = "../🧪️tests/↩️inverse-admission/🦀️.rs"]
mod inverse_admission_tests;

//#endregion 🔖️Message
