//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `calamine` (reader) + `rust_xlsxwriter` (writer) reference pairing so the subject's
//! own mutation has an independent result to be compared against instead of being checked against
//! its own reading.
//!
//! **The constraint that shapes this module** — no single crate both reads and modifies an XLSX.
//! `calamine` 0.36 parses a workbook into resolved cell values but exposes no accessor for the raw
//! shared-string table (`Xlsx<RS>::strings: Vec<String>` and `read_shared_strings` are both private
//! — confirmed by reading `calamine-0.36.1/src/xlsx/mod.rs`); it collapses `t="s"` shared-string
//! references and `t="inlineStr"` literal text into the same resolved `Data::String`. `rust_xlsxwriter`
//! 0.96 can only assemble a brand-new package — never open and patch an existing one — and its own
//! shared-string table (`shared_strings_table.rs`) is populated ONLY as a byproduct of `write_string`
//! on a cell; there is no API to insert, remove or target a pool entry independent of a cell write.
//! Concretely: sheet/cell mutations (`InsertSheet`, `RemoveSheet`, `RenameSheet`, `SetCell`,
//! `RemoveCell`, `SetSnapshot`, `NoMutation`) round-trip through "read the whole workbook into a
//! grid, apply the change to the grid, rebuild the whole workbook from the grid" — a genuine second
//! producer, hence `@mode-differential`. `InsertSharedString`/`RemoveSharedString`/`SetSharedString`
//! address the shared-string pool by an INDEX that is independent of any cell reference — exactly the
//! axis neither reference crate exposes — so this module cannot independently perform them; it
//! reports that honestly (see `oracle_apply_mutation`'s dispatch below) rather than faking a
//! differential result, per the fleet brief's §6.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Grid
/// 🔢️ A cell value as the independent reader/writer pairing can observe and reproduce it — the
/// `Data::Int`/`Data::Float` split `calamine` draws collapses to one numeric case here since XLSX
/// itself stores every number as an IEEE-754 double (ECMA-376 §18.17.2, ST_Xstring numeric literal),
/// never a real reader/writer distinction.
#[cfg(feature = "oracles")]
#[derive(Clone, Debug, PartialEq)]
enum GridValue {
    Number(f64),
    Bool(bool),
    Text(String),
}

/// 📄 One sheet as `(name, [(row, col, value)])` — `row`/`col` are 0-based, `calamine`'s and
/// `rust_xlsxwriter`'s own native convention (the subset's OWN `XlsxCell::row` is 1-based; the JSON
/// spec this module reads carries the subset's 1-based convention, converted at the boundary below,
/// so one wire contract serves both this module and the subject's own mutation code).
#[cfg(feature = "oracles")]
type GridSheet = (String, Vec<(u32, u32, GridValue)>);

/// 📥️ Independent read: every sheet, every non-empty cell, through `calamine`'s resolved
/// `Data` — this IS the reader's own semantic view; it cannot and does not distinguish a
/// shared-string reference from an inline string (see module doc comment).
#[cfg(feature = "oracles")]
fn read_workbook_grid(input: &[u8]) -> Result<Vec<GridSheet>, String> {
    use calamine::{Data, Reader, Xlsx};
    let mut workbook: Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(input)).map_err(|error| format!("independent reader could not open the workbook: {error}"))?;
    let mut sheets = Vec::new();
    for name in workbook.sheet_names() {
        let range = workbook.worksheet_range(&name).map_err(|error| format!("independent reader could not read sheet {name:?}: {error}"))?;
        let mut cells = Vec::new();
        for (row, col, value) in range.used_cells() {
            let value = match value {
                Data::Int(v) => GridValue::Number(*v as f64),
                Data::Float(v) => GridValue::Number(*v),
                Data::String(v) => GridValue::Text(v.clone()),
                Data::Bool(v) => GridValue::Bool(*v),
                Data::DateTimeIso(v) => GridValue::Text(v.clone()),
                Data::DurationIso(v) => GridValue::Text(v.clone()),
                Data::DateTime(v) => GridValue::Text(format!("{v:?}")),
                Data::Error(kind) => return Err(format!("sheet {name:?} cell ({row},{col}) is a formula error the independent reader cannot project: {kind:?}")),
                Data::Empty => continue,
            };
            cells.push((row as u32, col as u32, value));
        }
        sheets.push((name, cells));
    }
    Ok(sheets)
}

/// 📤️ Independent write: assembles a BRAND-NEW package from `sheets` — `rust_xlsxwriter` has no
/// "open and patch" path (see module doc comment), so every oracle mutation below rebuilds the
/// entire workbook from its post-mutation grid rather than editing the original bytes.
#[cfg(feature = "oracles")]
fn write_workbook_grid(sheets: &[GridSheet]) -> Result<Vec<u8>, String> {
    use rust_xlsxwriter::Workbook;
    let mut workbook = Workbook::new();
    for (name, cells) in sheets {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(name).map_err(|error| format!("independent writer rejected sheet name {name:?}: {error}"))?;
        for (row, col, value) in cells {
            let col = u16::try_from(*col).map_err(|_| format!("column {col} exceeds the independent writer's column range"))?;
            let write_result = match value {
                GridValue::Number(n) => worksheet.write_number(*row, col, *n),
                GridValue::Bool(b) => worksheet.write_boolean(*row, col, *b),
                GridValue::Text(t) => worksheet.write_string(*row, col, t.as_str()),
            };
            write_result.map_err(|error| format!("independent writer could not write cell ({row},{col}) on sheet {name:?}: {error}"))?;
        }
    }
    workbook.save_to_buffer().map_err(|error| format!("independent writer could not assemble the workbook: {error}"))
}
//#endregion 🔖️Grid

//#region 🔖️SpecReaders
/// 🔀️ This module's own JSON wire contract: `row` is 1-based (the subset's `XlsxCell::row`
/// convention, ECMA-376's own `<row r="N">` index), `col` is 0-based — matching both sides so one
/// spec drives the subject's typed mutation AND this module's grid without a second translation.
#[cfg(feature = "oracles")]
fn mutation_params(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}
#[cfg(feature = "oracles")]
fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn string(value: &Json, key: &str) -> String {
    match value.get(key) {
        Some(Json::String(text)) => text.clone(),
        _ => String::new(),
    }
}
#[cfg(feature = "oracles")]
fn json_to_grid_value(value: &Json) -> Result<GridValue, String> {
    match value {
        Json::Number(n) => Ok(GridValue::Number(*n)),
        Json::Bool(b) => Ok(GridValue::Bool(*b)),
        Json::String(s) => Ok(GridValue::Text(s.clone())),
        other => Err(format!("cell value must be a number, boolean or string, got {other:?}")),
    }
}
/// 🔁️ One-based (`XlsxCell::row` convention) -> zero-based (this module's/`calamine`'s convention).
#[cfg(feature = "oracles")]
fn row0(one_based_row: f64) -> Result<u32, String> {
    let row = one_based_row as i64;
    if row < 1 {
        return Err(format!("row must be >= 1 (1-based), got {row}"));
    }
    Ok((row - 1) as u32)
}
#[cfg(feature = "oracles")]
fn cells_from_json(value: &Json, key: &str) -> Result<Vec<(u32, u32, GridValue)>, String> {
    value
        .array(key)
        .iter()
        .map(|entry| {
            let row = row0(number(entry, "row").ok_or_else(|| format!("{key} entry missing `row`"))?)?;
            let col = number(entry, "col").ok_or_else(|| format!("{key} entry missing `col`"))? as u32;
            let value = json_to_grid_value(entry.get("value").ok_or_else(|| format!("{key} entry missing `value`"))?)?;
            Ok((row, col, value))
        })
        .collect()
}
#[cfg(feature = "oracles")]
fn sheets_from_json(value: &Json, key: &str) -> Result<Vec<GridSheet>, String> {
    value.array(key).iter().map(|entry| Ok((string(entry, "name"), cells_from_json(entry, "cells")?))).collect()
}
//#endregion 🔖️SpecReaders

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
///
/// `insert-shared-string`/`remove-shared-string`/`set-shared-string` do NOT go through
/// `calamine`/`rust_xlsxwriter`: the raw pool those three address is invisible to the first's read
/// model and unreachable by index through the second's write API. They go through the `zip` +
/// `quick-xml` pairing instead (see the [`shared_strings`] module), which reads and rewrites
/// `xl/sharedStrings.xml` as the OPC PART it is. That is a genuine second producer for an
/// index-addressed pool edit, and [`project_shared_string_pool`] reads the result back out of the
/// bytes — nothing about the pool is carried by the caller any more.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => write_workbook_grid(&sheets_from_json(&params, "sheets")?),
        "insert-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            sheets.push((string(&params, "name"), cells_from_json(&params, "cells")?));
            write_workbook_grid(&sheets)
        }
        "remove-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            let name = string(&params, "name");
            let before = sheets.len();
            sheets.retain(|(sheet_name, _)| sheet_name != &name);
            if sheets.len() == before {
                return Err(format!("remove-sheet: no sheet named {name:?}"));
            }
            write_workbook_grid(&sheets)
        }
        "rename-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            let name = string(&params, "name");
            let new_name = string(&params, "newName");
            let sheet = sheets.iter_mut().find(|(sheet_name, _)| sheet_name == &name).ok_or_else(|| format!("rename-sheet: no sheet named {name:?}"))?;
            sheet.0 = new_name;
            write_workbook_grid(&sheets)
        }
        "set-cell" => {
            let mut sheets = read_workbook_grid(input)?;
            let sheet_name = string(&params, "sheetName");
            let row = row0(number(&params, "row").ok_or("set-cell: missing `row`")?)?;
            let col = number(&params, "col").ok_or("set-cell: missing `col`")? as u32;
            let value = json_to_grid_value(params.get("value").ok_or("set-cell: missing `value`")?)?;
            let (_, cells) = sheets.iter_mut().find(|(name, _)| name == &sheet_name).ok_or_else(|| format!("set-cell: no sheet named {sheet_name:?}"))?;
            match cells.iter_mut().find(|(r, c, _)| *r == row && *c == col) {
                Some(cell) => cell.2 = value,
                None => cells.push((row, col, value)),
            }
            write_workbook_grid(&sheets)
        }
        "remove-cell" => {
            let mut sheets = read_workbook_grid(input)?;
            let sheet_name = string(&params, "sheetName");
            let row = row0(number(&params, "row").ok_or("remove-cell: missing `row`")?)?;
            let col = number(&params, "col").ok_or("remove-cell: missing `col`")? as u32;
            let (_, cells) = sheets.iter_mut().find(|(name, _)| name == &sheet_name).ok_or_else(|| format!("remove-cell: no sheet named {sheet_name:?}"))?;
            cells.retain(|(r, c, _)| !(*r == row && *c == col));
            write_workbook_grid(&sheets)
        }
        "insert-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            pool.push(string(&params, "value"));
            shared_strings::write_pool(input, &pool)
        }
        "remove-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            let index = number(&params, "index").ok_or("remove-shared-string: missing `index`")?.max(0.0) as usize;
            if index >= pool.len() {
                return Err(format!("remove-shared-string: index {index} is outside the {}-entry pool", pool.len()));
            }
            pool.remove(index);
            shared_strings::write_pool(input, &pool)
        }
        "set-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            let index = number(&params, "index").ok_or("set-shared-string: missing `index`")?.max(0.0) as usize;
            let value = string(&params, "value");
            let slot = pool.get_mut(index).ok_or_else(|| format!("set-shared-string: index {index} is outside the pool"))?;
            if *slot == value {
                return Err(format!("set-shared-string: the pool already holds {value:?} at index {index} — the mutation would be unobservable"));
            }
            *slot = value;
            shared_strings::write_pool(input, &pool)
        }
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `calamine` + `rust_xlsxwriter`
/// pairing every mutation above uses — proves the reference pairing itself is stable on the real
/// fixture before the subject's own codec is asked to be. Genuinely rebuilds the package (never a
/// literal byte passthrough): `rust_xlsxwriter` cannot reproduce another writer's object layout, so
/// this is a real, if weak, round trip rather than an identity operation dressed up as one.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    write_workbook_grid(&read_workbook_grid(input)?)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️SharedStringPool
/// 📑️ The raw `xl/sharedStrings.xml` pool, read and written through the `zip` + `quick-xml` pairing
/// this owner already registers and the six OOXML conformance subsets already run on
/// (`crate::document::ooxml`), NOT through `calamine`/`rust_xlsxwriter`.
///
/// 🪞️ This replaces a documented gap rather than papering over one. `calamine`'s `Xlsx<RS>::strings`
/// and `read_shared_strings` are both private, and `rust_xlsxwriter` populates its own table only as
/// a byproduct of `write_string`, so neither can address the pool BY INDEX — which is exactly what
/// this catalog's three shared-string kinds do. The conclusion drawn from that was that no second
/// producer existed and the three kinds had to return their input unchanged. That conclusion was
/// wrong: the pool is a part of an OPC package, and a second producer for a PART is the container
/// codec plus an XML reader/writer, which this owner has linked all along. `calamine` and
/// `rust_xlsxwriter` remain the reference pairing for the GRID; the pool is a storage-layer part and
/// gets the reference pairing a part deserves.
///
/// 📐️ ECMA-376 §18.4: `sst` holds `si` items, each a string item whose text is its `t` runs
/// concatenated (a rich-text `si` splits its text across `r`/`t` children). `count` is the number of
/// cell references into the pool and `uniqueCount` the number of items; a rewrite here can only know
/// the second, so it writes `uniqueCount` and leaves `count` equal to it — an ECMA-376-legal
/// declaration, and the same one every writer that rebuilds a pool from scratch emits.
#[cfg(feature = "oracles")]
pub mod shared_strings {
    use crate::document::ooxml::{part_bytes, read_parts, set_part, write_parts};
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    pub const PART: &str = "xl/sharedStrings.xml";
    const SST_NAMESPACE: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";

    /// 👁️ Every `si` item of the real pool, in document order, each one's `t` runs concatenated.
    pub fn read_pool(input: &[u8]) -> Result<Vec<String>, String> {
        let parts = read_parts(input)?;
        let Some(bytes) = part_bytes(&parts, PART) else { return Ok(Vec::new()) };
        let text = std::str::from_utf8(bytes).map_err(|error| format!("{PART} is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let mut pool: Vec<String> = Vec::new();
        let mut in_item = false;
        let mut in_text = false;
        let mut current = String::new();
        loop {
            match reader.read_event().map_err(|error| format!("independent XML reader could not read {PART}: {error}"))? {
                Event::Start(start) => match local_name(start.name().as_ref()) {
                    "si" => {
                        in_item = true;
                        current.clear();
                    }
                    "t" if in_item => in_text = true,
                    _ => {}
                },
                Event::Text(run) if in_text => current.push_str(&run.xml10_content()),
                // 🔣️ `quick-xml` 0.42 surfaces `&amp;`, `&lt;`, `&#233;` and friends as their own
                // event rather than folding them into the text, so a shared string carrying one is
                // reassembled here instead of silently losing the character.
                Event::GeneralRef(reference) if in_text => current.push_str(&resolve_reference(&reference.xml10_content())?),
                Event::End(end) => match local_name(end.name().as_ref()) {
                    "t" => in_text = false,
                    "si" if in_item => {
                        in_item = false;
                        pool.push(std::mem::take(&mut current));
                    }
                    _ => {}
                },
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(pool)
    }

    /// ✍️ Rewrites `xl/sharedStrings.xml` from `pool` alone and reassembles the whole container from
    /// its parts — never a patch of the input bytes. Every other part is carried through verbatim.
    pub fn write_pool(input: &[u8], pool: &[String]) -> Result<Vec<u8>, String> {
        let mut parts = read_parts(input)?;
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>");
        xml.push_str(&format!("<sst xmlns=\"{SST_NAMESPACE}\" count=\"{}\" uniqueCount=\"{}\">", pool.len(), pool.len()));
        for item in pool {
            xml.push_str("<si><t xml:space=\"preserve\">");
            xml.push_str(&escape(item));
            xml.push_str("</t></si>");
        }
        xml.push_str("</sst>");
        set_part(&mut parts, PART, xml.into_bytes());
        write_parts(&parts)
    }

    /// 🔣️ The five predefined XML entities plus numeric character references — every general
    /// reference ECMA-376 §18.4 string content may legally carry without a DTD.
    fn resolve_reference(name: &str) -> Result<String, String> {
        Ok(match name {
            "amp" => "&".to_string(),
            "lt" => "<".to_string(),
            "gt" => ">".to_string(),
            "apos" => "'".to_string(),
            "quot" => "\"".to_string(),
            other => {
                let code = match other.strip_prefix("#x").or_else(|| other.strip_prefix("#X")) {
                    Some(hex) => u32::from_str_radix(hex, 16).map_err(|error| format!("{PART} carries an unreadable character reference &{other};: {error}"))?,
                    None => other.strip_prefix('#').ok_or_else(|| format!("{PART} carries an undeclared entity reference &{other};"))?.parse().map_err(|error| format!("{PART} carries an unreadable character reference &{other};: {error}"))?,
                };
                char::from_u32(code).ok_or_else(|| format!("{PART} carries character reference &{other}; which is not a Unicode scalar"))?.to_string()
            }
        })
    }

    fn local_name(name: &str) -> &str {
        match name.find(':') {
            Some(index) => &name[index + 1..],
            None => name,
        }
    }

    fn escape(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        for character in text.chars() {
            match character {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                other => out.push(other),
            }
        }
        out
    }
}

/// 👁️ Projects the raw shared-string pool the three index-addressed kinds actually move — read out
/// of the BYTES by the independent `zip` + `quick-xml` implementation, not tracked by the caller.
/// Deliberately separate from [`project_xlsx_workbook`]: the pool is a STORAGE-layer optimisation
/// (ECMA-376 §18.4 — a workbook whose cells carry inline strings instead is semantically the same
/// workbook), and the grid kinds run through `rust_xlsxwriter`, which legitimately renormalises the
/// pool while preserving every cell value. Holding a grid mutation to a pool it never claimed to
/// preserve would report a false divergence; holding a pool mutation to the pool is the real test.
#[cfg(feature = "oracles")]
pub fn project_shared_string_pool(input: &[u8]) -> Result<Json, String> {
    let pool = shared_strings::read_pool(input)?;
    Ok(Json::Object(vec![
        ("sharedStringCount".to_string(), Json::Number(pool.len() as f64)),
        ("sharedStrings".to_string(), Json::Array(pool.into_iter().map(Json::String).collect())),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_shared_string_pool(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ The undo of one of the three pool kinds, read out of `base` by the independent implementation
/// alone.
///
/// ⚠️ `remove-shared-string` at an INTERIOR index has no inverse in this vocabulary, and that is a
/// property of the vocabulary rather than of this oracle: `XlsxMutation::InsertSharedString` carries
/// only a `value` and appends at `shared_strings.len()`
/// (`../🧬️schema/🧬️mutations/🦀️component.rs:145`), so nothing in the catalog can put a string back at
/// position 7 of 229. Only removing the LAST entry is invertible, which is why this case's Examples
/// row addresses index 228 of the real 229-entry pool. Reported rather than worked around; the
/// production `inverse()` at that file's line 173 answers `SetSharedString`, which restores neither
/// the length nor the element that shifted into the hole.
#[cfg(feature = "oracles")]
pub fn shared_string_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    let pool = shared_strings::read_pool(base)?;
    let params = mutation_params(forward);
    let index = match params.get("index") {
        Some(Json::Number(value)) => value.max(0.0) as usize,
        _ => 0,
    };
    let object = |pairs: Vec<(&str, Json)>| Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let spec = |kind: &str, pairs: Vec<(&str, Json)>| object(vec![("kind", Json::String(kind.to_string())), ("params", object(pairs))]);
    Ok(match forward.str("kind").as_str() {
        "insert-shared-string" => spec("remove-shared-string", vec![("index", Json::Number(pool.len() as f64))]),
        "remove-shared-string" => {
            let value = pool.get(index).ok_or_else(|| format!("remove-shared-string has no inverse: index {index} is outside the base's {}-entry pool", pool.len()))?;
            if index + 1 != pool.len() {
                return Err(format!(
                    "remove-shared-string at index {index} of {} has no inverse in this vocabulary: insert-shared-string carries only a value and appends, so no declared kind can put a string back at an interior position",
                    pool.len()
                ));
            }
            spec("insert-shared-string", vec![("value", Json::String(value.clone()))])
        }
        "set-shared-string" => {
            let value = pool.get(index).ok_or_else(|| format!("set-shared-string has no inverse: index {index} is outside the base's {}-entry pool", pool.len()))?;
            spec("set-shared-string", vec![("index", Json::Number(index as f64)), ("value", Json::String(value.clone()))])
        }
        other => return Err(format!("{other:?} is not one of this catalog's shared-string kinds")),
    })
}

#[cfg(not(feature = "oracles"))]
pub fn shared_string_inverse_spec(_base: &[u8], _forward: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️SharedStringPool

//#region 🔖️Projection
/// 👁️ Projects XLSX bytes with the INDEPENDENT `calamine` reader onto the `semantic-spreadsheet-v1`
/// shape this case's oracle and subject are both compared through. `expected_shared_string_count` is
/// caller-tracked metadata (like `csv`'s `has_header`, see that subset's own oracle module) rather
/// than read from these bytes — `calamine` cannot observe the raw pool size either (module doc
/// comment); the caller computes the oracle side by arithmetic and reads the subject side from its
/// own real `XlsxWorkbook::shared_strings.len()`, so the two are still genuinely compared, just not
/// both independently derived from bytes.
#[cfg(feature = "oracles")]
pub fn project_xlsx_workbook(bytes: &[u8], expected_shared_string_count: usize) -> Result<Json, String> {
    let sheets = read_workbook_grid(bytes)?;
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("xlsx".to_string())),
        ("sharedStringCount".to_string(), Json::Number(expected_shared_string_count as f64)),
        (
            "sheets".to_string(),
            Json::Array(
                sheets
                    .into_iter()
                    .map(|(name, cells)| {
                        Json::Object(vec![
                            ("name".to_string(), Json::String(name)),
                            (
                                "cells".to_string(),
                                Json::Array(
                                    cells
                                        .into_iter()
                                        .map(|(row, col, value)| {
                                            Json::Object(vec![
                                                ("row".to_string(), Json::Number((row + 1) as f64)),
                                                ("col".to_string(), Json::Number(col as f64)),
                                                (
                                                    "value".to_string(),
                                                    match value {
                                                        GridValue::Number(n) => Json::Number(n),
                                                        GridValue::Bool(b) => Json::Bool(b),
                                                        GridValue::Text(t) => Json::String(t),
                                                    },
                                                ),
                                            ])
                                        })
                                        .collect(),
                                ),
                            ),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn project_xlsx_workbook(_bytes: &[u8], _expected_shared_string_count: usize) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    fn spec(kind: &str, params: Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
    }
    fn cell(row: f64, col: f64, value: Json) -> Json {
        Json::Object(vec![("row".to_string(), Json::Number(row)), ("col".to_string(), Json::Number(col)), ("value".to_string(), value)])
    }
    fn sheet(name: &str, cells: Vec<Json>) -> Json {
        Json::Object(vec![("name".to_string(), Json::String(name.to_string())), ("cells".to_string(), Json::Array(cells))])
    }

    /// 🧫️ The REAL committed workbook this subset's case runs on — 11 parts, two worksheets and a
    /// genuine 229-entry shared-string table. The synthetic [`fixture_bytes`] below is a
    /// `rust_xlsxwriter` build whose pool holds one entry, which is the right input for the grid
    /// kinds and the wrong one for anything that measures the pool.
    fn real_fixture_bytes() -> Vec<u8> {
        std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗿️artifacts/📕️xlsx/🧫️fixtures/📕️reuse-marketplaces.xlsx")).expect("the committed reuse-marketplaces workbook")
    }

    fn fixture_bytes() -> Vec<u8> {
        write_workbook_grid(&[("Sheet1".to_string(), vec![(0, 0, GridValue::Text("hello".to_string())), (0, 1, GridValue::Number(1.0)), (1, 0, GridValue::Bool(true))])]).unwrap()
    }

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        let input = fixture_bytes();
        let output = oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn insert_and_remove_sheet_are_real_transformations() {
        let input = fixture_bytes();
        let inserted = oracle_apply_mutation(&input, &spec("insert-sheet", sheet("New", vec![cell(1.0, 0.0, Json::String("fresh".to_string()))]))).unwrap();
        let grid = read_workbook_grid(&inserted).unwrap();
        assert_eq!(grid.len(), 2);
        assert_eq!(grid[1].0, "New");

        let removed = oracle_apply_mutation(&inserted, &spec("remove-sheet", Json::Object(vec![("name".to_string(), Json::String("New".to_string()))]))).unwrap();
        assert_eq!(read_workbook_grid(&removed).unwrap().len(), 1);
    }

    #[test]
    fn rename_sheet_changes_only_the_name() {
        let input = fixture_bytes();
        let renamed = oracle_apply_mutation(&input, &spec("rename-sheet", Json::Object(vec![("name".to_string(), Json::String("Sheet1".to_string())), ("newName".to_string(), Json::String("Renamed".to_string()))]))).unwrap();
        let grid = read_workbook_grid(&renamed).unwrap();
        assert_eq!(grid[0].0, "Renamed");
        assert_eq!(grid[0].1.len(), 3);
    }

    #[test]
    fn set_and_remove_cell_are_real_transformations() {
        let input = fixture_bytes();
        let set = oracle_apply_mutation(
            &input,
            &spec(
                "set-cell",
                Json::Object(vec![("sheetName".to_string(), Json::String("Sheet1".to_string())), ("row".to_string(), Json::Number(1.0)), ("col".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String("changed".to_string()))]),
            ),
        )
        .unwrap();
        let grid = read_workbook_grid(&set).unwrap();
        assert!(grid[0].1.contains(&(0, 0, GridValue::Text("changed".to_string()))));

        let removed = oracle_apply_mutation(&set, &spec("remove-cell", Json::Object(vec![("sheetName".to_string(), Json::String("Sheet1".to_string())), ("row".to_string(), Json::Number(1.0)), ("col".to_string(), Json::Number(0.0))]))).unwrap();
        assert!(!read_workbook_grid(&removed).unwrap()[0].1.iter().any(|(r, c, _)| *r == 0 && *c == 0));
    }

    #[test]
    /// 📑️ The three pool kinds really move the real 229-entry `xl/sharedStrings.xml`, and the pool
    /// is really read back out of the bytes. This replaces a test that asserted the opposite — that
    /// they were a byte identity — which was true only of the `calamine`/`rust_xlsxwriter` pairing
    /// and never of the package.
    fn shared_string_kinds_move_the_real_pool() {
        let input = real_fixture_bytes();
        let pool = shared_strings::read_pool(&input).expect("the real fixture carries a shared-string pool");
        assert_eq!(pool.len(), 229, "the committed fixture's own uniqueCount");

        let inserted = oracle_apply_mutation(&input, &spec("insert-shared-string", Json::Object(vec![("value".to_string(), Json::String("Ökobau Referenzquelle 2024".to_string()))]))).unwrap();
        let grown = shared_strings::read_pool(&inserted).unwrap();
        assert_eq!(grown.len(), 230);
        assert_eq!(grown.last().map(String::as_str), Some("Ökobau Referenzquelle 2024"));

        let removed = oracle_apply_mutation(&inserted, &spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(229.0))]))).unwrap();
        assert_eq!(shared_strings::read_pool(&removed).unwrap(), pool, "removing the appended entry restores the pool exactly");

        let set = oracle_apply_mutation(&input, &spec("set-shared-string", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String("Aktualisierter Quellwert".to_string()))]))).unwrap();
        assert_eq!(shared_strings::read_pool(&set).unwrap()[0], "Aktualisierter Quellwert");
    }

    /// ⚠️ An INTERIOR removal has no inverse in this vocabulary — `InsertSharedString` appends — and
    /// the oracle refuses to invent one rather than returning an undo that does not undo.
    #[test]
    fn an_interior_shared_string_removal_is_refused_an_inverse() {
        let input = real_fixture_bytes();
        let forward = spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(0.0))]));
        let error = shared_string_inverse_spec(&input, &forward).expect_err("index 0 of 229 is interior");
        assert!(error.contains("no inverse in this vocabulary"), "{error}");
        let last = spec("remove-shared-string", Json::Object(vec![("index".to_string(), Json::Number(228.0))]));
        assert_eq!(shared_string_inverse_spec(&input, &last).unwrap().str("kind"), "insert-shared-string");
    }

    /// 🕳️ A parameter set that would leave the pool exactly as it was is an error, not a pass.
    #[test]
    fn a_shared_string_write_that_changes_nothing_is_refused() {
        let input = real_fixture_bytes();
        let pool = shared_strings::read_pool(&input).unwrap();
        let unchanged = spec("set-shared-string", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("value".to_string(), Json::String(pool[0].clone()))]));
        assert!(oracle_apply_mutation(&input, &unchanged).is_err());
    }

    #[test]
    fn project_xlsx_workbook_carries_the_caller_tracked_shared_string_count() {
        let bytes = fixture_bytes();
        let projection = project_xlsx_workbook(&bytes, 3).unwrap();
        assert_eq!(projection.str("format"), "xlsx");
        assert_eq!(projection.get("sharedStringCount"), Some(&Json::Number(3.0)));
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = fixture_bytes();
        let result = oracle_apply_mutation(&input, &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
