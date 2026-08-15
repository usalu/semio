//! 🧬️ SemioModelMutation — named-variant vocabulary (gif 89a / docx precedent): a sparse
//! `SetSnapshot` plus insert/remove/set per collection (spatial/elements/relations). Every
//! variant's `diff()`/`inverse()` is HAND-WRITTEN below (schema-design.md: apply-and-capture via
//! clone+apply+re-diff is banned -- each variant constructs its `SemioModelDiff` directly).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::{
    dec_element, dec_element_class, dec_geometry_ref, dec_list, dec_property_set, dec_relation, dec_relation_kind, dec_spatial_kind, dec_spatial_node, dec_str, dec_transform, decode_option, diff_set_snapshot, enc_element, enc_element_class,
    enc_geometry_ref, enc_list, enc_property_set, enc_relation, enc_relation_kind, enc_spatial_kind, enc_spatial_node, enc_str, enc_transform, encode_option, ModelRelationDiff, SemioModelDiff, SemioModelElementDiff, SpatialNodeDiff,
};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, PropertySet, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioModelMutation` block below
/// calls `Self::parse_op(...)` via trait method syntax, which needs `OpText` in scope in
/// production code too, not merely under `#[cfg(test)]` (same fix `stdio.semio.flow`'s own
/// mutations facet needed).
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️DoubleOption
/// 🕳️ Standard double-`Option` serde workaround: with plain `#[serde(default)]`, a field typed
/// `Option<Option<T>>` can't distinguish "untouched" (key absent) from "cleared" (`Some(None)`,
/// key present with JSON `null`) — both collapse to the outer `None` on decode, because serde's
/// blanket `Deserialize for Option<T>` treats `null` as absence at ANY nesting depth. Combined with
/// `skip_serializing_if = "Option::is_none"` (so "untouched" omits the key entirely), this makes
/// key-PRESENT-with-`null` unambiguously mean `Some(None)`.
fn deserialize_double_option<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}
//#endregion 🔖️DoubleOption

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioModelMutation {
    #[default]
    NoMutation,
    /// 🧩 Sparse full-state replace -- `diff()` is `SemioModelDiff::between`, never a
    /// `snapshot: Option<Snapshot>` full-replace slot (schema-design.md).
    SetSnapshot {
        snapshot: SemioModelSnapshot,
    },
    InsertSpatialNode {
        node: SpatialNode,
    },
    RemoveSpatialNode {
        id: String,
    },
    SetSpatialNode {
        id: String,
        #[serde(default)]
        kind: Option<SpatialKind>,
        #[serde(default)]
        name: Option<String>,
        /// 🕳️ Tri-state (`None` = untouched, `Some(None)` = cleared, `Some(Some(_))` = set) — the
        /// classic double-`Option` serde footgun: without `skip_serializing_if`/`deserialize_with`,
        /// `Some(None)` and the untouched `None` both serialize to JSON `null` and collapse back to
        /// the SAME outer `None` on decode (`w2a-verify-report.md`'s model finding, same shape as
        /// `spatial_id` below).
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
        parent_id: Option<Option<String>>,
        #[serde(default)]
        placement: Option<SemioTransform>,
    },
    InsertElement {
        element: SemioModelElement,
    },
    RemoveElement {
        id: String,
    },
    SetElement {
        id: String,
        #[serde(default)]
        class: Option<ElementClass>,
        #[serde(default)]
        placement: Option<SemioTransform>,
        #[serde(default)]
        geometry: Option<GeometryRef>,
        /// 🕳️ Tri-state, same double-`Option` footgun/fix as `SetSpatialNode.parent_id` above —
        /// confirmed live by the verifier: `Some(None)` round-tripped to the outer `None` through
        /// `print_op`/`parse_op` before this fix.
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
        spatial_id: Option<Option<String>>,
        #[serde(default)]
        psets: Option<Vec<PropertySet>>,
    },
    InsertRelation {
        relation: ModelRelation,
    },
    RemoveRelation {
        id: String,
    },
    SetRelation {
        id: String,
        #[serde(default)]
        kind: Option<RelationKind>,
        #[serde(default)]
        from: Option<String>,
        #[serde(default)]
        to: Option<String>,
    },
}

impl Mutation<SemioModelSnapshot> for SemioModelMutation {
    type Diff = SemioModelDiff;

    fn diff(&self, base: &SemioModelSnapshot) -> Self::Diff {
        match self {
            SemioModelMutation::NoMutation => SemioModelDiff::default(),
            SemioModelMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SemioModelMutation::InsertSpatialNode { node } => SemioModelDiff { spatial: Some(NamedTripleDiff { added: vec![node.clone()], ..Default::default() }), ..Default::default() },
            SemioModelMutation::RemoveSpatialNode { id } => SemioModelDiff { spatial: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() },
            SemioModelMutation::SetSpatialNode { id, kind, name, parent_id, placement } => SemioModelDiff {
                spatial: Some(NamedTripleDiff { modified: vec![NamedModified { key: id.clone(), diff: SpatialNodeDiff { kind: *kind, name: name.clone(), parent_id: parent_id.clone(), placement: *placement } }], ..Default::default() }),
                ..Default::default()
            },
            SemioModelMutation::InsertElement { element } => SemioModelDiff { elements: Some(NamedTripleDiff { added: vec![element.clone()], ..Default::default() }), ..Default::default() },
            SemioModelMutation::RemoveElement { id } => SemioModelDiff { elements: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() },
            SemioModelMutation::SetElement { id, class, placement, geometry, spatial_id, psets } => SemioModelDiff {
                elements: Some(NamedTripleDiff {
                    modified: vec![NamedModified { key: id.clone(), diff: SemioModelElementDiff { class: class.clone(), placement: *placement, geometry: geometry.clone(), spatial_id: spatial_id.clone(), psets: psets.clone() } }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            SemioModelMutation::InsertRelation { relation } => SemioModelDiff { relations: Some(NamedTripleDiff { added: vec![relation.clone()], ..Default::default() }), ..Default::default() },
            SemioModelMutation::RemoveRelation { id } => SemioModelDiff { relations: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() },
            SemioModelMutation::SetRelation { id, kind, from, to } => {
                SemioModelDiff { relations: Some(NamedTripleDiff { modified: vec![NamedModified { key: id.clone(), diff: ModelRelationDiff { kind: kind.clone(), from: from.clone(), to: to.clone() } }], ..Default::default() }), ..Default::default() }
            }
        }
    }

    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<Self> {
        match self {
            SemioModelMutation::NoMutation => vec![SemioModelMutation::NoMutation],
            SemioModelMutation::SetSnapshot { .. } => vec![SemioModelMutation::SetSnapshot { snapshot: base.clone() }],

            SemioModelMutation::InsertSpatialNode { node } => vec![SemioModelMutation::RemoveSpatialNode { id: node.id.clone() }],
            SemioModelMutation::RemoveSpatialNode { id } => match base.spatial.iter().find(|n| &n.id == id) {
                Some(original) => vec![SemioModelMutation::InsertSpatialNode { node: original.clone() }],
                None => vec![SemioModelMutation::NoMutation],
            },
            SemioModelMutation::SetSpatialNode { id, kind, name, parent_id, placement } => match base.spatial.iter().find(|n| &n.id == id) {
                Some(original) => vec![SemioModelMutation::SetSpatialNode {
                    id: id.clone(),
                    kind: kind.as_ref().map(|_| original.kind),
                    name: name.as_ref().map(|_| original.name.clone()),
                    parent_id: parent_id.as_ref().map(|_| original.parent_id.clone()),
                    placement: placement.as_ref().map(|_| original.placement),
                }],
                None => vec![SemioModelMutation::NoMutation],
            },

            SemioModelMutation::InsertElement { element } => vec![SemioModelMutation::RemoveElement { id: element.id.clone() }],
            SemioModelMutation::RemoveElement { id } => match base.elements.iter().find(|e| &e.id == id) {
                Some(original) => vec![SemioModelMutation::InsertElement { element: original.clone() }],
                None => vec![SemioModelMutation::NoMutation],
            },
            SemioModelMutation::SetElement { id, class, placement, geometry, spatial_id, psets } => match base.elements.iter().find(|e| &e.id == id) {
                Some(original) => vec![SemioModelMutation::SetElement {
                    id: id.clone(),
                    class: class.as_ref().map(|_| original.class.clone()),
                    placement: placement.as_ref().map(|_| original.placement),
                    geometry: geometry.as_ref().map(|_| original.geometry.clone()),
                    spatial_id: spatial_id.as_ref().map(|_| original.spatial_id.clone()),
                    psets: psets.as_ref().map(|_| original.psets.clone()),
                }],
                None => vec![SemioModelMutation::NoMutation],
            },

            SemioModelMutation::InsertRelation { relation } => vec![SemioModelMutation::RemoveRelation { id: relation.id.clone() }],
            SemioModelMutation::RemoveRelation { id } => match base.relations.iter().find(|r| &r.id == id) {
                Some(original) => vec![SemioModelMutation::InsertRelation { relation: original.clone() }],
                None => vec![SemioModelMutation::NoMutation],
            },
            SemioModelMutation::SetRelation { id, kind, from, to } => match base.relations.iter().find(|r| &r.id == id) {
                Some(original) => vec![SemioModelMutation::SetRelation { id: id.clone(), kind: kind.as_ref().map(|_| original.kind.clone()), from: from.as_ref().map(|_| original.from.clone()), to: to.as_ref().map(|_| original.to.clone()) }],
                None => vec![SemioModelMutation::NoMutation],
            },
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_model_mutation(snapshot: &mut SemioModelSnapshot, mutation: &SemioModelMutation) -> SemioModelDiff {
    let diff = <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Mutation

//#region 🔖️OpCodecs
/// 🎙️ P2 pilot (model): hand-rolled `OpText`/`OpBinary` real structured codecs — replacing the old
/// plain-`serde_json` passthrough. Grammar: `keyword arg=value ...` (space-separated), reusing
/// `schema::diff`'s `pub(crate)` grammar primitives — same convention `stdio.semio.flow`'s own
/// mutations facet uses. Deliberately NOT `#[derive(dsl::DslOps)]` + `#[dsl(block)]` — that path
/// requires every nested type in the mutation's field tree to itself implement `dsl::DslField` (via
/// `dsl::DslRecord`), a repo-wide framework capability this hand-rolled vocabulary does not depend
/// on (f6-final-summary.md §4: generics/tuple/nested-array derive gaps).
fn enc_semio_model_snapshot(s: &SemioModelSnapshot) -> String {
    format!(
        "[{},{},{},{}]",
        enc_str(&s.schema),
        format!("[{}]", s.spatial.iter().map(enc_spatial_node).collect::<Vec<_>>().join(",")),
        format!("[{}]", s.elements.iter().map(enc_element).collect::<Vec<_>>().join(",")),
        format!("[{}]", s.relations.iter().map(enc_relation).collect::<Vec<_>>().join(",")),
    )
}
fn dec_semio_model_snapshot(s: &str) -> Result<SemioModelSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, spatial, elements, relations] = parts.as_slice() else { return Err(format!("snapshot: expected 4 fields, got {}", parts.len())) };
    let spatial = split_top_level(strip_brackets(spatial)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_spatial_node).collect::<Result<Vec<_>, String>>()?;
    let elements = split_top_level(strip_brackets(elements)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_element).collect::<Result<Vec<_>, String>>()?;
    let relations = split_top_level(strip_brackets(relations)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_relation).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioModelSnapshot { schema: dec_str(schema)?, spatial, elements, relations })
}

fn print_semio_model_mutation(m: &SemioModelMutation) -> String {
    match m {
        SemioModelMutation::NoMutation => "no-mutation".to_string(),
        SemioModelMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_semio_model_snapshot(snapshot)),
        SemioModelMutation::InsertSpatialNode { node } => format!("insert-spatial-node node={}", enc_spatial_node(node)),
        SemioModelMutation::RemoveSpatialNode { id } => format!("remove-spatial-node id={}", enc_str(id)),
        SemioModelMutation::SetSpatialNode { id, kind, name, parent_id, placement } => format!(
            "set-spatial-node id={} kind={} name={} parent_id={} placement={}",
            enc_str(id),
            encode_option(kind, |v: &SpatialKind| enc_spatial_kind(v).to_string()),
            encode_option(name, |v: &String| enc_str(v)),
            encode_option(parent_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
            encode_option(placement, enc_transform),
        ),
        SemioModelMutation::InsertElement { element } => format!("insert-element element={}", enc_element(element)),
        SemioModelMutation::RemoveElement { id } => format!("remove-element id={}", enc_str(id)),
        SemioModelMutation::SetElement { id, class, placement, geometry, spatial_id, psets } => format!(
            "set-element id={} class={} placement={} geometry={} spatial_id={} psets={}",
            enc_str(id),
            encode_option(class, enc_element_class),
            encode_option(placement, enc_transform),
            encode_option(geometry, enc_geometry_ref),
            encode_option(spatial_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
            encode_option(psets, |v: &Vec<PropertySet>| enc_list(v, enc_property_set)),
        ),
        SemioModelMutation::InsertRelation { relation } => format!("insert-relation relation={}", enc_relation(relation)),
        SemioModelMutation::RemoveRelation { id } => format!("remove-relation id={}", enc_str(id)),
        SemioModelMutation::SetRelation { id, kind, from, to } => {
            format!("set-relation id={} kind={} from={} to={}", enc_str(id), encode_option(kind, enc_relation_kind), encode_option(from, |v: &String| enc_str(v)), encode_option(to, |v: &String| enc_str(v)),)
        }
    }
}
fn parse_semio_model_mutation(line: &str) -> Result<SemioModelMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioModelMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("model mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("model mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioModelMutation::SetSnapshot { snapshot: dec_semio_model_snapshot(arg("snapshot")?)? }),
        "insert-spatial-node" => Ok(SemioModelMutation::InsertSpatialNode { node: dec_spatial_node(arg("node")?)? }),
        "remove-spatial-node" => Ok(SemioModelMutation::RemoveSpatialNode { id: dec_str(arg("id")?)? }),
        "set-spatial-node" => Ok(SemioModelMutation::SetSpatialNode {
            id: dec_str(arg("id")?)?,
            kind: decode_option(arg("kind")?, dec_spatial_kind)?,
            name: decode_option(arg("name")?, dec_str)?,
            parent_id: decode_option(arg("parent_id")?, |s| decode_option(s, dec_str))?,
            placement: decode_option(arg("placement")?, dec_transform)?,
        }),
        "insert-element" => Ok(SemioModelMutation::InsertElement { element: dec_element(arg("element")?)? }),
        "remove-element" => Ok(SemioModelMutation::RemoveElement { id: dec_str(arg("id")?)? }),
        "set-element" => Ok(SemioModelMutation::SetElement {
            id: dec_str(arg("id")?)?,
            class: decode_option(arg("class")?, dec_element_class)?,
            placement: decode_option(arg("placement")?, dec_transform)?,
            geometry: decode_option(arg("geometry")?, dec_geometry_ref)?,
            spatial_id: decode_option(arg("spatial_id")?, |s| decode_option(s, dec_str))?,
            psets: decode_option(arg("psets")?, |s| dec_list(s, dec_property_set))?,
        }),
        "insert-relation" => Ok(SemioModelMutation::InsertRelation { relation: dec_relation(arg("relation")?)? }),
        "remove-relation" => Ok(SemioModelMutation::RemoveRelation { id: dec_str(arg("id")?)? }),
        "set-relation" => Ok(SemioModelMutation::SetRelation { id: dec_str(arg("id")?)?, kind: decode_option(arg("kind")?, dec_relation_kind)?, from: decode_option(arg("from")?, dec_str)?, to: decode_option(arg("to")?, dec_str)? }),
        other => Err(format!("model mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioModelMutation {
    fn print_op(&self) -> String {
        print_semio_model_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_semio_model_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🏷️ Ordinal table, same declaration order as `SemioModelMutation`'s own enum variants and
/// `parse_semio_model_mutation`'s keyword match — the real binary `tag` field's source of truth.
const OP_KEYWORDS: [&str; 11] = ["no-mutation", "set-snapshot", "insert-spatial-node", "remove-spatial-node", "set-spatial-node", "insert-element", "remove-element", "set-element", "insert-relation", "remove-relation", "set-relation"];
fn variant_ordinal(m: &SemioModelMutation) -> u8 {
    match m {
        SemioModelMutation::NoMutation => 0,
        SemioModelMutation::SetSnapshot { .. } => 1,
        SemioModelMutation::InsertSpatialNode { .. } => 2,
        SemioModelMutation::RemoveSpatialNode { .. } => 3,
        SemioModelMutation::SetSpatialNode { .. } => 4,
        SemioModelMutation::InsertElement { .. } => 5,
        SemioModelMutation::RemoveElement { .. } => 6,
        SemioModelMutation::SetElement { .. } => 7,
        SemioModelMutation::InsertRelation { .. } => 8,
        SemioModelMutation::RemoveRelation { .. } => 9,
        SemioModelMutation::SetRelation { .. } => 10,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_semio_model_mutation` (empty for
/// `no-mutation`) — the binary frame's `tag` byte already carries the keyword, so the text keyword
/// itself is redundant in the binary payload.
fn print_semio_model_mutation_args(m: &SemioModelMutation) -> String {
    match print_semio_model_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ P2 pilot (model): real binary op frame, replacing the old `serde_json::to_vec`/`from_slice`
/// shortcut. `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`OP_KEYWORDS`]) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_semio_model_mutation`/`parse_semio_model_mutation` text codec rather than re-deriving a
/// second independent encoding.
impl OpBinary for SemioModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_semio_model_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion 🔖️OpCodecs

//#region 🔖️Demo
/// 🌱 Shared fixture helpers + representative `SemioModelMutation` cases (one per variant) —
/// single source of truth for this facet's own tests AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn sample_transform() -> SemioTransform {
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};
    SemioTransform { translation: SemioPoint3 { x: 5.0, y: 6.0, z: 7.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}

#[cfg(test)]
pub(crate) fn fixture() -> SemioModelSnapshot {
    let mut snap = SemioModelSnapshot::default();
    snap.spatial.push(SpatialNode { id: "s1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: SemioTransform::identity() });
    snap.elements.push(SemioModelElement { id: "e1".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
    snap.relations.push(ModelRelation { id: "r1".into(), kind: RelationKind::Aggregates, from: "e1".into(), to: "s1".into() });
    snap
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioModelMutation> {
    let base = fixture();
    vec![
        SemioModelMutation::NoMutation,
        SemioModelMutation::SetSnapshot { snapshot: base.clone() },
        SemioModelMutation::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Space, name: "Room".into(), parent_id: None, placement: SemioTransform::identity() } },
        SemioModelMutation::RemoveSpatialNode { id: "s1".into() },
        SemioModelMutation::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: None, parent_id: Some(Some("root".into())), placement: None },
        SemioModelMutation::InsertElement { element: SemioModelElement { id: "e2".into(), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] } },
        SemioModelMutation::RemoveElement { id: "e1".into() },
        SemioModelMutation::SetElement { id: "e1".into(), class: None, placement: None, geometry: Some(GeometryRef::None), spatial_id: Some(None), psets: None },
        SemioModelMutation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::Other { label: "custom".into() }, from: "e1".into(), to: "s1".into() } },
        SemioModelMutation::RemoveRelation { id: "r1".into() },
        SemioModelMutation::SetRelation { id: "r1".into(), kind: Some(RelationKind::ConnectsTo), from: None, to: None },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ mutation_diff_law + inverse_law, exercised for every non-trivial variant: `mutation.diff(base)`
    /// must equal what `apply_semio_model_mutation` actually applied, and applying the mutation's
    /// own `inverse()` must restore `base` exactly.
    fn assert_round_trips(base: &SemioModelSnapshot, mutation: SemioModelMutation) {
        let diff = <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(&mutation, base);
        let mut applied = base.clone();
        let produced = apply_semio_model_mutation(&mut applied, &mutation);
        assert_eq!(produced, diff, "diff() must match what apply_semio_model_mutation actually applied for {mutation:?}");
        let expected = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, base);
        assert_eq!(applied, expected, "applying the mutation must equal applying its own diff for {mutation:?}");

        let inv = <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(&mutation, base);
        let mut restored = applied.clone();
        for m in &inv {
            let _ = apply_semio_model_mutation(&mut restored, m);
        }
        assert_eq!(&restored, base, "inverse must restore the original base for {mutation:?}");
    }

    #[test]
    fn mutation_diff_law_and_inverse_law_cover_every_collection() {
        let base = fixture();

        assert_round_trips(&base, SemioModelMutation::NoMutation);

        let mut swapped = base.clone();
        swapped.elements[0].class = ElementClass::Slab;
        assert_round_trips(&base, SemioModelMutation::SetSnapshot { snapshot: swapped });

        assert_round_trips(&base, SemioModelMutation::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Building, name: "Bldg".into(), parent_id: Some("s1".into()), placement: sample_transform() } });
        assert_round_trips(&base, SemioModelMutation::RemoveSpatialNode { id: "s1".into() });
        assert_round_trips(&base, SemioModelMutation::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: Some("Renamed".into()), parent_id: Some(None), placement: Some(sample_transform()) });

        assert_round_trips(
            &base,
            SemioModelMutation::InsertElement {
                element: SemioModelElement { id: "e2".into(), class: ElementClass::Door, placement: sample_transform(), geometry: GeometryRef::Mesh { mesh_id: "m1".into() }, spatial_id: Some("s1".into()), psets: vec![] },
            },
        );
        assert_round_trips(&base, SemioModelMutation::RemoveElement { id: "e1".into() });
        assert_round_trips(
            &base,
            SemioModelMutation::SetElement {
                id: "e1".into(),
                class: Some(ElementClass::Column),
                placement: Some(sample_transform()),
                geometry: Some(GeometryRef::Brep { brep_id: "b1".into() }),
                spatial_id: Some(Some("s1".into())),
                psets: Some(vec![]),
            },
        );

        assert_round_trips(&base, SemioModelMutation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::VoidsElement, from: "e1".into(), to: "s1".into() } });
        assert_round_trips(&base, SemioModelMutation::RemoveRelation { id: "r1".into() });
        assert_round_trips(&base, SemioModelMutation::SetRelation { id: "r1".into(), kind: Some(RelationKind::FillsVoid), from: Some("e1".into()), to: Some("s1".into()) });
    }

    /// 🧪️ op_text_binary_roundtrip_law: real hand-rolled `OpText`/`OpBinary` round trip, one
    /// instance of every variant (`demo_mutation_cases()` — single source of truth also shared with
    /// the composer's `ops_grammar_conformance_law`/`protocol_walk_law`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioModelMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioModelMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
