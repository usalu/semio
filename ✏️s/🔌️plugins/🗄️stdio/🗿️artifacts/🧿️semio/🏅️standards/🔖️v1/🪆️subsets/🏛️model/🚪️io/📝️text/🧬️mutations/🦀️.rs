//! 📝️ Text representation codec surface for `stdio.semio.model` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::model::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets, NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::model::schema::diff::{diff_set_snapshot, ModelRelationDiff, SemioModelDiff, SemioModelElementDiff, SpatialNodeDiff};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{parse_f64};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_relation};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_relation};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_relation_kind};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_relation_kind};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_element};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_element};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_spatial_node};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_spatial_node};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_property_set};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_property_set};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_geometry_ref};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_geometry_ref};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_element_class};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_element_class};
use crate::standards::v1::subsets::model::io::text::snapshot::{dec_spatial_kind};
use crate::standards::v1::subsets::model::io::text::snapshot::{enc_spatial_kind};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_transform};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_transform};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, PropertySet, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioModelMutation` block below
/// calls `Self::parse_op(...)` via trait method syntax, which needs `OpText` in scope in
/// production code too, not merely under `#[cfg(test)]` (same fix `stdio.semio.flow`'s own
/// mutations facet needed).
use protocol::{OpBinary, OpText};
use semio_s_artifact_stdio_contract::deserialize_double_option;

/// 🔢️ `[x,y,z]` in the op text's number spelling.
pub(crate) fn enc_triple(values: &[f64; 3]) -> String {
    format!("[{},{},{}]", values[0], values[1], values[2])
}

/// 🔢️ The inverse of [`enc_triple`].
pub(crate) fn dec_triple(text: &str) -> Result<[f64; 3], String> {
    let parts = split_top_level(strip_brackets(text)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("triple: expected 3 numbers, got {}", parts.len())) };
    Ok([parse_f64(x)?, parse_f64(y)?, parse_f64(z)?])
}

/// 🎙️ P2 pilot (model): hand-rolled `OpText`/`OpBinary` real structured codecs — replacing the old
/// plain-`serde_json` passthrough. Grammar: `keyword arg=value ...` (space-separated), reusing
/// `schema::diff`'s `pub(crate)` grammar primitives — same convention `stdio.semio.flow`'s own
/// mutations facet uses. Deliberately NOT `#[derive(dsl::DslOps)]` + `#[dsl(block)]` — that path
/// requires every nested type in the mutation's field tree to itself implement `dsl::DslField` (via
/// `dsl::DslRecord`), a repo-wide framework capability this hand-rolled vocabulary does not depend
/// on (f6-final-summary.md §4: generics/tuple/nested-array derive gaps).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_model_snapshot(s: &SemioModelSnapshot) -> String {
    format!(
        "[{},{},{},{}]",
        enc_str(&s.schema),
        format_args!("[{}]", s.spatial.iter().map(enc_spatial_node).collect::<Vec<_>>().join(",")),
        format_args!("[{}]", s.elements.iter().map(enc_element).collect::<Vec<_>>().join(",")),
        format_args!("[{}]", s.relations.iter().map(enc_relation).collect::<Vec<_>>().join(",")),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_model_snapshot(s: &str) -> Result<SemioModelSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, spatial, elements, relations] = parts.as_slice() else { return Err(format!("snapshot: expected 4 fields, got {}", parts.len())) };
    let spatial = split_top_level(strip_brackets(spatial)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_spatial_node).collect::<Result<Vec<_>, String>>()?;
    let elements = split_top_level(strip_brackets(elements)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_element).collect::<Result<Vec<_>, String>>()?;
    let relations = split_top_level(strip_brackets(relations)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_relation).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioModelSnapshot { schema: dec_str(schema)?, spatial, elements, relations })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_model_mutation(m: &SemioModelMutation) -> String {
    match m {
        SemioModelMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_semio_model_snapshot(snapshot)),
        SemioModelMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node }) => format!("insert-spatial-node node={}", enc_spatial_node(node)),
        SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id }) => format!("remove-spatial-node id={}", enc_str(id)),
        SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode { id, kind, name, parent_id, placement }) => format!(
            "set-spatial-node id={} kind={} name={} parent_id={} placement={}",
            enc_str(id),
            encode_option(kind, |v: &SpatialKind| enc_spatial_kind(v).to_string()),
            encode_option(name, |v: &String| enc_str(v)),
            encode_option(parent_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
            encode_option(placement, enc_transform),
        ),
        SemioModelMutation::InsertElement(insert_element::InsertElement { element }) => format!("insert-element element={}", enc_element(element)),
        SemioModelMutation::RemoveElement(remove_element::RemoveElement { id }) => format!("remove-element id={}", enc_str(id)),
        SemioModelMutation::SetElement(set_element::SetElement { id, class, placement, geometry, spatial_id, psets }) => format!(
            "set-element id={} class={} placement={} geometry={} spatial_id={} psets={}",
            enc_str(id),
            encode_option(class, enc_element_class),
            encode_option(placement, enc_transform),
            encode_option(geometry, enc_geometry_ref),
            encode_option(spatial_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
            encode_option(psets, |v: &Vec<PropertySet>| enc_list(v, enc_property_set)),
        ),
        SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation }) => format!("insert-relation relation={}", enc_relation(relation)),
        SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id }) => format!("remove-relation id={}", enc_str(id)),
        SemioModelMutation::SetRelation(set_relation::SetRelation { id, kind, from, to }) => {
            format!("set-relation id={} kind={} from={} to={}", enc_str(id), encode_option(kind, enc_relation_kind), encode_option(from, |v: &String| enc_str(v)), encode_option(to, |v: &String| enc_str(v)),)
        }
        SemioModelMutation::DragElements(drag_elements::DragElements { targets, offset }) => format!("drag-elements targets={} offset={}", enc_list(targets, |v: &String| enc_str(v)), enc_triple(offset)),
        SemioModelMutation::RotateElements(rotate_elements::RotateElements { targets, axis, angle }) => format!("rotate-elements targets={} axis={} angle={angle}", enc_list(targets, |v: &String| enc_str(v)), enc_triple(axis)),
        SemioModelMutation::ScaleElements(scale_elements::ScaleElements { targets, factors }) => format!("scale-elements targets={} factors={}", enc_list(targets, |v: &String| enc_str(v)), enc_triple(factors)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_model_mutation(line: &str) -> Result<SemioModelMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioModelMutation::PatchSnapshot(crate::standards::v1::subsets::model::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("model mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("model mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioModelMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_semio_model_snapshot(arg("snapshot")?)? })),
        "insert-spatial-node" => Ok(SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: dec_spatial_node(arg("node")?)? })),
        "remove-spatial-node" => Ok(SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: dec_str(arg("id")?)? })),
        "set-spatial-node" => Ok(SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode {
            id: dec_str(arg("id")?)?,
            kind: decode_option(arg("kind")?, dec_spatial_kind)?,
            name: decode_option(arg("name")?, dec_str)?,
            parent_id: decode_option(arg("parent_id")?, |s| decode_option(s, dec_str))?,
            placement: decode_option(arg("placement")?, dec_transform)?,
        })),
        "insert-element" => Ok(SemioModelMutation::InsertElement(insert_element::InsertElement { element: dec_element(arg("element")?)? })),
        "remove-element" => Ok(SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: dec_str(arg("id")?)? })),
        "set-element" => Ok(SemioModelMutation::SetElement(set_element::SetElement {
            id: dec_str(arg("id")?)?,
            class: decode_option(arg("class")?, dec_element_class)?,
            placement: decode_option(arg("placement")?, dec_transform)?,
            geometry: decode_option(arg("geometry")?, dec_geometry_ref)?,
            spatial_id: decode_option(arg("spatial_id")?, |s| decode_option(s, dec_str))?,
            psets: decode_option(arg("psets")?, |s| dec_list(s, dec_property_set))?,
        })),
        "insert-relation" => Ok(SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: dec_relation(arg("relation")?)? })),
        "remove-relation" => Ok(SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: dec_str(arg("id")?)? })),
        "set-relation" => {
            Ok(SemioModelMutation::SetRelation(set_relation::SetRelation { id: dec_str(arg("id")?)?, kind: decode_option(arg("kind")?, dec_relation_kind)?, from: decode_option(arg("from")?, dec_str)?, to: decode_option(arg("to")?, dec_str)? }))
        }
        "drag-elements" => Ok(SemioModelMutation::DragElements(drag_elements::DragElements { targets: dec_list(arg("targets")?, dec_str)?, offset: dec_triple(arg("offset")?)? })),
        "rotate-elements" => Ok(SemioModelMutation::RotateElements(rotate_elements::RotateElements { targets: dec_list(arg("targets")?, dec_str)?, axis: dec_triple(arg("axis")?)?, angle: parse_f64(arg("angle")?)? })),
        "scale-elements" => Ok(SemioModelMutation::ScaleElements(scale_elements::ScaleElements { targets: dec_list(arg("targets")?, dec_str)?, factors: dec_triple(arg("factors")?)? })),
        other => Err(format!("model mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioModelMutation {
    fn print_op(&self) -> String {
        print_semio_model_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_semio_model_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
