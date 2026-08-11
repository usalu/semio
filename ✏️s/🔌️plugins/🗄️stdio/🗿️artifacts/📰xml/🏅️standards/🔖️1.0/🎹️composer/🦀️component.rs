//! 🎹️ XmlComposer (1.0 standard) — aggregates its subsets' composer entries value-level:
//! ✳️any (the flat 1.0 read/write) and ✳️valid (W3C XML 1.0 §5.1 validity, ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::xml::standards::v1_0::subsets::any::composer::XmlComposer as XmlRawAnyComposer;
use crate::artifacts::xml::standards::v1_0::subsets::valid::composer::XmlValidComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<XmlRawAnyComposer>(), composer_entry_of::<XmlValidComposer>()]).as_slice()
}
