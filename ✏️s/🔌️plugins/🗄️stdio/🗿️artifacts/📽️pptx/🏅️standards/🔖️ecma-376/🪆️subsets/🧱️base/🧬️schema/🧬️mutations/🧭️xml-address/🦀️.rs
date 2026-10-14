//! 🧭️ Revision-bound canonical PresentationML addresses and projections.

use super::{move_slide, remove_slide, remove_shape, insert_slide, insert_shape, replace_xml_node, PptxMutation};
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::diff::PptxDiff;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::diff::PptxXmlPartDiff;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::diff::PptxXmlPartsDelta;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxTransform;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxXmlPart;
use semio_s_artifact_stdio_xml::schema::diff::{diff_at_path, XmlChildAdded, XmlChildModified, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::attribute_value;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::element_matches;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::expanded_element_name;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::namespace_scope;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::resolve_office_document_relationship;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::DRAWINGML_NAMESPACES;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::OFFICE_RELATIONSHIP_NAMESPACES;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::PRESENTATIONML_NAMESPACES;
use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::xml_address::*;

//#region 🔖️Plans
/// 🧩️ One prepared edit: its compact diff, and the exact mutation that undoes it.
pub struct PptxPlan {
    pub diff: PptxDiff,
    pub inverse: PptxMutation,
}

/// 🧩️ The diff that applies `leaf` to the node `node_path` names inside XML part `part_path`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_diff(part_path: &str, node_path: &[usize], leaf: XmlNodeDiff) -> PptxDiff {
    PptxDiff { schema: None, opc: None, xml_parts: Some(PptxXmlPartsDelta::modification(part_path, PptxXmlPartDiff { content_type: None, document: Some(diff_at_path(node_path, leaf)) })) }
}

/// 🌳 The element diff that only edits the children of its target.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn children_leaf(removed: Vec<usize>, modified: Vec<XmlChildModified>, added: Vec<XmlChildAdded>) -> XmlNodeDiff {
    XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed, modified, added }) })
}

/// 🧭️ The address `node` would carry at `node_path` under a parent whose namespace scope is `parent_scope`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hypothetical_address(part_path: &str, node_path: Vec<usize>, parent_scope: &[(String, String)], node: &XmlNode) -> Result<PptxXmlAddress, String> {
    validate_path(&node_path)?;
    let XmlNode::Element { name, .. } = node else { return Err("PPTX XML address must identify an element".into()) };
    let scope = namespace_scope(parent_scope, node);
    let (namespace_uri, local_name) = expanded_element_name(name, &scope)?;
    Ok(PptxXmlAddress { part_path: part_path.into(), node_path, namespace_uri, local_name, revision: pptx_xml_subtree_revision(node) })
}

/// 🧭️ The container (and its namespace scope) `address` names, together with the slot positions of its children that match `is_slot`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn container_slots<'a>(snapshot: &'a PptxSnapshot, container: &PptxXmlAddress, is_slot: impl Fn(&XmlNode, &[(String, String)]) -> Result<bool, String>) -> Result<(&'a XmlNode, Vec<(String, String)>, Vec<usize>), String> {
    resolve_pptx_xml_address(snapshot, container)?;
    let root = part(snapshot, &container.part_path)?.document.root.as_ref().ok_or_else(|| format!("PPTX XML part {} has no root", container.part_path))?;
    let (node, scope) = scoped_node_at_path(root, &container.node_path)?;
    let mut slots = Vec::new();
    for (index, child) in element_children(node)?.iter().enumerate() {
        if is_slot(child, &namespace_scope(&scope, child))? {
            slots.push(index);
        }
    }
    Ok((node, scope, slots))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_slide_entry(node: &XmlNode, scope: &[(String, String)]) -> Result<bool, String> {
    element_matches(node, scope, PRESENTATIONML_NAMESPACES, "sldId")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_shape_node(node: &XmlNode, scope: &[(String, String)]) -> Result<bool, String> {
    let XmlNode::Element { name, .. } = node else { return Ok(false) };
    let (namespace, local) = expanded_element_name(name, scope)?;
    Ok(PRESENTATIONML_NAMESPACES.contains(&namespace.as_str()) && !matches!(local.as_str(), "nvGrpSpPr" | "grpSpPr"))
}

/// ➕️ Inserts slide `entry` into the vacancy; the inverse removes that exact entry.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_slide_plan(snapshot: &PptxSnapshot, vacancy: &PptxXmlVacancyAddress, entry: &XmlNode) -> Result<PptxPlan, String> {
    let (container, scope, slots) = container_slots(snapshot, &vacancy.container, is_slide_entry)?;
    let physical = slots.get(vacancy.index).copied().unwrap_or(element_children(container)?.len());
    let entry_scope = namespace_scope(&scope, entry);
    let relationship_id = attribute_value(entry, &entry_scope, OFFICE_RELATIONSHIP_NAMESPACES, "id")?.ok_or_else(|| "PPTX slide entry has no relationship id".to_string())?.to_string();
    let slide_id = attribute_value(entry, &entry_scope, &[""], "id")?.unwrap_or_default().to_string();
    let relationship = snapshot.opc.relationships_for(&vacancy.container.part_path).iter().find(|relationship| relationship.id == relationship_id).ok_or_else(|| format!("PPTX slide relationship {relationship_id} does not exist"))?;
    let slide_part_path = resolve_relationship_target(&vacancy.container.part_path, &relationship.target);
    let address = PptxSlideAddress { entry: hypothetical_address(&vacancy.container.part_path, child_path(&vacancy.container.node_path, physical), &scope, entry)?, slide_part_path, relationship_id, slide_id };
    Ok(PptxPlan {
        diff: part_diff(&vacancy.container.part_path, &vacancy.container.node_path, children_leaf(Vec::new(), Vec::new(), vec![XmlChildAdded { index: physical, item: entry.clone() }])),
        inverse: PptxMutation::RemoveSlide(remove_slide::RemoveSlide { address }),
    })
}

/// 🧭️ The container address after child `index` of the node at `container` is removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn container_after_removal(snapshot: &PptxSnapshot, container: &PptxXmlAddress, index: usize) -> Result<PptxXmlAddress, String> {
    let root = part(snapshot, &container.part_path)?.document.root.as_ref().ok_or_else(|| format!("PPTX XML part {} has no root", container.part_path))?;
    let (node, scope) = scoped_node_at_path(root, &container.node_path)?;
    let mut shrunk = node.clone();
    let XmlNode::Element { children, .. } = &mut shrunk else { return Err("PPTX container is not an element".into()) };
    children.remove(index);
    let parent_scope = match container.node_path.split_last() {
        Some((_, parent)) => scoped_node_at_path(root, parent)?.1,
        None => Vec::new(),
    };
    let _ = scope;
    hypothetical_address(&container.part_path, container.node_path.clone(), &parent_scope, &shrunk)
}

/// 🧭️ The address of the container that holds `entry` (the parent of `entry.node_path`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn container_of(snapshot: &PptxSnapshot, entry: &PptxXmlAddress) -> Result<(PptxXmlAddress, usize), String> {
    let (index, parent_path) = entry.node_path.split_last().ok_or_else(|| "PPTX root cannot be addressed as a child".to_string())?;
    Ok((address(snapshot, &entry.part_path, parent_path.to_vec())?, *index))
}

/// ➖️ Removes slide `address`; the inverse re-inserts that exact entry at its ordinal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_slide_plan(snapshot: &PptxSnapshot, address: &PptxSlideAddress) -> Result<PptxPlan, String> {
    if !pptx_slides(snapshot)?.iter().any(|slide| slide.address == *address) {
        return Err("PPTX slide address is stale".into());
    }
    let (container, index) = container_of(snapshot, &address.entry)?;
    let (node, _, slots) = container_slots(snapshot, &container, is_slide_entry)?;
    let ordinal = slots.iter().position(|slot| *slot == index).ok_or_else(|| "PPTX slide entry is not a slide id".to_string())?;
    let removed = element_children(node)?[index].clone();
    Ok(PptxPlan {
        diff: part_diff(&container.part_path, &container.node_path, children_leaf(vec![index], Vec::new(), Vec::new())),
        inverse: PptxMutation::InsertSlide(insert_slide::InsertSlide { vacancy: PptxXmlVacancyAddress { container: container_after_removal(snapshot, &container, index)?, index: ordinal }, entry: removed }),
    })
}

/// 🔀️ Moves slide `address` to `destination_index`: the slide ids between the two slots are rewritten in place, so the sparse diff names only them.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn move_slide_plan(snapshot: &PptxSnapshot, address: &PptxSlideAddress, destination_index: usize) -> Result<PptxPlan, String> {
    let slides = pptx_slides(snapshot)?;
    let from = slides.iter().position(|slide| slide.address == *address).ok_or_else(|| "PPTX slide address is stale".to_string())?;
    if slides.is_empty() || destination_index >= slides.len() {
        return Err("PPTX slide destination is outside the slide list".into());
    }
    let (container, index) = container_of(snapshot, &address.entry)?;
    let (node, _, slots) = container_slots(snapshot, &container, is_slide_entry)?;
    let children = element_children(node)?;
    let mut values: Vec<XmlNode> = slots.iter().map(|slot| children[*slot].clone()).collect();
    let moved = values.remove(from);
    values.insert(destination_index, moved);
    let (low, high) = (from.min(destination_index), from.max(destination_index));
    let modified: Vec<XmlChildModified> = (low..=high).filter(|slot| values[*slot] != children[slots[*slot]]).map(|slot| XmlChildModified { index: slots[slot], diff: XmlNodeDiff::Replace { node: Some(values[slot].clone()) } }).collect();
    let moved_entry = PptxXmlAddress { node_path: child_path(&container.node_path, slots[destination_index]), ..address.entry.clone() };
    let moved_address = PptxSlideAddress { entry: moved_entry, ..address.clone() };
    let diff = if modified.is_empty() { PptxDiff::default() } else { part_diff(&container.part_path, &container.node_path, children_leaf(Vec::new(), modified, Vec::new())) };
    let _ = index;
    Ok(PptxPlan { diff, inverse: PptxMutation::MoveSlide(move_slide::MoveSlide { address: moved_address, destination_index: from }) })
}

/// ➕️ Inserts `shape` into the vacancy; the inverse removes that exact shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_shape_plan(snapshot: &PptxSnapshot, vacancy: &PptxXmlVacancyAddress, shape: &XmlNode) -> Result<PptxPlan, String> {
    let (container, scope, slots) = container_slots(snapshot, &vacancy.container, is_shape_node)?;
    let physical = slots.get(vacancy.index).copied().unwrap_or(element_children(container)?.len());
    let shape_scope = namespace_scope(&scope, shape);
    let shape_id = shape_id(shape, &shape_scope)?.ok_or_else(|| "PPTX inserted shape has no non-visual id, so it could not be addressed or undone".to_string())?;
    let address = PptxShapeAddress { node: hypothetical_address(&vacancy.container.part_path, child_path(&vacancy.container.node_path, physical), &scope, shape)?, shape_id };
    Ok(PptxPlan {
        diff: part_diff(&vacancy.container.part_path, &vacancy.container.node_path, children_leaf(Vec::new(), Vec::new(), vec![XmlChildAdded { index: physical, item: shape.clone() }])),
        inverse: PptxMutation::RemoveShape(remove_shape::RemoveShape { address }),
    })
}

/// ➖️ Removes shape `address`; the inverse re-inserts that exact shape at its ordinal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_shape_plan(snapshot: &PptxSnapshot, address: &PptxShapeAddress) -> Result<PptxPlan, String> {
    pptx_shape(snapshot, address)?;
    let (container, index) = container_of(snapshot, &address.node)?;
    let (node, _, slots) = container_slots(snapshot, &container, is_shape_node)?;
    let ordinal = slots.iter().position(|slot| *slot == index).ok_or_else(|| "PPTX shape is not a presentation shape".to_string())?;
    let removed = element_children(node)?[index].clone();
    Ok(PptxPlan {
        diff: part_diff(&container.part_path, &container.node_path, children_leaf(vec![index], Vec::new(), Vec::new())),
        inverse: PptxMutation::InsertShape(insert_shape::InsertShape { vacancy: PptxXmlVacancyAddress { container: container_after_removal(snapshot, &container, index)?, index: ordinal }, shape: removed }),
    })
}

/// ✍️ Replaces the node at `address` with `replacement`; the inverse replaces it back with the exact previous node. The replacement has to keep
/// the node's expanded name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn replace_node_plan(snapshot: &PptxSnapshot, address: &PptxXmlAddress, replacement: XmlNode) -> Result<PptxPlan, String> {
    let previous = resolve_pptx_xml_address(snapshot, address)?.clone();
    let root = part(snapshot, &address.part_path)?.document.root.as_ref().ok_or_else(|| format!("PPTX XML part {} has no root", address.part_path))?;
    let parent_scope = match address.node_path.split_last() {
        Some((_, parent)) => scoped_node_at_path(root, parent)?.1,
        None => Vec::new(),
    };
    let after = hypothetical_address(&address.part_path, address.node_path.clone(), &parent_scope, &replacement)?;
    if after.namespace_uri != address.namespace_uri || after.local_name != address.local_name {
        return Err("PPTX replacement XML node changes the expanded name".into());
    }
    let diff = if previous == replacement { PptxDiff::default() } else { part_diff(&address.part_path, &address.node_path, XmlNodeDiff::Replace { node: Some(replacement) }) };
    Ok(PptxPlan { diff, inverse: PptxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: after, node: previous }) })
}

/// ✍️ Rewrites the first DrawingML text run of shape `address` to `text` and blanks the rest.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_shape_text_plan(snapshot: &PptxSnapshot, address: &PptxShapeAddress, text: &str) -> Result<PptxPlan, String> {
    if pptx_shape(snapshot, address)?.text.is_none() {
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
    let mut edited = node.clone();
    for (index, path) in paths.iter().enumerate() {
        let target = node_mut_at_path(&mut edited, path).ok_or_else(|| "PPTX text address became stale".to_string())?;
        set_text_node(target, if index == 0 { text } else { "" })?;
    }
    replace_node_plan(snapshot, &address.node, edited)
}

/// 📐️ Writes the offset and extent of shape `address` to `position`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_shape_position_plan(snapshot: &PptxSnapshot, address: &PptxShapeAddress, position: PptxTransform) -> Result<PptxPlan, String> {
    if pptx_shape(snapshot, address)?.position.is_none() {
        return Err("PPTX shape has no editable transform".into());
    }
    let node = resolve_pptx_xml_address(snapshot, &address.node)?;
    let root = part(snapshot, &address.node.part_path)?.document.root.as_ref().ok_or_else(|| "PPTX slide has no root".to_string())?;
    let (_, scope) = scoped_node_at_path(root, &address.node.node_path)?;
    let xfrm_path = find_descendant_path(node, &scope, DRAWINGML_NAMESPACES, "xfrm", &mut Vec::new())?.ok_or_else(|| "PPTX shape has no transform".to_string())?;
    let mut xfrm_scope = scope.clone();
    let mut cursor = node;
    for index in &xfrm_path {
        let XmlNode::Element { children, .. } = cursor else { return Err("PPTX transform path traverses a non-element".into()) };
        cursor = children.get(*index).ok_or_else(|| "PPTX transform path is outside the shape".to_string())?;
        xfrm_scope = namespace_scope(&xfrm_scope, cursor);
    }
    let mut edited = node.clone();
    let xfrm = node_mut_at_path(&mut edited, &xfrm_path).ok_or_else(|| "PPTX transform became stale".to_string())?;
    let XmlNode::Element { children, .. } = xfrm else { return Err("PPTX transform is not an element".into()) };
    let off_index = direct_child_index(children, &xfrm_scope, DRAWINGML_NAMESPACES, "off")?.ok_or_else(|| "PPTX transform has no offset".to_string())?;
    let ext_index = direct_child_index(children, &xfrm_scope, DRAWINGML_NAMESPACES, "ext")?.ok_or_else(|| "PPTX transform has no extent".to_string())?;
    set_attr(&mut children[off_index], "x", position.x.to_string())?;
    set_attr(&mut children[off_index], "y", position.y.to_string())?;
    set_attr(&mut children[ext_index], "cx", position.cx.to_string())?;
    set_attr(&mut children[ext_index], "cy", position.cy.to_string())?;
    replace_node_plan(snapshot, &address.node, edited)
}
//#endregion 🔖️Plans
