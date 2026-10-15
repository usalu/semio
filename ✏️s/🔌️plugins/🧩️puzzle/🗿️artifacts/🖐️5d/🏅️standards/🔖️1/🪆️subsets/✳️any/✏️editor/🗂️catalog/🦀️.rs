//! 🗂️ Catalogue of the puzzle 5d editor: the kind and part labels, the inferred kind rows and the kind distribution helpers. See [`super`].

use super::*;

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
        .map(catalogue::catalog_kind_label)
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
    let part_label = |part: &crate::Puzzle5dPart| text(&part.part_3d.label).or_else(|| text(&part.part_2d.text)).or_else(|| text(&part.part_kind)).unwrap_or_else(|| part.id.clone());
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
