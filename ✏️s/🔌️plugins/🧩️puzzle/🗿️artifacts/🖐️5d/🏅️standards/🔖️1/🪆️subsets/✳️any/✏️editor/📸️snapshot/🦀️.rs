//! 📸️ Native editor root with typed authority and a derived intrinsic host view.
use crate::{Puzzle5dSnapshot,Puzzle5dMutation};
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
use semio_framework_value::DslValue;

//#region 🔖️PlaySnapshot
/// 🌱️ Immutable typed puzzle authority and its deferred first-party intrinsic view.
#[derive(Debug)]
pub struct Puzzle5dPlaySnapshot {
    typed: std::sync::Arc<Puzzle5dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<DslValue>>,
}

impl Puzzle5dPlaySnapshot {
    /// 🎯️ Builds an immutable root from an admitted typed snapshot.
    pub fn new(typed: Puzzle5dSnapshot) -> Self {
        Self::from_typed(typed)
    }

    /// 🧬️ A root produced by typed mutation application; its `Value` projection is deferred.
    pub(crate) fn from_typed(typed: Puzzle5dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ The intrinsic view, materialized once per immutable root.
    pub fn value(&self) -> &DslValue {
        self.value.get_or_init(|| std::sync::Arc::new(semio_framework_value::ToValue::to_value(self.typed.as_ref()))).as_ref()
    }

    /// 🧬️ The typed authority, without materializing the legacy projection.
    pub fn typed(&self) -> &Puzzle5dSnapshot {
        self.typed.as_ref()
    }

    /// 🧷️ The typed authority shared, never copied — the base a tool transaction yields against.
    pub fn typed_arc(&self) -> std::sync::Arc<Puzzle5dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }
}

impl Clone for Puzzle5dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl PartialEq for Puzzle5dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

/// 🩹️ Hand-written: `ArtifactEditor::Snapshot` needs `ToValue + FromValue`, and this struct's typed/lazy
/// split has no field-wise derive shape, so both bridge through the `Value` projection `value()`/`new()`
/// maintain.
impl semio_framework_value::ToValue for Puzzle5dPlaySnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(self.typed())
    }
}

impl semio_framework_value::FromValue for Puzzle5dPlaySnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Puzzle5dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed)
    }
}



/// 🧒️ Visits the literal persisted child owned by the typed Puzzle5d parent.
impl semio_framework_schema_composition::ArtifactCompositionFields for Puzzle5dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema_composition::ArtifactCompositionFields::visit_child_refs(self.typed(),visitor)
    }
}



impl MutationDiff<Puzzle5dPlaySnapshot> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dPlaySnapshot> {
        MutationDiff::<Puzzle5dSnapshot>::apply(self, projection.typed(), capability).map(Puzzle5dPlaySnapshot::from_typed).map_err(|error| error.under(["document"]))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle5dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Puzzle5dPlaySnapshot> for Puzzle5dDiff {
    fn inverse(&self, base: &Puzzle5dPlaySnapshot) -> Self {
        DiffAlgebra::<Puzzle5dSnapshot>::inverse(self, base.typed())
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle5dSnapshot>::is_empty(self)
    }
}

impl Mutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation {
    type Diff = Puzzle5dDiff;

    /// 🧷️ Not hand-written — see the identical note on `impl Mutation<Value>` above. The metadata
    /// is projection-independent, so this forwards to the derive's own table too, same as
    /// `may_emit_foreign_steps` already does immediately below.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle5dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle5dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle5dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle5dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle5dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle5dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle5dSnapshot>>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle5dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Puzzle5dPlaySnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        Mutation::<Puzzle5dSnapshot>::diff(self, projection.typed())
    }

    fn inverse(&self, projection: &Puzzle5dPlaySnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok({
        Mutation::<Puzzle5dSnapshot>::inverse(self, projection.typed())?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle5dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle5dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle5dSnapshot>::conflict_target(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle5dSnapshot>` impl above never actually reads `Puzzle5dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle5dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle5dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle5dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::semantics(self)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle5dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

/// ♻️ The typed authority retires through its own owner, then the deferred first-party host view, each as its own bounded cursor turn.
impl semio_framework_value::retirement::RetireOwned for Puzzle5dPlaySnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { typed, value } = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(typed), semio_framework_value::retirement::deferred(value.into_inner())])
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.typed), semio_framework_value::retirement::deferred_birth_bytes::<Option<std::sync::Arc<DslValue>>>()])
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}
