//! 📸️ Native editor root with typed authority and a derived intrinsic host view.
use crate::{Puzzle3dSnapshot,Puzzle3dMutation};
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
use semio_framework_value::DslValue;

//#region 🔖️PlaySnapshot
/// 🌱️ Immutable typed puzzle authority and its deferred first-party intrinsic view.
#[derive(Debug)]
pub struct Puzzle3dPlaySnapshot {
    typed: std::sync::Arc<Puzzle3dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<DslValue>>,
}

impl Puzzle3dPlaySnapshot {
    /// 🎯️ Builds an immutable root from an admitted typed snapshot.
    pub fn new(typed: Puzzle3dSnapshot) -> Self {
        Self::from_typed(typed)
    }

    /// 🧬️ Retains typed mutation authority and defers its native host view.
    pub(crate) fn from_typed(typed: Puzzle3dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ Materializes the first-party host view once per immutable root.
    pub fn value(&self) -> &DslValue {
        self.value.get_or_init(|| std::sync::Arc::new(semio_framework_value::ToValue::to_value(self.typed.as_ref()))).as_ref()
    }

    /// 🧬️ Borrows the immutable typed authority.
    pub fn typed(&self) -> &Puzzle3dSnapshot {
        self.typed.as_ref()
    }

    /// 🧬️ Shares the immutable typed authority — what a tool request holds without copying the document.
    pub fn typed_arc(&self) -> std::sync::Arc<Puzzle3dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }
}

/// 🌱️ Encodes the admitted typed authority through the native value port.
impl semio_framework_value::ToValue for Puzzle3dPlaySnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue { <Puzzle3dSnapshot as semio_framework_value::ToValue>::to_value(self.typed()) }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::ToValue>::to_value_controlled(self.typed(), control) }
}

impl semio_framework_value::FromValue for Puzzle3dPlaySnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed) }
    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Self::from_typed) }
}

impl Clone for Puzzle3dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl PartialEq for Puzzle3dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

/// 🧒️ Composition view of the play snapshot: a puzzle document owns no child artifacts, so the
/// typed snapshot's own (empty) composition is the whole answer.
impl semio_framework_schema_composition::ArtifactCompositionFields for Puzzle3dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema_composition::ArtifactCompositionFields::visit_child_refs(self.typed.as_ref(), visitor)
    }
    fn child_slots() -> &'static [semio_framework_schema_composition::ChildSlotSpec] {
        <Puzzle3dSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::child_slots()
    }
    fn link_slots() -> &'static [semio_framework_schema_composition::LinkSlotSpec] {
        <Puzzle3dSnapshot as semio_framework_schema_composition::ArtifactCompositionFields>::link_slots()
    }
}

impl MutationDiff<Puzzle3dPlaySnapshot> for Puzzle3dDiff {
    fn apply(&self, projection: &Puzzle3dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle3dPlaySnapshot> {
        MutationDiff::<Puzzle3dSnapshot>::apply(self, projection.typed(), capability).map(Puzzle3dPlaySnapshot::from_typed)
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle3dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Puzzle3dPlaySnapshot> for Puzzle3dDiff {
    fn inverse(&self, base: &Puzzle3dPlaySnapshot) -> Self {
        DiffAlgebra::<Puzzle3dSnapshot>::inverse(self, base.typed())
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle3dSnapshot>::is_empty(self)
    }
}

impl Mutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation {
    type Diff = Puzzle3dDiff;

    /// 🧷️ Reuses the typed mutation metadata.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle3dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle3dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle3dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle3dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle3dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle3dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        Mutation::<Puzzle3dSnapshot>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle3dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Puzzle3dPlaySnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        Mutation::<Puzzle3dSnapshot>::diff(self, projection.typed.as_ref())
    }

    fn inverse(&self, projection: &Puzzle3dPlaySnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        Mutation::<Puzzle3dSnapshot>::inverse(self, projection.typed.as_ref())?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle3dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle3dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle3dSnapshot>::conflict_target(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle3dSnapshot>` impl above never actually reads `Puzzle3dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle3dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle3dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle3dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::semantics(self)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle3dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot
