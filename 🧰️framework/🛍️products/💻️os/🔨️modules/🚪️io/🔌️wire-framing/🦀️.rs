//! 🔌️ Wire framing: the bounded wire framing of compose, decode and list requests.
//!
//! Holds the `🔖️Wire` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️Wire






/// 🔒️ Process-local intern table: `ArtifactDialect` → a genuine `&'static Dialect`. Cross-plugin
/// compose results can name a dialect the RECEIVING plugin never registered a `&'static Dialect`
/// constant for (it belongs to whichever plugin actually produced it) — `Dialect`'s `&'static str`
/// fields can't be manufactured from an arbitrary runtime `String` without leaking memory, so this
/// interns each DISTINCT coordinate exactly once (a bounded, one-time leak per never-before-seen
/// dialect string for the lifetime of the process, the same tradeoff any string-interning table
/// makes) and reuses it for every subsequent occurrence of that coordinate.
///
/// Deliberately validates nothing against a subset vocabulary (ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES considered and rejected adding a check
/// here): this runs on the RECEIVING side of cross-plugin compose, where the local process by
/// design has no way to know the producing plugin's subset vocabulary — hard-failing on an
/// "unrecognized" subset here would break legitimate cross-plugin io for any dialect the receiver
/// simply hasn't loaded a manifest for. Authority already lives at the right boundaries instead:
/// `wire_artifact_compose` rejects source dialects the local `ComposerEntry.reads` doesn't declare,
/// `run_subset_validation` reports (never fails on) a missing validator, and static "does this
/// standard's on-disk subset vocabulary match its declared manifest" enforcement is
/// `policyStandardSubsetVocabularyBreaches`'s job in script.ts, not runtime intern's.
/// 🚫️ A typed wire boundary rejection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoWireError {
    Decode { operation: &'static str, message: String },
    Encode { operation: &'static str, message: String },
    Limit { operation: &'static str, detail: String },
    Registry(IoRegistryUnavailable),
    Resolve(String),
    Subset(SubsetValidationError),
    InternUnavailable,
}

impl std::fmt::Display for IoWireError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode { operation, message } => write!(formatter, "{operation} decode failed: {message}"),
            Self::Encode { operation, message } => write!(formatter, "{operation} encode failed: {message}"),
            Self::Limit { operation, detail } => write!(formatter, "{operation} exceeds wire limit: {detail}"),
            Self::Registry(error) => write!(formatter, "registry unavailable: {}", error.registry),
            Self::Resolve(message) => formatter.write_str(message),
            Self::Subset(error) => write!(formatter, "subset validation failed: {error:?}"),
            Self::InternUnavailable => formatter.write_str("dialect intern registry unavailable"),
        }
    }
}

impl std::error::Error for IoWireError {}

pub const MAX_IO_WIRE_BYTES: usize = 4 * 1024 * 1024;
const MAX_IO_WIRE_SOURCES: usize = 256;
pub const MAX_IO_WIRE_DIALECT_COMPONENT_BYTES: usize = 192;
const MAX_IO_WIRE_INTERNED_DIALECTS: usize = 512;

fn ensure_wire_bytes(operation: &'static str, bytes: &[u8]) -> Result<(), IoWireError> {
    if bytes.len() > MAX_IO_WIRE_BYTES {
        return Err(IoWireError::Limit { operation, detail: format!("{} bytes exceeds {MAX_IO_WIRE_BYTES}", bytes.len()) });
    }
    Ok(())
}

async fn validate_wire_dialect(operation: &'static str, dialect: &ArtifactDialect) -> Result<(), IoWireError> {
    for (name, value) in [("artifact_kind", &dialect.artifact_kind), ("standard", &dialect.standard), ("subset", &dialect.subset)] {
        if value.is_empty() || value.len() > MAX_IO_WIRE_DIALECT_COMPONENT_BYTES || value.bytes().any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control() || matches!(byte, b'@' | b'/' | b'!')) {
            return Err(IoWireError::Limit { operation, detail: format!("invalid bounded dialect {name}") });
        }
    }
    Ok(())
}

async fn validate_wire_payload(operation: &'static str, payload: &IoPayload) -> Result<(), IoWireError> {
    let bytes = match payload {
        IoPayload::Text(text) => text.len(),
        IoPayload::Binary(bytes) => bytes.len(),
    };
    if bytes > MAX_IO_WIRE_BYTES {
        return Err(IoWireError::Limit { operation, detail: format!("payload {bytes} bytes exceeds {MAX_IO_WIRE_BYTES}") });
    }
    Ok(())
}

async fn validate_wire_key(key: &IoKey) -> Result<(), IoWireError> {
    for value in [&key.artifact_kind, &key.standard, &key.subset, &key.format_kind, &key.format_standard, &key.format_subset] {
        if value.is_empty() || value.len() > MAX_IO_WIRE_DIALECT_COMPONENT_BYTES || value.bytes().any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control()) {
            return Err(IoWireError::Limit { operation: "io-key", detail: "key component is empty or exceeds the bounded wire grammar".to_string() });
        }
    }
    Ok(())
}

async fn encode_wire_json<T: ToValue>(operation: &'static str, value: &T) -> Result<Vec<u8>, IoWireError> {
    let bytes = semio_framework_pack_json::to_json_string(value).into_bytes();
    ensure_wire_bytes(operation, &bytes)?;
    Ok(bytes)
}

/// 🌉️ `encode_wire_json`'s inverse: decodes bounded wire bytes (already UTF-8 JSON text, per the
/// `T: ToValue` half above) via `pack::json`/`FromValue` instead of `serde_json`.
async fn decode_wire_json<T: FromValue>(operation: &'static str, bytes: &[u8]) -> Result<T, IoWireError> {
    let text = std::str::from_utf8(bytes).map_err(|error| IoWireError::Decode { operation, message: error.to_string() })?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoWireError::Decode { operation, message: error.to_string() })
}

async fn intern_dialect(dialect: &ArtifactDialect) -> Result<Dialect, IoWireError> {
    validate_wire_dialect("dialect", dialect).await?;
    static INTERNED: std::sync::OnceLock<RwLock<HashMap<ArtifactDialect, Dialect>>> = std::sync::OnceLock::new();
    let table = INTERNED.get_or_init(|| RwLock::new(HashMap::new()));
    if let Some(found) = table.read().map_err(|_| IoWireError::InternUnavailable)?.get(dialect) {
        return Ok(*found);
    }
    let mut write = table.write().map_err(|_| IoWireError::InternUnavailable)?;
    if let Some(found) = write.get(dialect) {
        return Ok(*found);
    }
    if write.len() >= MAX_IO_WIRE_INTERNED_DIALECTS {
        return Err(IoWireError::Limit { operation: "dialect", detail: format!("intern table reached {MAX_IO_WIRE_INTERNED_DIALECTS} entries") });
    }
    let leaked = Dialect { artifact_kind: Box::leak(dialect.artifact_kind.clone().into_boxed_str()), standard: StandardId(Box::leak(dialect.standard.clone().into_boxed_str())), subset: SubsetId(Box::leak(dialect.subset.clone().into_boxed_str())) };
    write.insert(dialect.clone(), leaked);
    Ok(leaked)
}

/// 🌉️ Decodes a wire `WireComposedArtifact` (JSON bytes) into a native `ComposedArtifact`,
/// interning its dialect via `intern_dialect`. The receiving-side half of `wire_artifact_compose`
/// — used by a guest's `io_dispatch` fallback hook once `host.io-compose` returns.
pub async fn wire_decode_composed_artifact(bytes: &[u8]) -> Result<ComposedArtifact, IoWireError> {
    ensure_wire_bytes("composed-artifact", bytes)?;
    let wire: WireComposedArtifact = decode_wire_json("composed-artifact", bytes).await?;
    validate_wire_dialect("composed-artifact", &wire.dialect).await?;
    validate_wire_payload("composed-artifact", &wire.payload).await?;
    if wire.diagnostics.len() > MAX_IO_WIRE_SOURCES {
        return Err(IoWireError::Limit { operation: "composed-artifact", detail: "too many diagnostics".to_string() });
    }
    Ok(ComposedArtifact { dialect: intern_dialect(&wire.dialect).await?, payload: wire.payload, diagnostics: wire.diagnostics, confidence: wire.confidence })
}

/// 🌉️ Encodes this process's own composer roster (`list_composer_entries`) as JSON bytes — the
/// body of the WIT `list-artifact-dialects` guest export (see D3, ticket 26/08/10/
/// ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION). JSON (not `pack_rt::encode_wire_value`)
/// is a deliberate simplification for this first cut: the WIT signature is an opaque `list<u8>`
/// either way, so swapping the wire encoding later needs no ABI change, and this module has no
/// existing dependency on `store`/`dsl`'s pack machinery worth introducing just for this.
pub async fn wire_list_composer_entries() -> Result<Vec<u8>, IoWireError> {
    let entries = list_composer_entries().await.map_err(IoWireError::Registry)?;
    encode_wire_json("composer-entry-list", &entries).await
}

/// 🌉️ Decodes a wire `(IoKey, Vec<WireComposeSource>)` request and composes it against THIS
/// process's own local registry only — never the fallback hook. A guest receiving an incoming
/// `artifact-compose` call is, by construction, the plugin the host router already decided owns
/// the key; falling through again here would be a pointless extra hop at best and a reentrancy
/// risk at worst (see the host router's own one-hop guard). The body of the WIT
/// `artifact-compose` guest export. Errors are flattened to a message string, matching how every
/// other fallible call on this ABI surfaces errors (a `Fault`, not structured data) — see
/// `migrate-artifact`'s `plugin-error` for the existing precedent.
pub async fn wire_artifact_compose(key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, IoWireError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    ensure_wire_bytes("io-key", key_bytes)?;
    ensure_wire_bytes("compose-source", sources_bytes)?;
    let key: IoKey = decode_wire_json("io-key", key_bytes).await?;
    validate_wire_key(&key).await?;
    let wire_sources: Vec<WireComposeSource> = decode_wire_json("compose-source", sources_bytes).await?;
    if wire_sources.len() > MAX_IO_WIRE_SOURCES {
        return Err(IoWireError::Limit { operation: "compose-source", detail: format!("{} sources exceeds {MAX_IO_WIRE_SOURCES}", wire_sources.len()) });
    }
    let entry = resolve(&key).await.map_err(|error| error.unavailable.map_or_else(|| IoWireError::Resolve(error.message), IoWireError::Registry))?;
    let mut sources = Vec::with_capacity(wire_sources.len());
    for wire in wire_sources {
        validate_wire_dialect("compose-source", &wire.dialect).await?;
        validate_wire_payload("compose-source", &wire.payload).await?;
        let dialect = entry.reads.iter().copied().find(|&d| ArtifactDialect::from(d) == wire.dialect).ok_or_else(|| IoWireError::Resolve(format!("composer for {} does not read dialect {}", key.artifact_kind, wire.dialect.to_coordinate())))?;
        sources.push(ErasedComposeSource { dialect, payload: wire.payload });
    }
    match ::semio_framework_async::poll::resolve_ready((entry.compose)(&sources)) {
        Ok(mut composed) => {
            run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics).await.map_err(IoWireError::Subset)?;
            encode_wire_json("composed-artifact", &WireComposedArtifact::from(composed)).await
        }
        Err(error) => Err(IoWireError::Resolve(error.message)),
    }
}
//#endregion 🔖️Wire
