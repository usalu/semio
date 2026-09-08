use crate::standards::v1::subsets::any::io as io_root;
use crate::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf reads.
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.png@1.2/*` → `s.remodel.remodeling@1/*` — one PNG becomes a one-frame
/// `MediaKind::ImageSequence` stream over a real durable image asset, the same document shape
/// `📥️import-frames` builds for a photo set, so a single dropped photo is a legal (if degenerate)
/// reconstruction input rather than an unusable blob. `IoFidelity::Lossy`: nothing else in the scene
/// exists yet.
pub struct PngIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for PngIntoRemodeling {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]) => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "png→remodeling: expected a binary png payload".to_string(), diagnostics: Vec::new() });
        };
        let scene = io_root::scene_from_png_bytes(bytes).map_err(|reason| IoError { message: format!("png→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
