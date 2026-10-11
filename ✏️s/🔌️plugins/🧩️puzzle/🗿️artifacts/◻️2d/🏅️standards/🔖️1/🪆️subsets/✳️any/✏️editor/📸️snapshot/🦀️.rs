//! 📸️ Native editor root with typed authority and a derived intrinsic host view.
use crate::{Puzzle2dSnapshot,Puzzle2dMutation};
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
use semio_framework_value::DslValue;
use semio_framework_pack_json::Value as HostValue;

//#region 🔖️PlaySnapshot
/// 🌱️ Immutable typed puzzle authority and its deferred first-party intrinsic view.
#[derive(Debug)]
pub struct Puzzle2dPlaySnapshot {
    typed: std::sync::Arc<Puzzle2dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<HostValue>>,
}

impl Puzzle2dPlaySnapshot {
    /// 🎯️ Builds an immutable root from an admitted typed snapshot.
    pub fn new(typed: Puzzle2dSnapshot) -> Self {
        Self::from_typed(typed)
    }

    /// 🧬️ A root produced by typed mutation application; its `Value` projection is deferred.
    pub(crate) fn from_typed(typed: Puzzle2dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ The intrinsic view, materialized once per immutable root.
    pub fn value(&self) -> &HostValue {
        self.value.get_or_init(|| std::sync::Arc::new(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self.typed.as_ref())))).as_ref()
    }

    /// 🤝️ The legacy play projection as a shared root — what an owned tool event carries without copying the document.
    pub fn shared_value(&self) -> std::sync::Arc<HostValue> {
        std::sync::Arc::clone(self.value.get_or_init(|| std::sync::Arc::new(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self.typed.as_ref())))))
    }

    /// 🫱️ The typed authority as a shared root, for the same reason.
    pub fn shared_typed(&self) -> std::sync::Arc<Puzzle2dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }

    /// 🧬️ The typed authority, without materializing the legacy projection.
    pub fn typed(&self) -> &Puzzle2dSnapshot {
        self.typed.as_ref()
    }
}

impl Clone for Puzzle2dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl PartialEq for Puzzle2dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

/// 🌱️ Encodes the admitted typed authority through the native value port.
impl semio_framework_value::ToValue for Puzzle2dPlaySnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(self.typed())
    }
}

impl semio_framework_value::FromValue for Puzzle2dPlaySnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Puzzle2dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed)
    }
}

/// 🧒️ Composition view of the play snapshot: a puzzle 2d document owns no child artifacts.
impl semio_framework_schema_composition::ArtifactCompositionFields for Puzzle2dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl MutationDiff<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dPlaySnapshot> {
        MutationDiff::<Puzzle2dSnapshot>::apply(self, projection.typed(), capability).map(Puzzle2dPlaySnapshot::from_typed).map_err(|error| error.under(["document"]))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn inverse(&self, base: &Puzzle2dPlaySnapshot) -> Self {
        DiffAlgebra::<Puzzle2dSnapshot>::inverse(self, base.typed())
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle2dSnapshot>::is_empty(self)
    }
}

impl Mutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    /// 🧷️ Not hand-written — see the identical note on `impl Mutation<Value>` above. The metadata
    /// is projection-independent, so this forwards to the derive's own table too, same as
    /// `may_emit_foreign_steps` already does immediately below.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle2dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle2dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle2dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle2dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle2dSnapshot>>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle2dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Puzzle2dPlaySnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        Mutation::<Puzzle2dSnapshot>::diff(self, projection.typed())
    }

    fn inverse(&self, projection: &Puzzle2dPlaySnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        Mutation::<Puzzle2dSnapshot>::inverse(self, projection.typed())?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle2dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle2dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle2dSnapshot>::conflict_target(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle2dSnapshot>` impl above never actually reads `Puzzle2dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle2dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle2dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle2dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::semantics(self)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

/// ♻️ The typed authority retires through its own owner, then the deferred first-party host view, each as its own bounded cursor turn.
impl semio_framework_value::retirement::RetireOwned for Puzzle2dPlaySnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { typed, value } = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(typed), semio_framework_value::retirement::deferred(value.into_inner())])
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.typed), semio_framework_value::retirement::deferred_birth_bytes::<Option<std::sync::Arc<HostValue>>>()])
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}
