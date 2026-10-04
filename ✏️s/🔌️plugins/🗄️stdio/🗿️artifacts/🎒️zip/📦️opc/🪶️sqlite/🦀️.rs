//! 📦️ Literal OPC package entities for individually authored container schemas.
use super::{OpcPackage, OpcTargetMode};
use semio_framework_os_kernel::sqlite_snapshot::{artifact::Projection, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase};

#[path = "💰️backing/🦀️.rs"]
mod backing;

/// 🗂️ Six explicitly declared OPC entity tables.
#[derive(Clone, Copy)]
pub struct OpcSqliteTables {
    pub package: &'static str,
    pub part: &'static str,
    pub default_type: &'static str,
    pub override_type: &'static str,
    pub relationship_owner: &'static str,
    pub relationship: &'static str,
}

fn add(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "OPC relational size overflow"))?;
    Ok(())
}
fn add_bytes(total: &mut usize, value: usize) -> Result<(), ValueError> {
    *total = total.checked_add(value).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "OPC relational size overflow"))?;
    Ok(())
}
fn mode(value: OpcTargetMode) -> &'static str {
    match value {
        OpcTargetMode::Internal => "internal",
        OpcTargetMode::External => "external",
    }
}

/// 📏️ Measures the actual typed OPC rows and scalar bytes before ownership.
pub fn measure_opc_package(package: &OpcPackage, control: &mut SqliteSnapshotControl<'_>) -> Result<(usize, usize), ValueError> {
    let (mut rows, mut bytes) = (1usize, 8usize);
    add_bytes(&mut bytes, package.comment.len())?;
    control.check_rows(rows)?;
    control.check_value_bytes(bytes)?;
    for part in &package.parts {
        add(&mut rows, 1)?;
        add_bytes(&mut bytes, 24)?;
        add_bytes(&mut bytes, part.path.len())?;
        add_bytes(&mut bytes, part.content_type.len())?;
        add_bytes(&mut bytes, part.bytes.len())?;
        control.check_rows(rows)?;
        control.check_value_bytes(bytes)?;
        if rows % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, rows, 0)?;
        }
    }
    for pair in package.content_types.defaults.iter().chain(&package.content_types.overrides) {
        add(&mut rows, 1)?;
        add_bytes(&mut bytes, 24)?;
        add_bytes(&mut bytes, pair.0.len())?;
        add_bytes(&mut bytes, pair.1.len())?;
        control.check_rows(rows)?;
        control.check_value_bytes(bytes)?;
        if rows % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, rows, 0)?;
        }
    }
    for (owner, relationships) in package.relationships.groups() {
        add(&mut rows, 1)?;
        add_bytes(&mut bytes, 16)?;
        add_bytes(&mut bytes, owner.len())?;
        control.check_rows(rows)?;
        control.check_value_bytes(bytes)?;
        if rows % 256 == 0 {
            control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, rows, 0)?;
        }
        for relationship in relationships {
            add(&mut rows, 1)?;
            add_bytes(&mut bytes, 24)?;
            for size in [relationship.id.len(), relationship.rel_type.len(), relationship.target.len(), mode(relationship.target_mode).len()] {
                add_bytes(&mut bytes, size)?;
            }
            control.check_rows(rows)?;
            control.check_value_bytes(bytes)?;
            if rows % 256 == 0 {
                control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, rows, 0)?;
            }
        }
    }
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, rows, rows)?;
    Ok((rows, bytes))
}

/// 🏗️ Appends one typed package into an enclosing owner's single controlled projection.
pub fn append_opc_package(package: &OpcPackage, tables: OpcSqliteTables, output: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    backing::project(package, tables, output)
}

/// 🧱️ Reconstructs exactly the typed package without ordinary archive normalization.
pub fn reconstruct_opc_package(database: &SqliteDatabase, tables: OpcSqliteTables, control: &mut SqliteSnapshotControl<'_>) -> Result<OpcPackage, ValueError> {
    backing::reconstruct(database, tables, control)
}

use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};
