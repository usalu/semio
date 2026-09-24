//! 🔮️ Mutation oracle for the JFIF 1.01 🧱️baseline conformance-class vocabulary.
//!
//! Reference: `libjpeg-jpg-jfif-1-01-baseline-marker-cli` (libjpeg-turbo, IJG AND BSD-3-Clause AND
//! Zlib), run as separate processes. [`read_axes`] takes the frame header and entropy-coding axes of the
//! real scan out of `djpeg -v -v`'s marker trace — the SOFn code, each frame component's sampling
//! factors, every DHT table's class and id, and whether a DAC segment is present — and the sample
//! precision out of `rdjpgcom -verbose`. Each kind is then applied to those axes as ITU-T T.81 defines
//! the field it names, and [`verdict`] reads the class off the specification's own tables. This
//! repository's decoder, snapshot and checker are never consulted.
//!
//! @see https://www.w3.org/Graphics/JPEG/itu-t81.pdf — Table B.1, §4.2, Annex F, §B.2.2, §B.2.4.2
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the vocabulary this module is measured against.

use semio_repo_test_host::Json;

//#region 🔖️Axes
/// 🧭️ The scan's baseline axes as libjpeg-turbo reads them.
#[derive(Clone, Debug, PartialEq)]
pub struct Axes {
    pub sof_marker: u8,
    pub precision: u32,
    pub arithmetic: bool,
    pub huffman_tables: Vec<(String, u32)>,
    pub components: Vec<(u32, u32, u32)>,
}

fn run(command: &str, args: &[&std::ffi::OsStr]) -> Result<String, String> {
    let output = std::process::Command::new(command).args(args).output().map_err(|error| format!("{command} could not be started: {error}"))?;
    Ok(format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)))
}

/// 📖️ Reads the axes of the JPEG at `document` with `djpeg -v -v` (decoded pixels go to a sibling
/// file) and `rdjpgcom -verbose`.
pub fn read_axes(document: &std::path::Path) -> Result<Axes, String> {
    let pixels = document.with_extension("djpeg.ppm");
    let trace = run("djpeg", &["-v".as_ref(), "-v".as_ref(), "-outfile".as_ref(), pixels.as_os_str(), document.as_os_str()])?;
    let summary = run("rdjpgcom", &["-verbose".as_ref(), document.as_os_str()])?;
    let hex = |text: &str| u32::from_str_radix(text.trim_start_matches("0x"), 16).map_err(|error| format!("libjpeg-turbo printed {text:?}: {error}"));
    let sof_marker = trace.lines().find_map(|line| line.strip_prefix("Start Of Frame ")).and_then(|rest| rest.split(':').next()).ok_or("djpeg printed no Start Of Frame")?;
    let precision = summary.lines().find_map(|line| line.split(", ").find_map(|part| part.strip_suffix(" bits per sample"))).ok_or("rdjpgcom printed no bits per sample")?;
    let mut huffman_tables = Vec::new();
    let mut components = Vec::new();
    let mut in_frame = false;
    for line in trace.lines() {
        if !line.starts_with(' ') {
            in_frame = line.starts_with("Start Of Frame ");
        }
        if let Some(table) = line.strip_prefix("Define Huffman Table ") {
            let value = hex(table.trim())?;
            huffman_tables.push((if value >> 4 == 0 { "dc" } else { "ac" }.to_string(), value & 0x0f));
        } else if let Some(component) = line.trim_start().strip_prefix("Component ").filter(|_| in_frame) {
            let (id, rest) = component.split_once(": ").ok_or_else(|| format!("djpeg component line {line:?}"))?;
            let sampling = rest.split_whitespace().next().unwrap_or("");
            let (h, v) = sampling.split_once("hx").ok_or_else(|| format!("djpeg sampling {sampling:?}"))?;
            let parse = |text: &str| text.trim_end_matches('v').parse::<u32>().map_err(|error| format!("djpeg sampling {text:?}: {error}"));
            components.push((parse(id)?, parse(h)?, parse(v)?));
        }
    }
    Ok(Axes {
        sof_marker: hex(sof_marker)? as u8,
        precision: precision.trim().parse().map_err(|error| format!("rdjpgcom precision {precision:?}: {error}"))?,
        arithmetic: trace.lines().any(|line| line.starts_with("Define Arithmetic Table")),
        huffman_tables,
        components,
    })
}
//#endregion 🔖️Axes

//#region 🔖️Semantics
fn number(params: &Json, key: &str) -> Result<u32, String> {
    match params.get(key) {
        Some(Json::Number(value)) => Ok(*value as u32),
        _ => Err(format!("params carry no numeric {key:?}")),
    }
}

fn flag(params: &Json, key: &str) -> bool {
    matches!(params.get(key), Some(Json::Bool(true)))
}

/// 🦠️ Applies one kind to the axes as T.81 defines the field it names. An insertion of a table or a
/// component that is already there, a removal of one that is not, and a sampling change of an absent
/// component change nothing; an index past the end appends.
pub fn apply(axes: &Axes, kind: &str, params: &Json) -> Result<Axes, String> {
    let mut next = axes.clone();
    match kind {
        "no-mutation" => {}
        "set-snapshot" => {
            next.sof_marker = number(params, "sofMarker")? as u8;
            next.precision = number(params, "precision")?;
            next.arithmetic = flag(params, "arithmetic");
        }
        "set-sof-marker" => next.sof_marker = number(params, "marker")? as u8,
        "set-sample-precision" => next.precision = number(params, "precision")?,
        "set-arithmetic" => next.arithmetic = flag(params, "arithmetic"),
        "insert-huffman-table" => {
            let key = (params.str("class"), number(params, "id")?);
            if !next.huffman_tables.contains(&key) {
                let at = (number(params, "index")? as usize).min(next.huffman_tables.len());
                next.huffman_tables.insert(at, key);
            }
        }
        "remove-huffman-table" => {
            let key = (params.str("class"), number(params, "id")?);
            next.huffman_tables.retain(|table| *table != key);
        }
        "insert-frame-component" => {
            let id = number(params, "id")?;
            if !next.components.iter().any(|component| component.0 == id) {
                let at = (number(params, "index")? as usize).min(next.components.len());
                next.components.insert(at, (id, number(params, "hSampling")?, number(params, "vSampling")?));
            }
        }
        "remove-frame-component" => {
            let id = number(params, "id")?;
            next.components.retain(|component| component.0 != id);
        }
        "set-component-sampling" => {
            let id = number(params, "id")?;
            let (h, v) = (number(params, "hSampling")?, number(params, "vSampling")?);
            for component in next.components.iter_mut().filter(|component| component.0 == id) {
                (component.1, component.2) = (h, v);
            }
        }
        other => return Err(format!("no T.81 baseline semantics for kind {other:?}")),
    }
    Ok(next)
}

/// 🛡️ The baseline class read off T.81's own tables: SOF0 only (Table B.1), 8-bit samples (§4.2),
/// Huffman entropy coding only (Annex F), at most two DC and two AC tables (§B.2.4.2), at most four
/// frame components and sampling factors within 1..=4 (§B.2.2).
pub fn verdict(axes: &Axes) -> Vec<&'static str> {
    let mut codes = Vec::new();
    if axes.sof_marker != 0xC0 {
        codes.push("stdio.jpg.baseline.sof-marker");
    }
    if axes.precision != 8 {
        codes.push("stdio.jpg.baseline.precision");
    }
    if axes.arithmetic {
        codes.push("stdio.jpg.baseline.arithmetic-conditioning-present");
    }
    let count = |class: &str| axes.huffman_tables.iter().filter(|table| table.0 == class).count();
    if count("dc") > 2 || count("ac") > 2 {
        codes.push("stdio.jpg.baseline.huffman-table-count");
    }
    if axes.components.len() > 4 {
        codes.push("stdio.jpg.baseline.component-sampling");
    }
    for component in &axes.components {
        if !(1..=4).contains(&component.1) || !(1..=4).contains(&component.2) {
            codes.push("stdio.jpg.baseline.component-sampling");
        }
    }
    codes
}
//#endregion 🔖️Semantics

//#region 🔖️Projection
/// 🎯️ The conformance projection the case compares.
pub fn project(axes: &Axes) -> Json {
    let strings = |values: Vec<String>| Json::Array(values.into_iter().map(Json::String).collect());
    Json::Object(vec![
        ("format".to_string(), Json::String("jpg-baseline".to_string())),
        ("sofMarker".to_string(), Json::String(format!("{:02x}", axes.sof_marker))),
        ("precision".to_string(), Json::Number(f64::from(axes.precision))),
        ("arithmetic".to_string(), Json::Bool(axes.arithmetic)),
        ("componentCount".to_string(), Json::Number(axes.components.len() as f64)),
        ("huffmanTables".to_string(), strings(axes.huffman_tables.iter().map(|(class, id)| format!("{class}:{id}")).collect())),
        ("components".to_string(), strings(axes.components.iter().map(|(id, h, v)| format!("{id}:{h}x{v}")).collect())),
        ("conformance".to_string(), strings(verdict(axes).into_iter().map(str::to_string).collect())),
    ])
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
