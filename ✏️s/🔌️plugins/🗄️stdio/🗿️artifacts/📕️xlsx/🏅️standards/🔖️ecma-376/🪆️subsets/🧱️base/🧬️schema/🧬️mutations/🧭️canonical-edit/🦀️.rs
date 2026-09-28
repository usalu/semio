//! 🧭️ Canonical SpreadsheetML mutation semantics over authoritative XML parts.

use super::*;
use crate::standards::v_ecma_376::subsets::base::io::export::serializers::worksheet_to_xml_with_namespace;
use crate::standards::v_ecma_376::subsets::base::io::{attribute_value, element_matches, expanded_element_name, namespace_scope, REL_TYPE_WORKSHEET, R_NS, R_NS_STRICT, SML_NS, SML_NS_STRICT, WORKSHEET_CONTENT_TYPE};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

const SPREADSHEETML_NAMESPACES: [&str; 2] = [SML_NS, SML_NS_STRICT];
const OFFICE_RELATIONSHIP_NAMESPACES: [&str; 2] = [R_NS, R_NS_STRICT];

fn qualified_like(parent: &str, local: &str) -> String {
    parent.split_once(':').map_or_else(|| local.into(), |(prefix, _)| format!("{prefix}:{local}"))
}

fn text_element(name: String, text: impl Into<String>) -> XmlNode {
    XmlNode::Element { name, attrs: Vec::new(), children: vec![XmlNode::Text { text: text.into() }] }
}

fn set_attr(attrs: &mut Vec<XmlAttr>, name: &str, value: Option<String>) {
    if let Some(index) = attrs.iter().position(|attr| attr.name == name) {
        if let Some(value) = value {
            attrs[index].value = value;
        } else {
            attrs.remove(index);
        }
    } else if let Some(value) = value {
        attrs.push(XmlAttr { name: name.into(), value });
    }
}

fn cell_value_nodes(cell_name: &str, current: &[XmlNode], scope: &[(String, String)], namespace: &str, value: &XlsxCellValue) -> Result<(Option<String>, Vec<XmlNode>), String> {
    let child_name = |local| qualified_like(cell_name, local);
    let mut formula_attrs = None;
    for node in current {
        let child_scope = namespace_scope(scope, node);
        if element_matches(node, &child_scope, &[namespace], "f")? {
            let XmlNode::Element { attrs, .. } = node else { unreachable!() };
            formula_attrs = Some(attrs.clone());
            break;
        }
    }
    let v = |text: String| text_element(child_name("v"), text);
    Ok(match value {
        XlsxCellValue::Number(number) => (None, vec![v(number.to_string())]),
        XlsxCellValue::SharedString(index) => (Some("s".into()), vec![v(index.to_string())]),
        XlsxCellValue::InlineString(text) => (
            Some("inlineStr".into()),
            vec![XmlNode::Element {
                name: child_name("is"),
                attrs: Vec::new(),
                children: vec![XmlNode::Element { name: child_name("t"), attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }], children: vec![XmlNode::Text { text: text.clone() }] }],
            }],
        ),
        XlsxCellValue::Boolean(value) => (Some("b".into()), vec![v(if *value { "1".into() } else { "0".into() })]),
        XlsxCellValue::Error(error) => (Some("e".into()), vec![v(error.clone())]),
        XlsxCellValue::Formula { expr, cached } => {
            let mut children = vec![XmlNode::Element { name: child_name("f"), attrs: formula_attrs.unwrap_or_default(), children: vec![XmlNode::Text { text: expr.clone() }] }];
            let cell_type = match cached.as_deref() {
                Some(XlsxCellValue::Number(number)) => {
                    children.push(v(number.to_string()));
                    None
                }
                Some(XlsxCellValue::SharedString(index)) => {
                    children.push(v(index.to_string()));
                    Some("s".into())
                }
                Some(XlsxCellValue::InlineString(text)) => {
                    children.push(v(text.clone()));
                    Some("str".into())
                }
                Some(XlsxCellValue::Boolean(value)) => {
                    children.push(v(if *value { "1".into() } else { "0".into() }));
                    Some("b".into())
                }
                Some(XlsxCellValue::Error(error)) => {
                    children.push(v(error.clone()));
                    Some("e".into())
                }
                _ => None,
            };
            (cell_type, children)
        }
        XlsxCellValue::Empty => (None, Vec::new()),
    })
}

fn set_addressed_cell(snapshot: &mut XlsxSnapshot, address: &cell_address::XlsxCellAddress, value: &XlsxCellValue) -> Result<(), String> {
    let scope = cell_address::addressed_cell_scope(snapshot, address)?;
    let node = cell_address::addressed_cell_mut(snapshot, address)?;
    let XmlNode::Element { name, attrs, children } = node else { return Err("XLSX cell address resolved a non-element".into()) };
    let (cell_type, replacement) = cell_value_nodes(name, children, &scope, &address.namespace_uri, value)?;
    set_attr(attrs, "t", cell_type);
    let mut value_children = Vec::new();
    for (index, child) in children.iter().enumerate() {
        let child_scope = namespace_scope(&scope, child);
        let mut is_value_child = false;
        for local in ["f", "v", "is"] {
            is_value_child |= element_matches(child, &child_scope, &[address.namespace_uri.as_str()], local)?;
        }
        if is_value_child {
            value_children.push(index);
        }
    }
    let insertion = value_children.first().copied().unwrap_or(children.len());
    for index in value_children.into_iter().rev() {
        children.remove(index);
    }
    let insertion = insertion.min(children.len());
    children.splice(insertion..insertion, replacement);
    Ok(())
}

fn shared_strings_path(snapshot: &XlsxSnapshot) -> Result<String, String> {
    let workbook_path = snapshot
        .opc
        .resolve_relationship("", semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| snapshot.opc.resolve_relationship("", crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_OFFICE_DOCUMENT_STRICT))
        .ok_or_else(|| "missing workbook relationship".to_string())?;
    let relationship = snapshot.opc.relationships_for(&workbook_path).iter().find(|relationship| relationship.rel_type.ends_with("/sharedStrings")).ok_or_else(|| "workbook has no shared strings relationship".to_string())?;
    Ok(semio_s_artifact_stdio_zip::opc::resolve_relationship_target(&workbook_path, &relationship.target))
}

fn edit_shared_strings(snapshot: &mut XlsxSnapshot, edit: impl FnOnce(&mut Vec<XmlNode>, &[(String, String)], &str) -> Result<(), String>) -> Result<(), String> {
    let path = shared_strings_path(snapshot)?;
    let (scope, namespace) = {
        let root = snapshot.xml_part(&path).and_then(|part| part.document.root.as_ref()).ok_or_else(|| format!("missing shared strings part {path}"))?;
        let scope = namespace_scope(&[], root);
        let (namespace, local) = expanded_element_name(
            match root {
                XmlNode::Element { name, .. } => name,
                _ => return Err("shared strings part has no element root".into()),
            },
            &scope,
        )?;
        if local != "sst" || !SPREADSHEETML_NAMESPACES.contains(&namespace.as_str()) {
            return Err("shared strings root is not a SpreadsheetML sst element".into());
        }
        (scope, namespace)
    };
    let document = &mut snapshot.xml_part_mut(&path).ok_or_else(|| format!("missing shared strings part {path}"))?.document;
    let Some(XmlNode::Element { children, attrs, .. }) = document.root.as_mut() else { return Err("shared strings part has no element root".into()) };
    edit(children, &scope, &namespace)?;
    let mut count = 0usize;
    for node in children.iter() {
        let child_scope = namespace_scope(&scope, node);
        if element_matches(node, &child_scope, &[namespace.as_str()], "si")? {
            count += 1;
        }
    }
    let count = count.to_string();
    set_attr(attrs, "count", Some(count.clone()));
    set_attr(attrs, "uniqueCount", Some(count));
    Ok(())
}

fn replace_text_contributions(node: &mut XmlNode, scope: &[(String, String)], namespace: &str, value: &str) -> Result<(), String> {
    fn visit(node: &mut XmlNode, scope: &[(String, String)], namespace: &str, value: &str, replaced: &mut bool) -> Result<(), String> {
        let node_scope = namespace_scope(scope, node);
        let is_text = element_matches(node, &node_scope, &[namespace], "t")?;
        let XmlNode::Element { children, .. } = node else { return Ok(()) };
        if is_text {
            for child in children {
                if let XmlNode::Text { text } = child {
                    if *replaced {
                        text.clear();
                    } else {
                        text.clear();
                        text.push_str(value);
                        *replaced = true;
                    }
                }
            }
        } else {
            for child in children {
                visit(child, &node_scope, namespace, value, replaced)?;
            }
        }
        Ok(())
    }
    let mut replaced = false;
    visit(node, scope, namespace, value, &mut replaced)?;
    if !replaced {
        if let XmlNode::Element { name, children, .. } = node {
            children.push(text_element(qualified_like(name, "t"), value));
        }
    }
    Ok(())
}

fn remove_addressed_cell(snapshot: &mut XlsxSnapshot, address: &cell_address::XlsxCellAddress) -> Result<(), String> {
    let part_index = cell_address::resolve_xlsx_cell_address(snapshot, address)?.part_index;
    let (&index, parent_path) = address.node_path.split_last().ok_or_else(|| "cannot remove worksheet root".to_string())?;
    fn node_mut<'a>(node: &'a mut XmlNode, path: &[usize]) -> Option<&'a mut XmlNode> {
        let Some((&index, rest)) = path.split_first() else { return Some(node) };
        let XmlNode::Element { children, .. } = node else { return None };
        node_mut(children.get_mut(index)?, rest)
    }
    let root = snapshot.xml_parts[part_index].document.root.as_mut().ok_or_else(|| "worksheet has no root".to_string())?;
    let XmlNode::Element { children, .. } = node_mut(root, parent_path).ok_or_else(|| "cell parent path is stale".to_string())? else { return Err("cell parent is not an element".into()) };
    children.remove(index);
    Ok(())
}

fn rename_sheet(snapshot: &mut XlsxSnapshot, old_name: &str, new_name: &str) -> Result<(), String> {
    if snapshot.project_workbook().map_err(|error| error.to_string())?.sheets.iter().any(|sheet| sheet.name == new_name) {
        return Err(format!("worksheet {new_name:?} already exists"));
    }
    let workbook_path = snapshot
        .opc
        .resolve_relationship("", semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| snapshot.opc.resolve_relationship("", crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_OFFICE_DOCUMENT_STRICT))
        .ok_or_else(|| "missing workbook relationship".to_string())?;
    let root = snapshot.xml_part_mut(&workbook_path).and_then(|part| part.document.root.as_mut()).ok_or_else(|| "missing workbook root".to_string())?;
    let root_scope = namespace_scope(&[], root);
    if !element_matches(root, &root_scope, &SPREADSHEETML_NAMESPACES, "workbook")? {
        return Err("workbook root is not a SpreadsheetML workbook element".into());
    }
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    let mut sheet = None;
    for child in children {
        let scope = namespace_scope(&root_scope, child);
        if !element_matches(child, &scope, &SPREADSHEETML_NAMESPACES, "sheets")? {
            continue;
        }
        let XmlNode::Element { children, .. } = child else { unreachable!() };
        for candidate in children {
            let candidate_scope = namespace_scope(&scope, candidate);
            if element_matches(candidate, &candidate_scope, &SPREADSHEETML_NAMESPACES, "sheet")? && attribute_value(candidate, &candidate_scope, &[""], "name")? == Some(old_name) {
                sheet = Some(candidate);
                break;
            }
        }
        break;
    }
    let sheet = sheet.ok_or_else(|| format!("missing worksheet {old_name:?}"))?;
    let XmlNode::Element { attrs, .. } = sheet else { unreachable!() };
    set_attr(attrs, "name", Some(new_name.into()));
    Ok(())
}

fn workbook_path(snapshot: &XlsxSnapshot) -> Result<String, String> {
    snapshot
        .opc
        .resolve_relationship("", semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| snapshot.opc.resolve_relationship("", crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_OFFICE_DOCUMENT_STRICT))
        .ok_or_else(|| "missing workbook relationship".to_string())
}

fn worksheet_membership(snapshot: &XlsxSnapshot, sheet_name: &str) -> Result<(String, usize, usize, String), String> {
    let workbook_path = workbook_path(snapshot)?;
    let workbook = snapshot.xml_part(&workbook_path).ok_or_else(|| format!("missing workbook part {workbook_path}"))?;
    let root = workbook.document.root.as_ref().ok_or_else(|| "missing workbook root".to_string())?;
    let root_scope = namespace_scope(&[], root);
    if !element_matches(root, &root_scope, &SPREADSHEETML_NAMESPACES, "workbook")? {
        return Err("workbook root is not a SpreadsheetML workbook element".into());
    }
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    for (sheets_index, child) in children.iter().enumerate() {
        let scope = namespace_scope(&root_scope, child);
        if !element_matches(child, &scope, &SPREADSHEETML_NAMESPACES, "sheets")? {
            continue;
        }
        let XmlNode::Element { children, .. } = child else { unreachable!() };
        for (sheet_index, sheet) in children.iter().enumerate() {
            let sheet_scope = namespace_scope(&scope, sheet);
            if element_matches(sheet, &sheet_scope, &SPREADSHEETML_NAMESPACES, "sheet")? && attribute_value(sheet, &sheet_scope, &[""], "name")? == Some(sheet_name) {
                let relationship_id = attribute_value(sheet, &sheet_scope, &OFFICE_RELATIONSHIP_NAMESPACES, "id")?.ok_or_else(|| format!("worksheet {sheet_name:?} has no relationship id"))?;
                return Ok((workbook_path, sheets_index, sheet_index, relationship_id.into()));
            }
        }
    }
    Err(format!("missing worksheet {sheet_name:?}"))
}

fn insert_sheet(snapshot: &mut XlsxSnapshot, sheet: &XlsxSheet) -> Result<(), String> {
    let workbook = snapshot.project_workbook().map_err(|error| error.to_string())?;
    if workbook.sheets.iter().any(|candidate| candidate.name == sheet.name) {
        return Err(format!("worksheet {:?} already exists", sheet.name));
    }
    if sheet.name.is_empty() || sheet.name.chars().count() > 31 || sheet.name.chars().any(|character| matches!(character, ':' | '\\' | '/' | '?' | '*' | '[' | ']')) {
        return Err("worksheet name is invalid".into());
    }
    let workbook_path = workbook_path(snapshot)?;
    let (namespace, relationship_namespace, sheet_name, relationship_prefix, next_sheet_id) = {
        let workbook = snapshot.xml_part(&workbook_path).ok_or_else(|| format!("missing workbook part {workbook_path}"))?;
        let root = workbook.document.root.as_ref().ok_or_else(|| "missing workbook root".to_string())?;
        let root_scope = namespace_scope(&[], root);
        let XmlNode::Element { children, .. } = root else { return Err("workbook root is not an element".into()) };
        let (namespace, local) = match root {
            XmlNode::Element { name, .. } => expanded_element_name(name, &root_scope)?,
            _ => unreachable!(),
        };
        if local != "workbook" || !SPREADSHEETML_NAMESPACES.contains(&namespace.as_str()) {
            return Err("workbook root is not a SpreadsheetML workbook element".into());
        }
        let relationship_namespace = snapshot
            .opc
            .relationships_for(&workbook_path)
            .iter()
            .find(|relationship| relationship.rel_type.ends_with("/worksheet"))
            .map(|relationship| if relationship.rel_type.starts_with(R_NS_STRICT) { R_NS_STRICT } else { R_NS })
            .unwrap_or(if namespace == SML_NS_STRICT { R_NS_STRICT } else { R_NS });
        let mut sheets_name = None;
        let mut relationship_prefix = None;
        let mut next_sheet_id = 1u64;
        for child in children {
            let scope = namespace_scope(&root_scope, child);
            if !element_matches(child, &scope, &[namespace.as_str()], "sheets")? {
                continue;
            }
            let XmlNode::Element { name, children, .. } = child else { unreachable!() };
            sheets_name = Some(qualified_like(name, "sheet"));
            for (prefix, uri) in &scope {
                if !prefix.is_empty() && uri == relationship_namespace {
                    relationship_prefix = Some(prefix.clone());
                    break;
                }
            }
            for existing in children {
                let existing_scope = namespace_scope(&scope, existing);
                if element_matches(existing, &existing_scope, &[namespace.as_str()], "sheet")? {
                    if let Some(value) = attribute_value(existing, &existing_scope, &[""], "sheetId")? {
                        next_sheet_id = next_sheet_id.max(value.parse::<u64>().map_err(|_| "worksheet sheetId is not an unsigned integer")?.saturating_add(1));
                    }
                }
            }
            break;
        }
        (
            namespace,
            relationship_namespace,
            sheets_name.ok_or_else(|| "workbook has no SpreadsheetML sheets element".to_string())?,
            relationship_prefix.ok_or_else(|| "workbook has no namespace prefix for worksheet relationship attributes".to_string())?,
            next_sheet_id,
        )
    };
    let directory = workbook_path.rsplit_once('/').map_or("", |(directory, _)| directory);
    let mut ordinal = 1usize;
    let worksheet_path = loop {
        let candidate = if directory.is_empty() { format!("worksheets/sheet{ordinal}.xml") } else { format!("{directory}/worksheets/sheet{ordinal}.xml") };
        if snapshot.xml_part(&candidate).is_none() && snapshot.opc.part(&candidate).is_none() {
            break candidate;
        }
        ordinal += 1;
    };
    let target = worksheet_path.strip_prefix(&format!("{directory}/")).unwrap_or(&worksheet_path).to_string();
    let relationship_type = if relationship_namespace == R_NS_STRICT { format!("{R_NS_STRICT}/worksheet") } else { REL_TYPE_WORKSHEET.into() };
    let relationship_id = snapshot.opc.add_generated_relationship(&workbook_path, &relationship_type, &target);
    snapshot.opc.content_types.set_override(&worksheet_path, WORKSHEET_CONTENT_TYPE);
    snapshot.xml_parts.push(crate::schema::snapshot::XlsxXmlPart { path: worksheet_path, content_type: WORKSHEET_CONTENT_TYPE.into(), document: worksheet_to_xml_with_namespace(sheet, &namespace) });
    let root = snapshot.xml_part_mut(&workbook_path).and_then(|part| part.document.root.as_mut()).ok_or_else(|| "missing workbook root".to_string())?;
    let root_scope = namespace_scope(&[], root);
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    for child in children {
        let scope = namespace_scope(&root_scope, child);
        if element_matches(child, &scope, &[namespace.as_str()], "sheets")? {
            let XmlNode::Element { children, .. } = child else { unreachable!() };
            children.push(XmlNode::Element {
                name: sheet_name,
                attrs: vec![XmlAttr { name: "name".into(), value: sheet.name.clone() }, XmlAttr { name: "sheetId".into(), value: next_sheet_id.to_string() }, XmlAttr { name: format!("{relationship_prefix}:id"), value: relationship_id }],
                children: Vec::new(),
            });
            return Ok(());
        }
    }
    Err("workbook has no SpreadsheetML sheets element".into())
}

fn remove_sheet(snapshot: &mut XlsxSnapshot, sheet_name: &str) -> Result<(), String> {
    if snapshot.project_workbook().map_err(|error| error.to_string())?.sheets.len() <= 1 {
        return Err("a workbook must retain at least one worksheet".into());
    }
    let (workbook_path, sheets_index, sheet_index, relationship_id) = worksheet_membership(snapshot, sheet_name)?;
    let relationship = snapshot.opc.relationships_for(&workbook_path).iter().find(|relationship| relationship.id == relationship_id).cloned().ok_or_else(|| format!("worksheet {sheet_name:?} references an unknown relationship"))?;
    if relationship.target_mode != semio_s_artifact_stdio_zip::opc::OpcTargetMode::Internal {
        return Err("worksheet relationship is external".into());
    }
    let worksheet_path = resolve_relationship_target(&workbook_path, &relationship.target);
    let root = snapshot.xml_part_mut(&workbook_path).and_then(|part| part.document.root.as_mut()).ok_or_else(|| "missing workbook root".to_string())?;
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    let XmlNode::Element { children: sheets, .. } = children.get_mut(sheets_index).ok_or_else(|| "worksheet container address is stale".to_string())? else { return Err("worksheet container is not an element".into()) };
    sheets.remove(sheet_index);
    snapshot.opc.relationships.entry(workbook_path).and_modify(|relationships| relationships.retain(|relationship| relationship.id != relationship_id));
    snapshot.opc.relationships.remove(&worksheet_path);
    snapshot.xml_parts.retain(|part| part.path != worksheet_path);
    let override_path = format!("/{worksheet_path}");
    snapshot.opc.content_types.overrides.retain(|(path, _)| path != &override_path);
    Ok(())
}

pub(super) fn mutate(base: &XlsxSnapshot, mutation: &XlsxMutation) -> Result<XlsxSnapshot, String> {
    if let XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) = mutation {
        snapshot.validate_authority().map_err(|error| error.to_string())?;
        return Ok(snapshot.clone());
    }
    let mut next = base.clone();
    match mutation {
        XlsxMutation::SetSnapshot(_) => unreachable!(),
        XlsxMutation::SetCell(set_cell::SetCell { address, value }) => set_addressed_cell(&mut next, address, value)?,
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }) => remove_addressed_cell(&mut next, address)?,
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => edit_shared_strings(&mut next, |children, scope, namespace| {
            let mut remaining = *index;
            let mut selected = None;
            for (physical, node) in children.iter().enumerate() {
                let child_scope = namespace_scope(scope, node);
                if element_matches(node, &child_scope, &[namespace], "si")? {
                    if remaining == 0 {
                        selected = Some((physical, child_scope));
                        break;
                    }
                    remaining -= 1;
                }
            }
            let (physical, child_scope) = selected.ok_or_else(|| format!("shared string index {index} is outside the table"))?;
            replace_text_contributions(&mut children[physical], &child_scope, namespace, value)?;
            Ok(())
        })?,
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => edit_shared_strings(&mut next, |children, scope, namespace| {
            let mut name = None;
            for node in children.iter() {
                let child_scope = namespace_scope(scope, node);
                if element_matches(node, &child_scope, &[namespace], "si")? {
                    let XmlNode::Element { name: node_name, .. } = node else { unreachable!() };
                    name = Some(node_name.clone());
                    break;
                }
            }
            let name = name.unwrap_or_else(|| scope.iter().find(|(_, uri)| uri == namespace).map_or_else(|| "si".into(), |(prefix, _)| if prefix.is_empty() { "si".into() } else { format!("{prefix}:si") }));
            children.push(XmlNode::Element {
                name: name.clone(),
                attrs: Vec::new(),
                children: vec![XmlNode::Element { name: qualified_like(&name, "t"), attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }], children: vec![XmlNode::Text { text: value.clone() }] }],
            });
            Ok(())
        })?,
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => {
            let workbook = next.project_workbook().map_err(|error| error.to_string())?;
            if workbook.sheets.iter().flat_map(|sheet| &sheet.cells).any(|cell| matches!(&cell.value, XlsxCellValue::SharedString(value) if *value == *index)) {
                return Err(format!("shared string index {index} is still referenced"));
            }
            edit_shared_strings(&mut next, |children, scope, namespace| {
                let mut remaining = *index;
                let mut physical = None;
                for (position, node) in children.iter().enumerate() {
                    let child_scope = namespace_scope(scope, node);
                    if element_matches(node, &child_scope, &[namespace], "si")? {
                        if remaining == 0 {
                            physical = Some(position);
                            break;
                        }
                        remaining -= 1;
                    }
                }
                let physical = physical.ok_or_else(|| format!("shared string index {index} is outside the table"))?;
                children.remove(physical);
                Ok(())
            })?;
        }
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => rename_sheet(&mut next, name, new_name)?,
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => insert_sheet(&mut next, sheet)?,
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => remove_sheet(&mut next, name)?,
    }
    next.validate_authority().map_err(|error| error.to_string())?;
    Ok(next)
}
