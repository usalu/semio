//! 🔡️ Mutation input schema reader: input definitions, audits and instances derived from JSON schema documents.

use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::Number;
use semio_framework_value::ToValue;
use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::OnceLock;
use semio_framework_manifest_arg_schema::{ActionArgDef, ActionArgOption, ArgFormat, ArgPresentation, ArgSchema, NumberScale, OptionSource, ReferenceIdType, SnapSource};

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️MutationInputs

/// 🧭️ Resolves a cross-document `$ref`: the schema document whose `$id` is `id` (the reference without its fragment).
pub trait InputSchemaResolver {
    fn resolve(&self, id: &str) -> Option<DslValue>;
}

impl<F: Fn(&str) -> Option<DslValue>> InputSchemaResolver for F {
    fn resolve(&self, id: &str) -> Option<DslValue> {
        self(id)
    }
}

/// 🚫️ The class of a [`InputSchemaError`], shared verbatim with the TypeScript twin and the lint `schema-mutation-input-ui`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum InputSchemaErrorCode {
    Malformed,
    RefUnresolved,
    UiInvalid,
    WidgetIncompatible,
    LabelMissing,
    OptionLabelMissing,
    LocaleMissing,
    WordOnlyFloat,
}

/// 🚫️ Why a mutation payload schema yields no input descriptors: the `code`, the RFC 6901 `pointer` of the input
/// in the payload (`""` for the payload itself, `-` for "every array item") and a human `detail`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct InputSchemaError {
    pub code: InputSchemaErrorCode,
    pub pointer: String,
    pub detail: String,
}

impl std::fmt::Display for InputSchemaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?} at {:?}: {}", self.code, self.pointer, self.detail)
    }
}

impl std::error::Error for InputSchemaError {}

/// 📚️ The framework input-label glossary (`🔣️input-labels.json`, schema `InputLabelGlossary`): a label in every
/// locale for the most frequent mutation input names, the fallback when an input carries no `x-semio-ui.label`.
const INPUT_LABEL_GLOSSARY_JSON: &str = include_str!("../🔣️input-labels.json");

/// 📚️ The parsed [`INPUT_LABEL_GLOSSARY_JSON`], field name → label.
pub fn input_label_glossary() -> &'static BTreeMap<String, LocalizedLabel> {
    static GLOSSARY: std::sync::OnceLock<BTreeMap<String, LocalizedLabel>> = std::sync::OnceLock::new();
    GLOSSARY.get_or_init(|| {
        let document = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(INPUT_LABEL_GLOSSARY_JSON, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the input label glossary is JSON"));
        let labels = document.get("labels").and_then(DslValue::as_object).expect("the input label glossary carries labels");
        labels.iter().map(|(name, label)| (name.clone(), input_localized_text(label).expect("every glossary label names every locale"))).collect()
    })
}

/// 🧬️ Reads a mutation leaf's payload JSON Schema (`MutationLeaf::PAYLOAD_SCHEMA`) into one [`ActionArgDef`] per
/// input, in `properties` order: `$ref`s resolve against the document itself (`#/$defs/…`) or, for another `$id`,
/// through `resolver`; hard bounds come from the standard keywords, UI facts from `x-semio-ui` (an outer annotation
/// overrides the one on the `$ref` target key by key); a `const` or `role: discriminator` input is skipped; objects
/// recurse into their fields. Without `x-semio-ui` the widget is inferred exactly as [`ActionArgDef::control`]
/// derives it, a string or string array named `<kind>Id(s)` becomes a [`ArgSchema::Reference`] of that kind, and
/// the label comes from [`input_label_glossary`] — an input with no label in every locale is an error, never a
/// fallback. TypeScript twin: `mutationInputDefs` (`🛂️manifest/🟦️.ts`); fixture `🧫️fixtures/🧫️mutation-inputs`.
pub fn mutation_input_defs(schema_json: &str, resolver: &dyn InputSchemaResolver) -> Result<Vec<ActionArgDef>, InputSchemaError> {
    InputSchemaReader::parse(schema_json, resolver, false)?.inputs()
}

/// 🧺️ Every finding of a mutation leaf payload schema at once: the collecting twin of the fail-fast
/// [`mutation_input_defs`] a runtime uses. Each refused input (at every nested pointer) is recorded and reading goes on
/// past it — a label, description, group, order, role or widget fault keeps the input, a value fault drops its schema —
/// so `findings` names every pointer an author must fix (first occurrence of each distinct finding, in reading order;
/// its first entry is the error the fail-fast reader returns) and `inputs` holds what reads despite them. TypeScript
/// twin: `mutationInputAudit`.
#[derive(Clone, Debug, PartialEq)]
pub struct InputSchemaAudit {
    pub inputs: Vec<ActionArgDef>,
    pub findings: Vec<InputSchemaError>,
}

/// 🧺️ Reads `schema_json` in collecting mode — see [`InputSchemaAudit`].
pub fn mutation_input_audit(schema_json: &str, resolver: &dyn InputSchemaResolver) -> InputSchemaAudit {
    let reader = match InputSchemaReader::parse(schema_json, resolver, true) {
        Ok(reader) => reader,
        Err(error) => return InputSchemaAudit { inputs: Vec::new(), findings: vec![error] },
    };
    let read = reader.inputs();
    let mut findings = reader.findings.map(std::cell::RefCell::into_inner).unwrap_or_default();
    let inputs = match read {
        Ok(inputs) => inputs,
        Err(error) => {
            findings.push(error);
            Vec::new()
        }
    };
    let mut distinct: Vec<InputSchemaError> = Vec::with_capacity(findings.len());
    for finding in findings {
        if !distinct.contains(&finding) {
            distinct.push(finding);
        }
    }
    InputSchemaAudit { inputs, findings: distinct }
}

/// 🧭️ The [`InputSchemaResolver`] over every JSON Schema document registered in the OS-wide `schema://` export registry
/// (artifact facets and named scope exports) and every document a published mutation leaf references
/// (`semio_framework_schema_registry::registered_referenced_schema_documents`), and the framework's own shared documents
/// ([`FRAMEWORK_INPUT_SCHEMA_DOCUMENTS`], held without any registration): the document whose `$id` is `id`, or `None`
/// when nobody published it.
pub fn registered_input_schema_document(id: &str) -> Option<DslValue> {
    let mut texts: Vec<&'static str> = semio_framework_schema_registry::with_schema_export_registry(|registry| {
        registry.entries().filter(|entry| entry.format == semio_framework_schema_registry::SchemaFormat::JsonSchema).filter_map(|entry| registry.resolve(entry.scope, entry.export, entry.format).ok()).filter(|text| text.contains(id)).collect()
    });
    texts.extend(semio_framework_schema_registry::registered_referenced_schema_documents().into_iter().filter(|text| text.contains(id)));
    texts.extend(FRAMEWORK_INPUT_SCHEMA_DOCUMENTS.into_iter().filter(|text| text.contains(id)));
    texts.into_iter().find_map(|text| {
        let document = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?);
        (document.get("$id").and_then(DslValue::as_str) == Some(id)).then_some(document)
    })
}

/// 🧾️ The instance a leaf payload schema validates for a candidate `payload` (a `Mutation::payload_value`): the payload
/// with every root `const` property the schema declares (the aggregate discriminator `payload_value` strips, e.g.
/// `"mutation": "moveNode"`) spliced in where absent — hand the result to `OwnedJsonSchemaValidator`. TypeScript twin:
/// `mutationInputInstance`.
pub fn mutation_input_instance(schema_json: &str, resolver: &dyn InputSchemaResolver, payload: &DslValue) -> Result<DslValue, InputSchemaError> {
    let reader = InputSchemaReader::parse(schema_json, resolver, false)?;
    let mut root = reader.resolve(None, reader.root.clone(), "")?;
    let mut entries = payload.as_object().map(<[(String, DslValue)]>::to_vec).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, "", "a mutation payload is an object"))?;
    if root.node.get("properties").is_none() && root.node.get("allOf").is_none() && input_union(&root.node).is_some() {
        let (key, variants) = reader.variants(&root, "")?;
        let chosen = payload.get(&key).and_then(DslValue::as_str).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, "", format!("a union payload names its variant in {key}")))?;
        root = variants.into_iter().find(|variant| variant.value == chosen).map(|variant| variant.member).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, "", format!("{chosen} is no variant of this payload union")))?;
    }
    let mut members = Vec::new();
    reader.members(&root, "", 0, &mut members)?;
    for (document, node) in &members {
        for (key, property) in node.get("properties").and_then(DslValue::as_object).unwrap_or_default() {
            if entries.iter().any(|(name, _)| name == key) {
                continue;
            }
            if let Some(value) = reader.resolve(document.clone(), property.clone(), &input_pointer("", key))?.node.get("const") {
                entries.push((key.clone(), value.clone()));
            }
        }
    }
    Ok(DslValue::Object(entries))
}

/// 🔀️ The `oneOf`/`anyOf` branches of a schema node, if it is a union.
fn input_union(node: &DslValue) -> Option<&[DslValue]> {
    node.get("oneOf").or_else(|| node.get("anyOf")).and_then(DslValue::as_array)
}

/// 🔢️ The framework value schema, whose `$defs/Binary64` (the exact binary64 word) and `$defs/Binary64Transport` (that word or a
/// plain number) define the numeric transport the reader edits as a number wherever a carrier references or restates it.
const INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON: &str = include_str!("../../🌱️value/🧬️schema/🔣️.json");

/// 🪣️ The shared schema documents of the framework's own namespace an input schema of any plugin may `$ref`:
/// [`registered_input_schema_document`] always holds them, so publishing them is never a plugin's duty (design §23).
/// Today the framework value schema (`framework/value/schema.json`); the OS adds its own at every app construction.
pub const FRAMEWORK_INPUT_SCHEMA_DOCUMENTS: [&str; 1] = [INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON];

/// 🏷️ Schema keys that annotate without constraining; a shape comparison ignores them (and every `x-` key).
const INPUT_SHAPE_ANNOTATIONS: [&str; 6] = ["title", "description", "$comment", "examples", "default", "format"];

/// 🧬️ The constraint shape of a schema node: annotations dropped, object keys sorted, `required` sorted — equal shapes
/// validate the same instances.
fn input_shape(node: &DslValue) -> DslValue {
    match node {
        DslValue::Object(entries) => {
            let mut kept: Vec<(String, DslValue)> = entries
                .iter()
                .filter(|(key, _)| !key.starts_with("x-") && !INPUT_SHAPE_ANNOTATIONS.contains(&key.as_str()))
                .map(|(key, value)| match (key.as_str(), value) {
                    ("required", DslValue::Array(names)) => {
                        let mut names = names.clone();
                        names.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
                        (key.clone(), DslValue::Array(names))
                    }
                    _ => (key.clone(), input_shape(value)),
                })
                .collect();
            kept.sort_by(|left, right| left.0.cmp(&right.0));
            DslValue::Object(kept)
        }
        DslValue::Array(items) => DslValue::Array(items.iter().map(input_shape).collect()),
        other => other.clone(),
    }
}

/// 🔢️ The shapes of the framework's numeric transport: the binary64 word and the plain-number branch of the transport.
fn input_numeric_transport() -> &'static (DslValue, DslValue) {
    static SHAPES: std::sync::OnceLock<(DslValue, DslValue)> = std::sync::OnceLock::new();
    SHAPES.get_or_init(|| {
        let schema = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the framework value schema is JSON"));
        let definition = |name: &str| schema.get("$defs").and_then(|definitions| definitions.get(name)).cloned().unwrap_or_else(|| panic!("the framework value schema defines {name}"));
        let number = input_union(&definition("Binary64Transport")).and_then(|branches| branches.iter().find(|branch| branch.get("$ref").is_none())).map(input_shape).expect("the framework numeric transport has a plain-number branch");
        (input_shape(&definition("Binary64")), number)
    })
}

/// 🔀️ One variant of a discriminated payload union: the `const` its discriminator pins, the resolved member, and that
/// member's resolved discriminator property.
struct InputVariant {
    value: String,
    member: ResolvedInput,
    discriminator: ResolvedInput,
}

const INPUT_UI_KEYS: [&str; 19] = ["widget", "role", "label", "description", "step", "precision", "softMin", "softMax", "snaps", "snapSource", "optionSource", "unit", "displayUnit", "displayFactor", "scale", "group", "order", "options", "ref"];

const INPUT_UI_NUMBER_KEYS: [&str; 9] = ["step", "precision", "softMin", "softMax", "snaps", "snapSource", "displayUnit", "displayFactor", "scale"];

/// 🧭️ The number facets a vector shares across its components (every [`INPUT_UI_NUMBER_KEYS`] but the slider travel keys).
const INPUT_UI_VECTOR_KEYS: [&str; 6] = ["step", "precision", "snaps", "snapSource", "displayUnit", "displayFactor"];

const INPUT_REF_DEPTH: usize = 32;

fn input_error(code: InputSchemaErrorCode, pointer: &str, detail: impl Into<String>) -> InputSchemaError {
    InputSchemaError { code, pointer: pointer.to_string(), detail: detail.into() }
}

fn input_pointer(parent: &str, key: &str) -> String {
    format!("{parent}/{}", key.replace('~', "~0").replace('/', "~1"))
}

/// 🌐️ A `{<locale>: text}` map naming every locale, or a `{<terminology>: {<locale>: text}}` matrix naming every cell.
fn input_localized_text(value: &DslValue) -> Result<LocalizedLabel, InputSchemaErrorCode> {
    let entries = value.as_object().ok_or(InputSchemaErrorCode::UiInvalid)?;
    let cell = |map: &DslValue, locale: Locale| map.get(locale.as_str()).and_then(DslValue::as_str).filter(|text| !text.is_empty()).map(str::to_string);
    if entries.iter().all(|(key, _)| Locale::parse(key).is_some()) {
        if Locale::ALL.iter().any(|locale| cell(value, *locale).is_none()) {
            return Err(InputSchemaErrorCode::LocaleMissing);
        }
        return Ok(LocalizedLabel::from_fn(|_, locale| cell(value, locale).unwrap_or_default()));
    }
    if entries.iter().all(|(key, map)| Terminology::parse(key).is_some() && map.as_object().is_some_and(|cells| cells.iter().all(|(locale, _)| Locale::parse(locale).is_some()))) {
        let complete = Terminology::ALL.iter().all(|terminology| value.get(terminology.as_str()).is_some_and(|map| Locale::ALL.iter().all(|locale| cell(map, *locale).is_some())));
        if !complete {
            return Err(InputSchemaErrorCode::LocaleMissing);
        }
        return Ok(LocalizedLabel::from_fn(|terminology, locale| value.get(terminology.as_str()).and_then(|map| cell(map, locale)).unwrap_or_default()));
    }
    Err(InputSchemaErrorCode::UiInvalid)
}

/// 🔤️ The single non-null JSON type a schema node declares (`"object"` for bare `properties`, `"array"` for bare
/// `items`, `"string"` for an all-string `enum`), or `None` when it names several or none.
fn input_type(node: &DslValue) -> Option<&'static str> {
    const TYPES: [&str; 6] = ["string", "integer", "number", "boolean", "object", "array"];
    let named: Vec<&str> = match node.get("type") {
        Some(DslValue::String(name)) => vec![name.as_str()],
        Some(DslValue::Array(names)) => names.iter().filter_map(DslValue::as_str).filter(|name| *name != "null").collect(),
        _ if node.get("properties").is_some() => vec!["object"],
        _ if node.get("items").is_some() => vec!["array"],
        _ if node.get("enum").and_then(DslValue::as_array).is_some_and(|values| !values.is_empty() && values.iter().all(|value| value.as_str().is_some())) => vec!["string"],
        _ => Vec::new(),
    };
    match named.as_slice() {
        [name] => TYPES.iter().find(|candidate| *candidate == name).copied(),
        _ => None,
    }
}

/// 🎯️ `<kind>Id` / `<kind>_id` (a string) or `<kind>Ids` / `<kind>_ids` (an array) names a reference to `kind`;
/// a leading `new` of a replacement value (`newZoneId`) is not part of the kind.
fn input_inferred_reference_kind(key: &str, many: bool) -> Option<String> {
    let stem = if many { key.strip_suffix("Ids").or_else(|| key.strip_suffix("_ids")) } else { key.strip_suffix("Id").or_else(|| key.strip_suffix("_id")) }?;
    let stem = match stem.strip_prefix("new") {
        Some(rest) if rest.starts_with(|first: char| first.is_ascii_uppercase()) => rest,
        Some("") => return None,
        _ => stem,
    };
    let mut characters = stem.chars();
    let first = characters.next().filter(char::is_ascii_alphabetic)?;
    if !characters.clone().all(|character| character.is_ascii_alphanumeric() || character == '_') {
        return None;
    }
    Some(first.to_ascii_lowercase().to_string() + characters.as_str())
}

/// 🧬️ A schema node with every `$ref` hop and nullable union resolved, the document it lives in (`None` = the leaf
/// itself), and the merged `x-semio-ui` annotation (outer keys win).
struct ResolvedInput {
    document: Option<String>,
    node: DslValue,
    ui: Vec<(String, DslValue)>,
    refs: Vec<String>,
    nullable: bool,
}

impl ResolvedInput {
    fn ui(&self, key: &str) -> Option<&DslValue> {
        self.ui.iter().find(|(name, _)| name == key).map(|(_, value)| value)
    }

    fn ui_str(&self, key: &str) -> Option<&str> {
        self.ui(key).and_then(DslValue::as_str)
    }
}

struct InputSchemaReader<'r> {
    root: DslValue,
    resolver: &'r dyn InputSchemaResolver,
    documents: std::cell::RefCell<BTreeMap<String, DslValue>>,
    active: std::cell::RefCell<Vec<String>>,
    findings: Option<std::cell::RefCell<Vec<InputSchemaError>>>,
}

impl<'r> InputSchemaReader<'r> {
    fn parse(schema_json: &str, resolver: &'r dyn InputSchemaResolver, collect: bool) -> Result<Self, InputSchemaError> {
        let root = semio_framework_pack_json::parse(schema_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| input_error(InputSchemaErrorCode::Malformed, "", error.to_string()))?;
        Ok(Self {
            root: semio_framework_pack_json::to_dsl_value(&root),
            resolver,
            documents: std::cell::RefCell::new(BTreeMap::new()),
            active: std::cell::RefCell::new(vec!["#".to_string()]),
            findings: collect.then(|| std::cell::RefCell::new(Vec::new())),
        })
    }

    /// 🧺️ In collecting mode records `result`'s error and goes on with `fallback`; the fail-fast mode passes it through.
    fn recover<T>(&self, result: Result<T, InputSchemaError>, fallback: impl FnOnce() -> T) -> Result<T, InputSchemaError> {
        match (result, &self.findings) {
            (Err(error), Some(findings)) => {
                findings.borrow_mut().push(error);
                Ok(fallback())
            }
            (result, _) => result,
        }
    }

    /// 🧬️ Every input of the payload: a discriminated union's selector and variant fields, else the object's fields.
    fn inputs(&self) -> Result<Vec<ActionArgDef>, InputSchemaError> {
        let payload = self.resolve(None, self.root.clone(), "")?;
        if payload.node.get("properties").is_none() && payload.node.get("allOf").is_none() {
            if input_union(&payload.node).is_some() {
                return self.variant_inputs(&payload, "");
            }
            return match input_type(&payload.node) {
                Some("object") | None => Ok(Vec::new()),
                _ => Err(input_error(InputSchemaErrorCode::Malformed, "", "a mutation payload schema describes an object")),
            };
        }
        self.fields(&payload, "")
    }

    /// ♾️ Expands `node` unless a `$ref` it passed through is already being expanded on this path: a recursive schema then
    /// yields a structured value (`ArgSchema::Any`), edited as a whole and validated by the leaf schema, never an endless tree.
    fn guarded(&self, node: &ResolvedInput, expand: impl FnOnce() -> Result<ArgSchema, InputSchemaError>) -> Result<ArgSchema, InputSchemaError> {
        if node.refs.iter().any(|key| self.active.borrow().contains(key)) {
            return Ok(ArgSchema::Any);
        }
        let mark = self.active.borrow().len();
        self.active.borrow_mut().extend(node.refs.iter().cloned());
        let expanded = expand();
        self.active.borrow_mut().truncate(mark);
        expanded
    }

    fn target(&self, document: &Option<String>, reference: &str, pointer: &str) -> Result<(Option<String>, DslValue), InputSchemaError> {
        let unresolved = |detail: String| input_error(InputSchemaErrorCode::RefUnresolved, pointer, detail);
        let (id, fragment) = reference.split_once('#').unwrap_or((reference, ""));
        let document = if id.is_empty() { document.clone() } else { Some(id.to_string()) };
        let fragment_path: Vec<String> = fragment.split('/').skip(1).map(|segment| segment.replace("~1", "/").replace("~0", "~")).collect();
        let walk = |start: &DslValue| -> Option<DslValue> {
            let mut current = start;
            for segment in &fragment_path {
                current = match current {
                    DslValue::Object(_) => current.get(segment)?,
                    DslValue::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
                    _ => return None,
                };
            }
            Some(current.clone())
        };
        let found = match &document {
            None => walk(&self.root),
            Some(id) => {
                if !self.documents.borrow().contains_key(id) {
                    let fetched = self.resolver.resolve(id).ok_or_else(|| unresolved(format!("no schema document has $id {id}")))?;
                    self.documents.borrow_mut().insert(id.clone(), fetched);
                }
                walk(&self.documents.borrow()[id])
            }
        };
        found.map(|node| (document, node)).ok_or_else(|| unresolved(format!("{reference} lands on nothing")))
    }

    fn resolve(&self, document: Option<String>, node: DslValue, pointer: &str) -> Result<ResolvedInput, InputSchemaError> {
        let mut resolved = ResolvedInput { document, node, ui: Vec::new(), refs: Vec::new(), nullable: false };
        for _ in 0..INPUT_REF_DEPTH {
            if let Some(annotation) = resolved.node.get("x-semio-ui") {
                let entries = self.recover(annotation.as_object().ok_or_else(|| input_error(InputSchemaErrorCode::UiInvalid, pointer, "x-semio-ui is an object")), Default::default)?;
                for (key, value) in entries {
                    if !INPUT_UI_KEYS.contains(&key.as_str()) {
                        self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("x-semio-ui carries the undeclared key {key}"))), || ())?;
                        continue;
                    }
                    if !resolved.ui.iter().any(|(name, _)| name == key) {
                        resolved.ui.push((key.clone(), value.clone()));
                    }
                }
            }
            if let Some(reference) = resolved.node.get("$ref").and_then(DslValue::as_str).map(str::to_string) {
                let (document, node) = self.target(&resolved.document, &reference, pointer)?;
                resolved.refs.push(format!("{}#{}", document.as_deref().unwrap_or_default(), reference.split_once('#').map_or("", |(_, fragment)| fragment)));
                resolved.document = document;
                resolved.node = node;
                continue;
            }
            let union = resolved.node.get("oneOf").or_else(|| resolved.node.get("anyOf")).and_then(DslValue::as_array).map(<[DslValue]>::to_vec);
            if let Some(branches) = union {
                let null = |branch: &DslValue| branch.get("type").and_then(DslValue::as_str) == Some("null") && branch.as_object().is_some_and(|entries| entries.len() == 1);
                let concrete: Vec<&DslValue> = branches.iter().filter(|branch| !null(branch)).collect();
                if concrete.len() == 1 && concrete.len() < branches.len() {
                    resolved.node = concrete[0].clone();
                    resolved.nullable = true;
                    continue;
                }
            }
            resolved.nullable |= resolved.node.get("type").and_then(DslValue::as_array).is_some_and(|types| types.iter().any(|name| name.as_str() == Some("null")));
            if self.numeric_transport(&resolved.document, &resolved.node, pointer) {
                if input_shape(&resolved.node) == input_numeric_transport().0 {
                    self.recover(Err(input_error(InputSchemaErrorCode::WordOnlyFloat, pointer, "a number input accepts only the exact binary64 word, never the plain number its payload and every draft carry; reference framework/value/schema.json#/$defs/Binary64Transport")), || ())?;
                }
                resolved.node = DslValue::Object(vec![("type".to_string(), DslValue::String("number".to_string()))]);
            }
            return Ok(resolved);
        }
        Err(input_error(InputSchemaErrorCode::Malformed, pointer, "a $ref chain exceeds 32 hops"))
    }

    /// 🔢️ Whether `node` (in `document`) is the framework's numeric transport ([`input_numeric_transport`]): the binary64 word
    /// itself (read as a number but refused as `WordOnlyFloat`: the payload and every draft carry the plain number), or a
    /// two-branch union of that word and a plain number (each branch restated or referenced) with nothing but annotations
    /// beside it — an input edited as a number, its facets from the property's `x-semio-ui`.
    fn numeric_transport(&self, document: &Option<String>, node: &DslValue, pointer: &str) -> bool {
        let (word, number) = input_numeric_transport();
        if input_shape(node) == *word {
            return true;
        }
        let Some(branches) = input_union(node).filter(|branches| branches.len() == 2) else { return false };
        if node.as_object().is_some_and(|entries| entries.iter().any(|(key, _)| !matches!(key.as_str(), "anyOf" | "oneOf") && !key.starts_with("x-") && !INPUT_SHAPE_ANNOTATIONS.contains(&key.as_str()))) {
            return false;
        }
        let shapes: Vec<DslValue> = branches
            .iter()
            .map(|branch| {
                let (mut owner, mut current) = (document.clone(), branch.clone());
                for _ in 0..INPUT_REF_DEPTH {
                    let Some(reference) = current.get("$ref").and_then(DslValue::as_str).map(str::to_string) else { break };
                    let Ok(target) = self.target(&owner, &reference, pointer) else { return DslValue::Null };
                    (owner, current) = target;
                }
                input_shape(&current)
            })
            .collect();
        shapes.contains(word) && shapes.contains(number)
    }

    /// 🧩️ The object itself and, depth first, every `allOf` member it composes (each resolved in its own document).
    fn members(&self, object: &ResolvedInput, pointer: &str, depth: usize, members: &mut Vec<(Option<String>, DslValue)>) -> Result<(), InputSchemaError> {
        if depth == INPUT_REF_DEPTH {
            return Err(input_error(InputSchemaErrorCode::Malformed, pointer, "an allOf composition exceeds 32 levels"));
        }
        members.push((object.document.clone(), object.node.clone()));
        for member in object.node.get("allOf").and_then(DslValue::as_array).unwrap_or_default() {
            let resolved = self.resolve(object.document.clone(), member.clone(), pointer)?;
            self.members(&resolved, pointer, depth + 1, members)?;
        }
        Ok(())
    }

    /// 🔀️ A discriminated union's variants: the first property of the first member that every member pins to a
    /// distinct string `const`, and per member that value, the member and its discriminator property.
    fn variants(&self, union: &ResolvedInput, pointer: &str) -> Result<(String, Vec<InputVariant>), InputSchemaError> {
        let mut members = Vec::new();
        for branch in input_union(&union.node).unwrap_or_default() {
            let member = self.resolve(union.document.clone(), branch.clone(), pointer)?;
            let mut composed = Vec::new();
            self.members(&member, pointer, 0, &mut composed)?;
            let mut pins: Vec<(String, Option<String>, ResolvedInput)> = Vec::new();
            for (document, node) in &composed {
                for (key, property) in node.get("properties").and_then(DslValue::as_object).unwrap_or_default() {
                    if pins.iter().any(|(name, _, _)| name == key) {
                        continue;
                    }
                    let resolved = self.resolve(document.clone(), property.clone(), &input_pointer(pointer, key))?;
                    let value = resolved.node.get("const").and_then(DslValue::as_str).map(str::to_string);
                    pins.push((key.clone(), value, resolved));
                }
            }
            members.push((member, pins));
        }
        let pinned = |key: &str| -> Option<Vec<String>> {
            let values: Vec<String> = members.iter().map(|(_, pins)| pins.iter().find(|(name, _, _)| name == key).and_then(|(_, value, _)| value.clone())).collect::<Option<_>>()?;
            values.iter().enumerate().all(|(index, value)| !values[..index].contains(value)).then_some(values)
        };
        let key = members.first().and_then(|(_, pins)| pins.iter().map(|(name, _, _)| name.clone()).find(|name| pinned(name).is_some())).ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, pointer, "a payload union names no property every variant pins to a distinct string const"))?;
        let variants = members
            .into_iter()
            .filter_map(|(member, pins)| pins.into_iter().find(|(name, _, _)| *name == key).and_then(|(_, value, discriminator)| Some(InputVariant { value: value?, member, discriminator })))
            .collect();
        Ok((key, variants))
    }

    /// 🔀️ A discriminated union's inputs: one required variant selector (segmented up to four variants, else a select;
    /// each option labelled by its member's `x-semio-ui.label` or the glossary) followed by every member's fields, each
    /// grouped under its variant's value so an editor shows only the active variant's fields.
    fn variant_inputs(&self, union: &ResolvedInput, pointer: &str) -> Result<Vec<ActionArgDef>, InputSchemaError> {
        let (key, variants) = self.variants(union, pointer)?;
        let selector = input_pointer(pointer, &key);
        let discriminator = &variants.first().ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, pointer, "a payload union has no variant"))?.discriminator;
        let label = match discriminator.ui("label") {
            Some(label) => input_localized_text(label).map_err(|code| input_error(code, &selector, "label names every locale")),
            None => input_label_glossary().get(&key).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::LabelMissing, &selector, format!("{key} has no x-semio-ui.label and no glossary label"))),
        };
        let label = self.recover(label, LocalizedLabel::default)?;
        let description = discriminator.ui("description").map(|description| input_localized_text(description).map_err(|code| input_error(code, &selector, "description names every locale"))).transpose();
        let description = self.recover(description, || None)?;
        let options = variants
            .iter()
            .map(|variant| {
                let label = match variant.member.ui("label") {
                    Some(label) => input_localized_text(label).map_err(|code| input_error(code, &selector, format!("variant {} names every locale", variant.value))),
                    None => input_label_glossary().get(&variant.value).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::OptionLabelMissing, &selector, format!("variant {} has no x-semio-ui.label and no glossary label", variant.value))),
                };
                Ok(ActionArgOption { value: variant.value.clone(), label: self.recover(label, LocalizedLabel::default)? })
            })
            .collect::<Result<Vec<_>, InputSchemaError>>()?;
        let presentation = (options.len() <= 4).then_some(ArgPresentation::Segmented);
        let mut inputs = vec![ActionArgDef { id: input_pointer("", &key), label, schema: ArgSchema::String { options, option_source: None, min_len: None, max_len: None, pattern: None, format: None }, presentation, required: true, nullable: false, default: None, description, group: None, order: None }];
        for variant in &variants {
            for mut field in self.fields(&variant.member, pointer)? {
                field.group = Some(variant.value.clone());
                inputs.push(field);
            }
        }
        Ok(inputs)
    }

    fn fields(&self, object: &ResolvedInput, pointer: &str) -> Result<Vec<ActionArgDef>, InputSchemaError> {
        let mut members = Vec::new();
        self.members(object, pointer, 0, &mut members)?;
        let required: Vec<&str> = members.iter().flat_map(|(_, node)| node.get("required").and_then(DslValue::as_array).unwrap_or_default().iter().filter_map(DslValue::as_str)).collect();
        let mut fields = Vec::new();
        let mut seen = Vec::new();
        for (document, node) in &members {
            for (key, property) in node.get("properties").and_then(DslValue::as_object).unwrap_or_default() {
                if seen.contains(&key.as_str()) {
                    continue;
                }
                seen.push(key.as_str());
                let child_pointer = input_pointer(pointer, key);
                let Some(resolved) = self.recover(self.resolve(document.clone(), property.clone(), &child_pointer).map(Some), || None)? else { continue };
                if let Some(field) = self.recover(self.input(key, &resolved, required.contains(&key.as_str()), &child_pointer), || None)? {
                    fields.push(field);
                }
            }
        }
        Ok(fields)
    }

    fn input(&self, key: &str, input: &ResolvedInput, required: bool, pointer: &str) -> Result<Option<ActionArgDef>, InputSchemaError> {
        let role = match input.ui("role") {
            None => None,
            Some(DslValue::String(role)) if ["value", "target", "discriminator"].contains(&role.as_str()) => Some(role.as_str()),
            Some(_) => self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "role is value, target or discriminator")), || None)?,
        };
        if role == Some("discriminator") || input.node.get("const").is_some() {
            return Ok(None);
        }
        let widget = match input.ui("widget") {
            None => None,
            Some(DslValue::String(widget)) if ["slider", "stepper", "dial", "toggle", "select", "segmented", "text", "multiline", "vector", "color", "reference", "hidden"].contains(&widget.as_str()) => Some(widget.as_str()),
            Some(_) => self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "widget is not a declared widget")), || None)?,
        };
        let label = match input.ui("label") {
            Some(label) => input_localized_text(label).map_err(|code| input_error(code, pointer, "label names every locale")),
            None => input_label_glossary().get(key).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::LabelMissing, pointer, format!("{key} has no x-semio-ui.label and no glossary label"))),
        };
        let label = self.recover(label, LocalizedLabel::default)?;
        let description = input.ui("description").map(|description| input_localized_text(description).map_err(|code| input_error(code, pointer, "description names every locale"))).transpose();
        let description = self.recover(description, || None)?;
        let group = match input.ui("group") {
            None => None,
            Some(DslValue::String(group)) if !group.is_empty() => Some(group.clone()),
            Some(_) => self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "group is a non-empty string")), || None)?,
        };
        let order = match input.ui("order") {
            None => None,
            Some(order) => self.recover(order.as_i64().map(Some).ok_or_else(|| input_error(InputSchemaErrorCode::UiInvalid, pointer, "order is an integer")), || None)?,
        };
        let Some(schema) = self.recover(self.guarded(input, || self.schema(key, input, role, widget, pointer)).map(Some), || None)? else {
            return Ok(Some(ActionArgDef { id: input_pointer("", key), label, schema: ArgSchema::Any, presentation: None, required, nullable: input.nullable, default: None, description, group, order }));
        };
        let compatible = match (widget, &schema) {
            (Some("slider" | "stepper" | "dial"), ArgSchema::Number { .. }) | (Some("toggle"), ArgSchema::Boolean) | (Some("vector"), ArgSchema::Vector { .. }) | (Some("reference"), ArgSchema::Reference { .. }) | (Some("hidden") | None, _) => true,
            (Some("color"), ArgSchema::Vector { dims, min, max, .. }) => matches!(dims, 3 | 4) && *min == Some(0.0) && *max == Some(1.0),
            (Some("select" | "segmented"), ArgSchema::String { options, option_source, .. }) => !options.is_empty() || option_source.is_some(),
            (Some("text" | "multiline"), ArgSchema::String { options, option_source, .. }) => options.is_empty() && option_source.is_none(),
            _ => false,
        };
        if !compatible {
            self.recover(Err(input_error(InputSchemaErrorCode::WidgetIncompatible, pointer, format!("widget {} cannot edit this value", widget.unwrap_or_default()))), || ())?;
        }
        let presentation = match widget {
            Some("slider") => Some(ArgPresentation::Slider),
            Some("stepper") => Some(ArgPresentation::Stepper),
            Some("dial") => Some(ArgPresentation::Dial),
            Some("segmented") => Some(ArgPresentation::Segmented),
            Some("multiline") => Some(ArgPresentation::Multiline),
            Some("hidden") => Some(ArgPresentation::Hidden),
            Some("color") => Some(ArgPresentation::Color),
            _ => None,
        };
        let default = input.node.get("default").cloned();
        Ok(Some(ActionArgDef { id: input_pointer("", key), label, schema, presentation, required, nullable: input.nullable, default, description, group, order }))
    }

    fn schema(&self, key: &str, input: &ResolvedInput, role: Option<&str>, widget: Option<&str>, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        let kind = input_type(&input.node);
        if !matches!(kind, Some("integer" | "number")) {
            if let Some((name, _)) = input.ui.iter().find(|(name, _)| INPUT_UI_NUMBER_KEYS.contains(&name.as_str()) && !(kind == Some("array") && INPUT_UI_VECTOR_KEYS.contains(&name.as_str()))) {
                self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{name} only applies to a number"))), || ())?;
            }
        }
        if input.ui("unit").is_some() && !matches!(kind, Some("integer" | "number" | "array")) {
            self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "unit only applies to a number or a vector")), || ())?;
        }
        if input.ui("optionSource").is_some() && (kind != Some("string") || input.node.get("enum").is_some()) {
            self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "optionSource only sources the options of a string without an enum")), || ())?;
        }
        if widget == Some("hidden") && matches!(kind, Some("object" | "array")) {
            return Ok(ArgSchema::Any);
        }
        let many = kind == Some("array");
        let items = match (many, input.node.get("items")) {
            (true, Some(items)) => self.recover(self.resolve(input.document.clone(), items.clone(), &format!("{pointer}/-")).map(Some), || None)?,
            _ => None,
        };
        let item_kind = items.as_ref().and_then(|items| input_type(&items.node));
        let id_kind = if many { item_kind } else { kind };
        let reference_shaped = id_kind == Some("string");
        let explicit_reference = input.ui("ref").is_some() || widget == Some("reference") || role == Some("target");
        if explicit_reference && !matches!(id_kind, Some("string" | "integer")) {
            return Err(input_error(InputSchemaErrorCode::WidgetIncompatible, pointer, "a reference is a string or integer id or an array of them"));
        }
        let inferred_kind = (role.is_none() && widget.is_none() && reference_shaped && input.node.get("enum").is_none()).then(|| input_inferred_reference_kind(key, many)).flatten();
        if explicit_reference || inferred_kind.is_some() {
            if let Some((name, _)) = input.ui.iter().find(|(name, _)| name == "unit" || INPUT_UI_NUMBER_KEYS.contains(&name.as_str())) {
                self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{name} does not apply to a reference"))), || ())?;
            }
            let id_type = if id_kind == Some("integer") { ReferenceIdType::Integer } else { ReferenceIdType::String };
            return self.reference(key, input, many, inferred_kind, id_type, pointer);
        }
        if input.ui("options").is_some() && input.node.get("enum").is_none() {
            self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "options only label enum values")), || ())?;
        }
        match kind {
            Some("string") => Ok(ArgSchema::String {
                options: input_options(input, pointer)?,
                option_source: input_option_source(input, pointer)?,
                min_len: input.node.get("minLength").and_then(DslValue::as_u64).map(|length| length as u32),
                max_len: input.node.get("maxLength").and_then(DslValue::as_u64).map(|length| length as u32),
                pattern: input.node.get("pattern").and_then(DslValue::as_str).map(str::to_string),
                format: (input.node.get("format").and_then(DslValue::as_str) == Some("uri")).then_some(ArgFormat::Uri),
            }),
            Some(number @ ("integer" | "number")) => input_number(input, number == "integer", pointer),
            Some("boolean") => Ok(ArgSchema::Boolean),
            Some("object") if input.node.get("properties").is_some() || input.node.get("allOf").is_some() => Ok(ArgSchema::Object { fields: self.fields(input, pointer)? }),
            Some("array") => {
                let bound = |name: &str| input.node.get(name).and_then(DslValue::as_u64).map(|count| count as u32);
                let (min_items, max_items) = (bound("minItems"), bound("maxItems"));
                let numeric = matches!(item_kind, Some("integer" | "number"));
                let fixed = min_items.filter(|count| Some(*count) == max_items && (2..=4).contains(count));
                if matches!(widget, Some("vector" | "color")) || (numeric && fixed.is_some()) {
                    let (Some(dims), Some(items)) = (fixed.filter(|_| numeric), items.as_ref()) else {
                        return Err(input_error(InputSchemaErrorCode::WidgetIncompatible, pointer, "a vector is an array of 2 to 4 numbers of fixed length"));
                    };
                    let component = ResolvedInput { document: items.document.clone(), node: items.node.clone(), ui: input.ui.clone(), refs: Vec::new(), nullable: items.nullable };
                    let ArgSchema::Number { min, min_exclusive, max, max_exclusive, snaps, snap_source, precision, display_unit, display_factor, .. } = input_number(&component, item_kind == Some("integer"), pointer)? else {
                        unreachable!("input_number reads a number");
                    };
                    return Ok(ArgSchema::Vector {
                        dims,
                        min: min.filter(|_| !min_exclusive),
                        max: max.filter(|_| !max_exclusive),
                        unit: input_string(input, "unit", pointer)?,
                        step: input_positive(input, "step", pointer)?,
                        snaps,
                        snap_source,
                        precision,
                        display_unit,
                        display_factor,
                    });
                }
                if let Some((name, _)) = input.ui.iter().find(|(name, _)| INPUT_UI_VECTOR_KEYS.contains(&name.as_str())).filter(|_| !numeric) {
                    self.recover(Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{name} only applies to a number or a vector"))), || ())?;
                }
                let items = match &items {
                    Some(items) if numeric => {
                        let mut ui = items.ui.clone();
                        ui.extend(input.ui.iter().filter(|(name, _)| (name == "unit" || INPUT_UI_VECTOR_KEYS.contains(&name.as_str())) && !items.ui.iter().any(|(own, _)| own == name)).cloned());
                        self.item_schema(&ResolvedInput { document: items.document.clone(), node: items.node.clone(), ui, refs: items.refs.clone(), nullable: items.nullable }, &format!("{pointer}/-"))?
                    }
                    Some(items) => self.item_schema(items, &format!("{pointer}/-"))?,
                    None => ArgSchema::Any,
                };
                Ok(ArgSchema::Array { items: Box::new(items), min_items, max_items })
            }
            _ => Ok(ArgSchema::Any),
        }
    }

    fn item_schema(&self, items: &ResolvedInput, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        self.guarded(items, || self.item_schema_expanded(items, pointer))
    }

    fn item_schema_expanded(&self, items: &ResolvedInput, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        if items.node.get("const").is_some() {
            return Ok(ArgSchema::Any);
        }
        match input_type(&items.node) {
            Some("string") => Ok(ArgSchema::String { options: input_options(items, pointer)?, option_source: input_option_source(items, pointer)?, min_len: None, max_len: None, pattern: None, format: None }),
            Some(number @ ("integer" | "number")) => input_number(items, number == "integer", pointer),
            Some("boolean") => Ok(ArgSchema::Boolean),
            Some("object") if items.node.get("properties").is_some() || items.node.get("allOf").is_some() => Ok(ArgSchema::Object { fields: self.fields(items, pointer)? }),
            Some("array") => {
                let inner = match items.node.get("items") {
                    Some(inner) => self.item_schema(&self.resolve(items.document.clone(), inner.clone(), &format!("{pointer}/-"))?, &format!("{pointer}/-"))?,
                    None => ArgSchema::Any,
                };
                let bound = |name: &str| items.node.get(name).and_then(DslValue::as_u64).map(|count| count as u32);
                Ok(ArgSchema::Array { items: Box::new(inner), min_items: bound("minItems"), max_items: bound("maxItems") })
            }
            _ => Ok(ArgSchema::Any),
        }
    }

    fn reference(&self, key: &str, input: &ResolvedInput, many: bool, inferred_kind: Option<String>, id_type: ReferenceIdType, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
        let invalid = |detail: &str| input_error(InputSchemaErrorCode::UiInvalid, pointer, detail);
        let reference = input.ui("ref");
        if reference.is_some_and(|reference| reference.as_object().is_none_or(|entries| entries.iter().any(|(name, _)| !["kind", "domain", "granularity"].contains(&name.as_str())))) {
            return Err(invalid("ref is {kind, domain?, granularity?}"));
        }
        let kinds = match reference.and_then(|reference| reference.get("kind")) {
            Some(DslValue::String(kind)) if !kind.is_empty() => vec![kind.clone()],
            Some(DslValue::Array(kinds)) if !kinds.is_empty() && kinds.iter().all(|kind| kind.as_str().is_some_and(|kind| !kind.is_empty())) => kinds.iter().filter_map(DslValue::as_str).map(str::to_string).collect(),
            Some(_) => return Err(invalid("ref.kind is a non-empty string or a non-empty array of them")),
            None => vec![inferred_kind.or_else(|| input_inferred_reference_kind(key, many)).ok_or_else(|| invalid("a target names its ref.kind"))?],
        };
        let text = |name: &str| -> Result<Option<String>, InputSchemaError> {
            match reference.and_then(|reference| reference.get(name)) {
                None => Ok(None),
                Some(DslValue::String(value)) if !value.is_empty() => Ok(Some(value.clone())),
                Some(_) => Err(invalid("ref.domain and ref.granularity are non-empty strings")),
            }
        };
        let bound = |name: &str| input.node.get(name).and_then(DslValue::as_u64).map(|count| count as u32);
        Ok(ArgSchema::Reference { kinds, domain: text("domain")?, granularity: text("granularity")?, many, min_items: many.then(|| bound("minItems")).flatten(), max_items: many.then(|| bound("maxItems")).flatten(), id_type })
    }
}

fn input_string(input: &ResolvedInput, key: &str, pointer: &str) -> Result<Option<String>, InputSchemaError> {
    match input.ui(key) {
        None => Ok(None),
        Some(DslValue::String(value)) if !value.is_empty() => Ok(Some(value.clone())),
        Some(_) => Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{key} is a non-empty string"))),
    }
}

fn input_finite(input: &ResolvedInput, key: &str, pointer: &str) -> Result<Option<f64>, InputSchemaError> {
    match input.ui(key) {
        None => Ok(None),
        Some(value) => value.as_f64().filter(|value| value.is_finite()).map(Some).ok_or_else(|| input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{key} is a finite number"))),
    }
}

fn input_positive(input: &ResolvedInput, key: &str, pointer: &str) -> Result<Option<f64>, InputSchemaError> {
    match input_finite(input, key, pointer)? {
        Some(value) if value <= 0.0 => Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, format!("{key} is positive"))),
        value => Ok(value),
    }
}

fn input_options(input: &ResolvedInput, pointer: &str) -> Result<Vec<ActionArgOption>, InputSchemaError> {
    let Some(values) = input.node.get("enum").and_then(DslValue::as_array) else { return Ok(Vec::new()) };
    let labels = input.ui("options");
    if labels.is_some_and(|labels| labels.as_object().is_none_or(|entries| entries.iter().any(|(value, _)| !values.iter().any(|candidate| candidate.as_str() == Some(value.as_str()))))) {
        return Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "options labels only declared enum values"));
    }
    values
        .iter()
        .map(|value| {
            let value = value.as_str().ok_or_else(|| input_error(InputSchemaErrorCode::Malformed, pointer, "a labelled enum lists strings"))?;
            let label = match labels.and_then(|labels| labels.get(value)) {
                Some(label) => input_localized_text(label).map_err(|code| input_error(code, pointer, format!("option {value} names every locale")))?,
                None => input_label_glossary().get(value).cloned().ok_or_else(|| input_error(InputSchemaErrorCode::OptionLabelMissing, pointer, format!("option {value} has no label")))?,
            };
            Ok(ActionArgOption { value: value.to_string(), label })
        })
        .collect()
}

/// 🗝️ A string input's `x-semio-ui.optionSource`: `{snapshot: "<pointer template>"}`, an RFC 6901 template (`/…`) whose
/// `{field}` segments name members of the edited payload.
fn input_option_source(input: &ResolvedInput, pointer: &str) -> Result<Option<OptionSource>, InputSchemaError> {
    match input.ui("optionSource").map(DslValue::as_object) {
        None => Ok(None),
        Some(Some([(name, DslValue::String(template))])) if name == "snapshot" && template.starts_with('/') => Ok(Some(OptionSource::Snapshot { pointer: template.clone() })),
        Some(_) => Err(input_error(InputSchemaErrorCode::UiInvalid, pointer, "optionSource is {snapshot: pointer template}")),
    }
}

fn input_number(input: &ResolvedInput, integer: bool, pointer: &str) -> Result<ArgSchema, InputSchemaError> {
    let invalid = |detail: String| input_error(InputSchemaErrorCode::UiInvalid, pointer, detail);
    let keyword = |name: &str| input.node.get(name).and_then(DslValue::as_f64);
    let bound = |inclusive: Option<f64>, exclusive: Option<f64>, lower: bool| -> (Option<f64>, bool) {
        match (inclusive, exclusive) {
            (inclusive, None) => (inclusive, false),
            (Some(inclusive), Some(exclusive)) if (lower && inclusive > exclusive) || (!lower && inclusive < exclusive) => (Some(inclusive), false),
            (_, Some(exclusive)) if integer => (Some(if lower { exclusive.floor() + 1.0 } else { exclusive.ceil() - 1.0 }), false),
            (_, Some(exclusive)) => (Some(exclusive), true),
        }
    };
    let (min, min_exclusive) = bound(keyword("minimum"), keyword("exclusiveMinimum"), true);
    let (max, max_exclusive) = bound(keyword("maximum"), keyword("exclusiveMaximum"), false);
    let inside = |value: f64| min.is_none_or(|min| if min_exclusive { value > min } else { value >= min }) && max.is_none_or(|max| if max_exclusive { value < max } else { value <= max });
    let snaps = match input.ui("snaps") {
        None => Vec::new(),
        Some(DslValue::Array(values)) => values.iter().map(|value| value.as_f64().filter(|value| value.is_finite())).collect::<Option<Vec<f64>>>().ok_or_else(|| invalid("snaps are finite numbers".to_string()))?,
        Some(_) => return Err(invalid("snaps is an array of numbers".to_string())),
    };
    if let Some(snap) = snaps.iter().find(|snap| !inside(**snap)) {
        return Err(invalid(format!("snap {snap} lies outside the hard bounds")));
    }
    let snap_source = match input.ui("snapSource") {
        None => None,
        Some(source) => Some(match source.as_object() {
            Some([(name, DslValue::Bool(true))]) if name == "step" => SnapSource::Step,
            Some([(name, DslValue::String(key))]) if name == "config" && !key.is_empty() => SnapSource::Config { key: key.clone() },
            Some([(name, DslValue::String(target))]) if name == "snapshot" && (target.is_empty() || target.starts_with('/')) => SnapSource::Snapshot { pointer: target.clone() },
            _ => return Err(invalid("snapSource is {step: true}, {config: key} or {snapshot: pointer}".to_string())),
        }),
    };
    let soft_min = input_finite(input, "softMin", pointer)?;
    let soft_max = input_finite(input, "softMax", pointer)?;
    if soft_min.is_some_and(|soft| !inside(soft)) || soft_max.is_some_and(|soft| !inside(soft)) || soft_min.zip(soft_max).is_some_and(|(low, high)| low >= high) {
        return Err(invalid("softMin < softMax lie inside the hard bounds".to_string()));
    }
    let scale = match input.ui_str("scale") {
        None if input.ui("scale").is_none() => None,
        Some("linear") => Some(NumberScale::Linear),
        Some("log") => Some(NumberScale::Log),
        _ => return Err(invalid("scale is linear or log".to_string())),
    };
    if scale == Some(NumberScale::Log) && !soft_min.or(min).is_some_and(|low| low > 0.0) {
        return Err(invalid("a log scale starts at a positive softMin or minimum".to_string()));
    }
    let precision = match input.ui("precision") {
        None => None,
        Some(precision) => Some(precision.as_u64().filter(|digits| *digits <= 15).ok_or_else(|| invalid("precision is an integer 0..=15".to_string()))? as u32),
    };
    let display_factor = input_finite(input, "displayFactor", pointer)?;
    if display_factor == Some(0.0) {
        return Err(invalid("displayFactor is non-zero".to_string()));
    }
    Ok(ArgSchema::Number {
        min,
        min_exclusive,
        max,
        max_exclusive,
        step: input_positive(input, "step", pointer)?.or(integer.then_some(1.0)),
        integer,
        unit: input_string(input, "unit", pointer)?,
        snaps,
        snap_source,
        soft_min,
        soft_max,
        precision,
        display_unit: input_string(input, "displayUnit", pointer)?,
        display_factor,
        scale,
    })
}

//#endregion 🔖️MutationInputs
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest

#[cfg(test)]
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
#[cfg(test)]
use semio_framework_value::retained_clone::RetainedCloneGrant;
#[cfg(test)]
use semio_framework_value::retained_clone::RetainedCloneProgress;
#[cfg(test)]
use semio_framework_manifest_arg_schema::{ActionArgControl, AppRole, reference_id_text};

#[cfg(test)]
#[global_allocator]
static TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path = "../🔖️action-args/♻️retirement/🧪️tests/🦀️.rs"]
mod argument_retirement_tests;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️MutationInputs
#[cfg(test)]
#[path = "../🧪️tests/🧪️mutation-inputs/🦀️.rs"]
mod mutation_inputs_tests;
//#endregion 🔖️MutationInputs
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
