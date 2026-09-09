//! 🔺️ SemioObjectDiff — sparse per-field diff over `SemioObjectSnapshot`. Four independently
//! diffable fields (`transform`, `brep`, `mesh`, `properties`) — an `Option<…>` slot per field
//! (`None` = untouched by this diff), the same per-field shape `🖼️image`'s diff facet uses. The
//! two-level `Option<Option<store::ArtifactChild<S>>>` on the three child fields is real, not
//! decorative: outer `None` = "this diff doesn't touch the slot", `Some(None)` = "clear it",
//! `Some(Some(handle))` = "set it to this handle" — each triad's own `🔺️diff` leaf builds this
//! directly from `(payload, base)`, never apply-then-capture.

use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object.diff")]
pub struct SemioObjectDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<SemioTransform>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brep: Option<Option<store::ArtifactChild<SemioBrepSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Option<store::ArtifactChild<SemioMeshSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Option<store::ArtifactChild<SemioValueSnapshot>>>,
}

impl dsl::FromValue for SemioObjectDiff {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut diff = Self::default();
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "transform" => diff.transform = Some(dsl::FromValue::from_value(value)?),
                "brep" => diff.brep = Some(dsl::FromValue::from_value(value)?),
                "mesh" => diff.mesh = Some(dsl::FromValue::from_value(value)?),
                "properties" => diff.properties = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(format!("unknown Object diff field {key}"))),
            }
        }
        diff.validate().map_err(dsl::ValueError::new)?;
        Ok(diff)
    }
}

impl SemioObjectDiff {
    /// 🧩️ Validates every supplied child replacement before publication.
    pub fn validate(&self) -> Result<(), String> {
        use crate::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if let Some(Some(child)) = &self.brep { validate_semio_child_identity(&child.child_id, &child.target, "brep")?; }
        if let Some(Some(child)) = &self.mesh { validate_semio_child_identity(&child.child_id, &child.target, "mesh")?; }
        if let Some(Some(child)) = &self.properties { validate_semio_child_identity(&child.child_id, &child.target, "value")?; }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.transform.is_none() && self.brep.is_none() && self.mesh.is_none() && self.properties.is_none()
    }
}

impl MutationDiff<SemioObjectSnapshot> for SemioObjectDiff {
    fn apply(&self, base: &SemioObjectSnapshot) -> protocol::MutationApplyResult<SemioObjectSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.child-identity".into(), message, target: Vec::new() })?;
        let mut next = base.clone();
        if let Some(t) = &self.transform {
            next.transform = *t;
        }
        if let Some(b) = &self.brep {
            next.brep = b.clone();
        }
        if let Some(m) = &self.mesh {
            next.mesh = m.clone();
        }
        if let Some(p) = &self.properties {
            next.properties = p.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.transform.is_some() {
            self.transform = other.transform;
        }
        if other.brep.is_some() {
            self.brep = other.brep;
        }
        if other.mesh.is_some() {
            self.mesh = other.mesh;
        }
        if other.properties.is_some() {
            self.properties = other.properties;
        }
    }
}

/// 🧮️ `object`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch.
impl protocol::command::DiffAlgebra<SemioObjectSnapshot> for SemioObjectDiff {
    fn between(base: &SemioObjectSnapshot, other: &SemioObjectSnapshot) -> Self {
        SemioObjectDiff {
            transform: (base.transform != other.transform).then_some(other.transform),
            brep: (base.brep != other.brep).then(|| other.brep.clone()),
            mesh: (base.mesh != other.mesh).then(|| other.mesh.clone()),
            properties: (base.properties != other.properties).then(|| other.properties.clone()),
        }
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Self {
        SemioObjectDiff {
            transform: self.transform.as_ref().map(|_| base.transform),
            brep: self.brep.as_ref().map(|_| base.brep.clone()),
            mesh: self.mesh.as_ref().map(|_| base.mesh.clone()),
            properties: self.properties.as_ref().map(|_| base.properties.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
use crate::standards::v1::subsets::object::schema::snapshot::{dec_child_opt, dec_transform, enc_child_opt, enc_transform};

/// 🧾️ `<hex-flag><line>` per field, `\n`-joined, empty string = no-op diff — real, not decorative.
/// `t=`/`b=`/`m=`/`p=` prefixes; a field absent from the diff simply has no line.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_object_diff(d: &SemioObjectDiff) -> String {
    let mut lines = Vec::new();
    if let Some(t) = &d.transform {
        lines.push(format!("t={}", enc_transform(t)));
    }
    if let Some(b) = &d.brep {
        lines.push(format!("b={}", enc_child_opt(b)));
    }
    if let Some(m) = &d.mesh {
        lines.push(format!("m={}", enc_child_opt(m)));
    }
    if let Some(p) = &d.properties {
        lines.push(format!("p={}", enc_child_opt(p)));
    }
    lines.join(";")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_object_diff(line: &str) -> Result<SemioObjectDiff, String> {
    let mut d = SemioObjectDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
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
    fn print_diff(&self) -> String {
        print_object_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_object_diff(line).and_then(|diff| { diff.validate()?; Ok(diff) }).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=transform, bit1=brep,
    /// bit2=mesh, bit3=properties), then each present field's own real encoding in bit order.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        use crate::standards::v1::subsets::object::schema::snapshot::{write_child_opt, write_transform};
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence: u8 = 0;
        if self.transform.is_some() {
            presence |= 0b0001;
        }
        if self.brep.is_some() {
            presence |= 0b0010;
        }
        if self.mesh.is_some() {
            presence |= 0b0100;
        }
        if self.properties.is_some() {
            presence |= 0b1000;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(t) = &self.transform {
            write_transform(&mut out, t);
        }
        if let Some(b) = &self.brep {
            write_child_opt(&mut out, b);
        }
        if let Some(m) = &self.mesh {
            write_child_opt(&mut out, m);
        }
        if let Some(p) = &self.properties {
            write_child_opt(&mut out, p);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        use crate::standards::v1::subsets::object::schema::snapshot::{read_child_opt, read_transform};
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
        let diff = SemioObjectDiff { transform, brep, mesh, properties };
        diff.validate().map_err(map_err)?;
        Ok(diff)
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioObjectDiff` cases — single source of truth for
/// `diff_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioObjectDiff> {
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    vec![
        SemioObjectDiff::default(),
        SemioObjectDiff { transform: Some(SemioTransform { translation: SemioPoint3 { x: 5.0, y: 0.0, z: 0.0 }, ..SemioTransform::identity() }), ..Default::default() },
        SemioObjectDiff { brep: Some(None), ..Default::default() },
        SemioObjectDiff {
            mesh: Some(Some(store::ArtifactChild::new(
                "m1".into(),
                store::os_io::ArtifactRef { artifact_id: "m1".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() } },
            ))),
            ..Default::default()
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
