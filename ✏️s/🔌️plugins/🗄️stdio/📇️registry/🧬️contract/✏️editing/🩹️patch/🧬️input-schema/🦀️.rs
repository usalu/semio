//! 🧬️ The patch input schema typed by the snapshot sub-schema a pointer addresses.

use super::*;

//#region 🧬️InputSchema
/// 📚️ Schema documents by `$id`, resolved once through the runtime's input-schema resolver.
fn schema_document(id: &str) -> Option<Arc<DslValue>> {
    static DOCUMENTS: OnceLock<Mutex<HashMap<String, Arc<DslValue>>>> = OnceLock::new();
    let documents = DOCUMENTS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(document) = documents.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).get(id) {
        return Some(Arc::clone(document));
    }
    let document = Arc::new(semio_framework_plugin::registered_input_schema_document(id)?);
    documents.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(id.to_string(), Arc::clone(&document));
    Some(document)
}

/// 🔍️ Resolves a schema document by its `$id`.
pub type SnapshotSchemaResolver<'r> = &'r dyn Fn(&str) -> Option<Arc<DslValue>>;

/// 🧭️ One place in a schema document: the document's `$id` and the JSON Pointer of the node inside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotSchemaLocation {
    pub document: String,
    pub pointer: String,
}

impl SnapshotSchemaLocation {
    /// 🔗️ The `$ref` that names this location.
    pub fn reference(&self) -> String {
        if self.pointer.is_empty() {
            self.document.clone()
        } else {
            format!("{}#{}", self.document, self.pointer)
        }
    }

    fn child(&self, suffix: &str) -> Self {
        Self { document: self.document.clone(), pointer: format!("{}/{suffix}", self.pointer) }
    }
}

fn escape(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

fn node<'d>(document: &'d DslValue, pointer: &str) -> Option<&'d DslValue> {
    let mut current = document;
    for raw in pointer.split('/').skip(1) {
        let segment = raw.replace("~1", "/").replace("~0", "~");
        current = match current {
            DslValue::Object(_) => current.get(&segment)?,
            DslValue::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current)
}

/// 🔗️ `location` with every `$ref` it starts with followed (bounded), and the document the result lives in.
fn followed(location: &SnapshotSchemaLocation, resolve: SnapshotSchemaResolver<'_>) -> Option<(SnapshotSchemaLocation, Arc<DslValue>)> {
    let mut current = location.clone();
    for _ in 0..SNAPSHOT_SCHEMA_REF_HOPS {
        let document = resolve(&current.document)?;
        let Some(reference) = node(&document, &current.pointer)?.get("$ref").and_then(DslValue::as_str).map(str::to_string) else { return Some((current, document)) };
        let (id, fragment) = reference.split_once('#').unwrap_or((reference.as_str(), ""));
        current = SnapshotSchemaLocation { document: if id.is_empty() { current.document.clone() } else { id.to_string() }, pointer: fragment.to_string() };
    }
    None
}

fn is_index(segment: &str) -> bool {
    segment == "-" || (!segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit()))
}

/// 🪜️ Every location that may describe the instance member `segment` of a value described by `location`: a declared
/// property, the items of an array, the additional properties of a map — through every union branch that admits it.
fn member_locations(location: &SnapshotSchemaLocation, segment: &str, resolve: SnapshotSchemaResolver<'_>, depth: usize) -> Vec<SnapshotSchemaLocation> {
    let Some((resolved, document)) = followed(location, resolve).filter(|_| depth < SNAPSHOT_SCHEMA_REF_HOPS) else { return Vec::new() };
    let Some(schema) = node(&document, &resolved.pointer) else { return Vec::new() };
    let mut found = Vec::new();
    for union in ["oneOf", "anyOf", "allOf"] {
        if let Some(DslValue::Array(members)) = schema.get(union) {
            for position in 0..members.len() {
                found.extend(member_locations(&resolved.child(&format!("{union}/{position}")), segment, resolve, depth + 1));
            }
        }
    }
    if schema.get("properties").and_then(|properties| properties.get(segment)).is_some() {
        found.push(resolved.child(&format!("properties/{}", escape(segment))));
    } else if is_index(segment) && schema.get("items").is_some() {
        match schema.get("items") {
            Some(DslValue::Array(items)) => match segment.parse::<usize>().ok().filter(|position| *position < items.len()) {
                Some(position) => found.push(resolved.child(&format!("items/{position}"))),
                None if matches!(schema.get("additionalItems"), Some(DslValue::Object(_))) => found.push(resolved.child("additionalItems")),
                None => {}
            },
            Some(DslValue::Object(_)) => found.push(resolved.child("items")),
            _ => {}
        }
    } else if matches!(schema.get("additionalProperties"), Some(DslValue::Object(_))) {
        found.push(resolved.child("additionalProperties"));
    }
    found
}

/// 🧭️ The location of the sub-schema of `document` (a snapshot schema `$id`) that describes the instance at `segments`,
/// read from the schema alone: a union passes on the branches that admit the next segment, and a location is answered only
/// when every admitting branch resolves to the same schema. `None` when a document does not resolve, a segment is not
/// admitted, or the location is ambiguous.
pub fn snapshot_schema_location(document: &str, segments: &[String], resolve: SnapshotSchemaResolver<'_>) -> Option<SnapshotSchemaLocation> {
    let mut location = SnapshotSchemaLocation { document: document.to_string(), pointer: String::new() };
    for segment in segments {
        let candidates = member_locations(&location, segment, resolve, 0);
        let (first, document) = followed(candidates.first()?, resolve)?;
        let target = node(&document, &first.pointer)?;
        for candidate in candidates.iter().skip(1) {
            let (other, other_document) = followed(candidate, resolve)?;
            if other != first && node(&other_document, &other.pointer)? != target {
                return None;
            }
        }
        location = candidates.into_iter().next()?;
    }
    Some(location)
}

fn hidden_string() -> DslValue {
    DslValue::Object(vec![("type".into(), DslValue::String("string".into())), ("x-semio-ui".into(), DslValue::Object(vec![("widget".into(), DslValue::String("hidden".into()))]))])
}

fn hidden_index() -> DslValue {
    DslValue::Object(vec![
        ("type".into(), DslValue::String("integer".into())),
        ("minimum".into(), DslValue::Number(semio_framework_value::Number::UInt(0))),
        ("maximum".into(), DslValue::Number(semio_framework_value::Number::UInt(SNAPSHOT_PATCH_MAX_INDEX))),
        ("x-semio-ui".into(), DslValue::Object(vec![("widget".into(), DslValue::String("hidden".into()))])),
    ])
}

/// 🧬️ The input schema of `patch` against the snapshot schema whose `$id` is `snapshot_schema`, resolving documents
/// through `resolve`, as a leaf payload schema (`{patch: …}`): the operation pinned, its pointers and position hidden, a
/// written value typed by the snapshot sub-schema its pointer addresses (a `$ref` to that location, so its own UI facts
/// apply) and a rename's new key as text, declaring the inverse rows every `patch-snapshot` leaf answers
/// (`x-semio-inverse-rows.bounded`, [`SNAPSHOT_PATCH_MAX_INVERSE_PARTS`]). `None` when that location does not resolve or
/// is ambiguous.
pub fn snapshot_patch_input_schema_text(snapshot_schema: &str, patch: &SnapshotPatch, resolve: SnapshotSchemaResolver<'_>) -> Option<String> {
    let operation = ("operation".to_string(), DslValue::Object(vec![("const".into(), DslValue::String(patch.operation().into()))]));
    let (mut properties, mut required) = (vec![operation], vec!["operation", "path"]);
    match patch {
        SnapshotPatch::Set { path, .. } | SnapshotPatch::Insert { path, .. } => {
            let location = snapshot_schema_location(snapshot_schema, &decode_pointer(path).ok()?, resolve)?;
            properties.push(("path".into(), hidden_string()));
            properties.push(("value".into(), DslValue::Object(vec![("$ref".into(), DslValue::String(location.reference()))])));
            required.push("value");
            if let SnapshotPatch::Insert { index: Some(_), .. } = patch {
                properties.push(("index".into(), hidden_index()));
            }
        }
        SnapshotPatch::Remove { .. } => properties.push(("path".into(), hidden_string())),
        SnapshotPatch::Move { index, .. } => {
            properties.push(("from".into(), hidden_string()));
            properties.push(("path".into(), hidden_string()));
            required.push("from");
            if index.is_some() {
                properties.push(("index".into(), hidden_index()));
            }
        }
        SnapshotPatch::Rename { .. } => {
            properties.push(("path".into(), hidden_string()));
            properties.push(("key".into(), DslValue::Object(vec![("type".into(), DslValue::String("string".into()))])));
            required.push("key");
        }
        SnapshotPatch::Splice { .. } => return None,
    }
    let strings = |names: &[&str]| DslValue::Array(names.iter().map(|name| DslValue::String((*name).into())).collect());
    let label = DslValue::Object(vec![("en".into(), DslValue::String("Snapshot edit".into())), ("de".into(), DslValue::String("Änderung der Momentaufnahme".into()))]);
    let body = DslValue::Object(vec![
        ("type".into(), DslValue::String("object".into())),
        ("additionalProperties".into(), DslValue::Bool(false)),
        ("required".into(), strings(&required)),
        ("properties".into(), DslValue::Object(properties)),
        ("x-semio-ui".into(), DslValue::Object(vec![("label".into(), label)])),
    ]);
    let schema = DslValue::Object(vec![
        ("$schema".into(), DslValue::String("http://json-schema.org/draft-07/schema#".into())),
        ("title".into(), DslValue::String("PatchSnapshot".into())),
        ("x-semio-inverse-rows".into(), DslValue::Object(vec![("bounded".into(), DslValue::Number(semio_framework_value::Number::UInt(SNAPSHOT_PATCH_MAX_INVERSE_PARTS as u64)))])),
        ("type".into(), DslValue::String("object".into())),
        ("additionalProperties".into(), DslValue::Bool(false)),
        ("required".into(), strings(&["patch"])),
        ("properties".into(), DslValue::Object(vec![("patch".into(), body)])),
    ]);
    Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&schema)))
}

/// 🧬️ [`snapshot_patch_input_schema_text`] over the runtime's registered schema documents, admitted by the input reader
/// and interned (one leaked text per distinct schema, bounded by the snapshot schemas' locations). `None` sends the
/// caller to the leaf's own structural payload schema.
pub fn snapshot_patch_input_schema(snapshot_schema: &str, patch: &SnapshotPatch) -> Option<&'static str> {
    static SCHEMAS: OnceLock<Mutex<HashMap<String, Option<&'static str>>>> = OnceLock::new();
    let text = snapshot_patch_input_schema_text(snapshot_schema, patch, &schema_document)?;
    let schemas = SCHEMAS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(known) = schemas.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).get(&text) {
        return *known;
    }
    let interned = semio_framework_plugin::mutation_input_defs(&text, &semio_framework_plugin::registered_input_schema_document).is_ok().then(|| &*Box::leak(text.clone().into_boxed_str()));
    schemas.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(text, interned);
    interned
}
//#endregion 🧬️InputSchema
