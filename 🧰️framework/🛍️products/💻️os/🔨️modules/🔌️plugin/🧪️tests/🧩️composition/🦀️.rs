//#region 🧬️ComposedParentFixture
type ParentFixtureChild = store::ArtifactChild<TestSnapshot>;

thread_local! { static PARENT_SNAPSHOT_CLONES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

#[derive(Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_schema::ArtifactSchema)]
#[artifact_schema(id = "s.test.composed")]
struct ComposedParentSnapshot {
    #[state(artifact)]
    #[child(kind = "s.test.child")]
    slot: Option<ParentFixtureChild>,
}

impl Clone for ComposedParentSnapshot {
    fn clone(&self) -> Self {
        PARENT_SNAPSHOT_CLONES.with(|count| count.set(count.get() + 1));
        Self { slot: self.slot.clone() }
    }
}

impl store::ArtifactDsl for ComposedParentSnapshot {
    const EXTENSION: &'static str = "composed-parent-test";
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let value = serde_json::from_str::<serde_json::Value>(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        store::os_pack::json::to_json_string(self)
    }
}

impl ArtifactPack for ComposedParentSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(store::os_pack::json::to_json_string(self).into_bytes())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = serde_json::from_slice::<serde_json::Value>(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

impl protocol::MutationDiff<ComposedParentSnapshot> for NoConfig {
    fn apply(&self, base: &ComposedParentSnapshot) -> protocol::MutationApplyResult<ComposedParentSnapshot> { Ok(base.clone()) }
    fn absorb(&mut self, _other: Self) {}
}

impl protocol::Mutation<ComposedParentSnapshot> for NoConfigMutation {
    type Diff = NoConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { match *self {} }
    fn diff(&self, _base: &ComposedParentSnapshot) -> protocol::MutationOutcome<NoConfig> { match *self {} }
    fn inverse(&self, _base: &ComposedParentSnapshot) -> Vec<Self> { match *self {} }
}

#[derive(Default)]
struct ComposedParentApp<const HAS_CHILD: bool = true>;

impl<const HAS_CHILD: bool> ArtifactApp for ComposedParentApp<HAS_CHILD> {
    const APP_ID: &'static str = "s.test.composed@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.composed-test/v1";
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.composed", standard: StandardId("1"), subset: SubsetId::ANY };
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("test.parent-projection"), error.to_string()))
    }
    type Snapshot = ComposedParentSnapshot;
    type Mutation = NoConfigMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = PublicationPresence;
    type PresenceMutation = PublicationPresenceMutation;
    type Transient = PublicationTransient;
    type TransientMutation = PublicationTransientMutation;
    type Command = NoConfigMutation;

    async fn initial_snapshot() -> Self::Snapshot {
        Self::Snapshot { slot: HAS_CHILD.then(|| ParentFixtureChild::new("child-1".into(), ArtifactRef {
            artifact_id: "child-1".into(),
            dialect: store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() },
        })) }
    }
    async fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match *command {}
    }
    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data("Composed parent fixture"))
    }
    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> { Some(bounded_document_store_owners()) }
    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> { Some(bounded_document_store_disposer()) }
    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> { TestApp::<false>::build_config_store_owners() }
    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> { TestApp::<false>::build_config_store_disposer() }
    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> { TestApp::<false>::build_draft_store_owners() }
    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> { TestApp::<false>::build_draft_store_disposer() }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_local_root_retirement_factory() }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_peer_retirement_factory() }
    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { TestApp::<false>::build_presence_store_disposer() }
    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { TestApp::<false>::build_transient_store_disposer() }
}

async fn assert_parent_restore_case<const HAS_CHILD: bool>(row: &serde_json::Value) {
    let mut app = VcsArtifactApp::<ComposedParentApp<HAS_CHILD>, TestMembers>::new(ComposedParentApp::default()).await;
    let dialect = test_child_dialect().await;
    let child_id = row["childId"].as_str().unwrap();
    let slot = row["slot"].as_str().unwrap();
    let mut member = TestMembers::create(child_id, &dialect, &TestSnapshot::default().encode_pack()).await.unwrap();
    member.set_owner(Some(store::OwnerRef {
        parent: ArtifactRef { artifact_id: app.store.envelope().id.clone(), dialect: ComposedParentApp::<HAS_CHILD>::DIALECT.into() },
        slot: slot.into(), child_id: child_id.into(),
    })).await;
    let packed = member.envelope_pack_bytes().await.unwrap();
    close_member_admission_fixture(&mut member);
    PARENT_SNAPSHOT_CLONES.with(|count| count.set(0));
    let admitted = app.open_child(slot, child_id, dialect, &packed).await.is_ok();
    let parent_clones = PARENT_SNAPSHOT_CLONES.with(std::cell::Cell::get);
    let published = app.test_child_admission_state(1).generation;
    close_member_admission_app(&mut app);
    let expected = row["accepted"].as_bool().unwrap();
    assert_eq!(admitted, expected, "{}", row["id"]);
    assert_eq!(published, u64::from(expected), "{}", row["id"]);
    assert_eq!(parent_clones, 0, "{}", row["id"]);
}

#[semio_framework_async_macros::async_test]
async fn member_factory_parent_snapshot_restore_matches_neutral_corpus() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🏪️store/🧩️composition/🪪️member-dialect/🧪️tests/🔣️.json"))).unwrap();
    for row in fixture["publicRestoreCases"].as_array().unwrap() {
        if row["parentHasChild"].as_bool().unwrap() { assert_parent_restore_case::<true>(row).await; }
        else { assert_parent_restore_case::<false>(row).await; }
    }
    eprintln!("[DEBUG] actual parent snapshot restore matched four independent neutral authority cases");
}
//#endregion 🧬️ComposedParentFixture
