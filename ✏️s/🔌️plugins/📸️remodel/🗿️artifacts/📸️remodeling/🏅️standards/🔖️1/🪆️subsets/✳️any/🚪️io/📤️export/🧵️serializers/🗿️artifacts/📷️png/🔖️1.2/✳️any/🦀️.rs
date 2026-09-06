use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf writes.
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.png@1.2/*` — the reconstruction's raster preview: the DSM,
/// else the orthophoto, else the DTM, else the mesh's baked texture, read back as real composed
/// `s.stdio.semio/v1/image` content through `io_root::remodeling_png_asset`. A scene with no raster
/// and no texture returns a typed `Err` rather than a blank canvas. `IoFidelity::Lossy`: one raster is
/// not the scene.
pub struct RemodelingIntoPng;

impl Serializer<RemodelingSnapshot> for RemodelingIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let asset = io_root::remodeling_png_asset(from).map_err(|reason| IoError { message: format!("remodeling→png: {reason}"), diagnostics: Vec::new() })?;
        if asset.mime != "image/png" {
            return Err(IoError { message: format!("remodeling→png: the selected asset is {:?}, not image/png", asset.mime), diagnostics: Vec::new() });
        }
        let bytes = base64_codec::base64_standard_decode(asset.data.as_bytes()).map_err(|error| IoError { message: format!("remodeling→png: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
