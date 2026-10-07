//! 🧭️ Revision-bound canonical PresentationML addresses and projections.

use crate::schema::snapshot::{PptxTransform, PptxXmlPart};
use crate::standards::v_ecma_376::subsets::base::{schema::{vocabulary::{attribute_value,element_matches,expanded_element_name,namespace_scope,resolve_office_document_relationship,DRAWINGML_NAMESPACES,OFFICE_RELATIONSHIP_NAMESPACES,PRESENTATIONML_NAMESPACES}}};
use crate::PptxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxXmlAddress {
    pub part_path: String,
    pub node_path: Vec<usize>,
    pub namespace_uri: String,
    pub local_name: String,
    pub revision: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxSlideAddress {
    pub entry: PptxXmlAddress,
    pub slide_part_path: String,
    pub relationship_id: String,
    pub slide_id: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxShapeAddress {
    pub node: PptxXmlAddress,
    pub shape_id: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxXmlVacancyAddress {
    pub container: PptxXmlAddress,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxShapeProjection {
    pub address: PptxShapeAddress,
    pub text: Option<String>,
    pub position: Option<PptxTransform>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxSlideProjection {
    pub address: PptxSlideAddress,
    pub shapes: Vec<PptxShapeProjection>,
}

fn part<'a>(snapshot: &'a PptxSnapshot, path: &str) -> Result<&'a PptxXmlPart, String> {
    snapshot.xml_parts.iter().find(|part| part.path == path).ok_or_else(|| format!("PPTX XML part {path} does not exist"))
}

fn part_mut<'a>(snapshot: &'a mut PptxSnapshot, path: &str) -> Result<&'a mut PptxXmlPart, String> {
    snapshot.xml_parts.iter_mut().find(|part| part.path == path).ok_or_else(|| format!("PPTX XML part {path} does not exist"))
}

fn validate_path(path: &[usize]) -> Result<(), String> {
    if path.iter().any(|index| *index as u64 > MAX_SAFE_INTEGER) {
        return Err("PPTX XML node path exceeds the schema integer domain".into());
    }
    Ok(())
}

fn validate_revision(revision: &str) -> Result<(), String> {
    if revision.len() != 16 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
        return Err("PPTX XML revision must be sixteen lowercase hexadecimal digits".into());
    }
    Ok(())
}

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

fn hash_node(hash: &mut u64, node: &XmlNode) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            hash_bytes(hash, b"E");
            hash_bytes(hash, name.as_bytes());
            for attr in attrs {
                hash_bytes(hash, b"A");
                hash_bytes(hash, attr.name.as_bytes());
                hash_bytes(hash, b"=");
                hash_bytes(hash, attr.value.as_bytes());
            }
            for child in children {
                hash_node(hash, child);
            }
            hash_bytes(hash, b"/E");
        }
        XmlNode::Text { text } => {
            hash_bytes(hash, b"T");
            hash_bytes(hash, text.as_bytes());
        }
        XmlNode::CData { text } => {
            hash_bytes(hash, b"C");
            hash_bytes(hash, text.as_bytes());
        }
        XmlNode::Comment { text } => {
            hash_bytes(hash, b"M");
            hash_bytes(hash, text.as_bytes());
        }
        XmlNode::ProcessingInstruction { target, data } => {
            hash_bytes(hash, b"P");
            hash_bytes(hash, target.as_bytes());
            hash_bytes(hash, data.as_bytes());
        }
    }
}

pub fn pptx_xml_subtree_revision(node: &XmlNode) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    hash_node(&mut hash, node);
    format!("{hash:016x}")
}

fn node_at_path<'a>(root: &'a XmlNode, path: &[usize]) -> Option<&'a XmlNode> {
    let mut node = root;
    for index in path {
        let XmlNode::Element { children, .. } = node else { return None };
        node = children.get(*index)?;
    }
    Some(node)
}

fn node_mut_at_path<'a>(root: &'a mut XmlNode, path: &[usize]) -> Option<&'a mut XmlNode> {
    let mut node = root;
    for index in path {
        let XmlNode::Element { children, .. } = node else { return None };
        node = children.get_mut(*index)?;
    }
    Some(node)
}

fn scoped_node_at_path<'a>(root: &'a XmlNode, path: &[usize]) -> Result<(&'a XmlNode, Vec<(String, String)>), String> {
    let mut node = root;
    let mut scope = namespace_scope(&[], node);
    for index in path {
        let XmlNode::Element { children, .. } = node else { return Err("PPTX XML address traverses a non-element".into()) };
        node = children.get(*index).ok_or_else(|| "PPTX XML address is outside the retained tree".to_string())?;
        scope = namespace_scope(&scope, node);
    }
    Ok((node, scope))
}

fn address(snapshot: &PptxSnapshot, part_path: &str, node_path: Vec<usize>) -> Result<PptxXmlAddress, String> {
    validate_path(&node_path)?;
    let root = part(snapshot, part_path)?.document.root.as_ref().ok_or_else(|| format!("PPTX XML part {part_path} has no root"))?;
    let (node, scope) = scoped_node_at_path(root, &node_path)?;
    let XmlNode::Element { name, .. } = node else { return Err("PPTX XML address must identify an element".into()) };
    let (namespace_uri, local_name) = expanded_element_name(name, &scope)?;
    Ok(PptxXmlAddress { part_path: part_path.into(), node_path, namespace_uri, local_name, revision: pptx_xml_subtree_revision(node) })
}

pub fn resolve_pptx_xml_address<'a>(snapshot: &'a PptxSnapshot, address: &PptxXmlAddress) -> Result<&'a XmlNode, String> {
    validate_path(&address.node_path)?;
    validate_revision(&address.revision)?;
    let root = part(snapshot, &address.part_path)?.document.root.as_ref().ok_or_else(|| format!("PPTX XML part {} has no root", address.part_path))?;
    let (node, scope) = scoped_node_at_path(root, &address.node_path)?;
    let XmlNode::Element { name, .. } = node else { return Err("PPTX XML address no longer identifies an element".into()) };
    let (namespace_uri, local_name) = expanded_element_name(name, &scope)?;
    if namespace_uri != address.namespace_uri || local_name != address.local_name || pptx_xml_subtree_revision(node) != address.revision {
        return Err("PPTX XML address revision or expanded name is stale".into());
    }
    Ok(node)
}

pub fn resolve_pptx_xml_address_mut<'a>(snapshot: &'a mut PptxSnapshot, address: &PptxXmlAddress) -> Result<&'a mut XmlNode, String> {
    resolve_pptx_xml_address(snapshot, address)?;
    let root = part_mut(snapshot, &address.part_path)?.document.root.as_mut().ok_or_else(|| format!("PPTX XML part {} has no root", address.part_path))?;
    node_mut_at_path(root, &address.node_path).ok_or_else(|| "PPTX XML address became stale during apply".into())
}

fn matching_child_indices(children: &[XmlNode], parent_scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<Vec<usize>, String> {
    let mut output = Vec::new();
    for (index, child) in children.iter().enumerate() {
        let scope = namespace_scope(parent_scope, child);
        if element_matches(child, &scope, namespaces, local)? {
            output.push(index);
        }
    }
    Ok(output)
}

fn direct_child_index(children: &[XmlNode], parent_scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<Option<usize>, String> {
    Ok(matching_child_indices(children, parent_scope, namespaces, local)?.into_iter().next())
}

fn element_children(node: &XmlNode) -> Result<&[XmlNode], String> {
    match node {
        XmlNode::Element { children, .. } => Ok(children),
        _ => Err("PPTX canonical element became non-element".into()),
    }
}

fn shape_id(node: &XmlNode, scope: &[(String, String)]) -> Result<Option<String>, String> {
    let children = element_children(node)?;
    for child in children {
        let child_scope = namespace_scope(scope, child);
        let XmlNode::Element { children: nonvisual, .. } = child else { continue };
        let (_, local) = match child {
            XmlNode::Element { name, .. } => expanded_element_name(name, &child_scope)?,
            _ => continue,
        };
        if !local.starts_with("nv") {
            continue;
        }
        for candidate in nonvisual {
            let candidate_scope = namespace_scope(&child_scope, candidate);
            if element_matches(candidate, &candidate_scope, PRESENTATIONML_NAMESPACES, "cNvPr")? {
                return Ok(attribute_value(candidate, &candidate_scope, &[""], "id")?.map(str::to_string));
            }
        }
    }
    Ok(None)
}

fn collect_text(node: &XmlNode, scope: &[(String, String)], output: &mut String, paragraph: &mut bool) -> Result<(), String> {
    let XmlNode::Element { children, .. } = node else { return Ok(()) };
    let is_paragraph = element_matches(node, scope, DRAWINGML_NAMESPACES, "p")?;
    if is_paragraph && *paragraph {
        output.push('\n');
    }
    if is_paragraph {
        *paragraph = true;
    }
    if element_matches(node, scope, DRAWINGML_NAMESPACES, "t")? {
        for child in children {
            if let XmlNode::Text { text } | XmlNode::CData { text } = child {
                output.push_str(text);
            }
        }
        return Ok(());
    }
    for child in children {
        let child_scope = namespace_scope(scope, child);
        collect_text(child, &child_scope, output, paragraph)?;
    }
    Ok(())
}

fn shape_text(node: &XmlNode, scope: &[(String, String)]) -> Result<Option<String>, String> {
    let mut text = String::new();
    let mut paragraph = false;
    collect_text(node, scope, &mut text, &mut paragraph)?;
    Ok(paragraph.then_some(text))
}

fn parse_i64(value: Option<&str>) -> i64 {
    value.and_then(|value| value.parse().ok()).unwrap_or_default()
}

fn find_descendant<'a>(node: &'a XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<Option<(&'a XmlNode, Vec<(String, String)>)>, String> {
    if element_matches(node, scope, namespaces, local)? {
        return Ok(Some((node, scope.to_vec())));
    }
    let XmlNode::Element { children, .. } = node else { return Ok(None) };
    for child in children {
        let child_scope = namespace_scope(scope, child);
        if let Some(found) = find_descendant(child, &child_scope, namespaces, local)? {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

fn shape_position(node: &XmlNode, scope: &[(String, String)]) -> Result<Option<PptxTransform>, String> {
    let Some((xfrm, xfrm_scope)) = find_descendant(node, scope, DRAWINGML_NAMESPACES, "xfrm")? else { return Ok(None) };
    let children = element_children(xfrm)?;
    let off = direct_child_index(children, &xfrm_scope, DRAWINGML_NAMESPACES, "off")?.and_then(|index| children.get(index));
    let ext = direct_child_index(children, &xfrm_scope, DRAWINGML_NAMESPACES, "ext")?.and_then(|index| children.get(index));
    let off_scope = off.map(|node| namespace_scope(&xfrm_scope, node));
    let ext_scope = ext.map(|node| namespace_scope(&xfrm_scope, node));
    let x = match off {
        Some(node) => attribute_value(node, off_scope.as_deref().unwrap_or(&xfrm_scope), &[""], "x")?,
        None => None,
    };
    let y = match off {
        Some(node) => attribute_value(node, off_scope.as_deref().unwrap_or(&xfrm_scope), &[""], "y")?,
        None => None,
    };
    let cx = match ext {
        Some(node) => attribute_value(node, ext_scope.as_deref().unwrap_or(&xfrm_scope), &[""], "cx")?,
        None => None,
    };
    let cy = match ext {
        Some(node) => attribute_value(node, ext_scope.as_deref().unwrap_or(&xfrm_scope), &[""], "cy")?,
        None => None,
    };
    Ok(Some(PptxTransform { x: parse_i64(x), y: parse_i64(y), cx: parse_i64(cx), cy: parse_i64(cy) }))
}

fn child_path(parent: &[usize], index: usize) -> Vec<usize> {
    let mut path = parent.to_vec();
    path.push(index);
    path
}

pub fn pptx_slides(snapshot: &PptxSnapshot) -> Result<Vec<PptxSlideProjection>, String> {
    let presentation_path = resolve_office_document_relationship(&snapshot.opc).ok_or_else(|| "PPTX package has no officeDocument relationship".to_string())?;
    let presentation = part(snapshot, &presentation_path)?;
    let root = presentation.document.root.as_ref().ok_or_else(|| "PPTX presentation XML has no root".to_string())?;
    let root_scope = namespace_scope(&[], root);
    if !element_matches(root, &root_scope, PRESENTATIONML_NAMESPACES, "presentation")? {
        return Err("PPTX officeDocument target is not a PresentationML presentation".into());
    }
    let root_children = element_children(root)?;
    let list_index = direct_child_index(root_children, &root_scope, PRESENTATIONML_NAMESPACES, "sldIdLst")?.ok_or_else(|| "PPTX presentation has no slide id list".to_string())?;
    let list = &root_children[list_index];
    let list_scope = namespace_scope(&root_scope, list);
    let list_children = element_children(list)?;
    let mut slides = Vec::new();
    for entry_index in matching_child_indices(list_children, &list_scope, PRESENTATIONML_NAMESPACES, "sldId")? {
        let entry = &list_children[entry_index];
        let entry_scope = namespace_scope(&list_scope, entry);
        let relationship_id = attribute_value(entry, &entry_scope, OFFICE_RELATIONSHIP_NAMESPACES, "id")?.ok_or_else(|| "PPTX slide entry has no relationship id".to_string())?.to_string();
        let slide_id = attribute_value(entry, &entry_scope, &[""], "id")?.unwrap_or_default().to_string();
        let relationship = snapshot.opc.relationships_for(&presentation_path).iter().find(|relationship| relationship.id == relationship_id).ok_or_else(|| format!("PPTX slide relationship {relationship_id} does not exist"))?;
        let slide_part_path = resolve_relationship_target(&presentation_path, &relationship.target);
        let slide_part = part(snapshot, &slide_part_path)?;
        let slide_root = slide_part.document.root.as_ref().ok_or_else(|| format!("PPTX slide {slide_part_path} has no root"))?;
        let slide_scope = namespace_scope(&[], slide_root);
        if !element_matches(slide_root, &slide_scope, PRESENTATIONML_NAMESPACES, "sld")? {
            return Err(format!("PPTX slide {slide_part_path} root is not PresentationML sld"));
        }
        let slide_children = element_children(slide_root)?;
        let c_sld_index = direct_child_index(slide_children, &slide_scope, PRESENTATIONML_NAMESPACES, "cSld")?.ok_or_else(|| format!("PPTX slide {slide_part_path} has no cSld"))?;
        let c_sld = &slide_children[c_sld_index];
        let c_sld_scope = namespace_scope(&slide_scope, c_sld);
        let c_sld_children = element_children(c_sld)?;
        let tree_index = direct_child_index(c_sld_children, &c_sld_scope, PRESENTATIONML_NAMESPACES, "spTree")?.ok_or_else(|| format!("PPTX slide {slide_part_path} has no shape tree"))?;
        let tree = &c_sld_children[tree_index];
        let tree_scope = namespace_scope(&c_sld_scope, tree);
        let tree_children = element_children(tree)?;
        let mut shapes = Vec::new();
        for (shape_index, shape) in tree_children.iter().enumerate() {
            let shape_scope = namespace_scope(&tree_scope, shape);
            let XmlNode::Element { name, .. } = shape else { continue };
            let (namespace, local) = expanded_element_name(name, &shape_scope)?;
            if !PRESENTATIONML_NAMESPACES.contains(&namespace.as_str()) || matches!(local.as_str(), "nvGrpSpPr" | "grpSpPr") {
                continue;
            }
            let Some(shape_id) = shape_id(shape, &shape_scope)? else { continue };
            let node_path = vec![c_sld_index, tree_index, shape_index];
            shapes.push(PptxShapeProjection { address: PptxShapeAddress { node: address(snapshot, &slide_part_path, node_path)?, shape_id }, text: shape_text(shape, &shape_scope)?, position: shape_position(shape, &shape_scope)? });
        }
        let entry_path = vec![list_index, entry_index];
        slides.push(PptxSlideProjection { address: PptxSlideAddress { entry: address(snapshot, &presentation_path, entry_path)?, slide_part_path, relationship_id, slide_id }, shapes });
    }
    Ok(slides)
}

pub fn pptx_shape(snapshot: &PptxSnapshot, address: &PptxShapeAddress) -> Result<PptxShapeProjection, String> {
    let node = resolve_pptx_xml_address(snapshot, &address.node)?;
    let (_, scope) = scoped_node_at_path(part(snapshot, &address.node.part_path)?.document.root.as_ref().ok_or_else(|| "PPTX slide has no root".to_string())?, &address.node.node_path)?;
    if shape_id(node, &scope)?.as_deref() != Some(address.shape_id.as_str()) {
        return Err("PPTX shape identity is stale".into());
    }
    Ok(PptxShapeProjection { address: address.clone(), text: shape_text(node, &scope)?, position: shape_position(node, &scope)? })
}

pub fn pptx_slide_vacancy(snapshot: &PptxSnapshot, index: usize) -> Result<PptxXmlVacancyAddress, String> {
    let presentation_path = resolve_office_document_relationship(&snapshot.opc).ok_or_else(|| "PPTX package has no officeDocument relationship".to_string())?;
    let root = part(snapshot, &presentation_path)?.document.root.as_ref().ok_or_else(|| "PPTX presentation XML has no root".to_string())?;
    let root_scope = namespace_scope(&[], root);
    let children = element_children(root)?;
    let list_index = direct_child_index(children, &root_scope, PRESENTATIONML_NAMESPACES, "sldIdLst")?.ok_or_else(|| "PPTX presentation has no slide id list".to_string())?;
    let list = &children[list_index];
    let list_scope = namespace_scope(&root_scope, list);
    let count = matching_child_indices(element_children(list)?, &list_scope, PRESENTATIONML_NAMESPACES, "sldId")?.len();
    if index > count {
        return Err("PPTX slide vacancy is outside the slide list".into());
    }
    Ok(PptxXmlVacancyAddress { container: address(snapshot, &presentation_path, vec![list_index])?, index })
}

pub fn pptx_shape_vacancy(snapshot: &PptxSnapshot, slide: &PptxSlideAddress, index: usize) -> Result<PptxXmlVacancyAddress, String> {
    if !pptx_slides(snapshot)?.iter().any(|candidate| candidate.address == *slide) {
        return Err("PPTX slide address is stale".into());
    }
    let root = part(snapshot, &slide.slide_part_path)?.document.root.as_ref().ok_or_else(|| "PPTX slide XML has no root".to_string())?;
    let root_scope = namespace_scope(&[], root);
    let children = element_children(root)?;
    let c_sld_index = direct_child_index(children, &root_scope, PRESENTATIONML_NAMESPACES, "cSld")?.ok_or_else(|| "PPTX slide has no cSld".to_string())?;
    let c_sld = &children[c_sld_index];
    let c_sld_scope = namespace_scope(&root_scope, c_sld);
    let c_sld_children = element_children(c_sld)?;
    let tree_index = direct_child_index(c_sld_children, &c_sld_scope, PRESENTATIONML_NAMESPACES, "spTree")?.ok_or_else(|| "PPTX slide has no shape tree".to_string())?;
    let tree = &c_sld_children[tree_index];
    let tree_scope = namespace_scope(&c_sld_scope, tree);
    let mut count = 0;
    for node in element_children(tree)? {
        let scope = namespace_scope(&tree_scope, node);
        let XmlNode::Element { name, .. } = node else { continue };
        let (namespace, local) = expanded_element_name(name, &scope)?;
        if PRESENTATIONML_NAMESPACES.contains(&namespace.as_str()) && !matches!(local.as_str(), "nvGrpSpPr" | "grpSpPr") {
            count += 1;
        }
    }
    if index > count {
        return Err("PPTX shape vacancy is outside the shape tree".into());
    }
    Ok(PptxXmlVacancyAddress { container: address(snapshot, &slide.slide_part_path, vec![c_sld_index, tree_index])?, index })
}

fn set_attr(node: &mut XmlNode, name: &str, value: String) -> Result<(), String> {
    let XmlNode::Element { attrs, .. } = node else { return Err("PPTX attribute target is not an element".into()) };
    if let Some(attr) = attrs.iter_mut().find(|attr| attr.name == name) {
        attr.value = value;
    } else {
        attrs.push(XmlAttr { name: name.into(), value });
    }
    Ok(())
}

fn collect_text_paths(node: &XmlNode, scope: &[(String, String)], path: &mut Vec<usize>, output: &mut Vec<Vec<usize>>) -> Result<(), String> {
    if element_matches(node, scope, DRAWINGML_NAMESPACES, "t")? {
        output.push(path.clone());
        return Ok(());
    }
    let XmlNode::Element { children, .. } = node else { return Ok(()) };
    for (index, child) in children.iter().enumerate() {
        path.push(index);
        let child_scope = namespace_scope(scope, child);
        collect_text_paths(child, &child_scope, path, output)?;
        path.pop();
    }
    Ok(())
}

fn set_text_node(node: &mut XmlNode, value: &str) -> Result<(), String> {
    let XmlNode::Element { children, .. } = node else { return Err("PPTX text target is not an element".into()) };
    if let Some(text) = children.iter_mut().find_map(|child| match child {
        XmlNode::Text { text } | XmlNode::CData { text } => Some(text),
        _ => None,
    }) {
        *text = value.into();
    } else {
        children.insert(0, XmlNode::Text { text: value.into() });
    }
    Ok(())
}

pub fn set_shape_text(snapshot: &mut PptxSnapshot, address: &PptxShapeAddress, text: &str) -> Result<(), String> {
    let projection = pptx_shape(snapshot, address)?;
    if projection.text.is_none() {
        return Err("PPTX shape has no editable text body".into());
    }
    let node = resolve_pptx_xml_address(snapshot, &address.node)?;
    let root = part(snapshot, &address.node.part_path)?.document.root.as_ref().ok_or_else(|| "PPTX slide has no root".to_string())?;
    let (_, scope) = scoped_node_at_path(root, &address.node.node_path)?;
    let mut paths = Vec::new();
    collect_text_paths(node, &scope, &mut Vec::new(), &mut paths)?;
    if paths.is_empty() {
        return Err("PPTX text body has no DrawingML text node".into());
    }
    let shape = resolve_pptx_xml_address_mut(snapshot, &address.node)?;
    for (index, path) in paths.iter().enumerate() {
        let target = node_mut_at_path(shape, path).ok_or_else(|| "PPTX text address became stale during apply".to_string())?;
        set_text_node(target, if index == 0 { text } else { "" })?;
    }
    Ok(())
}

fn find_descendant_path(node: &XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str, path: &mut Vec<usize>) -> Result<Option<Vec<usize>>, String> {
    if element_matches(node, scope, namespaces, local)? {
        return Ok(Some(path.clone()));
    }
    let XmlNode::Element { children, .. } = node else { return Ok(None) };
    for (index, child) in children.iter().enumerate() {
        path.push(index);
        let child_scope = namespace_scope(scope, child);
        if let Some(found) = find_descendant_path(child, &child_scope, namespaces, local, path)? {
            return Ok(Some(found));
        }
        path.pop();
    }
    Ok(None)
}

pub fn set_shape_position(snapshot: &mut PptxSnapshot, address: &PptxShapeAddress, position: PptxTransform) -> Result<(), String> {
    let projection = pptx_shape(snapshot, address)?;
    if projection.position.is_none() {
        return Err("PPTX shape has no editable transform".into());
    }
    let node = resolve_pptx_xml_address(snapshot, &address.node)?;
    let root = part(snapshot, &address.node.part_path)?.document.root.as_ref().ok_or_else(|| "PPTX slide has no root".to_string())?;
    let (_, scope) = scoped_node_at_path(root, &address.node.node_path)?;
    let xfrm_path = find_descendant_path(node, &scope, DRAWINGML_NAMESPACES, "xfrm", &mut Vec::new())?.ok_or_else(|| "PPTX shape has no transform".to_string())?;
    let shape = resolve_pptx_xml_address_mut(snapshot, &address.node)?;
    let xfrm = node_mut_at_path(shape, &xfrm_path).ok_or_else(|| "PPTX transform became stale during apply".to_string())?;
    let xfrm_scope = namespace_scope(&scope, xfrm);
    let XmlNode::Element { children, .. } = xfrm else { return Err("PPTX transform is not an element".into()) };
    let off_index = direct_child_index(children, &xfrm_scope, DRAWINGML_NAMESPACES, "off")?.ok_or_else(|| "PPTX transform has no offset".to_string())?;
    let ext_index = direct_child_index(children, &xfrm_scope, DRAWINGML_NAMESPACES, "ext")?.ok_or_else(|| "PPTX transform has no extent".to_string())?;
    set_attr(&mut children[off_index], "x", position.x.to_string())?;
    set_attr(&mut children[off_index], "y", position.y.to_string())?;
    set_attr(&mut children[ext_index], "cx", position.cx.to_string())?;
    set_attr(&mut children[ext_index], "cy", position.cy.to_string())?;
    Ok(())
}

fn reorder_matching_children(children: &mut [XmlNode], scope: &[(String, String)], namespaces: &[&str], local: &str, from: usize, to: usize) -> Result<(), String> {
    let slots = matching_child_indices(children, scope, namespaces, local)?;
    if from >= slots.len() || to >= slots.len() {
        return Err("PPTX move destination is outside the addressed collection".into());
    }
    let mut values = slots.iter().map(|index| children[*index].clone()).collect::<Vec<_>>();
    let value = values.remove(from);
    values.insert(to, value);
    for (slot, value) in slots.into_iter().zip(values) {
        children[slot] = value;
    }
    Ok(())
}

pub fn move_slide(snapshot: &mut PptxSnapshot, address: &PptxSlideAddress, destination_index: usize) -> Result<(), String> {
    let slides = pptx_slides(snapshot)?;
    let from = slides.iter().position(|slide| slide.address == *address).ok_or_else(|| "PPTX slide address is stale".to_string())?;
    if slides.is_empty() || destination_index >= slides.len() {
        return Err("PPTX slide destination is outside the slide list".into());
    }
    let mut parent_path = address.entry.node_path.clone();
    parent_path.pop().ok_or_else(|| "PPTX slide entry has no parent".to_string())?;
    let root = part(snapshot, &address.entry.part_path)?.document.root.as_ref().ok_or_else(|| "PPTX presentation has no root".to_string())?;
    let (parent, scope) = scoped_node_at_path(root, &parent_path)?;
    let XmlNode::Element { .. } = parent else { return Err("PPTX slide list is not an element".into()) };
    let presentation = part_mut(snapshot, &address.entry.part_path)?;
    let parent = node_mut_at_path(presentation.document.root.as_mut().ok_or_else(|| "PPTX presentation has no root".to_string())?, &parent_path).ok_or_else(|| "PPTX slide list became stale".to_string())?;
    let XmlNode::Element { children, .. } = parent else { return Err("PPTX slide list is not an element".into()) };
    reorder_matching_children(children, &scope, PRESENTATIONML_NAMESPACES, "sldId", from, destination_index)
}

pub fn remove_shape(snapshot: &mut PptxSnapshot, address: &PptxShapeAddress) -> Result<(), String> {
    pptx_shape(snapshot, address)?;
    let (index, parent_path) = address.node.node_path.split_last().ok_or_else(|| "PPTX shape root cannot be removed".to_string())?;
    let root = part_mut(snapshot, &address.node.part_path)?.document.root.as_mut().ok_or_else(|| "PPTX slide has no root".to_string())?;
    let parent = node_mut_at_path(root, parent_path).ok_or_else(|| "PPTX shape parent became stale".to_string())?;
    let XmlNode::Element { children, .. } = parent else { return Err("PPTX shape parent is not an element".into()) };
    children.remove(*index);
    Ok(())
}

pub fn insert_shape(snapshot: &mut PptxSnapshot, vacancy: &PptxXmlVacancyAddress, shape: XmlNode) -> Result<(), String> {
    resolve_pptx_xml_address(snapshot, &vacancy.container)?;
    let root = part(snapshot, &vacancy.container.part_path)?.document.root.as_ref().ok_or_else(|| "PPTX slide has no root".to_string())?;
    let (_, scope) = scoped_node_at_path(root, &vacancy.container.node_path)?;
    let container = resolve_pptx_xml_address_mut(snapshot, &vacancy.container)?;
    let XmlNode::Element { children, .. } = container else { return Err("PPTX shape vacancy container is not an element".into()) };
    let mut slots = Vec::new();
    for (index, child) in children.iter().enumerate() {
        let child_scope = namespace_scope(&scope, child);
        let XmlNode::Element { name, .. } = child else { continue };
        let (namespace, local) = expanded_element_name(name, &child_scope)?;
        if PRESENTATIONML_NAMESPACES.contains(&namespace.as_str()) && !matches!(local.as_str(), "nvGrpSpPr" | "grpSpPr") {
            slots.push(index);
        }
    }
    let physical = if vacancy.index < slots.len() { slots[vacancy.index] } else { children.len() };
    children.insert(physical, shape);
    Ok(())
}

pub fn remove_slide(snapshot: &mut PptxSnapshot, address: &PptxSlideAddress) -> Result<(), String> {
    let slides = pptx_slides(snapshot)?;
    if !slides.iter().any(|slide| slide.address == *address) {
        return Err("PPTX slide address is stale".into());
    }
    let (index, parent_path) = address.entry.node_path.split_last().ok_or_else(|| "PPTX slide entry root cannot be removed".to_string())?;
    let root = part_mut(snapshot, &address.entry.part_path)?.document.root.as_mut().ok_or_else(|| "PPTX presentation has no root".to_string())?;
    let parent = node_mut_at_path(root, parent_path).ok_or_else(|| "PPTX slide list became stale".to_string())?;
    let XmlNode::Element { children, .. } = parent else { return Err("PPTX slide list is not an element".into()) };
    children.remove(*index);
    Ok(())
}

pub fn insert_slide(snapshot: &mut PptxSnapshot, vacancy: &PptxXmlVacancyAddress, entry: XmlNode) -> Result<(), String> {
    resolve_pptx_xml_address(snapshot, &vacancy.container)?;
    let root = part(snapshot, &vacancy.container.part_path)?.document.root.as_ref().ok_or_else(|| "PPTX presentation has no root".to_string())?;
    let (_, scope) = scoped_node_at_path(root, &vacancy.container.node_path)?;
    let container = resolve_pptx_xml_address_mut(snapshot, &vacancy.container)?;
    let XmlNode::Element { children, .. } = container else { return Err("PPTX slide vacancy container is not an element".into()) };
    let slots = matching_child_indices(children, &scope, PRESENTATIONML_NAMESPACES, "sldId")?;
    let physical = if vacancy.index < slots.len() { slots[vacancy.index] } else { children.len() };
    children.insert(physical, entry);
    Ok(())
}
