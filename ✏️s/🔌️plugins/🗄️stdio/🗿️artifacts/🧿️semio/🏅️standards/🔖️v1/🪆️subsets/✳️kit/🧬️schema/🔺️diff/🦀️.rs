//! 🔺️ SemioKitDiff — sparse per-field diff over `SemioKitSnapshot`. Six independently diffable
//! fields, each an `Option<…>` slot (`None` = untouched by this diff) — the same per-field shape
//! `✳️object`'s diff facet uses, scaled up to six fields including two CHILD collections, one
//! optional CHILD slot, and one LINK collection. Every mutation triad's own `🔺️diff` leaf builds
//! the touched field's whole new value directly from `(payload, base)`, never apply-then-capture
//! — `📓️taxonomy.md`'s whole-list-replace convention, unchanged by D2's resolution (Concern B:
//! the shape itself was never the defect).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use protocol::MutationDiff;
use schema::ArtifactSchema;

//#region 🔖️ListWrappers
/// 📋 Whole-list wrappers, one per collection field — every mutation triad rebuilds the full
/// ordered `values` vec from `base` and wraps it here (`✳️text`'s own `SemioTextRunList` shape).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitTypeList {
    pub values: Vec<SemioKitType>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitDesignList {
    pub values: Vec<SemioKitDesign>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitObjectChildList {
    pub values: Vec<store::ArtifactChild<SemioObjectSnapshot>>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitModelChildList {
    pub values: Vec<store::ArtifactChild<SemioModelSnapshot>>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitLinkList {
    pub values: Vec<store::ArtifactLink>,
}
//#endregion 🔖️ListWrappers

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.kit.diff")]
pub struct SemioKitDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<SemioKitTypeList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub designs: Option<SemioKitDesignList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<SemioKitObjectChildList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<SemioKitModelChildList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Option<store::ArtifactChild<SemioValueSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub representations: Option<SemioKitLinkList>,
}

impl SemioKitDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.types.is_none() && self.designs.is_none() && self.objects.is_none() && self.models.is_none() && self.properties.is_none() && self.representations.is_none()
    }
}

impl MutationDiff<SemioKitSnapshot> for SemioKitDiff {
    fn apply(&self, base: &SemioKitSnapshot) -> protocol::MutationApplyResult<SemioKitSnapshot> {
        let mut next = base.clone();
        if let Some(t) = &self.types {
            next.types = t.values.clone();
        }
        if let Some(d) = &self.designs {
            next.designs = d.values.clone();
        }
        if let Some(o) = &self.objects {
            next.objects = o.values.clone();
        }
        if let Some(m) = &self.models {
            next.models = m.values.clone();
        }
        if let Some(p) = &self.properties {
            next.properties = p.clone();
        }
        if let Some(r) = &self.representations {
            next.representations = r.values.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.types.is_some() {
            self.types = other.types;
        }
        if other.designs.is_some() {
            self.designs = other.designs;
        }
        if other.objects.is_some() {
            self.objects = other.objects;
        }
        if other.models.is_some() {
            self.models = other.models;
        }
        if other.properties.is_some() {
            self.properties = other.properties;
        }
        if other.representations.is_some() {
            self.representations = other.representations;
        }
    }
}

/// 🧮️ `kit`'s own `DiffAlgebra` — required by the `✳️base` envelope's own dispatch.
impl protocol::command::DiffAlgebra<SemioKitSnapshot> for SemioKitDiff {
    fn between(base: &SemioKitSnapshot, other: &SemioKitSnapshot) -> Self {
        SemioKitDiff {
            types: (base.types != other.types).then(|| SemioKitTypeList { values: other.types.clone() }),
            designs: (base.designs != other.designs).then(|| SemioKitDesignList { values: other.designs.clone() }),
            objects: (base.objects != other.objects).then(|| SemioKitObjectChildList { values: other.objects.clone() }),
            models: (base.models != other.models).then(|| SemioKitModelChildList { values: other.models.clone() }),
            properties: (base.properties != other.properties).then(|| other.properties.clone()),
            representations: (base.representations != other.representations).then(|| SemioKitLinkList { values: other.representations.clone() }),
        }
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Self {
        SemioKitDiff {
            types: self.types.as_ref().map(|_| SemioKitTypeList { values: base.types.clone() }),
            designs: self.designs.as_ref().map(|_| SemioKitDesignList { values: base.designs.clone() }),
            objects: self.objects.as_ref().map(|_| SemioKitObjectChildList { values: base.objects.clone() }),
            models: self.models.as_ref().map(|_| SemioKitModelChildList { values: base.models.clone() }),
            properties: self.properties.as_ref().map(|_| base.properties.clone()),
            representations: self.representations.as_ref().map(|_| SemioKitLinkList { values: base.representations.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{dec_child_list, dec_child_opt, dec_design_list, dec_link_list, dec_type_list, enc_child_list, enc_child_opt, enc_design_list, enc_link_list, enc_type_list};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_kit_diff(d: &SemioKitDiff) -> String {
    let mut fields = Vec::new();
    if let Some(t) = &d.types {
        fields.push(format!("t={}", enc_type_list(&t.values)));
    }
    if let Some(dd) = &d.designs {
        fields.push(format!("d={}", enc_design_list(&dd.values)));
    }
    if let Some(o) = &d.objects {
        fields.push(format!("o={}", enc_child_list(&o.values)));
    }
    if let Some(m) = &d.models {
        fields.push(format!("m={}", enc_child_list(&m.values)));
    }
    if let Some(p) = &d.properties {
        fields.push(format!("p={}", enc_child_opt(p)));
    }
    if let Some(r) = &d.representations {
        fields.push(format!("r={}", enc_link_list(&r.values)));
    }
    fields.join(";")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_kit_diff(line: &str) -> Result<SemioKitDiff, String> {
    let mut d = SemioKitDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for field in line.split(';') {
        let (tag, rest) = field.split_once('=').ok_or_else(|| format!("kit diff: missing '=' in {field:?}"))?;
        match tag {
            "t" => d.types = Some(SemioKitTypeList { values: dec_type_list(rest)? }),
            "d" => d.designs = Some(SemioKitDesignList { values: dec_design_list(rest)? }),
            "o" => d.objects = Some(SemioKitObjectChildList { values: dec_child_list(rest)? }),
            "m" => d.models = Some(SemioKitModelChildList { values: dec_child_list(rest)? }),
            "p" => d.properties = Some(dec_child_opt(rest)?),
            "r" => d.representations = Some(SemioKitLinkList { values: dec_link_list(rest)? }),
            other => return Err(format!("kit diff: unknown field tag {other:?}")),
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for SemioKitDiff {
    fn print_diff(&self) -> String {
        print_kit_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_kit_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=types, bit1=designs,
    /// bit2=objects, bit3=models, bit4=properties, bit5=representations), then each present
    /// field's own real encoding in bit order.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{write_child_list, write_child_opt, write_design_list, write_link_list, write_type_list};
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence: u8 = 0;
        if self.types.is_some() {
            presence |= 0b0000_0001;
        }
        if self.designs.is_some() {
            presence |= 0b0000_0010;
        }
        if self.objects.is_some() {
            presence |= 0b0000_0100;
        }
        if self.models.is_some() {
            presence |= 0b0000_1000;
        }
        if self.properties.is_some() {
            presence |= 0b0001_0000;
        }
        if self.representations.is_some() {
            presence |= 0b0010_0000;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(t) = &self.types {
            write_type_list(&mut out, &t.values);
        }
        if let Some(d) = &self.designs {
            write_design_list(&mut out, &d.values);
        }
        if let Some(o) = &self.objects {
            write_child_list(&mut out, &o.values);
        }
        if let Some(m) = &self.models {
            write_child_list(&mut out, &m.values);
        }
        if let Some(p) = &self.properties {
            write_child_opt(&mut out, p);
        }
        if let Some(r) = &self.representations {
            write_link_list(&mut out, &r.values);
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{read_child_list, read_child_opt, read_design_list, read_link_list, read_type_list};
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let map_err = |e: String| protocol::ProtocolError::Malformed { what: "kit diff field", offset: 2, detail: e };
        let types = if presence & 0b0000_0001 != 0 { Some(SemioKitTypeList { values: read_type_list(&mut reader).map_err(map_err)? }) } else { None };
        let designs = if presence & 0b0000_0010 != 0 { Some(SemioKitDesignList { values: read_design_list(&mut reader).map_err(map_err)? }) } else { None };
        let objects = if presence & 0b0000_0100 != 0 { Some(SemioKitObjectChildList { values: read_child_list(&mut reader).map_err(map_err)? }) } else { None };
        let models = if presence & 0b0000_1000 != 0 { Some(SemioKitModelChildList { values: read_child_list(&mut reader).map_err(map_err)? }) } else { None };
        let properties = if presence & 0b0001_0000 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
        let representations = if presence & 0b0010_0000 != 0 { Some(SemioKitLinkList { values: read_link_list(&mut reader).map_err(map_err)? }) } else { None };
        Ok(SemioKitDiff { types, designs, objects, models, properties, representations })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioKitDiff` cases — single source of truth for
/// `diff_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioKitDiff> {
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::demo_kit_snapshot;
    vec![
        SemioKitDiff::default(),
        SemioKitDiff { types: Some(SemioKitTypeList { values: demo_kit_snapshot().types }), ..Default::default() },
        SemioKitDiff { properties: Some(None), ..Default::default() },
        SemioKitDiff { representations: Some(SemioKitLinkList { values: demo_kit_snapshot().representations }), ..Default::default() },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::demo_kit_snapshot;
    use protocol::DiffCodec;

    #[semio_framework_async_macros::async_test]
    async fn apply_replaces_touched_fields_only() {
        let base = demo_kit_snapshot();
        let diff = SemioKitDiff { properties: Some(None), ..Default::default() };
        let next = diff.apply(&base).expect("apply must succeed for a well-formed fixture");
        assert!(next.properties.is_none());
        assert_eq!(next.types, base.types, "untouched fields must be preserved");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_last_write_wins_per_field() {
        let mut d1 = SemioKitDiff { properties: Some(None), ..Default::default() };
        let d2 = SemioKitDiff { objects: Some(SemioKitObjectChildList::default()), ..Default::default() };
        d1.absorb(d2.clone());
        assert_eq!(d1.properties, Some(None));
        assert_eq!(d1.objects, Some(SemioKitObjectChildList::default()));
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioKitDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioKitDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🔖️Tests
