//! 📇️ Kind catalogs and shipped kinds: the manifest and document kind catalogs, the inferred kind entries and the shipped node kinds with their options. See [`super`].

use super::*;
use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::terminology::puzzle2d_localized;
use std::collections::BTreeSet;

fn manifest_catalog_rows(kinds: &[semio_framework_graph_manifest::KindDef]) -> Value {
    Value::Array(
        kinds
            .iter()
            .map(|kind| {
                let mut row = semio_framework_pack_json::Object::new();
                row.insert("id".to_string(), json!(kind.id));
                row.insert("name".to_string(), json!(kind.name));
                if let Some(semio_framework_value::DslValue::Object(presentation)) = kind.presentation.as_ref() {
                    for (key, value) in presentation {
                        row.insert(key.clone(), semio_framework_pack_json::from_dsl_value(value));
                    }
                }
                Value::Object(row)
            })
            .collect(),
    )
}

/// 🗂️ Board engine catalogs for a manifest id. Every shipped puzzle2d document names a
/// `meta.manifestId` and carries no catalogs of its own, so this — not `meta.kindCatalogs` — is where
/// their node/handle kinds actually come from. Each row is the manifest row's `id`/`name` merged with
/// its flattened `presentation`. Port kinds without a `presentation.color` are dropped because the
/// engine rejects a colourless handle kind outright, which would discard the whole catalog push.
pub fn manifest_board_kind_catalogs_json(manifest_id: &str) -> Option<String> {
    let manifest = crate::standards::v1::subsets::any::schema::graph_manifest::manifest_by_id(manifest_id)?;
    let visual_port_kinds: Vec<semio_framework_graph_manifest::KindDef> = manifest.port_kinds.iter().filter(|kind| kind.presentation.as_ref().is_some_and(|p| p.get("color").is_some())).cloned().collect();
    Some(
        json!({
            "handleKinds": manifest_catalog_rows(&visual_port_kinds),
            "wireKinds": manifest_catalog_rows(&manifest.wire_kinds),
            "nodeKinds": manifest_catalog_rows(&manifest.node_kinds),
            "edgeKinds": manifest_catalog_rows(&manifest.edge_kinds),
        })
        .to_string(),
    )
}

fn catalog_row_subset(row: &Value, keys: &[&str]) -> Value {
    let mut out = semio_framework_pack_json::Object::new();
    for key in keys {
        match row.get(*key) {
            Some(value) if !value.is_null() => {
                out.insert((*key).to_string(), value.clone());
            }
            _ => {}
        }
    }
    Value::Object(out)
}

fn catalog_rows_subset(catalogs: &Value, slice: &str, keys: &[&str]) -> Option<Value> {
    let rows = catalogs.get(slice).and_then(Value::as_array)?;
    Some(Value::Array(rows.iter().map(|row| catalog_row_subset(row, keys)).collect()))
}

/// 🗂️ Projects the document's `meta.kindCatalogs` onto the board engine's catalog contract. The
/// document owns `nodes`/`🐙️handles`/`edges`/`wires` ([`Puzzle2dKindCatalogs`]) while
/// `BoardHost::set_board_kind_catalogs_from_json` reads `nodeKinds`/`handleKinds`/`edgeKinds`/`wireKinds`
/// and rejects any row still carrying the document's `label`, so each row is narrowed to the keys the
/// engine actually reads. Without this the engine's `node_kinds` map stays empty and every
/// brush/fill candidate lookup silently yields nothing. A slice absent from the document is left out
/// entirely rather than emitted empty, because the engine reads an omitted array as "leave that
/// slice alone" and an empty one as "clear it".
///
/// Documents that carry no `meta.kindCatalogs` of their own — which is every shipped example, they
/// name a `meta.manifestId` instead — resolve their catalogs from the compile-time manifest registry
/// via [`manifest_board_kind_catalogs_json`].
pub fn board_kind_catalogs_json(snapshot: &Value) -> Option<String> {
    let meta = snapshot.get("meta");
    meta.and_then(|meta| meta.get("kindCatalogs")).and_then(document_board_kind_catalogs_json).or_else(|| meta.and_then(|meta| meta.get("manifestId")).and_then(Value::as_str).and_then(manifest_board_kind_catalogs_json))
}

/// 🗂️ The `meta.kindCatalogs` half of [`board_kind_catalogs_json`]. Returns `None` when the document
/// contributes no node kinds, so a document carrying an empty catalog bundle still falls through to
/// its manifest rather than clearing the engine's catalogs.
fn document_board_kind_catalogs_json(catalogs: &Value) -> Option<String> {
    let node_kinds = catalogs.get("nodes").and_then(Value::as_array).map(|rows| {
        Value::Array(
            rows.iter()
                .map(|row| {
                    let mut node = catalog_row_subset(row, &["id", "name", "icon", "color", "shape", "scale"]);
                    let handles: Vec<Value> = row.get("handles").and_then(Value::as_array).map_or_else(Vec::new, |templates| {
                        templates.iter().filter(|template| template.get("handleKind").and_then(Value::as_str).is_some_and(|kind| !kind.trim().is_empty())).map(|template| catalog_row_subset(template, &["handleKind", "angle", "radius"])).collect()
                    });
                    set_member(&mut node, "handles", Value::Array(handles));
                    node
                })
                .collect(),
        )
    });
    let mut out = semio_framework_pack_json::Object::new();
    for (key, rows) in [
        ("handleKinds", catalog_rows_subset(catalogs, "handles", &["id", "name", "color", "defaultWireKind", "scale"])),
        ("wireKinds", catalog_rows_subset(catalogs, "wires", &["id", "name", "defaultEdgeKind"])),
        ("nodeKinds", node_kinds),
        ("edgeKinds", catalog_rows_subset(catalogs, "edges", &["id", "name", "color", "stroke", "pattern", "sourceTip", "targetTip", "directed"])),
    ] {
        if let Some(rows) = rows {
            out.insert(key.to_string(), rows);
        }
    }
    let contributes_node_kinds = out.get("nodeKinds").and_then(Value::as_array).is_some_and(|rows| !rows.is_empty());
    contributes_node_kinds.then(|| Value::Object(out).to_string())
}

/// 🗂️ [`board_kind_catalogs_json`] under the SAME node-kind resolution law the fill run already uses
/// (`precompute::fill::fill_kind_rows`): a brush or suggestion candidate can only be built from a node
/// kind that carries handle templates, so a resolved catalog whose `nodeKinds` are all template-less —
/// Concrete Forest's manifest rows are presentation only — falls back to the kinds the document itself
/// implies ([`inferred_node_kind_rows`]), and a document resolving no catalog at all uses them outright.
/// Without this the engine's usable `node_kinds` map stays empty and every candidate lookup silently
/// yields nothing: a picker that opens on a free handle and lists nothing forever.
pub fn board_kind_catalogs_json_or_inferred(snapshot: &Value) -> Option<String> {
    let inferred = || {
        let rows = inferred_node_kind_rows(snapshot);
        (!rows.is_empty()).then_some(rows)
    };
    let Some(json) = board_kind_catalogs_json(snapshot) else {
        return inferred().map(|rows| json!({ "nodeKinds": Value::Array(rows) }).to_string());
    };
    let Some(mut catalogs) = semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).ok().filter(|value| value.as_object().is_some()) else {
        return Some(json);
    };
    let templated = catalogs.get("nodeKinds").and_then(Value::as_array).is_some_and(|rows| rows.iter().any(|row| row.get("handles").and_then(Value::as_array).is_some_and(|handles| !handles.is_empty())));
    if templated {
        return Some(json);
    }
    match inferred() {
        Some(rows) => {
            set_member(&mut catalogs, "nodeKinds", Value::Array(rows));
            Some(catalogs.to_string())
        }
        None => Some(json),
    }
}

/// 🗂️ The kind ids present in the document itself, used whenever the snapshot carries no explicit
/// `meta.kindCatalogs` slice.
pub fn inferred_kind_entries(snapshot: &Value, field: &str) -> Vec<Value> {
    let mut ids = BTreeSet::new();
    match field {
        "nodes" => return inferred_node_kind_rows(snapshot),
        "handles" => {
            for node in board_snapshot_nodes(snapshot) {
                if let Some(handles) = node.get("handles").and_then(|value| value.as_array()) {
                    for handle in handles {
                        if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                            ids.insert(kind.to_string());
                        }
                    }
                }
            }
        }
        "edges" => {
            for edge in board_snapshot_edges(snapshot) {
                if let Some(kind) = edge.get("edgeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        _ => {}
    }
    ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect()
}

/// 🧬️ The node kind rows a catalog-less document IMPLIES: one row per distinct `nodeKind`, shaped like
/// the first node carrying it — shape, size (as the engine's `scale` of its 96-unit kind footprint),
/// icon and that node's handles as templates. A document that came from no manifest (Concrete Forest,
/// or any board a user drew) has exactly the kinds it shows, so the catalogue panel offers them and
/// the fill can place them; a document whose meta carries `kindCatalogs` never reaches this.
pub fn inferred_node_kind_rows(snapshot: &Value) -> Vec<Value> {
    let mut rows: Vec<Value> = Vec::new();
    for node in board_snapshot_nodes(snapshot) {
        let Some(kind) = node.get("nodeKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()) else { continue };
        if rows.iter().any(|row| row.get("id").and_then(Value::as_str) == Some(kind)) {
            continue;
        }
        let rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
        let size = if rectangle {
            node.get("width").and_then(Value::as_f64).unwrap_or(48.0).max(node.get("height").and_then(Value::as_f64).unwrap_or(48.0))
        } else {
            node.get("radius").and_then(Value::as_f64).unwrap_or(24.0) * 2.0
        };
        let handles: Vec<Value> = node
            .get("handles")
            .and_then(Value::as_array)
            .map(|handles| {
                handles
                    .iter()
                    .map(|handle| {
                        let mut template = json!({ "handleKind": handle.get("handleKind").and_then(Value::as_str).unwrap_or("port"), "angle": handle.get("angle").and_then(Value::as_f64).unwrap_or(0.0) });
                        if let Some(radius) = handle.get("radius").and_then(Value::as_f64) {
                            set_member(&mut template, "radius", json!(radius));
                        }
                        template
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut row = json!({ "id": kind, "name": kind, "shape": if rectangle { "rectangle" } else { "circle" }, "scale": size / 96.0, "radius": size * 0.5, "width": size, "height": size, "handles": handles });
        if let Some(icon) = node.get("iconKind").and_then(Value::as_str) {
            set_member(&mut row, "iconKind", json!(icon));
        }
        rows.push(row);
    }
    rows
}

/// 🗂️ The node-kind rows a document actually offers: its own `meta.kindCatalogs.nodes` when it
/// declares them, else the rows its `meta.manifestId` declares, else the rows its own nodes imply.
/// One seam so the catalogue panel, the `addNode` arg form and the Add Node dialog can never drift
/// onto different kind sets.
pub fn puzzle2d_node_kind_rows(snapshot: &Value) -> Vec<Value> {
    if let Some(rows) = kind_catalog_entries(snapshot, "nodes").filter(|rows| !rows.is_empty()) {
        return rows.to_vec();
    }
    let manifest_rows = snapshot
        .get("meta")
        .and_then(|meta| meta.get("manifestId"))
        .and_then(Value::as_str)
        .and_then(manifest_board_kind_catalogs_json)
        .and_then(|json| semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).ok())
        .and_then(|catalogs| catalogs.get("nodeKinds").and_then(Value::as_array).cloned())
        .unwrap_or_default();
    if !manifest_rows.is_empty() {
        return manifest_rows;
    }
    inferred_kind_entries(snapshot, "nodes")
}

/// 🗂️ The `kind` options the `addNode` arg form and the Add Node dialog offer. `AppDefinition` is
/// minted once per process and never sees the live document, so the union of the SHIPPED examples'
/// own node kinds is the reachable kind set — never the literal `"node"` option this select used to
/// hardcode, which could not add a single real kind of either example (puzzle3d's own §23 bug).
///
/// 🖐️ Those rows are AUTHORED below rather than derived, in the two shipped examples' own catalog
/// order, and pinned to the documents by `shipped_node_kinds_are_the_two_examples_own_catalog_rows`.
///
/// 🐛️ Deriving it dereferenced `CONCRETE_FOREST_EXAMPLE_JSON` and `NAKAGIN_EXAMPLE_JSON`, i.e.
/// parsed 96 005 B of authored DSL, re-serialised it to JSON and parsed that back into two
/// `semio_framework_pack_json::Value`s — on the `AppDefinition` path, which is the `describe()` path AND every
/// actor boot. Measured natively on 2026-09-22 (slice PZ2,
/// `🗑️generated/pz2-native-profile-*.txt`): `create_puzzle2d_app()` cost 157 ms cold against ~1 ms
/// with those statics warm, so ALL of it was this one select.
pub const PUZZLE2D_SHIPPED_NODE_KINDS: &[(&str, &str)] = &[
    ("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
    ("Balcony", "Balcony"),
    ("Base", "Base"),
    ("Base Blob", "Base Blob"),
    ("Bridge", "Bridge"),
    ("Capital", "Capital"),
    ("Capsule", "Capsule"),
    ("Capsule Backslash", "Capsule Backslash"),
    ("Capsule J", "Capsule J"),
    ("Capsule L", "Capsule L"),
    ("Capsule P", "Capsule P"),
    ("Capsule q", "Capsule q"),
    ("Capsule S", "Capsule S"),
    ("Capsule Slash", "Capsule Slash"),
    ("Capsule With Balcony Backslash", "Capsule With Balcony Backslash"),
    ("Capsule With Balcony J", "Capsule With Balcony J"),
    ("Capsule With Balcony L", "Capsule With Balcony L"),
    ("Capsule With Balcony P", "Capsule With Balcony P"),
    ("Capsule With Balcony Q", "Capsule With Balcony Q"),
    ("Capsule With Balcony S", "Capsule With Balcony S"),
    ("Capsule With Balcony Slash", "Capsule With Balcony Slash"),
    ("Capsule With Balcony Z", "Capsule With Balcony Z"),
    ("Capsule Z", "Capsule Z"),
    ("Cylindric Capital", "Cylindric Capital"),
    ("Cylindric First Storey Tambour", "Cylindric First Storey Tambour"),
    ("Cylindric Last Storey Tambour", "Cylindric Last Storey Tambour"),
    ("Cylindric Single Storey Tambour", "Cylindric Single Storey Tambour"),
    ("Cylindric Tambour", "Cylindric Tambour"),
    ("Ellipsoid", "Ellipsoid"),
    ("First Storey Tambour", "First Storey Tambour"),
    ("Last Storey Tambour", "Last Storey Tambour"),
    ("Single Storey Tambour", "Single Storey Tambour"),
    ("Tambour", "Tambour"),
    ("Trapezoid", "Trapezoid"),
    ("Trapezoid Capsule Backslash", "Trapezoid Capsule Backslash"),
    ("Trapezoid Capsule J", "Trapezoid Capsule J"),
    ("Trapezoid Capsule L", "Trapezoid Capsule L"),
    ("Trapezoid Capsule P", "Trapezoid Capsule P"),
    ("Trapezoid Capsule Q", "Trapezoid Capsule Q"),
    ("Trapezoid Capsule S", "Trapezoid Capsule S"),
    ("Trapezoid Capsule Slash", "Trapezoid Capsule Slash"),
    ("Trapezoid Capsule Z", "Trapezoid Capsule Z"),
    ("Piece", "Piece"),
];

/// 🗂️ The `kind` select's options, mapped from [`PUZZLE2D_SHIPPED_NODE_KINDS`].
pub fn puzzle2d_node_kind_options() -> Vec<ActionArgOption> {
    PUZZLE2D_SHIPPED_NODE_KINDS.iter().take(PUZZLE2D_NODE_KIND_OPTIONS_MAX).map(|(id, label)| ActionArgOption::new(*id, LocalizedLabel::data(*label))).collect()
}

/// 🗂️ The one `kind` select both the standalone `addNode` arg form and the Add Node dialog declare —
/// built twice from the same catalog so the two forms can never drift apart.
pub fn puzzle2d_node_kind_arg() -> ActionArgDef {
    let options = puzzle2d_node_kind_options();
    let default = options.first().map(|option| option.value.clone()).unwrap_or_default();
    ActionArgDef::select("kind", puzzle2d_localized(|l| l.kind), options).required().default_value(&default)
}
