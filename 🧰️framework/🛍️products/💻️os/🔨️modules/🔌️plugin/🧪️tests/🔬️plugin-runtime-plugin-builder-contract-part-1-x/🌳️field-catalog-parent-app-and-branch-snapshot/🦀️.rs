// 🌳️ Field catalog, parent app and branch snapshot: the owned field catalog, the composed parent app, the recursive branch snapshot, its diff and semantic mutations and the canonical json.

//#region 🧬️ComposedParentFixture
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct ComposedParentEnvelopeOwnedFieldCatalog;

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<ComposedParentSnapshot, RecursiveFixtureMutation> for ComposedParentEnvelopeOwnedFieldCatalog {
    fn begin_vcs(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    ) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<ComposedParentSnapshot, RecursiveFixtureMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<ComposedParentSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<ComposedParentSnapshot>::default()),
            std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<RecursiveFixtureMutation>::default()),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<ComposedParentSnapshot, RecursiveFixtureMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    ) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<ComposedParentSnapshot>> {
        Box::new(ComposedParentSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(
        &self,
        _operation: semio_framework_job::OperationId,
        _generation: semio_framework_job::Generation,
        path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    ) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<RecursiveFixtureMutation>> {
        Box::new(ComposedParentRejectedFieldAuthority { path, terminal: false })
    }

    fn begin_spr_conflict(
        &self,
        _operation: semio_framework_job::OperationId,
        _generation: semio_framework_job::Generation,
        path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    ) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(ComposedParentRejectedFieldAuthority { path, terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<RecursiveFixtureMutation>>> {
        store::os_store::owned_schema_decode::artifact_bounded_history_entry_decoder()
    }
}

/// 🚫️ A parent revision whose composition only the APP knows to be unrestorable: the structural
/// projection accepts the very same snapshot, so a framework seam that still calls
/// `semio_framework_composition_admission::ChildRestoreProjection::from_snapshot` directly answers no fault at all here.
const COMPOSED_PARENT_PROJECTION_REFUSAL_REVISION: i32 = -7;

/// 🗣️ The exact diagnostic the app owns; the structural error type has no way to spell it.
const COMPOSED_PARENT_PROJECTION_REFUSAL_MESSAGE: &str = "composed parent child projection failed: revision is not restorable";

#[derive(Default)]
struct ComposedParentApp<const HAS_CHILD: bool = true>;

impl<const HAS_CHILD: bool> ArtifactApp for ComposedParentApp<HAS_CHILD> {
    const APP_ID: &'static str = "s.test.composed@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.composed-test/v1";
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.composed", standard: StandardId("1"), subset: SubsetId::ANY };
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        if snapshot.revision == COMPOSED_PARENT_PROJECTION_REFUSAL_REVISION {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("test.parent-projection"), COMPOSED_PARENT_PROJECTION_REFUSAL_MESSAGE));
        }
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("test.parent-projection"), error.to_string()))
    }
    type Snapshot = ComposedParentSnapshot;
    type Mutation = RecursiveFixtureMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = PublicationPresence;
    type PresenceMutation = PublicationPresenceMutation;
    type Transient = PublicationTransient;
    type TransientMutation = PublicationTransientMutation;
    type Command = NoConfigMutation;

    fn reference_birth()->Option<fn()->Self::Snapshot>{Some(||{
        Self::Snapshot { slot: HAS_CHILD.then(|| ParentFixtureChild::new("child-1".into(), ArtifactRef {
            artifact_id: "child-1".into(),
            dialect: ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() },
        })), revision: 0 }
    })}
    async fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, Self::Draft>, _engines: &semio_framework_2d_compute::EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match *command {}
    }
    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(semio_framework_ui_locale::Label::data("Composed parent fixture"))
    }
    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(
            std::sync::Arc::new(ComposedParentEnvelopeOwnedFieldCatalog),
            std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<ComposedParentSnapshot>::default()),
            std::sync::Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<RecursiveFixtureMutation>::default()),
        ))
    }
    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let birth = recursive_fixture_owners_birth_demand::<Self::Snapshot>()?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() })
    }
    fn build_document_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(recursive_fixture_owners::<Self::Snapshot>("recursive-parent", grant))
    }
    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> { Some(bounded_document_store_disposer()) }
    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        _operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> semio_framework_plugin_ownership::store_lifecycle::ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        let exact = envelope.schema == Self::DOCUMENT_SCHEMA && envelope.dialect == Some(Self::DIALECT.into()) && envelope.cursor.is_some();
        let Some(candidate_generation) = generation.0.checked_add(1) else { return Err(envelope) };
        if !exact {
            return Err(envelope);
        }
        Ok(semio_framework_plugin_ownership::store_lifecycle::ArtifactStoreInitializationJob::new(Box::new(ReadyComposedParentInitialization::new(envelope, candidate_generation, actor))))
    }
    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> { TestApp::<false>::build_config_store_disposer() }
    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> { TestApp::<false>::build_draft_store_disposer() }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_local_root_retirement_factory() }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_peer_retirement_factory() }
    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { TestApp::<false>::build_presence_store_disposer() }
    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { TestApp::<false>::build_transient_store_disposer() }
}

type RecursiveFixtureChild = store::os_store::artifact_child::ArtifactChild<TestSnapshot>;

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_schema::ArtifactSchema)]
#[artifact_schema(id = "s.test.child")]
struct RecursiveBranchSnapshot {
    #[state(artifact)]
    pub count: i32,
    #[state(artifact)]
    pub label: String,
    #[state(artifact)]
    #[child(kind = "s.test.child")]
    pub nested: Option<RecursiveFixtureChild>,
}


impl store::os_store::serialization::ArtifactSqliteSnapshotHost for RecursiveBranchSnapshot {



}
impl pack::snapshot::ArtifactSqliteSnapshot for RecursiveBranchSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("../../🧩️composition/🌿️branch.sql");
fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotPhase};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        let mut values = vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.count)), SqliteValue::Text(self.label.clone())];
        values.extend(match &self.nested { Some(child) => vec![SqliteValue::Text(child.target.artifact_id.clone()), SqliteValue::Text(child.target.dialect.artifact_kind.clone()), SqliteValue::Text(child.target.dialect.standard.clone()), SqliteValue::Text(child.target.dialect.subset.clone())], None => vec![SqliteValue::Null; 4] });
        database.table_mut("branch_state")?.rows.push(SqliteRow { rowid: 1, values });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {

        use store::sqlite_snapshot::{SqliteValue, SqliteSnapshotPhase};
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("branch_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "branch_state requires one state row")); }
        let row = &rows[0];
        let child = match row.values.get(3..).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "branch state row is truncated"))? {
            [SqliteValue::Null, SqliteValue::Null, SqliteValue::Null, SqliteValue::Null] => None,
            [SqliteValue::Text(id), SqliteValue::Text(kind), SqliteValue::Text(standard), SqliteValue::Text(subset)] => {
                let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: id.clone(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.clone(), standard: standard.clone(), subset: subset.clone() } };
                let target = semio_framework_artifact_reference::ArtifactRef::parse_uri(&target.to_uri()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error))?;
                Some(store::os_store::artifact_child::ArtifactChild::new(target.artifact_id.clone(), target))
            }
            _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "optional child identity must have all four fields or none")),
        };
        let snapshot = Self { count: i32::try_from(row.integer(1)?).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?, label: row.text(2)?.to_string(), nested: child };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct RecursiveBranchDiff {
    pub count: Option<i32>,
}

impl protocol::DiffAlgebra<RecursiveBranchSnapshot> for RecursiveBranchDiff {

    fn is_empty(&self) -> bool {
        self.count.is_none()
    }
}

impl protocol::ColdDiffAlgebra<RecursiveBranchSnapshot> for RecursiveBranchDiff {
    fn inverse(&self, base: &RecursiveBranchSnapshot) -> Self {
        Self { count: self.count.map(|_| base.count) }
    }
}

impl protocol::MutationDiff<RecursiveBranchSnapshot> for RecursiveBranchDiff {



}

impl protocol::ColdMutationDiff<RecursiveBranchSnapshot> for RecursiveBranchDiff {
    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }

    fn apply(&self, base: &mut RecursiveBranchSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
                if let Some(value) = self.count { base.count = value; }
        Ok(())
    }
}

impl Mutation<RecursiveBranchSnapshot> for RecursiveFixtureMutation {
    type Diff = RecursiveBranchDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR }


}

impl semio_framework_os_kernel::ColdMutation<RecursiveBranchSnapshot> for RecursiveFixtureMutation {
    type ReferenceDiff = RecursiveBranchDiff;
    fn diff(&self, _base: &RecursiveBranchSnapshot) -> protocol::MutationOutcome<Self::ReferenceDiff> { protocol::MutationOutcome::new(RecursiveBranchDiff { count: Some(self.value) }) }
    fn inverse(&self, base: &RecursiveBranchSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| { vec![Self { value: base.count }]
    })())
}
}

const RECURSIVE_FIXTURE_SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "recursive-value", kind: "set-recursive-value", record: "RecursiveValueSet" };

impl protocol::SemanticMutation<RecursiveBranchSnapshot> for RecursiveFixtureMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        std::slice::from_ref(&RECURSIVE_FIXTURE_SEMANTICS)
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        &RECURSIVE_FIXTURE_SEMANTICS
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set recursive value to {}", self.value), &format!("Rekursiven Wert auf {} setzen", self.value))
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::SemanticMutation<ComposedParentSnapshot> for RecursiveFixtureMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        std::slice::from_ref(&RECURSIVE_FIXTURE_SEMANTICS)
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        &RECURSIVE_FIXTURE_SEMANTICS
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::SemanticMutation<RecursiveBranchSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl store::ArtifactCanonicalJson for RecursiveFixtureMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<store::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        match path {
            [] => Ok(store::ArtifactCanonicalJsonNode::Object(1)),
            [0] => Ok(store::ArtifactCanonicalJsonNode::I64(i64::from(self.value))),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "canonical-edit.invalid-typed-path")),
        }
    }

    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<store::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        match (path, index) {
            ([], 0) => Ok("value".into()),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "canonical-edit.invalid-typed-path")),
        }
    }

    fn canonical_json_borrowed_root(&self) -> Result<Option<store::ArtifactCanonicalJsonValue<'_>>, semio_framework_value::ValueError> {
        use store::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonObject as Object, ArtifactCanonicalJsonValue as Value};
        Ok(Some(Value::Object(Object::new([("value", Value::Scalar(Node::I64(i64::from(self.value))))].into_iter()))))
    }
}
//#endregion 🧬️ComposedParentFixture
