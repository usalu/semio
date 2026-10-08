//! 🚪️ Mesh IO contracts and their native representation implementations.
use crate::MeshData;
use self::text::{mesh_to_obj,mesh_from_obj};
use self::binary::{mesh_to_glb,mesh_from_glb,mesh_to_stl,mesh_from_stl};

#[path="📝️text/🦀️.rs"]
pub mod text;
#[path="💾️binary/🦀️.rs"]
pub mod binary;

//#region MeshCodec
/// 🔌️ Format-keyed mesh export codec; concrete implementations below are zero-dependency
/// (hand-rolled OBJ/GLB/STL). B-Rep apps additionally get `SolidExporter` (kernel/3d/brep/rs) which
/// wraps the real kernel's STEP/STL/OBJ writers, and reuse `GlbExporter`/`GlbImporter` here via a
/// tessellation bridge so GLB is the same codec everywhere. `format_kind` is the short stdio format
/// kind id (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
pub trait MeshExporter: Send + Sync {
    fn format_kind(&self) -> &'static str;
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String>;
}

/// 🔌️ Format-keyed mesh import codec; see `MeshExporter`.
pub trait MeshImporter: Send + Sync {
    fn format_kind(&self) -> &'static str;
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String>;
}

pub struct ObjExporter;
impl MeshExporter for ObjExporter {
    fn format_kind(&self) -> &'static str {
        "obj"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_obj(mesh, "mesh").into_bytes())
    }
}

pub struct ObjImporter;
impl MeshImporter for ObjImporter {
    fn format_kind(&self) -> &'static str {
        "obj"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        mesh_from_obj(text)
    }
}

pub struct GlbExporter;
impl MeshExporter for GlbExporter {
    fn format_kind(&self) -> &'static str {
        "glb"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_glb(mesh))
    }
}

pub struct GlbImporter;
impl MeshImporter for GlbImporter {
    fn format_kind(&self) -> &'static str {
        "glb"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_glb(bytes)
    }
}

pub struct StlExporter;
impl MeshExporter for StlExporter {
    fn format_kind(&self) -> &'static str {
        "stl"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_stl(mesh))
    }
}

pub struct StlImporter;
impl MeshImporter for StlImporter {
    fn format_kind(&self) -> &'static str {
        "stl"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_stl(bytes)
    }
}
//#endregion MeshCodec

//#region IoError
/// ⚠️ Media IO error shared by ArtifactImport/Export and framework codecs. `Unsupported` carries the
/// unsupported format's kind id string (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
#[derive(Clone, Debug, PartialEq)]
pub enum IoError {
    Format(String),
    Unsupported(String),
    Payload(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Format(m) => write!(f, "format: {m}"),
            Self::Unsupported(fmt) => write!(f, "unsupported: {fmt}"),
            Self::Payload(m) => write!(f, "payload: {m}"),
        }
    }
}

impl std::error::Error for IoError {}
//#endregion IoError


#[cfg(test)]
use crate::*;
#[cfg(test)]
use self::text::*;
#[cfg(test)]
use self::binary::*;
#[cfg(test)]
use semio_framework_pack_json as json;
//#region 🧪️Tests
// 🧪 Relocated verbatim (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
// G2) from `🧰️framework/🔨️modules/🔺️mesh/🦀️.rs`'s own `#[cfg(test)] mod tests` — that
// file's DOC COMMENT already said its mesh content "now dissolved into semio-framework-mesh-
// engine" (i.e. HERE), but its 20 tests exercising exactly this crate's own public functions
// (`mesh_box`/`mesh_from_obj`/`ObjExporter`/etc.) were left behind, orphaned, testing a module
// they no longer lived in. This region is that overdue move, landing them with the functions they
// actually exercise. The other 9 tests in that same old `mod tests` block exercised the unrelated
// DWG codec that file also held; those moved to `semio-s-plugin-stdio`'s `ac1024`/`🚪️io` instead.
#[cfg(test)]
#[path = "../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️GltfOracleDifferential
/// 🔬️ Differential test oracle: decodes the SAME `.glb`/`.gltf` bytes through the third-party
/// `gltf` crate (kept ONLY as a `[dev-dependencies]` reference here — never linked into any
/// production target, per ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
/// ARTIFACTS) and asserts the DECODED STRUCTURE matches this crate's own first-party
/// `mesh_from_glb`. Compares structure, not bytes — glTF exporters are not byte-deterministic.
#[cfg(test)]
#[path = "../🧪️tests/🔬️gltf-oracle-differential/🦀️.rs"]
mod gltf_oracle_differential;
//#endregion 🧪️GltfOracleDifferential

//#region 🧪️MeshDataJsonOracleDifferential
/// 🧪️ Validates `From<MeshData> for pack::json::Value` against `serde_json`, the third-party
/// oracle, rather than against a hand-written expectation: both must produce the same JSON for the
/// same mesh. This is what pins the camelCase renaming and the `skip_serializing_if` sparseness —
/// a first-party impl that silently emitted `face_ids`, or emitted `uvs: []` where serde omits the
/// key, would still compile and would still round-trip through our own reader, but would change the
/// bytes the viewer/editor windows put on the wire.
#[cfg(test)]
#[path = "../🧪️tests/🔬️mesh-data-json-oracle/🦀️.rs"]
mod mesh_data_json_oracle_tests;
//#endregion 🧪️MeshDataJsonOracleDifferential

//#region 🧪️MeshDataFromValueRoundTrip
/// 🔄️ `FromValue` is the literal inverse of `ToValue`/`From<MeshData> for pack::json::Value` above:
/// `FromValue::from_value(ToValue::to_value(&mesh)) == mesh` for a dense mesh (every always-emitted
/// field only) and for one with every sparse field also populated. A differential oracle test proves
/// the SAME JSON this type's `ToValue` produces decodes through serde_json's own `Deserialize` to
/// the identical `MeshData` our first-party `FromValue` decodes from the equivalent `DslValue` tree
/// — not just that our own encode/decode pair agrees with itself.
#[cfg(test)]
#[path = "../🧪️tests/🔬️mesh-data-from-value-round-trip/🦀️.rs"]
mod mesh_data_from_value_round_trip;
//#endregion 🧪️MeshDataFromValueRoundTrip

