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
//! `RemoveCell`) round-trip through "read the whole workbook into a
//! grid, apply the change to the grid, rebuild the whole workbook from the grid" — a genuine second
//! producer, hence `@mode-differential`. `InsertSharedString`/`RemoveSharedString`/`SetSharedString`
//! address the shared-string pool by an INDEX that is independent of any cell reference — exactly the
//! axis neither reference crate exposes — so this module cannot independently perform them; it
//! reports that honestly (see `oracle_apply_mutation`'s dispatch below) rather than faking a
//! differential result, per the fleet brief's §6.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

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
/// 🔀️ Every spec's `params` is the leaf's own wire payload (`XlsxMutation::payload_value()`). A cell's `row` is 1-based
/// (the subset's `XlsxCell::row`, ECMA-376's own `<row r="N">` index) and its `col` 0-based, converted to this module's
/// 0-based grid at the boundary below.
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
/// 🔢️ A wire `XlsxCellValue` (`{kind, value}`) as a grid value — the three kinds a `calamine` read and a `rust_xlsxwriter`
/// write both reproduce. A pool reference, an error or a formula is outside what this pairing can write back, and is
/// refused rather than approximated.
#[cfg(feature = "oracles")]
fn cell_value(value: &Json) -> Result<GridValue, String> {
    match (value.str("kind").as_str(), value.get("value")) {
        ("number", Some(Json::Number(n))) => Ok(GridValue::Number(*n)),
        ("boolean", Some(Json::Bool(b))) => Ok(GridValue::Bool(*b)),
        ("inlineString", Some(Json::String(s))) => Ok(GridValue::Text(s.clone())),
        _ => Err(format!("cell value {} is outside what the calamine + rust_xlsxwriter pairing reproduces", value.to_string())),
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
/// 📄️ One wire `XlsxSheet` (`{name, cells: [{row, col, value}]}`) as a grid sheet.
#[cfg(feature = "oracles")]
fn sheet_of(sheet: &Json) -> Result<GridSheet, String> {
    let cells = sheet
        .array("cells")
        .iter()
        .map(|cell| {
            let row = row0(number(cell, "row").ok_or("sheet cell missing `row`")?)?;
            let col = number(cell, "col").ok_or("sheet cell missing `col`")? as u32;
            Ok((row, col, cell_value(cell.get("value").ok_or("sheet cell missing `value`")?)?))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((string(sheet, "name"), cells))
}
//#endregion 🔖️SpecReaders

//#region 🔖️CellAddress
/// 🧭️ A wire `XlsxCellAddress` resolved against the package it was taken on, independently of the subject's
/// `resolve_xlsx_cell_address`: `partPath` names the worksheet part, `nodePath` the child indices from its root element to
/// the addressed `c` element (every element, text run, CDATA section, comment and processing instruction is one child),
/// and that element's own `r` reference names the cell; the part's sheet name is read out of the main part and its
/// relationships. The lineage `revision` is the subject's own staleness guard and plays no part in a reference edit.
#[cfg(feature = "oracles")]
mod cell_address {
    use semio_s_plugin_stdio_document_test_oracle::ooxml::{main_part, part_bytes, read_parts, relationships_part_path};
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::Json;

    /// 🌳️ One child slot of an element: an element with its attributes and children, or any other node.
    enum Node {
        Element { name: String, attrs: Vec<(String, String)>, children: Vec<Node> },
        Other,
    }

    fn attrs_of(start: &BytesStart) -> Result<Vec<(String, String)>, String> {
        start
            .attributes()
            .map(|attribute| {
                let attribute = attribute.map_err(|error| error.to_string())?;
                let value = attribute.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
                Ok((attribute.key.as_ref().to_string(), value.to_string()))
            })
            .collect()
    }

    fn element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<Node, String> {
        let (name, attrs) = (start.name().as_ref().to_string(), attrs_of(&start)?);
        let mut children = Vec::new();
        let mut in_text = false;
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::End(_) => return Ok(Node::Element { name, attrs, children }),
                Event::Start(child) => {
                    in_text = false;
                    children.push(element(reader, child)?);
                }
                Event::Empty(child) => {
                    in_text = false;
                    children.push(Node::Element { name: child.name().as_ref().to_string(), attrs: attrs_of(&child)?, children: Vec::new() });
                }
                Event::Text(_) | Event::GeneralRef(_) => {
                    if !in_text {
                        children.push(Node::Other);
                    }
                    in_text = true;
                }
                Event::CData(_) | Event::Comment(_) | Event::PI(_) => {
                    in_text = false;
                    children.push(Node::Other);
                }
                Event::Eof => return Err(format!("unclosed element <{name}>")),
                Event::Decl(_) | Event::DocType(_) => return Err(format!("declaration inside element <{name}>")),
            }
        }
    }

    fn root(bytes: &[u8]) -> Result<Node, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        let mut reader = Reader::from_str(text);
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::Start(start) => return element(&mut reader, start),
                Event::Empty(start) => return Ok(Node::Element { name: start.name().as_ref().to_string(), attrs: attrs_of(&start)?, children: Vec::new() }),
                Event::Eof => return Err("part has no root element".to_string()),
                _ => {}
            }
        }
    }

    fn local(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }

    fn attribute<'a>(attrs: &'a [(String, String)], key: impl Fn(&str) -> bool) -> Option<&'a str> {
        attrs.iter().find(|(name, _)| key(name)).map(|(_, value)| value.as_str())
    }

    fn elements<'a>(node: &'a Node, name: &'a str) -> Vec<&'a Node> {
        let mut found = Vec::new();
        if let Node::Element { name: own, children, .. } = node {
            if local(own) == name {
                found.push(node);
            }
            for child in children {
                found.extend(elements(child, name));
            }
        }
        found
    }

    /// 🔗️ `target` of a relationship owned by `source`, as a package part path.
    fn resolve(source: &str, target: &str) -> String {
        if let Some(absolute) = target.strip_prefix('/') {
            return absolute.to_string();
        }
        let mut segments: Vec<&str> = source.rsplit_once('/').map(|(directory, _)| directory.split('/').collect()).unwrap_or_default();
        for segment in target.split('/') {
            match segment {
                "" | "." => {}
                ".." => {
                    segments.pop();
                }
                other => segments.push(other),
            }
        }
        segments.join("/")
    }

    /// 🏷️ The sheet name whose worksheet part is `part_path`, through the main part's `sheets` and its relationships.
    fn sheet_name(parts: &[(String, Vec<u8>)], part_path: &str) -> Result<String, String> {
        let workbook = main_part(parts)?;
        let rels = root(part_bytes(parts, &relationships_part_path(&workbook)).ok_or_else(|| format!("{workbook} has no relationships part"))?)?;
        let id = elements(&rels, "Relationship")
            .into_iter()
            .find_map(|relationship| match relationship {
                Node::Element { attrs, .. } if attribute(attrs, |key| key == "Target").is_some_and(|target| resolve(&workbook, target) == part_path) => attribute(attrs, |key| key == "Id").map(str::to_string),
                _ => None,
            })
            .ok_or_else(|| format!("{workbook} declares no relationship to {part_path}"))?;
        let document = root(part_bytes(parts, &workbook).ok_or_else(|| format!("main part {workbook} is absent"))?)?;
        elements(&document, "sheet")
            .into_iter()
            .find_map(|sheet| match sheet {
                Node::Element { attrs, .. } if attribute(attrs, |key| key.ends_with(":id")) == Some(id.as_str()) => attribute(attrs, |key| key == "name").map(str::to_string),
                _ => None,
            })
            .ok_or_else(|| format!("{workbook} names no sheet for relationship {id}"))
    }

    /// 🧭️ `(sheet name, 0-based row, 0-based column)` of the cell `address` names in `input`.
    pub fn addressed_cell(input: &[u8], address: &Json) -> Result<(String, u32, u32), String> {
        let parts = read_parts(input)?;
        let part_path = address.str("partPath");
        let mut node = &root(part_bytes(&parts, &part_path).ok_or_else(|| format!("the addressed part {part_path} is not in the package"))?)?;
        for step in address.array("nodePath") {
            let (Json::Number(index), Node::Element { children, .. }) = (&step, node) else { return Err(format!("node path {} descends through a non-element", address.to_string())) };
            node = children.get(*index as usize).ok_or_else(|| format!("node path child {index} is outside the element"))?;
        }
        let Node::Element { name, attrs, .. } = node else { return Err("the node path ends on a non-element".to_string()) };
        if local(name) != "c" || local(name) != address.str("localName") {
            return Err(format!("the node path ends on <{name}>, not the addressed cell element"));
        }
        let reference = attribute(attrs, |key| key == "r").ok_or("the addressed cell carries no `r` reference")?;
        let split = reference.find(|character: char| character.is_ascii_digit()).ok_or_else(|| format!("cell reference {reference:?} has no row"))?;
        let column = reference[..split].bytes().try_fold(0u32, |acc, letter| if letter.is_ascii_uppercase() { Ok(acc * 26 + u32::from(letter - b'A' + 1)) } else { Err(format!("cell reference {reference:?} has a malformed column")) })?;
        let row: u32 = reference[split..].parse().map_err(|error| format!("cell reference {reference:?}: {error}"))?;
        if row == 0 || column == 0 {
            return Err(format!("cell reference {reference:?} is out of range"));
        }
        Ok((sheet_name(&parts, &part_path)?, row - 1, column - 1))
    }
}
//#endregion 🔖️CellAddress

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
///
/// `insert-shared-string`/`remove-shared-string`/`set-shared-string` do NOT go through
/// `calamine`/`rust_xlsxwriter`: the raw pool those three address is invisible to the first's read
/// model and unreachable by index through the second's write API. They go through the `zip` +
/// `quick-xml` pairing instead (see the [`shared_strings`] module), which reads and rewrites
/// `xl/sharedStrings.xml` as the OPC PART it is.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "insert-sheet" => {
            let mut sheets = read_workbook_grid(input)?;
            sheets.push(sheet_of(params.get("sheet").ok_or("insert-sheet: missing `sheet`")?)?);
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
            let sheet = sheets.iter_mut().find(|(sheet_name, _)| sheet_name == &name).ok_or_else(|| format!("rename-sheet: no sheet named {name:?}"))?;
            sheet.0 = string(&params, "newName");
            write_workbook_grid(&sheets)
        }
        "set-cell" => {
            let (sheet_name, row, col) = cell_address::addressed_cell(input, params.get("address").ok_or("set-cell: missing `address`")?)?;
            let value = cell_value(params.get("value").ok_or("set-cell: missing `value`")?)?;
            let mut sheets = read_workbook_grid(input)?;
            put_cell(&mut sheets, &sheet_name, row, col, Some(value))?;
            write_workbook_grid(&sheets)
        }
        "remove-cell" => {
            let (sheet_name, row, col) = cell_address::addressed_cell(input, params.get("address").ok_or("remove-cell: missing `address`")?)?;
            let mut sheets = read_workbook_grid(input)?;
            put_cell(&mut sheets, &sheet_name, row, col, None)?;
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
            if shared_strings::is_referenced(input, index)? {
                return Err(format!("remove-shared-string: pool entry {index} is still referenced by a cell"));
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
        kind @ ("set-relationship" | "remove-relationship" | "set-content-type" | "remove-content-type") => plumbing::apply(input, kind, &params),
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🎬️ The real pre-state a kind runs on. `remove-shared-string` needs a pool entry no cell references, and every entry of
/// the real workbook's pool is referenced, so for that kind this is the real package after the reference has appended one
/// unreferenced entry — the removal under test is still the reference's own, on genuine OPC. Every other kind reads the
/// committed bytes untouched.
#[cfg(feature = "oracles")]
pub fn oracle_arrange(input: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
    match forward.str("kind").as_str() {
        "remove-shared-string" => {
            let mut pool = shared_strings::read_pool(input)?;
            pool.push(UNREFERENCED_ENTRY.to_string());
            shared_strings::write_pool(input, &pool)
        }
        _ => Ok(input.to_vec()),
    }
}

/// 🧾️ The pool entry [`oracle_arrange`] appends: text no cell of the real workbook carries.
#[cfg(feature = "oracles")]
const UNREFERENCED_ENTRY: &str = "Nicht referenzierter Eintrag";

#[cfg(not(feature = "oracles"))]
pub fn oracle_arrange(_input: &[u8], _forward: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ✍️ Writes (`Some`) or clears (`None`) one grid cell of the named sheet.
#[cfg(feature = "oracles")]
fn put_cell(sheets: &mut [GridSheet], sheet_name: &str, row: u32, col: u32, value: Option<GridValue>) -> Result<(), String> {
    let (_, cells) = sheets.iter_mut().find(|(name, _)| name == sheet_name).ok_or_else(|| format!("no sheet named {sheet_name:?}"))?;
    cells.retain(|(r, c, _)| !(*r == row && *c == col));
    if let Some(value) = value {
        cells.push((row, col, value));
    }
    Ok(())
}

#[cfg(feature = "oracles")]
fn applied<T>(mut value: T, step: impl FnOnce(&mut T) -> Result<(), String>) -> Result<T, String> {
    step(&mut value)?;
    Ok(value)
}

/// ↩️ Undoes `forward` on `mutated`, sourcing whatever it discarded from `original` (the package the forward kind ran on) —
/// the algebra `XlsxMutation::inverse` defines, computed independently by the reference pairing. A cell address is
/// lineage-bound to `original`, so it is resolved there and the undo applied to the rebuilt grid by coordinate; the three pool kinds go through
/// [`shared_string_inverse_spec`].
#[cfg(feature = "oracles")]
pub fn oracle_apply_inverse(original: &[u8], mutated: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(forward);
    let kind = forward.str("kind");
    let cell_undo = |field: &str| -> Result<Vec<u8>, String> {
        let (sheet_name, row, col) = cell_address::addressed_cell(original, params.get(field).ok_or_else(|| format!("{kind}: missing `{field}`"))?)?;
        let before = read_workbook_grid(original)?.into_iter().find(|(name, _)| name == &sheet_name).and_then(|(_, cells)| cells.into_iter().find(|(r, c, _)| *r == row && *c == col)).map(|(_, _, value)| value);
        let sheets = applied(read_workbook_grid(mutated)?, |sheets| put_cell(sheets, &sheet_name, row, col, before))?;
        write_workbook_grid(&sheets)
    };
    match kind.as_str() {
        "insert-sheet" => {
            let name = params.get("sheet").map(|sheet| string(sheet, "name")).unwrap_or_default();
            let mut sheets = read_workbook_grid(mutated)?;
            sheets.retain(|(sheet_name, _)| sheet_name != &name);
            write_workbook_grid(&sheets)
        }
        "remove-sheet" => {
            let name = string(&params, "name");
            let originals = read_workbook_grid(original)?;
            let index = originals.iter().position(|(sheet_name, _)| sheet_name == &name).ok_or_else(|| format!("remove-sheet inverse: no sheet named {name:?} in the original"))?;
            let mut sheets = read_workbook_grid(mutated)?;
            sheets.insert(index.min(sheets.len()), originals[index].clone());
            write_workbook_grid(&sheets)
        }
        "rename-sheet" => {
            let new_name = string(&params, "newName");
            let mut sheets = read_workbook_grid(mutated)?;
            let sheet = sheets.iter_mut().find(|(sheet_name, _)| sheet_name == &new_name).ok_or_else(|| format!("rename-sheet inverse: no sheet named {new_name:?}"))?;
            sheet.0 = string(&params, "name");
            write_workbook_grid(&sheets)
        }
        "set-cell" | "remove-cell" => cell_undo("address"),
        "insert-shared-string" | "remove-shared-string" | "set-shared-string" => oracle_apply_mutation(mutated, &shared_string_inverse_spec(original, forward)?),
        "set-relationship" | "remove-relationship" | "set-content-type" | "remove-content-type" => plumbing::undo(original, mutated, &kind, &params),
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_inverse(_original: &[u8], _mutated: &[u8], _forward: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `calamine` + `rust_xlsxwriter`
/// pairing every grid mutation above uses — proves the reference pairing itself is stable on the real
/// fixture before the subject's own codec is asked to be. Genuinely rebuilds the package (never a
/// literal byte passthrough): `rust_xlsxwriter` cannot reproduce another writer's object layout.
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
/// (`semio_s_plugin_stdio_document_test_oracle::ooxml`), NOT through `calamine`/`rust_xlsxwriter`.
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
    use semio_s_plugin_stdio_document_test_oracle::ooxml::{part_bytes, read_parts, set_part, write_parts};
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use quick_xml::XmlVersion;

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

    /// 🔗️ Whether any worksheet cell of the package references pool entry `index` (`<c t="s"><v>index</v></c>`, ECMA-376
    /// §18.3.1.4). Removing such an entry would leave the cell's reference dangling, which this vocabulary refuses.
    pub fn is_referenced(input: &[u8], index: usize) -> Result<bool, String> {
        let parts = read_parts(input)?;
        for (path, bytes) in parts.iter().filter(|(path, _)| path.starts_with("xl/worksheets/") && path.ends_with(".xml")) {
            let text = std::str::from_utf8(bytes).map_err(|error| format!("{path} is not valid utf-8: {error}"))?;
            let mut reader = Reader::from_str(text);
            let (mut shared_cell, mut in_value, mut value) = (false, false, String::new());
            loop {
                match reader.read_event().map_err(|error| format!("independent XML reader could not read {path}: {error}"))? {
                    Event::Start(start) => match local_name(start.name().as_ref()) {
                        "c" => shared_cell = start.attributes().flatten().any(|attribute| attribute.key.as_ref() == "t" && attribute.normalized_value(XmlVersion::Explicit1_0).is_ok_and(|kind| kind == "s")),
                        "v" if shared_cell => {
                            in_value = true;
                            value.clear();
                        }
                        _ => {}
                    },
                    Event::Text(run) if in_value => value.push_str(&run.xml10_content()),
                    Event::End(end) => match local_name(end.name().as_ref()) {
                        "v" if in_value => {
                            in_value = false;
                            if value.trim().parse::<usize>().ok() == Some(index) {
                                return Ok(true);
                            }
                        }
                        "c" => shared_cell = false,
                        _ => {}
                    },
                    Event::Eof => break,
                    _ => {}
                }
            }
        }
        Ok(false)
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
/// (`../🧬️schema/🧬️mutations/🦀️.rs:145`), so nothing in the catalog can put a string back at
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

//#region 🔖️Plumbing
/// 🪢️ The four OPC-plumbing kinds (`set-relationship`, `remove-relationship`, `set-content-type`, `remove-content-type`), performed on `[Content_Types].xml` and the `*.rels`
/// parts through the `zip` + `quick-xml` pairing the six OOXML conformance subsets already run on -- the rows of these parts are flat, so each is read into its
/// ordered rows, edited, and written back; every other part is carried verbatim. `calamine`/`rust_xlsxwriter` never see the plumbing.
#[cfg(feature = "oracles")]
mod plumbing {
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::Json;
    use semio_s_plugin_stdio_document_test_oracle::ooxml::{read_parts, relationships_part_path, write_parts};

    const CONTENT_TYPES_PART: &str = "[Content_Types].xml";
    const RELATIONSHIPS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

    type Attrs = Vec<(String, String)>;

    /// 🧱️ A plumbing part: its root element and the ordered flat rows below it.
    struct Flat {
        root: String,
        root_attrs: Attrs,
        rows: Vec<(String, Attrs)>,
    }

    impl Flat {
        fn attr<'a>(row: &'a (String, Attrs), key: &str) -> Option<&'a str> {
            row.1.iter().find(|(name, _)| name == key).map(|(_, value)| value.as_str())
        }

        /// 📍️ The row index of the `tag` row whose `key` attribute is `value`.
        fn position(&self, tag: &str, key: &str, value: &str) -> Option<usize> {
            self.rows.iter().position(|row| row.0 == tag && Self::attr(row, key) == Some(value))
        }

        /// 📍️ The physical position a new `tag` row takes: before the `index`-th existing row of its tag, else after the last of its tag -- a content-types default
        /// after the last default, an override and a relationship at the end.
        fn slot(&self, tag: &str, index: Option<usize>) -> usize {
            let slots: Vec<usize> = self.rows.iter().enumerate().filter(|(_, row)| row.0 == tag).map(|(at, _)| at).collect();
            match index.and_then(|at| slots.get(at).copied()) {
                Some(physical) => physical,
                None => match slots.last() {
                    Some(last) => last + 1,
                    None if tag == "Default" => 0,
                    None => self.rows.len(),
                },
            }
        }
    }

    fn attrs_of(start: &BytesStart) -> Result<Attrs, String> {
        start
            .attributes()
            .map(|attribute| {
                let attribute = attribute.map_err(|error| error.to_string())?;
                let value = attribute.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
                Ok((attribute.key.as_ref().to_string(), value.to_string()))
            })
            .collect()
    }

    fn parse(bytes: &[u8]) -> Result<Flat, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        let mut reader = Reader::from_str(text);
        let mut flat: Option<Flat> = None;
        let mut depth = 0usize;
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::Start(start) => {
                    match flat.as_mut() {
                        None => flat = Some(Flat { root: start.name().as_ref().to_string(), root_attrs: attrs_of(&start)?, rows: Vec::new() }),
                        Some(flat) if depth == 1 => flat.rows.push((start.name().as_ref().to_string(), attrs_of(&start)?)),
                        Some(_) => {}
                    }
                    depth += 1;
                }
                Event::Empty(start) => match flat.as_mut() {
                    None => flat = Some(Flat { root: start.name().as_ref().to_string(), root_attrs: attrs_of(&start)?, rows: Vec::new() }),
                    Some(flat) if depth == 1 => flat.rows.push((start.name().as_ref().to_string(), attrs_of(&start)?)),
                    Some(_) => {}
                },
                Event::End(_) => depth = depth.saturating_sub(1),
                Event::Eof => break,
                _ => {}
            }
        }
        flat.ok_or_else(|| "part has no root element".to_string())
    }

    fn escape(text: &str) -> String {
        text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
    }

    fn render(flat: &Flat) -> Vec<u8> {
        let attrs = |attrs: &Attrs| attrs.iter().map(|(key, value)| format!(" {key}=\"{}\"", escape(value))).collect::<String>();
        let mut out = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><{}{}>", flat.root, attrs(&flat.root_attrs));
        for row in &flat.rows {
            out.push_str(&format!("<{}{}/>", row.0, attrs(&row.1)));
        }
        out.push_str(&format!("</{}>", flat.root));
        out.into_bytes()
    }

    fn optional_index(params: &Json) -> Option<usize> {
        match params.get("index") {
            Some(Json::Number(number)) if *number >= 0.0 => Some(*number as usize),
            _ => None,
        }
    }

    fn flag(params: &Json, key: &str) -> bool {
        matches!(params.get(key), Some(Json::Bool(true)))
    }

    /// 🏷️ The row tag, its key attribute and the normalized key of a content-types entry.
    fn entry(params: &Json) -> (bool, &'static str, &'static str, String) {
        let is_override = flag(params, "isOverride");
        let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
        let name = params.str("name");
        (is_override, tag, key, name)
    }

    fn part_of(parts: &[(String, Vec<u8>)], path: &str) -> Option<Result<Flat, String>> {
        parts.iter().find(|(name, _)| name == path).map(|(_, bytes)| parse(bytes))
    }

    fn put(parts: &mut Vec<(String, Vec<u8>)>, path: &str, flat: &Flat) {
        let bytes = render(flat);
        match parts.iter_mut().find(|(name, _)| name == path) {
            Some(existing) => existing.1 = bytes,
            None => parts.push((path.to_string(), bytes)),
        }
    }

    pub fn apply(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut parts = read_parts(input)?;
        match kind {
            "set-relationship" => {
                let path = relationships_part_path(&params.str("owner"));
                let id = params.str("id");
                let mut attrs = vec![("Id".to_string(), id.clone()), ("Type".to_string(), params.str("relType")), ("Target".to_string(), params.str("target"))];
                if flag(params, "external") {
                    attrs.push(("TargetMode".to_string(), "External".to_string()));
                }
                let row = ("Relationship".to_string(), attrs);
                let mut flat = match part_of(&parts, &path) {
                    Some(flat) => flat?,
                    None => Flat { root: "Relationships".to_string(), root_attrs: vec![("xmlns".to_string(), RELATIONSHIPS_NS.to_string())], rows: Vec::new() },
                };
                match flat.position("Relationship", "Id", &id) {
                    Some(at) => flat.rows[at] = row,
                    None => {
                        let physical = flat.slot("Relationship", optional_index(params));
                        flat.rows.insert(physical, row);
                    }
                }
                put(&mut parts, &path, &flat);
            }
            "remove-relationship" => {
                let owner = params.str("owner");
                let path = relationships_part_path(&owner);
                let id = params.str("id");
                let mut flat = part_of(&parts, &path).ok_or_else(|| format!("remove-relationship: no relationships are owned by {owner:?}"))??;
                let at = flat.position("Relationship", "Id", &id).ok_or_else(|| format!("remove-relationship: {owner:?} owns no relationship {id:?}"))?;
                flat.rows.remove(at);
                if flat.rows.iter().any(|row| row.0 == "Relationship") {
                    put(&mut parts, &path, &flat);
                } else {
                    parts.retain(|(name, _)| *name != path);
                }
            }
            "set-content-type" => {
                let (is_override, tag, key, name) = entry(params);
                let mut flat = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
                let content_type = params.str("contentType");
                if !is_override && flat.rows.iter().any(|row| row.0 == "Default" && Flat::attr(row, key).is_some_and(|existing| existing != name && existing.eq_ignore_ascii_case(&name))) {
                    return Err(format!("set-content-type: default extension {name:?} already exists in another letter case"));
                }
                match flat.position(tag, key, &name) {
                    Some(at) => {
                        if let Some(existing) = flat.rows[at].1.iter_mut().find(|(attr, _)| attr == "ContentType") {
                            existing.1 = content_type;
                        }
                    }
                    None => {
                        let physical = flat.slot(tag, optional_index(params));
                        flat.rows.insert(physical, (tag.to_string(), vec![(key.to_string(), name), ("ContentType".to_string(), content_type)]));
                    }
                }
                put(&mut parts, CONTENT_TYPES_PART, &flat);
            }
            "remove-content-type" => {
                let (_, tag, key, name) = entry(params);
                let mut flat = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
                let at = flat.position(tag, key, &name).ok_or_else(|| format!("remove-content-type: the content types hold no {tag} {name:?}"))?;
                flat.rows.remove(at);
                put(&mut parts, CONTENT_TYPES_PART, &flat);
            }
            other => return Err(format!("{other:?} is no plumbing kind")),
        }
        write_parts(&parts)
    }

    /// ↩️ Undoes a plumbing `kind` on `mutated`, sourcing the previous row and its position from `original`: the previous row restored at its position, or the row the forward
    /// wrote removed.
    pub fn undo(original: &[u8], mutated: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let parts = read_parts(original)?;
        let object = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        let (undo_kind, undo_params) = match kind {
            "set-relationship" | "remove-relationship" => {
                let owner = params.str("owner");
                let id = params.str("id");
                let previous = match part_of(&parts, &relationships_part_path(&owner)) {
                    Some(flat) => {
                        let flat = flat?;
                        flat.position("Relationship", "Id", &id).map(|at| (flat.rows.iter().take(at).filter(|row| row.0 == "Relationship").count(), flat.rows[at].clone()))
                    }
                    None => None,
                };
                match previous {
                    Some((at, row)) => (
                        "set-relationship",
                        object(vec![
                            ("owner", Json::String(owner)),
                            ("id", Json::String(id)),
                            ("relType", Json::String(Flat::attr(&row, "Type").unwrap_or_default().to_string())),
                            ("target", Json::String(Flat::attr(&row, "Target").unwrap_or_default().to_string())),
                            ("external", Json::Bool(Flat::attr(&row, "TargetMode") == Some("External"))),
                            ("index", Json::Number(at as f64)),
                        ]),
                    ),
                    None if kind == "set-relationship" => ("remove-relationship", object(vec![("owner", Json::String(owner)), ("id", Json::String(id))])),
                    None => return Ok(mutated.to_vec()),
                }
            }
            _ => {
                let (is_override, tag, key, name) = entry(params);
                let flat = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
                match flat.position(tag, key, &name) {
                    Some(at) => (
                        "set-content-type",
                        object(vec![
                            ("isOverride", Json::Bool(is_override)),
                            ("name", Json::String(name)),
                            ("contentType", Json::String(Flat::attr(&flat.rows[at], "ContentType").unwrap_or_default().to_string())),
                            ("index", Json::Number(flat.rows.iter().take(at).filter(|row| row.0 == tag).count() as f64)),
                        ]),
                    ),
                    None if kind == "set-content-type" => ("remove-content-type", object(vec![("isOverride", Json::Bool(is_override)), ("name", Json::String(name))])),
                    None => return Ok(mutated.to_vec()),
                }
            }
        };
        apply(mutated, undo_kind, &undo_params)
    }

    /// 🪢️ The owner part a `*.rels` part belongs to (`""` for the package root).
    fn owner_of(path: &str) -> Option<String> {
        let file = path.rsplit('/').next()?;
        let stem = file.strip_suffix(".rels")?;
        let directory = path[..path.len() - file.len()].strip_suffix("_rels/")?;
        Some(format!("{directory}{stem}"))
    }

    /// 🪢️ The package plumbing in the order the package states it: the `[Content_Types].xml` defaults and overrides as ordered pairs, and every owner's relationships as ordered
    /// `{id, type, target, external}` rows (owners sorted by part name).
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let parts = read_parts(bytes)?;
        let types = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
        let pairs = |tag: &str, key: &str| Json::Array(types.rows.iter().filter(|row| row.0 == tag).map(|row| Json::Array(vec![Json::String(Flat::attr(row, key).unwrap_or_default().to_string()), Json::String(Flat::attr(row, "ContentType").unwrap_or_default().to_string())])).collect());
        let mut owners: Vec<(String, String)> = parts.iter().filter_map(|(path, _)| owner_of(path).map(|owner| (owner, path.clone()))).collect();
        owners.sort();
        let mut relationships = Vec::with_capacity(owners.len());
        for (owner, path) in owners {
            let flat = part_of(&parts, &path).ok_or("a listed relationships part vanished")??;
            let rows = flat
                .rows
                .iter()
                .filter(|row| row.0 == "Relationship")
                .map(|row| {
                    Json::Object(vec![
                        ("id".to_string(), Json::String(Flat::attr(row, "Id").unwrap_or_default().to_string())),
                        ("type".to_string(), Json::String(Flat::attr(row, "Type").unwrap_or_default().to_string())),
                        ("target".to_string(), Json::String(Flat::attr(row, "Target").unwrap_or_default().to_string())),
                        ("external".to_string(), Json::Bool(Flat::attr(row, "TargetMode") == Some("External"))),
                    ])
                })
                .collect();
            relationships.push((owner, Json::Array(rows)));
        }
        Ok(Json::Object(vec![("defaults".to_string(), pairs("Default", "Extension")), ("overrides".to_string(), pairs("Override", "PartName")), ("relationships".to_string(), Json::Object(relationships))]))
    }
}

/// 🪢️ The package plumbing of `bytes`, in the order the package states it. @see [`plumbing::project`].
#[cfg(feature = "oracles")]
pub fn project_xlsx_plumbing(bytes: &[u8]) -> Result<Json, String> {
    plumbing::project(bytes)
}

#[cfg(not(feature = "oracles"))]
pub fn project_xlsx_plumbing(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Plumbing

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
