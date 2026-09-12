//#region 🧬️ComposedParentFixture
type ParentFixtureChild = store::ArtifactChild<TestSnapshot>;

thread_local! { static PARENT_SNAPSHOT_CLONES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

#[derive(Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_schema::ArtifactSchema)]
#[artifact_schema(id = "s.test.composed")]
struct ComposedParentSnapshot {
    #[state(artifact)]
    #[child(kind = "s.test.child")]
    slot: Option<ParentFixtureChild>,
    #[state(artifact)]
    revision: i32,
}

impl Clone for ComposedParentSnapshot {
    fn clone(&self) -> Self {
        PARENT_SNAPSHOT_CLONES.with(|count| count.set(count.get() + 1));
        Self { slot: self.slot.clone(), revision: self.revision }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct RecursiveFixtureMutation {
    value: i32,
}

impl protocol::OpText for RecursiveFixtureMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let value = line
            .strip_prefix("set-recursive-value ")
            .ok_or_else(|| TextError::new("expected set-recursive-value", TextSpan::at(1, 1)))?
            .parse()
            .map_err(|_| TextError::new("recursive value must be i32", TextSpan::at(1, 1)))?;
        Ok(Self { value })
    }

    fn print_op(&self) -> String {
        format!("set-recursive-value {}", self.value)
    }
}

impl protocol::OpBinary for RecursiveFixtureMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut bytes = vec![0x72];
        bytes.extend_from_slice(&self.value.to_be_bytes());
        Ok(bytes)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.len() != 5 || bytes.first() != Some(&0x72) {
            return Err(protocol::ProtocolError::Malformed { what: "recursive-fixture-mutation", offset: 0, detail: "expected tag 0x72 and four i32 bytes".into() });
        }
        Ok(Self { value: i32::from_be_bytes(bytes[1..].try_into().expect("exact recursive i32 width")) })
    }
}

const RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR: protocol::MutationLeafDescriptor = protocol::MutationLeafDescriptor {
    schema_version: 1,
    owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition",
    semantic_kind: "set-recursive-value",
    display_name: "Set Recursive Value",
    emoji: "🧬️",
    aggregate_variant: "SetRecursiveValue",
    payload_schema: "RecursiveFixtureMutation",
    text_opcode: Some("set-recursive-value"),
    binary_tag: Some(0x72),
    invertibility: protocol::MutationInvertibility::ExplicitMutation,
    diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
    outcome_classes: &[protocol::MutationOutcomeClass::Applied],
    composition: protocol::MutationComposition::Atomic,
    required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::Text, protocol::MutationLanguageSurface::Binary],
};

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct ComposedParentDiff {
    revision: Option<i32>,
}

impl protocol::MutationDiff<ComposedParentSnapshot> for ComposedParentDiff {
    fn apply(&self, base: &ComposedParentSnapshot) -> protocol::MutationApplyResult<ComposedParentSnapshot> {
        Ok(ComposedParentSnapshot { slot: base.slot.clone(), revision: self.revision.unwrap_or(base.revision) })
    }

    fn absorb(&mut self, other: Self) {
        if other.revision.is_some() {
            self.revision = other.revision;
        }
    }
}

impl Mutation<ComposedParentSnapshot> for RecursiveFixtureMutation {
    type Diff = ComposedParentDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR }
    fn diff(&self, _base: &ComposedParentSnapshot) -> protocol::MutationOutcome<Self::Diff> { protocol::MutationOutcome::new(ComposedParentDiff { revision: Some(self.value) }) }
    fn inverse(&self, base: &ComposedParentSnapshot) -> Vec<Self> { vec![Self { value: base.revision }] }
}

impl store::ArtifactDsl for ComposedParentSnapshot {
    const EXTENSION: &'static str = "composed-parent-test";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let value = serde_json::from_str::<Value>(text).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
        <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
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
        let value = serde_json::from_slice::<Value>(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

struct ComposedParentOwnedRetirement<T: Send + 'static>(Option<T>);

impl<T: Send + 'static> store::ErasedSnapshotRetirement for ComposedParentOwnedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

struct ComposedParentOwnedRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);

impl<T> Default for ComposedParentOwnedRetirementFactory<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: Send + 'static> store::ArtifactOwnedValueRetirementFactory<T> for ComposedParentOwnedRetirementFactory<T> {
    fn retire_owned(&self, value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(ComposedParentOwnedRetirement(Some(value)))
    }
}

#[expect(clippy::large_enum_variant, reason = "The fixture keeps its bounded retained hex authority inline through one exact field transition.")]
enum ComposedParentSnapshotDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<{ store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct ComposedParentSnapshotDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: ComposedParentSnapshotDecodeState,
    value: std::mem::ManuallyDrop<Option<ComposedParentSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl ComposedParentSnapshotDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: ComposedParentSnapshotDecodeState::AwaitToken,
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, ComposedParentSnapshotDecodeState::Published | ComposedParentSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<ComposedParentSnapshot> for ComposedParentSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if matches!(self.state, ComposedParentSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(self.diagnostic("composed-parent-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = ComposedParentSnapshotDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let path = self.path;
        let ComposedParentSnapshotDecodeState::Decode(authority) = &mut self.state else {
            return Err(store::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.snapshot-pack-token-replayed", offset: token.start, line: 0, column: 0, path });
        };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or(store::OwnedSchemaDecodeDiagnostic {
                    code: "composed-parent-envelope.snapshot-pack-missing",
                    offset: token.start,
                    line: 0,
                    column: 0,
                    path,
                })?;
                let value = <ComposedParentSnapshot as ArtifactPack>::decode_pack(bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic {
                    code: "composed-parent-envelope.snapshot-pack-malformed",
                    offset: token.start,
                    line: 0,
                    column: 0,
                    path,
                })?;
                assert!(authority.release(), "completed composed-parent snapshot pack releases its retained bytes exactly once");
                *self.value = Some(value);
                self.state = ComposedParentSnapshotDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(self.diagnostic("composed-parent-envelope.snapshot-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<ComposedParentSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, ComposedParentSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("composed-parent-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("composed-parent-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = ComposedParentSnapshotDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let ComposedParentSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = ComposedParentSnapshotDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&ComposedParentOwnedRetirementFactory::<ComposedParentSnapshot>::default(), value));
                self.state = ComposedParentSnapshotDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = ComposedParentSnapshotDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("composed-parent snapshot retirement remains retained");
        match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic {
            code: "composed-parent-envelope.snapshot-retirement-fault",
            offset: 0,
            line: 0,
            column: 0,
            path,
        })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = ComposedParentSnapshotDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("composed-parent-envelope.snapshot-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        ComposedParentSnapshotDecodeAuthority::terminal_is_empty(self)
    }
}

impl Drop for ComposedParentSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "composed-parent snapshot decoder dropped before publication or bounded retirement");
    }
}

struct ComposedParentRejectedFieldAuthority {
    path: store::OwnedSchemaPath,
    terminal: bool,
}

impl store::ArtifactEnvelopeMutationFieldAuthority<RecursiveFixtureMutation> for ComposedParentRejectedFieldAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.fresh-mutation-not-admitted", offset: token.start, line: 0, column: 0, path: self.path })
    }

    fn publish_reserved(
        &mut self,
        _target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<RecursiveFixtureMutation>,
        _reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.fresh-mutation-not-admitted", offset: 0, line: 0, column: 0, path: self.path })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl store::ArtifactEnvelopeSprConflictAuthority for ComposedParentRejectedFieldAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: self.path })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

struct ComposedParentEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<ComposedParentSnapshot, RecursiveFixtureMutation> for ComposedParentEnvelopeOwnedFieldCatalog {
    fn begin_vcs(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: store::OwnedSchemaPath,
    ) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<ComposedParentSnapshot, RecursiveFixtureMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(ComposedParentOwnedRetirementFactory::<ComposedParentSnapshot>::default()),
            std::sync::Arc::new(ComposedParentOwnedRetirementFactory::<RecursiveFixtureMutation>::default()),
            self.edit_history_decoder(),
        ))
    }

    fn begin_snapshot(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: store::OwnedSchemaPath,
    ) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<ComposedParentSnapshot>> {
        Box::new(ComposedParentSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(
        &self,
        _operation: semio_framework_job::OperationId,
        _generation: semio_framework_job::Generation,
        path: store::OwnedSchemaPath,
    ) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<RecursiveFixtureMutation>> {
        Box::new(ComposedParentRejectedFieldAuthority { path, terminal: false })
    }

    fn begin_spr_conflict(
        &self,
        _operation: semio_framework_job::OperationId,
        _generation: semio_framework_job::Generation,
        path: store::OwnedSchemaPath,
    ) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(ComposedParentRejectedFieldAuthority { path, terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<RecursiveFixtureMutation>>> {
        store::artifact_bounded_history_entry_decoder()
    }
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

    async fn initial_snapshot() -> Self::Snapshot {
        Self::Snapshot { slot: HAS_CHILD.then(|| ParentFixtureChild::new("child-1".into(), ArtifactRef {
            artifact_id: "child-1".into(),
            dialect: ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() },
        })), revision: 0 }
    }
    async fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match *command {}
    }
    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data("Composed parent fixture"))
    }
    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(store::ArtifactEnvelopeDecodeOwnerBundle::new(
            std::sync::Arc::new(ComposedParentEnvelopeOwnedFieldCatalog),
            std::sync::Arc::new(ComposedParentOwnedRetirementFactory::<ComposedParentSnapshot>::default()),
            std::sync::Arc::new(ComposedParentOwnedRetirementFactory::<RecursiveFixtureMutation>::default()),
        ))
    }
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(bounded_document_store_owners().with_one_item_preparation(bounded_config_store_one_item_preparation_factory("recursive-parent", 4096)))
    }
    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> { Some(bounded_document_store_disposer()) }
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        _operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> crate::app::ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> {
        let exact = envelope.schema == Self::DOCUMENT_SCHEMA && envelope.dialect == Some(Self::DIALECT.into()) && envelope.cursor.is_some();
        let Some(candidate_generation) = generation.0.checked_add(1) else { return Err(envelope) };
        if !exact {
            return Err(envelope);
        }
        Ok(crate::app::ArtifactStoreInitializationJob::new(Box::new(ReadyComposedParentInitialization::new(envelope, candidate_generation))))
    }
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> { TestApp::<false>::build_config_store_owners() }
    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> { TestApp::<false>::build_config_store_disposer() }
    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> { TestApp::<false>::build_draft_store_owners() }
    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> { TestApp::<false>::build_draft_store_disposer() }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_local_root_retirement_factory() }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::<false>::build_presence_peer_retirement_factory() }
    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { TestApp::<false>::build_presence_store_disposer() }
    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { TestApp::<false>::build_transient_store_disposer() }
}

type RecursiveFixtureChild = store::ArtifactChild<TestSnapshot>;

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_schema::ArtifactSchema)]
#[artifact_schema(id = "s.test.child")]
struct RecursiveBranchSnapshot {
    #[state(artifact)]
    count: i32,
    #[state(artifact)]
    label: String,
    #[state(artifact)]
    #[child(kind = "s.test.child")]
    nested: Option<RecursiveFixtureChild>,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct RecursiveBranchDiff {
    count: Option<i32>,
}

impl protocol::MutationDiff<RecursiveBranchSnapshot> for RecursiveBranchDiff {
    fn apply(&self, base: &RecursiveBranchSnapshot) -> protocol::MutationApplyResult<RecursiveBranchSnapshot> {
        Ok(RecursiveBranchSnapshot {
            count: self.count.unwrap_or(base.count),
            label: base.label.clone(),
            nested: base.nested.clone(),
        })
    }

    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }
}

impl Mutation<RecursiveBranchSnapshot> for RecursiveFixtureMutation {
    type Diff = RecursiveBranchDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR }
    fn diff(&self, _base: &RecursiveBranchSnapshot) -> protocol::MutationOutcome<Self::Diff> { protocol::MutationOutcome::new(RecursiveBranchDiff { count: Some(self.value) }) }
    fn inverse(&self, base: &RecursiveBranchSnapshot) -> Vec<Self> { vec![Self { value: base.count }] }
}

struct RecursiveBranchSnapshotOpen {
    request: std::mem::ManuallyDrop<Option<store::MemberOpenRequest>>,
    snapshot: std::mem::ManuallyDrop<Option<RecursiveBranchSnapshot>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    input: Vec<u8>,
    expected_bytes: Option<usize>,
    diagnostic: Option<store::MemberOpenDiagnostic>,
    terminal: bool,
}

impl RecursiveBranchSnapshotOpen {
    fn reject(&mut self, diagnostic: store::MemberOpenDiagnostic) -> store::MemberSnapshotOpenStep {
        self.diagnostic.get_or_insert(diagnostic);
        store::MemberSnapshotOpenStep::Rejected(self.diagnostic.expect("recursive snapshot diagnostic"))
    }
}

impl store::MemberSnapshotOpenOperation for RecursiveBranchSnapshotOpen {
    type Snapshot = RecursiveBranchSnapshot;

    fn begin(request: store::MemberOpenRequest) -> Result<Self, store::MemberOpenAdmissionError> {
        if let Err(diagnostic) = request.admitted_expected() {
            return Err(store::MemberOpenAdmissionError { diagnostic, request });
        }
        Ok(Self {
            request: std::mem::ManuallyDrop::new(Some(request)),
            snapshot: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            input: Vec::new(),
            expected_bytes: None,
            diagnostic: None,
            terminal: false,
        })
    }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> store::MemberSnapshotOpenStep {
        if let Some(diagnostic) = self.diagnostic {
            return store::MemberSnapshotOpenStep::Rejected(diagnostic);
        }
        if self.terminal {
            return store::MemberSnapshotOpenStep::Rejected(store::MemberOpenDiagnostic::Stale);
        }
        let frame = match self.request.as_mut().expect("recursive snapshot retains request").step_input(cx) {
            store::MemberOpenInputStep::Framed(frame) => frame,
            store::MemberOpenInputStep::Pending(progress) => return store::MemberSnapshotOpenStep::Pending(progress),
            store::MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
        };
        let expected_bytes = frame.snapshot_range().1;
        if self.expected_bytes.is_none() {
            if self.input.try_reserve_exact(expected_bytes).is_err() {
                return self.reject(store::MemberOpenDiagnostic::Capacity);
            }
            self.expected_bytes = Some(expected_bytes);
        }
        if self.input.len() < expected_bytes {
            let mut chunk = [0u8; 256];
            let maximum = chunk.len().min(expected_bytes - self.input.len());
            let copied = match self.request.as_ref().expect("recursive snapshot retains request").copy_snapshot_chunk(self.input.len(), &mut chunk[..maximum], cx) {
                Ok(copied) => copied,
                Err(diagnostic) => return self.reject(diagnostic),
            };
            self.input.extend_from_slice(&chunk[..copied]);
            return store::MemberSnapshotOpenStep::Pending(store::MemberOpenProgress {
                phase: store::MemberOpenPhase::Snapshot,
                completed: self.input.len() as u64,
                total: expected_bytes as u64,
            });
        }
        if self.snapshot.is_none() {
            let snapshot = match <RecursiveBranchSnapshot as ArtifactPack>::decode_pack(&self.input) {
                Ok(snapshot) => snapshot,
                Err(_) => return self.reject(store::MemberOpenDiagnostic::Decode),
            };
            *self.snapshot = Some(snapshot);
        }
        store::MemberSnapshotOpenStep::Ready
    }

    fn take_ready(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Option<(Self::Snapshot, store::MemberOpenRequest)> {
        if self.terminal || self.diagnostic.is_some() || self.request.as_ref()?.check_step_authority(cx).is_err() || self.input.len() != self.expected_bytes? {
            return None;
        }
        let snapshot = self.snapshot.take()?;
        let request = self.request.take()?;
        drop(std::mem::take(&mut self.input));
        self.expected_bytes = None;
        self.terminal = true;
        Some((snapshot, request))
    }
}

impl store::ErasedSnapshotRetirement for RecursiveBranchSnapshotOpen {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if self.terminal {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.diagnostic.get_or_insert(store::MemberOpenDiagnostic::Cancelled);
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("recursive snapshot retirement returned false terminal".into()),
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => {
                    Err("recursive snapshot retirement exceeded its exact grant".into())
                }
                step => Ok(step),
            };
        }
        if let Some(snapshot) = self.snapshot.take() {
            *self.active = Some(Box::new(TestOwnedValueRetirement(Some(snapshot))));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(request) = self.request.as_mut() {
            return match request.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if request.terminal_is_empty() => {
                    drop(self.request.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("recursive snapshot request returned false terminal".into()),
                step => Ok(step),
            };
        }
        if !self.input.is_empty() {
            let released_bytes = maximum_bytes.min(self.input.len());
            self.input.truncate(self.input.len() - released_bytes);
            if self.input.is_empty() {
                drop(std::mem::take(&mut self.input));
                self.expected_bytes = None;
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        self.expected_bytes = None;
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() && self.input.is_empty() && self.expected_bytes.is_none()
    }
}

impl Drop for RecursiveBranchSnapshotOpen {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || store::ErasedSnapshotRetirement::terminal_is_empty(self), "recursive snapshot decoder dropped before exact handoff or bounded retirement");
    }
}

macro_rules! recursive_fixture_snapshot {
    ($snapshot:ty, $extension:literal) => {
        impl store::ArtifactDsl for $snapshot {
            const EXTENSION: &'static str = $extension;
            fn parse_dsl(text: &str) -> Result<Self, TextError> {
                let value = serde_json::from_str::<Value>(text).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
                <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
            }
            fn print_dsl(&self) -> String { store::os_pack::json::to_json_string(self) }
        }

        impl ArtifactPack for $snapshot {
            fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                Ok(store::os_pack::json::to_json_string(self).into_bytes())
            }
            fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                let value = serde_json::from_slice::<Value>(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
                <Self as protocol::FromValue>::from_value(value.into()).map_err(|error| store::PackError::Schema(error.to_string()))
            }
        }

        impl store::MemberStoreOwner<RecursiveFixtureMutation> for $snapshot {
            type SnapshotOpen = RecursiveBranchSnapshotOpen;
            fn member_store_owners() -> store::DocumentStoreOwners<Self, RecursiveFixtureMutation> {
                bounded_document_store_owners().with_one_item_preparation(bounded_config_store_one_item_preparation_factory("recursive-member", 4096))
            }
        }
    };
}

recursive_fixture_snapshot!(RecursiveBranchSnapshot, "recursive-branch-test");

store::space_members! {
    pub enum RecursiveTestMembers, RecursiveTestMembersOpen {
        Branch("s.test.child", "native", "*", "semio.recursive-branch-test/v1") => (RecursiveBranchSnapshot, RecursiveFixtureMutation),
    }
}

async fn assert_parent_restore_case<const HAS_CHILD: bool>(row: &Value) {
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
    let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🏪️store/🧩️composition/🪪️member-dialect/🧫️fixtures/🔣️.json"))).unwrap();
    for row in fixture["publicRestoreCases"].as_array().unwrap() {
        if row["parentHasChild"].as_bool().unwrap() { assert_parent_restore_case::<true>(row).await; }
        else { assert_parent_restore_case::<false>(row).await; }
    }
    eprintln!("[DEBUG] actual parent snapshot restore matched four independent neutral authority cases");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReadyComposedParentInitializationPhase {
    Begin,
    Seed,
    Replay,
    Applied,
    Redo,
    Build,
    Complete,
}

struct ReadyComposedParentInitialization {
    envelope: Option<store::ArtifactEnvelope<ComposedParentSnapshot, RecursiveFixtureMutation>>,
    runtime: Option<store::ArtifactStoreInitializationRuntime<ComposedParentSnapshot>>,
    candidate: Option<store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>>,
    retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    candidate_generation: u64,
    edit_index: usize,
    lane_index: usize,
    operation_index: usize,
    phase: ReadyComposedParentInitializationPhase,
    closing: bool,
}

impl ReadyComposedParentInitialization {
    fn new(envelope: store::ArtifactEnvelope<ComposedParentSnapshot, RecursiveFixtureMutation>, candidate_generation: u64) -> Self {
        Self {
            envelope: Some(envelope),
            runtime: None,
            candidate: None,
            retirement: None,
            candidate_generation,
            edit_index: 0,
            lane_index: 0,
            operation_index: 0,
            phase: ReadyComposedParentInitializationPhase::Begin,
            closing: false,
        }
    }

    fn from_candidate(candidate: store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>) -> Self {
        Self {
            envelope: None,
            runtime: None,
            candidate: Some(candidate),
            retirement: None,
            candidate_generation: 0,
            edit_index: 0,
            lane_index: 0,
            operation_index: 0,
            phase: ReadyComposedParentInitializationPhase::Complete,
            closing: false,
        }
    }

    fn fault(cx: &mut semio_framework_job::StepContext<'_>, code: &[u8]) -> semio_framework_job::StepOutcome {
        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault {
            detail: crate::app::retained_job_payload(cx, semio_framework_job::JobPayloadStream::Fault, code),
        })
    }
}

impl crate::app::ArtifactStoreInitializationAuthority<ComposedParentSnapshot, RecursiveFixtureMutation> for ReadyComposedParentInitialization {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if self.closing || cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if cx.should_yield() {
                return semio_framework_job::StepOutcome::Yield;
            }
            let bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(4096);
            return match retirement.close_step(1, bytes) {
                Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes <= bytes => {
                    cx.consume_fuel((released_items + released_bytes).max(1) as u64);
                    semio_framework_job::StepOutcome::Yield
                }
                Ok(store::SnapshotRetirementStep::Complete) if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    cx.consume_fuel(1);
                    semio_framework_job::StepOutcome::Yield
                }
                _ => Self::fault(cx, b"recursive-parent-replay-retirement-rejected"),
            };
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        cx.consume_fuel(1);
        let envelope = match self.envelope.as_ref() {
            Some(envelope) => envelope,
            None if self.phase == ReadyComposedParentInitializationPhase::Complete => {
                return semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
            None => return Self::fault(cx, b"recursive-parent-envelope-owner-missing"),
        };
        match self.phase {
            ReadyComposedParentInitializationPhase::Begin => {
                let initial = envelope.vcs.initial_snapshot.clone();
                let initial_digest = *semio_framework_hash::hash(&initial.encode_pack()).as_bytes();
                self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest));
                self.phase = ReadyComposedParentInitializationPhase::Seed;
            }
            ReadyComposedParentInitializationPhase::Seed => {
                if let Some(edit) = envelope.vcs.edits.get(self.edit_index) {
                    let Some(runtime) = self.runtime.as_mut() else { return Self::fault(cx, b"recursive-parent-runtime-owner-missing") };
                    if self.operation_index == 0 {
                        if runtime.seed_mutation(protocol::MutationId(edit.id.clone())).is_err() {
                            return Self::fault(cx, b"recursive-parent-edit-seed-rejected");
                        }
                        self.operation_index = 1;
                    } else if let Some(meta) = edit.mutation_meta.get(self.operation_index - 1) {
                        if let Some(id) = &meta.mutation_id {
                            if id.0 != edit.id && runtime.seed_mutation(id.clone()).is_err() {
                                return Self::fault(cx, b"recursive-parent-mutation-seed-rejected");
                            }
                        }
                        runtime.observe_timestamp(meta.timestamp);
                        self.operation_index += 1;
                    } else {
                        runtime.observe_sequence(edit.sequence_number);
                        self.operation_index = 0;
                        self.edit_index += 1;
                    }
                } else {
                    self.edit_index = 0;
                    self.operation_index = 0;
                    self.phase = ReadyComposedParentInitializationPhase::Replay;
                }
            }
            ReadyComposedParentInitializationPhase::Replay => {
                let Some(cursor) = envelope.cursor.as_ref() else { return Self::fault(cx, b"recursive-parent-cursor-missing") };
                if let Some(edit_id) = cursor.applied_edit_ids.get(self.lane_index) {
                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id) else { return Self::fault(cx, b"recursive-parent-applied-edit-missing") };
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let Some(current) = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut) else {
                            return Self::fault(cx, b"recursive-parent-current-owner-missing");
                        };
                        let next = match protocol::MutationDiff::apply(operation.diff(current).diff(), current) {
                            Ok(next) => next,
                            Err(_) => return Self::fault(cx, b"recursive-parent-replay-rejected"),
                        };
                        let displaced = std::mem::replace(current, next);
                        self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(
                            &ComposedParentOwnedRetirementFactory::<ComposedParentSnapshot>::default(),
                            displaced,
                        ));
                        self.operation_index += 1;
                    } else {
                        self.operation_index = 0;
                        self.lane_index += 1;
                    }
                } else {
                    self.lane_index = 0;
                    self.phase = ReadyComposedParentInitializationPhase::Applied;
                }
            }
            ReadyComposedParentInitializationPhase::Applied | ReadyComposedParentInitializationPhase::Redo => {
                let Some(cursor) = envelope.cursor.as_ref() else { return Self::fault(cx, b"recursive-parent-cursor-missing") };
                let ids = if self.phase == ReadyComposedParentInitializationPhase::Applied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                if let Some(id) = ids.get(self.lane_index) {
                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *id) else { return Self::fault(cx, b"recursive-parent-lane-edit-missing") };
                    let Some(runtime) = self.runtime.as_mut() else { return Self::fault(cx, b"recursive-parent-runtime-owner-missing") };
                    let result = if self.phase == ReadyComposedParentInitializationPhase::Applied { runtime.push_applied_edit(edit) } else { runtime.push_redo_edit(edit) };
                    if result.is_err() {
                        return Self::fault(cx, b"recursive-parent-lane-seed-rejected");
                    }
                    self.lane_index += 1;
                } else {
                    self.lane_index = 0;
                    self.phase = if self.phase == ReadyComposedParentInitializationPhase::Applied {
                        ReadyComposedParentInitializationPhase::Redo
                    } else {
                        ReadyComposedParentInitializationPhase::Build
                    };
                }
            }
            ReadyComposedParentInitializationPhase::Build => {
                let cursor = envelope.cursor.as_ref().expect("checked recursive parent cursor");
                let actor = cursor.applied_edit_ids.last().and_then(|id| envelope.vcs.edits.iter().find(|edit| edit.id == *id)).and_then(|edit| edit.actor.clone());
                let runtime = self.runtime.as_mut().expect("recursive parent runtime remains retained");
                runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                runtime.set_local_actor_id(actor);
                let envelope = self.envelope.take().expect("recursive parent envelope remains retained");
                let runtime = self.runtime.take().expect("recursive parent runtime remains retained");
                let owners = ComposedParentApp::<true>::build_document_store_owners().expect("recursive parent owners");
                self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, self.candidate_generation, owners));
                self.phase = ReadyComposedParentInitializationPhase::Complete;
            }
            ReadyComposedParentInitializationPhase::Complete => {
                return semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
        }
        semio_framework_job::StepOutcome::Yield
    }

    fn request_cancel(&mut self) {
        self.closing = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>> {
        self.candidate.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if let Some(candidate) = self.candidate.as_mut() {
            return match candidate.close_owned_step(maximum_items, maximum_bytes).map_err(Fault::from)? {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "composed parent initializer candidate is externally retained" }),
                store::SnapshotRetirementStep::Complete if candidate.close_owned_terminal_is_empty() => {
                    drop(self.candidate.take());
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("test.composed-parent-close"), "candidate close returned false terminal")),
            };
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(maximum_items, maximum_bytes).map_err(Fault::from)? {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "composed parent initializer retirement is blocked" }),
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("test.composed-parent-retirement"), "retirement returned false terminal")),
            };
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory = ComposedParentOwnedRetirementFactory::<ComposedParentSnapshot>::default();
            return match runtime.close_step(&factory, maximum_items, maximum_bytes).map_err(Fault::from)? {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
                store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "composed parent runtime retirement is blocked" }),
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("test.composed-parent-runtime-retirement"), "runtime retirement returned false terminal")),
            };
        }
        if let Some(envelope) = self.envelope.take() {
            let bundle = ComposedParentApp::<true>::build_envelope_decode_owner_bundle().expect("recursive parent retirement catalog");
            self.retirement = Some(bundle.retire_envelope(envelope));
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        Ok(PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.envelope.is_none() && self.runtime.is_none() && self.candidate.is_none() && self.retirement.is_none()
    }
}

impl Drop for ReadyComposedParentInitialization {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "recursive parent initializer dropped before exact handoff or retained close");
    }
}

async fn retained_composed_replacement_fixture(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    parent_id: &str,
    child_count: i32,
) -> (crate::app::ArtifactEnvelopeDecodeOperationHandle, crate::app::OwnedDocumentMemberIngress) {
    let generation = semio_framework_job::Generation(app.store.generation_now());
    let child_dialect = test_child_dialect().await;
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let child_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect.clone() };
    let snapshot = ComposedParentSnapshot { slot: Some(ParentFixtureChild::new("child-1".into(), child_reference.clone())), revision: 0 };
    let mut envelope = store::create_document_envelope::<ComposedParentSnapshot, RecursiveFixtureMutation>(ComposedParentApp::<true>::DOCUMENT_SCHEMA, parent_id, snapshot, None);
    envelope.dialect = Some(parent_dialect.clone());
    let mut candidate = store::ArtifactStore::new(envelope).await.expect("candidate parent store");
    candidate.install_document_store_owners_exact(ComposedParentApp::<true>::build_document_store_owners().expect("candidate parent owners"));
    let job = crate::app::ArtifactStoreInitializationJob::new(Box::new(ReadyComposedParentInitialization::from_candidate(candidate)));
    let operation = semio_framework_job::OperationId(operation);
    app.store_replacement_jobs.insert_admitted(
        operation.0,
        crate::app::ActiveArtifactStoreReplacement::new(operation, generation, app.child_content_generation, job),
    );

    let owner = store::OwnerRef {
        parent: ArtifactRef { artifact_id: parent_id.into(), dialect: parent_dialect },
        slot: "slot".into(),
        child_id: "child-1".into(),
    };
    let mut member = TestMembers::create("child-1", &child_dialect, &TestSnapshot { count: child_count, label: "replacement".into() }.encode_pack()).await.expect("candidate child");
    member.set_owner(Some(owner.clone())).await;
    let bytes = member.envelope_pack_bytes().await.expect("candidate child full envelope");
    close_member_admission_fixture(&mut member);
    let chunks = bytes.chunks(store::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: bytes.len() }).expect("candidate member page credits");
    for chunk in chunks {
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded member page")).unwrap_or_else(|_| panic!("pre-admitted candidate member page"));
    }
    pages.seal().expect("candidate member page set");
    let request = store::MemberOpenRequest::new(operation, generation, u64::MAX, child_reference.clone(), Some(owner.clone()), pages).admit(1).unwrap_or_else(|_| panic!("candidate member request"));
    let ingress = crate::app::OwnedDocumentMemberIngress::try_new(0, child_reference, owner, request).unwrap_or_else(|_| panic!("candidate member ingress"));
    (crate::app::ArtifactEnvelopeDecodeOperationHandle { operation, generation }, ingress)
}

async fn drive_composed_replacement_to(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    handle: crate::app::ArtifactEnvelopeDecodeOperationHandle,
    target: crate::app::ActiveArtifactStoreReplacementState,
) {
    for _ in 0..100_000 {
        if app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.state == target) {
            return;
        }
        app.maintenance_stage = 14;
        let _ = PluginApp::maintenance_step(app, 1, 4096).expect("retained composed replacement step");
        semio_framework_async::yield_once().await;
    }
    panic!("retained composed replacement did not reach {target:?}");
}

async fn live_composed_replacement_app() -> VcsArtifactApp<ComposedParentApp, TestMembers> {
    let mut app = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default()).await;
    let dialect = test_child_dialect().await;
    let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack()).await.expect("live child");
    app.register_child("slot", "child-1", dialect, member).await.expect("live child publication");
    app
}

fn recursive_replacement_vectors() -> Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧫️fixtures/🧩️composition/🔣️.json"))).expect("recursive replacement vectors")
}

fn recursive_replacement_case<'a>(vectors: &'a Value, id: &str) -> &'a Value {
    vectors["cases"].as_array().expect("recursive replacement cases").iter().find(|row| row["id"] == id).expect("recursive replacement case")
}

fn close_recursive_member_fixture(member: &mut RecursiveTestMembers) {
    for _ in 0..65_536 {
        match member.close_owned_step(1, 4096).expect("recursive fixture member closes under its exact grant") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
            store::SnapshotRetirementStep::Blocked => panic!("recursive fixture member has no external owner"),
            store::SnapshotRetirementStep::Complete => {
                assert!(member.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("recursive fixture member did not close");
}

fn close_recursive_parent_fixture(store: &mut store::ArtifactStore<ComposedParentSnapshot, RecursiveFixtureMutation>) {
    for _ in 0..65_536 {
        match store.close_owned_step(1, 4096).expect("recursive archive parent closes under its exact grant") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
            store::SnapshotRetirementStep::Blocked => panic!("recursive archive parent has no external owner"),
            store::SnapshotRetirementStep::Complete => {
                assert!(store.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("recursive archive parent did not close");
}

fn close_recursive_replacement_app(app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>) {
    for _ in 0..100_000 {
        match app.close_step(1, 4096).expect("recursive fixture app closes under its exact grant") {
            PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
            PluginCloseStep::AwaitingInput { reason } => panic!("recursive fixture app close awaited input: {reason}"),
            PluginCloseStep::Blocked { reason } => panic!("recursive fixture app has no external owner: {reason}"),
            PluginCloseStep::Complete => {
                assert!(app.close_terminal_is_empty());
                return;
            }
        }
    }
    panic!("recursive fixture app did not close");
}

async fn recursive_member_envelope(reference: &ArtifactRef, owner: &store::OwnerRef, snapshot_pack: &[u8]) -> Vec<u8> {
    let mut member = Box::pin(RecursiveTestMembers::create(&reference.artifact_id, &reference.dialect, snapshot_pack)).await.expect("recursive candidate member");
    Box::pin(member.set_owner(Some(owner.clone()))).await;
    let bytes = Box::pin(member.envelope_pack_bytes()).await.expect("recursive candidate full envelope");
    close_recursive_member_fixture(&mut member);
    bytes
}

async fn recursive_member_envelope_with_history(reference: &ArtifactRef, owner: &store::OwnerRef, snapshot_pack: &[u8], value: i32) -> Vec<u8> {
    let mut member = Box::pin(RecursiveTestMembers::create(&reference.artifact_id, &reference.dialect, snapshot_pack)).await.expect("recursive historical member");
    Box::pin(member.set_owner(Some(owner.clone()))).await;
    match &mut member {
        RecursiveTestMembers::Branch(store) => {
            Box::pin(store.dispatch(store::ArtifactCommand::Apply { mutations: vec![RecursiveFixtureMutation { value }], description: Some("recursive historical member mutation".into()) }))
                .await
                .expect("recursive historical member mutation");
        }
    }
    let bytes = Box::pin(member.envelope_pack_bytes()).await.expect("recursive historical member full envelope");
    close_recursive_member_fixture(&mut member);
    bytes
}

fn recursive_member_history_offset(bytes: &[u8]) -> usize {
    let mut value = 0usize;
    let mut shift = 0usize;
    for (index, byte) in bytes.iter().copied().take(10).enumerate() {
        value |= usize::from(byte & 127) << shift;
        if byte & 128 == 0 {
            return index + 1 + value;
        }
        shift += 7;
    }
    panic!("recursive member envelope lacks its bounded snapshot frame")
}

async fn append_missing_edit_reference_to_member_history(entry: &mut protocol::OwnedDocumentMemberPackEntry) {
    let history_offset = recursive_member_history_offset(&entry.envelope_pack);
    let mut history = Box::pin(protocol::decode_history(
        &entry.envelope_pack[history_offset..],
        &protocol::os_spr::history::DecodeOptions::default(),
    ))
    .await
    .expect("decode recursive member history fixture");
    history.changes.push(protocol::HistoryChange {
        id: "malformed-child-after-valid-prefix".into(),
        saved_at: "2026-09-12T00:00:00Z".into(),
        edit_ids: vec!["missing-child-edit-after-valid-prefix".into()],
        description: Some("must fail after retained member framing and semantic decode".into()),
    });
    let encoded = Box::pin(protocol::encode_history(
        &history,
        &protocol::os_spr::history::EncodeOptions {
            write_backwards_section: true,
            ..protocol::os_spr::history::EncodeOptions::default()
        },
    ))
    .await
    .expect("encode semantically malformed retained member history");
    entry.envelope_pack.truncate(history_offset);
    entry.envelope_pack.extend(encoded);
}

async fn recursive_member_ingress(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    ordinal: usize,
    reference: ArtifactRef,
    owner: store::OwnerRef,
    snapshot_pack: Vec<u8>,
) -> crate::app::OwnedDocumentMemberIngress {
    let bytes = Box::pin(recursive_member_envelope(&reference, &owner, &snapshot_pack)).await;
    let chunks = bytes.chunks(store::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: bytes.len() }).expect("recursive candidate member page credits");
    for chunk in chunks {
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(chunk).expect("recursive bounded member page")).unwrap_or_else(|_| panic!("recursive candidate member page admission"));
    }
    pages.seal().expect("recursive candidate member page set");
    let request = store::MemberOpenRequest::new(operation, generation, u64::MAX, reference.clone(), Some(owner.clone()), pages).admit(1).unwrap_or_else(|_| panic!("recursive candidate member request"));
    crate::app::OwnedDocumentMemberIngress::try_new(ordinal, reference, owner, request).unwrap_or_else(|_| panic!("recursive candidate member ingress"))
}

fn recursive_archive_reference(reference: &ArtifactRef) -> protocol::DocumentArchiveArtifactRef {
    protocol::DocumentArchiveArtifactRef {
        artifact_id: reference.artifact_id.clone(),
        artifact_kind: reference.dialect.artifact_kind.clone(),
        standard: reference.dialect.standard.clone(),
        subset: reference.dialect.subset.clone(),
    }
}

async fn recursive_parent_document_files(
    parent_id: &str,
    schema: &str,
    dialect: ArtifactDialect,
    snapshot: ComposedParentSnapshot,
) -> store::ArtifactPackFiles {
    let mut envelope = store::create_document_envelope::<ComposedParentSnapshot, RecursiveFixtureMutation>(schema, parent_id, snapshot, None);
    envelope.dialect = Some(dialect);
    let mut parent_store = Box::new(Box::pin(store::ArtifactStore::new(envelope)).await.expect("recursive archive parent store"));
    parent_store.install_document_store_owners_exact(ComposedParentApp::<true>::build_document_store_owners().expect("recursive archive parent owners"));
    let generation = parent_store.generation_now();
    Box::pin(parent_store.apply_one(
        generation,
        RecursiveFixtureMutation { value: 7 },
        Some("persisted recursive parent history".into()),
        protocol::HistoryLane::Document,
    ))
    .await
    .expect("recursive archive parent mutation");
    let files = Box::pin(store::print_document_pack(parent_store.envelope())).await.expect("recursive archive parent pack");
    close_recursive_parent_fixture(&mut parent_store);
    files
}

async fn recursive_document_archive(parent_id: &str) -> protocol::DocumentArchivePack {
    let child_dialect = test_child_dialect().await;
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let root_reference = ArtifactRef { artifact_id: parent_id.into(), dialect: parent_dialect.clone() };
    let branch_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect.clone() };
    let leaf_reference = ArtifactRef { artifact_id: "grandchild-1".into(), dialect: child_dialect };
    let branch_owner = store::OwnerRef { parent: root_reference, slot: "slot".into(), child_id: branch_reference.artifact_id.clone() };
    let leaf_owner = store::OwnerRef { parent: branch_reference.clone(), slot: "nested".into(), child_id: leaf_reference.artifact_id.clone() };
    let branch_snapshot = RecursiveBranchSnapshot {
        count: 23,
        label: "archive-branch".into(),
        nested: Some(RecursiveFixtureChild::new(leaf_reference.artifact_id.clone(), leaf_reference.clone())),
    };
    let leaf_snapshot = RecursiveBranchSnapshot { count: 47, label: "archive-leaf".into(), nested: None };
    let parent_snapshot = ComposedParentSnapshot { slot: Some(ParentFixtureChild::new(branch_reference.artifact_id.clone(), branch_reference.clone())), revision: 0 };
    let parent = Box::pin(recursive_parent_document_files(parent_id, ComposedParentApp::<true>::DOCUMENT_SCHEMA, parent_dialect, parent_snapshot)).await;
    let branch_envelope = Box::pin(recursive_member_envelope_with_history(&branch_reference, &branch_owner, &branch_snapshot.encode_pack(), 29)).await;
    let leaf_envelope = Box::pin(recursive_member_envelope_with_history(&leaf_reference, &leaf_owner, &leaf_snapshot.encode_pack(), 53)).await;
    protocol::DocumentArchivePack {
        parent_pack: parent.pack,
        parent_spr: parent.spr,
        members: vec![
            protocol::OwnedDocumentMemberPackEntry {
                ordinal: 0,
                reference: recursive_archive_reference(&branch_reference),
                owner: protocol::DocumentArchiveOwnerRef { parent: recursive_archive_reference(&branch_owner.parent), slot: branch_owner.slot, child_id: branch_owner.child_id },
                envelope_pack: branch_envelope,
            },
            protocol::OwnedDocumentMemberPackEntry {
                ordinal: 1,
                reference: recursive_archive_reference(&leaf_reference),
                owner: protocol::DocumentArchiveOwnerRef { parent: recursive_archive_reference(&leaf_owner.parent), slot: leaf_owner.slot, child_id: leaf_owner.child_id },
                envelope_pack: leaf_envelope,
            },
        ],
    }
}

async fn drive_recursive_document_archive_load(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    operation: u64,
) -> protocol::DocumentArchiveLoadStatus {
    for _ in 0..1_000_000 {
        let status = Box::pin(PluginApp::poll_document_archive_load(app, operation)).await.expect("recursive archive operation status");
        if matches!(
            status.state,
            protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault
        ) {
            return status;
        }
        let _ = PluginApp::maintenance_step(app, 1, store::OWNED_SCHEMA_DECODE_PAGE_BYTES).expect("recursive archive public maintenance step");
        semio_framework_async::yield_once().await;
    }
    panic!("recursive archive operation exceeded its public maintenance progress authority")
}

async fn retained_recursive_replacement_fixture(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    operation: u64,
    parent_id: &str,
    mode: &str,
) -> (crate::app::ArtifactEnvelopeDecodeOperationHandle, Vec<crate::app::OwnedDocumentMemberIngress>) {
    let generation = semio_framework_job::Generation(app.store.generation_now());
    let child_dialect = test_child_dialect().await;
    let leaf_dialect = child_dialect.clone();
    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let branch_reference = ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect.clone() };
    let leaf_reference = ArtifactRef { artifact_id: "grandchild-1".into(), dialect: leaf_dialect.clone() };
    let root_reference = ArtifactRef { artifact_id: parent_id.into(), dialect: parent_dialect.clone() };
    let snapshot = ComposedParentSnapshot { slot: Some(ParentFixtureChild::new(branch_reference.artifact_id.clone(), branch_reference.clone())), revision: 0 };
    let mut envelope = store::create_document_envelope::<ComposedParentSnapshot, RecursiveFixtureMutation>(ComposedParentApp::<true>::DOCUMENT_SCHEMA, parent_id, snapshot, None);
    envelope.dialect = Some(parent_dialect);
    let mut candidate = store::ArtifactStore::new(envelope).await.expect("recursive candidate parent store");
    candidate.install_document_store_owners_exact(ComposedParentApp::<true>::build_document_store_owners().expect("recursive candidate parent owners"));
    let job = crate::app::ArtifactStoreInitializationJob::new(Box::new(ReadyComposedParentInitialization::from_candidate(candidate)));
    let operation = semio_framework_job::OperationId(operation);
    app.store_replacement_jobs.insert_admitted(
        operation.0,
        crate::app::ActiveArtifactStoreReplacement::new(operation, generation, app.child_content_generation, job),
    );

    let branch_owner = store::OwnerRef { parent: root_reference.clone(), slot: "slot".into(), child_id: branch_reference.artifact_id.clone() };
    let leaf_owner = store::OwnerRef { parent: branch_reference.clone(), slot: "nested".into(), child_id: leaf_reference.artifact_id.clone() };
    let branch_snapshot = RecursiveBranchSnapshot {
        count: 23,
        label: "replacement-branch".into(),
        nested: Some(RecursiveFixtureChild::new(leaf_reference.artifact_id.clone(), leaf_reference.clone())),
    };
    let leaf_snapshot = RecursiveBranchSnapshot { count: 47, label: "replacement-leaf".into(), nested: None };
    let mut ingress = Vec::new();
    ingress.push(recursive_member_ingress(operation, generation, 0, branch_reference.clone(), branch_owner.clone(), branch_snapshot.encode_pack()).await);
    match mode {
        "complete" => ingress.push(recursive_member_ingress(operation, generation, 1, leaf_reference, leaf_owner, leaf_snapshot.encode_pack()).await),
        "missing" => {}
        "extra" => {
            ingress.push(recursive_member_ingress(operation, generation, 1, leaf_reference, leaf_owner, leaf_snapshot.encode_pack()).await);
            let extra_reference = ArtifactRef { artifact_id: "orphan-1".into(), dialect: leaf_dialect };
            let extra_owner = store::OwnerRef { parent: root_reference, slot: "orphan".into(), child_id: extra_reference.artifact_id.clone() };
            ingress.push(recursive_member_ingress(operation, generation, 2, extra_reference, extra_owner, RecursiveBranchSnapshot { count: 59, label: "unreachable".into(), nested: None }.encode_pack()).await);
        }
        "duplicate" => ingress.push(recursive_member_ingress(operation, generation, 1, branch_reference, branch_owner, branch_snapshot.encode_pack()).await),
        other => panic!("unknown recursive replacement fixture mode {other}"),
    }
    (crate::app::ArtifactEnvelopeDecodeOperationHandle { operation, generation }, ingress)
}

fn admit_recursive_member_set(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    handle: crate::app::ArtifactEnvelopeDecodeOperationHandle,
    row: &Value,
    ingress: Vec<crate::app::OwnedDocumentMemberIngress>,
) {
    eprintln!("[DEBUG] recursive replacement member admission: reserving exact roster");
    assert!(app.try_begin_owned_document_members(handle, ingress.len(), u64::MAX).expect("recursive member ingress registry"));
    let mut ingress = ingress.into_iter().map(Some).collect::<Vec<_>>();
    for ordinal in row["admissionOrder"].as_array().expect("recursive admission order") {
        let ordinal = ordinal.as_u64().expect("recursive admission ordinal") as usize;
        eprintln!("[DEBUG] recursive replacement member admission: transferring ordinal {ordinal}");
        let member = ingress.get_mut(ordinal).and_then(Option::take).expect("recursive admission ordinal is unique and in range");
        app.admit_owned_document_member(handle, member).unwrap_or_else(|_| panic!("recursive exact candidate member admission"));
    }
    assert!(ingress.iter().all(Option::is_none));
    eprintln!("[DEBUG] recursive replacement member admission: sealing exact roster");
    app.seal_owned_document_members(handle).expect("recursive complete candidate member set");
}

fn drive_recursive_replacement_step(app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>) -> Result<PluginCloseStep, Fault> {
    PluginApp::maintenance_step(app, 1, 4096)
}

async fn drive_recursive_replacement_to(
    app: &mut VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>,
    handle: crate::app::ArtifactEnvelopeDecodeOperationHandle,
    target: crate::app::ActiveArtifactStoreReplacementState,
) {
    for _ in 0..100_000 {
        if app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.state == target) {
            return;
        }
        let _ = drive_recursive_replacement_step(app);
        semio_framework_async::yield_once().await;
    }
    panic!("recursive retained replacement did not reach {target:?}");
}

async fn live_recursive_replacement_app() -> Box<VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>> {
    let mut app = Box::pin(VcsArtifactApp::<ComposedParentApp, RecursiveTestMembers>::new(ComposedParentApp::default())).await;
    let dialect = test_child_dialect().await;
    let member = Box::pin(RecursiveTestMembers::create("child-1", &dialect, &RecursiveBranchSnapshot { count: 0, label: "live".into(), nested: None }.encode_pack())).await.expect("live recursive child");
    Box::pin(app.register_child("slot", "child-1", dialect, member)).await.expect("live recursive child publication");
    Box::new(app)
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_publishes_parent_members_view_graph_window_and_retires_displaced_bundle_atomically() {
    let mut app = live_composed_replacement_app().await;
    let old_parent_id = app.store.envelope().id.clone();
    let child_dialect = test_child_dialect().await;
    app.pending_child_pins.push(vcs::CompositionPin { child_ref: ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect }, checkpoint_id: "old-checkpoint".into() });
    let old_window_generation = app.window_transient_store.document_generation();
    let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 701, "replacement-parent", 23).await;
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers).await;
    assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("member ingress registry"));
    app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("exact candidate member admission"));
    app.seal_owned_document_members(handle).expect("complete candidate member set");

    for _ in 0..100_000 {
        let committed = app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.committed);
        if committed {
            assert_eq!(app.store.envelope().id, "replacement-parent");
            assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("new content view").count, 23);
            assert_eq!(app.composition.graph_mut().await.owner_of("child-1").await, Some("replacement-parent"));
            assert!(app.pending_child_pins.is_empty());
            assert_eq!(app.window_transient_store.document_generation(), old_window_generation + 1);
        } else {
            assert_eq!(app.store.envelope().id, old_parent_id);
            assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("old content view").count, 0);
            assert_eq!(app.window_transient_store.document_generation(), old_window_generation);
        }
        if app.poll_artifact_store_replacement(handle) == crate::app::ArtifactEnvelopeDecodeOperationPoll::Ready {
            break;
        }
        app.maintenance_stage = 14;
        let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("atomic composed replacement drive");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("terminal replacement acknowledgement"));
    close_member_admission_app(&mut app);
    eprintln!("[DEBUG] recursive replacement published parent/member/content/coordinator/window together and incrementally retired old roots, members, pins, and store");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_cancellation_during_open_closure_and_view_preparation_preserves_the_live_bundle() {
    for (case, target) in [
        ("member-open", crate::app::ActiveArtifactStoreReplacementState::OpeningMembers),
        ("closure", crate::app::ActiveArtifactStoreReplacementState::ValidatingClosure),
        ("views", crate::app::ActiveArtifactStoreReplacementState::PreparingCandidateViews),
    ] {
        let mut app = live_composed_replacement_app().await;
        let old_parent_id = app.store.envelope().id.clone();
        let old_window_generation = app.window_transient_store.document_generation();
        let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 710 + case.len() as u64, &format!("cancel-{case}"), 31).await;
        drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers).await;
        assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("cancel member registry"));
        app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("cancel candidate ingress"));
        app.seal_owned_document_members(handle).expect("cancel candidate seal");
        drive_composed_replacement_to(&mut app, handle, target).await;
        if case == "member-open" {
            app.maintenance_stage = 14;
            let _ = PluginApp::maintenance_step(&mut app, 1, 4096).expect("begin exact member open before cancellation");
            assert!(app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.active_member_open.is_some()));
        }
        app.cancel_artifact_store_replacement(handle).expect("cancel retained composed replacement");
        drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::Complete).await;
        assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Cancelled, "{case}");
        assert_eq!(app.store.envelope().id, old_parent_id, "{case}");
        assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("live content retained").count, 0, "{case}");
        assert_eq!(app.window_transient_store.document_generation(), old_window_generation, "{case}");
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("cancel acknowledgement"));
        close_member_admission_app(&mut app);
    }
    eprintln!("[DEBUG] recursive replacement cancellation retained the live bundle during member open, closure validation, and immutable-view preparation");
}

#[semio_framework_async_macros::async_test]
async fn retained_composed_replacement_rejects_a_real_live_child_generation_change_before_publication() {
    let mut app = live_composed_replacement_app().await;
    let old_parent_id = app.store.envelope().id.clone();
    let old_window_generation = app.window_transient_store.document_generation();
    let (handle, ingress) = retained_composed_replacement_fixture(&mut app, 729, "stale-parent", 47).await;
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers).await;
    assert!(app.try_begin_owned_document_members(handle, 1, u64::MAX).expect("stale member registry"));
    app.admit_owned_document_member(handle, ingress).unwrap_or_else(|_| panic!("stale candidate ingress"));
    app.seal_owned_document_members(handle).expect("stale candidate seal");
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::CandidateReady).await;
    let generation = app.admit_child_content_publication().expect("real live child publication authority");
    app.publish_child_content_member(generation, "slot", "child-1").await.expect("real live child generation change");
    drive_composed_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::Complete).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.store.envelope().id, old_parent_id);
    assert_eq!(app.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").expect("live content retained").count, 0);
    assert_eq!(app.window_transient_store.document_generation(), old_window_generation);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("stale replacement acknowledgement"));
    close_member_admission_app(&mut app);
    eprintln!("[DEBUG] recursive replacement fenced a real child-content generation change and retired its unpublished candidate without changing parent, child view, graph, or windows");
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_replacement_publishes_the_complete_nested_bundle_atomically() {
    eprintln!("[DEBUG] recursive replacement success: reading vectors");
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, "recursive-unordered-success");
    eprintln!("[DEBUG] recursive replacement success: constructing live bundle");
    let mut app = live_recursive_replacement_app().await;
    let old_parent_id = app.store.envelope().id.clone();
    let old_window_generation = app.window_transient_store.document_generation();
    let old_content_generation = app.child_content_generation;
    let child_dialect = test_child_dialect().await;
    app.pending_child_pins.push(vcs::CompositionPin { child_ref: ArtifactRef { artifact_id: "child-1".into(), dialect: child_dialect }, checkpoint_id: "displaced-checkpoint".into() });
    let parent_id = vectors["root"]["artifactId"].as_str().expect("recursive replacement root id");
    eprintln!("[DEBUG] recursive replacement success: constructing candidate ingress");
    let (handle, ingress) = retained_recursive_replacement_fixture(&mut app, 741, parent_id, row["mode"].as_str().expect("recursive replacement mode")).await;
    eprintln!(
        "[DEBUG] recursive replacement success: admitting complete member set active={} app={} ingress={}",
        std::mem::size_of::<crate::app::ActiveArtifactStoreReplacement<ComposedParentSnapshot, RecursiveFixtureMutation, RecursiveTestMembers>>(),
        std::mem::size_of::<VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>>(),
        std::mem::size_of::<crate::app::OwnedDocumentMemberIngress>(),
    );
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);
    eprintln!("[DEBUG] recursive replacement success: driving publication");

    for _ in 0..100_000 {
        let committed = app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.committed);
        if committed {
            assert_eq!(app.store.envelope().id, parent_id);
            assert_eq!(app.child_content_root.slots().len(), 2);
            assert_eq!(app.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("published recursive branch").count, 23);
            assert_eq!(app.child_content_root.typed_read::<RecursiveBranchSnapshot>("nested", "grandchild-1").expect("published recursive leaf").count, 47);
            assert_eq!(app.composition.graph_mut().await.owner_of("child-1").await, Some(parent_id));
            assert_eq!(app.composition.graph_mut().await.owner_of("grandchild-1").await, Some("child-1"));
            assert!(app.pending_child_pins.is_empty());
            assert_eq!(app.child_content_generation, old_content_generation + 1);
            assert_eq!(app.window_transient_store.document_generation(), old_window_generation + 1);
        } else {
            assert_eq!(app.store.envelope().id, old_parent_id);
            assert_eq!(app.child_content_root.slots(), vec![("slot".into(), "child-1".into())]);
            assert_eq!(app.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("retained live recursive branch").count, 0);
            assert!(app.child_content_root.dialect("nested", "grandchild-1").is_none());
            assert_eq!(app.child_content_generation, old_content_generation);
            assert_eq!(app.window_transient_store.document_generation(), old_window_generation);
        }
        if app.poll_artifact_store_replacement(handle) == crate::app::ArtifactEnvelopeDecodeOperationPoll::Ready {
            break;
        }
        let _ = drive_recursive_replacement_step(&mut app).expect("atomic recursive replacement drive");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Ready);
    let packs = PluginApp::child_packs(app.as_ref()).await.expect("published recursive member registry packs");
    assert_eq!(packs.iter().map(|entry| entry.child_id.as_str()).collect::<std::collections::BTreeSet<_>>(), std::collections::BTreeSet::from(["child-1", "grandchild-1"]));
    assert!(app.test_child_admission_state(1).roots_retiring_empty);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("recursive terminal replacement acknowledgement"));
    close_recursive_replacement_app(app.as_mut());
    assert_eq!(row["published"], true);
    eprintln!("[DEBUG] recursive retained replacement published the parent, two nested member stores, immutable content, ownership graph, child generation, pins, and window generation in one observable turn, then bounded-retired every displaced owner");
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_document_archive_round_trips_the_complete_owned_closure() {
    eprintln!("[DEBUG] recursive archive: constructing retained parent and two member envelopes");
    let archive = recursive_document_archive("archive-parent").await;
    let expected = archive.clone();
    eprintln!("[DEBUG] recursive archive: constructing live bundle and loading exact closure");
    let mut app = live_recursive_replacement_app().await;
    PluginApp::begin_document_archive_load(app.as_mut(), 81, archive).expect("recursive archive admission");
    let status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 81)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 81).expect("recursive archive acknowledgement");
    assert_eq!(app.store.envelope().id, "archive-parent");
    assert_eq!(app.store.snapshot().expect("archive parent current snapshot").revision, 7);
    assert_eq!(app.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("archive branch content").count, 29);
    assert_eq!(app.child_content_root.typed_read::<RecursiveBranchSnapshot>("nested", "grandchild-1").expect("archive leaf content").count, 53);
    eprintln!("[DEBUG] recursive archive: reading generation-fenced persisted closure");
    let persisted = Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("recursive archive read");
    assert_eq!(persisted.members.len(), 2);
    assert_eq!(persisted.members[0].owner.parent.artifact_id, "archive-parent");
    assert_eq!(persisted.members[1].owner.parent.artifact_id, "child-1");
    assert_eq!(persisted, expected);

    let mut malformed = persisted.clone();
    malformed.parent_pack.push(0);
    PluginApp::begin_document_archive_load(app.as_mut(), 82, malformed).expect("malformed recursive archive retains admission owner");
    let malformed_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 82)).await;
    assert_eq!(malformed_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 82).expect("malformed recursive archive acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after malformed parent"), persisted);

    let mut malformed_history = persisted.clone();
    let mut history = Box::pin(protocol::decode_history(
        &malformed_history.parent_spr,
        &protocol::os_spr::history::DecodeOptions::default(),
    ))
        .await
        .expect("decode recursive archive history fixture");
    history.changes.push(protocol::HistoryChange {
        id: "malformed-after-valid-prefix".into(),
        saved_at: "2026-09-12T00:00:00Z".into(),
        edit_ids: vec!["missing-edit-after-valid-prefix".into()],
        description: Some("must fail after retained framing and semantic decode".into()),
    });
    malformed_history.parent_spr = Box::pin(protocol::encode_history(
        &history,
        &protocol::os_spr::history::EncodeOptions {
            write_backwards_section: true,
            ..protocol::os_spr::history::EncodeOptions::default()
        },
    ))
    .await
    .expect("encode semantically malformed retained history");
    PluginApp::begin_document_archive_load(app.as_mut(), 85, malformed_history).expect("semantic recursive archive refusal retains admission owner");
    let malformed_history_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 85)).await;
    assert_eq!(malformed_history_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 85).expect("semantic recursive archive refusal acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after semantic history refusal"), persisted);

    let mut malformed_child_history = persisted.clone();
    Box::pin(append_missing_edit_reference_to_member_history(&mut malformed_child_history.members[0])).await;
    PluginApp::begin_document_archive_load(app.as_mut(), 86, malformed_child_history).expect("semantic recursive child refusal retains admission owner");
    let malformed_child_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 86)).await;
    assert_eq!(malformed_child_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 86).expect("semantic recursive child refusal acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after semantic child history refusal"), persisted);

    let parent_dialect: ArtifactDialect = ComposedParentApp::<true>::DIALECT.into();
    let child_reference = ArtifactRef {
        artifact_id: "child-1".into(),
        dialect: test_child_dialect().await,
    };
    let foreign = Box::pin(recursive_parent_document_files(
        "foreign-schema-parent",
        "semio.foreign-composed-test/v1",
        parent_dialect,
        ComposedParentSnapshot { slot: Some(ParentFixtureChild::new("child-1".into(), child_reference)), revision: 0 },
    ))
    .await;
    let mut refused = persisted.clone();
    refused.parent_pack = foreign.pack;
    refused.parent_spr = foreign.spr;
    PluginApp::begin_document_archive_load(app.as_mut(), 83, refused).expect("foreign recursive archive retains admission owner");
    let refused_status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 83)).await;
    assert_eq!(refused_status.state, protocol::DocumentArchiveLoadState::Fault);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 83).expect("foreign recursive archive acknowledgement");
    assert_eq!(Box::pin(PluginApp::document_archive(app.as_ref())).await.expect("live archive after foreign parent"), persisted);
    close_recursive_replacement_app(app.as_mut());
    eprintln!("[DEBUG] recursive archive load/read preserved the full two-level owner closure through atomic replacement and deterministic persistence; malformed and foreign-schema parents retained the prior live archive");
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_acknowledgement() {
    let archive = recursive_document_archive("cancelled-archive-parent").await;
    let mut app = live_recursive_replacement_app().await;
    let previous = app.store.envelope().id.clone();
    PluginApp::begin_document_archive_load(app.as_mut(), 84, archive).expect("recursive archive cancellation admission");
    assert!(PluginApp::acknowledge_document_archive_load(app.as_mut(), 84).is_err());
    PluginApp::cancel_document_archive_load(app.as_mut(), 84).expect("recursive archive cancellation request");
    let requested = Box::pin(PluginApp::poll_document_archive_load(app.as_mut(), 84)).await.expect("recursive archive cancellation request status");
    assert!(matches!(requested.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running));
    assert!(PluginApp::acknowledge_document_archive_load(app.as_mut(), 84).is_err());
    let status = Box::pin(drive_recursive_document_archive_load(app.as_mut(), 84)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Cancelled);
    PluginApp::acknowledge_document_archive_load(app.as_mut(), 84).expect("recursive archive cancelled acknowledgement");
    assert_eq!(app.store.envelope().id, previous);
    close_recursive_replacement_app(app.as_mut());
    eprintln!("[DEBUG] cancelled recursive archive retained its exact input owner, published no candidate, bounded-retired the two members plus parent bytes, and released only after terminal acknowledgement");
}

async fn verify_recursive_rejection_case(id: &'static str, operation: u64) {
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, id);
    let mut app = Box::pin(live_recursive_replacement_app()).await;
    let old_parent_id = app.store.envelope().id.clone();
    let old_window_generation = app.window_transient_store.document_generation();
    let old_content_generation = app.child_content_generation;
    let (handle, ingress) = Box::pin(retained_recursive_replacement_fixture(&mut app, operation, &format!("rejected-{id}"), row["mode"].as_str().expect("recursive rejection mode"))).await;
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::Complete)).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Fault, "{id}");
    assert_eq!(app.store.envelope().id, old_parent_id, "{id}");
    assert_eq!(app.child_content_root.slots(), vec![("slot".into(), "child-1".into())], "{id}");
    assert_eq!(app.child_content_root.typed_read::<RecursiveBranchSnapshot>("slot", "child-1").expect("rejection retained live recursive branch").count, 0, "{id}");
    assert_eq!(app.child_content_generation, old_content_generation, "{id}");
    assert_eq!(app.window_transient_store.document_generation(), old_window_generation, "{id}");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("bounded rejected replacement acknowledgement"), "{id}");
    close_recursive_replacement_app(app.as_mut());
    assert_eq!(row["published"], false, "{id}");
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_replacement_rejects_missing_extra_and_duplicate_members_after_bounded_retirement() {
    for (offset, id) in ["missing-grandchild", "extra-unreachable-member", "duplicate-member"].into_iter().enumerate() {
        Box::pin(verify_recursive_rejection_case(id, 750 + offset as u64)).await;
    }
    eprintln!("[DEBUG] recursive retained replacement rejected missing, unreachable-extra, and duplicate decoded member stores while bounded-retiring every request, store, snapshot read, identity, and candidate owner");
}

async fn verify_recursive_cancellation_case(id: &'static str, operation: u64) {
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, id);
    let target = match row["cancelAt"].as_str().expect("recursive cancellation phase") {
        "memberOpen" => crate::app::ActiveArtifactStoreReplacementState::OpeningMembers,
        "closure" => crate::app::ActiveArtifactStoreReplacementState::ValidatingClosure,
        "viewPreparation" => crate::app::ActiveArtifactStoreReplacementState::PreparingCandidateViews,
        other => panic!("unknown recursive cancellation phase {other}"),
    };
    eprintln!("[DEBUG] recursive replacement {id}: constructing live bundle");
    let mut app = Box::pin(live_recursive_replacement_app()).await;
    let old_parent_id = app.store.envelope().id.clone();
    let old_window_generation = app.window_transient_store.document_generation();
    let old_content_generation = app.child_content_generation;
    eprintln!("[DEBUG] recursive replacement {id}: constructing candidate ingress");
    let (handle, ingress) = Box::pin(retained_recursive_replacement_fixture(&mut app, operation, &format!("cancelled-{id}"), row["mode"].as_str().expect("recursive cancellation mode"))).await;
    eprintln!("[DEBUG] recursive replacement {id}: admitting complete member set");
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);
    eprintln!("[DEBUG] recursive replacement {id}: driving to {target:?}");
    Box::pin(drive_recursive_replacement_to(&mut app, handle, target)).await;
    if target == crate::app::ActiveArtifactStoreReplacementState::OpeningMembers {
        for _ in 0..crate::app::MAINTENANCE_STAGES {
            if app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.active_member_open.is_some()) {
                break;
            }
            let _ = drive_recursive_replacement_step(&mut app).expect("begin recursive member open before cancellation");
        }
        assert!(app.store_replacement_jobs.get(handle.operation.0).is_some_and(|active| active.active_member_open.is_some()));
    }
    eprintln!("[DEBUG] recursive replacement {id}: requesting cancellation");
    app.cancel_artifact_store_replacement(handle).expect("cancel recursive retained replacement");
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::Complete)).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Cancelled, "{id}");
    assert_eq!(app.store.envelope().id, old_parent_id, "{id}");
    assert_eq!(app.child_content_root.slots(), vec![("slot".into(), "child-1".into())], "{id}");
    assert_eq!(app.child_content_generation, old_content_generation, "{id}");
    assert_eq!(app.window_transient_store.document_generation(), old_window_generation, "{id}");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("bounded cancelled replacement acknowledgement"), "{id}");
    eprintln!("[DEBUG] recursive replacement {id}: closing retained live app");
    close_recursive_replacement_app(app.as_mut());
}

async fn verify_recursive_stale_authority_case() {
    let vectors = recursive_replacement_vectors();
    let row = recursive_replacement_case(&vectors, "stale-child-content-authority");
    eprintln!("[DEBUG] recursive replacement stale-child-content-authority: constructing live bundle");
    let mut app = Box::pin(live_recursive_replacement_app()).await;
    let old_parent_id = app.store.envelope().id.clone();
    let old_window_generation = app.window_transient_store.document_generation();
    let old_content_generation = app.child_content_generation;
    eprintln!("[DEBUG] recursive replacement stale-child-content-authority: constructing and admitting candidate");
    let (handle, ingress) = Box::pin(retained_recursive_replacement_fixture(&mut app, 769, "stale-recursive-parent", row["mode"].as_str().expect("recursive stale mode"))).await;
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::AwaitingMembers)).await;
    admit_recursive_member_set(&mut app, handle, row, ingress);
    eprintln!("[DEBUG] recursive replacement stale-child-content-authority: driving candidate ready");
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::CandidateReady)).await;
    let generation = app.admit_child_content_publication().expect("real concurrent child-content publication authority");
    app.publish_child_content_member(generation, "slot", "child-1").await.expect("real concurrent child-content publication");
    Box::pin(drive_recursive_replacement_to(&mut app, handle, crate::app::ActiveArtifactStoreReplacementState::Complete)).await;
    assert_eq!(app.poll_artifact_store_replacement(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.store.envelope().id, old_parent_id);
    assert_eq!(app.child_content_root.slots(), vec![("slot".into(), "child-1".into())]);
    assert_eq!(app.child_content_generation, old_content_generation + 1);
    assert_eq!(app.window_transient_store.document_generation(), old_window_generation);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("bounded stale replacement acknowledgement"));
    close_recursive_replacement_app(app.as_mut());
    assert_eq!(row["staleAuthority"], true);
    assert_eq!(row["published"], false);
}

#[semio_framework_async_macros::async_test]
async fn retained_window_input_recursive_replacement_cancellation_and_stale_authority_preserve_the_live_bundle() {
    for (offset, id) in ["cancel-member-open", "cancel-closure", "cancel-view-preparation"].into_iter().enumerate() {
        Box::pin(verify_recursive_cancellation_case(id, 760 + offset as u64)).await;
    }
    Box::pin(verify_recursive_stale_authority_case()).await;
    eprintln!("[DEBUG] recursive retained replacement cancellation at member-open, closure, and view phases plus a real stale content authority retained the complete live bundle and bounded-retired every unpublished owner");
}
//#endregion 🧬️ComposedParentFixture

//#region 📨️EnvelopeDecodeLadder
/// 🐢️ Field owner that yields a fixed number of decode steps before it completes, so the decode
/// ladder is measurable without a domain field catalog. `budget` steps of `Pending` on the same
/// token exercise exactly the redelivery loop the real fresh decoder runs.
struct SlowEnvelopeFieldDecoder {
    budget: usize,
    steps: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl SlowEnvelopeFieldDecoder {
    fn new(budget: usize, steps: &std::sync::Arc<std::sync::atomic::AtomicUsize>) -> Self {
        Self { budget, steps: std::sync::Arc::clone(steps) }
    }
}

impl store::ArtifactEnvelopeFieldDecoder<ComposedParentSnapshot, RecursiveFixtureMutation> for SlowEnvelopeFieldDecoder {
    fn accept_field_token(
        &mut self,
        _field_id: u16,
        _token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        self.steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        cx.consume_fuel(1);
        if self.budget == 0 {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        self.budget -= 1;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
    }

    fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Ok(store::ArtifactEnvelopeFieldDecodeStep::RecordComplete)
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        true
    }
}

fn envelope_law_pages() -> store::OwnedSchemaDecodePages {
    let wire = br#"{"schema":"semio.composed-test/v1","id":"envelope-decode-law","vcs":{}}"#;
    let chunks = wire.chunks(store::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages =
        store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: wire.len() }).expect("envelope law page credits");
    for chunk in chunks {
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded envelope law page")).unwrap_or_else(|_| panic!("pre-admitted envelope law page"));
    }
    pages.seal().expect("sealed envelope law page set");
    pages
}

fn install_slow_envelope_decode(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    budget: usize,
    steps: &std::sync::Arc<std::sync::atomic::AtomicUsize>,
) -> crate::app::ArtifactEnvelopeDecodeOperationHandle {
    let operation = semio_framework_job::OperationId(operation);
    let generation = semio_framework_job::Generation(app.store.generation_now());
    app.admit_artifact_envelope_decode_owner(operation, generation, envelope_law_pages(), Box::new(SlowEnvelopeFieldDecoder::new(budget, steps)), store::ArtifactEnvelopeDecodeCompletion::new())
        .unwrap_or_else(|_| panic!("exact envelope decode owner admission"))
}

/// 🚿️ Drives one live envelope decode to its terminal poll through the reactor-turn pump, so the
/// app can close with every field lease returned.
async fn drain_envelope_decode(app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>, handle: crate::app::ArtifactEnvelopeDecodeOperationHandle) -> crate::app::ArtifactEnvelopeDecodeOperationPoll {
    let mut poll = crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending;
    for _ in 0..100_000 {
        PluginApp::advance_typed_operation_publication(app).await.expect("one reactor turn drives the envelope decode worker");
        poll = app.advance_artifact_envelope_load(handle).expect("envelope load advancement");
        if poll != crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending {
            return poll;
        }
        semio_framework_async::yield_once().await;
    }
    poll
}

/// ⚖️ LAW: a decode that has not finished is reported as `Pending`, never as a terminal `Fault`.
/// The store replacement it will hand to does not exist until the decode is `Ready`, and reading
/// that absent job here made every in-progress document load fail closed at the first turn
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault() {
    let mut app = live_composed_replacement_app().await;
    let steps = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let handle = install_slow_envelope_decode(&mut app, 811, 64, &steps);
    assert_eq!(app.poll_artifact_envelope_decode(handle), crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending);
    assert_eq!(
        app.advance_artifact_envelope_load(handle).expect("live decode advancement"),
        crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending,
        "an unfinished decode is pending, not a terminal fault"
    );
    assert_eq!(
        app.poll_artifact_store_replacement(handle),
        crate::app::ArtifactEnvelopeDecodeOperationPoll::Fault,
        "no replacement job exists yet, which is exactly the reading that must not leak out of the load API"
    );
    drain_envelope_decode(&mut app, handle).await;
    close_member_admission_app(&mut app);
}

/// ⚖️ LAW: the reactor turn pumps the envelope decode worker to its terminal poll.
/// `ActiveArtifactEnvelopeDecode::drive` only SUBMITS a step to the process pool, and the pool runs
/// a submitted step on wasm solely when it is pumped; before this the ONLY driver was the
/// cooperative-maintenance rotation, so a document load never advanced inside an interactive turn.
#[semio_framework_async_macros::async_test]
async fn one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll() {
    let mut app = live_composed_replacement_app().await;
    let steps = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let handle = install_slow_envelope_decode(&mut app, 812, 256, &steps);
    assert!(PluginApp::has_runnable_typed_operations(&app), "a live envelope decode is runnable reactor-turn work");
    let poll = drain_envelope_decode(&mut app, handle).await;
    let driven = steps.load(std::sync::atomic::Ordering::SeqCst);
    assert_ne!(poll, crate::app::ArtifactEnvelopeDecodeOperationPoll::Pending, "the reactor-turn pump left the decode pending after {driven} steps");
    assert!(driven >= 256, "the reactor-turn pump drove only {driven} decode steps");
    assert!(!PluginApp::has_runnable_typed_operations(&app), "a retired decode is no longer runnable reactor-turn work");
    close_member_admission_app(&mut app);
}
//#endregion 📨️EnvelopeDecodeLadder
