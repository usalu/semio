//! 🏷️ Scoped WordprocessingML names shared by read projections and canonical edits.

use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

pub(crate) const WORDPROCESSINGML_TRANSITIONAL: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub(crate) const WORDPROCESSINGML_STRICT: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
pub(crate) const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";
pub(crate) type Bindings = Vec<(String, String)>;

pub(crate) fn apply_bindings(node: &XmlNode, bindings: &mut Bindings) {
    let XmlNode::Element { attrs, .. } = node else { return };
    for attr in attrs {
        let prefix = if attr.name == "xmlns" { Some("") } else { attr.name.strip_prefix("xmlns:") };
        if let Some(prefix) = prefix {
            if let Some(existing) = bindings.iter_mut().find(|(key, _)| key == prefix) {
                existing.1.clone_from(&attr.value);
            } else {
                bindings.push((prefix.to_string(), attr.value.clone()));
            }
        }
    }
}

pub(crate) fn scoped_bindings(node: &XmlNode, parent: &[(String, String)]) -> Bindings {
    let mut bindings = parent.to_vec();
    apply_bindings(node, &mut bindings);
    bindings
}

fn namespace<'a>(name: &str, bindings: &'a [(String, String)], attribute: bool) -> Result<&'a str, ValueError> {
    let (prefix, _) = name.split_once(':').unwrap_or(("", name));
    if prefix == "xml" {
        Ok(XML_NAMESPACE)
    } else if prefix.is_empty() && attribute {
        Ok("")
    } else {
        match bindings.iter().rev().find(|(key, _)| key == prefix) {
            Some((_, value)) => Ok(value),
            None if prefix.is_empty() => Ok(""),
            None => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("unbound XML namespace prefix in {name}"))),
        }
    }
}

pub(crate) fn expanded_name(name: &str, bindings: &[(String, String)]) -> Result<String, ValueError> {
    let local = name.split_once(':').map_or(name, |(_, local)| local);
    Ok(format!("{{{}}}{local}", namespace(name, bindings, false)?))
}

pub(crate) fn word_namespace<'a>(node: &XmlNode, bindings: &'a [(String, String)]) -> Option<&'a str> {
    let XmlNode::Element { name, .. } = node else { return None };
    let value = namespace(name, bindings, false).ok()?;
    matches!(value, WORDPROCESSINGML_TRANSITIONAL | WORDPROCESSINGML_STRICT).then_some(value)
}

pub(crate) fn word_local_name<'a>(node: &'a XmlNode, bindings: &[(String, String)]) -> Option<&'a str> {
    word_namespace(node, bindings)?;
    let XmlNode::Element { name, .. } = node else { return None };
    Some(name.split_once(':').map_or(name.as_str(), |(_, local)| local))
}

pub(crate) fn is_word_name(node: &XmlNode, local: &str, bindings: &[(String, String)]) -> bool {
    word_local_name(node, bindings) == Some(local)
}

pub(crate) fn word_attr<'a>(node: &'a XmlNode, local: &str, bindings: &[(String, String)]) -> Option<&'a str> {
    let word = word_namespace(node, bindings)?;
    let XmlNode::Element { attrs, .. } = node else { return None };
    attrs.iter().find_map(|attr| {
        let attr_local = attr.name.split_once(':').map_or(attr.name.as_str(), |(_, local)| local);
        (attr_local == local && namespace(&attr.name, bindings, true).is_ok_and(|value| value == word)).then_some(attr.value.as_str())
    })
}

pub(crate) fn qualified_word_prefix(node: &mut XmlNode, bindings: &mut Bindings) -> Result<String, ValueError> {
    let word = word_namespace(node, bindings).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"property container is not WordprocessingML".to_string()))?.to_string();
    let XmlNode::Element { name, attrs, .. } = node else { unreachable!() };
    if let Some((prefix, _)) = name.split_once(':') {
        return Ok(prefix.to_string());
    }
    if let Some((prefix, _)) = bindings.iter().find(|(prefix, value)| !prefix.is_empty() && value == &word) {
        return Ok(prefix.clone());
    }
    let mut prefix = "w".to_string();
    let mut index = 1;
    while bindings.iter().any(|(key, _)| key == &prefix) {
        prefix = format!("w{index}");
        index += 1;
    }
    attrs.push(XmlAttr { name: format!("xmlns:{prefix}"), value: word.clone() });
    bindings.push((prefix.clone(), word));
    Ok(prefix)
}

pub(crate) fn set_word_attr(node: &mut XmlNode, local: &str, value: &str, bindings: &mut Bindings) -> Result<(), ValueError> {
    let word = word_namespace(node, bindings).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"property is not WordprocessingML".to_string()))?.to_string();
    let XmlNode::Element { attrs, .. } = node else { unreachable!() };
    if let Some(attr) = attrs.iter_mut().find(|attr| {
        let attr_local = attr.name.split_once(':').map_or(attr.name.as_str(), |(_, local)| local);
        attr_local == local && namespace(&attr.name, bindings, true).is_ok_and(|value| value == word)
    }) {
        attr.value = value.into();
        return Ok(());
    }
    let prefix = qualified_word_prefix(node, bindings)?;
    let XmlNode::Element { attrs, .. } = node else { unreachable!() };
    attrs.push(XmlAttr { name: format!("{prefix}:{local}"), value: value.into() });
    Ok(())
}
