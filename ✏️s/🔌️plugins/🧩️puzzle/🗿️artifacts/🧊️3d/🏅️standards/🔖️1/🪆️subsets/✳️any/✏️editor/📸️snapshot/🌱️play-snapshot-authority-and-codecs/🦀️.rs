//! 🌱️ Play snapshot authority and codecs: the immutable typed puzzle authority with its host view, mutation and diff algebra, retirement and the dsl, pack and sqlite codecs. See [`super`].

use super::*;

//#region 🔖️PlaySnapshot
/// 🌱️ Immutable typed puzzle authority and its deferred first-party intrinsic view.
#[derive(Debug)]
pub struct Puzzle3dPlaySnapshot {
    pub(super) typed: std::sync::Arc<Puzzle3dSnapshot>,
    pub(super) value: std::sync::OnceLock<std::sync::Arc<DslValue>>,
}

impl Puzzle3dPlaySnapshot {
    /// 🎯️ Builds an immutable root from an admitted typed snapshot.
    pub fn new(typed: Puzzle3dSnapshot) -> Self {
        Self::from_typed(typed)
    }

    /// 🧬️ Retains typed mutation authority and defers its native host view.
    pub fn from_typed(typed: Puzzle3dSnapshot) -> Self {
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
    fn to_value(&self) -> DslValue { <Puzzle3dSnapshot as semio_framework_value::ToValue>::to_value(self.typed()) }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::ToValue>::to_value_controlled(self.typed(), control) }
}

impl semio_framework_value::FromValue for Puzzle3dPlaySnapshot {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed) }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { <Puzzle3dSnapshot as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Self::from_typed) }
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

    fn payload_value(&self) -> DslValue {
        Mutation::<Puzzle3dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
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
    fn from_payload_value(kind: &str, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
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

/// ♻️ The typed authority retires through its own owner, then the deferred first-party host view, each as its own bounded cursor turn.
impl semio_framework_value::retirement::RetireOwned for Puzzle3dPlaySnapshot {
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

//#region 🚪️Codecs
impl pack::snapshot::ArtifactDsl for Puzzle3dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle3d-play";

    fn envelope_id() -> &'static str { <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::envelope_id() }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map(Self::from_typed) }
    fn print_dsl(&self) -> String { <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(self.typed()) }
}

/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle3dSnapshot`'s derived record
/// layout and pack-schema identity.
impl pack::snapshot::ArtifactPack for Puzzle3dPlaySnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Puzzle3dSnapshot as pack::snapshot::ArtifactPack>::record_spec()
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Puzzle3dPlaySnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}

impl ArtifactSqliteSnapshotHost for Puzzle3dPlaySnapshot {
    
    
    
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, c: &mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        Puzzle3dSnapshot::decode_sqlite_snapshot_native(payload, c,native_control).map(Self::from_typed)
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, c: &mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, ValueError> {
        self.typed().encode_sqlite_snapshot_native(encoding, c,native_owner)
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        self.typed().validate_sqlite_snapshot_subset(dialect, d, c)
    }
}

impl pack::snapshot::ArtifactSqliteSnapshot for Puzzle3dPlaySnapshot {
const SQLITE_SCHEMA: &'static str = Puzzle3dSnapshot::SQLITE_SCHEMA;
fn to_sqlite_database(&self, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        self.typed().to_sqlite_database(c)
    }
fn from_sqlite_database(d: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        Puzzle3dSnapshot::from_sqlite_database(d, c).map(Self::from_typed)
    }
}
//#endregion 🚪️Codecs
