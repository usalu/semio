//! ⚡️ EnergyPlus 25.2 positional IDF records and dictionary-driven epJSON interchange.

use semio_framework_pack_json::{Object, Value};

#[path = "🔤️.rs"]
mod lexical;
pub use lexical::*;

/// 🗂️ An official IDD field: numeric values retain autosize/autocalculate strings.
#[derive(Clone, Copy)]
pub struct IdfField {
    pub name: &'static str,
    pub numeric: bool,
    pub choices: &'static [&'static str],
}

/// 📚️ Official field order and extensible array mapping for an EnergyPlus object.
pub struct IdfObject {
    pub kind: &'static str,
    pub fields: &'static [IdfField],
    pub extension: &'static str,
    pub extensibles: &'static [IdfField],
}

fn field_value(field: &IdfField, text: &str) -> Result<Value, String> {
    let text = field.choices.iter().find(|choice| choice.eq_ignore_ascii_case(text)).copied().unwrap_or(text);
    if field.numeric {
        if let Ok(number) = text.parse::<i64>() { return Ok(Value::from(number)); }
        if let Ok(number) = text.parse::<f64>() {
            if !number.is_finite() { return Err(format!("IDF: non-finite {}", field.name)); }
            return Ok(Value::from(number));
        }
        if !text.eq_ignore_ascii_case("autosize") && !text.eq_ignore_ascii_case("autocalculate") {
            return Err(format!("IDF: invalid numeric {} value {text:?}", field.name));
        }
    }
    Ok(Value::String(text.into()))
}

/// 📥️ Converts positional records using a canonical dictionary, refusing unknown or excess fields.
pub fn records_to_epjson(records: &[IdfRecord], dictionary: &[IdfObject]) -> Result<Value, String> {
    let mut root = Object::new();
    for record in records {
        let schema = dictionary.iter().find(|item| item.kind.eq_ignore_ascii_case(&record.kind)).ok_or_else(|| format!("IDF: unsupported object type {}", record.kind))?;
        let mut fields = Object::new();
        let mut name = None;
        for (index, value) in record.fields.iter().take(schema.fields.len()).enumerate() {
            let field = &schema.fields[index];
            if value.is_empty() { continue; }
            if field.name == "name" { name = Some(value.clone()); }
            else { fields.insert(field.name, field_value(field, value)?); }
        }
        let mut extension = Vec::new();
        let tail = record.fields.get(schema.fields.len()..).unwrap_or_default();
        if !tail.is_empty() {
            if schema.extensibles.is_empty() { return Err(format!("IDF: excess fields in {}", record.kind)); }
            if tail.len() % schema.extensibles.len() != 0 { return Err(format!("IDF: incomplete extensible group in {}", record.kind)); }
            for group in tail.chunks(schema.extensibles.len()) {
                let mut values = Object::new();
                for (field, value) in schema.extensibles.iter().zip(group) {
                    if !value.is_empty() { values.insert(field.name, field_value(field, value)?); }
                }
                extension.push(Value::Object(values));
            }
            fields.insert(schema.extension, Value::Array(extension));
        }
        if schema.fields.iter().any(|field| field.name == "name") && name.is_none() {
            return Err(format!("IDF: missing required name in {}", schema.kind));
        }
        let count = root.get(schema.kind).and_then(Value::as_object).map_or(0, Object::len);
        let name = name.unwrap_or_else(|| format!("{} {}", schema.kind, count + 1));
        if root.get(schema.kind).is_none() { root.insert(schema.kind, Value::Object(Object::new())); }
        let Some(Value::Object(objects)) = root.get_mut(schema.kind) else { unreachable!() };
        if objects.iter().any(|(key, _)| key.eq_ignore_ascii_case(&name)) { return Err(format!("IDF: duplicate {} name {name:?}", schema.kind)); }
        objects.insert(name, Value::Object(fields));
    }
    Ok(Value::Object(root))
}

fn scalar_text(value: &Value) -> Result<String, String> {
    match value {
        Value::String(text) => Ok(text.clone()),
        Value::Number(_) => Ok(semio_framework_pack_json::to_string(value)),
        Value::Null => Ok(String::new()),
        _ => Err("IDF: field must be a string or number".into()),
    }
}

/// 📤️ Converts epJSON fields to IDD positions, preserving blanks before later fields.
pub fn epjson_to_records(document: &Value, dictionary: &[IdfObject]) -> Result<Vec<IdfRecord>, String> {
    let root = document.as_object().ok_or("IDF: epJSON root must be an object")?;
    let mut records = Vec::new();
    for (kind, objects) in root.iter() {
        let schema = dictionary.iter().find(|item| item.kind == kind).ok_or_else(|| format!("IDF: unsupported object type {kind}"))?;
        for (name, object) in objects.as_object().ok_or("IDF: expected named objects")?.iter() {
            let object = object.as_object().ok_or("IDF: expected object fields")?;
            for (key, _) in object.iter() {
                if key != schema.extension && !schema.fields.iter().any(|field| field.name == key) { return Err(format!("IDF: unknown {kind} field {key}")); }
            }
            let mut fields = Vec::new();
            for field in schema.fields {
                fields.push(if field.name == "name" { name.to_string() } else { object.get(field.name).map(scalar_text).transpose()?.unwrap_or_default() });
            }
            if let Some(extension) = object.get(schema.extension) {
                let extension = extension.as_array().ok_or("IDF: extensible group must be an array")?;
                for group in extension {
                    let group = group.as_object().ok_or("IDF: expected extensible object")?;
                    if group.iter().any(|(key, _)| !schema.extensibles.iter().any(|field| field.name == key)) { return Err(format!("IDF: unknown extensible field in {kind}")); }
                    for field in schema.extensibles { fields.push(group.get(field.name).map(scalar_text).transpose()?.unwrap_or_default()); }
                }
            } else {
                while fields.last().is_some_and(String::is_empty) { fields.pop(); }
            }
            records.push(IdfRecord { kind: kind.into(), fields });
        }
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_blanks_unicode_and_crlf_preserve_positions() {
        let records = parse_records("\u{feff}! header\r\nZoNe, Büro, ! north axis\r\n,0,0,0;\nVersion,25.2;").unwrap();
        assert_eq!(records[0], IdfRecord { kind: "ZoNe".into(), fields: vec!["Büro".into(), "".into(), "0".into(), "0".into(), "0".into()] });
        assert_eq!(parse_records(&write_records(&records).unwrap()).unwrap(), records);
    }

    #[test]
    fn official_energyplus_fixture_preserves_every_positional_field() {
        let records = parse_records(include_str!("🧪️tests/1ZoneUncontrolled.idf")).unwrap();
        assert!(records.iter().any(|record| record.kind == "BuildingSurface:Detailed"));
        let document = records_to_epjson(&records, DICTIONARY).unwrap();
        let canonical = epjson_to_records(&document, DICTIONARY).unwrap();
        assert_eq!(records_to_epjson(&canonical, DICTIONARY).unwrap(), document);
        assert_eq!(parse_records(&write_records(&canonical).unwrap()).unwrap(), canonical);
        let building = document.as_object().unwrap().get("Building").unwrap().as_object().unwrap();
        let fields = building.get("Simple One Zone (Wireframe DXF)").unwrap().as_object().unwrap();
        assert_eq!(fields.get("terrain"), Some(&Value::String("Suburbs".into())));
        assert_eq!(fields.get("north_axis"), Some(&Value::from(0_i64)));
    }

    #[test]
    fn refuses_case_insensitive_duplicate_names_and_unknown_objects() {
        assert!(records_to_epjson(&parse_records("Zone,A;Zone,a;").unwrap(), DICTIONARY).is_err());
        assert!(records_to_epjson(&parse_records("Invented:Type,A;").unwrap(), DICTIONARY).is_err());
    }

    #[test]
    fn rejects_truncated_or_unrepresentable_records() {
        assert!(parse_records("Zone, A,").is_err());
        assert!(parse_records("; ").is_err());
        assert!(write_records(&[IdfRecord { kind: "Zone".into(), fields: vec!["A!B".into()] }]).is_err());
    }
}

#[path = "📚️.rs"]
mod dictionary;
pub use dictionary::DICTIONARY;

