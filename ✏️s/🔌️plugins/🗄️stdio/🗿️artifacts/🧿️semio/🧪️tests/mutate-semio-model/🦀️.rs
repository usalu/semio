//! 🦀️ Semio MODEL exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-model-mutate` is the
//! registered oracle `semio-model-python-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️model/
//! 🧪️oracle/🔣️.json`) — an independent Python implementation of the semio model carrier and
//! its eleven verbs, written from the committed grammar, protocol and specification vectors, living
//! beside this file as `🐍️component.py`. The runner dispatches the oracle role to that adapter and
//! the subject role here, and compares the two projections under `@comparison-ordered-json-v1`.
//! Registering oracle handlers here as well would put this repository's own answer on both sides of
//! that comparison, which is the precise failure the platform exists to prevent.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! capsule tower, `spec-vector-<kind>` requires the applied snapshot to be the committed
//! after-snapshot AND the undone one to be the before-snapshot, and `identity-round-trip` requires
//! all four committed encodings to be reproduced byte for byte through `law::carrier_is_exact`.
//!
//! **How the fixtures reach typed values.** The generated test host links only `semio-repo-test-host`
//! and, behind `sut`, this subset's own crate — no `serde`, no `serde_json`, and this subset exports
//! no JSON bridge for its snapshot or its mutation — so the subject module below carries its own
//! small, forward-only, structural decoder over the framework's dependency-free `Json`. It decodes
//! JSON STRUCTURE only, field by field, mirroring each committed payload's own declared shape; it
//! never invents or reimplements any mutation SEMANTICS, which still run through the real
//! `apply_semio_model_mutation`/`semio_model_mutation_inverse`. Every input is read from a fixture
//! the FEATURE declares — the mutation parameters from the scenario's doc string, the specification
//! vectors from the `local://` URIs its data table names — so neither adapter holds a transcription
//! that could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioModelMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-spatial-node",
    "remove-spatial-node",
    "set-spatial-node",
    "insert-element",
    "remove-element",
    "set-element",
    "insert-relation",
    "remove-relation",
    "set-relation",
];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::mutations::{
        apply_semio_model_mutation, insert_element, insert_relation, insert_spatial_node, remove_element, remove_relation, remove_spatial_node, semio_model_mutation_inverse, set_element, set_relation, set_snapshot, set_spatial_node, SemioModelMutation,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{
        decode_semio_model_pack, encode_semio_model_pack, parse_semio_model_dsl, print_semio_model_dsl, ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode,
    };
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

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
    /// while each leaf payload's FIELDS keep their Rust spelling — a container `rename_all` on an
    /// enum renames variants only. `parent_id`/`spatial_id` are tri-state `Option<Option<String>>`
    /// slots carrying `skip_serializing_if`, so an absent key means "untouched"; the `Some(None)`
    /// "cleared" state has no canonical JSON form and no committed vector expresses it.
    ///
    /// 🧭️ `"noMutation"` is the dropped `NoMutation` verb's committed spelling (`no` is not an
    /// APPROVED_VERB, so the leaf migration could not keep it as a variant) — it maps to the
    /// identity mutation `SetSnapshot(base.clone())` rather than failing, so the committed
    /// `no-mutation` scenario keeps exercising the "nothing changes" law instead of being deleted.
    fn decode_mutation(json: &Json, base: &SemioModelSnapshot) -> SemioModelMutation {
        match json.str("mutation").as_str() {
            "noMutation" => SemioModelMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            "setSnapshot" => SemioModelMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: decode_snapshot(field(json, "snapshot")) }),
            "insertSpatialNode" => SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: decode_spatial_node(field(json, "node")) }),
            "removeSpatialNode" => SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: json.str("id") }),
            "setSpatialNode" => SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode {
                id: json.str("id"),
                kind: present(json, "kind").then(|| decode_spatial_kind(&json.str("kind"))),
                name: present(json, "name").then(|| json.str("name")),
                parent_id: json.get("parent_id").map(|_| opt_string(json, "parent_id")),
                placement: present(json, "placement").then(|| decode_transform(field(json, "placement"))),
            }),
            "insertElement" => SemioModelMutation::InsertElement(insert_element::InsertElement { element: decode_element(field(json, "element")) }),
            "removeElement" => SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: json.str("id") }),
            "setElement" => SemioModelMutation::SetElement(set_element::SetElement {
                id: json.str("id"),
                class: present(json, "class").then(|| decode_element_class(field(json, "class"))),
                placement: present(json, "placement").then(|| decode_transform(field(json, "placement"))),
                geometry: present(json, "geometry").then(|| decode_geometry_ref(field(json, "geometry"))),
                spatial_id: json.get("spatial_id").map(|_| opt_string(json, "spatial_id")),
                psets: present(json, "psets").then(|| json.array("psets").iter().map(decode_property_set).collect()),
            }),
            "insertRelation" => SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: decode_relation(field(json, "relation")) }),
            "removeRelation" => SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: json.str("id") }),
            "setRelation" => SemioModelMutation::SetRelation(set_relation::SetRelation {
                id: json.str("id"),
                kind: present(json, "kind").then(|| decode_relation_kind(field(json, "kind"))),
                from: present(json, "from").then(|| json.str("from")),
                to: present(json, "to").then(|| json.str("to")),
            }),
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
    //#endregion 🔖️Projection

    //#region 🔖️Input
    /// 🏢️ The two-node demo building, in both encodings the domain commits for it — small, but the
    /// only `stdio.semio.model` bytes in this artifact a codec other than the Python one wrote.
    const BUILDING_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio";
    const BUILDING_PACK: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🎒️example.pack.semio";
    /// 🏗️ The real 3-node, 181-element, 362-relation capsule tower and its binary twin, derived once
    /// from the committed Nakagin Capsule Tower IFC with IfcOpenShell.
    const TOWER_DSL: &str = "local://🏗️nakagin-capsule-tower.dsl.semio";
    const TOWER_PACK: &str = "local://🏗️nakagin-capsule-tower.pack.semio";

    fn utf8(bytes: Vec<u8>, what: &str) -> Result<String, String> {
        String::from_utf8(bytes).map_err(|error| format!("{what} is not UTF-8: {error}"))
    }

    /// 🏗️ The real capsule tower model, parsed through this repository's own DSL codec.
    fn tower(ctx: &Context) -> Result<SemioModelSnapshot, String> {
        parse_semio_model_dsl(&utf8(ctx.fixture_bytes(TOWER_DSL)?, "the committed capsule tower model")?)
    }

    /// 📜️ The scenario's own committed mutation parameters — the feature owns the vector. `base`
    /// is only consulted for the `no-mutation` scenario's identity mapping.
    fn mutation(ctx: &Context, base: &SemioModelSnapshot) -> Result<SemioModelMutation, String> {
        let json = semio_repo_test_host::parse_json(ctx.doc_string()?).map_err(|error| format!("{}: the scenario's mutation payload must decode: {error}", ctx.scenario.id))?;
        Ok(decode_mutation(&json, base))
    }

    /// 🧫️ Every `local://` URI the scenario's steps name, in step order, including the ones its data
    /// table carries — which is how the specification vectors are declared.
    fn step_fixtures(ctx: &Context) -> Vec<String> {
        let mut found = Vec::new();
        let mut scan = |text: &str| {
            let mut rest = text;
            while let Some(at) = rest.find("local://") {
                let tail = &rest[at..];
                let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
                found.push(tail[..end].to_string());
                rest = &tail[end..];
            }
        };
        for (_, text) in &ctx.scenario.steps {
            scan(text);
        }
        if let Ok(rows) = ctx.data_table() {
            for row in rows {
                for cell in row {
                    scan(cell);
                }
            }
        }
        found
    }

    fn vector(ctx: &Context, position: usize, label: &str) -> Result<Json, String> {
        let uri = step_fixtures(ctx).into_iter().nth(position).ok_or_else(|| format!("{}: the scenario names no {label} fixture", ctx.scenario.id))?;
        ctx.fixture_json(&uri)
    }

    fn apply(current: &mut SemioModelSnapshot, step: &SemioModelMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_model_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioModelSnapshot, expected: &SemioModelSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }
    //#endregion 🔖️Input

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real capsule tower model by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = tower(ctx)?;
        let step = mutation(ctx, &current)?;
        apply(&mut current, &step, &ctx.scenario.id)?;
        let projection = snapshot_json(&current);
        Ok(Outcome::with_raw(print_semio_model_dsl(&current).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law on the real model: applying the verb and then its OWN computed
    /// inverse must restore the capsule tower exactly — collection ORDER, the property sets and the
    /// 1 840 `f64` transform components included.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = tower(ctx)?;
        let step = mutation(ctx, &base)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = snapshot_json(&current);
        for undo in semio_model_mutation_inverse(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the capsule tower model", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🧫️ The same verb on its committed `(before, mutation, after)` vector, whose before-snapshot is
    /// the real committed demo building decoded — a THIRD statement of what the verb means,
    /// independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_snapshot(&vector(ctx, 0, "before-snapshot")?);
        let step = decode_mutation(&vector(ctx, 1, "mutation")?, &base);
        let expected = decode_snapshot(&vector(ctx, 2, "after-snapshot")?);
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied model does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        let applied = snapshot_json(&current);
        for undo in semio_model_mutation_inverse(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the committed mutation did not restore its before-snapshot", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("applied".to_string(), applied), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🔁️ All four committed encodings — the demo building's two and the capsule tower's two — each
    /// re-emitted from the parsed document.
    ///
    /// 🔒️ **The byte half of the identity law, asserted as `carrier_is_exact` and asserted in both
    /// directions.** `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary
    /// twin, so reproducing them BYTE FOR BYTE is the correct answer here and
    /// `law::reparsed_not_copied` would be exactly backwards. Nor is it a self-comparison: the demo
    /// building's bytes were written by THIS codec and the Python oracle reproduces them from the
    /// grammar alone, while the capsule tower's bytes were written by the PYTHON implementation and
    /// this codec has to reproduce THOSE — `OT` element classes, `M` mesh references and elements
    /// with no `spatialId` among them, three tags no committed pack had carried before.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let building_dsl = ctx.fixture_bytes(BUILDING_DSL)?;
        let building = parse_semio_model_dsl(&utf8(building_dsl.clone(), "the committed demo building")?)?;
        let building_printed = print_semio_model_dsl(&building);
        carrier_is_exact(building_printed.as_bytes(), &building_dsl)?;
        let building_pack = ctx.fixture_bytes(BUILDING_PACK)?;
        let building_unpacked = decode_semio_model_pack(&building_pack)?;
        if building_unpacked != building {
            return Err(disagreement("identity-round-trip: the demo building's binary twin decodes to a different model than its text", &building_unpacked, &building));
        }
        let building_repacked = encode_semio_model_pack(&building);
        carrier_is_exact(&building_repacked, &building_pack)?;
        let tower_dsl = ctx.fixture_bytes(TOWER_DSL)?;
        let model = parse_semio_model_dsl(&utf8(tower_dsl.clone(), "the committed capsule tower model")?)?;
        let tower_printed = print_semio_model_dsl(&model);
        carrier_is_exact(tower_printed.as_bytes(), &tower_dsl)?;
        let reparsed = parse_semio_model_dsl(&tower_printed)?;
        if reparsed != model {
            return Err(disagreement("identity-round-trip: printing the capsule tower back to DSL and reparsing it lost content", &reparsed, &model));
        }
        let tower_pack = ctx.fixture_bytes(TOWER_PACK)?;
        let tower_unpacked = decode_semio_model_pack(&tower_pack)?;
        if tower_unpacked != model {
            return Err(disagreement("identity-round-trip: the capsule tower's binary twin decodes to a different model than its text", &tower_unpacked, &model));
        }
        let tower_repacked = encode_semio_model_pack(&model);
        carrier_is_exact(&tower_repacked, &tower_pack)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("building".to_string(), snapshot_json(&building)),
            ("buildingDslDigest".to_string(), Json::String(digest(building_printed.as_bytes()))),
            ("buildingPackDigest".to_string(), Json::String(digest(&building_repacked))),
            ("towerDslDigest".to_string(), Json::String(digest(tower_printed.as_bytes()))),
            ("towerPackDigest".to_string(), Json::String(digest(&tower_repacked))),
            ("towerSpatial".to_string(), Json::Number(model.spatial.len() as f64)),
            ("towerElements".to_string(), Json::Number(model.elements.len() as f64)),
            ("towerRelations".to_string(), Json::Number(model.relations.len() as f64)),
            ("towerDslLength".to_string(), Json::Number(tower_printed.len() as f64)),
            ("towerPackLength".to_string(), Json::Number(tower_repacked.len() as f64)),
        ])))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Only subject handlers are
/// registered: the oracle role belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
