//! 🔖️ The viewer's typed command channel: the one `setCamera` row and its binary op codec.

use crate::modes::view::windows::model as model_window;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use super::ENERGY_MODEL_VIEW_TOOL_IDS;

//#region 🔖️Command
/// 👁️ The viewer's typed command channel. It has exactly one row, and that row is the reason the
/// channel is no longer inert: the react `World3dHost` dispatches `setCamera` after every orbit of
/// ANY World3d window, read-only or not, and a window kind that does not declare and reduce it drops
/// every gesture (`dropped action "setCamera" … no window kind declares it`).
///
/// 🔒️ Row order is the binary variant ordinal: appending is safe, reordering is a wire break.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum EnergyModelViewCommand {
    /// 🎥️ The orbit pose as the canonical `{position,target,zoom,up?}` JSON the host sends. One text
    /// field because a `dsl::DslOps` variant binds scalars only — and because that string IS what
    /// `World3dScene::camera_json` consumes.
    #[dsl(key = "setCamera")]
    SetCamera { camera: String },
}

impl Default for EnergyModelViewCommand {
    /// 🌱️ An empty pose, which `camera_emit` refuses explicitly rather than applying — there is no
    /// "do nothing" verb to default to any more, and inventing a pose here would let a malformed
    /// dispatch write a camera nobody asked for.
    fn default() -> Self {
        Self::SetCamera { camera: String::new() }
    }
}

impl EnergyModelViewCommand {
    /// 🪪️ The manifest action id this command stands for.
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::SetCamera { .. } => model_window::SET_CAMERA_ACTION_ID,
        }
    }
}

//#region 🔖️OpCodec
impl protocol::OpBinary for EnergyModelViewCommand {
    /// 🧵️ The exact manifest ids this typed command schema owns — the left-hand side of
    /// `validate_tool_job_rows`' set equality against the `bounded_first_step_tool_proofs!` rows.
    const TOOL_JOB_IDS: &'static [&'static str] = ENERGY_MODEL_VIEW_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
//#endregion 🔖️Command
