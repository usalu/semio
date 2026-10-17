//! 🗂️ Catalogue of the puzzle 5d editor: the kind and part labels, the inferred kind rows and the kind distribution helpers. See [`super`].

use crate::Puzzle5dSnapshot;
use semio_framework_plugin::ActionArgOption;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_pack_json::Value;
use std::collections::HashMap;
use crate::editor::puzzle5d::entities::{PUZZLE5D_DEFAULT_PART_RADIUS, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, Puzzle5dDocument, Puzzle5dPart, puzzle5d_grip_full_id};
use crate::editor::puzzle5d::view::{engine_grip_kind, Puzzle5dScene};
use crate::editor::puzzle5d::entities::{grips_from_templates, resolve_part_kind_mesh_url, Puzzle5dFreshIds, Puzzle5dPart2d, Puzzle5dPart3d, PUZZLE5D_FLAT_TO_WORLD};

//#region 🏷️Labels
/// 🏷️ The catalogue display name one part kind carries (`label` → `name` → its own id), else the kind
/// id itself — the 5d twin of `puzzle3d_kind_catalog_label`.
pub fn puzzle5d_kind_catalog_label(document: &Puzzle5dDocument, kind_id: &str) -> String {
    document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(kind_id))
        .map(catalog_kind_label)
        .unwrap_or_else(|| kind_id.to_string())
}

fn puzzle5d_label_root(label: &str) -> String {
    let Some((base, suffix)) = label.rsplit_once(' ') else {
        return label.to_string();
    };
    if suffix.parse::<u32>().is_ok() { base.to_string() } else { label.to_string() }
}

fn puzzle5d_label_number(label: &str, root: &str) -> Option<u32> {
    if label == root {
        return Some(1);
    }
    label.strip_prefix(root)?.strip_prefix(' ')?.parse().ok()
}

/// 🏷️ The authored label of one part — its volume label, else its flat text, else the kind's catalogue
/// display name, else its id. Same precedence as `puzzle3d_object_display_label`, with 5d's second
/// authored carrier (the flat aspect's `text`) between the label and the catalogue name.
pub fn puzzle5d_part_display_label(part: &Puzzle5dPart, document: &Puzzle5dDocument) -> String {
    if let Some(label) = part.part_3d.label.as_deref().filter(|value| !value.is_empty()) {
        return label.to_string();
    }
    if !part.part_2d.text.is_empty() {
        return part.part_2d.text.clone();
    }
    if part.part_kind.is_empty() {
        return part.id.clone();
    }
    puzzle5d_kind_catalog_label(document, &part.part_kind)
}

/// 🪧️ What a history-edit reference chip reads for one entity of the typed document (design §16.4) — the outliner's
/// names over the document itself: a part by its volume label, else its flat text, else its kind id (the catalogue rides
/// in a child store the chip does not open), a grip as `<part> · <grip kind>`, a fastener as `<part> → <part>`. Target
/// volumes carry no authored name, so they keep the framework's `<Kind> <short id>`; so does an id outside `kinds`.
pub fn puzzle5d_entity_label(snapshot: &Puzzle5dSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    let wants = |kind: &str| kinds.is_empty() || kinds.iter().any(|declared| declared == kind);
    let text = |value: &Option<String>| value.clone().filter(|text| !text.is_empty());
    let part_label = |part: &crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart| text(&part.part_3d.label).or_else(|| text(&part.part_2d.text)).or_else(|| text(&part.part_kind)).unwrap_or_else(|| part.id.clone());
    let port = |full_id: &str| snapshot.parts.iter().find_map(|part| part.grips.iter().find(|grip| puzzle5d_grip_full_id(&part.id, &grip.id) == full_id).map(|grip| (part, grip)));
    let label = if let Some(part) = wants(PUZZLE5D_GRANULARITY_PART).then(|| snapshot.parts.iter().find(|part| part.id == id)).flatten() {
        Some(part_label(part))
    } else if let Some((part, grip)) = wants(PUZZLE5D_GRANULARITY_GRIP).then(|| port(id)).flatten() {
        Some(format!("{} \u{b7} {}", part_label(part), text(&grip.grip_kind).or_else(|| text(&grip.grip_2d.grip_kind)).unwrap_or_else(|| grip.id.clone())))
    } else if let Some(fastener) = wants(PUZZLE5D_GRANULARITY_FASTENER).then(|| snapshot.fasteners.iter().find(|fastener| fastener.id == id)).flatten() {
        let end = |full_id: &str| port(full_id).map_or_else(|| full_id.to_string(), |(part, _)| part_label(part));
        Some(format!("{} \u{2192} {}", end(&fastener.source), end(&fastener.target)))
    } else {
        None
    };
    label.filter(|label| label != id).map(|label| LocalizedLabel::data(&label))
}

/// 🔢️ The next distinct part label for one kind — the first instance takes the catalogue name, each
/// further one appends ` 2`, ` 3`, … to the root its peers already carry.
pub fn puzzle5d_next_part_label(parts: &[Puzzle5dPart], document: &Puzzle5dDocument, kind_id: &str) -> String {
    let catalog_base = puzzle5d_kind_catalog_label(document, kind_id);
    let peers: Vec<&Puzzle5dPart> = parts.iter().filter(|part| part.part_kind == kind_id).collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let root = peers.iter().find_map(|part| part.part_3d.label.as_deref().filter(|value| !value.is_empty())).map(puzzle5d_label_root).unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for part in peers {
        match part.part_3d.label.as_deref().filter(|value| !value.is_empty()) {
            Some(label) => {
                if let Some(number) = puzzle5d_label_number(label, &root) {
                    max = max.max(number);
                }
            }
            None => unlabeled += 1,
        }
    }
    max = max.max(unlabeled);
    if max == 0 { root } else { format!("{root} {}", max + 1) }
}

/// 🔢️ `puzzle5d_next_part_label` read off the JSON projection instead of the typed document — the
/// numbering a RETAINED creation Work needs, because those works only ever hold `Puzzle5dPlaySnapshot`'s
/// projection. Same precedence and the same `puzzle5d_label_root`/`puzzle5d_label_number` arithmetic, so
/// a part created by the palette and a part created by `addNode` land on the same series.
pub fn puzzle5d_next_part_label_from_projection(projection: &Value, kind_id: &str) -> String {
    let catalog_base = projection
        .get("kindCatalogs")
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(kind_id))
        .and_then(|entry| ["label", "name", "id"].into_iter().find_map(|key| entry.get(key).and_then(Value::as_str).filter(|value| !value.is_empty())))
        .unwrap_or(kind_id)
        .to_string();
    let peer_label = |part: &Value| part.get("3d").and_then(|pose| pose.get("label")).and_then(Value::as_str).filter(|value| !value.is_empty()).map(str::to_string);
    let peers: Vec<String> = projection
        .get("parts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|part| part.get("partKind").and_then(Value::as_str) == Some(kind_id))
        .map(|part| peer_label(part).unwrap_or_default())
        .collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let root = peers.iter().find(|label| !label.is_empty()).map(|label| puzzle5d_label_root(label)).unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for label in &peers {
        if label.is_empty() {
            unlabeled += 1;
        } else if let Some(number) = puzzle5d_label_number(label, &root) {
            max = max.max(number);
        }
    }
    max = max.max(unlabeled);
    if max == 0 { root } else { format!("{root} {}", max + 1) }
}
//#endregion 🏷️Labels

//#region 🧬️InferredKinds
/// 🧬️ The part-kind rows a catalogue-less document IMPLIES: one row per distinct `partKind`, shaped
/// like the first part carrying it — flat shape and size, icon, mesh and that part's grips as
/// templates. None of the three 5d examples authors `kindCatalogs`, so without this every catalogue
/// section and every catalogue drop would be empty; a document whose `kindCatalogs` names parts never
/// reaches this (mirrors `inferred_node_kind_rows` in puzzle 2d).
pub fn puzzle5d_inferred_part_kind_rows(document: &Puzzle5dDocument) -> Vec<Value> {
    let mut rows: Vec<Value> = Vec::new();
    for part in &document.parts {
        if part.part_kind.is_empty() || rows.iter().any(|row| row.get("id").and_then(Value::as_str) == Some(part.part_kind.as_str())) {
            continue;
        }
        let rectangle = part.part_2d.shape == "rectangle";
        let size = if rectangle {
            part.part_2d.width.unwrap_or(PUZZLE5D_DEFAULT_PART_RADIUS * 2.0).max(part.part_2d.height.unwrap_or(PUZZLE5D_DEFAULT_PART_RADIUS * 2.0))
        } else {
            (if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS }) * 2.0
        };
        let grips: Vec<Value> = part
            .grips
            .iter()
            .map(|grip| {
                semio_framework_pack_json::json!({
                    "gripKind": engine_grip_kind(grip),
                    "angle": grip.grip_2d.angle,
                    "radius": grip.grip_3d.radius,
                    "position": grip.grip_3d.position,
                    "direction": grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]),
                })
            })
            .collect();
        let mut row = semio_framework_pack_json::json!({
            "id": part.part_kind,
            "name": part.part_kind,
            "shape": if rectangle { "rectangle" } else { "circle" },
            "radius": size * 0.5,
            "width": size,
            "height": size,
            "grips": grips,
        });
        if let Some(object) = row.as_object_mut() {
            if let Some(icon) = part.part_2d.icon_kind.as_deref().filter(|icon| !icon.is_empty()) {
                object.insert("iconKind", semio_framework_pack_json::json!(icon));
            }
            if let Some(url) = part.part_3d.mesh_url.as_deref().filter(|url| !url.is_empty()) {
                object.insert("meshUrl", semio_framework_pack_json::json!(url));
            }
        }
        rows.push(row);
    }
    rows
}

/// 🧬️ The read-only kind rows of the grip/fastener/rope slices a catalogue-less document implies —
/// one `{id, name}` per distinct kind the document already names. `ropes` has no document carrier, so
/// it stays empty and its section keeps its placeholder.
pub fn puzzle5d_inferred_kind_rows(document: &Puzzle5dDocument, slice: &str) -> Vec<Value> {
    if slice == "parts" {
        return puzzle5d_inferred_part_kind_rows(document);
    }
    let mut ids: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    match slice {
        "grips" => {
            for part in &document.parts {
                ids.extend(part.grips.iter().map(engine_grip_kind).filter(|kind| !kind.is_empty()));
            }
        }
        "fasteners" => ids.extend(document.fasteners.iter().filter_map(|fastener| fastener.fastener_kind.clone()).filter(|kind| !kind.is_empty())),
        _ => {}
    }
    ids.into_iter().map(|id| semio_framework_pack_json::json!({ "id": id, "name": id })).collect()
}
//#region 🙈️SelectionFlags
//#endregion 🙈️SelectionFlags
//#endregion 🧬️InferredKinds

//#region 🔖️Distribution
pub fn puzzle5d_kind_ids(document: &Puzzle5dDocument, slice: &str) -> Vec<String> {
    let mut ids: Vec<String> =
        document.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(slice)).and_then(|value| value.as_array()).into_iter().flatten().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
    if ids.is_empty() {
        let mut inferred: Vec<String> = match slice {
            "parts" => document.parts.iter().map(|part| part.part_kind.clone()).collect(),
            "grips" => document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| grip.grip_kind.clone())).collect(),
            _ => Vec::new(),
        };
        inferred.sort();
        inferred.dedup();
        ids = inferred;
    }
    ids
}

pub fn puzzle5d_uniform_kind_weights(ids: &[String]) -> HashMap<String, f64> {
    if ids.is_empty() {
        return HashMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

pub fn puzzle5d_normalize_kind_weight_group(weights: &HashMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> HashMap<String, f64> {
    if kind_ids.is_empty() {
        return HashMap::new();
    }
    if kind_ids.len() == 1 {
        return HashMap::from([(kind_ids[0].clone(), 1.0)]);
    }
    let new_value = new_value.clamp(0.0, 1.0);
    let others: Vec<&String> = kind_ids.iter().filter(|id| id.as_str() != changed_id).collect();
    let remainder = (1.0 - new_value).max(0.0);
    let other_sum: f64 = others.iter().map(|id| weights.get(*id).copied().unwrap_or(0.0)).sum();
    let mut next = HashMap::new();
    next.insert(changed_id.to_string(), new_value);
    if remainder <= f64::EPSILON {
        for id in others {
            next.insert((*id).clone(), 0.0);
        }
        return next;
    }
    if other_sum <= f64::EPSILON {
        let each = remainder / others.len() as f64;
        for id in others {
            next.insert((*id).clone(), each);
        }
    } else {
        for id in others {
            let old = weights.get(id).copied().unwrap_or(0.0);
            next.insert((*id).clone(), old / other_sum * remainder);
        }
    }
    next
}

pub fn puzzle5d_ensure_catalog_kind_weights(weights: &mut HashMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle5d_uniform_kind_weights(kind_ids);
        return;
    }
    let sum: f64 = kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    if (sum - 1.0).abs() > 0.001 {
        for id in kind_ids {
            if let Some(weight) = weights.get_mut(id) {
                *weight /= sum;
            }
        }
    }
}

pub fn puzzle5d_kind_weight_sum(weights: &HashMap<String, f64>, kind_ids: &[String]) -> f64 {
    kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum()
}
//#endregion 🔖️Distribution

/// 🎨️ Palette drop: creates a free paired part at the flat drop point, deriving the volume origin from the nearest peer part's offset.
pub fn add_palette_part(envelope: &mut Puzzle5dScene, part_kind: &str, x: f64, y: f64) {
    let flat_to_world = PUZZLE5D_FLAT_TO_WORLD;
    let origin = envelope
        .document
        .parts
        .first()
        .map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| [peer.part_3d.origin[0] + (x - peer.part_2d.x) * flat_to_world, peer.part_3d.origin[1] - (y - peer.part_2d.y) * flat_to_world, peer.part_3d.origin[2]]);
    let id = Puzzle5dFreshIds::from_document(&envelope.document).next_part();
    let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
    let grips = grips_from_templates(&envelope.document, part_kind);
    // 🏷️ Stamped at creation, not derived at paint: the outliner, the 3D instance and a later export
    // must all read the SAME authored name, and only this moment knows how many peers of this kind
    // the document already holds.
    let label = puzzle5d_next_part_label(&envelope.document.parts, &envelope.document, part_kind);
    envelope.document.parts.push(Puzzle5dPart {
        id: id,
        anchor: Default::default(),
        part_kind: part_kind.into(),
        part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.into(), icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: Some(label) },
        grips,
    });
    let _ = id;
}

/// 🏷️ One catalogue entry's display name: its authored `label`, else its `name`, else its id.
pub fn catalog_kind_label(entry: &Value) -> String {
    entry
        .get("label")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()))
        .or_else(|| entry.get("id").and_then(|value| value.as_str()))
        .unwrap_or("kind")
        .into()
}

/// 🗨️ Fixed ceiling on the part-kind rows the `addPartKind` arg form and the "Add Part" dialog offer —
/// the manifest is minted once per process, so this select is built eagerly and must stay bounded
/// however wide a catalog a future example declares.
pub const PUZZLE5D_PART_KIND_OPTIONS_MAX: usize = 64;

/// 🗨️ The part kinds the "Add Part" dialog (and `addPartKind`'s own arg form) offers — read from the
/// shipped documents' own `kindCatalogs.parts`, exactly the rows `📌️panels/🛍️catalogue` renders,
/// deduplicated across documents in catalog order, with the same part-kind inference fallback the
/// catalogue panel applies to a document that declares no catalog. `AppDefinition` is built once per
/// process and never sees the live document, so the union of the shipped catalogs IS the reachable
/// kind set — not the single literal `"Part"` this select used to hardcode, which could not add a
/// single real kind of any example.
///
/// 🌙️ `capsule-dream` is deliberately NOT in that union, for two independent reasons measured on
/// 2026-09-21 (`📓️pz1-catalog-zero-diagnostics.md` §2.3):
/// 1. **It names no kind a human could pick.** Its own `kindCatalogs.parts` is empty — the catalog
///    is a composed child, absent from the standalone document — so the inference fallback runs over
///    its 2 880 parts, whose `part-kind` column holds the child's raw UUIDs
///    (`"0e240cd2-7f98-42b6-af39-34e7ee4fad35"`, …). `nakagin`'s and `concrete-forest`'s hold names
///    (`Base`, `Bridge`, `Capital`, `Tambour`, `Capsule With Balcony J`, …). A select that offers
///    UUIDs is a worse select, and with `PUZZLE5D_PART_KIND_OPTIONS_MAX = 64` they crowd out real
///    kinds.
/// 2. **It is what made this package undescribable.** Dereferencing
///    `CAPSULE_DREAM_EXAMPLE_DOCUMENT` here parses 3 035 200 B of DSL, re-serialises ~3.5 MB of
///    JSON and deserialises it into `Puzzle5dDocument` — inside the owned interpreter, while the
///    bundle is assembled, i.e. before `describe()` emits anything. `AppDefinition` is built on the
///    describe path, so this one call put `🧩️puzzle` over the 1 800 s guest epoch on its own.
///
/// 🖐️ Those rows are AUTHORED below rather than derived, in the two named examples' own catalog
/// order, and pinned to the documents by
/// `shipped_part_kinds_are_the_two_named_examples_own_catalog_rows`.
///
/// 🐛️ Deriving it dereferenced `CONCRETE_FOREST_EXAMPLE_DOCUMENT` and `NAKAGIN_EXAMPLE_DOCUMENT`,
/// i.e. parsed 171 591 B of authored DSL, re-serialised it to 205 896 B of JSON and deserialised
/// that into two typed `Puzzle5dDocument`s — on the `AppDefinition` path, which is the `describe()`
/// path AND every actor boot. Measured natively on 2026-09-22 (slice PZ2,
/// `🗑️generated/pz2-native-profile-*.txt`): `create_puzzle5d_app()` cost 338 ms cold and 1 ms with
/// those two statics already warm, so ALL of it was this one select. Thirteen authored pairs cost
/// nothing, and the law below is what keeps them true.
pub const PUZZLE5D_SHIPPED_PART_KINDS: &[(&str, &str)] = &[
    ("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
    ("Base", "Base"),
    ("Bridge", "Bridge"),
    ("Capital", "Capital"),
    ("Capsule With Balcony Backslash", "Capsule With Balcony Backslash"),
    ("Capsule With Balcony J", "Capsule With Balcony J"),
    ("Capsule With Balcony L", "Capsule With Balcony L"),
    ("Capsule With Balcony P", "Capsule With Balcony P"),
    ("Capsule With Balcony S", "Capsule With Balcony S"),
    ("Capsule With Balcony Slash", "Capsule With Balcony Slash"),
    ("First Storey Tambour", "First Storey Tambour"),
    ("Last Storey Tambour", "Last Storey Tambour"),
    ("Tambour", "Tambour"),
];

/// 🗂️ The `partKind` select's options, mapped from [`PUZZLE5D_SHIPPED_PART_KINDS`].
pub fn puzzle5d_part_kind_options() -> Vec<ActionArgOption> {
    PUZZLE5D_SHIPPED_PART_KINDS.iter().take(PUZZLE5D_PART_KIND_OPTIONS_MAX).map(|(id, label)| ActionArgOption::new(*id, LocalizedLabel::data(*label))).collect()
}

/// 🗨️ The kind the `partKind` select stages when nothing is picked — the first catalog row, never a
/// literal id no catalog declares.
pub fn puzzle5d_default_part_kind(options: &[ActionArgOption]) -> String {
    options.first().map(|option| option.value.clone()).unwrap_or_default()
}
