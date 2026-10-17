//! 📦️ Package and presentation model of the PPTX oracle: the zip package reader and writer, the transform, shape and slide records and the presentation and slide reader. See [`super`].

use super::*;

//#region 🔖️Package
/// 📦 Every OPC part, read/written by the registered `zip` reference implementation —
/// independent of `semio_s_artifact_stdio_zip::opc::OpcPackage`, this repository's own codec.
#[derive(Clone, Debug, Default)]
pub(super) struct Package {
    pub(super) parts: HashMap<String, Vec<u8>>,
    pub(super) order: Vec<String>,
}

impl Package {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free) — see R9
    pub(super) fn xml(&self, path: &str) -> Result<XNode, String> {
        parse_document(self.parts.get(path).ok_or_else(|| format!("OPC part missing: {path}"))?)
    }
}

pub(super) fn read_zip(bytes: &[u8]) -> Result<Package, String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes.to_vec())).map_err(|e| format!("independent reader could not parse the PPTX (ZIP): {e}"))?;
    let mut parts = HashMap::with_capacity(archive.len());
    let mut order = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut member = archive.by_index(index).map_err(|e| format!("independent reader could not read PPTX ZIP entry {index}: {e}"))?;
        if member.is_dir() {
            continue;
        }
        let name = member.name().to_string();
        let mut data = Vec::new();
        member.read_to_end(&mut data).map_err(|e| format!("independent reader could not decompress {name}: {e}"))?;
        order.push(name.clone());
        parts.insert(name, data);
    }
    Ok(Package { parts, order })
}

pub(super) fn write_zip(pkg: &Package) -> Result<Vec<u8>, String> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::<u8>::new()));
    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut names: Vec<&String> = pkg.parts.keys().collect();
    names.sort();
    for name in names {
        writer.start_file(name.clone(), options).map_err(|e| format!("zip start_file {name}: {e}"))?;
        writer.write_all(&pkg.parts[name]).map_err(|e| format!("zip write {name}: {e}"))?;
    }
    let cursor = writer.finish().map_err(|e| format!("zip finish: {e}"))?;
    Ok(cursor.into_inner())
}
//#endregion 🔖️Package

//#region 🔖️Types
/// 📐 A shape's `a:xfrm` position/size, in EMUs — mirrors
/// `crate::schema::snapshot::PptxTransform` field-for-field, independent type.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct Transform {
    pub(super) x: i64,
    pub(super) y: i64,
    pub(super) cx: i64,
    pub(super) cy: i64,
}

/// 🖼️ One shape from a slide's `p:spTree` — mirrors `PptxShape`'s 4 variants independently.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum PShape {
    TextBox { text: String, position: Transform },
    Picture { blip_rel_id: String, position: Transform },
    Placeholder { kind: String, text: String, position: Transform },
    Other { node: XNode },
}

/// 🎞️ One slide: its shape tree, in document order.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct PSlide {
    pub(super) shapes: Vec<PShape>,
}
//#endregion 🔖️Types

//#region 🔖️ReadPresentation
/// 🔎️ `_rels/.rels` → the office-document relationship's target (`ppt/presentation.xml`).
pub(super) fn presentation_part_path(pkg: &Package) -> Result<String, String> {
    let root_rels = pkg.xml("_rels/.rels")?;
    let found = root_rels.children_named("Relationship").find(|rel| rel.attr("Type").map(|t| t.ends_with("/officeDocument")).unwrap_or(false)).and_then(|rel| rel.attr("Target")).map(|t| t.to_string());
    found.ok_or_else(|| "_rels/.rels: no officeDocument relationship".to_string())
}

pub(super) fn rels_path_for(part_path: &str) -> String {
    let (dir, file) = match part_path.rfind('/') {
        Some(index) => (&part_path[..index], &part_path[index + 1..]),
        None => ("", part_path),
    };
    if dir.is_empty() {
        format!("_rels/{file}.rels")
    } else {
        format!("{dir}/_rels/{file}.rels")
    }
}

fn resolve_relative(base_dir: &str, target: &str) -> String {
    let mut segments: Vec<&str> = base_dir.split('/').filter(|s| !s.is_empty()).collect();
    for part in target.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }
    segments.join("/")
}

fn position_from_shape(node: &XNode) -> Transform {
    let Some(sp_pr) = node.child("p:spPr") else { return Transform::default() };
    let Some(xfrm) = sp_pr.child("a:xfrm") else { return Transform::default() };
    let mut t = Transform::default();
    if let Some(off) = xfrm.child("a:off") {
        t.x = off.attr("x").and_then(|v| v.parse().ok()).unwrap_or(0);
        t.y = off.attr("y").and_then(|v| v.parse().ok()).unwrap_or(0);
    }
    if let Some(ext) = xfrm.child("a:ext") {
        t.cx = ext.attr("cx").and_then(|v| v.parse().ok()).unwrap_or(0);
        t.cy = ext.attr("cy").and_then(|v| v.parse().ok()).unwrap_or(0);
    }
    t
}

/// 🔎️ Every `a:p` inside `p:txBody`, run text concatenated per paragraph, paragraphs joined
/// by `\n` — a reasonably-scoped simplification of the full `Vec<PptxParagraph>` model (this
/// oracle checks shape TEXT CONTENT and boundaries, not per-run bold/italic/font-size styling).
fn text_from_shape(node: &XNode) -> String {
    let Some(tx_body) = node.child("p:txBody") else { return String::new() };
    tx_body.children_named("a:p").map(|p| p.children_named("a:r").map(|r| r.child("a:t").map(|t| t.text()).unwrap_or_default()).collect::<String>()).collect::<Vec<_>>().join("\n")
}

fn shape_from_node(node: &XNode) -> PShape {
    let XNode::Element { name, .. } = node else { return PShape::Other { node: node.clone() } };
    match name.as_str() {
        "p:sp" => {
            let ph_type = node.child("p:nvSpPr").and_then(|nv| nv.child("p:nvPr")).and_then(|nv_pr| nv_pr.child("p:ph")).map(|ph| ph.attr("type").unwrap_or("body").to_string());
            let position = position_from_shape(node);
            let text = text_from_shape(node);
            match ph_type {
                Some(kind) => PShape::Placeholder { kind, text, position },
                None => PShape::TextBox { text, position },
            }
        }
        "p:pic" => {
            let blip_rel_id = node.child("p:blipFill").and_then(|fill| fill.child("a:blip")).and_then(|blip| blip.attr("r:embed")).unwrap_or("").to_string();
            PShape::Picture { blip_rel_id, position: position_from_shape(node) }
        }
        _ => PShape::Other { node: node.clone() },
    }
}

/// 🔎️ Parses one `ppt/slides/slideN.xml` part into its ordered shape list — `p:spTree`'s
/// direct children, skipping the group's own `p:nvGrpSpPr`/`p:grpSpPr` container elements.
fn read_slide(pkg: &Package, path: &str) -> Result<PSlide, String> {
    let doc = pkg.xml(path)?;
    let c_sld = doc.child("p:cSld").ok_or_else(|| format!("{path}: missing p:cSld"))?;
    let sp_tree = c_sld.child("p:spTree").ok_or_else(|| format!("{path}: missing p:spTree"))?;
    let XNode::Element { children, .. } = sp_tree else { return Err(format!("{path}: p:spTree is not an element")) };
    let shapes = children.iter().filter(|c| !matches!(c, XNode::Element { name, .. } if name == "p:nvGrpSpPr" || name == "p:grpSpPr")).map(shape_from_node).collect();
    Ok(PSlide { shapes })
}

/// 🔎️ The full ordered slide list: `_rels/.rels` → `presentation.xml` → `p:sldIdLst`'s
/// ordered `r:id`s → `presentation.xml.rels` → each `ppt/slides/slideN.xml` in presentation
/// order — every hop resolved independently of `crate`'s own importer.
pub(super) fn read_presentation(pkg: &Package) -> Result<Vec<PSlide>, String> {
    let pres_path = presentation_part_path(pkg)?;
    let pres_doc = pkg.xml(&pres_path)?;
    let sld_id_lst = pres_doc.child("p:sldIdLst");
    let ordered_rids: Vec<String> = sld_id_lst.map(|lst| lst.children_named("p:sldId").filter_map(|sld_id| sld_id.attr("r:id").map(|s| s.to_string())).collect()).unwrap_or_default();

    let pres_rels_path = rels_path_for(&pres_path);
    let pres_rels = pkg.xml(&pres_rels_path)?;
    let base_dir = pres_path.rfind('/').map(|i| &pres_path[..i]).unwrap_or("");
    let mut rid_to_path: HashMap<String, String> = HashMap::new();
    for rel in pres_rels.children_named("Relationship") {
        if let (Some(id), Some(target)) = (rel.attr("Id"), rel.attr("Target")) {
            rid_to_path.insert(id.to_string(), resolve_relative(base_dir, target));
        }
    }

    ordered_rids.iter().map(|rid| rid_to_path.get(rid).ok_or_else(|| format!("presentation.xml.rels: no relationship {rid}")).and_then(|path| read_slide(pkg, path))).collect()
}
//#endregion 🔖️ReadPresentation
