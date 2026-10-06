//! 📥️ Actual XML forests and document fields are reconstructed from paid typed relations.
use super::super::*;
use super::rows::{invalid, Groups, Identities};
use semio_framework_os_kernel::sqlite_snapshot::transfer::reserve;
use semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled;
type Result<T> = std::result::Result<T, ValueError>;
const PHASE: SqliteSnapshotPhase = SqliteSnapshotPhase::ReconstructSnapshot;
struct Forest(Vec<Option<XmlNode>>);
impl Drop for Forest {
    fn drop(&mut self) {
        for node in &mut self.0 {
            if let Some(node) = node.take() {
                retire_nodes([node]);
            }
        }
    }
}
fn tick(control: &mut SqliteSnapshotControl<'_>, work: &mut usize, total: usize) -> Result<()> {
    *work = work.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "XML construction work overflow"))?;
    if *work % 256 == 0 {
        control.checkpoint(PHASE, *work, total)?;
    }
    Ok(())
}
fn claim(nodes: &Identities<'_>, owners: &mut [bool], id: i64) -> Result<usize> {
    let index = nodes.index(id)?;
    if owners[index] {
        return Err(invalid("XML node has multiple owners"));
    }
    owners[index] = true;
    Ok(index)
}
fn root(row: &SqliteRow) -> Result<Option<i64>> {
    match row.values.get(2) {
        Some(SqliteValue::Null) => Ok(None),
        Some(SqliteValue::Integer(id)) => Ok(Some(*id)),
        _ => Err(invalid("XML document root must be an integer or NULL")),
    }
}
pub(in super::super) fn reconstruct(database: &SqliteDatabase, ddl: &str, tables: XmlSqliteTables, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<XmlSnapshot>> {
    control.check_database(database, PHASE)?;
    validate_sqlite_database_schema_controlled(database, ddl, PHASE, control)?;
    let mut sets = reserve(13, control)?;
    for (name, width) in tables.names().into_iter().zip([3, 2, 2, 2, 2, 2, 3, 5, 4, 5, 6, 7, 6]) {
        sets.push(Identities::new(database.table(name)?, width, control)?);
    }
    let documents = &sets[0];
    let nodes = &sets[1];
    let mut total = 0usize;
    for table in &database.tables {
        add(&mut total, table.rows.len())?;
    }
    let mut work = 0;
    for (index, row) in documents.rows.iter().enumerate() {
        tick(control, &mut work, total)?;
        if row.rowid != integer(index + 1)? {
            return Err(invalid("XML document identities must be dense"));
        }
        row.text(1)?;
    }
    let attrs = Groups::new(&sets[7], Some(2), None, control)?;
    let children = Groups::new(&sets[8], Some(2), None, control)?;
    let misc = Groups::new(&sets[9], Some(3), Some(2), control)?;
    let declarations = Groups::new(&sets[10], None, None, control)?;
    let doctypes = Groups::new(&sets[11], None, None, control)?;
    let entities = Groups::new(&sets[12], Some(2), None, control)?;
    let mut components = reserve(nodes.rows.len(), control)?;
    let mut owners = reserve(nodes.rows.len(), control)?;
    let mut visited = reserve(nodes.rows.len(), control)?;
    let mut forest = Forest(reserve(nodes.rows.len(), control)?);
    for _ in &nodes.rows {
        components.push(None);
        owners.push(false);
        visited.push(false);
        forest.0.push(None);
        tick(control, &mut work, total)?;
    }
    for (index, row) in nodes.rows.iter().enumerate() {
        tick(control, &mut work, total)?;
        if !matches!(row.text(1)?, "element" | "text" | "cdata" | "comment" | "processing_instruction") {
            return Err(invalid("XML node kind differs"));
        }
        let _ = index;
    }
    for (table, kind) in [(2, "element"), (3, "text"), (4, "cdata"), (5, "comment"), (6, "processing_instruction")] {
        for row in &sets[table].rows {
            tick(control, &mut work, total)?;
            let index = nodes.index(row.rowid)?;
            if nodes.rows[index].text(1)? != kind || components[index].replace(*row).is_some() {
                return Err(invalid("XML component kind or ownership differs"));
            }
            row.text(1)?;
            if table == 6 {
                row.text(2)?;
            }
        }
    }
    if components.iter().any(Option::is_none) {
        return Err(invalid("XML node lacks its typed component"));
    }
    let mut roots = reserve(nodes.rows.len(), control)?;
    for row in &documents.rows {
        if let Some(id) = root(row)? {
            roots.push(claim(nodes, &mut owners, id)?);
        }
        tick(control, &mut work, total)?;
    }
    for row in misc.all() {
        documents.get(row.integer(1)?)?;
        roots.push(claim(nodes, &mut owners, row.integer(4)?)?);
        tick(control, &mut work, total)?;
    }
    for row in children.all() {
        let parent = nodes.get(row.integer(1)?)?;
        if parent.text(1)? != "element" {
            return Err(invalid("XML child requires an element parent"));
        }
        claim(nodes, &mut owners, row.integer(3)?)?;
        tick(control, &mut work, total)?;
    }
    for row in attrs.all() {
        if nodes.get(row.integer(1)?)?.text(1)? != "element" {
            return Err(invalid("XML attribute requires an element parent"));
        }
        row.text(3)?;
        row.text(4)?;
        tick(control, &mut work, total)?;
    }
    if owners.iter().any(|owned| !*owned) {
        return Err(invalid("XML node lacks a document or element owner"));
    }
    for row in declarations.all() {
        documents.get(row.integer(1)?)?;
        tick(control, &mut work, total)?;
    }
    for row in doctypes.all() {
        documents.get(row.integer(1)?)?;
        tick(control, &mut work, total)?;
    }
    for row in entities.all() {
        sets[11].get(row.integer(1)?)?;
        tick(control, &mut work, total)?;
    }
    let capacity = nodes.rows.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "XML construction frontier extent overflow"))?;
    let mut pending = reserve(capacity, control)?;
    for root in roots.into_iter().rev() {
        pending.push((root, false));
    }
    while let Some((index, finish)) = pending.pop() {
        tick(control, &mut work, total)?;
        let id = nodes.rows[index].rowid;
        if !finish {
            if visited[index] {
                return Err(invalid("XML child graph contains a cycle"));
            }
            visited[index] = true;
            pending.push((index, true));
            for row in children.children(id, None, control)?.iter().rev() {
                pending.push((nodes.index(row.integer(3)?)?, false));
            }
            continue;
        }
        let payload = components[index].ok_or_else(|| invalid("XML component disappeared"))?;
        let node = match nodes.rows[index].text(1)? {
            "element" => {
                let rows = attrs.children(id, None, control)?;
                let mut attributes = reserve(rows.len(), control)?;
                for row in rows {
                    attributes.push(XmlAttr { name: restored(row.text(3)?, control)?, value: restored(row.text(4)?, control)? });
                    tick(control, &mut work, total)?;
                }
                let rows = children.children(id, None, control)?;
                let mut children = XmlNodeList(reserve(rows.len(), control)?);
                for row in rows {
                    let index = nodes.index(row.integer(3)?)?;
                    children.0.push(forest.0[index].take().ok_or_else(|| invalid("XML child was not reconstructed"))?);
                    tick(control, &mut work, total)?;
                }
                XmlNode::Element { name: restored(payload.text(1)?, control)?, attrs: attributes, children: std::mem::take(&mut children.0) }
            }
            "text" => XmlNode::Text { text: restored(payload.text(1)?, control)? },
            "cdata" => XmlNode::CData { text: restored(payload.text(1)?, control)? },
            "comment" => XmlNode::Comment { text: restored(payload.text(1)?, control)? },
            "processing_instruction" => XmlNode::ProcessingInstruction { target: restored(payload.text(1)?, control)?, data: restored(payload.text(2)?, control)? },
            _ => return Err(invalid("XML node kind differs")),
        };
        forest.0[index] = Some(node);
    }
    if visited.iter().any(|visited| !*visited) {
        return Err(invalid("XML child graph is disconnected or cyclic"));
    }
    let mut result = XmlSnapshots(reserve(documents.rows.len(), control)?);
    for row in &documents.rows {
        let id = row.rowid;
        let mut owner = XmlDocumentOwner(Some(XmlDocument::default()));
        let doc = owner.0.as_mut().ok_or_else(|| invalid("XML document owner is empty"))?;
        if let Some(root) = root(row)? {
            doc.root = Some(forest.0[nodes.index(root)?].take().ok_or_else(|| invalid("XML root was not reconstructed"))?);
        }
        for (position, target) in [("prolog", &mut doc.prolog), ("epilog", &mut doc.epilog)] {
            let rows = misc.children(id, Some(position), control)?;
            *target = reserve(rows.len(), control)?;
            for row in rows {
                target.push(forest.0[nodes.index(row.integer(4)?)?].take().ok_or_else(|| invalid("XML boundary was not reconstructed"))?);
                tick(control, &mut work, total)?;
            }
        }
        if let Some(row) = declarations.children(id, None, control)?.first() {
            doc.declaration = Some(XmlDeclaration {
                version: restored(row.text(2)?, control)?,
                encoding: restored_optional(row.optional_text(3)?, control)?,
                standalone: if row.values[4] == SqliteValue::Null { None } else { Some(boolean(row, 4)?) },
                quote: match row.text(5)? {
                    "double" => XmlQuote::Double,
                    "single" => XmlQuote::Single,
                    _ => return Err(invalid("XML declaration quote differs")),
                },
            });
        }
        if let Some(row) = doctypes.children(id, None, control)?.first() {
            doc.doctype = Some(XmlDoctype::default());
            let doctype = doc.doctype.as_mut().ok_or_else(|| invalid("XML doctype owner is empty"))?;
            doctype.prolog_position = super::super::super::position::parse(row.text(2)?).map_err(|message| invalid(&message))?;
            doctype.name = restored(row.text(3)?, control)?;
            doctype.external_id = match (row.optional_text(4)?, row.optional_text(5)?, row.optional_text(6)?) {
                (None, None, None) => None,
                (Some("system"), None, Some(system_id)) => Some(XmlExternalId::System { system_id: restored(system_id, control)? }),
                (Some("public"), Some(public_id), Some(system_id)) => Some(XmlExternalId::Public { public_id: restored(public_id, control)?, system_id: restored(system_id, control)? }),
                _ => return Err(invalid("XML external identifier shape differs")),
            };
            let rows = entities.children(row.rowid, None, control)?;
            doctype.declarations = reserve(rows.len(), control)?;
            for row in rows {
                doctype.declarations.push(XmlDtdDeclaration::Entity { parameter: boolean(row, 3)?, name: restored(row.text(4)?, control)?, value: restored(row.text(5)?, control)? });
                tick(control, &mut work, total)?;
            }
        }
        boundaries(XmlDocumentView::from(&*doc), control, PHASE)?;
        let schema = restored(row.text(1)?, control)?;
        result.0.push(XmlSnapshot { schema, doc: owner.0.take().ok_or_else(|| invalid("XML document owner is empty"))? });
    }
    if forest.0.iter().any(Option::is_some) {
        return Err(invalid("XML graph has unclaimed components"));
    }
    control.checkpoint(PHASE, total, total)?;
    Ok(std::mem::take(&mut result.0))
}
