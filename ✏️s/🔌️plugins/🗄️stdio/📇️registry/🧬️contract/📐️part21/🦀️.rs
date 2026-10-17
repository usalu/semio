//! 📐️ The canonical ISO 10303-21 (Part-21, "STEP physical file") codec of the stdio contract: tokenizer, generic instance
//! graph, canonical writer and the one JSON projection every STEP-family contract describes. Shared by every format of the
//! family — step (AP203/214/242), ifc (2x3, 4): IFC is literally STEP syntax with another EXPRESS schema — each building its
//! typed view on this graph. https://www.iso.org/standard/63141.html
//! Escape/matrix logic was pre-verified via a standalone scratch binary (ticket
//! `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`); moved here from step's `io::part21` (ticket
//! `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`, LB2 p16).

#[cfg(test)]
#[global_allocator]
static ORIGINAL_OWNER_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

use semio_framework_value::{edit_through_value, DslValue, FromValue, ToValue, ValueEdit, ValueError};
use semio_framework_value_derive as value_derive;
use std::fmt;
use std::fmt::Write as _;

#[path = "🚦️controlled/🦀️.rs"]
mod controlled;

#[path = "🔢️values/🦀️.rs"]
mod values;
pub use values::*;

#[path = "📄️document-model/🦀️.rs"]
mod document_model;
pub use document_model::*;

#[path = "🔤️lexer-and-parse/🦀️.rs"]
mod lexer_and_parse;
pub use lexer_and_parse::*;

#[path = "✍️writer/🦀️.rs"]
mod writer;
pub use writer::*;

#[path = "🧾️value-and-canonical-codecs/🦀️.rs"]
mod value_and_canonical_codecs;
use value_and_canonical_codecs::*;

//#region 🌲️CanonicalTree
use semio_framework_pack_json::{ArtifactCanonicalJsonNode, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree};

//#endregion 🌲️CanonicalTree

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
