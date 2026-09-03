//! 🧰 Shared node addressing for direct XML mutations.
use crate::artifacts::xml::schema::snapshot::{XmlDocument, XmlNode};
use crate::artifacts::xml::XmlSnapshot;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(transparent)]
pub struct XmlNodePath(pub Vec<usize>);
impl XmlNodePath {
    pub fn root() -> Self { Self(Vec::new()) }
    pub fn resolve<'a>(&self, root: Option<&'a XmlNode>) -> Option<&'a XmlNode> {
        let mut current = root?;
        for &index in &self.0 { let XmlNode::Element { children, .. } = current else { return None }; current = children.get(index)?; }
        Some(current)
    }
}

pub(crate) fn encode_snapshot(snapshot: &XmlSnapshot) -> String {
    use crate::artifacts::xml::schema::diff::{enc_declaration, enc_doctype, enc_prolog, enc_str, enc_xml_node, encode_option};
    format!("[{},{},{},{},{}]", enc_str(&snapshot.schema), encode_option(&snapshot.doc.root, enc_xml_node), encode_option(&snapshot.doc.doctype, enc_doctype), encode_option(&snapshot.doc.declaration, enc_declaration), enc_prolog(&snapshot.doc.prolog))
}

pub(crate) fn decode_snapshot(value: &str) -> Result<XmlSnapshot, String> {
    use crate::artifacts::xml::schema::diff::{dec_declaration, dec_doctype, dec_prolog, dec_str, dec_xml_node, decode_option, split_top_level, strip_brackets};
    let parts = split_top_level(strip_brackets(value)?, ',');
    let [schema, root, doctype, declaration, prolog] = parts.as_slice() else { return Err(format!("xml snapshot: expected 5 fields, got {}", parts.len())); };
    Ok(XmlSnapshot { schema: dec_str(schema)?, doc: XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)? } })
}

pub(crate) fn encode_snapshot_binary(snapshot: &XmlSnapshot, output: &mut Vec<u8>) {
    use crate::artifacts::xml::schema::diff::{enc_declaration_bin, enc_doctype_bin, enc_prolog_bin, enc_xml_node_bin, write_str_lp};
    write_str_lp(output, &snapshot.schema);
    output.push(u8::from(snapshot.doc.root.is_some()));
    if let Some(root) = &snapshot.doc.root { enc_xml_node_bin(root, output); }
    output.push(u8::from(snapshot.doc.doctype.is_some()));
    if let Some(doctype) = &snapshot.doc.doctype { enc_doctype_bin(doctype, output); }
    output.push(u8::from(snapshot.doc.declaration.is_some()));
    if let Some(declaration) = &snapshot.doc.declaration { enc_declaration_bin(declaration, output); }
    enc_prolog_bin(&snapshot.doc.prolog, output);
}

pub(crate) fn decode_snapshot_binary(reader: &mut store::ByteReader<'_>) -> Result<XmlSnapshot, String> {
    use crate::artifacts::xml::schema::diff::{dec_declaration_bin, dec_doctype_bin, dec_prolog_bin, dec_xml_node_bin, read_str_lp};
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    Ok(XmlSnapshot { schema, doc: XmlDocument { root, doctype, declaration, prolog: dec_prolog_bin(reader)? } })
}
