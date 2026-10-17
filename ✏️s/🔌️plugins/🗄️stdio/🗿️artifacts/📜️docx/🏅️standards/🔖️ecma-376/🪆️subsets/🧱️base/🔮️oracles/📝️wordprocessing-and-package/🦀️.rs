//! 📝️ The WordprocessingML block model, its parse and serialize halves and the package read and write.

use super::*;

//#region 🔖️WordprocessingModel
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct WRun {
    pub(super) text: String,
    pub(super) bold: bool,
    pub(super) italic: bool,
    pub(super) underline: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct WParagraph {
    pub(super) runs: Vec<WRun>,
    pub(super) style: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct WCell {
    pub(super) blocks: Vec<WBlock>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct WRow {
    pub(super) cells: Vec<WCell>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct WTable {
    pub(super) rows: Vec<WRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum WBlock {
    Paragraph(WParagraph),
    Table(WTable),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct WStyle {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) based_on: Option<String>,
}

//#region 🔖️Parse
/// 🔘️ An ST_OnOff toggle (ECMA-376 Part 1 §17.17.4): on unless `w:val` switches it off with `0`/`false`/`off`.
fn on_off(node: &XNode) -> bool {
    !matches!(node, XNode::Element { attrs, .. } if matches!(find_attr(attrs, "w:val"), Some("0" | "false" | "off")))
}

fn run_from_xnode(node: &XNode) -> WRun {
    let mut run = WRun::default();
    let XNode::Element { children, .. } = node else { return run };
    for child in children {
        let XNode::Element { name, children: inner, .. } = child else { continue };
        match name.as_str() {
            "w:rPr" => {
                for prop in inner {
                    let XNode::Element { name, .. } = prop else { continue };
                    match name.as_str() {
                        "w:b" => run.bold = on_off(prop),
                        "w:i" => run.italic = on_off(prop),
                        "w:u" => run.underline = !matches!(prop, XNode::Element { attrs, .. } if find_attr(attrs, "w:val") == Some("none")),
                        _ => {}
                    }
                }
            }
            "w:t" => {
                for t in inner {
                    if let XNode::Text(text) = t {
                        run.text.push_str(text);
                    }
                }
            }
            _ => {}
        }
    }
    run
}

fn paragraph_from_xnode(node: &XNode) -> WParagraph {
    let mut paragraph = WParagraph::default();
    let XNode::Element { children, .. } = node else { return paragraph };
    for child in children {
        let XNode::Element { name, children: inner, .. } = child else { continue };
        match name.as_str() {
            "w:pPr" => {
                for prop in inner {
                    let XNode::Element { name, attrs, .. } = prop else { continue };
                    if name == "w:pStyle" {
                        paragraph.style = find_attr(attrs, "w:val").map(str::to_string);
                    }
                }
            }
            "w:r" => paragraph.runs.push(run_from_xnode(child)),
            _ => {}
        }
    }
    paragraph
}

fn cell_from_xnode(node: &XNode) -> WCell {
    let mut cell = WCell::default();
    let XNode::Element { children, .. } = node else { return cell };
    for child in children {
        let XNode::Element { name, .. } = child else { continue };
        match name.as_str() {
            "w:p" => cell.blocks.push(WBlock::Paragraph(paragraph_from_xnode(child))),
            "w:tbl" => cell.blocks.push(WBlock::Table(table_from_xnode(child))),
            _ => {}
        }
    }
    cell
}

fn row_from_xnode(node: &XNode) -> WRow {
    let mut row = WRow::default();
    let XNode::Element { children, .. } = node else { return row };
    for child in children {
        let XNode::Element { name, .. } = child else { continue };
        if name == "w:tc" {
            row.cells.push(cell_from_xnode(child));
        }
    }
    row
}

fn table_from_xnode(node: &XNode) -> WTable {
    let mut table = WTable::default();
    let XNode::Element { children, .. } = node else { return table };
    for child in children {
        let XNode::Element { name, .. } = child else { continue };
        if name == "w:tr" {
            table.rows.push(row_from_xnode(child));
        }
    }
    table
}

/// 📄 `word/document.xml`'s `<w:document><w:body>...</w:body></w:document>` into a block tree,
/// independently re-derived by re-parsing the part's own bytes through `quick-xml` rather than
/// trusting whatever produced them. Mirrors the exact WordprocessingML shape
/// `crate::…::io::import::deserializers::document_from_xml` reads, reimplemented
/// here against this file's own `XNode` (no shared code, no shared dependency).
fn document_body_from_xnode(root: &XNode) -> Result<Vec<WBlock>, String> {
    let XNode::Element { name, children, .. } = root else { return Err("word/document.xml: root is not an element".to_string()) };
    if name != "w:document" {
        return Err(format!("word/document.xml: expected <w:document>, got <{name}>"));
    }
    let body = children
        .iter()
        .find_map(|c| match c {
            XNode::Element { name, children, .. } if name == "w:body" => Some(children),
            _ => None,
        })
        .ok_or("word/document.xml: missing <w:body>")?;
    let mut blocks = Vec::new();
    for node in body {
        let XNode::Element { name, .. } = node else { continue };
        match name.as_str() {
            "w:p" => blocks.push(WBlock::Paragraph(paragraph_from_xnode(node))),
            "w:tbl" => blocks.push(WBlock::Table(table_from_xnode(node))),
            _ => {}
        }
    }
    Ok(blocks)
}

fn styles_from_xnode(root: &XNode) -> Result<Vec<WStyle>, String> {
    let XNode::Element { name, children, .. } = root else { return Err("word/styles.xml: root is not an element".to_string()) };
    if name != "w:styles" {
        return Err(format!("word/styles.xml: expected <w:styles>, got <{name}>"));
    }
    let mut styles = Vec::new();
    for child in children {
        let XNode::Element { name, attrs, children: inner } = child else { continue };
        if name != "w:style" {
            continue;
        }
        let id = find_attr(attrs, "w:styleId").unwrap_or_default().to_string();
        let mut style_name = id.clone();
        let mut based_on = None;
        for prop in inner {
            let XNode::Element { name, attrs: pattrs, .. } = prop else { continue };
            match name.as_str() {
                "w:name" => style_name = find_attr(pattrs, "w:val").unwrap_or(&style_name).to_string(),
                "w:basedOn" => based_on = find_attr(pattrs, "w:val").map(str::to_string),
                _ => {}
            }
        }
        styles.push(WStyle { id, name: style_name, based_on });
    }
    Ok(styles)
}
//#endregion 🔖️Parse

//#region 🔖️Serialize
fn run_to_xnode(run: &WRun) -> XNode {
    let mut children = Vec::new();
    if run.bold || run.italic || run.underline {
        let mut props = Vec::new();
        if run.bold {
            props.push(elem("w:b", vec![], vec![]));
        }
        if run.italic {
            props.push(elem("w:i", vec![], vec![]));
        }
        if run.underline {
            props.push(elem("w:u", vec![], vec![]));
        }
        children.push(elem("w:rPr", vec![], props));
    }
    children.push(elem("w:t", vec![("xml:space", "preserve".to_string())], vec![XNode::Text(run.text.clone())]));
    elem("w:r", vec![], children)
}

fn paragraph_to_xnode(paragraph: &WParagraph) -> XNode {
    let mut children = Vec::new();
    if let Some(style) = &paragraph.style {
        children.push(elem("w:pPr", vec![], vec![elem("w:pStyle", vec![("w:val", style.clone())], vec![])]));
    }
    children.extend(paragraph.runs.iter().map(run_to_xnode));
    elem("w:p", vec![], children)
}

fn cell_to_xnode(cell: &WCell) -> XNode {
    elem("w:tc", vec![], cell.blocks.iter().map(block_to_xnode).collect())
}

fn row_to_xnode(row: &WRow) -> XNode {
    elem("w:tr", vec![], row.cells.iter().map(cell_to_xnode).collect())
}

fn table_to_xnode(table: &WTable) -> XNode {
    elem("w:tbl", vec![], table.rows.iter().map(row_to_xnode).collect())
}

fn block_to_xnode(block: &WBlock) -> XNode {
    match block {
        WBlock::Paragraph(p) => paragraph_to_xnode(p),
        WBlock::Table(t) => table_to_xnode(t),
    }
}

fn document_to_xnode(body: &[WBlock]) -> XNode {
    elem("w:document", vec![("xmlns:w", W_NS.to_string())], vec![elem("w:body", vec![], body.iter().map(block_to_xnode).collect())])
}

fn styles_to_xnode(styles: &[WStyle]) -> XNode {
    let children = styles
        .iter()
        .map(|style| {
            let mut inner = vec![elem("w:name", vec![("w:val", style.name.clone())], vec![])];
            if let Some(based_on) = &style.based_on {
                inner.push(elem("w:basedOn", vec![("w:val", based_on.clone())], vec![]));
            }
            elem("w:style", vec![("w:styleId", style.id.clone())], inner)
        })
        .collect();
    elem("w:styles", vec![("xmlns:w", W_NS.to_string())], children)
}
//#endregion 🔖️Serialize
//#endregion 🔖️WordprocessingModel

//#region 🔖️Package
/// 📦 The full real DOCX this oracle mutates: the lossless OPC container plus the typed
/// `document.xml`/`styles.xml` view, and the two parts' own paths (resolved once, through the
/// package's OWN relationships, exactly as a real Word consumer resolves them — never assumed).
pub(super) struct WPackage {
    pub(super) opc: OPackage,
    pub(super) main_path: String,
    pub(super) styles_path: Option<String>,
    pub(super) body: Vec<WBlock>,
    pub(super) styles: Vec<WStyle>,
}

pub(super) fn read_package(bytes: &[u8]) -> Result<WPackage, String> {
    let opc = read_opc(bytes)?;
    let main_path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).ok_or("OPC package has no root officeDocument relationship")?;
    let main_bytes = opc.part(&main_path).ok_or_else(|| format!("main document part {main_path} referenced by relationship but not present"))?.bytes.clone();
    let body = document_body_from_xnode(&parse_xml(&main_bytes)?)?;

    let styles_path = opc.resolve_relationship(&main_path, REL_TYPE_STYLES);
    let styles = match &styles_path {
        Some(path) => match opc.part(path) {
            Some(part) => styles_from_xnode(&parse_xml(&part.bytes)?)?,
            None => Vec::new(),
        },
        None => Vec::new(),
    };
    Ok(WPackage { opc, main_path, styles_path, body, styles })
}

/// 📦 Re-serializes `word/document.xml` and (when present) `word/styles.xml` from the typed
/// model alone and re-encodes the OPC container — every other real part (docProps/*, any part a
/// mutation added) survives verbatim.
pub(super) fn write_package(pkg: &WPackage) -> Result<Vec<u8>, String> {
    let mut opc = pkg.opc.clone();
    let main_content_type = opc.part(&pkg.main_path).map(|p| p.content_type.clone()).unwrap_or_else(|| "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml".to_string());
    opc.set_part(&pkg.main_path, &main_content_type, write_xml(&document_to_xnode(&pkg.body))?);
    if let Some(styles_path) = &pkg.styles_path {
        let styles_content_type = opc.part(styles_path).map(|p| p.content_type.clone()).unwrap_or_else(|| "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml".to_string());
        opc.set_part(styles_path, &styles_content_type, write_xml(&styles_to_xnode(&pkg.styles))?);
    }
    write_opc(&opc)
}
//#endregion 🔖️Package
