//! 💾️ Solid exchange codecs: the solid exporter and importer traits with their STL, OBJ and GLB implementations. See [`super`].

use super::*;

// #region 🔌️Codecs
/// 🔌️ Format-keyed solid export codec.
pub trait SolidExporter: Send + Sync {
    async fn format_kind(&self) -> &'static str;
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
}

/// 🔌️ Format-keyed solid import codec.
pub trait SolidImporter: Send + Sync {
    async fn format_kind(&self) -> &'static str;
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError>;
}

pub struct StlSolidExporter;

pub struct StlSolidImporter;

pub struct ObjSolidExporter;

pub struct ObjSolidImporter;

pub struct GlbSolidExporter;

pub struct GlbSolidImporter;

impl SolidExporter for StlSolidExporter {
    async fn format_kind(&self) -> &'static str {
        "stl"
    }
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_stl_sync(shapes, deflection)
    }
}

impl SolidImporter for StlSolidImporter {
    async fn format_kind(&self) -> &'static str {
        "stl"
    }
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        Ok(vec![kernel.import_stl_sync(bytes, tolerance)?])
    }
}

impl SolidExporter for ObjSolidExporter {
    async fn format_kind(&self) -> &'static str {
        "obj"
    }
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        Ok(kernel.export_obj_sync(shapes, deflection)?.into_bytes())
    }
}

impl SolidImporter for ObjSolidImporter {
    async fn format_kind(&self) -> &'static str {
        "obj"
    }
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|e| BrepError::InvalidInput(e.to_string()))?;
        Ok(vec![kernel.import_obj_sync(text, tolerance)?])
    }
}

impl SolidExporter for GlbSolidExporter {
    async fn format_kind(&self) -> &'static str {
        "glb"
    }
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_glb_sync(shapes, deflection)
    }
}

impl SolidImporter for GlbSolidImporter {
    async fn format_kind(&self) -> &'static str {
        "glb"
    }
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        Ok(vec![kernel.import_glb_sync(bytes, tolerance)?])
    }
}
// #endregion 🔌️Codecs
