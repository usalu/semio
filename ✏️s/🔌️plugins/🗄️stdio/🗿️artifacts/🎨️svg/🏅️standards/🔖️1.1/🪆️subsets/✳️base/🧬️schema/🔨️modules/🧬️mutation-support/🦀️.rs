//! 🧰 Shared attribute diff construction for direct SVG mutations.
use crate::artifacts::svg::schema::diff::{diff_at_path, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::artifacts::svg::schema::snapshot::node_at;
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{XmlDocument, XmlNode};

pub fn attribute_diff_at_path(base: &SvgSnapshot, path: &[usize], name: &str, value: Option<String>) -> SvgDiff {
    let target = node_at(&base.doc, path).ok();
    let existing = target.and_then(|node| match node { XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == name), _ => None });
    let attributes = match (existing, value) {
        (Some(_), Some(value)) => SvgAttributesDiff { removed: Vec::new(), modified: vec![SvgAttrModified { name: name.to_string(), value }], added: Vec::new() },
        (Some(_), None) => SvgAttributesDiff { removed: vec![name.to_string()], modified: Vec::new(), added: Vec::new() },
        (None, Some(value)) => { let index = match target { Some(XmlNode::Element { attrs, .. }) => attrs.len(), _ => 0 }; SvgAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgAttrAdded { index, name: name.to_string(), value }] } },
        (None, None) => SvgAttributesDiff::default(),
    };
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(attributes), children: None }))
}

pub(crate) fn encode_snapshot(snapshot: &SvgSnapshot) -> String {
    use crate::artifacts::svg::schema::diff::{enc_declaration, enc_doctype, enc_prolog, enc_str, enc_xml_node, encode_option};
    format!("[{},{},{},{},{}]", enc_str(&snapshot.schema), encode_option(&snapshot.doc.root, enc_xml_node), encode_option(&snapshot.doc.doctype, enc_doctype), encode_option(&snapshot.doc.declaration, enc_declaration), enc_prolog(&snapshot.doc.prolog))
}

pub(crate) fn decode_snapshot(value: &str) -> Result<SvgSnapshot, String> {
    use crate::artifacts::svg::schema::diff::{dec_declaration, dec_doctype, dec_prolog, dec_str, dec_xml_node, decode_option, split_top_level, strip_brackets};
    let parts = split_top_level(strip_brackets(value)?, ',');
    let [schema, root, doctype, declaration, prolog] = parts.as_slice() else { return Err(format!("svg snapshot: expected 5 fields, got {}", parts.len())); };
    Ok(SvgSnapshot { schema: dec_str(schema)?, doc: XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)? } })
}

pub(crate) fn encode_snapshot_binary(snapshot: &SvgSnapshot, output: &mut Vec<u8>) {
    use crate::artifacts::svg::schema::diff::{enc_declaration_bin, enc_doctype_bin, enc_prolog_bin, enc_xml_node_bin, write_str_lp};
    write_str_lp(output, &snapshot.schema);
    output.push(u8::from(snapshot.doc.root.is_some()));
    if let Some(root) = &snapshot.doc.root { enc_xml_node_bin(root, output); }
    output.push(u8::from(snapshot.doc.doctype.is_some()));
    if let Some(doctype) = &snapshot.doc.doctype { enc_doctype_bin(doctype, output); }
    output.push(u8::from(snapshot.doc.declaration.is_some()));
    if let Some(declaration) = &snapshot.doc.declaration { enc_declaration_bin(declaration, output); }
    enc_prolog_bin(&snapshot.doc.prolog, output);
}

pub(crate) fn decode_snapshot_binary(reader: &mut store::ByteReader<'_>) -> Result<SvgSnapshot, String> {
    use crate::artifacts::svg::schema::diff::{dec_declaration_bin, dec_doctype_bin, dec_prolog_bin, dec_xml_node_bin, read_str_lp};
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    Ok(SvgSnapshot { schema, doc: XmlDocument { root, doctype, declaration, prolog: dec_prolog_bin(reader)? } })
}
