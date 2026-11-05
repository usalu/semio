//! 🔣️ The declared Puzzle2d wire visits only the schema's binary64 field roles.
use semio_framework_value::{DslValue, Number};

pub(crate) const SCHEMA: &str = include_str!("🔣️.json");

fn member<'a>(value: &'a mut DslValue, key: &str) -> Result<Option<&'a mut DslValue>, String> {
    let DslValue::Object(entries) = value else { return Err("Puzzle2d JSON requires an object".into()); };
    let mut found = None;
    for (name, value) in entries {
        if name == key {
            if found.is_some() { return Err(format!("Puzzle2d JSON repeats field {key}")); }
            found = Some(value);
        }
    }
    Ok(found)
}

fn field(value: &mut DslValue, key: &str, visit: fn(&mut DslValue, bool) -> Result<(), String>, decode: bool) -> Result<(), String> {
    if let Some(value) = member(value, key)? {
        if !matches!(value, DslValue::Null) { visit(value, decode)?; }
    }
    Ok(())
}

fn rows(value: &mut DslValue, visit: fn(&mut DslValue, bool) -> Result<(), String>, decode: bool) -> Result<(), String> {
    let DslValue::Array(values) = value else { return Err("Puzzle2d JSON requires an array".into()); };
    for value in values { visit(value, decode)?; }
    Ok(())
}

fn word(value: &mut DslValue, decode: bool) -> Result<(), String> {
    if decode {
        let DslValue::Object(entries) = value else { return Err("Puzzle2d JSON binary64 requires a word object".into()); };
        if entries.len() != 1 || entries[0].0 != "bits" { return Err("Puzzle2d JSON binary64 requires only bits".into()); }
        let DslValue::String(text) = &entries[0].1 else { return Err("Puzzle2d JSON bits require text".into()); };
        if text.len() != 16 || !text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
            return Err("Puzzle2d JSON bits require sixteen lowercase hexadecimal digits".into());
        }
        let bits = u64::from_str_radix(text, 16).map_err(|error| error.to_string())?;
        *value = DslValue::float(f64::from_bits(bits));
    } else {
        let DslValue::Number(Number::Float(number)) = value else { return Err("Puzzle2d JSON native binary64 is absent".into()); };
        let bits = number.to_bits();
        *value = DslValue::object([("bits".into(), DslValue::String(format!("{bits:016x}")))]);
    }
    Ok(())
}

fn fields(value: &mut DslValue, keys: &[&str], decode: bool) -> Result<(), String> {
    for key in keys { field(value, key, word, decode)?; }
    Ok(())
}

fn camera(value: &mut DslValue, decode: bool) -> Result<(), String> { fields(value, &["x", "y", "zoom"], decode) }
fn handle(value: &mut DslValue, decode: bool) -> Result<(), String> { fields(value, &["angle", "radius", "scale"], decode) }
fn handles(value: &mut DslValue, decode: bool) -> Result<(), String> { rows(value, handle, decode) }
fn node(value: &mut DslValue, decode: bool) -> Result<(), String> {
    fields(value, &["x", "y", "radius", "width", "height", "scale"], decode)?;
    field(value, "handles", handles, decode)
}
fn nodes(value: &mut DslValue, decode: bool) -> Result<(), String> { rows(value, node, decode) }
fn edge(value: &mut DslValue, decode: bool) -> Result<(), String> { fields(value, &["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"], decode) }
fn edges(value: &mut DslValue, decode: bool) -> Result<(), String> { rows(value, edge, decode) }
fn region(value: &mut DslValue, decode: bool) -> Result<(), String> { fields(value, &["x", "y", "width", "height"], decode) }
fn regions(value: &mut DslValue, decode: bool) -> Result<(), String> { rows(value, region, decode) }
fn template(value: &mut DslValue, decode: bool) -> Result<(), String> { fields(value, &["angle", "t", "radius"], decode) }
fn templates(value: &mut DslValue, decode: bool) -> Result<(), String> { rows(value, template, decode) }
fn catalog_node(value: &mut DslValue, decode: bool) -> Result<(), String> { field(value, "handles", templates, decode) }
fn catalog_nodes(value: &mut DslValue, decode: bool) -> Result<(), String> { rows(value, catalog_node, decode) }
fn catalogs(value: &mut DslValue, decode: bool) -> Result<(), String> { field(value, "nodes", catalog_nodes, decode) }
fn meta(value: &mut DslValue, decode: bool) -> Result<(), String> { field(value, "kindCatalogs", catalogs, decode) }

pub(crate) fn convert(mut value: DslValue, decode: bool) -> Result<DslValue, String> {
    field(&mut value, "camera", camera, decode)?;
    field(&mut value, "nodes", nodes, decode)?;
    field(&mut value, "edges", edges, decode)?;
    field(&mut value, "targetRegions", regions, decode)?;
    field(&mut value, "meta", meta, decode)?;
    Ok(value)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
