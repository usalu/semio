//! 🧬️ Mutation application of the PPTX oracle: the kind switch that applies one declared mutation to a parsed package. See [`super`].

use super::*;

//#region 🔖️Forward
/// 🦠️ Applies one declared kind to the package. An unrecognised kind, or a target the package does not hold, is an
/// error — never a silent no-op.
pub(super) fn apply(mut pkg: Package, kind: &str, params: &Json) -> Result<Package, String> {
    let address = || params.get("address").ok_or_else(|| format!("{kind} carries no address"));
    match kind {
        "insert-slide" => insert_at_vacancy(&mut pkg, params.get("vacancy").ok_or("insert-slide carries no vacancy")?, wire_node(params.get("entry").ok_or("insert-slide carries no entry")?)?, true)?,
        "insert-shape" => insert_at_vacancy(&mut pkg, params.get("vacancy").ok_or("insert-shape carries no vacancy")?, wire_node(params.get("shape").ok_or("insert-shape carries no shape")?)?, false)?,
        "remove-slide" => remove_addressed(&mut pkg, address()?.get("entry").ok_or("a slide address carries no entry")?, &["p:sldId"])?,
        "remove-shape" => remove_addressed(&mut pkg, address()?.get("node").ok_or("a shape address carries no node")?, &["p:sp", "p:pic", "p:graphicFrame", "p:grpSp", "p:cxnSp"])?,
        "move-slide" => {
            let (part, path) = located(address()?.get("entry").ok_or("a slide address carries no entry")?)?;
            let (index, parent) = path.split_last().ok_or("a slide entry has no parent")?;
            let destination = match params.get("destinationIndex") { Some(Json::Number(value)) => *value as usize, _ => return Err("move-slide carries no destination".to_string()) };
            edit_part(&mut pkg, &part, |root| {
                let children = children_at(root, parent)?;
                let slots = slots(children, true);
                let from = slots.iter().position(|slot| slot == index).ok_or("the addressed slide entry is not in the slide list")?;
                if destination >= slots.len() {
                    return Err(format!("move-slide destination {destination} is outside the {}-slide list", slots.len()));
                }
                let mut entries = slots.iter().map(|slot| children[*slot].clone()).collect::<Vec<_>>();
                let entry = entries.remove(from);
                entries.insert(destination, entry);
                for (slot, entry) in slots.into_iter().zip(entries) {
                    children[slot] = entry;
                }
                Ok(())
            })?;
        }
        "set-shape-text" => {
            let (part, path) = located(address()?.get("node").ok_or("a shape address carries no node")?)?;
            let text = params.str("text");
            edit_part(&mut pkg, &part, |root| {
                let mut first = true;
                write_text(node_at(root, &path)?, &text, &mut first);
                if first { Err("the addressed shape has no DrawingML text node".to_string()) } else { Ok(()) }
            })?;
        }
        "set-shape-position" => {
            let (part, path) = located(address()?.get("node").ok_or("a shape address carries no node")?)?;
            let position = params.get("position").ok_or("set-shape-position carries no position")?;
            edit_part(&mut pkg, &part, |root| {
                let xfrm = first_named(node_at(root, &path)?, "a:xfrm").ok_or("the addressed shape has no transform")?;
                let XNode::Element { children, .. } = xfrm else { return Err("a transform is text".to_string()) };
                let off = children.iter_mut().find(|child| is_named(child, "a:off")).ok_or("the transform has no offset")?;
                set_attr(off, "x", position.str("x"));
                set_attr(off, "y", position.str("y"));
                let ext = children.iter_mut().find(|child| is_named(child, "a:ext")).ok_or("the transform has no extent")?;
                set_attr(ext, "cx", position.str("cx"));
                set_attr(ext, "cy", position.str("cy"));
                Ok(())
            })?;
        }
        "replace-xml-node" => {
            let (part, path) = located(address()?)?;
            let node = wire_node(params.get("node").ok_or("replace-xml-node carries no node")?)?;
            edit_part(&mut pkg, &part, |root| {
                *node_at(root, &path)? = node;
                Ok(())
            })?;
        }
        "set-relationship" => {
            let rels_path = rels_path_for(&params.str("owner"));
            let id = params.str("id");
            let mut attrs = vec![("Id".to_string(), id.clone()), ("Type".to_string(), params.str("relType")), ("Target".to_string(), params.str("target"))];
            if matches!(params.get("external"), Some(Json::Bool(true))) {
                attrs.push(("TargetMode".to_string(), "External".to_string()));
            }
            let row = XNode::Element { name: "Relationship".to_string(), attrs, children: Vec::new() };
            if pkg.parts.contains_key(&rels_path) {
                edit_part(&mut pkg, &rels_path, |root| {
                    let children = children_at(root, &[])?;
                    match children.iter().position(|child| is_named(child, "Relationship") && child.attr("Id") == Some(id.as_str())) {
                        Some(at) => children[at] = row,
                        None => {
                            let physical = insertion_slot(children, "Relationship", optional_index(params));
                            children.insert(physical, row);
                        }
                    }
                    Ok(())
                })?;
            } else {
                let root = XNode::Element { name: "Relationships".to_string(), attrs: vec![("xmlns".to_string(), RELATIONSHIPS_NS.to_string())], children: vec![row] };
                pkg.parts.insert(rels_path.clone(), serialize_document(&root)?);
                pkg.order.push(rels_path);
            }
        }
        "remove-relationship" => {
            let owner = params.str("owner");
            let rels_path = rels_path_for(&owner);
            let id = params.str("id");
            if !pkg.parts.contains_key(&rels_path) {
                return Err(format!("remove-relationship: no relationships are owned by {owner:?}"));
            }
            let mut emptied = false;
            edit_part(&mut pkg, &rels_path, |root| {
                let children = children_at(root, &[])?;
                let at = children.iter().position(|child| is_named(child, "Relationship") && child.attr("Id") == Some(id.as_str())).ok_or_else(|| format!("remove-relationship: {owner:?} owns no relationship {id:?}"))?;
                children.remove(at);
                emptied = !children.iter().any(|child| is_named(child, "Relationship"));
                Ok(())
            })?;
            if emptied {
                pkg.parts.remove(&rels_path);
                pkg.order.retain(|name| *name != rels_path);
            }
        }
        "set-content-type" => {
            let is_override = matches!(params.get("isOverride"), Some(Json::Bool(true)));
            let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
            let name = params.str("name");
            let content_type = params.str("contentType");
            edit_part(&mut pkg, CONTENT_TYPES_PART, |root| {
                let children = children_at(root, &[])?;
                if !is_override && children.iter().any(|child| is_named(child, "Default") && child.attr(key).is_some_and(|existing| existing != name && existing.eq_ignore_ascii_case(&name))) {
                    return Err(format!("set-content-type: default extension {name:?} already exists in another letter case"));
                }
                match children.iter().position(|child| is_named(child, tag) && child.attr(key) == Some(name.as_str())) {
                    Some(at) => set_attr(&mut children[at], "ContentType", content_type),
                    None => {
                        let row = XNode::Element { name: tag.to_string(), attrs: vec![(key.to_string(), name), ("ContentType".to_string(), content_type)], children: Vec::new() };
                        let physical = insertion_slot(children, tag, optional_index(params));
                        children.insert(physical, row);
                    }
                }
                Ok(())
            })?;
        }
        "remove-content-type" => {
            let is_override = matches!(params.get("isOverride"), Some(Json::Bool(true)));
            let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
            let name = params.str("name");
            edit_part(&mut pkg, CONTENT_TYPES_PART, |root| {
                let children = children_at(root, &[])?;
                let at = children.iter().position(|child| is_named(child, tag) && child.attr(key) == Some(name.as_str())).ok_or_else(|| format!("remove-content-type: the content types hold no {tag} {name:?}"))?;
                children.remove(at);
                Ok(())
            })?;
        }
        other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
    Ok(pkg)
}
//#endregion 🔖️Forward
