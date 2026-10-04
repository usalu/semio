//! 📤️ Literal XML rows and borrowed traversal frontiers are admitted before creation.
use super::super::*;
use semio_framework_os_kernel::sqlite_snapshot::{
    artifact::{Cell, Projection},
    transfer::reserve,
};
fn push_borrowed<T>(values: &mut Vec<T>, value: T, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    if values.len() == values.capacity() {
        let size = values.capacity().max(1).checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "XML frontier extent overflow"))?;
        let mut next = reserve(size, control)?;
        let total = values.len();
        for (index, value) in values.drain(..).enumerate() {
            next.push(value);
            if index % 256 == 0 {
                control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, index, total)?;
            }
        }
        *values = next;
    }
    values.push(value);
    Ok(())
}
pub(in super::super) fn measure<'a>(documents: impl ExactSizeIterator<Item = (&'a str, XmlDocumentView<'a>)>, control: &mut SqliteSnapshotControl<'_>) -> Result<(usize, usize), ValueError> {
    let (mut rows, mut bytes) = (0, 0);
    let document_count = documents.len();
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, document_count)?;
    for (index, (schema, doc)) in documents.enumerate() {
        add(&mut rows, 1)?;
        add_bytes(&mut bytes, schema.len() + if doc.root.is_some() { 16 } else { 8 })?;
        boundaries(doc, control, SqliteSnapshotPhase::ProjectSnapshot)?;
        if let Some(value) = doc.declaration {
            add(&mut rows, 1)?;
            add_bytes(&mut bytes, 22 + value.version.len() + value.encoding.as_ref().map_or(0, String::len) + usize::from(value.standalone.is_some()) * 8)?;
        }
        if let Some(value) = doc.doctype {
            add(&mut rows, 1)?;
            add_bytes(&mut bytes, 16 + super::super::super::position::digits(value.prolog_position) + value.name.len())?;
            match &value.external_id {
                None => {}
                Some(XmlExternalId::System { system_id }) => add_bytes(&mut bytes, 6 + system_id.len())?,
                Some(XmlExternalId::Public { public_id, system_id }) => add_bytes(&mut bytes, 6 + public_id.len() + system_id.len())?,
            }
            for entity in &value.declarations {
                let XmlDtdDeclaration::Entity { name, value, .. } = entity;
                add(&mut rows, 1)?;
                add_bytes(&mut bytes, 32 + name.len() + value.len())?;
                control.check_rows(rows)?;
                control.check_value_bytes(bytes)?;
            }
        }
        let boundary_count = doc.prolog.len().checked_add(doc.epilog.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML boundary count overflow"))?;
        add(&mut rows, boundary_count)?;
        add_bytes(&mut bytes, boundary_count.checked_mul(38).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "XML boundary bytes overflow"))?)?;
        control.check_rows(
            rows.checked_add(boundary_count.checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML boundary rows overflow"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML boundary rows overflow"))?,
        )?;
        let mut pending = Vec::new();
        if let Some(root) = doc.root {
            push_borrowed(&mut pending, root, control)?;
        }
        for node in doc.prolog.iter().chain(doc.epilog) {
            push_borrowed(&mut pending, node, control)?;
        }
        let mut visited = 0;
        while let Some(node) = pending.pop() {
            add(&mut rows, 2)?;
            add_bytes(&mut bytes, 16 + kind(node).len())?;
            match node {
                XmlNode::Element { name, attrs, children } => {
                    add_bytes(&mut bytes, name.len())?;
                    add(&mut rows, attrs.len())?;
                    add(&mut rows, children.len())?;
                    control.check_rows(
                        rows.checked_add(children.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML child count overflow"))?)
                            .ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML child count overflow"))?,
                    )?;
                    add_bytes(&mut bytes, children.len().checked_mul(32).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "XML child bytes overflow"))?)?;
                    for attr in attrs {
                        add_bytes(&mut bytes, 24 + attr.name.len() + attr.value.len())?;
                        control.check_value_bytes(bytes)?;
                        tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut visited, 0)?;
                    }
                    for child in children.iter().rev() {
                        push_borrowed(&mut pending, child, control)?;
                    }
                }
                XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => add_bytes(&mut bytes, text.len())?,
                XmlNode::ProcessingInstruction { target, data } => {
                    add_bytes(&mut bytes, target.len())?;
                    add_bytes(&mut bytes, data.len())?;
                }
            }
            control.check_rows(rows)?;
            control.check_value_bytes(bytes)?;
            tick(control, SqliteSnapshotPhase::ProjectSnapshot, &mut visited, 0)?;
        }
        if (index + 1) % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, index + 1, document_count)?;
        }
    }
    control.check_rows(rows)?;
    control.check_value_bytes(bytes)?;
    Ok((rows, bytes))
}
fn optional(value: Option<&str>) -> Cell<'_> {
    value.map_or(Cell::Null, Cell::Text)
}
fn decimal(value: u64, buffer: &mut [u8; 20]) -> &str {
    let mut start = 20;
    let mut value = value;
    loop {
        start -= 1;
        buffer[start] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    std::str::from_utf8(&buffer[start..]).expect("decimal digits are ASCII")
}
pub(in super::super) fn append<'a>(documents: impl ExactSizeIterator<Item = (&'a str, XmlDocumentView<'a>)> + Clone, tables: XmlSqliteTables, output: &mut Projection<'_, '_>) -> Result<(), ValueError> {
    let mut node_id = 0i64;
    let document_count=documents.len();
    output.checkpoint_work(0,document_count)?;
    for (index,(schema, doc)) in documents.enumerate() {
        let root = node_id.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML node identity overflow"))?;
        let document = output.insert(tables.document, &[Cell::Text(schema), if doc.root.is_some() { Cell::Integer(root) } else { Cell::Null }])?;
        if let Some(value) = doc.declaration {
            output.insert(
                tables.declaration,
                &[
                    Cell::Integer(document),
                    Cell::Text(&value.version),
                    optional(value.encoding.as_deref()),
                    value.standalone.map_or(Cell::Null, |value| Cell::Integer(i64::from(value))),
                    Cell::Text(if value.quote.is_double() { "double" } else { "single" }),
                ],
            )?;
        }
        if let Some(value) = doc.doctype {
            let (external, public, system) = match &value.external_id {
                None => (None, None, None),
                Some(XmlExternalId::System { system_id }) => (Some("system"), None, Some(system_id.as_str())),
                Some(XmlExternalId::Public { public_id, system_id }) => (Some("public"), Some(public_id.as_str()), Some(system_id.as_str())),
            };
            let mut position = [0; 20];
            let id = output.insert(tables.doctype, &[Cell::Integer(document), Cell::Text(decimal(value.prolog_position, &mut position)), Cell::Text(&value.name), optional(external), optional(public), optional(system)])?;
            for (ordinal, value) in value.declarations.iter().enumerate() {
                let XmlDtdDeclaration::Entity { parameter, name, value } = value;
                output.insert(tables.entity, &[Cell::Integer(id), Cell::Integer(integer(ordinal)?), Cell::Integer(i64::from(*parameter)), Cell::Text(name), Cell::Text(value)])?;
            }
        }
        let mut pending = output.allocate_frontier(0)?;
        for (ordinal, node) in doc.epilog.iter().enumerate().rev() {
            output.push_frontier(&mut pending, (node, Parent::Misc("epilog", ordinal)))?;
        }
        for (ordinal, node) in doc.prolog.iter().enumerate().rev() {
            output.push_frontier(&mut pending, (node, Parent::Misc("prolog", ordinal)))?;
        }
        if let Some(root) = doc.root {
            output.push_frontier(&mut pending, (root, Parent::Root))?;
        }
        while let Some((node, parent)) = pending.pop() {
            node_id = output.insert(tables.node, &[Cell::Text(kind(node))])?;
            match node {
                XmlNode::Element { name, attrs, children } => {
                    output.insert_key(tables.element, node_id, &[Cell::Text(name)])?;
                    for (ordinal, attr) in attrs.iter().enumerate() {
                        output.insert(tables.attribute, &[Cell::Integer(node_id), Cell::Integer(integer(ordinal)?), Cell::Text(&attr.name), Cell::Text(&attr.value)])?;
                    }
                    for (ordinal, child) in children.iter().enumerate().rev() {
                        output.push_frontier(&mut pending, (child, Parent::Child(node_id, ordinal)))?;
                    }
                }
                XmlNode::Text { text } => output.insert_key(tables.text, node_id, &[Cell::Text(text)])?,
                XmlNode::CData { text } => output.insert_key(tables.cdata, node_id, &[Cell::Text(text)])?,
                XmlNode::Comment { text } => output.insert_key(tables.comment, node_id, &[Cell::Text(text)])?,
                XmlNode::ProcessingInstruction { target, data } => output.insert_key(tables.processing_instruction, node_id, &[Cell::Text(target), Cell::Text(data)])?,
            }
            match parent {
                Parent::Root => {}
                Parent::Misc(position, ordinal) => {
                    output.insert(tables.document_misc, &[Cell::Integer(document), Cell::Text(position), Cell::Integer(integer(ordinal)?), Cell::Integer(node_id)])?;
                }
                Parent::Child(parent, ordinal) => {
                    output.insert(tables.child, &[Cell::Integer(parent), Cell::Integer(integer(ordinal)?), Cell::Integer(node_id)])?;
                }
            }
            output.checkpoint()?;
        }
        output.checkpoint()?;
        if (index+1)%256==0||index+1==document_count{output.checkpoint_work(index+1,document_count)?;}
    }
    Ok(())
}
pub(in super::super) fn project<'a>(documents: impl ExactSizeIterator<Item = (&'a str, XmlDocumentView<'a>)> + Clone, ddl: &str, tables: XmlSqliteTables, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    let mut output = Projection::new(ddl, control)?;
    append(documents, tables, &mut output)?;
    output.finish()
}
