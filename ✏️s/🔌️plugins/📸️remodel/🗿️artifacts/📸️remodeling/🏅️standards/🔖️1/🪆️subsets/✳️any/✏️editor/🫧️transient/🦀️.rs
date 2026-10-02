//! 🫧️ Ephemeral local-only state of ONE exact Remodeling window: the streamed import in flight (design §15, ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). An import spans one dispatch per picked file or decoded video frame, so its
//! tool state lives here between them, in the window whose dispatch started it: the stream its first decodable tick
//! minted and its progress. A tick arriving after its import ended — committed, aborted, or aborted from another window —
//! is dropped, and the window's closing aborts its import with zero trace. Tool state is never config and never history;
//! only the committed import edit is.

use crate::editor::remodeling::modes::{analyze, capture, model};
use semio_framework_plugin::{Fault, ViewModel, WindowTransientMutation, WindowTransientOwner, WindowTransientOwnerBundle, WindowTransientOwnerRegistry, WindowTransientSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️State
/// 📥️ One streamed import in flight: the stream its first decodable tick minted (`None` before), the ticks seen and the
/// ticks the host announced (`0`: unknown, a decoded video).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RemodelingImport {
    pub stream_id: Option<String>,
    pub done: u32,
    pub total: u32,
}

/// 🫧️ A Remodeling window's transient partition: its import in flight, `None` at rest.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RemodelingWindowTransient {
    pub import: Option<RemodelingImport>,
}

semio_framework_value::artifact_retire_struct!(RemodelingImport { stream_id, done, total });
semio_framework_value::artifact_retire_struct!(RemodelingWindowTransient { import });
//#endregion 🔖️State

//#region 🔖️Mutation
/// 🔁️ The one write the partition takes: its whole next state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub enum RemodelingWindowTransientMutation {
    Snapshot { transient: RemodelingWindowTransient },
}

impl protocol::Mutation<RemodelingWindowTransient> for RemodelingWindowTransientMutation {
    type Diff = RemodelingWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set Remodeling Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "remodeling.windowtransient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, _base: &RemodelingWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()),
        }
    }
    fn inverse(&self, base: &RemodelingWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<RemodelingWindowTransient> for RemodelingWindowTransient {
    fn apply(&self, _base: &RemodelingWindowTransient) -> protocol::MutationApplyResult<RemodelingWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl semio_framework_value::retirement::RetireOwned for RemodelingWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(transient)]),
        }
    }
}
//#endregion 🔖️Mutation

//#region 🔖️Codecs
/// 📜️ JSON text form inside the partition's own envelope id.
impl store::ArtifactDsl for RemodelingWindowTransient {
    const EXTENSION: &'static str = "remodelingwindowtransient";
    fn envelope_id() -> &'static str {
        "s.remodel.remodeling.windowtransient"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        dsl::json::to_json_string(self)
    }
}

/// 🎒️ Value-form pack.
impl store::ArtifactPack for RemodelingWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        semio_framework_value::ToValue::to_value(self).encode_pack_with(options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        semio_framework_value::FromValue::from_value(value).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

impl protocol::OpText for RemodelingWindowTransientMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for RemodelingWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Codecs

//#region 🔖️Owner
/// 📏️ The retained bytes one partition write holds — refused above the store's one-item bound.
fn retained_bytes(transient: &RemodelingWindowTransient) -> Option<usize> {
    let encoded = dsl::json::to_json_string(transient).len();
    let bytes = std::mem::size_of::<RemodelingWindowTransient>().checked_add(encoded.checked_mul(2)?)?;
    (bytes <= store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).then_some(bytes)
}

fn preflight(mutation: &RemodelingWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let RemodelingWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = retained_bytes(transient).ok_or_else(|| "Remodeling window transient exceeds its retained publication envelope".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn transfer(mutation: RemodelingWindowTransientMutation) -> RemodelingWindowTransient {
    match mutation {
        RemodelingWindowTransientMutation::Snapshot { transient } => transient,
    }
}

/// 🪟️ A Remodeling window kind an import can start in: every one of them, since the import verbs live in the palette.
pub trait RemodelingWindowKind: Send + Sync + 'static {
    const ID: &'static str;
}

/// 🧊️ The Model window kind.
pub struct ModelWindow;
/// 🖼️ The Frames window kind.
pub struct FramesWindow;
/// 📊️ The Report window kind.
pub struct ReportWindow;

impl RemodelingWindowKind for ModelWindow {
    const ID: &'static str = model::windows::model::REMODELING_PLAY_WINDOW_MAIN;
}
impl RemodelingWindowKind for FramesWindow {
    const ID: &'static str = capture::windows::frames::REMODELING_PLAY_WINDOW_FRAMES;
}
impl RemodelingWindowKind for ReportWindow {
    const ID: &'static str = analyze::windows::report::REMODELING_PLAY_WINDOW_REPORT;
}

/// 🪟️ The transient owner of one Remodeling window kind; every kind holds the same partition.
pub struct RemodelingWindowTransientOwner<K>(std::marker::PhantomData<K>);

impl<K: RemodelingWindowKind> WindowTransientOwner for RemodelingWindowTransientOwner<K> {
    const WINDOW_KIND_ID: &'static str = K::ID;
    type State = RemodelingWindowTransient;
    type Mutation = RemodelingWindowTransientMutation;
    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
        WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

/// 🗂️ Registers the partition for every Remodeling window kind.
pub fn register(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
    registry.register::<RemodelingWindowTransientOwner<ModelWindow>>()?;
    registry.register::<RemodelingWindowTransientOwner<FramesWindow>>()?;
    registry.register::<RemodelingWindowTransientOwner<ReportWindow>>()
}

/// 🫧️ The partition of the window a job context names, or the resting default.
pub fn current(snapshot: Option<&WindowTransientSnapshot>) -> RemodelingWindowTransient {
    snapshot
        .and_then(|value| value.get::<RemodelingWindowTransientOwner<ModelWindow>>().or_else(|| value.get::<RemodelingWindowTransientOwner<FramesWindow>>()).or_else(|| value.get::<RemodelingWindowTransientOwner<ReportWindow>>()))
        .cloned()
        .unwrap_or_default()
}

/// 🎯️ Addresses one partition write at the window being dispatched — refused without a window, since a streamed import
/// keeps its tool state in one.
pub fn addressed(view: &ViewModel, transient: RemodelingWindowTransient) -> Result<WindowTransientMutation, Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| Fault::from("remodeling-import-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::from("remodeling-window-stale"))?;
    let mutation = RemodelingWindowTransientMutation::Snapshot { transient };
    match kind {
        model::windows::model::REMODELING_PLAY_WINDOW_MAIN => Ok(WindowTransientMutation::of::<RemodelingWindowTransientOwner<ModelWindow>>(id, mutation)),
        capture::windows::frames::REMODELING_PLAY_WINDOW_FRAMES => Ok(WindowTransientMutation::of::<RemodelingWindowTransientOwner<FramesWindow>>(id, mutation)),
        analyze::windows::report::REMODELING_PLAY_WINDOW_REPORT => Ok(WindowTransientMutation::of::<RemodelingWindowTransientOwner<ReportWindow>>(id, mutation)),
        _ => Err(Fault::from("remodeling-window-kind-unknown")),
    }
}
//#endregion 🔖️Owner
