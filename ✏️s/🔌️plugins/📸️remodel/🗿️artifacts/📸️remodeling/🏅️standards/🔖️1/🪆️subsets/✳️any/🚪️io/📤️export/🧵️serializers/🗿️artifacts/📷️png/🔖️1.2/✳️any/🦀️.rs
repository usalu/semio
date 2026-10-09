use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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
    async fn serialize(from: &RemodelingSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let asset = io_root::remodeling_png_asset(from).map_err(|reason| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→png: {reason}"))))?;
        if asset.mime != "image/png" {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→png: the selected asset is {:?}, not image/png", asset.mime))));
        }
        let bytes = base64_codec::base64_standard_decode(asset.data.as_bytes()).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("remodeling→png: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
