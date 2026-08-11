//! 🎹️ JsonComposer (rfc8259 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any (the flat rfc8259 read/write) and ✳️i-json (RFC 7493 I-JSON, ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::json::standards::v_rfc8259::subsets::any::composer::JsonComposer as JsonRawAnyComposer;
use crate::artifacts::json::standards::v_rfc8259::subsets::i_json::composer::JsonIJsonComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<JsonRawAnyComposer>(), composer_entry_of::<JsonIJsonComposer>()]).as_slice()
}
