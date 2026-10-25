//! 🎚️ Typed media vocabulary: media types, ports, compatibility, converters and artifact kind specs.

use semio_framework_hash::Sha256;
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::Number;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use serde::Deserialize;
use serde::Serialize;
use std::any::Any;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️MediaVocabulary
//#region ArtifactKind

// 🔀️ Relocated verbatim from 🔺️mesh/🦀️.rs (ticket 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT
// wave 4a) — manifest-vocabulary types, not codec material; mesh keeps MeshData/Primitives/
// generic obj-glb-stl codecs and the DWG bit-codec, but the legacy media-format enum itself was retired
// in ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — every
// format-kind field below is now a plain string kind id.
/// 🧬️ Which geometry backend a resource kind's media exporters/importers target — the manifest-level
/// counterpart threaded onto `AppDefinition.artifact_kinds` (see `ArtifactKindSpec`). Canonical home for
/// what used to be duplicated verbatim in `framework/plugin/rs` and `framework/product/os/core/rs`; both
/// now re-export this definition instead of declaring their own.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum OsMediaCapability {
    MeshOnly,
    Brep,
}

/// 🗂️ An app-declared OS resource kind (e.g. a 3D mesh format, a raster format) — the manifest-level
/// counterpart to `AppBuilder::artifact_kind(...)` (`framework/plugin/rs`), letting `framework/product/os/core`
/// build its artifact catalog from `AppDefinition.artifact_kinds` at plugin registration time instead of
/// hardcoding a per-app match on kind-id strings. Carries the manifest-level media-kind fields
/// (`media_type`/`schema`/`export_formats`/`import_formats`) directly
/// so one spec carries both the OS-catalog presentation shape and the `MediaType` a wire actually negotiates
/// — see `crate::media_types_compatible`. `OsArtifactDescriptor` (`framework/product/os/core`) threads
/// `media_type` through so registry lookups return it alongside the rest of the descriptor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactKindSpec {
    pub id: String,
    /// 🗣️ The kind's own localized name — what a picker shows for it, never its editor app's label.
    pub label: LocalizedLabel,
    pub source_format: String,
    pub component_kind: String,
    pub dimension: String,
    pub media_capability: OsMediaCapability,
    pub media_type: MediaType,
    pub schema: String,
    /// 🗄️ Export target format kind ids (string, the legacy format enum was retired — ticket 26/08/11/
    /// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    /// 🎯️ Stdio export target kind ids (e.g. `stdio.json`) — additive peer of `export_formats`.
    pub export_stdio_kinds: Vec<String>,
    /// 📥️ Stdio import source kind ids — additive peer of `import_formats`.
    pub import_stdio_kinds: Vec<String>,
}

/// 🏠️ One artifact kind a package hosts — the kind id, one document schema of its owner's codecs, and the OWNER: the plugin
/// that owns the kind (declares it and its codec), a declared dependency of the host (see `PluginManifest::hosted_artifact_kinds`).
/// The owner is explicit, never read off the kind id — an embedded surface's kind (`3d.cad`) names no plugin.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostedArtifactKind {
    pub id: String,
    pub schema: String,
    pub owner: String,
}

//#endregion ArtifactKind
//#region MediaType

/// ⌨️ Typed-media lattice: every port/wire in the workflow carries a `MediaType` (`class` × `form`) instead of the legacy string `artifact_kind`. `MediaType` is what a wire negotiates; a format kind id string is only how bytes are encoded once they actually cross a process boundary (see `MediaWireFormat`). Dependent tickets retire `OsMediaCapability` (see the `ArtifactKind` region above) onto `MediaForm::{Brep,Mesh}`, which already covers what `OsMediaCapability::{Brep,MeshOnly}` expresses.
// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `🎠️kernel/🦀️.rs`
// and `🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs` (both owned by other agents this pass) still
// embed `MediaType`/`MediaClass`/`MediaForm` by value inside plain `#[derive(Serialize,
// Deserialize)]` types — dropping serde here breaks `cargo check -p semio-framework` today. Both
// derive families stay load-bearing simultaneously until those crates migrate; revisit once they do.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
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

/// 📋️ The shape/representation a `MediaClass` payload takes, orthogonal to `class` — e.g. `ThreeD` × `Brep` vs `ThreeD` × `Mesh`. `Any` only ever appears on the accepting side of a port (see `media_types_compatible`).
// 🚧️ BLOCKED: see `MediaClass` above — same cross-crate serde dependency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
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

/// 📡️ A port or wire's declared media type — the pair a producer offers or a consumer accepts.
// 🚧️ BLOCKED: see `MediaClass` above — same cross-crate serde dependency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MediaType {
    pub class: MediaClass,
    pub form: MediaForm,
}

semio_framework_value::artifact_retire_leaf!(MediaType);

/// 🔌️ How a `MediaType` is actually encoded once it crosses a process boundary — binary payloads
/// carry a format kind id string (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6), structured payloads carry
/// a schema id instead (see `ArtifactKindSpec::schema`).
// 🚧️ BLOCKED: see `MediaClass` above — `🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs`'s
// `WorkflowMediaPort`/`MediaContract` (owned by another agent this pass) still embed this by value
// inside plain `#[derive(Serialize, Deserialize)]` types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", tag = "kind")]
pub enum MediaWireFormat {
    Binary { format_kind: String },
    Document { schema: String },
    Intrinsic { schema: String },
}

/// 🔀️ Which side of a wire a `MediaPortSpec` sits on.
// 🚧️ BLOCKED: see `MediaWireFormat` above — same cross-crate serde dependency (`WorkflowMediaPort`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum MediaPortDirection {
    In,
    Out,
}

/// 🔢️ Whether a `MediaPortSpec` accepts/produces exactly one media value or a stream/collection of them — e.g. a mesh-array input that fans in from several upstream producers.
// 🚧️ BLOCKED: see `MediaWireFormat` above — same cross-crate serde dependency (`WorkflowMediaPort`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PortMultiplicity {
    One,
    Many,
}

/// 📕️ A single port an app exposes on the workflow — `kind_id` optionally pins it to one `ArtifactKindSpec.id` when the port is more specific than its `media_type` alone conveys.
// 🚧️ BLOCKED: see `MediaWireFormat` above — `WorkflowMediaPort.spec` embeds this by value inside a
// plain `#[derive(Serialize, Deserialize)]` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MediaPortSpec {
    pub id: String,
    pub label: String,
    pub direction: MediaPortDirection,
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind_id: Option<String>,
    pub required: bool,
    pub multiplicity: PortMultiplicity,
}

/// ⚖️ Result of checking whether a producer's `MediaType` can feed a consumer's accepted `MediaType`: exact match, a known lossy-but-allowed conversion, or outright rejection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaCompat {
    Direct,
    Convert { from: MediaForm, to: MediaForm },
    Reject,
}

/// 🔃️ One-way `MediaForm` conversions the workflow is allowed to insert implicitly (e.g. a B-Rep producer feeding a mesh-only consumer). `media_types_compatible` looks up `(produced, accepted)` directly, so add the reverse pair too if a conversion should also hold the other way.
const MEDIA_FORM_CONVERSIONS: &[(MediaForm, MediaForm)] = &[(MediaForm::Brep, MediaForm::Mesh), (MediaForm::Vector, MediaForm::Raster), (MediaForm::Design, MediaForm::Kit), (MediaForm::Type, MediaForm::Kit)];

/// 🔂️ The single source of truth for wire compatibility: classes must match exactly, `MediaForm::Any` on the accepting side takes anything within the class, equal forms are always direct, and everything else falls through to the explicit `MEDIA_FORM_CONVERSIONS` table.
pub async fn media_types_compatible(produced: &MediaType, accepted: &MediaType) -> MediaCompat {
    if produced.class != accepted.class {
        return MediaCompat::Reject;
    }
    if matches!(accepted.form, MediaForm::Any) || produced.form == accepted.form {
        return MediaCompat::Direct;
    }
    for (from, to) in MEDIA_FORM_CONVERSIONS {
        if *from == produced.form && *to == accepted.form {
            return MediaCompat::Convert { from: *from, to: *to };
        }
    }
    MediaCompat::Reject
}

//#endregion MediaType
//#region 🔖️AppIo

/// 🧷️ The non-format fields of `ArtifactKindSpec` (see `ArtifactKind` region above) that describe how
/// a resource presents in the OS catalog — split out so `AppIo` can carry its own `export_formats`/
/// `import_formats` lists without duplicating `ArtifactKindSpec`'s full shape (which stays alive
/// unchanged for now; later waves retire it onto `AppIo`).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactPresentation {
    pub id: String,
    pub name: String,
    pub dimension: String,
    pub component_kind: String,
}

/// 📺️ An app's full media I/O surface — the artifact schema/type every app carries implicitly (see
/// `artifact_in_port`/`artifact_out_port`) plus whatever additional workflow ports, catalog
/// export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
/// surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
/// `media_outputs`/`artifact_kinds` onto it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AppIo {
    pub artifact_schema: String,
    pub artifact_media_type: MediaType,
    /// 🔗️ App-specific ports only — the implicit artifact ports are auto-injected by `all_ports`.
    pub ports: Vec<MediaPortSpec>,
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    pub artifact: ArtifactPresentation,
}

impl AppIo {
    /// 🔐️ The implicit `"artifact:in"` port every app accepts, keyed by `self.artifact_media_type`.
    pub async fn artifact_in_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "artifact:in".into(), label: "Artifact".into(), direction: MediaPortDirection::In, media_type: self.artifact_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 🏺️ The implicit `"artifact:out"` port every app produces — see `artifact_in_port`.
    pub async fn artifact_out_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "artifact:out".into(), label: "Artifact".into(), direction: MediaPortDirection::Out, media_type: self.artifact_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 📇️ The full port list, in stable order: the implicit artifact ports first, followed by every app-specific port declared in `self.ports`.
    pub async fn all_ports(&self) -> Vec<MediaPortSpec> {
        let mut ports = vec![self.artifact_in_port().await, self.artifact_out_port().await];
        ports.extend(self.ports.clone());
        ports
    }

    /// 🏗️ Builds an `AppIo` from just its implicit artifact surface, with no extra ports/formats declared yet — chain `.with_ports(...)` to add app-specific ports.
    pub async fn from_artifact(schema: impl Into<String>, media_type: MediaType, artifact: ArtifactPresentation) -> Self {
        Self { artifact_schema: schema.into(), artifact_media_type: media_type, ports: Vec::new(), export_formats: Vec::new(), import_formats: Vec::new(), artifact }
    }

    /// 📎️ Attaches app-specific ports (beyond the implicit artifact ports) to this `AppIo`.
    pub async fn with_ports(mut self, ports: Vec<MediaPortSpec>) -> Self {
        self.ports = ports;
        self
    }
}

impl Default for AppIo {
    fn default() -> Self {
        Self {
            artifact_schema: String::new(),
            artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact: ArtifactPresentation { id: String::new(), name: String::new(), dimension: String::new(), component_kind: String::new() },
        }
    }
}

//#endregion 🔖️AppIo
//#region Media

/// 🎞️ The value that actually flows over a workflow wire, produced by `ArtifactApp::export_media` and consumed by `ArtifactApp::import_media`. Kept separate from the `MediaType` lattice above (which only negotiates *compatibility*, never carries a value) so headless runners and the UI share one payload shape.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Media {
    pub media_type: MediaType,
    pub payload: MediaPayload,
}

/// 📦️ Intrinsic payloads own complete typed values; explicit JSON and addressed binary payloads retain their declared media forms.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", tag = "kind")]
pub enum MediaPayload {
    Structured { schema: String, json: String },
    Binary { format_kind: String, blob_hash: String },
    Intrinsic { schema: String, value: DslValue },
}


/// 🔑️ A cheap identity for one port's current output, independent of serializing the full payload — the unit the `SpaceRunner` compares to decide whether a downstream node actually needs to see a new value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaFingerprint(pub String);

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` doesn't support tuple structs —
/// wire-shaped as the bare inner string, matching serde's own default newtype-struct behavior.
impl ToValue for MediaFingerprint {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}

impl FromValue for MediaFingerprint {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        String::from_value(value).map(MediaFingerprint)
    }
}

impl MediaFingerprint {
    /// 🌱️ Canonical fingerprint of a `Media` value: structured payloads hash their JSON text, binary payloads reuse their existing content hash directly (no re-hashing bytes already addressed by the blob store).
    pub fn of(media: &Media) -> Self {
        match &media.payload {
            MediaPayload::Structured { schema, json } => MediaFingerprint(semio_framework_hash::hash_parts(&[schema.as_str(), json.as_str()])),
            MediaPayload::Binary { blob_hash, .. } => MediaFingerprint(blob_hash.clone()),
            MediaPayload::Intrinsic { schema, value } => MediaFingerprint(fingerprint(schema,value)),
        }
    }
}

/// 🚧️ Failure exporting, importing, or fingerprinting media on a declared port.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum MediaError {
    UnknownPort(String),
    Incompatible { port: String, produced: MediaType, accepted: MediaType },
    Payload(String, String),
    NotImplemented,
}

impl std::fmt::Display for MediaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPort(port) => write!(formatter, "unknown media port `{port}`"),
            Self::Incompatible { port, produced, accepted } => {
                write!(formatter, "port `{port}` produced {produced:?} but the wire accepts {accepted:?}")
            }
            Self::Payload(port, message) => write!(formatter, "media payload error on port `{port}`: {message}"),
            Self::NotImplemented => formatter.write_str("media ports are not implemented for this app"),
        }
    }
}

impl std::error::Error for MediaError {}

/// 🧾️ A registered one-way conversion the workflow may insert on a wire when `media_types_compatible` reports `MediaCompat::Convert`. Kept behind a trait (never a bare closure) so converters can be enumerated, tested, and swapped without touching the runner.
pub trait MediaConverter: Send + Sync {
    fn source_form(&self) -> impl std::future::Future<Output = MediaForm> + Send;
    fn to_form(&self) -> impl std::future::Future<Output = MediaForm> + Send;
    fn convert(&self, media: &Media) -> impl std::future::Future<Output = Result<Media, MediaError>> + Send;
}

//#endregion Media
//#endregion 🔖️MediaVocabulary
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
enum Part<'a>{Value(&'a DslValue),Name(&'a str)}

fn bytes(hash:&mut Sha256,value:&[u8]){hash.update(&(value.len()as u64).to_le_bytes());hash.update(value);}

/// 🔣️ Hash the actual borrowed intrinsic owner directly, without a JSON projection.
pub(crate) fn fingerprint(schema:&str,value:&DslValue)->String{
 let mut hash=Sha256::new();hash.update(b"semio.media.intrinsic/v1\0");bytes(&mut hash,schema.as_bytes());let mut pending=vec![Part::Value(value)];
 while let Some(part)=pending.pop(){match part{
  Part::Name(name)=>bytes(&mut hash,name.as_bytes()),
  Part::Value(value)=>match value{
   DslValue::Null=>hash.update(&[0]),
   DslValue::Bool(value)=>hash.update(&[1,u8::from(*value)]),
   DslValue::Number(Number::UInt(value))=>{hash.update(&[2]);hash.update(&value.to_le_bytes());},
   DslValue::Number(Number::Int(value))=>{hash.update(&[3]);hash.update(&value.to_le_bytes());},
   DslValue::Number(Number::Float(value))=>{hash.update(&[4]);hash.update(&value.to_bits().to_le_bytes());},
   DslValue::String(value)=>{hash.update(&[5]);bytes(&mut hash,value.as_bytes());},
   DslValue::Bytes(value)=>{hash.update(&[6]);bytes(&mut hash,value);},
   DslValue::Array(items)=>{hash.update(&[7]);hash.update(&(items.len()as u64).to_le_bytes());for value in items.iter().rev(){pending.push(Part::Value(value));}},
   DslValue::Object(members)=>{hash.update(&[8]);hash.update(&(members.len()as u64).to_le_bytes());for(name,value)in members.iter().rev(){pending.push(Part::Value(value));pending.push(Part::Name(name));}}
  }
 }}
 semio_framework_hash::hex_lower(&hash.finalize())
}

// #region 🛂️Manifest
//#region 🔖️Manifest
#[cfg(test)]
#[path = "../🧪️tests/🔬️media-vocabulary/🦀️.rs"]
mod media_vocabulary_tests;
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
