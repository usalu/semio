//! 📝️ Owned baseline conformance projection.
use crate::TiffSnapshot;
pub fn encode_tiff_baseline_projection_json(snapshot:&TiffSnapshot)->String{semio_framework_pack_json::to_json_string(snapshot)}
