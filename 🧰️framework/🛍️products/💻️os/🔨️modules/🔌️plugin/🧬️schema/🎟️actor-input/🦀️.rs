//! 🎟️ The one wire payload `reactor.admit-actor-input` admits into the guest's single admitted-input slot.
//! Schema: `🧬️schema/🔣️.json` declares the decoded form (an invalid case with a `stage` is refused later, by the guest event lift, not by the envelope decode); `🧫️fixtures/🔣️.json` pins it, with the exact pack bytes (`packHex`) each case encodes to. The wire is the repo's container-less pack value, read straight into the typed envelope with no intermediate JSON tree; a poll event stays a value (`{ tag, val }`, camelCase fields) that the guest lifts.
use crate::sqlite_wire::{SnapshotCloseInput, SnapshotGrant, SnapshotInput, SnapshotLimits};
use semio_framework_value::{DslValue, Number};

/// 📐️ The fixed result extents of one poll, carried with its event because `actor-context-budget` bounds only authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TurnShape {
    pub max_effects: u32,
    pub max_patch_bytes: u32,
    pub max_frames: u32,
}

/// 📦️ A document's snapshot pack and history log, exactly `codec.document-pair`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentPairWire {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

/// 🎟️ Every turn input of the actor world: its operation tag names the one export allowed to consume the slot.
pub enum ActorInputWire {
    Poll { events: Vec<DslValue>, shape: TurnShape },
    StartJob { source: Option<semio_framework::kernel::OriginalJobSource> },
    Restore { state: Vec<u8> },
    SqliteSchema { dialect: String },
    SqliteExport { input: SnapshotInput },
    SqliteImport { input: SnapshotInput },
    SqliteTakeFile { ticket: u64 },
    SqliteTakePayload { ticket: u64 },
    SqliteRetirement { copy_bytes: u64 },
    SqliteClose { input: SnapshotCloseInput },
    PackSchemaHash { artifact_schema: String },
    Genesis { artifact_schema: String, document_id: String },
    PrintMirror { artifact_schema: String, pair: DocumentPairWire },
    ApplyOps { artifact_schema: String, pair: DocumentPairWire, ops: Vec<u8> },
    ReplayEnvelopes { artifact_schema: String, pair: DocumentPairWire, envelopes: Vec<u8> },
}

/// 🚫️ An admitted actor input outside the pack grammar or its schema; the guest reports the typed refusal, never the text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActorInputRefusal;

type Read<T> = Result<T, ActorInputRefusal>;

fn closed<'a>(value: &'a DslValue, names: &[&str]) -> Read<&'a DslValue> {
    let DslValue::Object(entries) = value else { return Err(ActorInputRefusal) };
    if entries.len() != names.len() || entries.iter().any(|(key, _)| !names.contains(&key.as_str())) {
        return Err(ActorInputRefusal);
    }
    Ok(value)
}
fn field<'a>(value: &'a DslValue, name: &str) -> Read<&'a DslValue> {
    value.get(name).ok_or(ActorInputRefusal)
}
fn uint(value: &DslValue) -> Read<u64> {
    value.as_u64().ok_or(ActorInputRefusal)
}
fn word(value: &DslValue) -> Read<u32> {
    u32::try_from(uint(value)?).map_err(|_| ActorInputRefusal)
}
fn text(value: &DslValue) -> Read<String> {
    value.as_str().map(str::to_owned).ok_or(ActorInputRefusal)
}
fn bytes(value: &DslValue) -> Read<Vec<u8>> {
    if let DslValue::Bytes(bytes) = value {
        return Ok(bytes.clone());
    }
    value.as_array().ok_or(ActorInputRefusal)?.iter().map(|byte| u8::try_from(uint(byte)?).map_err(|_| ActorInputRefusal)).collect()
}
fn optional_bytes(value: &DslValue) -> Read<Option<Vec<u8>>> {
    if value.is_null() { Ok(None) } else { bytes(value).map(Some) }
}
fn pair(value: &DslValue) -> Read<DocumentPairWire> {
    let value = closed(value, &["pack", "spr"])?;
    Ok(DocumentPairWire { pack: bytes(field(value, "pack")?)?, spr: bytes(field(value, "spr")?)? })
}
fn limits(value: &DslValue) -> Read<SnapshotLimits> {
    let value = closed(value, &["max_file_bytes", "max_value_bytes", "max_schema_bytes", "max_rows", "max_columns", "max_tables", "max_pages", "max_allocation_bytes"])?;
    Ok(SnapshotLimits {
        max_file_bytes: uint(field(value, "max_file_bytes")?)?,
        max_value_bytes: uint(field(value, "max_value_bytes")?)?,
        max_allocation_bytes: uint(field(value, "max_allocation_bytes")?)?,
        max_schema_bytes: uint(field(value, "max_schema_bytes")?)?,
        max_rows: uint(field(value, "max_rows")?)?,
        max_columns: uint(field(value, "max_columns")?)?,
        max_tables: uint(field(value, "max_tables")?)?,
        max_pages: uint(field(value, "max_pages")?)?,
    })
}
fn grant(value: &DslValue) -> Read<SnapshotGrant> {
    let value = closed(value, &["maximum_items", "maximum_copy_bytes", "maximum_capacity_bytes", "maximum_release_bytes", "maximum_depth"])?;
    Ok(SnapshotGrant {
        maximum_items: uint(field(value, "maximum_items")?)?,
        maximum_copy_bytes: uint(field(value, "maximum_copy_bytes")?)?,
        maximum_capacity_bytes: uint(field(value, "maximum_capacity_bytes")?)?,
        maximum_release_bytes: uint(field(value, "maximum_release_bytes")?)?,
        maximum_depth: uint(field(value, "maximum_depth")?)?,
    })
}
fn snapshot(value: &DslValue) -> Read<SnapshotInput> {
    let value = closed(value, &["dialect", "encoding", "payload", "limits", "native"])?;
    let encoding = text(field(value, "encoding")?)?;
    if !matches!(encoding.as_str(), "binary" | "text" | "") {
        return Err(ActorInputRefusal);
    }
    Ok(SnapshotInput { dialect: text(field(value, "dialect")?)?, encoding, payload: bytes(field(value, "payload")?)?, limits: limits(field(value, "limits")?)?, native: grant(field(value, "native")?)? })
}
fn job_source(value: &DslValue) -> Read<Option<semio_framework::kernel::OriginalJobSource>> {
    if value.is_null() {
        return Ok(None);
    }
    let value = closed(value, &["job", "kind", "input", "checkpoint"])?;
    Ok(Some(semio_framework::kernel::OriginalJobSource { job: uint(field(value, "job")?)?, kind: text(field(value, "kind")?)?, input: optional_bytes(field(value, "input")?)?, checkpoint: optional_bytes(field(value, "checkpoint")?)? }))
}

fn number(value: u64) -> DslValue {
    DslValue::Number(Number::UInt(value))
}
fn octets(value: &[u8]) -> DslValue {
    DslValue::Array(value.iter().map(|byte| number(u64::from(*byte))).collect())
}
fn object(mut entries: Vec<(&str, DslValue)>) -> DslValue {
    entries.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    DslValue::Object(entries.into_iter().map(|(key, value)| (key.to_owned(), value)).collect())
}
fn pair_value(pair: &DocumentPairWire) -> DslValue {
    object(vec![("pack", octets(&pair.pack)), ("spr", octets(&pair.spr))])
}
fn snapshot_value(input: &SnapshotInput) -> DslValue {
    let limits = &input.limits;
    let native = &input.native;
    object(vec![
        ("dialect", DslValue::String(input.dialect.clone())),
        ("encoding", DslValue::String(input.encoding.clone())),
        ("payload", octets(&input.payload)),
        ("limits", object(vec![("max_file_bytes", number(limits.max_file_bytes)), ("max_value_bytes", number(limits.max_value_bytes)), ("max_schema_bytes", number(limits.max_schema_bytes)), ("max_rows", number(limits.max_rows)), ("max_columns", number(limits.max_columns)), ("max_tables", number(limits.max_tables)), ("max_pages", number(limits.max_pages)), ("max_allocation_bytes", number(limits.max_allocation_bytes))])),
        ("native", grant_value(native)),
    ])
}
fn grant_value(native: &SnapshotGrant) -> DslValue {
    object(vec![("maximum_items", number(native.maximum_items)), ("maximum_copy_bytes", number(native.maximum_copy_bytes)), ("maximum_capacity_bytes", number(native.maximum_capacity_bytes)), ("maximum_release_bytes", number(native.maximum_release_bytes)), ("maximum_depth", number(native.maximum_depth))])
}
fn optional_octets(value: &Option<Vec<u8>>) -> DslValue {
    value.as_deref().map_or(DslValue::Null, octets)
}
fn schema_text(value: &str) -> DslValue {
    DslValue::String(value.to_owned())
}

impl ActorInputWire {
    /// 🔖️ The kebab-case operation tag this wire carries, as the schema declares it.
    pub fn operation(&self) -> &'static str {
        match self {
            Self::Poll { .. } => "poll",
            Self::StartJob { .. } => "start-job",
            Self::Restore { .. } => "restore",
            Self::SqliteSchema { .. } => "sqlite-schema",
            Self::SqliteExport { .. } => "sqlite-export",
            Self::SqliteImport { .. } => "sqlite-import",
            Self::SqliteTakeFile { .. } => "sqlite-take-file",
            Self::SqliteTakePayload { .. } => "sqlite-take-payload",
            Self::SqliteRetirement { .. } => "sqlite-retirement",
            Self::SqliteClose { .. } => "sqlite-close",
            Self::PackSchemaHash { .. } => "pack-schema-hash",
            Self::Genesis { .. } => "genesis",
            Self::PrintMirror { .. } => "print-mirror",
            Self::ApplyOps { .. } => "apply-ops",
            Self::ReplayEnvelopes { .. } => "replay-envelopes",
        }
    }

    /// 🌱️ The decoded value form the schema declares.
    pub fn to_value(&self) -> DslValue {
        let tag = ("operation", schema_text(self.operation()));
        match self {
            Self::Poll { events, shape } => object(vec![tag, ("events", DslValue::Array(events.clone())), ("shape", object(vec![("maxEffects", number(u64::from(shape.max_effects))), ("maxPatchBytes", number(u64::from(shape.max_patch_bytes))), ("maxFrames", number(u64::from(shape.max_frames)))]))]),
            Self::StartJob { source } => object(vec![tag, ("source", source.as_ref().map_or(DslValue::Null, |source| object(vec![("job", number(source.job)), ("kind", schema_text(&source.kind)), ("input", optional_octets(&source.input)), ("checkpoint", optional_octets(&source.checkpoint))])))]),
            Self::Restore { state } => object(vec![tag, ("state", octets(state))]),
            Self::SqliteSchema { dialect } => object(vec![tag, ("dialect", schema_text(dialect))]),
            Self::SqliteExport { input } | Self::SqliteImport { input } => object(vec![tag, ("input", snapshot_value(input))]),
            Self::SqliteTakeFile { ticket } | Self::SqliteTakePayload { ticket } => object(vec![tag, ("ticket", number(*ticket))]),
            Self::SqliteRetirement { copy_bytes } => object(vec![tag, ("copyBytes", number(*copy_bytes))]),
            Self::SqliteClose { input } => object(vec![tag, ("input", object(vec![("ticket", number(input.ticket)), ("native", grant_value(&input.native))]))]),
            Self::PackSchemaHash { artifact_schema } => object(vec![tag, ("artifactSchema", schema_text(artifact_schema))]),
            Self::Genesis { artifact_schema, document_id } => object(vec![tag, ("artifactSchema", schema_text(artifact_schema)), ("documentId", schema_text(document_id))]),
            Self::PrintMirror { artifact_schema, pair } => object(vec![tag, ("artifactSchema", schema_text(artifact_schema)), ("pair", pair_value(pair))]),
            Self::ApplyOps { artifact_schema, pair, ops } => object(vec![tag, ("artifactSchema", schema_text(artifact_schema)), ("pair", pair_value(pair)), ("ops", octets(ops))]),
            Self::ReplayEnvelopes { artifact_schema, pair, envelopes } => object(vec![tag, ("artifactSchema", schema_text(artifact_schema)), ("pair", pair_value(pair)), ("envelopes", octets(envelopes))]),
        }
    }

    /// 📥️ Reads the decoded value into the typed envelope; any key or shape outside the schema is refused.
    pub fn from_value(value: &DslValue) -> Read<Self> {
        let operation = text(field(value, "operation")?)?;
        let names: &[&str] = match operation.as_str() {
            "poll" => &["operation", "events", "shape"],
            "start-job" => &["operation", "source"],
            "restore" => &["operation", "state"],
            "sqlite-schema" => &["operation", "dialect"],
            "sqlite-export" | "sqlite-import" | "sqlite-close" => &["operation", "input"],
            "sqlite-take-file" | "sqlite-take-payload" => &["operation", "ticket"],
            "sqlite-retirement" => &["operation", "copyBytes"],
            "pack-schema-hash" => &["operation", "artifactSchema"],
            "genesis" => &["operation", "artifactSchema", "documentId"],
            "print-mirror" => &["operation", "artifactSchema", "pair"],
            "apply-ops" => &["operation", "artifactSchema", "pair", "ops"],
            "replay-envelopes" => &["operation", "artifactSchema", "pair", "envelopes"],
            _ => return Err(ActorInputRefusal),
        };
        let value = closed(value, names)?;
        let schema = || text(field(value, "artifactSchema")?);
        Ok(match operation.as_str() {
            "poll" => {
                let shape = closed(field(value, "shape")?, &["maxEffects", "maxPatchBytes", "maxFrames"])?;
                Self::Poll { events: field(value, "events")?.as_array().ok_or(ActorInputRefusal)?.to_vec(), shape: TurnShape { max_effects: word(field(shape, "maxEffects")?)?, max_patch_bytes: word(field(shape, "maxPatchBytes")?)?, max_frames: word(field(shape, "maxFrames")?)? } }
            }
            "start-job" => Self::StartJob { source: job_source(field(value, "source")?)? },
            "restore" => Self::Restore { state: bytes(field(value, "state")?)? },
            "sqlite-schema" => Self::SqliteSchema { dialect: text(field(value, "dialect")?)? },
            "sqlite-export" => Self::SqliteExport { input: snapshot(field(value, "input")?)? },
            "sqlite-import" => Self::SqliteImport { input: snapshot(field(value, "input")?)? },
            "sqlite-take-file" => Self::SqliteTakeFile { ticket: uint(field(value, "ticket")?)? },
            "sqlite-take-payload" => Self::SqliteTakePayload { ticket: uint(field(value, "ticket")?)? },
            "sqlite-retirement" => Self::SqliteRetirement { copy_bytes: uint(field(value, "copyBytes")?)? },
            "sqlite-close" => {
                let input = closed(field(value, "input")?, &["ticket", "native"])?;
                Self::SqliteClose { input: SnapshotCloseInput { ticket: uint(field(input, "ticket")?)?, native: grant(field(input, "native")?)? } }
            }
            "pack-schema-hash" => Self::PackSchemaHash { artifact_schema: schema()? },
            "genesis" => Self::Genesis { artifact_schema: schema()?, document_id: text(field(value, "documentId")?)? },
            "print-mirror" => Self::PrintMirror { artifact_schema: schema()?, pair: pair(field(value, "pair")?)? },
            "apply-ops" => Self::ApplyOps { artifact_schema: schema()?, pair: pair(field(value, "pair")?)?, ops: bytes(field(value, "ops")?)? },
            _ => Self::ReplayEnvelopes { artifact_schema: schema()?, pair: pair(field(value, "pair")?)?, envelopes: bytes(field(value, "envelopes")?)? },
        })
    }

    /// 📤️ Encodes the wire the host admits in the repo's container-less binary pack value (`store::pack_rt::encode_wire_value`).
    pub fn encode(&self) -> Vec<u8> {
        semio_os_kernel::pack_rt::encode_wire_value(&self.to_value())
    }

    /// 📥️ Decodes an admitted wire straight from pack bytes into the typed envelope.
    pub fn decode(bytes: &[u8]) -> Read<Self> {
        Self::from_value(&semio_os_kernel::pack_rt::decode_wire_value(bytes).map_err(|_| ActorInputRefusal)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn every_fixture_wire_decodes_from_its_pinned_pack_bytes_and_encodes_back_to_them() {
        let corpus: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        for case in corpus["valid"].as_array().unwrap() {
            let pinned = case["packHex"].as_str().unwrap();
            let bytes: Vec<u8> = (0..pinned.len()).step_by(2).map(|at| u8::from_str_radix(&pinned[at..at + 2], 16).unwrap()).collect();
            let wire = ActorInputWire::decode(&bytes).unwrap();
            assert_eq!(wire.operation(), case["operation"].as_str().unwrap());
            assert!(wire.to_value() == semio_framework_value::DslValue::from(&case["wire"]), "{}", case["operation"]);
            assert_eq!(hex(&wire.encode()), pinned, "{}", case["operation"]);
        }
        for case in corpus["invalid"].as_array().unwrap() {
            if case.get("stage").is_some() {
                continue;
            }
            let value = semio_framework_value::DslValue::from(&case["wire"]);
            assert!(ActorInputWire::decode(&semio_os_kernel::pack_rt::encode_wire_value(&value)).is_err(), "{}", case["reason"]);
        }
    }
}
