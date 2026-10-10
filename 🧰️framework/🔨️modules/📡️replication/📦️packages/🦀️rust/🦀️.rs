//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! The crate's `[lib] name` is `protocol`: every replica, authority and plugin crate speaks the
//! replication contract through that one canonical name.

// 🔕 async_fn_in_trait warns that callers can't assume Send on the returned future; R3 answers this
// structurally — every former dyn seam becomes a concrete enum so Send falls out at the spawn site.
// Never resolve this by adding `+ Send` to a trait method or by making it sync (R7).
#![allow(async_fn_in_trait)]
#![allow(ambiguous_glob_reexports, unused_imports)]

#[path = ""]
pub mod codec {
    #[path = "../../⚙️codec/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../../⚙️codec/🆔️ids/🦀️.rs"]
    pub mod ids;

    pub use self::ids::*;
}

#[path = "../../🚰️source/🦀️.rs"]
pub mod source;

use semio_framework_diagnostic::*;

pub use semio_framework_value as value;
pub use semio_framework_value::dsl_value;

#[cfg(test)]
#[path = "../../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[path = "../../🆔️ids/🦀️.rs"]
pub mod ids;

#[path = "../../🔢️scalar/🦀️.rs"]
pub mod scalar;

#[path = "../../📖️dictionary/🦀️.rs"]
pub mod dictionary;

#[path = "../../🔐️crypto/🦀️.rs"]
pub mod crypto;

#[path = "../../🚪️io/🦀️.rs"]
pub mod io;
pub use io::{DiffBinary, DiffCodec, DiffText, OpBinary, OpText};
pub use io::binary::causal::{decode_document_backbone_envelopes_exact, decode_document_backbone_envelopes_exact_with_limits, decode_envelope, decode_envelopes, decode_frontier, decode_ops_vec, encode_envelope, encode_envelopes, encode_frontier, encode_ops_vec, DocumentBackboneBatchLimitsV1, DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS, DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_SCHEMA_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES};

#[path = "../../🎮️mutation/🦀️.rs"]
pub mod mutation;

#[path = "../../🔗️causal/🦀️.rs"]
pub mod causal;

#[path = "../../⚔️conflict/🦀️.rs"]
pub mod conflict;

#[path = ""]
pub mod wire {

    #[path = "../../📡️wire/🎮️command/📥️ingress/🦀️.rs"]
    pub mod command_ingress;
    #[path = "../../🧾️wire/🦀️.rs"]
    mod codec;
    pub use codec::*;

    #[path = "../../📡️wire/🦀️.rs"]
    mod frames;
    pub use frames::*;

    #[path = "../../📡️wire/🏠️local-interaction/🦀️.rs"]
    pub mod local_interaction;
    pub use local_interaction::*;

    // 🧬️ The facade has always surfaced ids/crypto/dictionary through `wire::`.
    pub use super::crypto::*;
    pub use super::dictionary::*;
    pub use super::ids::*;
}

#[path = "../../📐️format/🦀️.rs"]
pub mod format;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;


#[path = "../../👕️peer-overlay/🦀️.rs"]
pub mod peer_overlay;
pub use peer_overlay::*;

pub use crate::causal::*;
pub use crate::causal::{FrontierComparison as RuntimeFrontierComparison, FrontierSummary as RuntimeFrontierSummary};
pub use crate::codec::*;
pub use crate::conflict::*;
pub use crate::format::*;
pub use crate::mutation::*;
pub use crate::scalar::*;
pub use crate::source::*;
pub use crate::value::*;
pub use crate::wire::*;

#[path = "../../🧬️retirement/🦀️.rs"]
mod retirement_integration;

pub use wire::command_ingress::{
    FixedCommandPage, CommandPageSet, PagedCommand, PagedCommandReader, CommandEnvelope, CommandBatch, CommandEnvelopeSet, RejectedCommandBuild, RejectedCommandBuildRegistry, CommandBatchProgress, CommandBatchDriver, CommandDriverRegistry, CommandPageCursor, CommandIngressStatus, COMMAND_PAGE_MAXIMUM_BYTES, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_BATCH_MAXIMUM_ITEMS, INVOCATION_RESULT_PACK_MAXIMUM_BYTES
};

pub use semio_framework_pack_error::PackRefusal;
