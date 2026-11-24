//! 🗂️ Natural file and media artifacts: the natural file codec, decode steps and cursors, the mounted natural file actions, the media artifact descriptor, errors and operation failures, the document load fault codes and the media fault helpers. See [`super`].

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use pack::snapshot::ArtifactPack;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, AppDefinition, ArgPresentation, CapabilityAudience, Fault, FaultCode, FaultOrigin, Media, MediaError, MediaPayload, MediaType, MediaWireFormat, OPEN_ARTIFACT_FILE_ACTION_ID, SAVE_ARTIFACT_FILE_ACTION_ID};
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::{DslValue, RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_plugin_decl_emission::Emit;
use semio_plugin_decl_media::media_intrinsic::media_projection_error;
use semio_plugin_decl_media::media_operation_refusal;
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_ui_locale::LocalizedLabel;

/// 📄️ One editor-owned natural-file codec, matched exactly to its declared representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaturalFileCodec {
    pub format_kind: &'static str,
    pub extension: &'static str,
    pub media_type: &'static str,
    pub binary: bool,
}

/// 🧭️ Monotonic counters reported by one format-owned natural-file decoder.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NaturalFileDecodeProgress {
    pub consumed_input_bytes: usize,
    pub decoded_items: usize,
    pub owned_output_bytes: usize,
    pub completed_work_units: u64,
}

/// 🧩️ One bounded turn of a format-owned natural-file decoder.
pub enum NaturalFileDecodeStep<T> {
    Yield,
    Progress(NaturalFileDecodeProgress),
    Complete { snapshot: T, progress: NaturalFileDecodeProgress },
}

/// 📂️ Format-owned incremental decode authority used by the reserved Open route.
pub trait NaturalFileDecodeCursor<T>: Send {
    fn advance(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<NaturalFileDecodeStep<T>, MediaError>;
    fn begin_close(&mut self);
    fn close_step(&mut self, effect:&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<PluginLifecycleStep, ValueError>;
    fn retirement_demands(&self, body:usize) -> Result<RetirementDemand,ValueError>;
    fn terminal_is_empty(&self) -> bool;

    /// 🧹️ Chains quoted close actions inside one funded effect: every action runs against what the turn's retirement grant still owns, so a turn retires as much as its grant, fuel and work admit instead of one action.
    fn close_chained(&mut self, effect:&mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<PluginLifecycleStep, ValueError> {
        use semio_framework_value::retained_clone::{retained_clone_grant_admits, RetainedCloneGrant, RetainedCloneProgress};
        let grant = effect.grant();
        let left = |total: RetainedCloneProgress| RetainedCloneGrant {
            maximum_items: grant.maximum_items.saturating_sub(total.copied_items),
            maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(total.copied_bytes),
            maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(total.retained_capacity_bytes),
            maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(total.released_bytes),
            maximum_depth: grant.maximum_depth,
        };
        let mut total = RetainedCloneProgress::default();
        for _ in 0..NATURAL_CLOSE_CHAIN_ACTIONS {
            let work_before = effect.performed_work();
            let quoted = self.retirement_demands(left(total).maximum_copy_bytes)?;
            let step = { let mut nested = effect.nested(left(total)); self.close_step(&mut nested)? };
            let complete = matches!(step, PluginLifecycleStep::Complete(_));
            let progress = match &step { PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => *progress, _ => RetainedCloneProgress::default() };
            let requoted = self.retirement_demands(left(total).maximum_copy_bytes)?;
            let moved = progress != RetainedCloneProgress::default() || effect.performed_work() != work_before || requoted != quoted;
            total = RetainedCloneProgress {
                copied_items: total.copied_items + progress.copied_items, copied_bytes: total.copied_bytes + progress.copied_bytes,
                retained_capacity_bytes: total.retained_capacity_bytes + progress.retained_capacity_bytes, released_bytes: total.released_bytes + progress.released_bytes,
            };
            if complete { return Ok(PluginLifecycleStep::Complete(total)); }
            if !moved || self.terminal_is_empty() || effect.remaining_work_units()? < 2 { break; }
            let next = left(total);
            if next.maximum_items == 0 || !retained_clone_grant_admits(next, self.retirement_demands(next.maximum_copy_bytes)?) { break; }
        }
        Ok(PluginLifecycleStep::Progress(total))
    }
}

/// 🧹️ Upper bound of chained close actions one decoder takes inside a single turn.
const NATURAL_CLOSE_CHAIN_ACTIONS: usize = 1 << 14;

/// 📦️ Exact owner chosen by an editor for natural-file decoding.
pub enum NaturalFileDecodeOwner<T> {
    Controlled(Box<dyn NaturalFileDecodeCursor<T>>),
    Synchronous(Vec<u8>),
}

/// 📄️ The ABI port reserved for an editor's mounted natural-file codec.
pub const NATURAL_FILE_PORT: &str = semio_framework::NATURAL_FILE_PORT_ID;

pub fn natural_file_input_media(codec: NaturalFileCodec, media_type: MediaType, data: Vec<u8>) -> Media {
    Media { media_type, payload: MediaPayload::Intrinsic { schema: codec.format_kind.to_string(), value: DslValue::Bytes(data) } }
}

/// 🌱️ The load/genesis emission of one decoded whole document — what a natural-file import and the default `document:in`
/// importer publish: `Effect::LoadDocument` carrying the document's pack over a fresh empty history, exactly like a catalogue
/// example load. It is not a mutation and writes no history row; the runtime stamps the live store's identity on it.
pub fn document_load_emit<S: ArtifactPack + ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ::semio_os_kernel::StoreState, Mutation, ConfigMutation, DraftMutation>(snapshot: &S, schema: &str) -> Emit<Mutation, ConfigMutation, DraftMutation> {
    let pack = snapshot.encode_pack();
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("", schema));
    Emit { effects: vec![Effect::LoadDocument { pack, spr }], ui_scope: UiDirtyScope::Full, ..Emit::default() }
}

/// 🎞️ Reference imports require an explicitly declared typed snapshot codec.
pub fn document_reference_load_emit<S: store::os_store::serialization::ArtifactStoreSnapshot + ::semio_os_kernel::StoreState,Mutation,ConfigMutation,DraftMutation>(snapshot:&S,schema:&str)->Result<Emit<Mutation,ConfigMutation,DraftMutation>,MediaError>{
 let codec=S::reference_snapshot_codec().ok_or(MediaError::NotImplemented)?;
 let pack=(codec.encode)(snapshot).map_err(|error|MediaError::Payload("artifact:in".into(),error.to_string()))?;
 let spr=::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("",schema));
 Ok(Emit{effects:vec![Effect::LoadDocument{pack,spr}],ui_scope:UiDirtyScope::Full,..Emit::default()})
}

pub fn natural_file_input_bytes<'a>(port: &str, codec: NaturalFileCodec, media: &'a Media) -> Result<&'a [u8], MediaError> {
    let MediaPayload::Intrinsic { schema, value: DslValue::Bytes(bytes) } = &media.payload else {
        return Err(MediaError::Payload(port.to_string(), "natural file importer requires intrinsic owned bytes".into()));
    };
    if schema != codec.format_kind {
        return Err(MediaError::Payload(port.to_string(), format!("natural file schema mismatch: expected {}, found {schema}", codec.format_kind)));
    }
    Ok(bytes)
}

fn natural_file_action_arg(id: &str, label_en: &str, label_de: &str, value: &impl semio_framework_value::ToValue) -> ActionArgDef {
    let mut argument = ActionArgDef::text(id, LocalizedLabel::native(label_en, label_de)).default_value(value);
    argument.presentation = Some(ArgPresentation::Hidden);
    argument
}

pub fn natural_file_action_definitions(codec: NaturalFileCodec) -> [ActionDefinition; 2] {
    let args = || {
        vec![
            natural_file_action_arg("formatKind", "Format kind", "Formatart", &codec.format_kind),
            natural_file_action_arg("extension", "File extension", "Dateiendung", &codec.extension),
            natural_file_action_arg("mediaType", "Media type", "Medientyp", &codec.media_type),
            natural_file_action_arg("binary", "Binary", "Binär", &codec.binary),
        ]
    };
    [
        ActionDefinition::resumable_framework(SAVE_ARTIFACT_FILE_ACTION_ID, LocalizedLabel::native("Save File", "Datei speichern"), ActionKind::Shell, "save").with_category("transfer").with_args(args()).audience(CapabilityAudience::Chrome),
        ActionDefinition::resumable_framework(OPEN_ARTIFACT_FILE_ACTION_ID, LocalizedLabel::native("Open File…", "Datei öffnen…"), ActionKind::Shell, "folder-open")
            .with_category("transfer")
            .with_args(args())
            .audience(CapabilityAudience::Chrome),
    ]
}

pub fn mount_natural_file_actions(definition: &mut AppDefinition, codec: NaturalFileCodec) {
    if !definition.io.export_formats.iter().any(|format| format == codec.format_kind) {
        definition.io.export_formats.push(codec.format_kind.to_string());
    }
    if !definition.io.import_formats.iter().any(|format| format == codec.format_kind) {
        definition.io.import_formats.push(codec.format_kind.to_string());
    }
    for action in natural_file_action_definitions(codec) {
        for window in definition.window_kinds.iter_mut() {
            if !window.actions.iter().any(|entry| entry.id == action.id) {
                window.actions.push(action.clone());
            }
        }
    }
}

/// 🎞️ Rust mirror of WIT's `media-artifact.descriptor-json` JSON shape. `edge_id`/`port_id`/`kind_id`
/// are dataflow-driver bookkeeping the SDK default below leaves untouched (opaque pass-through for the
/// caller); `media_type` is the declared `MediaType` the wire claims to satisfy; `wire` is the actual
/// encoding; `blob_hash` is set instead of inline `data` when the payload already lives in the host's
/// blob store.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive)]
#[value(rename_all = "camelCase")]
pub struct MediaArtifactDescriptor {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edge_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub port_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    pub wire: MediaWireFormat,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blob_hash: Option<String>,
}

/// 🚧️ Failure producing/consuming a `MediaArtifact` through `PluginApp::{produce_media, consume_media}`.
#[derive(Debug, semio_framework_value::RetireOwned)]
pub enum MediaArtifactError {
    Payload(String),
    Value(ValueError),
    Export(MediaError),
    Import(Fault),
    SchemaMismatch { expected: String, found: String },
    NoImporter(String),
}

impl semio_framework_job::OriginalDomainFailure for MediaArtifactError{
 fn domain_code(&self)->&str{match self{Self::Payload(_)=>"media.artifact.payload",Self::Value(_)=>"media.artifact.value",Self::Export(_)=>"media.artifact.export",Self::Import(_)=>"media.artifact.import",Self::SchemaMismatch{..}=>"media.artifact.schema-mismatch",Self::NoImporter(_)=>"media.artifact.no-importer"}}
 fn domain_message(&self)->&str{match self{Self::Payload(message)|Self::NoImporter(message)=>message,Self::Value(error)=>&error.message,Self::Import(error)=>&error.message,Self::SchemaMismatch{expected,..}=>expected,Self::Export(error)=>match error{MediaError::UnknownPort(port)|MediaError::Incompatible{port,..}=>port,MediaError::Payload(_,message)=>message,MediaError::NotImplemented=>"media ports are not implemented for this app"}}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match self{Self::Value(error)=>error.retained_progress(),Self::Import(error)=>error.retained_progress(),_=>Default::default()}}
}

impl semio_framework_job::OriginalDomainProjection for MediaArtifactError{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,ValueError>{use semio_framework_job::OriginalDomainView as V;match path{
  []=>Ok(V::IntrinsicObject(2)),[0]=>Ok(V::IntrinsicText(match self{Self::Payload(_)=>"payload",Self::Value(_)=>"value",Self::Export(_)=>"export",Self::Import(_)=>"import",Self::SchemaMismatch{..}=>"schemaMismatch",Self::NoImporter(_)=>"noImporter"})),
  [1,rest @ ..]=>match self{Self::Payload(message)|Self::NoImporter(message)if rest.is_empty()=>Ok(V::IntrinsicText(message)),Self::Value(error)=>semio_framework_job::project_original_value_error(error,rest),Self::Import(error)=>semio_framework_job::project_original_fault(error,rest),Self::Export(error)=>media_operation_refusal::project_media_error(error,rest),Self::SchemaMismatch{expected,found}=>match rest{[]=>Ok(V::IntrinsicObject(2)),[0]=>Ok(V::IntrinsicText(expected)),[1]=>Ok(V::IntrinsicText(found)),_=>Err(media_projection_error())},_=>Err(media_projection_error())},_=>Err(media_projection_error())
 }}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{[]=>["kind","original"].get(index).copied().ok_or_else(media_projection_error),[1,rest @ ..]=>match self{Self::Value(error)=>semio_framework_job::key_original_value_error(error,rest,index),Self::Import(error)=>semio_framework_job::key_original_fault(error,rest,index),Self::Export(error)=>media_operation_refusal::key_media_error(error,rest,index),Self::SchemaMismatch{..}if rest.is_empty()=>["expected","found"].get(index).copied().ok_or_else(media_projection_error),_=>Err(media_projection_error())},_=>Err(media_projection_error())}}
}

/// 🧳️ Preserves media artifact domain refusals beside complete initiating turn failures.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum MediaArtifactOperationFailure{Domain(MediaArtifactError),Original(semio_framework_job::OriginalTurnFailure)}

impl From<MediaArtifactError> for MediaArtifactOperationFailure{fn from(source:MediaArtifactError)->Self{Self::Domain(source)}}

impl From<semio_framework_job::OriginalTurnFailure> for MediaArtifactOperationFailure{fn from(source:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(source)}}

impl From<ValueError> for MediaArtifactOperationFailure{fn from(source:ValueError)->Self{Self::Original(source.into())}}

impl From<Fault> for MediaArtifactOperationFailure{fn from(source:Fault)->Self{Self::Original(source.into())}}

impl std::fmt::Display for MediaArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Payload(message) => formatter.write_str(message),
            Self::Value(error) => std::fmt::Display::fmt(error,formatter),
            Self::Export(error) => std::fmt::Display::fmt(error,formatter),
            Self::Import(error) => formatter.write_str(&error.message),
            Self::SchemaMismatch { expected, found } => write!(formatter, "document schema mismatch: expected {expected}, found {found}"),
            Self::NoImporter(format) => write!(formatter, "no binary importer registered for format {format:?}"),
        }
    }
}

impl std::error::Error for MediaArtifactError {
    fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::Value(error)=>Some(error),Self::Export(error)=>Some(error),_=>None}}
}

impl From<ValueError> for MediaArtifactError {
    fn from(error:ValueError)->Self{Self::Value(error)}
}

/// 🎞️ What consuming one `MediaArtifact` asks of the runtime: the import was applied (an ordinary, undoable edit or the app's
/// own typed operation), or the artifact is a whole document of the app's own schema, whose stamped archive the runtime admits
/// as a stepped document archive load (`📓️api-stepped-document-load.md` §4) — a whole document never folds inside the call.
#[derive(Debug)]
pub enum MediaConsumption {
    Applied,
    DocumentLoad(protocol::DocumentArchivePack),
}

/// 🚫️ The framework code a whole document of a foreign schema is refused with (`{found}`, `{expected}`; notice in the framework
/// table `FRAMEWORK_FAULT_NOTICE_LABELS`).
pub const MEDIA_SCHEMA_MISMATCH_CODE: &str = "plugin.media.schema-mismatch";

/// 🪧️ The codes of the whole-document load and save surface; each has its framework notice in every locale (kernel table
/// `FRAMEWORK_FAULT_NOTICE_LABELS`).
pub const DOCUMENT_LOAD_UNAVAILABLE_CODE: &str = "plugin.document-load.unavailable";

pub const DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE: &str = "plugin.document-load.too-many-members";

pub const DOCUMENT_LOAD_TOO_LARGE_CODE: &str = "plugin.document-load.too-large";

pub const DOCUMENT_LOAD_BUSY_CODE: &str = "plugin.document-load.busy";

pub const DOCUMENT_LOAD_INCOMPLETE_CODE: &str = "plugin.document-load.incomplete";

pub const DOCUMENT_LOAD_MEMBER_INVALID_CODE: &str = "plugin.document-load.member-invalid";

pub const DOCUMENT_LOAD_HISTORY_INVALID_CODE: &str = "plugin.document-load.history-invalid";

pub const DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE: &str = "plugin.document-load.operation-unknown";

pub const DOCUMENT_LOAD_CHANGED_CODE: &str = "plugin.document-load.changed";

pub const DOCUMENT_LOAD_FAILED_CODE: &str = "plugin.document-load.failed";

pub const DOCUMENT_LOAD_OTHER_DOCUMENT_CODE: &str = "plugin.document-load.other-document";

pub const DOCUMENT_LOAD_MEMBERS_DIFFER_CODE: &str = "plugin.document-load.members-differ";

/// 🚫️ A named refusal of the whole-document load and save surface: `code` is one of the `DOCUMENT_LOAD_*_CODE`s, the message
/// keeps the engineering detail a log reads.
pub fn document_load_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(code), message)
}

/// 🎞️ The archive a [`whole_document_media`] carrier holds.
pub async fn whole_document_archive(data: &[u8]) -> Result<protocol::DocumentArchivePack, MediaArtifactError> {
    let text = std::str::from_utf8(data).map_err(|_| MediaArtifactError::Payload("a whole-document media carrier is pk: base64 text".into()))?;
    let bytes = store::pack_rt::pack_value_from_base64(text).map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
    protocol::decode_document_archive_bytes(&bytes).await.map_err(|error| MediaArtifactError::Payload(error.to_string()))
}

/// 🚫️ The fault a refused media artifact answers: a foreign document schema under [`MEDIA_SCHEMA_MISMATCH_CODE`] with both schemas
/// as notice parameters, anything else as the runtime's internal fault.
pub fn media_artifact_fault(error: MediaArtifactError) -> Fault {
    match error {
        MediaArtifactError::SchemaMismatch { expected, found } => Fault::new(FaultOrigin::Framework, FaultCode::new(MEDIA_SCHEMA_MISMATCH_CODE), format!("document schema mismatch: expected {expected}, found {found}")).with_param("expected", expected).with_param("found", found),
        other => Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), other.to_string()),
    }
}
