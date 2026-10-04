//! 🖐️ Explicit Block5d JSON scalar paths preserve closed binary64 words.
use semio_framework_value::DslValue;
fn field(v: &mut DslValue, key: &str, f: fn(&mut DslValue, bool) -> Result<(), String>, decode: bool) -> Result<(), String> {
    let semio_framework_value::DslValue::Object(entries) = v else { return Err("Block5d JSON object required".into()) };
    if let Some((_, v)) = entries.iter_mut().find(|(name, _)| name == key) {
        if !matches!(v, DslValue::Null) {
            f(v, decode)?;
        }
    }
    Ok(())
}
fn rows(v: &mut DslValue, f: fn(&mut DslValue, bool) -> Result<(), String>, decode: bool) -> Result<(), String> {
    let semio_framework_value::DslValue::Array(values) = v else { return Err("Block5d JSON array required".into()) };
    for v in values {
        f(v, decode)?;
    }
    Ok(())
}
fn word(v: &mut DslValue, decode: bool) -> Result<(), String> {
    if decode {
        let semio_framework_value::DslValue::Object(entries) = v else { return Err("Block5d JSON closed binary64 object required".into()) };
        if entries.len() != 1 || entries[0].0 != "bits" {
            return Err("Block5d JSON scalar requires only bits".into());
        }
        let raw = entries[0].1.as_str().ok_or("Block5d JSON bits require text")?;
        if raw.len() != 16 || !raw.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
            return Err("Block5d JSON bits require sixteen lowercase hexadecimal digits".into());
        }
        let bits = u64::from_str_radix(raw, 16).map_err(|e| e.to_string())?;
        *v = semio_framework_value::DslValue::float(f64::from_bits(bits));
    } else {
        let raw = v.as_f64().ok_or("Block5d owned scalar required")?.to_bits();
        *v = semio_framework_value::DslValue::object([("bits".into(), semio_framework_value::DslValue::String(format!("{raw:016x}")))]);
    }
    Ok(())
}
fn vector(v: &mut DslValue, decode: bool) -> Result<(), String> {
    rows(v, word, decode)
}
fn part2d(v: &mut DslValue, decode: bool) -> Result<(), String> {
    field(v, "radius", word, decode)?;
    field(v, "width", word, decode)?;
    field(v, "height", word, decode)
}
fn part3d(v: &mut DslValue, decode: bool) -> Result<(), String> {
    field(v, "orientation", vector, decode)?;
    field(v, "scale", vector, decode)
}
fn grip(v: &mut DslValue, decode: bool) -> Result<(), String> {
    field(v, "angle", word, decode)?;
    field(v, "radius2d", word, decode)?;
    field(v, "position", vector, decode)?;
    field(v, "direction", vector, decode)?;
    field(v, "radius3d", word, decode)
}
fn grips(v: &mut DslValue, decode: bool) -> Result<(), String> {
    rows(v, grip, decode)
}
fn camera2d(v: &mut DslValue, decode: bool) -> Result<(), String> {
    field(v, "x", word, decode)?;
    field(v, "y", word, decode)?;
    field(v, "zoom", word, decode)
}
fn camera3d(v: &mut DslValue, decode: bool) -> Result<(), String> {
    field(v, "position", vector, decode)?;
    field(v, "target", vector, decode)?;
    field(v, "zoom", word, decode)
}
pub(crate) fn convert(mut v: DslValue, decode: bool) -> Result<DslValue, String> {
    field(&mut v, "part2d", part2d, decode)?;
    field(&mut v, "part3d", part3d, decode)?;
    field(&mut v, "grips", grips, decode)?;
    field(&mut v, "camera2d", camera2d, decode)?;
    field(&mut v, "camera3d", camera3d, decode)?;
    Ok(v)
}
