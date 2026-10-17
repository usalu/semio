//! 🧮️ The JSON parameter readers and writers and the run and block path addressing.

use super::*;

//#region 🔖️JsonValue
fn number_field(value: &Json, key: &str) -> f64 {
    match value.get(key) {
        Some(Json::Number(number)) => *number,
        _ => 0.0,
    }
}

fn usize_field(value: &Json, key: &str) -> usize {
    number_field(value, key).max(0.0) as usize
}

pub(super) fn bool_field(value: &Json, key: &str) -> bool {
    matches!(value.get(key), Some(Json::Bool(true)))
}

/// 🧮️ A wire byte array (`[0..=255, …]`).
pub(super) fn byte_array(value: Option<&Json>) -> Result<Vec<u8>, String> {
    match value {
        Some(Json::Array(items)) => items
            .iter()
            .map(|item| match item {
                Json::Number(byte) if (0.0..=255.0).contains(byte) && byte.fract() == 0.0 => Ok(*byte as u8),
                other => Err(format!("{other:?} is not a byte")),
            })
            .collect(),
        other => Err(format!("{other:?} is not a byte array")),
    }
}

pub(super) fn bytes_to_json(bytes: &[u8]) -> Json {
    Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
}

/// 📍️ The optional insertion `index` of a wire payload (`None` appends).
pub(super) fn optional_index(value: &Json) -> Option<usize> {
    match value.get("index") {
        Some(Json::Number(number)) if *number >= 0.0 => Some(*number as usize),
        _ => None,
    }
}

pub(super) fn non_empty(value: &Json, key: &str) -> Option<String> {
    match value.get(key) {
        Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
        _ => None,
    }
}

/// 🧭️ `{"segments":[{"blockIndex":..,"row":..,"cell":..}, ...], "index": N}` — the same
/// segments-plus-index shape `DocxBlockPath` itself carries (production field names, camelCased
/// for JSON), so a spec written for the oracle reads the same as one written for the subject.
#[derive(Clone, Debug)]
struct DPathSegment {
    block_index: usize,
    pub(super) row: usize,
    pub(super) cell: usize,
}

#[derive(Clone, Debug, Default)]
struct DPath {
    pub(super) segments: Vec<DPathSegment>,
    pub(super) index: usize,
}

pub(super) fn json_to_path(value: &Json) -> DPath {
    let segments = value.array("segments").iter().map(|s| DPathSegment { block_index: usize_field(s, "blockIndex"), row: usize_field(s, "row"), cell: usize_field(s, "cell") }).collect();
    DPath { segments, index: usize_field(value, "index") }
}

pub(super) fn path_to_json(path: &DPath) -> Json {
    Json::Object(vec![
        (
            "segments".to_string(),
            Json::Array(
                path.segments.iter().map(|s| Json::Object(vec![("blockIndex".to_string(), Json::Number(s.block_index as f64)), ("row".to_string(), Json::Number(s.row as f64)), ("cell".to_string(), Json::Number(s.cell as f64))])).collect(),
            ),
        ),
        ("index".to_string(), Json::Number(path.index as f64)),
    ])
}

fn json_to_run(value: &Json) -> WRun {
    WRun { text: value.str("text"), bold: bool_field(value, "bold"), italic: bool_field(value, "italic"), underline: bool_field(value, "underline") }
}

fn run_to_json(run: &WRun) -> Json {
    Json::Object(vec![("text".to_string(), Json::String(run.text.clone())), ("bold".to_string(), Json::Bool(run.bold)), ("italic".to_string(), Json::Bool(run.italic)), ("underline".to_string(), Json::Bool(run.underline))])
}

/// 🔎️ Owned block-spec JSON grammar mutation params speak: `{"kind":"paragraph","style":...,
/// "runs":[{"text":...,"bold":...,"italic":...,"underline":...}]}` |
/// `{"kind":"table","rows":[{"cells":[{"blocks":[...]}]}]}`.
pub(super) fn json_to_block(value: &Json) -> Result<WBlock, String> {
    match value.str("kind").as_str() {
        "paragraph" => Ok(WBlock::Paragraph(WParagraph { runs: value.array("runs").iter().map(json_to_run).collect(), style: non_empty(value, "style") })),
        "table" => Ok(WBlock::Table(WTable { rows: value.array("rows").iter().map(json_to_row).collect::<Result<_, _>>()? })),
        other => Err(format!("unknown block kind {other:?}")),
    }
}

fn json_to_row(value: &Json) -> Result<WRow, String> {
    Ok(WRow { cells: value.array("cells").iter().map(json_to_cell).collect::<Result<_, _>>()? })
}

fn json_to_cell(value: &Json) -> Result<WCell, String> {
    Ok(WCell { blocks: value.array("blocks").iter().map(json_to_block).collect::<Result<_, _>>()? })
}

pub(super) fn block_to_json(block: &WBlock) -> Json {
    match block {
        WBlock::Paragraph(p) => Json::Object(vec![
            ("kind".to_string(), Json::String("paragraph".to_string())),
            (
                "style".to_string(),
                match &p.style {
                    Some(s) => Json::String(s.clone()),
                    None => Json::Null,
                },
            ),
            ("runs".to_string(), Json::Array(p.runs.iter().map(run_to_json).collect())),
        ]),
        WBlock::Table(t) => Json::Object(vec![
            ("kind".to_string(), Json::String("table".to_string())),
            (
                "rows".to_string(),
                Json::Array(
                    t.rows
                        .iter()
                        .map(|row| Json::Object(vec![("cells".to_string(), Json::Array(row.cells.iter().map(|cell| Json::Object(vec![("blocks".to_string(), Json::Array(cell.blocks.iter().map(block_to_json).collect()))])).collect()))]))
                        .collect(),
                ),
            ),
        ]),
    }
}

pub(super) fn json_to_style(value: &Json) -> WStyle {
    WStyle { id: value.str("id"), name: value.str("name"), based_on: non_empty(value, "basedOn") }
}

pub(super) fn style_to_json(style: &WStyle) -> Json {
    Json::Object(vec![
        ("id".to_string(), Json::String(style.id.clone())),
        ("name".to_string(), Json::String(style.name.clone())),
        (
            "basedOn".to_string(),
            match &style.based_on {
                Some(v) => Json::String(v.clone()),
                None => Json::Null,
            },
        ),
    ])
}
//#endregion 🔖️JsonValue

//#region 🔖️PathAddressing
/// 🔎️ Resolves the block list `path`'s segments navigate to (the parent list `path.index` slots
/// into), immutable form. Mirrors `crate::…::diff::resolve_blocks`'s exact
/// `Table -> rows -> cells -> blocks` traversal, reimplemented independently.
pub(super) fn resolve_blocks<'a>(body: &'a [WBlock], segments: &[DPathSegment]) -> Option<&'a [WBlock]> {
    match segments.split_first() {
        None => Some(body),
        Some((seg, rest)) => {
            let WBlock::Table(table) = body.get(seg.block_index)? else { return None };
            let row = table.rows.get(seg.row)?;
            let cell = row.cells.get(seg.cell)?;
            resolve_blocks(&cell.blocks, rest)
        }
    }
}

pub(super) fn resolve_blocks_mut<'a>(body: &'a mut Vec<WBlock>, segments: &[DPathSegment]) -> Option<&'a mut Vec<WBlock>> {
    match segments.split_first() {
        None => Some(body),
        Some((seg, rest)) => {
            let WBlock::Table(table) = body.get_mut(seg.block_index)? else { return None };
            let row = table.rows.get_mut(seg.row)?;
            let cell = row.cells.get_mut(seg.cell)?;
            resolve_blocks_mut(&mut cell.blocks, rest)
        }
    }
}
/// 🧭️ The leaf wire's `DocxXmlAddress` (`{partPath, nodePath, expectedName, revision}`) of one run, resolved in this
/// oracle's OWN `quick-xml` tree of the main part: `nodePath` walks child indices from the root, and every step is
/// translated into this module's block model — `w:p`/`w:tbl` ordinal in a block list, `w:tr` and `w:tc` ordinals in
/// a table, `w:r` ordinal in a paragraph. Only runs this model carries (direct `w:r` children of a paragraph) are
/// addressable; the `revision` lineage guard is the subject's own and is not re-derived here.
pub(super) fn resolve_run_address(pkg: &WPackage, address: &Json) -> Result<(DPath, usize), String> {
    let part_path = address.str("partPath");
    if part_path != pkg.main_path {
        return Err(format!("run address names part {part_path:?}, not the main document part {:?}", pkg.main_path));
    }
    let main = pkg.opc.part(&pkg.main_path).ok_or("run address: the main document part is absent")?;
    let mut node = parse_xml(&main.bytes)?;
    let mut path = DPath::default();
    let mut table: Option<DPathSegment> = None;
    let mut run = None;
    for step in address.array("nodePath") {
        let index = match step {
            Json::Number(value) if value >= 0.0 && value.fract() == 0.0 => value as usize,
            other => return Err(format!("run address nodePath step {other:?} is not an index")),
        };
        let XNode::Element { name: parent, children, .. } = node else { return Err("run address descends through a text node".to_string()) };
        let child = children.get(index).cloned().ok_or_else(|| format!("run address step {index} is outside <{parent}>'s {} children", children.len()))?;
        let XNode::Element { name, .. } = &child else { return Err(format!("run address step {index} of <{parent}> is not an element")) };
        let ordinal = |names: &[&str]| children[..index].iter().filter(|sibling| matches!(sibling, XNode::Element { name, .. } if names.contains(&name.as_str()))).count();
        match (parent.as_str(), name.as_str()) {
            ("w:document", "w:body") => {}
            ("w:body" | "w:tc", "w:p" | "w:tbl") => {
                if let Some(segment) = table.take() {
                    path.segments.push(segment);
                }
                path.index = ordinal(&["w:p", "w:tbl"]);
            }
            ("w:tbl", "w:tr") => table = Some(DPathSegment { block_index: path.index, row: ordinal(&["w:tr"]), cell: 0 }),
            ("w:tr", "w:tc") => table.as_mut().ok_or("run address enters a cell outside a table")?.cell = ordinal(&["w:tc"]),
            ("w:p", "w:r") => run = Some(ordinal(&["w:r"])),
            (outer, inner) => return Err(format!("run address steps from <{outer}> into <{inner}>, outside this oracle's paragraph/table/run model")),
        }
        node = child;
    }
    let expected = address.str("expectedName");
    let XNode::Element { name, .. } = &node else { return Err("run address resolves to a text node".to_string()) };
    if name != "w:r" || !expected.ends_with("}r") {
        return Err(format!("run address resolves <{name}>, expected {expected:?}"));
    }
    Ok((path, run.ok_or("run address names no run")?))
}
//#endregion 🔖️PathAddressing
