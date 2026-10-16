//! 🌳️ Artifact version graph: the artifact version graph with its read view and value conversions.
//!
//! Holds items of the module; the root re-exports them.

use super::*;

// 🎞️ `Serialize`/`Deserialize` dropped — see the docstring above `Change`.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactVcs<P, Mutation> {
    pub genesis: P,
    pub edits: ArtifactHistoryLedger<Edit<Mutation>>,
    pub changes: ArtifactHistoryLedger<Change>,
    pub checkpoints: ArtifactHistoryLedger<Checkpoint>,
    pub alternatives: ArtifactHistoryLedger<Alternative>,
}

impl<P: FromValue, Mutation: FromValue> FromValue for ArtifactVcs<ArtifactGenesis<P>, Mutation> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "VCS requires an object")); };
        let mut admitted = std::collections::BTreeMap::new();
        for (key, value) in fields { if admitted.insert(key, value).is_some() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "VCS repeats a field")); } }
        let mut fields = admitted;
        let mut take = |key: &str| fields.remove(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("VCS missing {key}")));
        let genesis = ArtifactGenesis::from_value(take("genesis")?)?;
        let edits = ArtifactHistoryLedger::from_value(take("edits")?)?;
        let changes = ArtifactHistoryLedger::from_value(take("changes")?)?;
        let checkpoints = ArtifactHistoryLedger::from_value(take("checkpoints")?)?;
        let alternatives = ArtifactHistoryLedger::from_value(take("alternatives")?)?;
        if !fields.is_empty() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "VCS has unknown fields")); }
        Ok(Self { genesis, edits, changes, checkpoints, alternatives })
    }
}

pub struct ArtifactVcsRead<'a, P, Mutation> {
    pub(super) genesis: &'a P,
    pub(super) edits: ArtifactHistoryIter<'a, Edit<Mutation>>,
    pub(super) changes: ArtifactHistoryIter<'a, Change>,
    pub(super) checkpoints: ArtifactHistoryIter<'a, Checkpoint>,
    pub(super) alternatives: ArtifactHistoryIter<'a, Alternative>,
}

/// 🧬️ Projects logical admitted genesis facts and the history decision captured by this read.
impl<P: ArtifactGenesisWitness, Mutation: ToValue> ToValue for ArtifactVcsRead<'_, P, Mutation> where P::Snapshot:ToValue {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("genesis".to_string(), self.genesis.genesis_facts().to_value()),
            ("edits".to_string(), self.edits.to_value()),
            ("changes".to_string(), self.changes.to_value()),
            ("checkpoints".to_string(), self.checkpoints.to_value()),
            ("alternatives".to_string(), self.alternatives.to_value()),
        ])
    }
}

impl<P, Mutation> ArtifactVcs<P, Mutation> {
    pub fn group_visibility(&self) -> Result<Option<&ArtifactGroupVisibility>, ()> {
        let mut visibility: Option<&ArtifactGroupVisibility> = None;
        for candidate in [self.edits.group_visibility(), self.changes.group_visibility(), self.checkpoints.group_visibility(), self.alternatives.group_visibility()].into_iter().flatten() {
            if visibility.is_some_and(|owner| !std::ptr::eq(owner, candidate)) {
                return Err(());
            }
            visibility = Some(candidate);
        }
        Ok(visibility)
    }

    pub fn read_group(&self, decision: Option<&ArtifactGroupReadDecision<'_>>) -> Result<ArtifactVcsRead<'_, P, Mutation>, ()> {
        Ok(ArtifactVcsRead {
            genesis: &self.genesis,
            edits: self.edits.read_group(decision)?,
            changes: self.changes.read_group(decision)?,
            checkpoints: self.checkpoints.read_group(decision)?,
            alternatives: self.alternatives.read_group(decision)?,
        })
    }
}

/// 🧬️ `ToValue` for `ArtifactVcs` — serde's own `Serialize` was dropped from this type (see the
/// `Change` struct's docstring above). No `Result`, unlike the old `serde::Serializer` path, so
/// the two invariant violations that path reported via `Error::custom` (a torn group-visibility
/// authority across the four ledgers, or losing the captured decision between the two calls) become
/// panics here instead — both are "cannot happen" internal-consistency bugs, never a caller input
/// error, matching this same file's existing `.expect(...)` style for slot-authority invariants.
impl<P: ArtifactGenesisWitness, Mutation: ToValue> ToValue for ArtifactVcs<P, Mutation> where P::Snapshot:ToValue {
    fn to_value(&self) -> DslValue {
        let decision = self.group_visibility().expect("a single ArtifactVcs never spans two different group visibility authorities").map(ArtifactGroupVisibility::capture);
        self.read_group(decision.as_ref()).expect("the just-captured decision remains valid for the immediately following read").to_value()
    }
}
