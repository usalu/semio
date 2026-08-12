//! 🔺️ SemioObjectDiff — sparse per-field diff over `SemioObjectSnapshot`. Four independently
//! diffable fields (`transform`, `brep`, `mesh`, `properties`) — an `Option<…>` slot per field
//! (`None` = untouched by this diff), the same per-field shape `✳️image`'s diff facet uses. The
//! two-level `Option<Option<store::ArtifactChild<S>>>` on the three child fields is real, not
//! decorative: outer `None` = "this diff doesn't touch the slot", `Some(None)` = "clear it",
//! `Some(Some(handle))` = "set it to this handle" — each triad's own `🔺️diff` leaf builds this
//! directly from `(payload, base)`, never apply-then-capture.

use crate::artifacts::semio::standards::v1::engine::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use protocol::MutationDiff;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object.diff")]
pub struct SemioObjectDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<SemioTransform>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brep: Option<Option<store::ArtifactChild<SemioBrepSnapshot>>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Option<store::ArtifactChild<SemioMeshSnapshot>>>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Option<store::ArtifactChild<SemioValueSnapshot>>>,
}

impl SemioObjectDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.transform.is_none() && self.brep.is_none() && self.mesh.is_none() && self.properties.is_none()
    }
}

impl MutationDiff<SemioObjectSnapshot> for SemioObjectDiff {
    fn apply(&self, base: &SemioObjectSnapshot) -> SemioObjectSnapshot {
        let mut next = base.clone();
        if let Some(t) = &self.transform { next.transform = t.clone(); }
        if let Some(b) = &self.brep { next.brep = b.clone(); }
        if let Some(m) = &self.mesh { next.mesh = m.clone(); }
        if let Some(p) = &self.properties { next.properties = p.clone(); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.transform.is_some() { self.transform = other.transform; }
        if other.brep.is_some() { self.brep = other.brep; }
        if other.mesh.is_some() { self.mesh = other.mesh; }
        if other.properties.is_some() { self.properties = other.properties; }
    }
}

/// 🧮️ `object`'s own `DiffAlgebra` — required by the `✳️any` envelope's own dispatch.
impl protocol::command::DiffAlgebra<SemioObjectSnapshot> for SemioObjectDiff {
    fn between(base: &SemioObjectSnapshot, other: &SemioObjectSnapshot) -> Self {
        SemioObjectDiff {
            transform: (base.transform != other.transform).then(|| other.transform.clone()),
            brep: (base.brep != other.brep).then(|| other.brep.clone()),
            mesh: (base.mesh != other.mesh).then(|| other.mesh.clone()),
            properties: (base.properties != other.properties).then(|| other.properties.clone()),
        }
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Self {
        SemioObjectDiff {
            transform: self.transform.as_ref().map(|_| base.transform.clone()),
            brep: self.brep.as_ref().map(|_| base.brep.clone()),
            mesh: self.mesh.as_ref().map(|_| base.mesh.clone()),
            properties: self.properties.as_ref().map(|_| base.properties.clone()),
        }
    }
    fn is_empty(&self) -> bool { self.is_empty_diff() }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{enc_child_opt, dec_child_opt, enc_transform, dec_transform};

/// 🧾️ `<hex-flag><line>` per field, `\n`-joined, empty string = no-op diff — real, not decorative.
/// `t=`/`b=`/`m=`/`p=` prefixes; a field absent from the diff simply has no line.
fn print_object_diff(d: &SemioObjectDiff) -> String {
    let mut lines = Vec::new();
    if let Some(t) = &d.transform { lines.push(format!("t={}", enc_transform(t))); }
    if let Some(b) = &d.brep { lines.push(format!("b={}", enc_child_opt(b))); }
    if let Some(m) = &d.mesh { lines.push(format!("m={}", enc_child_opt(m))); }
    if let Some(p) = &d.properties { lines.push(format!("p={}", enc_child_opt(p))); }
    lines.join(";")
}
fn parse_object_diff(line: &str) -> Result<SemioObjectDiff, String> {
    let mut d = SemioObjectDiff::default();
    if line.is_empty() { return Ok(d); }
    for field in line.split(';') {
        let (tag, rest) = field.split_once('=').ok_or_else(|| format!("object diff: missing '=' in {field:?}"))?;
        match tag {
            "t" => d.transform = Some(dec_transform(rest)?),
            "b" => d.brep = Some(dec_child_opt(rest)?),
            "m" => d.mesh = Some(dec_child_opt(rest)?),
            "p" => d.properties = Some(dec_child_opt(rest)?),
            other => return Err(format!("object diff: unknown field tag {other:?}")),
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for SemioObjectDiff {
    fn print_diff(&self) -> String { print_object_diff(self) }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_object_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=transform, bit1=brep,
    /// bit2=mesh, bit3=properties), then each present field's own real encoding in bit order.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{write_transform, write_child_opt};
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence: u8 = 0;
        if self.transform.is_some() { presence |= 0b0001; }
        if self.brep.is_some() { presence |= 0b0010; }
        if self.mesh.is_some() { presence |= 0b0100; }
        if self.properties.is_some() { presence |= 0b1000; }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(t) = &self.transform { write_transform(&mut out, t); }
        if let Some(b) = &self.brep { write_child_opt(&mut out, b); }
        if let Some(m) = &self.mesh { write_child_opt(&mut out, m); }
        if let Some(p) = &self.properties { write_child_opt(&mut out, p); }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{read_transform, read_child_opt};
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let map_err = |e: String| protocol::ProtocolError::Malformed { what: "object diff field", offset: 2, detail: e };
        let transform = if presence & 0b0001 != 0 { Some(read_transform(&mut reader).map_err(map_err)?) } else { None };
        let brep = if presence & 0b0010 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
        let mesh = if presence & 0b0100 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
        let properties = if presence & 0b1000 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
        Ok(SemioObjectDiff { transform, brep, mesh, properties })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioObjectDiff` cases — single source of truth for
/// `diff_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SemioObjectDiff> {
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
    vec![
        SemioObjectDiff::default(),
        SemioObjectDiff { transform: Some(SemioTransform { translation: SemioPoint3 { x: 5.0, y: 0.0, z: 0.0 }, ..SemioTransform::identity() }), ..Default::default() },
        SemioObjectDiff { brep: Some(None), ..Default::default() },
        SemioObjectDiff {
            mesh: Some(Some(store::ArtifactChild::new(
                "mesh-x".into(),
                store::os_io::ArtifactRef { artifact_id: "m1".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() } },
            ))),
            ..Default::default()
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::DiffCodec;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::demo_object_snapshot;

    #[test]
    fn apply_replaces_touched_fields_only() {
        let base = demo_object_snapshot();
        let diff = SemioObjectDiff { brep: Some(None), ..Default::default() };
        let next = diff.apply(&base);
        assert!(next.brep.is_none());
        assert_eq!(next.mesh, base.mesh, "untouched fields must be preserved");
    }

    #[test]
    fn absorb_last_write_wins_per_field() {
        let mut d1 = SemioObjectDiff { brep: Some(None), ..Default::default() };
        let d2 = SemioObjectDiff { mesh: Some(None), ..Default::default() };
        d1.absorb(d2.clone());
        assert_eq!(d1.brep, Some(None));
        assert_eq!(d1.mesh, Some(None));
    }

    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioObjectDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioObjectDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🔖️Tests
