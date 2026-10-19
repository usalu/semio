//! 🔬️ The generic JSON-valued probe document: snapshot, diff, mutation, store and the codecs it mounts.

use store::SpaceMember as _;
use crate::probe_json;

//#region 🔖️ProbeDocument

/// 🧪️ A generic, single-JSON-value document kind this crate owns outright — used ONLY to exercise
/// the real `store::sync::ArtifactHost` / `ArtifactStore` / backbone-propagation pipeline end to end
/// (§6 of the packet brief: "a commit made headlessly propagates to any live shell on the same
/// folder/hub — prove it in a test if you can") without pretending to understand any actual plugin's
/// document schema, which is host-opaque by design (this file's own module doc). Schema id
/// `os.agent.probe/v1` is namespaced under `os.agent.*` precisely so it can never collide with a real
/// plugin schema id.
pub const PROBE_SCHEMA: &str = "os.agent.probe/v1";

pub const PROBE_PACK_SCHEMA_HASH: &str = "0302ac7cf70cd2452759542a24931331935527fb9388bb6967cbc0575ccf728e";

pub const PROBE_SURFACE_ID: &str = "os.mcp.probe.editor";

pub fn probe_record_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(Some("probe"), semio_framework_dsl_record::RecordLayout::Inline, vec![semio_framework_dsl_record::FieldSpec::new(0, "value", semio_framework_dsl_record::Shape::Value)])
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned)]
pub struct ProbeSnapshot(pub semio_framework_value::DslValue);

impl Default for ProbeSnapshot {
    fn default() -> Self { Self(semio_framework_value::DslValue::Null) }
}

impl pack::snapshot::ArtifactDsl for ProbeSnapshot {
    const EXTENSION: &'static str = "probe";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        probe_json::parse(text).map(ProbeSnapshot).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, error.to_string(), semio_framework_diagnostic::TextSpan::at(0, 0)))
    }

    fn print_dsl(&self) -> String {
        probe_json::print(&self.0).expect("Probe snapshot contains a valid JSON value")
    }
}

/// 🧩️ The JSON node tree declares no artifact child or link references.
impl semio_framework_schema_composition::ArtifactCompositionFields for ProbeSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl pack::snapshot::ArtifactPack for ProbeSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        probe_json::print(&self.0).map(String::into_bytes).map_err(store::PackError::from)
    }

    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let text = std::str::from_utf8(bytes).map_err(|_| store::PackError::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "Probe JSON input is not UTF8")))?;
        probe_json::parse(text).map(ProbeSnapshot).map_err(store::PackError::from)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(probe_record_spec())
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for ProbeSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

impl store::ArtifactPackReceiving for ProbeSnapshot {
 fn receive_pack(bytes:&[u8],owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,semio_framework_value::ValueError>{
  owner.native().checkpoint()?;
  let text=std::str::from_utf8(bytes).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Probe receiving JSON input is not UTF8"))?;
  probe_json::receive_snapshot(text,owner)
 }
}

/// 🌉️ Projects the first-party tree through its original controlled value implementation.
impl semio_framework_value::ToValue for ProbeSnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        self.0.clone()
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> { semio_framework_value::ToValue::to_value_controlled(&self.0, control) }
}

impl semio_framework_value::FromValue for ProbeSnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        probe_json::validate(&value)?;
        Ok(ProbeSnapshot(value))
    }
    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { probe_json::validate_with(value, &mut |_| control.checkpoint())?; <semio_framework_value::DslValue as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Self) }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned)]
pub struct ProbeDiff(pub semio_framework_value::DslValue);

impl Default for ProbeDiff {
    fn default() -> Self { Self(semio_framework_value::DslValue::Null) }
}

impl store::DiffAlgebra<ProbeSnapshot> for ProbeDiff {


    fn is_empty(&self) -> bool {
        false
    }
}
impl store::ColdDiffAlgebra<ProbeSnapshot> for ProbeDiff {
fn inverse(&self, base: &ProbeSnapshot) -> Self {
        ProbeDiff(base.0.clone())
    }
}

impl store::MutationDiff<ProbeSnapshot> for ProbeDiff {}
impl store::ColdMutationDiff<ProbeSnapshot> for ProbeDiff {
    fn apply(&self, base: &mut ProbeSnapshot, capability: store::ApplyCapability) -> store::MutationApplyResult<()> {
        store::PreparedField::new(ProbeSnapshot(self.0.clone()),drop,capability).install(base);Ok(())
    }

    fn absorb(&mut self, other: Self) {
        self.0 = other.0;
    }
}

/// 🔄️ Preserves the first-party replacement tree and its original controlled projection.
impl semio_framework_value::ToValue for ProbeDiff {
    fn to_value(&self) -> semio_framework_value::DslValue {
        self.0.clone()
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> { semio_framework_value::ToValue::to_value_controlled(&self.0, control) }
}

impl semio_framework_value::FromValue for ProbeDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        probe_json::validate(&value)?;
        Ok(ProbeDiff(value))
    }
    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { probe_json::validate_with(value, &mut |_| control.checkpoint())?; <semio_framework_value::DslValue as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Self) }
}

/// 🧪️ Replaces the complete ordered JSON node tree.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ProbeMutation {
    SetValue(semio_framework_value::DslValue),
}

/// 🧷️ Declares the atomic whole-tree replacement operation and its language surfaces.
const PROBE_SET_VALUE_DESCRIPTOR: store::MutationLeafDescriptor = store::MutationLeafDescriptor {
    schema_version: 1,
    owner: "os/mcp/workspace/🧬️mutations/set-value",
    semantic_kind: "set-value",
    display_name: "Set Value",
    emoji: "🧪",
    aggregate_variant: "SetValue",
    payload_schema: PROBE_SCHEMA,
    text_opcode: None,
    binary_tag: None,
    invertibility: store::MutationInvertibility::ExplicitMutation,
    diff_participation: store::MutationDiffParticipation::Detect,
    outcome_classes: &[store::MutationOutcomeClass::Applied],
    composition: store::MutationComposition::Atomic,
    required_language_surfaces: &[store::MutationLanguageSurface::Rust],
};

impl store::Mutation<ProbeSnapshot> for ProbeMutation {
    type Diff = ProbeDiff;
    const DESCRIPTORS: &'static [store::MutationLeafDescriptor] = &[PROBE_SET_VALUE_DESCRIPTOR];

    fn descriptor(&self) -> &'static store::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }




}
impl store::ColdMutation<ProbeSnapshot> for ProbeMutation {
type ReferenceDiff= ProbeDiff;
fn diff(&self, _base: &ProbeSnapshot) -> store::MutationOutcome<ProbeDiff> {
        let ProbeMutation::SetValue(value) = self;
        store::MutationOutcome::new(ProbeDiff(value.clone()))
    }
fn inverse(&self, base: &ProbeSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![ProbeMutation::SetValue(base.0.clone())]
    
    })())
}
}

/// 🫴️ Moves a validated replacement payload from its single declared variant.
impl semio_framework_value::FromValue for ProbeMutation {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a one-key `{}` object, found {value:?}", PROBE_SET_VALUE_DESCRIPTOR.aggregate_variant)));
        };
        match <[(String, semio_framework_value::DslValue); 1]>::try_from(entries) {
            Ok([(key, payload)]) if key == PROBE_SET_VALUE_DESCRIPTOR.aggregate_variant => { probe_json::validate(&payload)?; Ok(ProbeMutation::SetValue(payload)) },
            Ok([(key, _)]) => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown variant `{key}`, expected `{}`", PROBE_SET_VALUE_DESCRIPTOR.aggregate_variant))),
            Err(entries) => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected exactly one variant key, found {}", entries.len()))),
        }
    }
    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint()?;
        let semio_framework_value::DslValue::Object(entries) = value else { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "Probe mutation requires a single variant object")) };
        let [(key, payload)] = entries.as_slice() else { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "Probe mutation requires exactly one variant")) };
        if key != PROBE_SET_VALUE_DESCRIPTOR.aggregate_variant { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "Probe mutation variant is unknown")) }
        probe_json::validate_with(payload, &mut |_| control.checkpoint())?;
        <semio_framework_value::DslValue as semio_framework_value::FromValue>::from_value_controlled(payload, control).map(Self::SetValue)
    }
}

impl store::OpText for ProbeMutation {
    fn print_op(&self) -> String {
        let ProbeMutation::SetValue(value) = self;
        probe_json::print(value).expect("Probe mutation contains a valid JSON value")
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        probe_json::parse(line).map(ProbeMutation::SetValue).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, error.to_string(), semio_framework_diagnostic::TextSpan::at(0, 0)))
    }
}

impl store::OpBinary for ProbeMutation {
    fn encode_op(&self) -> Result<Vec<u8>, store::ProtocolError> {
        let ProbeMutation::SetValue(value) = self;
        probe_json::print(value).map(String::into_bytes).map_err(|error| store::ProtocolError::Malformed { what: "probe-op", offset: 0, detail: error.to_string() })
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, store::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| store::ProtocolError::Malformed { what: "probe-op", offset: 0, detail: error.to_string() })?;
        probe_json::parse(text).map(ProbeMutation::SetValue).map_err(|error| store::ProtocolError::Malformed { what: "probe-op", offset: 0, detail: error.to_string() })
    }
}

pub type ProbeStore = store::ArtifactStore<ProbeSnapshot, ProbeMutation>;

/// 🔒️ Registers the probe codec exactly once per process — mirrors
/// `🏪️store/🔄️sync/🦀️.rs`'s own test-only `ensure_demo_codec_registered`, but real (not
/// `#[cfg(test)]`): the backbone actor needs the registered `ArtifactCodec` to turn a local `Edit`
/// into a schema-agnostic wire `MutationEnvelope` it can persist/replicate.
pub fn ensure_probe_codec_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        store::os_store::codec_registry::register_document_codec(store::artifact_codec::<ProbeSnapshot, ProbeMutation>(PROBE_SCHEMA)).expect("register MCP probe codec");
    });
}

/// 🔐️ The one real owner catalog every `ProbeStore` installs right after construction
/// (`ensure_probe_artifact`) — required before `ArtifactStore::drop`'s terminal-empty witness can
/// ever be reached (`install_document_store_owners_exact`'s own doc: "There is no default catalog").
/// `store::ArtifactStoreCursorDisposer` is the store crate's own canonical full-store close driver
/// (used the same way by its `#[cfg(test)] demo_closable_store_owners`): it walks every detached
/// authority — displaced retirements, returned reads, history, metadata, message ledgers,
/// conflicts, the pending report, every runtime string (applied/redo edit ids, the revision
/// accumulator, the checkpoint/local-actor ids), the tail/current snapshot roots, the backbone, the
/// causal DAG, and finally the envelope shell — in the exact order `ArtifactStore::drop`'s witness
/// checks them.
pub fn probe_store_owners() -> store::os_store::document_store_owners::DocumentStoreOwners<ProbeSnapshot, ProbeMutation> {
    store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<ProbeSnapshot, ProbeMutation>().expect("Probe first-party store retirement catalog")
}

/// 🚪️ Drains one `ProbeStore` to `ArtifactStore::drop`'s exact terminal-empty witness before
/// letting it actually drop — the production teardown a `--folder`/`--hub`-bound `semio-os-mcp`
/// process needs at shutdown (`HeadlessWorkspace`'s own `Drop`, below) and what every `workspace`/
/// `artifact` test that opens a real probe now relies on too. `close_owned_step` is bounded per call
/// (`SpaceMember`'s own contract), so this is a loop to completion, not a single call; a `Blocked`
/// step means an external owner (the backbone's shared queue — its remote `ChannelBackboneRemote`
/// end lives on the `ArtifactHost` actor this same caller already asked to close) hasn't released
/// its side yet. That release happens on the real `WorkerPool` background thread the actor runs on,
/// concurrently with this thread — never something a plain busy-spin can force — so a `Blocked` step
/// yields the thread briefly instead of re-polling immediately, giving that background thread real
/// wall-clock time to finish tearing the actor down.
pub fn close_probe_store_to_terminal(mut probe_store: ProbeStore) {
    const PROBE_STORE_CLOSE_MAXIMUM_TURNS: usize = 20_000;
    const PROBE_STORE_CLOSE_BLOCKED_BACKOFF: std::time::Duration = std::time::Duration::from_millis(1);
    for _ in 0..PROBE_STORE_CLOSE_MAXIMUM_TURNS {
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 134_217_728, maximum_release_bytes: 134_217_728, maximum_depth: 256 };
        match probe_store.close_owned_step(grant) {
            Ok(semio_framework_value::RetainedCloneStep::Complete(progress)) => {
                assert!(progress.fits(grant), "Probe store close exceeded the original caller turn");
                assert!(probe_store.close_owned_terminal_is_empty(), "probe store close driver reported Complete without its exact terminal-empty witness");
                return;
            }
            Ok(semio_framework_value::RetainedCloneStep::Progress(progress)) => {
                assert!(progress.fits(grant), "Probe store close exceeded the original caller turn");
                if progress == semio_framework_value::RetainedCloneProgress::default() { std::thread::sleep(PROBE_STORE_CLOSE_BLOCKED_BACKOFF); }
            },
            Err(error) => {
                eprintln!("[probe-store-close] cannot reach terminal-empty: {error}");
                return;
            }
        }
    }
    eprintln!("[probe-store-close] did not reach terminal-empty within its bounded turn budget");
}

//#endregion 🔖️ProbeDocument
