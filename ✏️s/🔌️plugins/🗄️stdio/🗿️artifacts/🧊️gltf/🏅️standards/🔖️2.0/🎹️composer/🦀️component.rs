//! 🎹️ GltfComposer (2.0 standard) — aggregates its subsets' composer entries value-level
//! (only ✳️any exists today).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, composer_entry_of};
use crate::artifacts::gltf::standards::v2_0::subsets::any::composer::GltfComposer as GltfRawAnyComposer;

static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES.get_or_init(|| vec![composer_entry_of::<GltfRawAnyComposer>()]).as_slice()
}
