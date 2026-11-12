//! 🌉 Native ADD2 and glTF artifact composition with bounded, atomic geometry projection.
use pack::snapshot::ArtifactPack;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_diagnostic::{Diagnostic, Severity, TextSpan};
use semio_plugin::io::{AnalyzeSource, ComposeError, ComposeSource, ComposerEntry, Composition, Confidence};
use semio_plugin::ArtifactComposer;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::{geometry, snapshot::{IfcSnapshot, IfcValue}};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::{io as gltf_io, schema::snapshot::*};
use std::sync::OnceLock;
use semio_os_kernel::io::{io_mechanism::{self, ArchiveChildren, Deserializer, IoEntry, IoRunControl, Serializer}, io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult}};
use pack::snapshot::{ArtifactNativeSnapshot, NativeSnapshotEncoding, NativeSnapshotInput};
use semio_s_artifact_stdio_ifc4x3_step as io;
mod validation {
    pub use semio_s_artifact_stdio_ifc4x3_express::metadata;
}
#[path="native_pack.rs"]
pub mod original_pack;
#[path = "native_receiving.rs"]
mod native_receiving;
pub use native_receiving::{decoded, decoded_nested, encoded, failure, nested, nested_value, refused, scoped, settled};
#[path = "native-gltf-owned.rs"]
mod gltf_owned;

#[path = "native_gltf.rs"]
pub mod projection;
#[path = "native_graph.rs"]
pub mod graph;
#[path = "native_bim.rs"]
pub mod bim;
pub use projection::{gltf_to_ifc_controlled, ifc_to_gltf_controlled};

pub const GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

/// ⏱ Caller-owned progress authority; false cancels before publishing an artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExchangeProgress { pub phase: &'static str, pub completed: usize, pub total: usize }

/// 🚫 Atomic projection refusal, separate from successful lossy geometry diagnostics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeError { pub code: &'static str, pub message: String }
impl std::fmt::Display for ExchangeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, "{}: {}", self.code, self.message) }
}
impl std::error::Error for ExchangeError {}

pub fn error(code: &'static str, message: impl Into<String>) -> ExchangeError { ExchangeError { code, message: message.into() } }
pub fn checkpoint(control: &mut dyn FnMut(ExchangeProgress) -> bool, phase: &'static str, completed: usize, total: usize) -> Result<(), ExchangeError> {
    if control(ExchangeProgress { phase, completed, total }) { Ok(()) } else { Err(error("exchange.cancelled", "Geometry exchange cancelled; no artifact published")) }
}
pub fn loss(message: impl Into<String>) -> Diagnostic {
    let mut value = Diagnostic::error("exchange.geometry-only", TextSpan::default(), message);
    value.severity = Severity::Warning;
    value
}
pub fn composition_error(value: impl std::fmt::Display) -> ComposeError { ComposeError { message: value.to_string(), diagnostics: vec![] } }

/// 📤 Native IFC4x3 snapshot packs and ADD2 source compose into real glTF snapshot packs.
pub struct IfcToGltf;
impl ArtifactComposer for IfcToGltf {
    type Snapshot = GltfSnapshot;
    const WRITES: Dialect = GLTF_DIALECT;
    fn reads() -> &'static [Dialect] { &[crate::io::IFC4X3_DIALECT] }
    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let [source] = sources else { return Err(composition_error("Exactly one IFC4x3 source is required")); };
        if source.dialect != crate::io::IFC4X3_DIALECT { return Err(composition_error("Expected IFC4x3 source dialect")); }
        let snapshot = match &source.payload {
            AnalyzeSource::Text(text) => crate::io::parse_ifc4x3_step(text).map_err(composition_error)?,
            AnalyzeSource::Binary(bytes) => IfcSnapshot::decode_pack(bytes).map_err(composition_error)?,
        };
        ifc_to_gltf_controlled(&snapshot, &mut |_| true).map_err(composition_error)
    }
}

/// 📥 Native glTF packs, JSON and embedded GLB compose into an ADD2 native graph pack.
pub struct GltfToIfc;
impl ArtifactComposer for GltfToIfc {
    type Snapshot = IfcSnapshot;
    const WRITES: Dialect = crate::io::IFC4X3_DIALECT;
    fn reads() -> &'static [Dialect] { &[GLTF_DIALECT] }
    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let [source] = sources else { return Err(composition_error("Exactly one glTF source is required")); };
        if source.dialect != GLTF_DIALECT { return Err(composition_error("Expected glTF 2.0 source dialect")); }
        let snapshot = match &source.payload {
            AnalyzeSource::Text(text) => {
                if text.len()>64*1024*1024 {return Err(composition_error("glTF JSON source exceeds byte bound"));}
                gltf_io::import::parse_gltf_document(text.as_bytes()).map_err(composition_error)?
            },
            AnalyzeSource::Binary(bytes) if bytes.starts_with(b"glTF") => decode_glb_checked(bytes).map_err(composition_error)?,
            AnalyzeSource::Binary(bytes) => GltfSnapshot::decode_pack(bytes).map_err(composition_error)?,
        };
        gltf_to_ifc_controlled(&snapshot, &mut |_| true).map_err(composition_error)
    }
}

fn decode_glb_checked(bytes: &[u8]) -> Result<GltfSnapshot, ExchangeError> {
    if bytes.len() < 20 || bytes.len() > 64 * 1024 * 1024 || &bytes[..4] != b"glTF" || u32::from_le_bytes(bytes[4..8].try_into().unwrap()) != 2 || u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize != bytes.len() {
        return Err(error("exchange.glb", "Invalid or oversized GLB 2.0 frame"));
    }
    let mut at = 12;
    let mut chunks = 0;
    let mut bin_size = 0;
    let mut bin_start = 0;
    while at < bytes.len() {
        if at + 8 > bytes.len() { return Err(error("exchange.glb", "Truncated GLB chunk")); }
        let size = u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()) as usize;
        let kind = u32::from_le_bytes(bytes[at+4..at+8].try_into().unwrap());
        if size % 4 != 0 || size > bytes.len() - at - 8 || (chunks == 0 && kind != 0x4e4f534a) || (chunks == 1 && kind != 0x004e4942) || (chunks > 1 && matches!(kind,0x4e4f534a|0x004e4942)) { return Err(error("exchange.glb", "Expected one JSON and one aligned embedded BIN chunk before extension chunks")); }
        if chunks==0 && bytes[at+8..at+8+size].iter().rfind(|&&byte|!matches!(byte,b' '|b'\t'|b'\n'|b'\r'))!=Some(&b'}') {return Err(error("exchange.glb","Invalid JSON object framing"));}
        if chunks == 1 {bin_size=size;bin_start=at+8;}
        at += 8 + size;
        chunks += 1;
    }
    if chunks < 2 { return Err(error("exchange.glb", "One embedded GLB buffer is required")); }
    let snapshot = gltf_io::import::decode_glb(bytes).map_err(|e| error("exchange.glb", e))?;
    if snapshot.document.buffers.len()!=1 || snapshot.document.buffers[0].uri.is_some() || snapshot.buffers.len()!=1 || snapshot.document.buffers[0].byte_length!=snapshot.buffers[0].len() || bin_size.saturating_sub(snapshot.buffers[0].len())>3 {
        return Err(error("exchange.glb", "Declared embedded buffer extent is unavailable"));
    }
    if bytes[bin_start+snapshot.buffers[0].len()..bin_start+bin_size].iter().any(|&byte|byte!=0) {return Err(error("exchange.glb","BIN padding must contain zero bytes"));}
    Ok(snapshot)
}

pub fn io_error(value: ExchangeError) -> IoError {
    let kind=if value.code=="exchange.cancelled" {semio_framework_value::ValueRefusalKind::Canceled} else {semio_framework_value::ValueRefusalKind::InvalidValue};
    IoError {cause:semio_framework_value::ValueError::new(kind,value.message.clone()).into(),diagnostics:vec![Diagnostic::error(value.code,TextSpan::default(),value.message)]}
}

/// 🔌 Original caller cancellation reaches all graph and geometry loops; target pack encoding uses its original encoder.
pub struct IfcIntoGltfNative;
impl Serializer<IfcSnapshot> for IfcIntoGltfNative {
    const INTO: Dialect=GLTF_DIALECT;
    const FIDELITY: IoFidelity=IoFidelity::Lossy;
    async fn serialize(source:&IfcSnapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload> {
        gltf_owned::export_payload(source,false,control)
    }
}

/// 🔌 Foreign native glTF packs use the original decoder; bounded natural GLB/JSON carriers are admitted explicitly.
pub struct GltfIntoIfcNative;
impl Deserializer<IfcSnapshot> for GltfIntoIfcNative {
    const FROM: Dialect=GLTF_DIALECT;
    const FIDELITY: IoFidelity=IoFidelity::Lossy;
    async fn sniff(payload:&IoPayload)->semio_os_kernel::io::io_schema::Confidence {
        let IoPayload::Binary(bytes)=payload else{return semio_os_kernel::io::io_schema::Confidence::None;};
        let Ok(length)=semio_os_kernel::semio_format::declared_envelope_prefix_len("stdio.gltf",semio_os_kernel::semio_format::Component::Pack,1)else{return semio_os_kernel::io::io_schema::Confidence::None;};
        if bytes.len()>=length&&(0..length).all(|i|semio_os_kernel::semio_format::declared_binary_prefix_byte("stdio.gltf",semio_os_kernel::semio_format::Component::Pack,1,i).ok().flatten()==Some(bytes[i])){semio_os_kernel::io::io_schema::Confidence::High}else{semio_os_kernel::io::io_schema::Confidence::None}
    }
    async fn deserialize(payload:&IoPayload,control:&mut IoRunControl<'_,'_>)->IoResult<IfcSnapshot> {
        gltf_owned::import_payload(payload,false,control)
    }
}

/// 💾 Natural GLB export for host binary delivery, independently of target-native artifact pack composition.
pub struct IfcIntoGlb;
impl Serializer<IfcSnapshot> for IfcIntoGlb {
    const INTO:Dialect=semio_os_kernel::io::io_schema::CARRIER_BINARY;
    const FIDELITY:IoFidelity=IoFidelity::Lossy;
    async fn serialize(source:&IfcSnapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload> {
        gltf_owned::export_payload(source,true,control)
    }
}

/// 📥️ Actual natural GLB carriers borrow their original JSON and BIN spans under the source recipient.
pub struct GlbIntoIfcNative;
impl Deserializer<IfcSnapshot> for GlbIntoIfcNative {
    const FROM:Dialect=semio_os_kernel::io::io_schema::CARRIER_BINARY;
    const FIDELITY:IoFidelity=IoFidelity::Lossy;
    async fn sniff(payload:&IoPayload)->semio_os_kernel::io::io_schema::Confidence{match payload{IoPayload::Binary(bytes)if bytes.len()>=12&&bytes.starts_with(b"glTF")&&u32::from_le_bytes(bytes[4..8].try_into().unwrap())==2&&u32::from_le_bytes(bytes[8..12].try_into().unwrap())as usize==bytes.len()=>semio_os_kernel::io::io_schema::Confidence::High,_=>semio_os_kernel::io::io_schema::Confidence::None}}
    async fn deserialize(payload:&IoPayload,control:&mut IoRunControl<'_,'_>)->IoResult<IfcSnapshot>{gltf_owned::import_payload(payload,true,control)}
}

/// 📡 Controlled host routes of the glTF, GLB and UTF-8 STEP carriers have explicit lossy fidelity and publish target-owned native artifact packs; the domain exchanges extend this list in their own crate.
pub fn base_io_entries()->Vec<IoEntry> {
    vec![original_pack::serializer::<IfcIntoGltfNative>(),original_pack::deserializer::<GltfIntoIfcNative>(),original_pack::serializer::<IfcIntoGlb>(),original_pack::deserializer::<GlbIntoIfcNative>(),original_pack::serializer::<crate::io::Ifc4x3IntoTxtNative>(),original_pack::deserializer::<crate::io::TxtIntoIfc4x3Native>()]
}

/// 📡 The memoized carrier routes.
pub fn io_entries()->&'static[IoEntry] {
    static ENTRIES:OnceLock<Vec<IoEntry>>=OnceLock::new();
    ENTRIES.get_or_init(base_io_entries).as_slice()
}

/// 📇 Native composition registry routes publish target-owned artifact packs.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_plugin::composer_entry_of::<IfcToGltf>(), semio_plugin::composer_entry_of::<GltfToIfc>()]).as_slice()
}

#[cfg(test)]
#[path = "🧪️tests/native_exchange.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/native_step_entry.rs"]
mod step_entry_tests;
#[cfg(test)]
#[path = "🧪️tests/native_step_import_entry.rs"]
mod step_import_entry_tests;
