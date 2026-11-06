//! 🧾️ Exact native shapes retain their canonical serializer field declarations.
use super::*;
use semio_framework_kernel::*;
use semio_framework_kernel_identity::*;
use semio_framework_kernel_media_jobs::*;
use semio_framework_kernel_ui::UiTurnPatches;
use semio_ui_contract::PresenceUpdate;
use semio_framework_actor::JobCheckpoint;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,DslValue};
use semio_framework_manifest_media::*;
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TurnResult {
    pub retained_receipt: RetainedTurnReceipt,
    pub ui_patches: UiTurnPatches,
    pub effects: Vec<Effect>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub presence: Vec<PresenceUpdate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub next_wake: Option<u64>,
    pub status: TurnStatus,
    pub fuel_used: u64,
    pub command_ingress: CommandIngressStatus,
    pub cold_pair_ingress: ColdPairIngressStatus,
    pub lifecycle_receipt: Option<ActorInstanceLifecycleReceipt>,
    pub ui_patch_receipt: Option<ActorUiPatchReceipt>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct RetainedTurnReceipt{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub work_units_spent:u64,pub remaining:RetainedCloneGrant,pub remaining_fuel:u64}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Effect {
    OpenWindow {
        req: RequestId,
        kind: WindowKindId,
        params: DslValue,
    },
    CloseWindow {
        window: WindowHandle,
    },
    Notify {
        message: String,
    },
    ClipboardWrite {
        fragment: ClipboardFragment,
    },
    RequestSync,
    Navigate {
        uri: String,
    },
    LoadDocument {
        pack: Vec<u8>,
        spr: Vec<u8>,
    },
    OpenExternalUrl {
        url: String,
    },
    SetPanel {
        panel_json: String,
    },
    DownloadMediaExport {
        filename: String,
        mime_type: String,
        data: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
    },
    IconRenderExport {
        items: Vec<IconRenderExportItem>,
    },
    VideoRenderExport {
        filename: String,
        program: VideoRenderProgram,
    },
    RequestFileOpen {
        req: RequestId,
        accept: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        read_as: Option<String>,
        import_action: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
        #[value(default)]
        multiple: bool,
    },
    RequestMediaFrames {
        req: RequestId,
        accept: String,
        frame_action: String,
        done_action: String,
        fallback_action: String,
        #[value(default)]
        sample_stride: u32,
        #[value(default)]
        max_frames: u32,
        #[value(default)]
        max_long_edge_px: u32,
        #[value(default)]
        fps_hint: f64,
        #[value(default, skip_serializing_if = "Option::is_none")]
        payload: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    SpawnPluginInstance {
        req: RequestId,
        plugin_id: String,
        app_id: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        document_json: Option<String>,
    },
    OpenPluginInstance {
        plugin_id: String,
        app_id: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
    },
    SetActiveUtility {
        window_id: String,
        utility_id: String,
    },
    SetActiveTool {
        tool_id: String,
    },
    OpenDialog {
        req: RequestId,
        dialog_id: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    DispatchAction {
        req: RequestId,
        action: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
        delay_ms: u64,
    },
    ReplayShellCommand {
        action_id: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    InvokeExtension {
        req: RequestId,
        extension_id: String,
        capability: String,
        request_json: String,
    },
    SendMessage {
        target: MessageEndpoint,
        payload: Vec<u8>,
    },
    PublishEvent {
        topic: String,
        payload: Vec<u8>,
    },
    BlobWrite {
        req: RequestId,
        media_type: MediaType,
        bytes: Vec<u8>,
    },
    BlobLoad {
        req: RequestId,
        hash: String,
    },
    HttpRequest {
        req: RequestId,
        method: String,
        url: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        headers: Vec<(String, String)>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        body: Option<Vec<u8>>,
        #[value(default)]
        stream: bool,
    },
    DocumentRead {
        req: RequestId,
        doc: ArtifactHandle,
        lane: String,
    },
    DocumentWrite {
        req: RequestId,
        doc: ArtifactHandle,
        lane: String,
        ops: Vec<u8>,
    },
    LinkResolve {
        req: RequestId,
        link: String,
    },
    RegistryQuery {
        req: RequestId,
        kind: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        filter: Option<DslValue>,
    },
    IoCompose {
        req: RequestId,
        key: String,
        sources: Vec<String>,
    },
    CacheDerive {
        req: RequestId,
        engine_id: String,
        input: Vec<u8>,
    },
    CacheRead {
        req: RequestId,
        engine_id: String,
        key: String,
    },
    SetTimer {
        id: u64,
        after_ms: u64,
        #[value(default)]
        repeat: bool,
    },
    SpawnJob {
        job: u64,
        kind: String,
        input: Vec<u8>,
        placement: JobPlacement,
    },
    CancelJob {
        job: u64,
    },
    Respond {
        req: RequestId,
        result: RequestOutcome,
    },
    StorageRead {
        req: RequestId,
        key: String,
    },
    StorageWrite {
        req: RequestId,
        key: String,
        bytes: Vec<u8>,
    },
    StorageDelete {
        req: RequestId,
        key: String,
    },
    RequestCapability {
        req: RequestId,
        capability: CapabilityRequest,
    },
    ReleaseCapability {
        id: CapabilityId,
    },
    Subscribe {
        topic: String,
    },
    Unsubscribe {
        topic: String,
    },
    RequestServiceOperation {
        owner: String,
        service_id: String,
        action: String,
        payload: DslValue,
    },
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum TurnStatus {
    Idle,
    MoreWork,
    CheckpointReady { checkpoint: semio_framework_actor::JobCheckpoint },
    Faulted(Vec<u8>),
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum CommandIngressStatus {
    Idle,
    PageAccepted(CommandPageCursor),
    Backpressure(CommandPageCursor),
    CommandPending(CommandPageCursor),
    CommandComplete(CommandPageCursor),
    Fault { cursor: CommandPageCursor, fault: Vec<u8> },
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ColdPairIngressStatus {
    Idle,
    PageAccepted(ColdDocumentPairCursor),
    Backpressure(ColdDocumentPairCursor),
    Loading(ColdDocumentPairCursor),
    Applied(ColdDocumentPairApplied),
    Fault {
        cursor: ColdDocumentPairCursor,
        fault: Vec<u8>,
    },
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum ActorInstanceLifecycleReceipt {
    Captured {
        lifetime: ActorInstanceLifetime,
        #[value(with = "request_sequence")]
        request_sequence: u64,
    },
    Accepted {
        lifetime: ActorInstanceLifetime,
        #[value(with = "request_sequence")]
        request_sequence: u64,
        #[value(with = "decimal_generation")]
        close_generation: u64,
    },
    Retired {
        lifetime: ActorInstanceLifetime,
        #[value(with = "request_sequence")]
        request_sequence: u64,
        #[value(with = "decimal_generation")]
        close_generation: u64,
    },
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorUiPatchReceipt {
    pub lifetime: ActorInstanceLifetime,
    #[value(with = "decimal_generation")]
    pub patch_sequence: u64,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct RetainedTurnInput{pub operation:u64,pub generation:u64,pub epoch:u64,pub work:OriginalActorTurnWork,pub grant:RetainedCloneGrant}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct RequestId(pub u64);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct WindowKindId(pub String);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ClipboardFragment {
    pub schema: String,
    pub media_type: MediaType,
    pub dsl_text: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pack_bytes: Option<Vec<u8>>,
    pub source_app: String,
    pub label: String,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct IconRenderExportItem {
    pub filename: String,
    pub request: DslValue,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderProgram {
    pub schema: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub background: [f64; 4],
    pub paths: Vec<VideoRenderPath>,
    pub images: Vec<VideoRenderImage>,
    pub scenes: Vec<VideoRenderScene>,
    pub timeline: Vec<VideoRenderRun>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum MessageEndpoint {
    Shell { instance: PluginInstanceId },
    Backbone { uri: String },
    PluginInstance { id: PluginInstanceId },
    Extension { id: String },
    Topic { name: String },
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct MediaType {
    pub class: MediaClass,
    pub form: MediaForm,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum JobPlacement {
    Inline,
    Isolated,
    Exclusive,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum RequestOutcome {
    Ok(Vec<u8>),
    Err(Vec<u8>),
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct CapabilityRequest {
    pub id: CapabilityId,
    pub scope: String,
    pub reason: String,
    #[value(default)]
    pub optional: bool,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct CapabilityId(pub String);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
pub struct JobCheckpoint {
    pub state: Vec<u8>,
    pub applied_progress: u64,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct CommandPageCursor {
    pub owner: u64,
    pub generation: u64,
    pub command_index: u32,
    pub command_count: u32,
    pub instance: u32,
    pub seq: u64,
    pub kind: u8,
    pub page_index: u32,
    pub page_count: u32,
    pub item_count: u32,
    pub metadata: u32,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairCursor {
    pub lifetime: ActorInstanceLifetime,
    pub transfer_generation: u64,
    pub page_index: u32,
    pub page_count: u32,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairApplied {
    pub lifetime: ActorInstanceLifetime,
    pub transfer_generation: u64,
    pub baseline_frontier: ColdDocumentPairFrontier,
    pub aggregate_sha256: [u8; 32],
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceLifetime {
    #[value(with = "decimal_generation")]
    pub activation_generation: u64,
    pub instance_id: u32,
    #[value(with = "decimal_generation")]
    pub guest_lifetime: u64,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct OriginalActorTurnWork{pub clock_binding:u64,pub absolute_deadline_us:u64,pub fuel:u64}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderPath {
    pub verbs: String,
    pub points: Vec<f64>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderImage {
    pub url: String,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderScene {
    pub ops: Vec<VideoRenderOp>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderRun {
    pub scene: u32,
    pub frames: u32,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct PluginInstanceId(pub String);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum MediaClass {
    TwoD,
    ThreeD,
    Text,
    Data,
    Graph,
    Kit,
    Computation,
    Presentation,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum MediaForm {
    Any,
    Vector,
    Raster,
    Brep,
    Mesh,
    Document,
    Value,
    Dag,
    Trinity,
    Type,
    Design,
    Kit,
    Flow,
    Sequence,
    Procedure,
    Deck,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColdDocumentPairFrontier {
    pub document_id: String,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_sha256: [u8; 32],
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderOp {
    Fill { path: u32, transform: [f64; 6], color: [f64; 4] },
    Stroke { path: u32, transform: [f64; 6], color: [f64; 4], width: f64 },
    Image { image: u32, crop: [f64; 4], transform: [f64; 6], opacity: f64 },
}
}
