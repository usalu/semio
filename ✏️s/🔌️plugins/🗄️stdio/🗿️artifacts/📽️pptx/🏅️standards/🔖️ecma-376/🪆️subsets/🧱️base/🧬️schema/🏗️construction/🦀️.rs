//! 🏗️ Canonical PresentationML construction over retained OPC/XML authority.

use crate::schema::mutations::xml_address;
use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxSlide, PptxXmlPart};
use crate::standards::v_ecma_376::subsets::base::{schema::{vocabulary::{attr,attribute_value,element_matches,expanded_element_name,namespace_scope,resolve_office_document_relationship,DRAWINGML_NAMESPACES,OFFICE_RELATIONSHIP_NAMESPACES,PRESENTATIONML_NAMESPACES,SLIDE_CONTENT_TYPE}}};
use crate::PptxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{fresh_relationship_id, OpcRelationship, OpcTargetMode};
use std::collections::HashSet;

struct Namespaces {
    presentation_prefix: String,
    presentation_uri: String,
    drawing_prefix: String,
    drawing_uri: String,
    relationship_prefix: String,
    relationship_uri: String,
}

fn qname(prefix: &str, local: &str) -> String {
    if prefix.is_empty() { local.into() } else { format!("{prefix}:{local}") }
}

fn namespace_attr(prefix: &str, uri: &str) -> XmlAttr {
    let name = if prefix.is_empty() { "xmlns".into() } else { format!("xmlns:{prefix}") };
    attr(&name, uri)
}

fn namespace_binding(scope: &[(String, String)], namespaces: &[&str], kind: &str) -> Result<(String, String), String> {
    scope.iter().rev().find(|(_, uri)| namespaces.contains(&uri.as_str())).cloned().ok_or_else(|| format!("PPTX canonical XML has no {kind} namespace binding"))
}

fn element_namespace(node: &XmlNode, scope: &[(String, String)], namespaces: &[&str], kind: &str) -> Result<(String, String), String> {
    let XmlNode::Element { name, .. } = node else { return Err(format!("PPTX canonical construction expected a {kind} element")) };
    let (uri, _) = expanded_element_name(name, scope)?;
    if !namespaces.contains(&uri.as_str()) {
        return Err(format!("PPTX canonical XML element is not in the {kind} namespace"));
    }
    Ok((name.split_once(':').map_or("", |(prefix, _)| prefix).into(), uri))
}

fn namespaces(node: &XmlNode, scope: &[(String, String)], require_drawing: bool, require_relationships: bool) -> Result<Namespaces, String> {
    let (presentation_prefix, presentation_uri) = element_namespace(node, scope, PRESENTATIONML_NAMESPACES, "PresentationML")?;
    let drawing = namespace_binding(scope, DRAWINGML_NAMESPACES, "DrawingML");
    let (drawing_prefix, drawing_uri) = if require_drawing { drawing? } else { drawing.unwrap_or_default() };
    let relationship = namespace_binding(scope, OFFICE_RELATIONSHIP_NAMESPACES, "office relationship");
    let (relationship_prefix, relationship_uri) = if require_relationships { relationship? } else { relationship.unwrap_or_default() };
    if (require_drawing && drawing_prefix.is_empty()) || (require_relationships && relationship_prefix.is_empty()) {
        return Err("PPTX DrawingML and relationship namespaces require explicit prefixes where canonical construction uses them".into());
    }
    Ok(Namespaces { presentation_prefix, presentation_uri, drawing_prefix, drawing_uri, relationship_prefix, relationship_uri })
}

fn element_children(node: &XmlNode) -> Result<&[XmlNode], String> {
    match node {
        XmlNode::Element { children, .. } => Ok(children),
        _ => Err("PPTX canonical construction expected an XML element".into()),
    }
}

fn element_children_mut(node: &mut XmlNode) -> Result<&mut Vec<XmlNode>, String> {
    match node {
        XmlNode::Element { children, .. } => Ok(children),
        _ => Err("PPTX canonical construction expected a mutable XML element".into()),
    }
}

fn child_index(children: &[XmlNode], scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<Option<usize>, String> {
    for (index, child) in children.iter().enumerate() {
        let child_scope = namespace_scope(scope, child);
        if element_matches(child, &child_scope, namespaces, local)? {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

fn node_mut_at_path<'a>(root: &'a mut XmlNode, path: &[usize]) -> Result<&'a mut XmlNode, String> {
    let mut node = root;
    for index in path {
        node = element_children_mut(node)?.get_mut(*index).ok_or_else(|| "PPTX canonical construction path became stale".to_string())?;
    }
    Ok(node)
}

fn scoped_node_at_path<'a>(root: &'a XmlNode, path: &[usize]) -> Result<(&'a XmlNode, Vec<(String, String)>), String> {
    let mut node = root;
    let mut scope = namespace_scope(&[], root);
    for index in path {
        node = element_children(node)?.get(*index).ok_or_else(|| "PPTX canonical construction path became stale".to_string())?;
        scope = namespace_scope(&scope, node);
    }
    Ok((node, scope))
}

fn shape_tree(root: &XmlNode) -> Result<(Vec<usize>, Vec<(String, String)>), String> {
    let root_scope = namespace_scope(&[], root);
    if !element_matches(root, &root_scope, PRESENTATIONML_NAMESPACES, "sld")? {
        return Err("PPTX active slide root is not PresentationML sld".into());
    }
    let root_children = element_children(root)?;
    let c_sld_index = child_index(root_children, &root_scope, PRESENTATIONML_NAMESPACES, "cSld")?.ok_or_else(|| "PPTX active slide has no cSld".to_string())?;
    let c_sld = &root_children[c_sld_index];
    let c_sld_scope = namespace_scope(&root_scope, c_sld);
    let tree_index = child_index(element_children(c_sld)?, &c_sld_scope, PRESENTATIONML_NAMESPACES, "spTree")?.ok_or_else(|| "PPTX active slide has no shape tree".to_string())?;
    let tree = &element_children(c_sld)?[tree_index];
    Ok((vec![c_sld_index, tree_index], namespace_scope(&c_sld_scope, tree)))
}

fn is_shape(node: &XmlNode, scope: &[(String, String)]) -> Result<bool, String> {
    let XmlNode::Element { name, .. } = node else { return Ok(false) };
    let (namespace, local) = expanded_element_name(name, &namespace_scope(scope, node))?;
    Ok(PRESENTATIONML_NAMESPACES.contains(&namespace.as_str()) && !matches!(local.as_str(), "nvGrpSpPr" | "grpSpPr"))
}

fn plain_text_box(node: &XmlNode, scope: &[(String, String)]) -> Result<Option<(usize, Vec<(String, String)>)>, String> {
    let node_scope = namespace_scope(scope, node);
    if !element_matches(node, &node_scope, PRESENTATIONML_NAMESPACES, "sp")? {
        return Ok(None);
    }
    let children = element_children(node)?;
    let Some(nonvisual_index) = child_index(children, &node_scope, PRESENTATIONML_NAMESPACES, "nvSpPr")? else { return Ok(None) };
    let nonvisual = &children[nonvisual_index];
    let nonvisual_scope = namespace_scope(&node_scope, nonvisual);
    let Some(properties_index) = child_index(element_children(nonvisual)?, &nonvisual_scope, PRESENTATIONML_NAMESPACES, "nvPr")? else { return Ok(None) };
    let properties = &element_children(nonvisual)?[properties_index];
    let properties_scope = namespace_scope(&nonvisual_scope, properties);
    if child_index(element_children(properties)?, &properties_scope, PRESENTATIONML_NAMESPACES, "ph")?.is_some() {
        return Ok(None);
    }
    let Some(text_body_index) = child_index(children, &node_scope, PRESENTATIONML_NAMESPACES, "txBody")? else { return Ok(None) };
    let text_body = &children[text_body_index];
    Ok(Some((text_body_index, namespace_scope(&node_scope, text_body))))
}

fn run_node(run: &crate::schema::snapshot::PptxRun, drawing_prefix: &str) -> Result<XmlNode, String> {
    let mut children = Vec::new();
    if run.bold || run.italic || run.font_size.is_some() {
        let mut attrs = Vec::new();
        if let Some(size) = run.font_size {
            attrs.push(attr("sz", &size.checked_mul(100).ok_or_else(|| "PPTX run font size exceeds DrawingML range".to_string())?.to_string()));
        }
        if run.bold {
            attrs.push(attr("b", "1"));
        }
        if run.italic {
            attrs.push(attr("i", "1"));
        }
        children.push(XmlNode::Element { name: qname(drawing_prefix, "rPr"), attrs, children: Vec::new() });
    }
    children.push(XmlNode::Element { name: qname(drawing_prefix, "t"), attrs: Vec::new(), children: vec![XmlNode::Text { text: run.text.clone() }] });
    Ok(XmlNode::Element { name: qname(drawing_prefix, "r"), attrs: Vec::new(), children })
}

fn paragraph_node(paragraph: &PptxParagraph, drawing_prefix: &str) -> Result<XmlNode, String> {
    Ok(XmlNode::Element { name: qname(drawing_prefix, "p"), attrs: Vec::new(), children: paragraph.runs.iter().map(|run| run_node(run, drawing_prefix)).collect::<Result<_, _>>()? })
}

fn collect_shape_ids(node: &XmlNode, scope: &[(String, String)], ids: &mut HashSet<u32>) -> Result<(), String> {
    let node_scope = namespace_scope(scope, node);
    if element_matches(node, &node_scope, PRESENTATIONML_NAMESPACES, "cNvPr")? {
        if let Some(id) = attribute_value(node, &node_scope, &[""], "id")?.and_then(|value| value.parse().ok()) {
            ids.insert(id);
        }
    }
    for child in element_children(node).unwrap_or_default() {
        collect_shape_ids(child, &node_scope, ids)?;
    }
    Ok(())
}

fn fresh_shape_id(tree: &XmlNode, scope: &[(String, String)]) -> Result<u32, String> {
    let mut ids = HashSet::new();
    collect_shape_ids(tree, scope, &mut ids)?;
    (1..=u32::MAX).find(|id| !ids.contains(id)).ok_or_else(|| "PPTX shape identity space is exhausted".into())
}

fn text_box_node(paragraph: &PptxParagraph, names: &Namespaces, id: u32) -> Result<XmlNode, String> {
    let p = &names.presentation_prefix;
    let a = &names.drawing_prefix;
    Ok(XmlNode::Element {
        name: qname(p, "sp"),
        attrs: Vec::new(),
        children: vec![
            XmlNode::Element {
                name: qname(p, "nvSpPr"),
                attrs: Vec::new(),
                children: vec![
                    XmlNode::Element { name: qname(p, "cNvPr"), attrs: vec![attr("id", &id.to_string()), attr("name", &format!("TextBox {id}"))], children: Vec::new() },
                    XmlNode::Element { name: qname(p, "cNvSpPr"), attrs: vec![attr("txBox", "1")], children: Vec::new() },
                    XmlNode::Element { name: qname(p, "nvPr"), attrs: Vec::new(), children: Vec::new() },
                ],
            },
            XmlNode::Element {
                name: qname(p, "spPr"),
                attrs: Vec::new(),
                children: vec![XmlNode::Element {
                    name: qname(a, "xfrm"),
                    attrs: Vec::new(),
                    children: vec![
                        XmlNode::Element { name: qname(a, "off"), attrs: vec![attr("x", "0"), attr("y", "0")], children: Vec::new() },
                        XmlNode::Element { name: qname(a, "ext"), attrs: vec![attr("cx", "0"), attr("cy", "0")], children: Vec::new() },
                    ],
                }],
            },
            XmlNode::Element {
                name: qname(p, "txBody"),
                attrs: Vec::new(),
                children: vec![
                    XmlNode::Element { name: qname(a, "bodyPr"), attrs: Vec::new(), children: Vec::new() },
                    XmlNode::Element { name: qname(a, "lstStyle"), attrs: Vec::new(), children: Vec::new() },
                    paragraph_node(paragraph, a)?,
                ],
            },
        ],
    })
}

fn truly_empty(snapshot: &PptxSnapshot) -> bool {
    snapshot.opc.parts.is_empty()
        && snapshot.opc.content_types.defaults.is_empty()
        && snapshot.opc.content_types.overrides.is_empty()
        && snapshot.opc.relationships.owner_count() == 0
        && snapshot.opc.comment.is_empty()
        && snapshot.xml_parts.is_empty()
}

pub(crate) fn append_paragraph(snapshot: &mut PptxSnapshot, paragraph: PptxParagraph) -> Result<(), String> {
    if truly_empty(snapshot) {
        return Ok(());
    }
    let slides = xml_address::pptx_slides(snapshot)?;
    let Some(active_slide) = slides.last() else { return Ok(()) };
    let part_index = snapshot.xml_parts.iter().position(|part| part.path == active_slide.address.slide_part_path).ok_or_else(|| "PPTX active slide XML part is missing".to_string())?;
    let root = snapshot.xml_parts[part_index].document.root.as_ref().ok_or_else(|| "PPTX active slide has no XML root".to_string())?;
    let (tree_path, tree_scope) = shape_tree(root)?;
    let tree = tree_path.iter().try_fold(root, |node, index| element_children(node)?.get(*index).ok_or_else(|| "PPTX active slide shape tree path is stale".to_string()))?;
    let names = namespaces(tree, &tree_scope, true, false)?;
    let tree_children = element_children(tree)?;
    let mut last_shape_index = None;
    for (index, node) in tree_children.iter().enumerate() {
        if is_shape(node, &tree_scope)? {
            last_shape_index = Some(index);
        }
    }
    let append_target = match last_shape_index {
        Some(shape_index) => plain_text_box(&tree_children[shape_index], &tree_scope)?.map(|(text_body_index, text_body_scope)| (shape_index, text_body_index, text_body_scope)),
        None => None,
    };
    let root = snapshot.xml_parts[part_index].document.root.as_mut().ok_or_else(|| "PPTX active slide has no mutable XML root".to_string())?;
    let tree = node_mut_at_path(root, &tree_path)?;
    if let Some((shape_index, text_body_index, text_body_scope)) = append_target {
        let shape = element_children_mut(tree)?.get_mut(shape_index).ok_or_else(|| "PPTX active text box became stale".to_string())?;
        let text_body = element_children_mut(shape)?.get_mut(text_body_index).ok_or_else(|| "PPTX active text body became stale".to_string())?;
        let (drawing_prefix, _) = namespace_binding(&text_body_scope, DRAWINGML_NAMESPACES, "DrawingML")?;
        element_children_mut(text_body)?.push(paragraph_node(&paragraph, &drawing_prefix)?);
    } else {
        let id = fresh_shape_id(tree, &tree_scope)?;
        element_children_mut(tree)?.push(text_box_node(&paragraph, &names, id)?);
    }
    Ok(())
}

fn fresh_slide_path(snapshot: &PptxSnapshot) -> Result<String, String> {
    let mut occupied = snapshot.xml_parts.iter().map(|part| part.path.as_str()).chain(snapshot.opc.parts.iter().map(|part| part.path.as_str())).map(str::to_string).collect::<HashSet<_>>();
    occupied.extend(snapshot.opc.relationships.groups().map(|(owner, _)| owner.clone()));
    occupied.extend(snapshot.opc.content_types.overrides.iter().map(|(path, _)| path.trim_start_matches('/').to_string()));
    (1..=u32::MAX).map(|number| format!("ppt/slides/slide{number}.xml")).find(|path| !occupied.contains(path)).ok_or_else(|| "PPTX slide part identity space is exhausted".into())
}

fn fresh_slide_id(slides: &[xml_address::PptxSlideProjection]) -> Result<u32, String> {
    let occupied = slides.iter().filter_map(|slide| slide.address.slide_id.parse::<u32>().ok()).collect::<HashSet<_>>();
    (256..=u32::MAX).find(|id| !occupied.contains(id)).ok_or_else(|| "PPTX slide identity space is exhausted".into())
}

fn blank_slide(names: &Namespaces) -> XmlDocument {
    let p = &names.presentation_prefix;
    XmlDocument {
        root: Some(XmlNode::Element {
            name: qname(p, "sld"),
            attrs: vec![namespace_attr(&names.drawing_prefix, &names.drawing_uri), namespace_attr(p, &names.presentation_uri)],
            children: vec![XmlNode::Element {
                name: qname(p, "cSld"),
                attrs: Vec::new(),
                children: vec![XmlNode::Element {
                    name: qname(p, "spTree"),
                    attrs: Vec::new(),
                    children: vec![
                        XmlNode::Element {
                            name: qname(p, "nvGrpSpPr"),
                            attrs: Vec::new(),
                            children: vec![
                                XmlNode::Element { name: qname(p, "cNvPr"), attrs: vec![attr("id", "1"), attr("name", "")], children: Vec::new() },
                                XmlNode::Element { name: qname(p, "cNvGrpSpPr"), attrs: Vec::new(), children: Vec::new() },
                                XmlNode::Element { name: qname(p, "nvPr"), attrs: Vec::new(), children: Vec::new() },
                            ],
                        },
                        XmlNode::Element { name: qname(p, "grpSpPr"), attrs: Vec::new(), children: Vec::new() },
                    ],
                }],
            }],
        }),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
        epilog: Vec::new(),
    }
}

fn presentation_target(presentation_path: &str, slide_path: &str) -> String {
    relative_target(presentation_path, slide_path)
}

fn relative_target(owner: &str, target: &str) -> String {
    let mut base = owner.split('/').collect::<Vec<_>>();
    base.pop();
    let target = target.split('/').collect::<Vec<_>>();
    let common = base.iter().zip(&target).take_while(|(left, right)| left == right).count();
    std::iter::repeat_n("..", base.len() - common).chain(target[common..].iter().copied()).collect::<Vec<_>>().join("/")
}

pub(crate) fn append_slide(snapshot: &mut PptxSnapshot) -> Result<(), String> {
    if truly_empty(snapshot) {
        *snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(PptxPresentation { slides: vec![PptxSlide::default()] });
        return Ok(());
    }
    let slides = xml_address::pptx_slides(snapshot)?;
    let presentation_path = resolve_office_document_relationship(&snapshot.opc).ok_or_else(|| "PPTX package has no officeDocument relationship".to_string())?;
    let presentation_part = snapshot.xml_parts.iter().find(|part| part.path == presentation_path).ok_or_else(|| "PPTX presentation XML part is missing".to_string())?;
    let presentation_root = presentation_part.document.root.as_ref().ok_or_else(|| "PPTX presentation has no XML root".to_string())?;
    let presentation_scope = namespace_scope(&[], presentation_root);
    let names = namespaces(presentation_root, &presentation_scope, false, true)?;
    let path = fresh_slide_path(snapshot)?;
    let slide_id = fresh_slide_id(&slides)?;
    let existing_relationships = snapshot.opc.relationships_for(&presentation_path);
    let slide_type = existing_relationships.iter().find(|relationship| relationship.rel_type.ends_with("/slide")).map(|relationship| relationship.rel_type.clone()).unwrap_or_else(|| format!("{}/slide", names.relationship_uri));
    let mut taken = existing_relationships.iter().map(|relationship| relationship.id.clone()).collect();
    let relationship_id = fresh_relationship_id(&mut taken);
    let layout_template = slides
        .iter()
        .rev()
        .flat_map(|slide| snapshot.opc.relationships_for(&slide.address.slide_part_path))
        .find(|relationship| relationship.rel_type.ends_with("/slideLayout") && relationship.target_mode == OpcTargetMode::Internal)
        .cloned()
        .or_else(|| {
            snapshot.xml_parts.iter().find(|part| part.content_type.ends_with(".slideLayout+xml")).map(|part| OpcRelationship {
                id: "rId1".into(),
                rel_type: format!("{}/slideLayout", names.relationship_uri),
                target: relative_target(&path, &part.path),
                target_mode: OpcTargetMode::Internal,
            })
        })
        .ok_or_else(|| "PPTX presentation has no retained slide-layout part to extend".to_string())?;
    let slide_namespace_path = slides
        .last()
        .map(|slide| slide.address.slide_part_path.as_str())
        .or_else(|| snapshot.xml_parts.iter().find(|part| part.content_type.ends_with(".slideLayout+xml")).map(|part| part.path.as_str()))
        .ok_or_else(|| "PPTX presentation has no retained slide namespace source".to_string())?;
    let slide_namespace_root = snapshot
        .xml_parts
        .iter()
        .find(|part| part.path == slide_namespace_path)
        .and_then(|part| part.document.root.as_ref())
        .ok_or_else(|| "PPTX retained slide namespace source has no XML root".to_string())?;
    let slide_namespace_scope = namespace_scope(&[], slide_namespace_root);
    let slide_names = namespaces(slide_namespace_root, &slide_namespace_scope, true, false)?;
    let vacancy = xml_address::pptx_slide_vacancy(snapshot, slides.len())?;
    let (slide_list, slide_list_scope) = scoped_node_at_path(presentation_root, &vacancy.container.node_path)?;
    let entry_names = namespaces(slide_list, &slide_list_scope, false, true)?;
    let entry = XmlNode::Element {
        name: qname(&entry_names.presentation_prefix, "sldId"),
        attrs: vec![attr("id", &slide_id.to_string()), attr(&qname(&entry_names.relationship_prefix, "id"), &relationship_id)],
        children: Vec::new(),
    };
    let plan = xml_address::insert_slide_plan(snapshot, &vacancy, &entry)?;
    *snapshot = crate::protocol::apply_diff(&plan.diff, &*snapshot).map_err(|error| format!("PPTX slide insertion refused: {error:?}"))?;
    snapshot.xml_parts.push(PptxXmlPart { path: path.clone(), content_type: SLIDE_CONTENT_TYPE.into(), document: blank_slide(&slide_names) });
    snapshot.opc.content_types.set_override(&path, SLIDE_CONTENT_TYPE);
    snapshot.opc.add_relationship(&presentation_path, &relationship_id, &slide_type, &presentation_target(&presentation_path, &path));
    snapshot.opc.relationships.replace_owner(
        path,
        vec![OpcRelationship { id: "rId1".into(), rel_type: layout_template.rel_type, target: layout_template.target, target_mode: OpcTargetMode::Internal }],
    );
    Ok(())
}

#[path="🌱️minimal/🦀️.rs"]
pub mod minimal;
