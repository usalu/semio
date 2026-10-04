//! 🧊️ Explicit Puzzle3d JSON scalar paths preserve closed binary64 words.
use semio_framework_value::DslValue;
fn field(v: &mut DslValue, key: &str, f: fn(&mut DslValue, bool) -> Result<(), String>, decode: bool) -> Result<(), String> {
    let semio_framework_value::DslValue::Object(entries) = v else { return Err("Puzzle3d JSON object required".into()) };
    if let Some((_, v)) = entries.iter_mut().find(|(name, _)| name == key) {
        if !matches!(v, DslValue::Null) {
            f(v, decode)?;
        }
    }
    Ok(())
}
fn rows(v: &mut DslValue, f: fn(&mut DslValue, bool) -> Result<(), String>, decode: bool) -> Result<(), String> {
    let semio_framework_value::DslValue::Array(values) = v else { return Err("Puzzle3d JSON array required".into()) };
    for v in values {
        f(v, decode)?;
    }
    Ok(())
}
fn word(v: &mut DslValue, decode: bool) -> Result<(), String> {
    if decode {
        if v.as_f64().is_some_and(f64::is_finite) {
            return Ok(());
        }
        let semio_framework_value::DslValue::Object(entries) = v else { return Err("Puzzle3d JSON finite number or closed binary64 object required".into()) };
        if entries.len() != 1 || entries[0].0 != "bits" {
            return Err("Puzzle3d JSON scalar requires only bits".into());
        }
        let raw = entries[0].1.as_str().ok_or("Puzzle3d JSON bits require text")?;
        if raw.len() != 16 || !raw.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
            return Err("Puzzle3d JSON bits require sixteen lowercase hexadecimal digits".into());
        }
        let bits = u64::from_str_radix(raw, 16).map_err(|e| e.to_string())?;
        *v = semio_framework_value::DslValue::float(f64::from_bits(bits));
    } else {
        let raw = v.as_f64().ok_or("Puzzle3d owned scalar required")?.to_bits();
        *v = semio_framework_value::DslValue::object([("bits".into(), semio_framework_value::DslValue::String(format!("{raw:016x}")))]);
    }
    Ok(())
}
fn vector(v: &mut DslValue, decode: bool) -> Result<(), String> {
    rows(v, word, decode)
}
fn scale(v: &mut DslValue, d: bool) -> Result<(), String> {
    if matches!(v, DslValue::Array(_)) {
        vector(v, d)
    } else {
        word(v, d)
    }
}
fn vortex(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "position", vector, d)?;
    field(v, "direction", vector, d)?;
    field(v, "radius", word, d)
}
fn vortices(v: &mut DslValue, d: bool) -> Result<(), String> {
    rows(v, vortex, d)
}
fn object(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "origin", vector, d)?;
    field(v, "orientation", vector, d)?;
    field(v, "scale", scale, d)?;
    field(v, "vortices", vortices, d)
}
fn objects(v: &mut DslValue, d: bool) -> Result<(), String> {
    rows(v, object, d)
}
fn attraction(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "gap", word, d)?;
    field(v, "shift", word, d)?;
    field(v, "rise", word, d)?;
    field(v, "rotation", word, d)?;
    field(v, "turn", word, d)?;
    field(v, "tilt", word, d)?;
    field(v, "x", word, d)?;
    field(v, "y", word, d)
}
fn attractions(v: &mut DslValue, d: bool) -> Result<(), String> {
    rows(v, attraction, d)
}
fn target(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "origin", vector, d)?;
    field(v, "orientation", vector, d)?;
    field(v, "scale", scale, d)
}
fn targets(v: &mut DslValue, d: bool) -> Result<(), String> {
    rows(v, target, d)
}
fn reference(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "origin", vector, d)?;
    field(v, "widthWorld", word, d)
}
fn references(v: &mut DslValue, d: bool) -> Result<(), String> {
    rows(v, reference, d)
}
fn template(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "point", vector, d)?;
    field(v, "direction", vector, d)?;
    field(v, "t", word, d)?;
    field(v, "radius", word, d)
}
fn templates(v: &mut DslValue, d: bool) -> Result<(), String> {
    rows(v, template, d)
}
fn kind(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "vortices", templates, d)
}
fn kinds(v: &mut DslValue, d: bool) -> Result<(), String> {
    rows(v, kind, d)
}
fn catalog(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "objects", kinds, d)
}
fn meta(v: &mut DslValue, d: bool) -> Result<(), String> {
    field(v, "kindCatalogs", catalog, d)
}
pub(crate) fn convert(mut v: DslValue, decode: bool) -> Result<DslValue, String> {
    field(&mut v, "meta", meta, decode)?;
    field(&mut v, "objects", objects, decode)?;
    field(&mut v, "attractions", attractions, decode)?;
    field(&mut v, "targetVolumes", targets, decode)?;
    field(&mut v, "references", references, decode)?;
    Ok(v)
}
