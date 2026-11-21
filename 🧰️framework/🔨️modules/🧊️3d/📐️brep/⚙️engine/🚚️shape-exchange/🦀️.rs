//! 🚚️ Session entry points for portable B-Rep shape values.

use super::{Brep, BrepError, GeometryHandle, ImportedShape, ShapeValue};
use semio_framework_3d_brep_shape_value::registry_exchange;

impl Brep {
    /// 📤️ Captures exactly the handle’s native closure with its original labels and dense arena ids.
    pub fn export_shape(&self, handle: &GeometryHandle) -> Result<ShapeValue, BrepError> {
        registry_exchange::export_shape(&self.native, handle)
    }

    /// 📥️ Imports the shape value and returns its registered root handle.
    pub fn import_shape(&mut self, shape: &ShapeValue) -> Result<GeometryHandle, BrepError> {
        self.import_shape_mapped(shape).map(|imported| imported.handle)
    }

    /// 🏷️ Imports the shape value and reports the shift of its persistent labels.
    pub fn import_shape_mapped(&mut self, shape: &ShapeValue) -> Result<ImportedShape, BrepError> {
        registry_exchange::import_shape_mapped(&mut self.native, shape)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
