//! 🔺️ Ifc2x3Diff — real id-keyed instance diff over `Ifc2x3Snapshot.document.instances`
//! (`Part21Instance` is already keyed by a stable `u64` id, so unlike an index-keyed collection
//! this diff needs no position-transport algebra: `removed_instances`/`upserted_instances` are a
//! plain id-keyed set/map merge). Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES:
//! `4`'s `IfcDiff` is a `snapshot: Option<IfcSnapshot>` full-replace stub with no
//! `impl DiffAlgebra`; this standard's own diff is genuinely field-sparse instead.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot};
use crate::artifacts::step::engine::part21::{Part21Decimal, Part21Header, Part21Instance, Part21Value};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated) — reached via the still-public `os_spr::command` path
// instead, same as `txt`'s own `🔺️diff/🦀️component.rs`.
use protocol::os_spr::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ifc.2x3`. `header` is a whole-record replace (it's a 3-field header, not
/// worth a sub-algebra); `removed_instances`/`upserted_instances` are the id-keyed instance delta.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3.diff")]
pub struct Ifc2x3Diff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Part21Header>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed_instances: Vec<u64>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upserted_instances: Vec<Part21Instance>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edm_preamble: Option<Option<Ifc2x3EdmPreamble>>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_order: Option<Vec<u64>>,
}

async fn validate_ifc2x3_diff(diff: &Ifc2x3Diff, base: &Ifc2x3Snapshot) -> MutationApplyResult<()> {
    let mut base_ids = BTreeSet::new();
    for instance in &base.document.instances {
        if !base_ids.insert(instance.id) {
            return Err(MutationApplyError::new("duplicate-base-target", "base instance ids must be unique").at(["instances", &instance.id.to_string()]));
        }
    }
    let mut removed = BTreeSet::new();
    for &id in &diff.removed_instances {
        if !base_ids.contains(&id) || !removed.insert(id) {
            return Err(MutationApplyError::new("invalid-remove-target", "instance removal target must exist exactly once").at(["instances", &id.to_string()]));
        }
    }
    let mut upserted = BTreeSet::new();
    for instance in &diff.upserted_instances {
        if removed.contains(&instance.id) || !upserted.insert(instance.id) {
            return Err(MutationApplyError::new("invalid-upsert-target", "instance upsert target must be unique and not removed").at(["instances", &instance.id.to_string()]));
        }
    }
    let mut final_ids = base_ids;
    for id in removed {
        final_ids.remove(&id);
    }
    final_ids.extend(upserted);
    if let Some(order) = &diff.instance_order {
        let mut ordered = BTreeSet::new();
        for &id in order {
            if !final_ids.contains(&id) || !ordered.insert(id) {
                return Err(MutationApplyError::new("invalid-instance-order", "instance order must contain each final id exactly once").at(["instanceOrder", &id.to_string()]));
            }
        }
        if ordered != final_ids {
            let missing = final_ids.difference(&ordered).next().copied().unwrap_or_default();
            return Err(MutationApplyError::new("invalid-instance-order", "instance order must contain each final id exactly once").at(["instanceOrder", &missing.to_string()]));
        }
    }
    Ok(())
}

async fn apply_ifc2x3_diff_unchecked(diff: &Ifc2x3Diff, base: &Ifc2x3Snapshot) -> Ifc2x3Snapshot {
    let mut document = base.document.clone();
    if let Some(header) = &diff.header {
        document.header = header.clone();
    }
    let removed: HashSet<u64> = diff.removed_instances.iter().copied().collect();
    document.instances.retain(|i| !removed.contains(&i.id));
    let mut positions = document.instances.iter().enumerate().map(|(position, instance)| (instance.id, position)).collect::<std::collections::HashMap<_, _>>();
    for instance in &diff.upserted_instances {
        if let Some(position) = positions.get(&instance.id).copied() {
            document.instances[position] = instance.clone();
        } else {
            positions.insert(instance.id, document.instances.len());
            document.instances.push(instance.clone());
        }
    }
    if let Some(order) = &diff.instance_order {
        let mut by_id = document.instances.drain(..).map(|instance| (instance.id, instance)).collect::<std::collections::HashMap<_, _>>();
        document.instances.reserve(by_id.len());
        for id in order {
            if let Some(instance) = by_id.remove(id) {
                document.instances.push(instance);
            }
        }
    }
    Ifc2x3Snapshot { schema: diff.schema.clone().unwrap_or_else(|| base.schema.clone()), document, edm_preamble: diff.edm_preamble.clone().unwrap_or_else(|| base.edm_preamble.clone()) }
}

impl MutationDiff<Ifc2x3Snapshot> for Ifc2x3Diff {
    async fn apply(&self, base: &Ifc2x3Snapshot) -> MutationApplyResult<Ifc2x3Snapshot> {
        validate_ifc2x3_diff(self, base)?;
        Ok(apply_ifc2x3_diff_unchecked(self, base))
    }

    /// ➕️ Structural, base-free (id-keyed collections need no position transport, unlike an
    /// index-keyed one): `other`'s removal of an id cancels any pending upsert of that id in
    /// `self` (and vice versa — a later upsert of a formerly-removed id un-removes it).
    async fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.header.is_some() {
            self.header = other.header;
        }
        let removed_ids = other.removed_instances.iter().copied().collect::<HashSet<_>>();
        self.upserted_instances.retain(|instance| !removed_ids.contains(&instance.id));
        let mut known_removed = self.removed_instances.iter().copied().collect::<HashSet<_>>();
        for id in other.removed_instances {
            if known_removed.insert(id) {
                self.removed_instances.push(id);
            }
        }
        let upserted_ids = other.upserted_instances.iter().map(|instance| instance.id).collect::<HashSet<_>>();
        self.removed_instances.retain(|id| !upserted_ids.contains(id));
        let mut positions = self.upserted_instances.iter().enumerate().map(|(position, instance)| (instance.id, position)).collect::<std::collections::HashMap<_, _>>();
        for inst in other.upserted_instances {
            if let Some(position) = positions.get(&inst.id).copied() {
                self.upserted_instances[position] = inst;
            } else {
                positions.insert(inst.id, self.upserted_instances.len());
                self.upserted_instances.push(inst);
            }
        }
        if other.edm_preamble.is_some() {
            self.edm_preamble = other.edm_preamble;
        }
        if other.instance_order.is_some() {
            self.instance_order = other.instance_order;
        }
    }
}

impl DiffAlgebra<Ifc2x3Snapshot> for Ifc2x3Diff {
    /// 🔁️ Same `apply`+`between` composition proof `txt::TxtDiff::inverse` uses: `next =
    /// self.apply(base)`, so `between(next, base)` is by definition the diff that restores `base`.
    async fn inverse(&self, base: &Ifc2x3Snapshot) -> Self {
        let next = apply_ifc2x3_diff_unchecked(self, base);
        Self::between(&next, base)
    }

    async fn between(base: &Ifc2x3Snapshot, other: &Ifc2x3Snapshot) -> Self {
        let schema = if base.schema != other.schema { Some(other.schema.clone()) } else { None };
        let header = if base.document.header != other.document.header { Some(other.document.header.clone()) } else { None };
        let base_order = base.document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>();
        let other_order = other.document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>();
        let instance_order = (base_order != other_order).then(|| other_order.clone());
        let edm_preamble = (base.edm_preamble != other.edm_preamble).then(|| other.edm_preamble.clone());
        if base.document.instances.is_empty() {
            let mut upserted_instances = other.document.instances.clone();
            upserted_instances.sort_by_key(|instance| instance.id);
            return Ifc2x3Diff { schema, header, removed_instances: Vec::new(), upserted_instances, edm_preamble, instance_order };
        }
        if other.document.instances.is_empty() {
            let mut removed_instances = base_order;
            removed_instances.sort_unstable();
            return Ifc2x3Diff { schema, header, removed_instances, upserted_instances: Vec::new(), edm_preamble, instance_order };
        }
        let base_by_id: std::collections::HashMap<u64, &Part21Instance> = base.document.instances.iter().map(|i| (i.id, i)).collect();
        let other_by_id: std::collections::HashMap<u64, &Part21Instance> = other.document.instances.iter().map(|i| (i.id, i)).collect();
        let mut removed_instances: Vec<u64> = base_by_id.keys().filter(|id| !other_by_id.contains_key(id)).copied().collect();
        removed_instances.sort_unstable();
        let mut upserted_instances: Vec<Part21Instance> = other.document.instances.iter().filter(|i| base_by_id.get(&i.id).map(|b| *b != *i).unwrap_or(true)).cloned().collect();
        upserted_instances.sort_by_key(|i| i.id);
        Ifc2x3Diff { schema, header, removed_instances, upserted_instances, edm_preamble, instance_order }
    }

    async fn is_empty(&self) -> bool {
        self.schema.is_none() && self.header.is_none() && self.removed_instances.is_empty() && self.upserted_instances.is_empty() && self.edm_preamble.is_none() && self.instance_order.is_none()
    }
}

/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation.
pub async fn diff_set_snapshot(base: &Ifc2x3Snapshot, snapshot: &Ifc2x3Snapshot) -> Ifc2x3Diff {
    Ifc2x3Diff::between(base, snapshot)
}
pub async fn diff_upsert_instance(instance: &Part21Instance) -> Ifc2x3Diff {
    Ifc2x3Diff { upserted_instances: vec![instance.clone()], ..Default::default() }
}
pub async fn diff_remove_instance(id: u64) -> Ifc2x3Diff {
    Ifc2x3Diff { removed_instances: vec![id], ..Default::default() }
}
pub async fn diff_set_header(header: &Part21Header) -> Ifc2x3Diff {
    Ifc2x3Diff { header: Some(header.clone()), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real hand-rolled
/// `protocol::DiffCodec` for `Ifc2x3Diff` — this standard had NO `DiffCodec` impl at all before this
/// wave (confirmed by W0/F6's own census, the sole remaining `dsl-migration/diff-completeness`
/// breach across all 32 stdio standards). `Part21Value` is a genuine data-carrying enum (`Ref`/
/// `Str`/`Enum`/`Int`/`Real`/`List`/`Typed`, all with fields) reachable from `header`/
/// `upserted_instances` directly, so `#[derive(dsl::DslDiff)]` cannot be used here either (identical
/// `DslField`-unsatisfied root cause `4`'s own `IfcDiff`/`IfcValue` doc comment documents). Same
/// grammar style `4`'s own hand-rolled `IfcDiff`/`IfcValue` codec uses (bracket-depth-aware split,
/// hex for strings, single-uppercase-letter tag prefix for the data-carrying enum) — own local copy
/// per this dialect's per-file convention, `pub(crate)` so the mutations sibling can reuse rather
/// than duplicating a second time (same intra-artifact-reuse split `4`'s own files use).
//#region 🔖️TextPrimitives
pub(crate) async fn hex_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    hex_encode_into(bytes, &mut encoded);
    encoded
}
async fn hex_encode_into(bytes: &[u8], encoded: &mut String) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
}
pub(crate) async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) async fn enc_edm_preamble(preamble: &Ifc2x3EdmPreamble) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
        enc_str(&preamble.producer),
        enc_str(&preamble.module),
        enc_str(&preamble.creation_date),
        enc_str(&preamble.host),
        enc_str(&preamble.database),
        enc_str(&preamble.database_version),
        enc_str(&preamble.database_creation_date),
        enc_str(&preamble.schema),
        enc_str(&preamble.model),
        enc_str(&preamble.model_creation_date),
        enc_str(&preamble.header_model),
        enc_str(&preamble.header_model_creation_date),
        enc_str(&preamble.user),
        enc_str(&preamble.group),
        enc_str(&preamble.license),
        enc_str(&preamble.options)
    )
}
pub(crate) async fn dec_edm_preamble(s: &str) -> Result<Ifc2x3EdmPreamble, String> {
    let fields = split_top_level(strip_brackets(s)?, ',');
    let [producer, module, creation_date, host, database, database_version, database_creation_date, schema, model, model_creation_date, header_model, header_model_creation_date, user, group, license, options] = fields.as_slice() else {
        return Err(format!("EDM preamble: expected 16 fields, got {}", fields.len()));
    };
    Ok(Ifc2x3EdmPreamble {
        producer: dec_str(producer)?,
        module: dec_str(module)?,
        creation_date: dec_str(creation_date)?,
        host: dec_str(host)?,
        database: dec_str(database)?,
        database_version: dec_str(database_version)?,
        database_creation_date: dec_str(database_creation_date)?,
        schema: dec_str(schema)?,
        model: dec_str(model)?,
        model_creation_date: dec_str(model_creation_date)?,
        header_model: dec_str(header_model)?,
        header_model_creation_date: dec_str(header_model_creation_date)?,
        user: dec_str(user)?,
        group: dec_str(group)?,
        license: dec_str(license)?,
        options: dec_str(options)?,
    })
}
pub(crate) async fn enc_optional_edm_preamble(preamble: &Option<Ifc2x3EdmPreamble>) -> String {
    preamble.as_ref().map(|value| format!("[1,{}]", enc_edm_preamble(value))).unwrap_or_else(|| "[0]".into())
}
pub(crate) async fn dec_optional_edm_preamble(s: &str) -> Result<Option<Ifc2x3EdmPreamble>, String> {
    match split_top_level(strip_brackets(s)?, ',').as_slice() {
        ["0"] => Ok(None),
        ["1", value] => Ok(Some(dec_edm_preamble(value)?)),
        _ => Err(format!("optional EDM preamble: invalid payload {s:?}")),
    }
}
async fn parse_u64(s: &str) -> Result<u64, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    split_top_level_iter(s, sep).collect()
}
async fn split_top_level_iter(s: &str, sep: char) -> impl Iterator<Item = &str> {
    let separator = sep as u8;
    let mut depth = 0i32;
    let mut start = 0usize;
    let mut cursor = 0usize;
    std::iter::from_fn(move || {
        if s.is_empty() || cursor > s.len() {
            return None;
        }
        while cursor < s.len() {
            match s.as_bytes()[cursor] {
                b'[' => depth += 1,
                b']' => depth -= 1,
                byte if byte == separator && depth == 0 => {
                    let item = &s[start..cursor];
                    cursor += 1;
                    start = cursor;
                    return Some(item);
                }
                _ => {}
            }
            cursor += 1;
        }
        cursor += 1;
        Some(&s[start..])
    })
}
pub(crate) async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real varint/length-prefixed binary primitives (own local copy, never shared cross-artifact —
/// same convention the text primitives above use) — reuses `store::pack_rt::write_varint_u64`/
/// `store::ByteReader` rather than reinventing varint encode/decode. `pub(crate)` so the mutations
/// sibling can reuse these too.
pub(crate) async fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
pub(crate) async fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}
pub(crate) async fn enc_edm_preamble_bin(preamble: &Ifc2x3EdmPreamble, out: &mut Vec<u8>) {
    for value in [
        &preamble.producer,
        &preamble.module,
        &preamble.creation_date,
        &preamble.host,
        &preamble.database,
        &preamble.database_version,
        &preamble.database_creation_date,
        &preamble.schema,
        &preamble.model,
        &preamble.model_creation_date,
        &preamble.header_model,
        &preamble.header_model_creation_date,
        &preamble.user,
        &preamble.group,
        &preamble.license,
        &preamble.options,
    ] {
        write_str_bin(out, value);
    }
}
pub(crate) async fn dec_edm_preamble_bin(reader: &mut store::ByteReader<'_>) -> Result<Ifc2x3EdmPreamble, String> {
    Ok(Ifc2x3EdmPreamble {
        producer: read_str_bin(reader)?,
        module: read_str_bin(reader)?,
        creation_date: read_str_bin(reader)?,
        host: read_str_bin(reader)?,
        database: read_str_bin(reader)?,
        database_version: read_str_bin(reader)?,
        database_creation_date: read_str_bin(reader)?,
        schema: read_str_bin(reader)?,
        model: read_str_bin(reader)?,
        model_creation_date: read_str_bin(reader)?,
        header_model: read_str_bin(reader)?,
        header_model_creation_date: read_str_bin(reader)?,
        user: read_str_bin(reader)?,
        group: read_str_bin(reader)?,
        license: read_str_bin(reader)?,
        options: read_str_bin(reader)?,
    })
}
/// ➡️ Zigzag-encodes `value` into `store::pack_rt::write_varint_u64`'s unsigned domain — own local
/// copy (`store::pack_rt` only ships the unsigned writer; the read side's zigzag decode is already
/// built into `store::ByteReader::read_varint_i64`), same convention `4`'s own diff module uses.
async fn write_varint_i64(out: &mut Vec<u8>, value: i64) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    store::pack_rt::write_varint_u64(out, zigzag);
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️Part21ValueCodecs
/// 🔤️ `Part21Value`'s tag scheme, single uppercase letter + bracketed positional payload
/// (payload-free variants `Unset`/`Derived` are the bare letter, no brackets — never ambiguous
/// since every token boundary is whitespace/`,`/`;`, matching `4`'s own `IfcValue` convention
/// exactly, same isomorphic 9-variant shape): `U`=Unset, `D`=Derived, `I[n]`=Int, `R[n]`=Real
/// (Rust's `Display`/`FromStr` for `f64` round-trip exactly), `S[hex]`=Str, `E[hex]`=Enum,
/// `F[n]`=Ref, `A[v,v,...]`=List, `T[hex,[v,v,...]]`=Typed.
async fn enc_part21_value_into(v: &Part21Value, out: &mut String) {
    match v {
        Part21Value::Unset => out.push('U'),
        Part21Value::Derived => out.push('D'),
        Part21Value::Int(i) => {
            write!(out, "I[{i}]").expect("writing to String");
        }
        Part21Value::Real(r) => {
            write!(out, "R[{r}]").expect("writing to String");
        }
        Part21Value::Str(s) => {
            out.push_str("S[");
            hex_encode_into(s.as_bytes(), out);
            out.push(']');
        }
        Part21Value::Enum(s) => {
            out.push_str("E[");
            hex_encode_into(s.as_bytes(), out);
            out.push(']');
        }
        Part21Value::Ref(id) => {
            write!(out, "F[{id}]").expect("writing to String");
        }
        Part21Value::List(items) => {
            out.push_str("A[");
            enc_part21_values_into(items, out);
            out.push(']');
        }
        Part21Value::Typed(name, items) => {
            out.push_str("T[");
            hex_encode_into(name.as_bytes(), out);
            out.push_str(",[");
            enc_part21_values_into(items, out);
            out.push_str("]]");
        }
    }
}
async fn enc_part21_values_into(values: &[Part21Value], out: &mut String) {
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        enc_part21_value_into(value, out);
    }
}
pub(crate) async fn dec_part21_value(s: &str) -> Result<Part21Value, String> {
    if s == "U" {
        return Ok(Part21Value::Unset);
    }
    if s == "D" {
        return Ok(Part21Value::Derived);
    }
    if s.is_empty() {
        return Err("part21 value: empty token".to_string());
    }
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "I" => Ok(Part21Value::Int(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "R" => Ok(Part21Value::Real(Part21Decimal::parse(inner)?)),
        "S" => Ok(Part21Value::Str(dec_str(inner)?)),
        "E" => Ok(Part21Value::Enum(dec_str(inner)?)),
        "F" => Ok(Part21Value::Ref(inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())?)),
        "A" => {
            let items = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_part21_value).collect::<Result<Vec<_>, String>>()?;
            Ok(Part21Value::List(items))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [name, items_s] = parts.as_slice() else { return Err(format!("typed value: expected 2 fields, got {}", parts.len())) };
            let items = split_top_level(strip_brackets(items_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_part21_value).collect::<Result<Vec<_>, String>>()?;
            Ok(Part21Value::Typed(dec_str(name)?, items))
        }
        other => Err(format!("part21 value: unknown tag {other:?}")),
    }
}
pub(crate) async fn enc_part21_value_list(vs: &[Part21Value]) -> String {
    let mut out = String::new();
    enc_part21_value_list_into(vs, &mut out);
    out
}
async fn enc_part21_value_list_into(vs: &[Part21Value], out: &mut String) {
    out.push('[');
    enc_part21_values_into(vs, out);
    out.push(']');
}
pub(crate) async fn dec_part21_value_list(s: &str) -> Result<Vec<Part21Value>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_part21_value).collect()
}

//#region 🔖️Part21ValueBinaryCodecs
/// 🧪️ Real recursive binary twin of [`enc_part21_value`]/[`dec_part21_value`] above — same 0-8
/// ordinal order as the text codec's `U`-`T` tag range (matching `4`'s own `enc_ifc_value_bin`
/// ordinal choice for the isomorphic shape). `List`/`Typed` recurse into
/// [`enc_part21_value_list_bin`] exactly like their text-codec twins — genuine field-by-field
/// binary all the way down, no opaque tail needed (`Part21Value` itself is fully flat/
/// spec-expressible per variant, only the top-level `Ifc2x3Diff`/`Ifc2x3Mutation` frames stop at
/// this recursion's OWN entry point rather than an opaque byte chain).
pub(crate) async fn enc_part21_value_bin(v: &Part21Value, out: &mut Vec<u8>) {
    match v {
        Part21Value::Unset => out.push(0),
        Part21Value::Derived => out.push(1),
        Part21Value::Int(i) => {
            out.push(2);
            write_varint_i64(out, *i);
        }
        Part21Value::Real(r) => {
            out.push(3);
            out.push(r.negative as u8);
            write_str_bin(out, &r.coefficient);
            store::pack_rt::write_varint_u64(out, r.scale as u64);
            match r.exponent {
                None => out.push(0),
                Some(exponent) => {
                    out.push(1);
                    write_varint_i64(out, exponent as i64);
                }
            }
        }
        Part21Value::Str(s) => {
            out.push(4);
            write_str_bin(out, s);
        }
        Part21Value::Enum(s) => {
            out.push(5);
            write_str_bin(out, s);
        }
        Part21Value::Ref(id) => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, *id);
        }
        Part21Value::List(items) => {
            out.push(7);
            enc_part21_value_list_bin(items, out);
        }
        Part21Value::Typed(name, items) => {
            out.push(8);
            write_str_bin(out, name);
            enc_part21_value_list_bin(items, out);
        }
    }
}
pub(crate) async fn dec_part21_value_bin(reader: &mut store::ByteReader<'_>) -> Result<Part21Value, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(Part21Value::Unset),
        1 => Ok(Part21Value::Derived),
        2 => Ok(Part21Value::Int(reader.read_varint_i64().map_err(|e| e.to_string())?)),
        3 => {
            let negative = reader.read_u8().map_err(|e| e.to_string())? != 0;
            let coefficient = read_str_bin(reader)?;
            let scale = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
            let exponent = match reader.read_u8().map_err(|e| e.to_string())? {
                0 => None,
                1 => Some(reader.read_varint_i64().map_err(|e| e.to_string())? as i32),
                tag => return Err(format!("Part21Decimal exponent presence: unknown tag {tag}")),
            };
            Ok(Part21Value::Real(Part21Decimal { negative, coefficient, scale, exponent }))
        }
        4 => Ok(Part21Value::Str(read_str_bin(reader)?)),
        5 => Ok(Part21Value::Enum(read_str_bin(reader)?)),
        6 => Ok(Part21Value::Ref(reader.read_varint_u64().map_err(|e| e.to_string())?)),
        7 => Ok(Part21Value::List(dec_part21_value_list_bin(reader)?)),
        8 => {
            let name = read_str_bin(reader)?;
            let items = dec_part21_value_list_bin(reader)?;
            Ok(Part21Value::Typed(name, items))
        }
        other => Err(format!("part21 value binary: unknown tag {other}")),
    }
}
pub(crate) async fn enc_part21_value_list_bin(vs: &[Part21Value], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, vs.len() as u64);
    for v in vs {
        enc_part21_value_bin(v, out);
    }
}
pub(crate) async fn dec_part21_value_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<Part21Value>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_part21_value_bin(reader)).collect()
}
//#endregion 🔖️Part21ValueBinaryCodecs
//#endregion 🔖️Part21ValueCodecs

//#region 🔖️HeaderInstanceCodecs
/// 📦️ `[fileDescriptionList,fileNameList,fileSchemaList]` — three self-bracketed
/// `part21-value-list`s, matching `4`'s own `enc_ifc_header` positional shape exactly (both
/// standards' HEADER record is the same 3-tuple-of-raw-value-list Part-21 shape).
pub(crate) async fn enc_part21_header(h: &Part21Header) -> String {
    format!("[{},{},{}]", enc_part21_value_list(&h.file_description), enc_part21_value_list(&h.file_name), enc_part21_value_list(&h.file_schema))
}
pub(crate) async fn dec_part21_header(s: &str) -> Result<Part21Header, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [fd, fname, fs] = parts.as_slice() else { return Err(format!("part21 header: expected 3 fields, got {}", parts.len())) };
    Ok(Part21Header { file_description: dec_part21_value_list(fd)?, file_name: dec_part21_value_list(fname)?, file_schema: dec_part21_value_list(fs)? })
}
pub(crate) async fn enc_part21_header_bin(h: &Part21Header, out: &mut Vec<u8>) {
    enc_part21_value_list_bin(&h.file_description, out);
    enc_part21_value_list_bin(&h.file_name, out);
    enc_part21_value_list_bin(&h.file_schema, out);
}
pub(crate) async fn dec_part21_header_bin(reader: &mut store::ByteReader<'_>) -> Result<Part21Header, String> {
    let file_description = dec_part21_value_list_bin(reader)?;
    let file_name = dec_part21_value_list_bin(reader)?;
    let file_schema = dec_part21_value_list_bin(reader)?;
    Ok(Part21Header { file_description, file_name, file_schema })
}

/// 📦️ `[id,[entity,entity,...]]` — a `Part21Instance`'s `entities: Vec<(String,Vec<Part21Value>)>`
/// list has 1 entry for a simple instance, 2+ for a real spec-legal COMPLEX instance (ISO
/// 10303-21 §4.2) — same shape `4`'s own snapshot grammar's `instance-body = entity-record |
/// "(" entity-record+ ")"` recognizes at the exchange-file level, restated here for the diff/op
/// wire's own positional codec. Each entity is `[hexname,[args]]`.
pub(crate) async fn enc_part21_instance(inst: &Part21Instance) -> String {
    let mut out = String::new();
    enc_part21_instance_into(inst, &mut out);
    out
}
async fn enc_part21_instance_into(inst: &Part21Instance, out: &mut String) {
    out.push('[');
    write!(out, "{}", inst.id).expect("writing to String");
    out.push_str(",[");
    for (index, (name, args)) in inst.entities.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        out.push('[');
        hex_encode_into(name.as_bytes(), out);
        out.push(',');
        enc_part21_value_list_into(args, out);
        out.push(']');
    }
    out.push_str("]]");
}
pub(crate) async fn dec_part21_instance(s: &str) -> Result<Part21Instance, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id_s, entities_s] = parts.as_slice() else {
        return Err(format!("part21 instance: expected 2 fields, got {}", parts.len()));
    };
    let entities_inner = strip_brackets(entities_s)?;
    let entities = split_top_level_iter(entities_inner, ',')
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let e = split_top_level(strip_brackets(entry)?, ',');
            let [name, args] = e.as_slice() else {
                return Err(format!("part21 entity: expected 2 fields, got {}", e.len()));
            };
            Ok((dec_str(name)?, dec_part21_value_list(args)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Part21Instance { id: parse_u64(id_s)?, entities })
}
pub(crate) async fn enc_part21_instance_bin(inst: &Part21Instance, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, inst.id);
    store::pack_rt::write_varint_u64(out, inst.entities.len() as u64);
    for (name, args) in &inst.entities {
        write_str_bin(out, name);
        enc_part21_value_list_bin(args, out);
    }
}
pub(crate) async fn dec_part21_instance_bin(reader: &mut store::ByteReader<'_>) -> Result<Part21Instance, String> {
    let id = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entities = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let name = read_str_bin(reader)?;
        let args = dec_part21_value_list_bin(reader)?;
        entities.push((name, args));
    }
    Ok(Part21Instance { id, entities })
}
pub(crate) async fn enc_instance_list_into(list: &[Part21Instance], out: &mut String) {
    out.push('[');
    for (index, instance) in list.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        enc_part21_instance_into(instance, out);
    }
    out.push(']');
}
pub(crate) async fn dec_instance_list(s: &str) -> Result<Vec<Part21Instance>, String> {
    split_top_level_iter(strip_brackets(s)?, ',').filter(|s| !s.is_empty()).map(dec_part21_instance).collect()
}
pub(crate) async fn enc_instance_list_bin(list: &[Part21Instance], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for inst in list {
        enc_part21_instance_bin(inst, out);
    }
}
pub(crate) async fn dec_instance_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<Part21Instance>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_part21_instance_bin(reader)).collect()
}
//#endregion 🔖️HeaderInstanceCodecs

//#region 🔖️TopLevel
/// 🔖️ One line of space-separated `key=value` tokens, only the CHANGED top-level fields present,
/// in declared field order (`schema`/`header`/`removed`/`upserted`) — matching `4`'s own
/// `print_ifc_diff` shape exactly.
async fn print_ifc2x3_diff(d: &Ifc2x3Diff) -> String {
    let mut out = String::new();
    let separate = |out: &mut String| {
        if !out.is_empty() {
            out.push(' ');
        }
    };
    if let Some(s) = &d.schema {
        separate(&mut out);
        out.push_str("schema=");
        out.push_str(&enc_str(s));
    }
    if let Some(h) = &d.header {
        separate(&mut out);
        out.push_str("header=");
        out.push_str(&enc_part21_header(h));
    }
    if !d.removed_instances.is_empty() {
        separate(&mut out);
        out.push_str("removed=[");
        for (index, id) in d.removed_instances.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }
            out.push_str(&id.to_string());
        }
        out.push(']');
    }
    if !d.upserted_instances.is_empty() {
        separate(&mut out);
        out.push_str("upserted=");
        enc_instance_list_into(&d.upserted_instances, &mut out);
    }
    if let Some(preamble) = &d.edm_preamble {
        separate(&mut out);
        out.push_str("edm-preamble=");
        out.push_str(&enc_optional_edm_preamble(preamble));
    }
    if let Some(order) = &d.instance_order {
        separate(&mut out);
        out.push_str("instance-order=[");
        for (index, id) in order.iter().enumerate() {
            if index != 0 {
                out.push(',');
            }
            out.push_str(&id.to_string());
        }
        out.push(']');
    }
    out
}
async fn parse_ifc2x3_diff(line: &str) -> Result<Ifc2x3Diff, String> {
    let mut d = Ifc2x3Diff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("schema=") {
            d.schema = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("header=") {
            d.header = Some(dec_part21_header(rest)?);
        } else if let Some(rest) = token.strip_prefix("removed=") {
            d.removed_instances = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_u64).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = token.strip_prefix("upserted=") {
            d.upserted_instances = dec_instance_list(rest)?;
        } else if let Some(rest) = token.strip_prefix("edm-preamble=") {
            d.edm_preamble = Some(dec_optional_edm_preamble(rest)?);
        } else if let Some(rest) = token.strip_prefix("instance-order=") {
            d.instance_order = Some(split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|value| !value.is_empty()).map(parse_u64).collect::<Result<Vec<_>, _>>()?);
        } else {
            return Err(format!("ifc2x3 diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for Ifc2x3Diff {
    async fn print_diff(&self) -> String {
        print_ifc2x3_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_ifc2x3_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ REAL binary frame (`format u8 | flags u8 | field payloads...`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// no F6/text-as-binary shortcut ever existed for this facet (there was no `DiffCodec` impl at
    /// all before this wave). `flags` bit0..3 = `schema`/`header`/non-empty-`removed_instances`/
    /// non-empty-`upserted_instances` presence (same order the text grammar's token list uses);
    /// each present field's payload follows immediately in that order, real field-by-field binary
    /// all the way down — only the innermost recursive `Part21Value::List`/`Typed` payload bottoms
    /// out via `enc_part21_value_bin`'s own recursive call (not an opaque tail: `Part21Value` is
    /// fully spec-expressible per variant).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let flags: u8 = (self.schema.is_some() as u8)
            | ((self.header.is_some() as u8) << 1)
            | ((!self.removed_instances.is_empty() as u8) << 2)
            | ((!self.upserted_instances.is_empty() as u8) << 3)
            | ((self.edm_preamble.is_some() as u8) << 4)
            | ((self.instance_order.is_some() as u8) << 5);
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(s) = &self.schema {
            write_str_bin(&mut out, s);
        }
        if let Some(h) = &self.header {
            enc_part21_header_bin(h, &mut out);
        }
        if !self.removed_instances.is_empty() {
            store::pack_rt::write_varint_u64(&mut out, self.removed_instances.len() as u64);
            for id in &self.removed_instances {
                store::pack_rt::write_varint_u64(&mut out, *id);
            }
        }
        if !self.upserted_instances.is_empty() {
            enc_instance_list_bin(&self.upserted_instances, &mut out);
        }
        if let Some(preamble) = &self.edm_preamble {
            match preamble {
                None => out.push(0),
                Some(value) => {
                    out.push(1);
                    enc_edm_preamble_bin(value, &mut out);
                }
            }
        }
        if let Some(order) = &self.instance_order {
            store::pack_rt::write_varint_u64(&mut out, order.len() as u64);
            for id in order {
                store::pack_rt::write_varint_u64(&mut out, *id);
            }
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("diff format", 0, format!("unsupported format {format}")));
        }
        let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        if flags & !0b0011_1111 != 0 {
            return Err(malformed("diff flags", 1, "unknown flag bits".into()));
        }
        let schema = if flags & 1 != 0 { Some(read_str_bin(&mut reader).map_err(|e| malformed("diff schema", reader.position(), e))?) } else { None };
        let header = if flags & 2 != 0 { Some(dec_part21_header_bin(&mut reader).map_err(|e| malformed("diff header", reader.position(), e))?) } else { None };
        let removed_instances = if flags & 4 != 0 {
            let count = reader.read_varint_u64().map_err(|e| malformed("diff removed count", reader.position(), e.to_string()))?;
            let mut v = Vec::with_capacity(count as usize);
            for _ in 0..count {
                v.push(reader.read_varint_u64().map_err(|e| malformed("diff removed id", reader.position(), e.to_string()))?);
            }
            v
        } else {
            Vec::new()
        };
        let upserted_instances = if flags & 8 != 0 { dec_instance_list_bin(&mut reader).map_err(|e| malformed("diff upserted", reader.position(), e))? } else { Vec::new() };
        let edm_preamble = if flags & 16 != 0 {
            Some(match reader.read_u8().map_err(|e| malformed("diff EDM preamble presence", reader.position(), e.to_string()))? {
                0 => None,
                1 => Some(dec_edm_preamble_bin(&mut reader).map_err(|e| malformed("diff EDM preamble", reader.position(), e))?),
                tag => return Err(malformed("diff EDM preamble presence", reader.position(), format!("unknown tag {tag}"))),
            })
        } else {
            None
        };
        let instance_order = if flags & 32 != 0 {
            let count = reader.read_varint_u64().map_err(|error| malformed("diff instance order", reader.position(), error.to_string()))?;
            let mut order = Vec::with_capacity(count as usize);
            for _ in 0..count {
                order.push(reader.read_varint_u64().map_err(|error| malformed("diff instance order", reader.position(), error.to_string()))?);
            }
            Some(order)
        } else {
            None
        };
        if reader.remaining() != 0 {
            return Err(malformed("diff trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(Ifc2x3Diff { schema, header, removed_instances, upserted_instances, edm_preamble, instance_order })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ Representative `Ifc2x3Diff` cases — real `print_diff()`-conformance-law fodder
/// (`diff_grammar_conformance_law`) and `protocol_walk_law` fodder — the empty diff, a genuine
/// `between()` result exercising every top-level field (schema/header/removed/upserted, incl. a
/// COMPLEX instance and every `Part21Value` tag), and its reverse direction.
#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<Ifc2x3Diff> {
    let a = crate::artifacts::ifc::standards::v2x3::engine::demo_ifc2x3_snapshot();
    let mut b = a.clone();
    b.schema = "stdio.ifc.2x3.v2".into();
    b.document.header.file_name = vec![Part21Value::Str("changed.ifc".into())];
    b.document.instances.retain(|i| i.id != 2);
    if let Some(first) = b.document.instances.first_mut() {
        first.entities = vec![("IFCQUANTITYAREA".into(), vec![Part21Value::Real(10.5.into()), Part21Value::Enum("EDGE".into())]), ("IFCPHYSICALSIMPLEQUANTITY".into(), vec![Part21Value::Unset])];
    }
    b.document
        .instances
        .push(Part21Instance { id: 300, entities: vec![("IFCBUILDINGSTOREY".into(), vec![Part21Value::List(vec![Part21Value::Int(1), Part21Value::Int(2)]), Part21Value::Typed("IFCLENGTHMEASURE".into(), vec![Part21Value::Real(3000.0.into())])])] });
    vec![Ifc2x3Diff::default(), Ifc2x3Diff::between(&a, &b), Ifc2x3Diff::between(&b, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn invalid_instance_order_is_rejected_before_mutation() {
        let base = Ifc2x3Snapshot::default();
        let diff = Ifc2x3Diff { instance_order: Some(vec![1]), ..Default::default() };
        let error = diff.apply(&base).expect_err("unknown instance order target must be rejected");
        assert_eq!(error.code, "invalid-instance-order");
        assert_eq!(error.target, vec!["instanceOrder", "1"]);
        assert_eq!(base, Ifc2x3Snapshot::default());
    }
    use crate::artifacts::step::engine::part21::Part21Value;

    async fn inst(id: u64, name: &str) -> Part21Instance {
        Part21Instance { id, entities: vec![(name.to_string(), vec![Part21Value::Int(id as i64)])] }
    }

    async fn snap(schema: &str, header: Part21Header, instances: Vec<Part21Instance>) -> Ifc2x3Snapshot {
        let document = crate::artifacts::step::engine::part21::Part21Document { header, instances };
        Ifc2x3Snapshot { schema: schema.into(), document, edm_preamble: None }
    }

    /// 🧪️ THE acceptance criterion for "diff can change every field": schema, header, and
    /// instance add/remove/modify all round-trip through `between`+`apply`.
    #[test]
    async fn field_sweep_between_covers_every_field() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL"), inst(2, "IFCDOOR")]);
        let mut next_header = Part21Header::default();
        next_header.file_schema = vec![Part21Value::Str("IFC2X3".into())];
        let next = snap(
            "stdio.ifc.2x3.v2",
            next_header,
            vec![inst(1, "IFCWALLSTANDARDCASE"), inst(3, "IFCWINDOW")], // 1 modified, 2 removed, 3 added
        );
        let d = Ifc2x3Diff::between(&base, &next);
        assert!(d.schema.is_some());
        assert!(d.header.is_some());
        assert_eq!(d.removed_instances, vec![2]);
        assert_eq!(d.upserted_instances.len(), 2);
        assert_eq!(d.apply(&base).expect("valid between diff"), next);
    }

    #[test]
    async fn absorb_upsert_then_remove_same_id_cancels_to_removed_only() {
        let mut d1 = Ifc2x3Diff { upserted_instances: vec![inst(5, "IFCSLAB")], ..Default::default() };
        let d2 = Ifc2x3Diff { removed_instances: vec![5], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.upserted_instances.is_empty());
        assert_eq!(d1.removed_instances, vec![5]);
    }

    #[test]
    async fn absorb_remove_then_upsert_same_id_un_removes() {
        let mut d1 = Ifc2x3Diff { removed_instances: vec![7], ..Default::default() };
        let d2 = Ifc2x3Diff { upserted_instances: vec![inst(7, "IFCBEAM")], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.removed_instances.is_empty());
        assert_eq!(d1.upserted_instances, vec![inst(7, "IFCBEAM")]);
    }

    #[test]
    async fn absorb_matches_sequential_apply() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL")]);
        let d1 = Ifc2x3Diff { upserted_instances: vec![inst(2, "IFCDOOR")], ..Default::default() };
        let d2 = Ifc2x3Diff { removed_instances: vec![1], upserted_instances: vec![inst(3, "IFCWINDOW")], ..Default::default() };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let sequential = {
            let mid = d1.apply(&base).expect("valid first diff");
            d2.apply(&mid).expect("valid second diff")
        };
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), sequential);
    }

    #[test]
    async fn inverse_diff_level_roundtrip() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL"), inst(2, "IFCDOOR")]);
        let d = Ifc2x3Diff { removed_instances: vec![2], upserted_instances: vec![inst(1, "IFCWALLSTANDARDCASE"), inst(4, "IFCCOLUMN")], ..Default::default() };
        let next = d.apply(&base).expect("valid forward diff");
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&next).expect("valid inverse diff"), base);
    }

    #[test]
    async fn between_self_is_empty() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL")]);
        assert!(Ifc2x3Diff::between(&base, &base).is_empty());
    }

    //#region 🔖️diff_codec_text_binary_roundtrip_law
    /// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `Ifc2x3Diff` grammar — exercises every
    /// top-level field (`schema`/`header`/`removed`/`upserted`) and every `Part21Value` tag incl.
    /// `List`/`Typed` recursion and a real COMPLEX instance (2-entry `entities`).
    #[test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        let complex_inst =
            Part21Instance { id: 9, entities: vec![("IFCQUANTITYAREA".into(), vec![Part21Value::Real(10.5.into()), Part21Value::Int(-3), Part21Value::Enum("EDGE".into())]), ("IFCPHYSICALSIMPLEQUANTITY".into(), vec![Part21Value::Unset])] };
        let cases = vec![
            Ifc2x3Diff::default(),
            Ifc2x3Diff {
                schema: Some("stdio.ifc.2x3.v2".into()),
                header: Some(Part21Header {
                    file_description: vec![Part21Value::Str("desc".into())],
                    file_name: vec![Part21Value::List(vec![Part21Value::Str("a".into()), Part21Value::Unset])],
                    file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
                }),
                removed_instances: vec![1, 2],
                upserted_instances: vec![complex_inst.clone(), Part21Instance { id: 300, entities: vec![("IFCBUILDINGSTOREY".into(), vec![Part21Value::Typed("IFCLENGTHMEASURE".into(), vec![Part21Value::Real(3000.0.into())])])] }],
                edm_preamble: None,
                instance_order: None,
            },
            Ifc2x3Diff { removed_instances: vec![7], ..Default::default() },
            Ifc2x3Diff { upserted_instances: vec![complex_inst], ..Default::default() },
        ];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = Ifc2x3Diff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e:?}"));
            let decoded = Ifc2x3Diff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e:?}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion 🔖️diff_codec_text_binary_roundtrip_law
}
//#endregion 🧪️Tests
