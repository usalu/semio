//! 📤️ Media export: GLB export and import bridged through tessellation.

use super::*;

/// 🗑️ Disposes a geometry handle owned by the in-process brep kernel.
// #region 🔖️MediaExport
/// 📤️ Exports geometry handles owned by the in-process brep kernel to a solid/mesh interchange format. STEP/OBJ/STL go through the kernel's native codecs; GLB bridges through tessellation (`tessellate` → merged `MeshData` → `GlbExporter`) since the kernel has no native GLB writer wired here. Binary formats are base64-encoded. Returns `{"data","binary","format"}` or `{"error"}` JSON.

/// 🧊️ Bridges GLB export through mesh tessellation: tessellates every shape, merges the resulting triangle soup into one `MeshData`, and encodes it with the shared `GlbExporter` mesh codec (the same codec every other app uses for GLB).
pub fn export_glb_via_tessellation(kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepModuleError> {
    use semio_framework::mesh_io::MeshExporter;
    let mut merged = semio_framework::MeshData::default();
    for shape in shapes {
        let transfer = kernel.tessellate(shape, deflection)?;
        let mesh = semio_framework_3d_brep_engine::mesh_data_from_mesh_transfer(&transfer)?;
        let offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(mesh.positions);
        merged.normals.extend(mesh.normals);
        merged.indices.extend(mesh.indices.into_iter().map(|index| index + offset));
    }
    semio_framework::mesh_io::GlbExporter.export(&merged).map_err(BrepModuleError::Mesh)
}

/// 📥️ Imports STEP/OBJ/STL solid data (or GLB mesh data, bridged through the kernel's OBJ ingestion since it has no raw-mesh entry point) into the in-process kernel. STEP/OBJ expect UTF-8 text in `data`; STL/GLB expect base64-encoded bytes. Returns `{"handles":[...]}` or `{"error"}` JSON.

/// 🧊️ Bridges GLB import through the mesh codec: decodes GLB bytes to `MeshData` via `GlbImporter`, re-encodes it as OBJ text, and ingests that through the kernel's own OBJ importer.
pub fn import_glb_via_tessellation(kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<String>, BrepModuleError> {
    use semio_framework::mesh_io::MeshImporter;
    let mesh = semio_framework::mesh_io::GlbImporter.import(bytes).map_err(BrepModuleError::Mesh)?;
    let obj_text = semio_framework::mesh_io::text::mesh_to_obj(&mesh, "glb-import");
    kernel.import_obj(&obj_text, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)
}
// #endregion 🔖️MediaExport
