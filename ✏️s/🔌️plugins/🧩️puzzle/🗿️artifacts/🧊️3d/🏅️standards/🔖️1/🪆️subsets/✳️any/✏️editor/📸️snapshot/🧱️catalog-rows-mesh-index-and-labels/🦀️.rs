//! 🧱️ Catalog rows, mesh index and labels: the kind row normalization, the catalog upserts, the mesh urls and scales, the vortex world positions and the object display labels. See [`super`].

use super::*;

/// 🔌️ `kit:in` seam helper: keyed UPSERT of `incoming` rows (each shaped `{"id": "...", ...}`) into
/// `catalogs[section]` (creating the section as an empty array if absent) — replaces any existing row
/// with the same `"id"`, else appends. Deterministic/order-independent in the resulting SET of ids (a
/// `multiplicity: Many` port may fan in from several producers across several `import_media` calls);
/// when two producers disagree on one id's content, the most-recently-applied wins.
/// 🌉️ Rebuilds `row` into a fresh `Object` rather than mutating in place — this crate's own
/// `Object` (unlike `serde_json::Map`) has no `remove`, only `get`/`get_mut`/`insert`/`iter`, so
/// dropping the `meshUrl` key means copying every other key across instead.
fn puzzle3d_normalize_object_kind_row(row: Value) -> Value {
    let mesh_url = row.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
    let has_rep = row.get("representations").and_then(Value::as_array).is_some_and(|rows| rows.iter().any(|rep| rep.get("url").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some()));
    let id = row.get("id").and_then(Value::as_str).unwrap_or("kind").to_string();
    let Some(url) = mesh_url else {
        return row;
    };
    let Some(object) = row.as_object() else {
        return row;
    };
    let mut next = Object::new();
    for (key, value) in object.iter() {
        if key == "meshUrl" {
            continue;
        }
        next.insert(key.to_string(), value.clone());
    }
    if !has_rep {
        next.insert("representations".to_string(), json!([{ "id": format!("{id}:rep0"), "name": "default", "url": url, "mime": "", "description": "", "tags": [] }]));
    }
    Value::Object(next)
}

/// 🌉️ `catalogs` is stored as `Puzzle3dSceneMetadata.kind_catalogs: Option<dsl::DslValue>` (a
/// `#[derive(value_derive::ToValue, FromValue)]` field, so it cannot hold this crate's own
/// `Value` — see this file's own `dsl::DslValue` field-type note), while `incoming` is the
/// still-`Value`-shaped `import_media` fragment; `DslValue`'s `Object`/`Array` variants are bare
/// `Vec`s with no `entry`/`as_*_mut` sugar, so the section lookup below is index-based instead of
/// `serde_json::Value`'s `entry(...).or_insert_with(...)`.
pub fn puzzle3d_upsert_catalog_rows(catalogs: &mut semio_framework_value::DslValue, section: &str, incoming: Option<&Value>) {
    let Some(incoming_rows) = incoming.and_then(Value::as_array) else {
        return;
    };
    if incoming_rows.is_empty() {
        return;
    }
    let semio_framework_value::DslValue::Object(catalog_entries) = catalogs else {
        return;
    };
    let section_index = match catalog_entries.iter().position(|(key, _)| key == section) {
        Some(index) => index,
        None => {
            catalog_entries.push((section.to_string(), semio_framework_value::DslValue::Array(Vec::new())));
            catalog_entries.len() - 1
        }
    };
    if !matches!(catalog_entries[section_index].1, semio_framework_value::DslValue::Array(_)) {
        catalog_entries[section_index].1 = semio_framework_value::DslValue::Array(Vec::new());
    }
    let semio_framework_value::DslValue::Array(existing) = &mut catalog_entries[section_index].1 else {
        return;
    };
    for row in incoming_rows {
        let row = if section == "objects" { puzzle3d_normalize_object_kind_row(row.clone()) } else { row.clone() };
        let Some(id) = row.get("id").and_then(Value::as_str) else {
            continue;
        };
        let row_dsl = semio_framework_pack_json::to_dsl_value(&row);
        match existing.iter().position(|entry| entry.get("id").and_then(semio_framework_value::DslValue::as_str) == Some(id)) {
            Some(index) => existing[index] = row_dsl,
            None => existing.push(row_dsl),
        }
    }
}

pub fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| FromValue::from_value(semio_framework_pack_json::to_dsl_value(value)).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

/// 📢️ The localized notice of every refusal code the puzzle 3d guest names (design §20.12): a set-verb's missing flag value and
/// the fill tool's run refusals (`🛠️tools/🪣️fill`).
pub fn puzzle3d_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: LazyLock<[(&str, LocalizedLabel); 3]> = LazyLock::new(|| {
        [
            ("puzzle3d.action.flag-value-required", LocalizedLabel::native("Choose on or off for this setting.", "Für diese Einstellung Ein oder Aus wählen.")),
            ("puzzle3d.fill.scene-unavailable", LocalizedLabel::native("Fill cannot start: the document has nothing to place against.", "Füllen kann nicht starten: Das Dokument enthält nichts, woran platziert werden kann.")),
            ("puzzle3d.fill.provisional-unreadable", LocalizedLabel::native("The fill's unconfirmed placements could not be read back. Start the fill again.", "Die unbestätigten Platzierungen des Füllens konnten nicht gelesen werden. Füllen erneut starten.")),
        ]
    });
    &*NOTICES
}

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setSelectionLocked{locked}`) — the row target's explicit next state, so a stale view sets a value and never flips one.
pub fn puzzle3d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" => Some("hidden"),
        "setSelectionLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit `{entity, ids}` a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
pub fn puzzle3d_flag_value_action(id: &str, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation)
        .category("hand")
        .with_args([ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), ActionArgDef::toggle(flag, value).required()])
        .in_palette(false)
}

/// 🗂️ One document catalog's `objectKind id → meshUrl` rows, indexed in a single pass over the
/// untyped catalog array. Resolving one object's mesh straight out of that array is a linear scan, so
/// a caller that resolves a whole document pays O(objects × kinds) — a measured 7.4 ms of every
/// `sync_precompute_session` turn on the 180-object Nakagin document (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END W-P3). Every whole-document caller builds this once instead.
pub struct Puzzle3dKindMeshIndex<'a> {
    by_kind: HashMap<&'a str, &'a str>,
}

impl<'a> Puzzle3dKindMeshIndex<'a> {
    pub fn of(meta: &'a Puzzle3dSceneMetadata) -> Self {
        let rows = meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")).and_then(semio_framework_value::DslValue::as_array).unwrap_or_default();
        let mut by_kind = HashMap::with_capacity(rows.len());
        for entry in rows {
            // 🐛️ Imported rows are normalized to `representations` (`puzzle3d_normalize_object_kind_row` drops
            // `meshUrl`), so reading `meshUrl` alone dropped every kind no placed object renders: the fill's
            // mesh lane had no body for it and each of its candidates was rejected `mesh-unavailable`. Same
            // order as `resolve_object_kind_mesh_url`, so the lane key IS the key a candidate looks up.
            let url = entry.get("representations").and_then(semio_framework_value::DslValue::as_array).into_iter().flatten().filter_map(|rep| rep.get("url").and_then(semio_framework_value::DslValue::as_str)).map(str::trim).find(|url| !url.is_empty());
            let (Some(id), Some(url)) = (entry.get("id").and_then(semio_framework_value::DslValue::as_str), url.or_else(|| entry.get("meshUrl").and_then(semio_framework_value::DslValue::as_str))) else {
                continue;
            };
            by_kind.insert(id, url);
        }
        Self { by_kind }
    }

    /// 🥽️ The mesh identity one object renders and collides with — its own `meshUrl` when it carries
    /// a non-empty one, else its kind's catalog row.
    pub fn resolve<'b>(&'b self, object: &'b Puzzle3dObject) -> Option<&'b str> {
        if let Some(url) = object.mesh_url.as_deref().filter(|url| !url.is_empty()) {
            return Some(url);
        }
        self.by_kind.get(object.object_kind.as_deref()?).copied()
    }

    fn catalog_urls(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.by_kind.values().copied()
    }
}

pub fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dSceneMetadata) -> Option<String> {
    Puzzle3dKindMeshIndex::of(meta).resolve(object).map(str::to_string)
}

/// 🥽️ Every mesh identity the document renders or a catalog kind can place, sorted, so the world mesh lane
/// (and every tool run trace subject indexing it) keeps one stable order.
pub fn collect_mesh_urls(scene_snapshot: &Puzzle3dSceneSnapshot) -> Vec<String> {
    let index = Puzzle3dKindMeshIndex::of(&scene_snapshot.meta);
    let mut urls: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for object in &scene_snapshot.objects {
        if let Some(url) = index.resolve(object) {
            urls.insert(url);
        }
    }
    urls.extend(index.catalog_urls());
    urls.into_iter().map(str::to_string).collect()
}

pub fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
    match object.scale.as_ref().and_then(semio_framework_value::DslValue::as_array) {
        Some(values) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn target_volume_scale_json(volume: &Puzzle3dTargetVolume) -> [f64; 3] {
    match volume.scale.as_ref().and_then(semio_framework_value::DslValue::as_array) {
        Some(values) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn world_vortex_position(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, vortex.position);
    [object.origin.first().copied().unwrap_or(0.0) + rotated[0], object.origin.get(1).copied().unwrap_or(0.0) + rotated[1], object.origin.get(2).copied().unwrap_or(0.0) + rotated[2]]
}

pub fn resolve_vortex_world_position(scene_snapshot: &Puzzle3dSceneSnapshot, full_id: &str) -> Option<[f64; 3]> {
    for object in &scene_snapshot.objects {
        for vortex in &object.vortices {
            if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                return Some(world_vortex_position(object, vortex));
            }
        }
    }
    None
}

pub fn puzzle3d_catalog_entries<'a>(scene_snapshot: &'a Puzzle3dSceneSnapshot, section: &str) -> &'a [semio_framework_value::DslValue] {
    scene_snapshot.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(section)).and_then(|entries| entries.as_array()).unwrap_or(&[])
}

pub fn puzzle3d_kind_ids(scene_snapshot: &Puzzle3dSceneSnapshot, section: &str) -> Vec<String> {
    puzzle3d_catalog_entries(scene_snapshot, section).iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

pub fn next_object_id() -> String {
    let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("object-{next}")
}

/// 🏷️ One kind catalog row's display name — the same `label` / `name` / `id` precedence as the catalogue panel.
pub fn puzzle3d_kind_catalog_label(scene_snapshot: &Puzzle3dSceneSnapshot, kind_id: &str) -> String {
    puzzle3d_catalog_entries(scene_snapshot, "objects")
        .iter()
        .find(|entry| entry.get("id").and_then(semio_framework_value::DslValue::as_str) == Some(kind_id))
        .map(crate::catalog_entry_label)
        .unwrap_or_else(|| kind_id.to_string())
}

fn puzzle3d_label_root(label: &str) -> String {
    let Some((base, suffix)) = label.rsplit_once(' ') else {
        return label.to_string();
    };
    if suffix.parse::<u32>().is_ok() {
        base.to_string()
    } else {
        label.to_string()
    }
}

fn puzzle3d_label_number(label: &str, root: &str) -> Option<u32> {
    if label == root {
        return Some(1);
    }
    let rest = label.strip_prefix(root)?.strip_prefix(' ')?;
    rest.parse().ok()
}

/// 🏷️ What one object row and one 3D instance should read — authored label, else the kind's catalog name, else id.
pub fn puzzle3d_object_display_label(object: &Puzzle3dObject, scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    if let Some(label) = object.label.as_deref().filter(|value| !value.is_empty()) {
        return label.to_string();
    }
    object
        .object_kind
        .as_deref()
        .map(|kind| puzzle3d_kind_catalog_label(scene_snapshot, kind))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| object.id.clone())
}

/// 🪧️ What a history-edit reference chip reads for one entity of the typed document (design §16.4) — the outliner's
/// names: an object as [`puzzle3d_object_display_label`] names it (authored label, else its kind's catalog label or
/// name, else the kind id), a vortex as `<object> · <vortex label or kind>`, an attraction as `<attracting object> →
/// <attracted object>`. Target volumes and references carry no authored name, so they keep the framework's
/// `<Kind> <short id>`; so does an id that is not one of `kinds`.
pub fn puzzle3d_entity_label(snapshot: &Puzzle3dSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    let wants = |kind: &str| kinds.is_empty() || kinds.iter().any(|declared| declared == kind);
    let text = |value: &Option<String>| value.clone().filter(|text| !text.is_empty());
    let object_label = |object: &semio_s_artifact_puzzle_3d_model::Puzzle3dObject| {
        text(&object.label)
            .or_else(|| {
                let kind = object.object_kind.as_deref()?;
                let entry = snapshot.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.objects.iter().find(|entry| entry.id == kind));
                Some(entry.and_then(|entry| [&entry.label, &entry.name].into_iter().find(|name| !name.is_empty()).cloned()).unwrap_or_else(|| kind.to_string()))
            })
            .unwrap_or_else(|| object.id.clone())
    };
    let port = |full_id: &str| snapshot.objects.iter().find_map(|object| object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).map(|vortex| (object, vortex)));
    let label = if let Some(object) = wants(PUZZLE3D_GRANULARITY_OBJECT).then(|| snapshot.objects.iter().find(|object| object.id == id)).flatten() {
        Some(object_label(object))
    } else if let Some((object, vortex)) = wants(PUZZLE3D_GRANULARITY_VORTEX).then(|| port(id)).flatten() {
        Some(format!("{} \u{b7} {}", object_label(object), text(&vortex.label).or_else(|| text(&vortex.vortex_kind)).unwrap_or_else(|| vortex.id.clone())))
    } else if let Some(attraction) = wants(PUZZLE3D_GRANULARITY_ATTRACTION).then(|| snapshot.attractions.iter().find(|attraction| attraction.id == id)).flatten() {
        let end = |full_id: &str| port(full_id).map_or_else(|| full_id.to_string(), |(object, _)| object_label(object));
        Some(format!("{} \u{2192} {}", end(&attraction.attracting), end(&attraction.attracted)))
    } else {
        None
    };
    label.filter(|label| label != id).map(|label| LocalizedLabel::data(&label))
}

/// 🔢️ The next distinct object label for one kind — first instance takes the catalog name; further ones append ` 2`, ` 3`, … to the root taken from peers.
pub fn puzzle3d_next_object_label(objects: &[Puzzle3dObject], scene_snapshot: &Puzzle3dSceneSnapshot, kind_id: &str) -> String {
    let catalog_base = puzzle3d_kind_catalog_label(scene_snapshot, kind_id);
    let peers: Vec<&Puzzle3dObject> = objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind_id)).collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let root = peers
        .iter()
        .find_map(|object| object.label.as_deref().filter(|value| !value.is_empty()))
        .map(puzzle3d_label_root)
        .unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for object in peers {
        if let Some(label) = object.label.as_deref().filter(|value| !value.is_empty()) {
            if let Some(number) = puzzle3d_label_number(label, &root) {
                max = max.max(number);
            }
        } else {
            unlabeled += 1;
        }
    }
    max = max.max(unlabeled);
    if max == 0 {
        root
    } else {
        format!("{root} {}", max + 1)
    }
}

/// 🧊️ Seeds real vortices for a freshly placed object from its kind catalog's `vortices` templates, so it is immediately brushable instead of connector-less.
///
/// 🐛️ A catalog TEMPLATE names its seat `point` (`Puzzle3dCatalogVortexTemplate`), a placed vortex
/// names it `position` (`Puzzle3dVortex`) — reading only `position` here silently stacked every
/// seeded vortex of every catalogued kind at the object origin, so a freshly added object offered one
/// degenerate seat instead of its real rim. `position` is kept as the fallback for the
/// already-placed-shaped rows `kit:in` imports carry.
pub fn puzzle3d_vortices_from_kind_template(catalog_entry: &semio_framework_value::DslValue) -> Vec<Puzzle3dVortex> {
    catalog_entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let position = template.get("point").or_else(|| template.get("position")).and_then(|value| FromValue::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                    let direction = template.get("direction").and_then(|value| FromValue::from_value(value.clone()).ok());
                    let radius = template.get("radius").and_then(|value| value.as_f64());
                    Puzzle3dVortex { id: format!("v{index}"), vortex_kind: template.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string), position, direction, radius, hidden: false, locked: false }
                })
                .collect()
        })
        .unwrap_or_default()
}
