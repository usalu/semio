//! 🧬️ Document codec and JSON of the BCF oracle: the archive decode and encode and the JSON value mapping of the model. See [`super`].

use super::*;

//#region 🔖️Codec
/// 🔮️ Decodes a real bcfzip: `bcf.version`, every topic folder's `markup.bcf` + referenced
/// `.bcfv` + snapshot, and every unconsumed entry retained as a raw part — the same shape
/// `semio-s-plugin-stdio`'s own `decode_bcf` implements, composed here independently.
pub(super) fn decode(bytes: &[u8]) -> Result<ODoc, String> {
    let entries = read_entries(bytes)?;
    let mut version = String::new();
    let mut consumed: HashSet<String> = HashSet::new();
    if let Some((name, data)) = entries.iter().find(|(entry_name, _)| entry_name.eq_ignore_ascii_case("bcf.version")) {
        let root = parse_xml(data)?;
        if let Some(("Version", attrs, _)) = as_element(&root) {
            version = attr(attrs, "VersionId").unwrap_or_default().to_string();
        }
        consumed.insert(name.clone());
    }

    let mut folders: BTreeMap<&str, Vec<&(String, Vec<u8>)>> = BTreeMap::new();
    for entry in &entries {
        if let Some((folder, _)) = entry.0.split_once('/') {
            folders.entry(folder).or_default().push(entry);
        }
    }

    let mut topics = Vec::new();
    for (folder, folder_entries) in &folders {
        let markup_name = format!("{folder}/markup.bcf");
        let Some((markup_entry_name, markup_data)) = folder_entries.iter().find(|(name, _)| name.eq_ignore_ascii_case(&markup_name)) else { continue };
        let (mut topic, viewpoint_refs) = parse_markup(markup_data)?;
        consumed.insert(markup_entry_name.clone());

        let mut viewpoints = Vec::new();
        for reference in &viewpoint_refs {
            let mut camera = None;
            let mut components = None;
            if let Some(viewpoint_file) = &reference.viewpoint_file {
                let full = format!("{folder}/{viewpoint_file}");
                if let Some((name, data)) = folder_entries.iter().find(|(name, _)| name.eq_ignore_ascii_case(&full)) {
                    let (c, comp) = parse_visualization_info(data)?;
                    camera = c;
                    components = comp;
                    consumed.insert(name.clone());
                }
            }
            let mut snapshot = None;
            if let Some(snapshot_file) = &reference.snapshot_file {
                let full = format!("{folder}/{snapshot_file}");
                if let Some((name, data)) = folder_entries.iter().find(|(name, _)| name.eq_ignore_ascii_case(&full)) {
                    snapshot = Some(data.clone());
                    consumed.insert(name.clone());
                }
            }
            viewpoints.push(OViewpoint { guid: reference.guid.clone(), camera, components, snapshot });
        }
        topic.viewpoints = viewpoints;
        topics.push(topic);
    }

    let parts = entries.iter().filter(|(name, _)| !consumed.contains(name)).map(|(name, data)| ORawPart { name: name.clone(), data: data.clone() }).collect();
    Ok(ODoc { version, topics, parts })
}

/// 🔮️ Re-encodes a full `ODoc` into a real bcfzip.
pub(super) fn encode(doc: &ODoc) -> Result<Vec<u8>, String> {
    let mut entries: Vec<(String, Vec<u8>)> = vec![("bcf.version".to_string(), bcf_version_bytes(&doc.version))];
    for topic in &doc.topics {
        entries.push((format!("{}/markup.bcf", topic.guid), markup_bytes(topic)));
        for viewpoint in &topic.viewpoints {
            entries.push((format!("{}/{}.bcfv", topic.guid, viewpoint.guid), visualization_info_bytes(viewpoint)));
            if let Some(snapshot) = &viewpoint.snapshot {
                entries.push((format!("{}/{}.png", topic.guid, viewpoint.guid), snapshot.clone()));
            }
        }
    }
    for part in &doc.parts {
        entries.push((part.name.clone(), part.data.clone()));
    }
    write_entries(&entries)
}
//#endregion 🔖️Codec

//#region 🔖️JsonValue
pub(super) fn obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

pub(super) fn strings(items: Vec<Json>) -> Vec<String> {
    items.into_iter().filter_map(|item| if let Json::String(text) = item { Some(text) } else { None }).collect()
}

pub(super) fn strings_json(items: &[String]) -> Json {
    Json::Array(items.iter().map(|item| Json::String(item.clone())).collect())
}

fn bytes_of(value: &Json) -> Result<Vec<u8>, String> {
    match value {
        Json::Array(items) => items.iter().map(|item| if let Json::Number(byte) = item { Ok(*byte as u8) } else { Err(format!("a byte array holds only numbers, found {item:?}")) }).collect(),
        other => Err(format!("expected a byte array, found {other:?}")),
    }
}

pub(super) fn bytes_json(bytes: &[u8]) -> Json {
    Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
}

pub(super) fn optional_bytes(value: Option<&Json>) -> Result<Option<Vec<u8>>, String> {
    match value {
        Some(Json::Null) | None => Ok(None),
        Some(bytes) => bytes_of(bytes).map(Some),
    }
}

fn number_field(value: &Json, key: &str) -> f64 {
    match value.get(key) {
        Some(Json::Number(number)) => *number,
        _ => 0.0,
    }
}

fn point_from_json(value: &Json) -> OPoint3 {
    OPoint3 { x: number_field(value, "x"), y: number_field(value, "y"), z: number_field(value, "z") }
}

fn point_to_json(point: &OPoint3) -> Json {
    obj(vec![("x", Json::Number(point.x)), ("y", Json::Number(point.y)), ("z", Json::Number(point.z))])
}

/// 🔎️ `{"kind":"perspective","viewPoint":{x,y,z},"direction":{...},"upVector":{...},
/// "fieldOfView":...}` | `{"kind":"orthogonal",...,"viewToWorldScale":...}` — mirrors
/// `BcfCamera`'s own `#[serde(tag="kind")]` shape field-for-field.
pub(super) fn camera_from_json(value: &Json) -> Result<OCamera, String> {
    let view_point = value.get("viewPoint").map(point_from_json).unwrap_or_default();
    let direction = value.get("direction").map(point_from_json).unwrap_or_default();
    let up_vector = value.get("upVector").map(point_from_json).unwrap_or_default();
    match value.str("kind").as_str() {
        "perspective" => Ok(OCamera::Perspective { view_point, direction, up_vector, field_of_view: number_field(value, "fieldOfView") }),
        "orthogonal" => Ok(OCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale: number_field(value, "viewToWorldScale") }),
        other => Err(format!("unknown camera kind {other:?}")),
    }
}

pub(super) fn camera_to_json(camera: &OCamera) -> Json {
    match camera {
        OCamera::Perspective { view_point, direction, up_vector, field_of_view } => {
            obj(vec![("kind", Json::String("perspective".to_string())), ("viewPoint", point_to_json(view_point)), ("direction", point_to_json(direction)), ("upVector", point_to_json(up_vector)), ("fieldOfView", Json::Number(*field_of_view))])
        }
        OCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => obj(vec![
            ("kind", Json::String("orthogonal".to_string())),
            ("viewPoint", point_to_json(view_point)),
            ("direction", point_to_json(direction)),
            ("upVector", point_to_json(up_vector)),
            ("viewToWorldScale", Json::Number(*view_to_world_scale)),
        ]),
    }
}

/// 🔎️ `{"selection":[guid,...],"visibility":{"defaultVisibility":bool,"exceptions":[guid,...]},
/// "coloring":[{"color":"FFRRGGBB","components":[guid,...]}]}` — mirrors `BcfComponents`.
pub(super) fn components_from_json(value: &Json) -> OComponents {
    let visibility = match value.get("visibility") {
        Some(node) => OVisibility {
            default_visibility: match node.get("defaultVisibility") {
                Some(Json::Bool(flag)) => *flag,
                _ => true,
            },
            exceptions: strings(node.array("exceptions")),
        },
        None => OVisibility { default_visibility: true, exceptions: Vec::new() },
    };
    OComponents { selection: strings(value.array("selection")), visibility, coloring: value.array("coloring").iter().map(|entry| OColoring { color: entry.str("color"), components: strings(entry.array("components")) }).collect() }
}

pub(super) fn components_to_json(components: &OComponents) -> Json {
    obj(vec![
        ("selection", strings_json(&components.selection)),
        ("visibility", obj(vec![("defaultVisibility", Json::Bool(components.visibility.default_visibility)), ("exceptions", strings_json(&components.visibility.exceptions))])),
        ("coloring", Json::Array(components.coloring.iter().map(|coloring| obj(vec![("color", Json::String(coloring.color.clone())), ("components", strings_json(&coloring.components))])).collect())),
    ])
}

pub(super) fn comment_from_json(value: &Json) -> OComment {
    OComment {
        guid: value.str("guid"),
        date: value.str("date"),
        author: value.str("author"),
        text: value.str("text"),
        viewpoint_ref: match value.get("viewpointRef") {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        },
    }
}

pub(super) fn comment_to_json(comment: &OComment) -> Json {
    obj(vec![
        ("guid", Json::String(comment.guid.clone())),
        ("date", Json::String(comment.date.clone())),
        ("author", Json::String(comment.author.clone())),
        ("text", Json::String(comment.text.clone())),
        (
            "viewpointRef",
            match &comment.viewpoint_ref {
                Some(reference) => Json::String(reference.clone()),
                None => Json::Null,
            },
        ),
    ])
}

pub(super) fn viewpoint_from_json(value: &Json) -> Result<OViewpoint, String> {
    let camera = match value.get("camera") {
        Some(Json::Null) | None => None,
        Some(node) => Some(camera_from_json(node)?),
    };
    let components = match value.get("components") {
        Some(Json::Null) | None => None,
        Some(node) => Some(components_from_json(node)),
    };
    Ok(OViewpoint { guid: value.str("guid"), camera, components, snapshot: optional_bytes(value.get("snapshot"))? })
}

pub(super) fn viewpoint_to_json(viewpoint: &OViewpoint) -> Json {
    obj(vec![
        ("guid", Json::String(viewpoint.guid.clone())),
        (
            "camera",
            match &viewpoint.camera {
                Some(camera) => camera_to_json(camera),
                None => Json::Null,
            },
        ),
        (
            "components",
            match &viewpoint.components {
                Some(components) => components_to_json(components),
                None => Json::Null,
            },
        ),
        (
            "snapshot",
            match &viewpoint.snapshot {
                Some(bytes) => bytes_json(bytes),
                None => Json::Null,
            },
        ),
    ])
}

pub(super) fn topic_from_json(value: &Json) -> Result<OTopic, String> {
    Ok(OTopic {
        guid: value.str("guid"),
        title: value.str("title"),
        description: value.str("description"),
        status: value.str("status"),
        priority: value.str("priority"),
        labels: strings(value.array("labels")),
        creation_date: value.str("creationDate"),
        creation_author: value.str("creationAuthor"),
        comments: value.array("comments").iter().map(comment_from_json).collect(),
        viewpoints: value.array("viewpoints").iter().map(viewpoint_from_json).collect::<Result<_, _>>()?,
    })
}

pub(super) fn topic_to_json(topic: &OTopic) -> Json {
    obj(vec![
        ("guid", Json::String(topic.guid.clone())),
        ("title", Json::String(topic.title.clone())),
        ("description", Json::String(topic.description.clone())),
        ("status", Json::String(topic.status.clone())),
        ("priority", Json::String(topic.priority.clone())),
        ("labels", strings_json(&topic.labels)),
        ("creationDate", Json::String(topic.creation_date.clone())),
        ("creationAuthor", Json::String(topic.creation_author.clone())),
        ("comments", Json::Array(topic.comments.iter().map(comment_to_json).collect())),
        ("viewpoints", Json::Array(topic.viewpoints.iter().map(viewpoint_to_json).collect())),
    ])
}

fn doc_from_json(value: &Json) -> Result<ODoc, String> {
    Ok(ODoc {
        version: value.str("version"),
        topics: value.array("topics").iter().map(topic_from_json).collect::<Result<_, _>>()?,
        parts: value
            .array("parts")
            .iter()
            .map(|entry| Ok(ORawPart { name: entry.str("name"), data: optional_bytes(entry.get("data"))?.unwrap_or_default() }))
            .collect::<Result<_, String>>()?,
    })
}

/// 📸️ The whole document as the `BcfSnapshot` wire.
fn doc_to_json(doc: &ODoc) -> Json {
    obj(vec![
        ("schema", Json::String("stdio.bcf".to_string())),
        ("version", Json::String(doc.version.clone())),
        ("topics", Json::Array(doc.topics.iter().map(topic_to_json).collect())),
        ("parts", Json::Array(doc.parts.iter().map(|part| obj(vec![("name", Json::String(part.name.clone())), ("data", bytes_json(&part.data))])).collect())),
    ])
}
//#endregion 🔖️JsonValue
