//! 🔢️ Grid and cell addressing of the XLSX oracle: the workbook grid values, the spec readers and the A1 cell address arithmetic. See [`super`].

use super::*;

//#region 🔖️Grid
/// 🔢️ A cell value as the independent reader/writer pairing can observe and reproduce it — the
/// `Data::Int`/`Data::Float` split `calamine` draws collapses to one numeric case here since XLSX
/// itself stores every number as an IEEE-754 double (ECMA-376 §18.17.2, ST_Xstring numeric literal),
/// never a real reader/writer distinction.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum GridValue {
    Number(f64),
    Bool(bool),
    Text(String),
}

/// 📄 One sheet as `(name, [(row, col, value)])` — `row`/`col` are 0-based, `calamine`'s and
/// `rust_xlsxwriter`'s own native convention (the subset's OWN `XlsxCell::row` is 1-based; the JSON
/// spec this module reads carries the subset's 1-based convention, converted at the boundary below,
/// so one wire contract serves both this module and the subject's own mutation code).
pub(super) type GridSheet = (String, Vec<(u32, u32, GridValue)>);

/// 📥️ Independent read: every sheet, every non-empty cell, through `calamine`'s resolved
/// `Data` — this IS the reader's own semantic view; it cannot and does not distinguish a
/// shared-string reference from an inline string (see module doc comment).
pub(super) fn read_workbook_grid(input: &[u8]) -> Result<Vec<GridSheet>, String> {
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
pub(super) fn write_workbook_grid(sheets: &[GridSheet]) -> Result<Vec<u8>, String> {
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
pub(super) fn mutation_params(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}

pub(super) fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}

pub(super) fn string(value: &Json, key: &str) -> String {
    match value.get(key) {
        Some(Json::String(text)) => text.clone(),
        _ => String::new(),
    }
}

/// 🔢️ A wire `XlsxCellValue` (`{kind, value}`) as a grid value — the three kinds a `calamine` read and a `rust_xlsxwriter`
/// write both reproduce. A pool reference, an error or a formula is outside what this pairing can write back, and is
/// refused rather than approximated.
pub(super) fn cell_value(value: &Json) -> Result<GridValue, String> {
    match (value.str("kind").as_str(), value.get("value")) {
        ("number", Some(Json::Number(n))) => Ok(GridValue::Number(*n)),
        ("boolean", Some(Json::Bool(b))) => Ok(GridValue::Bool(*b)),
        ("inlineString", Some(Json::String(s))) => Ok(GridValue::Text(s.clone())),
        _ => Err(format!("cell value {} is outside what the calamine + rust_xlsxwriter pairing reproduces", value.to_string())),
    }
}

/// 🔁️ One-based (`XlsxCell::row` convention) -> zero-based (this module's/`calamine`'s convention).
fn row0(one_based_row: f64) -> Result<u32, String> {
    let row = one_based_row as i64;
    if row < 1 {
        return Err(format!("row must be >= 1 (1-based), got {row}"));
    }
    Ok((row - 1) as u32)
}

/// 📄️ One wire `XlsxSheet` (`{name, cells: [{row, col, value}]}`) as a grid sheet.
pub(super) fn sheet_of(sheet: &Json) -> Result<GridSheet, String> {
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
pub(super) mod cell_address {
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
