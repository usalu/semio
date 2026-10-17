//! 📜️ Format application and JSON schema of action arguments: the format check of one value and the JSON schema of an argument schema. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionArgs
/// 🧬️ Tags a leaf/nested `ArgSchema` JSON Schema object with its `ArgFormat` — `x-semio-format`
/// (the vendor extension every format carries) plus, for the two host-resolved refinements, the
/// `roles`/`dialect_arg` a host needs to resolve them (`x-semio-roles`/`x-semio-dialect-arg`) — and
/// the standard `format: "uri"` keyword where JSON Schema already defines one.
fn apply_arg_format(entries: &mut Vec<(String, DslValue)>, format: &ArgFormat) {
    let tag = match format {
        ArgFormat::ArtifactRef => "artifactRef",
        ArgFormat::WindowId => "windowId",
        ArgFormat::EntityId { entity_kind } => {
            entries.push(("x-semio-entity-kind".to_string(), DslValue::String(entity_kind.clone())));
            "entityId"
        }
        ArgFormat::IconId => "iconId",
        ArgFormat::Color => "color",
        ArgFormat::Uri => {
            entries.push(("format".to_string(), DslValue::String("uri".to_string())));
            "uri"
        }
        ArgFormat::Json => "json",
        ArgFormat::Locale => "locale",
        ArgFormat::Terminology => "terminology",
        ArgFormat::DocumentRevision => "documentRevision",
        ArgFormat::TargetRevision => "targetRevision",
        ArgFormat::ArtifactKind { roles } => {
            entries.push(("x-semio-roles".to_string(), DslValue::Array(roles.iter().map(ToValue::to_value).collect())));
            "artifactKind"
        }
        ArgFormat::SurfaceApp { roles, dialect_arg } => {
            entries.push(("x-semio-roles".to_string(), DslValue::Array(roles.iter().map(ToValue::to_value).collect())));
            entries.push(("x-semio-dialect-arg".to_string(), DslValue::String(dialect_arg.clone())));
            "surfaceApp"
        }
    };
    entries.push(("x-semio-format".to_string(), DslValue::String(tag.to_string())));
}

/// 📐️ JSON Schema 2020-12 for one `ArgSchema` node (recursive over `Array`/`Object`) — carries
/// `Number.unit`/`Vector.unit` as `x-semio-unit`, `String.format` via `apply_arg_format`, a
/// `Reference` as `x-semio-format: reference` with its `x-semio-ref`. A number's `step` is a UI fact, never
/// `multipleOf`, and so are the options an `optionSource` resolved from the document — never an `enum`, the fold decides an
/// unknown key. No `additionalProperties`/`$schema`/`$id` at this altitude; the catalog compiler owns the envelope.
pub(super) fn arg_schema_json_schema(schema: &ArgSchema, terminology: Terminology, locale: Locale) -> DslValue {
    match schema {
        ArgSchema::String { options, option_source, min_len, max_len, pattern, format } => {
            let mut entries = vec![("type".to_string(), DslValue::String("string".to_string()))];
            if !options.is_empty() && option_source.is_none() {
                entries.push(("enum".to_string(), DslValue::Array(options.iter().map(|option| DslValue::String(option.value.clone())).collect())));
            }
            if let Some(min_len) = min_len {
                entries.push(("minLength".to_string(), min_len.to_value()));
            }
            if let Some(max_len) = max_len {
                entries.push(("maxLength".to_string(), max_len.to_value()));
            }
            if let Some(pattern) = pattern {
                entries.push(("pattern".to_string(), DslValue::String(pattern.clone())));
            }
            if let Some(format) = format {
                apply_arg_format(&mut entries, format);
            }
            DslValue::Object(entries)
        }
        ArgSchema::Number { min, min_exclusive, max, max_exclusive, integer, unit, .. } => {
            let mut entries = vec![("type".to_string(), DslValue::String(if *integer { "integer" } else { "number" }.to_string()))];
            if let Some(min) = min {
                entries.push((if *min_exclusive { "exclusiveMinimum" } else { "minimum" }.to_string(), min.to_value()));
            }
            if let Some(max) = max {
                entries.push((if *max_exclusive { "exclusiveMaximum" } else { "maximum" }.to_string(), max.to_value()));
            }
            if let Some(unit) = unit {
                entries.push(("x-semio-unit".to_string(), DslValue::String(unit.clone())));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Boolean => DslValue::object([("type".to_string(), DslValue::String("boolean".to_string()))]),
        ArgSchema::Vector { dims, min, max, unit, .. } => {
            let mut component = vec![("type".to_string(), DslValue::String("number".to_string()))];
            component.extend(min.iter().map(|min| ("minimum".to_string(), DslValue::json_number(*min))));
            component.extend(max.iter().map(|max| ("maximum".to_string(), DslValue::json_number(*max))));
            let mut entries = vec![
                ("type".to_string(), DslValue::String("array".to_string())),
                ("items".to_string(), DslValue::Object(component)),
                ("minItems".to_string(), DslValue::uint(u64::from(*dims))),
                ("maxItems".to_string(), DslValue::uint(u64::from(*dims))),
            ];
            if let Some(unit) = unit {
                entries.push(("x-semio-unit".to_string(), DslValue::String(unit.clone())));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Reference { kinds, domain, granularity, many, min_items, max_items, id_type } => {
            let mut reference = vec![("kind".to_string(), DslValue::Array(kinds.iter().map(|kind| DslValue::String(kind.clone())).collect()))];
            reference.extend(domain.iter().map(|domain| ("domain".to_string(), DslValue::String(domain.clone()))));
            reference.extend(granularity.iter().map(|granularity| ("granularity".to_string(), DslValue::String(granularity.clone()))));
            let id = DslValue::object([
                ("type".to_string(), DslValue::String(if id_type.is_string() { "string" } else { "integer" }.to_string())),
                ("x-semio-format".to_string(), DslValue::String("reference".to_string())),
                ("x-semio-ref".to_string(), DslValue::Object(reference)),
            ]);
            if !*many {
                return id;
            }
            let mut entries = vec![("type".to_string(), DslValue::String("array".to_string())), ("items".to_string(), id)];
            entries.extend(min_items.iter().map(|min_items| ("minItems".to_string(), min_items.to_value())));
            entries.extend(max_items.iter().map(|max_items| ("maxItems".to_string(), max_items.to_value())));
            DslValue::Object(entries)
        }
        ArgSchema::Array { items, min_items, max_items } => {
            let mut entries = vec![("type".to_string(), DslValue::String("array".to_string())), ("items".to_string(), arg_schema_json_schema(items, terminology, locale))];
            if let Some(min_items) = min_items {
                entries.push(("minItems".to_string(), min_items.to_value()));
            }
            if let Some(max_items) = max_items {
                entries.push(("maxItems".to_string(), max_items.to_value()));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Object { fields } => {
            let mut properties = Vec::new();
            let mut required = Vec::new();
            for field in fields {
                properties.push((field.key(), field.json_schema(terminology, locale)));
                if field.required {
                    required.push(DslValue::String(field.key()));
                }
            }
            let mut entries = vec![
                ("type".to_string(), DslValue::String("object".to_string())),
                ("properties".to_string(), DslValue::Object(properties)),
                ("additionalProperties".to_string(), DslValue::Bool(false)),
            ];
            if !required.is_empty() {
                entries.push(("required".to_string(), DslValue::Array(required)));
            }
            DslValue::Object(entries)
        }
        ArgSchema::Any => DslValue::Object(vec![]),
    }
}
//#endregion 🔖️ActionArgs
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
