//! 🚪️ block5d ← txt — foreign `Deserializer<Block5dSnapshot>` on the framework's `io_mechanism`
//! channel: `store::ArtifactDsl::parse_dsl` on this subset's own `.semio` DSL snapshot text, the
//! exact inverse of the sibling `📤️export` leaf (`IoFidelity::Exact`).
//!
//! 🐛️ Repaired here (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): this file used to be an
//! `Err("txt import not yet implemented")` stub left by a copy-paste of stdio's own json↔txt bridge.

use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf reads.
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 📄️ The `.semio` text preamble every `block.block5d` document opens with — the sniff anchor.
pub const DSL_PREAMBLE: &str = "semio block.block5d.dsl ";

/// 🔤️ Parses `.semio` DSL text into this subset's snapshot — also used by the `🎒️zip` leaf.
pub fn from_dsl_text(text: &str) -> Result<Block5dSnapshot, IoError> {
    <Block5dSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: format!("txt→block5d: {error}"), diagnostics: Vec::new() })
}

/// 🧩️ `s.stdio.txt@utf-8/*` → `s.block.block5d@1/*`.
pub struct TxtIntoBlock5d;

impl Deserializer<Block5dSnapshot> for TxtIntoBlock5d {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.starts_with(DSL_PREAMBLE) => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Block5dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "txt→block5d: expected a text utf-8 payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_dsl_text(text)?))
    }
}
