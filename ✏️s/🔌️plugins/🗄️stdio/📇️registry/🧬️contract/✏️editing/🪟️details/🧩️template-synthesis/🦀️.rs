//! 🧩️ Template candidates synthesized from schemas, validity checks and schema annotations.

use super::*;

pub(super) fn template_candidate_is_valid(root: &DslValue, schema: &DslValue, value: &DslValue, depth: usize) -> bool {
    if depth >= 64 {
        return false;
    }
    let Some(schema) = resolved_schema(root, schema, depth) else { return false };
    if let Some(members) = union_members(schema) {
        let matches = members.iter().filter(|member| template_candidate_is_valid(root, member, value, depth + 1)).count();
        return if object_field(schema, "oneOf").is_some() { matches == 1 } else { matches > 0 };
    }
    if let Some(expected) = object_field(schema, "const") {
        if expected != value {
            return false;
        }
    }
    if let Some(DslValue::Array(values)) = object_field(schema, "enum") {
        if !values.contains(value) {
            return false;
        }
    }
    let shape = match value {
        DslValue::Null => ValueShape::Null,
        DslValue::Bool(_) => ValueShape::Bool,
        DslValue::Number(_) => ValueShape::Number,
        DslValue::String(_) => ValueShape::String,
        DslValue::Bytes(values) => ValueShape::Bytes { len: values.len() },
        DslValue::Array(values) => ValueShape::Array { len: values.len() },
        DslValue::Object(values) => ValueShape::Object { len: values.len() },
    };
    if !schema_accepts_shape(root, schema, &shape) {
        return false;
    }
    match value {
        DslValue::String(value) => {
            let length = value.chars().count();
            if schema_usize(object_field(schema, "minLength")).is_some_and(|minimum| length < minimum) || schema_usize(object_field(schema, "maxLength")).is_some_and(|maximum| length > maximum) {
                return false;
            }
        }
        DslValue::Number(value) => {
            let value = number_as_f64(*value);
            if schema_number(object_field(schema, "minimum")).is_some_and(|minimum| value < number_as_f64(minimum)) || schema_number(object_field(schema, "maximum")).is_some_and(|maximum| value > number_as_f64(maximum)) {
                return false;
            }
        }
        DslValue::Bytes(values) => {
            if schema_usize(object_field(schema, "minItems")).is_some_and(|minimum| values.len() < minimum) || schema_usize(object_field(schema, "maxItems")).is_some_and(|maximum| values.len() > maximum) {
                return false;
            }
            if let Some(items) = object_field(schema, "items") {
                for (index, byte) in values.iter().enumerate() {
                    let item = match items {
                        DslValue::Array(items) => items.get(index).or_else(|| items.last()),
                        item => Some(item),
                    };
                    if item.is_some_and(|item| !template_candidate_is_valid(root, item, &DslValue::Number(Number::UInt(u64::from(*byte))), depth + 1)) {
                        return false;
                    }
                }
            }
        }
        DslValue::Array(values) => {
            if schema_usize(object_field(schema, "minItems")).is_some_and(|minimum| values.len() < minimum) || schema_usize(object_field(schema, "maxItems")).is_some_and(|maximum| values.len() > maximum) {
                return false;
            }
            if let Some(items) = object_field(schema, "items") {
                for (index, value) in values.iter().enumerate() {
                    let item = match items {
                        DslValue::Array(items) => items.get(index).or_else(|| items.last()),
                        item => Some(item),
                    };
                    if item.is_some_and(|item| !template_candidate_is_valid(root, item, value, depth + 1)) {
                        return false;
                    }
                }
            }
        }
        DslValue::Object(values) => {
            if schema_usize(object_field(schema, "minProperties")).is_some_and(|minimum| values.len() < minimum) || schema_usize(object_field(schema, "maxProperties")).is_some_and(|maximum| values.len() > maximum) {
                return false;
            }
            if let Some(DslValue::Array(required)) = object_field(schema, "required") {
                if required
                    .iter()
                    .filter_map(|value| match value {
                        DslValue::String(value) => Some(value),
                        _ => None,
                    })
                    .any(|required| !values.iter().any(|(key, _)| key == required))
                {
                    return false;
                }
            }
            for (key, value) in values {
                let property = object_field(schema, "properties").and_then(|properties| object_field(properties, key));
                let property = property.or_else(|| match object_field(schema, "additionalProperties") {
                    Some(value @ DslValue::Object(_)) => Some(value),
                    Some(DslValue::Bool(false)) => return None,
                    _ => None,
                });
                if property.is_some_and(|property| !template_candidate_is_valid(root, property, value, depth + 1)) {
                    return false;
                }
                if property.is_none() && matches!(object_field(schema, "additionalProperties"), Some(DslValue::Bool(false))) {
                    return false;
                }
            }
        }
        _ => {}
    }
    true
}

fn template_validation_schema(root: &DslValue, schema: &DslValue) -> Option<DslValue> {
    let DslValue::Object(mut entries) = resolved_schema(root, schema, 0)?.clone() else { return None };
    for key in ["$schema", "$defs", "definitions", RESOLVED_DOCUMENTS_KEY] {
        if entries.iter().all(|(candidate, _)| candidate != key) {
            if let Some(value) = object_field(root, key) {
                entries.push((key.into(), value.clone()));
            }
        }
    }
    Some(DslValue::Object(entries))
}

fn template_candidate_is_authoritatively_valid(root: &DslValue, schema: &DslValue, value: &DslValue) -> bool {
    static CACHE: OnceLock<Mutex<HashMap<String, semio_framework_schema::OwnedJsonSchemaValidator>>> = OnceLock::new();
    let Some(schema) = template_validation_schema(root, schema) else { return false };
    let schema = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&schema));
    let validator = CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok().and_then(|cache| cache.get(&schema).cloned()).or_else(|| {
        let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&schema).ok()?;
        CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok()?.insert(schema.clone(), validator.clone());
        Some(validator)
    });
    let Some(validator) = validator else { return false };
    let value = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(value));
    validator.is_valid_json(&value)
}

pub(super) fn template_candidate_can_be_offered(root: &DslValue, schema: &DslValue, value: &DslValue, depth: usize) -> bool {
    template_candidate_is_valid(root, schema, value, depth) && template_candidate_is_authoritatively_valid(root, schema, value)
}

pub(super) fn template_from_schema(root: &DslValue, schema: &DslValue, depth: usize) -> Option<DslValue> {
    if depth >= 64 {
        return None;
    }
    let schema = resolved_schema(root, schema, depth)?;
    for keyword in ["default", "const", "examples", "enum"] {
        match object_field(schema, keyword) {
            Some(DslValue::Array(values)) if matches!(keyword, "examples" | "enum") => {
                if let Some(value) = values.iter().find(|value| template_candidate_can_be_offered(root, schema, value, depth + 1)) {
                    return Some(value.clone());
                }
            }
            Some(value) if !matches!(keyword, "examples" | "enum") && template_candidate_can_be_offered(root, schema, value, depth + 1) => return Some(value.clone()),
            _ => {}
        }
    }
    if let Some(members) = union_members(schema) {
        return members.iter().filter(|member| !schema_is_null(root, member)).find_map(|member| template_from_schema(root, member, depth + 1)).or_else(|| members.iter().find_map(|member| template_from_schema(root, member, depth + 1)));
    }
    if let Some(DslValue::Array(members)) = object_field(schema, "allOf") {
        return members.iter().find_map(|member| template_from_schema(root, member, depth + 1));
    }
    let kind = match object_field(schema, "type") {
        Some(DslValue::String(kind)) => Some(kind.as_str()),
        Some(DslValue::Array(kinds)) => kinds.iter().find_map(|kind| match kind {
            DslValue::String(kind) if kind != "null" => Some(kind.as_str()),
            _ => None,
        }),
        _ if object_field(schema, "properties").is_some() => Some("object"),
        _ if object_field(schema, "items").is_some() => Some("array"),
        _ => None,
    }?;
    match kind {
        "object" => {
            let properties = object_field(schema, "properties");
            let required = match object_field(schema, "required") {
                Some(DslValue::Array(required)) => required.as_slice(),
                _ => &[],
            };
            let mut fields = Vec::with_capacity(required.len());
            for required in required {
                let DslValue::String(key) = required else { return None };
                let property = object_field(properties?, key)?;
                fields.push((key.clone(), template_from_schema(root, property, depth + 1)?));
            }
            let minimum = schema_usize(object_field(schema, "minProperties")).unwrap_or(0);
            if fields.len() < minimum {
                let DslValue::Object(properties) = properties? else { return None };
                for (key, property) in properties {
                    if fields.iter().any(|(field, _)| field == key) {
                        continue;
                    }
                    if let Some(value) = template_from_schema(root, property, depth + 1) {
                        fields.push((key.clone(), value));
                    }
                    if fields.len() >= minimum {
                        break;
                    }
                }
            }
            let candidate = DslValue::Object(fields);
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "array" => {
            let count = match schema_number(object_field(schema, "minItems")) {
                Some(Number::UInt(value)) => usize::try_from(value).ok()?,
                Some(Number::Int(value)) if value >= 0 => usize::try_from(value).ok()?,
                _ => 0,
            };
            let item = object_field(schema, "items");
            let mut items = Vec::with_capacity(count);
            for _ in 0..count {
                items.push(template_from_schema(root, item?, depth + 1)?);
            }
            let candidate = DslValue::Array(items);
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "string" => {
            let length = match schema_number(object_field(schema, "minLength")) {
                Some(Number::UInt(value)) => usize::try_from(value).ok()?,
                Some(Number::Int(value)) if value >= 0 => usize::try_from(value).ok()?,
                _ => 0,
            };
            let candidate = DslValue::String("x".repeat(length));
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "integer" => {
            let candidate = DslValue::Number(schema_number(object_field(schema, "minimum")).or_else(|| schema_number(object_field(schema, "maximum"))).unwrap_or(Number::Int(0)));
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "number" => {
            let candidate = DslValue::Number(schema_number(object_field(schema, "minimum")).or_else(|| schema_number(object_field(schema, "maximum"))).unwrap_or(Number::Float(0.0)));
            template_candidate_can_be_offered(root, schema, &candidate, depth + 1).then_some(candidate)
        }
        "boolean" => template_candidate_can_be_offered(root, schema, &DslValue::Bool(false), depth + 1).then_some(DslValue::Bool(false)),
        "null" => template_candidate_can_be_offered(root, schema, &DslValue::Null, depth + 1).then_some(DslValue::Null),
        _ => None,
    }
}

pub(super) fn schema_enum_values(root: &DslValue, schema: &DslValue, depth: usize, values: &mut Vec<DslValue>) {
    if depth >= 64 {
        return;
    }
    let Some(schema) = resolved_schema(root, schema, depth) else { return };
    match object_field(schema, "enum") {
        Some(DslValue::Array(options)) => {
            for option in options {
                if !values.contains(option) {
                    values.push(option.clone());
                }
            }
        }
        _ => {}
    }
    if let Some(option) = object_field(schema, "const") {
        if !values.contains(option) {
            values.push(option.clone());
        }
    }
    if let Some(members) = union_members(schema) {
        for member in members {
            schema_enum_values(root, member, depth + 1, values);
        }
    }
}

pub(super) fn localized_schema_annotation(schema: &DslValue, key: &str, locale: Locale) -> Option<String> {
    let language = match locale {
        Locale::En => "en",
        Locale::De => "de",
    };
    for annotation in [format!("x-semio-{key}"), key.to_string()] {
        match object_field(schema, &annotation) {
            Some(DslValue::Object(entries)) => {
                if let Some(DslValue::String(value)) = entries.iter().find(|(candidate, _)| candidate == language).map(|(_, value)| value) {
                    return Some(value.clone());
                }
            }
            Some(DslValue::String(value)) if locale == Locale::En => return Some(value.clone()),
            _ => {}
        }
    }
    None
}

pub(super) fn humanized_schema_key(key: &str, locale: Locale) -> String {
    let mut words = String::with_capacity(key.len() + 8);
    let mut previous_lowercase = false;
    for character in key.chars() {
        if matches!(character, '_' | '-') {
            if !words.ends_with(' ') {
                words.push(' ');
            }
            previous_lowercase = false;
            continue;
        }
        if character.is_uppercase() && previous_lowercase {
            words.push(' ');
        }
        words.extend(character.to_lowercase());
        previous_lowercase = character.is_lowercase();
    }
    let words = words.trim();
    let mut result = String::with_capacity(words.len());
    let mut characters = words.chars();
    if let Some(first) = characters.next() {
        result.extend(first.to_uppercase());
        result.extend(characters);
    }
    match (locale, key) {
        (Locale::De, "schema") => "Schemakennung".into(),
        (Locale::De, "description") => "Beschreibung".into(),
        (Locale::De, "title") => "Titel".into(),
        _ => result,
    }
}

pub(super) fn snapshot_schema(schema_id: &str) -> Option<Arc<DslValue>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<DslValue>>>> = OnceLock::new();
    let base_id = if schema_id.starts_with("s.") { schema_id.to_string() } else { format!("s.{schema_id}") };
    if let Some(schema) = CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok()?.get(&base_id).cloned() {
        return Some(schema);
    }
    let (source, documents) = semio_framework_schema_registry::with_artifact_schema_registry(|registry| {
        let source = registry.get(&base_id)?.snapshot.json_schema;
        Some((source, registry.iter().map(|descriptor| descriptor.snapshot.json_schema).collect::<Vec<_>>()))
    })?;
    let root = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
    let documents = documents.into_iter().filter_map(|source| semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).collect();
    let schema = Arc::new(bundle_snapshot_schema(root, documents));
    CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().ok()?.insert(base_id, schema.clone());
    Some(schema)
}
