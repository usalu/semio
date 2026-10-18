//! 🌱️ Play snapshot authority: the immutable typed puzzle authority with its host view, mutation and diff algebra, retirement and the pack, dsl and sqlite codecs. See [`super`].

use super::*;

//#region 🔖️PlaySnapshot
/// 🌱️ Immutable typed puzzle authority and its deferred first-party intrinsic view.
#[derive(Debug)]
pub struct Puzzle2dPlaySnapshot {
    typed: std::sync::Arc<Puzzle2dSnapshot>,
    pub(super) value: std::sync::OnceLock<std::sync::Arc<HostValue>>,
}

impl Puzzle2dPlaySnapshot {
    /// 🎯️ Builds an immutable root from an admitted typed snapshot.
    pub fn new(typed: Puzzle2dSnapshot) -> Self {
        Self::from_typed(typed)
    }

    /// 🧬️ A root produced by typed mutation application; its `Value` projection is deferred.
    pub fn from_typed(typed: Puzzle2dSnapshot) -> Self {
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

//#region 🚪️Codecs
/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle2dSnapshot`'s derived record
/// layout and pack-schema identity.
impl pack::snapshot::ArtifactPack for Puzzle2dPlaySnapshot {
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::record_spec()
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Puzzle2dPlaySnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

impl pack::snapshot::ArtifactDsl for Puzzle2dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle2d-play";

    fn envelope_id() -> &'static str {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::envelope_id()
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map(Self::from_typed)
    }

    fn print_dsl(&self) -> String {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(self.typed())
    }
}

/// 🎮️ Persists the editor's typed authority through the declared board schema.
impl ArtifactSqliteSnapshotHost for crate::Puzzle2dPlaySnapshot{
 
 
 
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{
  let limits=control.limits();semantic_cells::extent(limits)?;
  store::decode_sqlite_snapshot_record_native(payload,<Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::envelope_id(),Puzzle2dSnapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {
   semantic_cells::admit_record(record,limits,native)?;
   native.charge(size_of::<Puzzle2dSnapshot>()+2*size_of::<usize>())?;
   Puzzle2dSnapshot::__dsl_from_record_controlled(record,native).map(Self::from_typed)
  })(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.typed().preflight_sqlite_snapshot_encoding(encoding,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{self.typed().encode_sqlite_snapshot_native(encoding,control,native_owner)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{self.typed().validate_sqlite_snapshot_subset(dialect,database,control)}
}

impl pack::snapshot::ArtifactSqliteSnapshot for crate::Puzzle2dPlaySnapshot {
const SQLITE_SCHEMA:&'static str=<Puzzle2dSnapshot as ArtifactSqliteSnapshotHost>::SQLITE_SCHEMA;
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.typed().to_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let bytes=size_of::<Puzzle2dSnapshot>().checked_add(2*size_of::<usize>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Play authority allocation overflow"))?;
  restore_authority(database,control,bytes).map(Self::from_typed)
 }
}
//#endregion 🚪️Codecs
