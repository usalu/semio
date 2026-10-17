//! 🎞️ Protocol semio_hub wire frames: the lane-tagged `ClientFrame`/`ServerFrame` envelopes a
//! browser/native sync client exchanges with the collaboration semio_hub, plus their binary codec. Frozen
//! contract: `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment`
//! §`protocol_wire`.
//!
//! 🎯️ W5: the byte encoding is now a fully hand-rolled binary layout — `lane: u8` followed by
//! `frame tag: u8` (the frame enum's variant declaration order) and its fields in declaration
//! order, with no body-length prefix (one frame per WS message) and no per-field tags. This
//! matches `crate::wire::🔖️WireCodec`'s convention (also used by `crate::causal::🔖️EnvelopeCodec`
//! and `os_dsl::op_rt`). `ArtifactDiff`/`InverseMutation` payloads are opaque `Vec<u8>` (never
//! `serde_json::Value`). `ClientFrame::Presence`/`ServerFrame::Presence` carry opaque presence
//! payload bytes (`peer: Vec<u8>` / `peers: Vec<Vec<u8>>`) — this crate has no dependency on
//! `framework_core` (where the concrete `PresencePeer` type and its binary codec live), so the
//! frame only ever moves the already-encoded blob a caller supplies. `protocol_core` supplies the
//! primitive codec (`write_varint_u64`/`write_str`/`write_bytes`/`write_hash32`/`write_bool` and
//! their `read_*` twins); this crate adds only the option/vec combinators and the frame/nested-enum
//! tag dispatch below.

use semio_framework_replication_wire_core::malformed;
use semio_framework_replication_wire_core::read_opt_bytes;
use semio_framework_replication_wire_core::read_opt_str;
use semio_framework_replication_wire_core::read_vec_bytes;
use semio_framework_replication_wire_core::write_opt_bytes;
use semio_framework_replication_wire_core::write_opt_str;
use semio_framework_replication_wire_core::write_vec_bytes;

//#region 🔖️Lane
/// 🛣️ Which logical channel a wire frame travels on: `Command` for causally-ordered,
/// durable operation batches; `Preview` for ephemeral, best-effort UI-state broadcast.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Command = 0,
    Preview = 1,
}

impl Lane {
    async fn to_byte(self) -> u8 {
        self as u8
    }

    async fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Lane::Command),
            1 => Some(Lane::Preview),
            _ => None,
        }
    }
}
//#endregion 🔖️Lane

//#region 🔖️ClientFrame
/// 📨️ One frame a client sends to the semio_hub.
#[derive(Clone, Debug, PartialEq)]
pub enum ClientFrame {
    SocketHelloV1 { wire_version: u32, protocol_version: u32, schema: String, pack_schema_hash: [u8; 32], resume_token: Option<String>, frontier: Option<semio_framework_replication_causal::FrontierSummary> },
    Commands { batch_id: u64, envelopes: Vec<semio_framework_replication_causal::MutationEnvelope> },
    FrontierAdvertise { frontier: semio_framework_replication_causal::FrontierSummary },
    PreviewPublish { key: String, seq: u64, payload: Vec<u8> },
    Presence { peer: Vec<u8> },
    CreditGrant { n: u32 },
    Bye,
}
//#endregion 🔖️ClientFrame

//#region 🔖️ServerFrame
/// 🚀️ How a `ServerFrame::Welcome` seeds a freshly (re)connected client's local state.
#[derive(Clone, Debug, PartialEq)]
pub enum Bootstrap {
    None,
    Snapshot { pack_hash: [u8; 32], inline: Option<Vec<u8>> },
    Tail,
    ArtifactBootstrap(Box<ArtifactBootstrap>),
}

#[path = "🥾️artifact-bootstrap/🦀️.rs"]
mod artifact_bootstrap;
pub use artifact_bootstrap::*;

#[path = "🛰️server-frame/🦀️.rs"]
mod server_frame;
pub use server_frame::*;

#[path = "🧵️codec-combinators/🦀️.rs"]
mod codec_combinators;
use codec_combinators::*;

#[path = "🔌️frame-codec/🦀️.rs"]
mod frame_codec;
pub use frame_codec::*;

//#endregion 🔖️ServerFrame

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
