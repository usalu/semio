//! 🔎️ Nodal components use exact declared slots under the original native receiving loan.
use super::*;
use crate::catalog::query::{IfcAttributeQueryStatus, with_ifc4x3_native_attribute};
#[derive(Clone, Copy)]
pub(super) enum Component {
    ForceX,
    ForceY,
    ForceZ,
    MomentX,
    MomentY,
    MomentZ,
}
impl Component {
    pub(super) const ALL: [Self; 6] = [Self::ForceX, Self::ForceY, Self::ForceZ, Self::MomentX, Self::MomentY, Self::MomentZ];
    pub(super) fn name(self) -> &'static str {
        match self { Self::ForceX => "ForceX", Self::ForceY => "ForceY", Self::ForceZ => "ForceZ", Self::MomentX => "MomentX", Self::MomentY => "MomentY", Self::MomentZ => "MomentZ" }
    }
    pub(super) fn unit(self) -> LoadUnit {
        match self { Self::ForceX | Self::ForceY | Self::ForceZ => LoadUnit::Force, Self::MomentX | Self::MomentY | Self::MomentZ => LoadUnit::Torque }
    }
    pub(super) fn dof(self) -> FemDof {
        match self { Self::ForceX => FemDof::Tx, Self::ForceY => FemDof::Ty, Self::ForceZ => FemDof::Tz, Self::MomentX => FemDof::Rx, Self::MomentY => FemDof::Ry, Self::MomentZ => FemDof::Rz }
    }
}
pub(super) fn receive(source: &IfcSnapshot, row: usize, component: Component, epoch: u64, owner: &mut Owner<'_, '_>) -> Result<Option<f64>, ValueError> {
    with_attribute(source,row,component.name(),"IFCSTRUCTURALLOADSINGLEFORCE",epoch,owner,|value,original| {
        work(original, size_of::<Option<f64>>())?;
        match value {
            Some(IfcValue::Unset) => Ok(None),
            Some(value) => number(Some(value)).map(Some),
            None => Err(invalid("nodal component has no serialized source value")),
        }
    })
}
pub(super) fn with_attribute<O:RetireOwned+'static>(source:&IfcSnapshot,row:usize,name:&'static str,declared:&'static str,epoch:u64,owner:&mut Owner<'_,'_>,visit:impl for<'v> FnOnce(Option<&'v IfcValue>,&mut Owner<'_,'_>)->Result<O,ValueError>)->Result<O,ValueError>{
    with_ifc4x3_native_attribute(source, row, name, Some(declared), epoch, owner, |found, original| {
        work(original, size_of::<Option<&IfcValue>>())?;
        match found.status {
            IfcAttributeQueryStatus::Unknown => return Err(unsupported("analytical attribute has unknown source constituents")),
            IfcAttributeQueryStatus::Ambiguous => return Err(invalid("analytical attribute declaration is ambiguous")),
            IfcAttributeQueryStatus::Missing => return Err(invalid("analytical attribute declaration is absent")),
            IfcAttributeQueryStatus::Unique => {}
        }
        let entry = found.matches[0].as_ref().ok_or_else(|| invalid("unique analytical attribute lacks its declaration"))?;
        visit(entry.value,original)
    })
}
#[cfg(test)]
#[path = "fem/nodal/native-query-tests.rs"]
mod tests;
