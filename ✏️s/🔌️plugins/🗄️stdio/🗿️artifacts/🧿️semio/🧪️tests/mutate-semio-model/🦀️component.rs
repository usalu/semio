//! 🦀️ Semio MODEL exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-model-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️model/🧪️oracle/🔣️component.json`): `stdio.semio.model` is a semio-NATIVE
//! format with no third-party reader or writer, and the two IFC-side candidates (IfcOpenShell on
//! the landed Python host, `ruststep` as a reader) can only reach a `SemioModelSnapshot` through
//! this repository's own IFC bridge, which would compare our importer against our exporter.
//!
//! `oracle` therefore reads the committed specification fixtures literally — no recomputation, no
//! reimplementation of mutation semantics — while `subject` drives this repository's own
//! `apply_semio_model_mutation` over the full eleven-kind `SemioModelMutation` vocabulary. Both
//! roles read the SAME committed bytes through the host's `Context::fixture_json`, so nothing about
//! a fixture is transcribed into either role's source where it could silently drift.
//!
//! The before-state of every vector is the real committed example artifact
//! `🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio`, and
//! `identity-round-trip` reads that artifact and its `.pack.semio` sibling directly, so the claim
//! that the vectors describe the real model is checked rather than asserted.
//!
//! The oracle-only build must never link the subject crate (fleet brief §5.3), so the subject module
//! below carries its own small, forward-only, hand-written structural JSON decoder built on the
//! framework's dependency-free `protocol::Json` — a mechanical field-by-field decode, never a
//! reimplementation of mutation semantics. The subject half is gated behind the generated host's `sut`
//! feature; the Rust SUBJECT phase RUNS. The os-kernel blocker earlier waves recorded here was cleared on
//! 2026-08-24 — `cargo check -p semio-framework-os-kernel --lib` exits 0 and `semio-s-plugin-stdio`
//! builds — so `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-model` really
//! executes every scenario below. The gate keeps the two BUILDS apart; it has never been a reason the
//! subject half goes unmeasured, and for this recorded no-oracle case the subject phase is the only phase
//! that runs at all.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never receives two sides to compare and the `oracle` handlers below are
//! the written statement of the reference answer rather than a second running party. Every law this
//! case claims is therefore asserted INSIDE the subject handler, which fails with both documents
//! printed. A handler that merely ran the mutation and returned would report a pass having checked
//! nothing.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioModelMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-spatial-node", "remove-spatial-node", "set-spatial-node", "insert-element", "remove-element", "set-element", "insert-relation", "remove-relation", "set-relation"];

/// 🗣️ The real committed example artifact, in both of the subset's own envelopes — read by
/// `identity-round-trip`'s subject role, which is the only role that decodes bytes rather than a
/// committed projection, so both constants belong to the `sut` build alone.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio";
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🎒️example.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🗂️ Builds the same `local://` URIs `component.feature` declares, so the fixture-resolution
/// contract has already proved every one of them exists and pinned its digest.
fn before_uri(kind: &str) -> String {
    format!("local://{kind}/⬅️before.json")
}
#[cfg(feature = "sut")]
fn mutation_uri(kind: &str) -> String {
    format!("local://{kind}/🦠️mutation.json")
}
fn after_uri(kind: &str) -> String {
    format!("local://{kind}/➡️after.json")
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally through the host.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let after = ctx.fixture_json(&after_uri(kind))?;
        let bytes = after.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, after))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started, member order included.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let before = ctx.fixture_json(&before_uri(kind))?;
        let bytes = before.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, before))
    }
}

/// 🔮️ The round-trip reference answer: the committed canonical snapshot of the real artifact.
fn identity_oracle(ctx: &Context) -> Result<Outcome, String> {
    let expected = ctx.fixture_json(&before_uri("no-mutation"))?;
    let bytes = expected.to_string().into_bytes();
    Ok(Outcome::with_raw(bytes, expected))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{after_uri, before_uri, mutation_uri, DSL_ASSET, PACK_ASSET};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::mutations::{apply_semio_model_mutation, semio_model_mutation_inverse, SemioModelMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{
        decode_semio_model_pack, encode_semio_model_pack, parse_semio_model_dsl, print_semio_model_dsl, ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode,
    };

    //#region 🔖️JsonReaders
    /// 🧫️ Structural readers over the framework's dependency-free `Json`. Every one of them mirrors
    /// one committed payload's own declared shape; none of them knows anything about mutation
    /// semantics.
    fn field<'a>(json: &'a Json, key: &str) -> &'a Json {
        json.get(key).unwrap_or_else(|| panic!("mutate-semio-model: fixture is missing the field {key:?}"))
    }
    fn number(json: &Json, key: &str) -> f64 {
        match json.get(key) {
            Some(Json::Number(value)) => *value,
            other => panic!("mutate-semio-model: expected a numeric field {key:?}, found {other:?}"),
        }
    }
    fn bool_field(json: &Json, key: &str) -> bool {
        matches!(json.get(key), Some(Json::Bool(true)))
    }
    fn opt_string(json: &Json, key: &str) -> Option<String> {
        match json.get(key) {
            Some(Json::String(value)) => Some(value.clone()),
            _ => None,
        }
    }
    fn present(json: &Json, key: &str) -> bool {
        !matches!(json.get(key), None | Some(Json::Null))
    }
    //#endregion 🔖️JsonReaders

    //#region 🔖️Decode
    fn decode_point3(json: &Json) -> SemioPoint3 {
        SemioPoint3 { x: number(json, "x"), y: number(json, "y"), z: number(json, "z") }
    }
    fn decode_quaternion(json: &Json) -> SemioQuaternion {
        SemioQuaternion { x: number(json, "x"), y: number(json, "y"), z: number(json, "z"), w: number(json, "w") }
    }
    fn decode_transform(json: &Json) -> SemioTransform {
        SemioTransform { translation: decode_point3(field(json, "translation")), rotation: decode_quaternion(field(json, "rotation")), scale: decode_point3(field(json, "scale")) }
    }
    fn decode_spatial_kind(tag: &str) -> SpatialKind {
        match tag {
            "site" => SpatialKind::Site,
            "building" => SpatialKind::Building,
            "storey" => SpatialKind::Storey,
            "space" => SpatialKind::Space,
            other => panic!("mutate-semio-model: unknown spatial kind {other:?}"),
        }
    }
    fn decode_spatial_node(json: &Json) -> SpatialNode {
        SpatialNode { id: json.str("id"), kind: decode_spatial_kind(&json.str("kind")), name: json.str("name"), parent_id: opt_string(json, "parentId"), placement: decode_transform(field(json, "placement")) }
    }
    fn decode_element_class(json: &Json) -> ElementClass {
        match json.str("kind").as_str() {
            "wall" => ElementClass::Wall,
            "slab" => ElementClass::Slab,
            "column" => ElementClass::Column,
            "beam" => ElementClass::Beam,
            "door" => ElementClass::Door,
            "window" => ElementClass::Window,
            "roof" => ElementClass::Roof,
            "stair" => ElementClass::Stair,
            "furniture" => ElementClass::Furniture,
            "other" => ElementClass::Other { name: json.str("name") },
            other => panic!("mutate-semio-model: unknown element class {other:?}"),
        }
    }
    fn decode_geometry_ref(json: &Json) -> GeometryRef {
        match json.str("kind").as_str() {
            "none" => GeometryRef::None,
            "brep" => GeometryRef::Brep { brep_id: json.str("brep_id") },
            "mesh" => GeometryRef::Mesh { mesh_id: json.str("mesh_id") },
            other => panic!("mutate-semio-model: unknown geometry reference {other:?}"),
        }
    }
    fn decode_pset_value(json: &Json) -> PsetValue {
        match json.str("kind").as_str() {
            "text" => PsetValue::Text { value: json.str("value") },
            "number" => PsetValue::Number { value: number(json, "value") },
            "boolean" => PsetValue::Boolean { value: bool_field(json, "value") },
            other => panic!("mutate-semio-model: unknown property-set value {other:?}"),
        }
    }
    fn decode_property(json: &Json) -> Property {
        Property { key: json.str("key"), value: decode_pset_value(field(json, "value")) }
    }
    fn decode_property_set(json: &Json) -> PropertySet {
        PropertySet { name: json.str("name"), properties: json.array("properties").iter().map(decode_property).collect() }
    }
    fn decode_element(json: &Json) -> SemioModelElement {
        SemioModelElement {
            id: json.str("id"),
            class: decode_element_class(field(json, "class")),
            placement: decode_transform(field(json, "placement")),
            geometry: decode_geometry_ref(field(json, "geometry")),
            spatial_id: opt_string(json, "spatialId"),
            psets: json.array("psets").iter().map(decode_property_set).collect(),
        }
    }
    fn decode_relation_kind(json: &Json) -> RelationKind {
        match json.str("kind").as_str() {
            "aggregates" => RelationKind::Aggregates,
            "containedIn" => RelationKind::ContainedIn,
            "connectsTo" => RelationKind::ConnectsTo,
            "fillsVoid" => RelationKind::FillsVoid,
            "voidsElement" => RelationKind::VoidsElement,
            "other" => RelationKind::Other { label: json.str("label") },
            other => panic!("mutate-semio-model: unknown relation kind {other:?}"),
        }
    }
    fn decode_relation(json: &Json) -> ModelRelation {
        ModelRelation { id: json.str("id"), kind: decode_relation_kind(field(json, "kind")), from: json.str("from"), to: json.str("to") }
    }
    fn decode_snapshot(json: &Json) -> SemioModelSnapshot {
        SemioModelSnapshot {
            schema: json.str("schema"),
            spatial: json.array("spatial").iter().map(decode_spatial_node).collect(),
            elements: json.array("elements").iter().map(decode_element).collect(),
            relations: json.array("relations").iter().map(decode_relation).collect(),
        }
    }

    /// 🧫️ `SemioModelMutation` is internally tagged on `mutation` with camelCase VARIANT names,
    /// while its struct-variant FIELDS keep their Rust spelling — a container `rename_all` on an
    /// enum renames variants only. `parent_id`/`spatial_id` are tri-state `Option<Option<String>>`
    /// slots carrying `skip_serializing_if`, so an absent key means "untouched"; the `Some(None)`
    /// "cleared" state has no canonical JSON form and no committed vector expresses it.
    fn decode_mutation(json: &Json) -> SemioModelMutation {
        match json.str("mutation").as_str() {
            "noMutation" => SemioModelMutation::NoMutation,
            "setSnapshot" => SemioModelMutation::SetSnapshot { snapshot: decode_snapshot(field(json, "snapshot")) },
            "insertSpatialNode" => SemioModelMutation::InsertSpatialNode { node: decode_spatial_node(field(json, "node")) },
            "removeSpatialNode" => SemioModelMutation::RemoveSpatialNode { id: json.str("id") },
            "setSpatialNode" => SemioModelMutation::SetSpatialNode {
                id: json.str("id"),
                kind: present(json, "kind").then(|| decode_spatial_kind(&json.str("kind"))),
                name: present(json, "name").then(|| json.str("name")),
                parent_id: json.get("parent_id").map(|_| opt_string(json, "parent_id")),
                placement: present(json, "placement").then(|| decode_transform(field(json, "placement"))),
            },
            "insertElement" => SemioModelMutation::InsertElement { element: decode_element(field(json, "element")) },
            "removeElement" => SemioModelMutation::RemoveElement { id: json.str("id") },
            "setElement" => SemioModelMutation::SetElement {
                id: json.str("id"),
                class: present(json, "class").then(|| decode_element_class(field(json, "class"))),
                placement: present(json, "placement").then(|| decode_transform(field(json, "placement"))),
                geometry: present(json, "geometry").then(|| decode_geometry_ref(field(json, "geometry"))),
                spatial_id: json.get("spatial_id").map(|_| opt_string(json, "spatial_id")),
                psets: present(json, "psets").then(|| json.array("psets").iter().map(decode_property_set).collect()),
            },
            "insertRelation" => SemioModelMutation::InsertRelation { relation: decode_relation(field(json, "relation")) },
            "removeRelation" => SemioModelMutation::RemoveRelation { id: json.str("id") },
            "setRelation" => SemioModelMutation::SetRelation {
                id: json.str("id"),
                kind: present(json, "kind").then(|| decode_relation_kind(field(json, "kind"))),
                from: present(json, "from").then(|| json.str("from")),
                to: present(json, "to").then(|| json.str("to")),
            },
            other => panic!("mutate-semio-model: no decoder for mutation variant {other:?}"),
        }
    }
    //#endregion 🔖️Decode

    //#region 🔖️Projection
    fn object(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }
    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }
    fn optional_text(value: &Option<String>) -> Json {
        match value {
            Some(inner) => Json::String(inner.clone()),
            None => Json::Null,
        }
    }
    fn point3_json(point: &SemioPoint3) -> Json {
        object(vec![("x", Json::Number(point.x)), ("y", Json::Number(point.y)), ("z", Json::Number(point.z))])
    }
    fn transform_json(transform: &SemioTransform) -> Json {
        object(vec![
            ("translation", point3_json(&transform.translation)),
            (
                "rotation",
                object(vec![("x", Json::Number(transform.rotation.x)), ("y", Json::Number(transform.rotation.y)), ("z", Json::Number(transform.rotation.z)), ("w", Json::Number(transform.rotation.w))]),
            ),
            ("scale", point3_json(&transform.scale)),
        ])
    }
    fn spatial_kind_text(kind: SpatialKind) -> &'static str {
        match kind {
            SpatialKind::Site => "site",
            SpatialKind::Building => "building",
            SpatialKind::Storey => "storey",
            SpatialKind::Space => "space",
        }
    }
    fn spatial_node_json(node: &SpatialNode) -> Json {
        object(vec![("id", text(&node.id)), ("kind", text(spatial_kind_text(node.kind))), ("name", text(&node.name)), ("parentId", optional_text(&node.parent_id)), ("placement", transform_json(&node.placement))])
    }
    fn element_class_json(class: &ElementClass) -> Json {
        match class {
            ElementClass::Wall => object(vec![("kind", text("wall"))]),
            ElementClass::Slab => object(vec![("kind", text("slab"))]),
            ElementClass::Column => object(vec![("kind", text("column"))]),
            ElementClass::Beam => object(vec![("kind", text("beam"))]),
            ElementClass::Door => object(vec![("kind", text("door"))]),
            ElementClass::Window => object(vec![("kind", text("window"))]),
            ElementClass::Roof => object(vec![("kind", text("roof"))]),
            ElementClass::Stair => object(vec![("kind", text("stair"))]),
            ElementClass::Furniture => object(vec![("kind", text("furniture"))]),
            ElementClass::Other { name } => object(vec![("kind", text("other")), ("name", text(name))]),
        }
    }
    fn geometry_ref_json(geometry: &GeometryRef) -> Json {
        match geometry {
            GeometryRef::None => object(vec![("kind", text("none"))]),
            GeometryRef::Brep { brep_id } => object(vec![("kind", text("brep")), ("brep_id", text(brep_id))]),
            GeometryRef::Mesh { mesh_id } => object(vec![("kind", text("mesh")), ("mesh_id", text(mesh_id))]),
        }
    }
    fn pset_value_json(value: &PsetValue) -> Json {
        match value {
            PsetValue::Text { value } => object(vec![("kind", text("text")), ("value", text(value))]),
            PsetValue::Number { value } => object(vec![("kind", text("number")), ("value", Json::Number(*value))]),
            PsetValue::Boolean { value } => object(vec![("kind", text("boolean")), ("value", Json::Bool(*value))]),
        }
    }
    fn property_set_json(set: &PropertySet) -> Json {
        object(vec![
            ("name", text(&set.name)),
            ("properties", Json::Array(set.properties.iter().map(|property| object(vec![("key", text(&property.key)), ("value", pset_value_json(&property.value))])).collect())),
        ])
    }
    fn element_json(element: &SemioModelElement) -> Json {
        object(vec![
            ("id", text(&element.id)),
            ("class", element_class_json(&element.class)),
            ("placement", transform_json(&element.placement)),
            ("geometry", geometry_ref_json(&element.geometry)),
            ("spatialId", optional_text(&element.spatial_id)),
            ("psets", Json::Array(element.psets.iter().map(property_set_json).collect())),
        ])
    }
    fn relation_kind_json(kind: &RelationKind) -> Json {
        match kind {
            RelationKind::Aggregates => object(vec![("kind", text("aggregates"))]),
            RelationKind::ContainedIn => object(vec![("kind", text("containedIn"))]),
            RelationKind::ConnectsTo => object(vec![("kind", text("connectsTo"))]),
            RelationKind::FillsVoid => object(vec![("kind", text("fillsVoid"))]),
            RelationKind::VoidsElement => object(vec![("kind", text("voidsElement"))]),
            RelationKind::Other { label } => object(vec![("kind", text("other")), ("label", text(label))]),
        }
    }
    fn relation_json(relation: &ModelRelation) -> Json {
        object(vec![("id", text(&relation.id)), ("kind", relation_kind_json(&relation.kind)), ("from", text(&relation.from)), ("to", text(&relation.to))])
    }

    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field. Collection ORDER is
    /// preserved deliberately — an id-keyed collection whose members append on insert makes order
    /// observable, and the inverse law is only true when it is restored.
    pub fn snapshot_json(snapshot: &SemioModelSnapshot) -> Json {
        object(vec![
            ("schema", text(&snapshot.schema)),
            ("spatial", Json::Array(snapshot.spatial.iter().map(spatial_node_json).collect())),
            ("elements", Json::Array(snapshot.elements.iter().map(element_json).collect())),
            ("relations", Json::Array(snapshot.relations.iter().map(relation_json).collect())),
        ])
    }
    fn outcome_of(snapshot: &SemioModelSnapshot) -> Outcome {
        let projection = snapshot_json(snapshot);
        let bytes = projection.to_string().into_bytes();
        Outcome::with_raw(bytes, projection)
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    fn fixture_for(kind: &str, ctx: &Context) -> Result<(SemioModelSnapshot, SemioModelMutation, SemioModelSnapshot), String> {
        let before = decode_snapshot(&ctx.fixture_json(&before_uri(kind))?);
        let mutation = decode_mutation(&ctx.fixture_json(&mutation_uri(kind))?);
        let after = decode_snapshot(&ctx.fixture_json(&after_uri(kind))?);
        Ok((before, mutation, after))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON the committed
    /// vectors are written in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioModelSnapshot, expected: &SemioModelSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }

    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot — the spatial containment tree, the element list, the relations and the
    /// property sets together, so an edit that reached the right element through the wrong spatial
    /// parent still fails. The assertion lives here rather than in the comparison because a recorded
    /// no-oracle case runs no oracle role: a handler that merely returned `Ok` would report a pass
    /// having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut base, mutation, expected) = fixture_for(kind, ctx)?;
            let outcome = apply_semio_model_mutation(&mut base, &mutation);
            if !semio_mutation_refusals(&outcome).is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", semio_mutation_refusals(&outcome)));
            }
            if base != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &base, &expected));
            }
            Ok(outcome_of(&base))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly, relations to deleted elements included — the
    /// cascade a `delete-element` performs is only provably undone if its own inverse rebuilds them.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation, _expected) = fixture_for(kind, ctx)?;
            let mut current = base.clone();
            let outcome = apply_semio_model_mutation(&mut current, &mutation);
            if !semio_mutation_refusals(&outcome).is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", semio_mutation_refusals(&outcome)));
            }
            for step in &semio_model_mutation_inverse(&mutation, &base) {
                let step_outcome = apply_semio_model_mutation(&mut current, step);
                if !semio_mutation_refusals(&step_outcome).is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", semio_mutation_refusals(&step_outcome)));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            Ok(outcome_of(&current))
        }
    }

    /// 🔁 The real committed artifact, decoded from BOTH of its envelopes and carried back through
    /// each of them. Nothing here transcribes the model: the only channel from the committed bytes
    /// to the projection is the subset's own codecs.
    /// 🔒️ **The byte half of the identity law — asserted, and asserted as `carrier_is_exact`.**
    /// `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin; the two
    /// committed example artifacts this scenario reads were produced by these very codecs, so
    /// reproducing them BYTE FOR BYTE is the correct answer here and `law::reparsed_not_copied`
    /// would be exactly backwards — the same reading `mutate-dag-1` records for `.dag.dsl.semio`
    /// and `mutate-bmp-v3` for its own reference-authored fixture. Saying so in prose alone would
    /// leave the claim an excuse; asserting it makes it checkable, and it fails with the offset of
    /// the first differing byte the moment the printer or the packer drifts. Nor is it a
    /// self-comparison: one side is a file committed to the repository, the other is computed now.
    pub fn identity(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed dsl artifact is not utf-8: {error}"))?;
        let from_text = parse_semio_model_dsl(&text)?;
        let pack_bytes = ctx.fixture_bytes(PACK_ASSET)?;
        let from_pack = decode_semio_model_pack(&pack_bytes)?;
        if from_text != from_pack {
            return Err("identity-round-trip: the committed dsl and pack envelopes decode to different models".to_string());
        }
        let repacked_bytes = encode_semio_model_pack(&from_text);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_model_pack(&repacked_bytes)?;
        let printed = print_semio_model_dsl(&repacked);
        carrier_is_exact(printed.as_bytes(), text.as_bytes())?;
        let reparsed = parse_semio_model_dsl(&printed)?;
        if reparsed != from_text {
            return Err("identity-round-trip: re-encoding through pack and dsl did not preserve the model".to_string());
        }
        Ok(outcome_of(&reparsed))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("identity-round-trip", identity_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity);
    }
    built
}
//#endregion 🔖️Registration
