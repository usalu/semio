//! 📤️ Direct OPC package fields append into one enclosing admitted relational projection.
use super::super::{mode, OpcPackage, OpcSqliteTables};
use semio_framework_os_kernel::sqlite_snapshot::artifact::{Cell, Projection};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn ordinal(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "OPC ordinal exceeds SQLite integer width"))
}
pub(in super::super) fn project(package: &OpcPackage, tables: OpcSqliteTables, projection: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    let id = projection.insert(tables.package, &[Cell::Text(&package.comment)])?;
    projection.checkpoint()?;
    for (index, part) in package.parts.iter().enumerate() {
        projection.insert(tables.part, &[Cell::Integer(id), Cell::Integer(ordinal(index)?), Cell::Text(&part.path), Cell::Text(&part.content_type), Cell::Blob(&part.bytes)])?;
        projection.checkpoint()?;
    }
    for (table, pairs) in [(tables.default_type, &package.content_types.defaults), (tables.override_type, &package.content_types.overrides)] {
        for (index, (name, value)) in pairs.iter().enumerate() {
            projection.insert(table, &[Cell::Integer(id), Cell::Integer(ordinal(index)?), Cell::Text(name), Cell::Text(value)])?;
            projection.checkpoint()?;
        }
    }
    for (owner, relationships) in package.relationships.groups() {
        let parent = projection.insert(tables.relationship_owner, &[Cell::Integer(id), Cell::Text(owner)])?;
        projection.checkpoint()?;
        for (index, relationship) in relationships.iter().enumerate() {
            projection
                .insert(tables.relationship, &[Cell::Integer(parent), Cell::Integer(ordinal(index)?), Cell::Text(&relationship.id), Cell::Text(&relationship.rel_type), Cell::Text(&relationship.target), Cell::Text(mode(relationship.target_mode))])?;
            projection.checkpoint()?;
        }
    }
    Ok(id)
}
