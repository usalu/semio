//! 🥣️ Companion test crate of the raster artifact crates: the full-stack laws that reach from the model through the vocabulary, the codecs and the host. The test files stay at their taxonomy nodes and are mounted here.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../../🧬️schema/🧪️tests/🔒️protection/🦀️.rs"]
mod model_protection_tests;
#[path = "../../🧬️schema/🧪️tests/🔬️boot-document/🦀️.rs"]
mod model_boot_document_tests;
#[path = "../../🧬️schema/🧪️tests/🖼️decoded-image/🦀️.rs"]
mod model_decoded_image_tests;
#[path = "../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod io_binary_snapshot_tests;
#[path = "../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod io_sqlite_snapshot_tests;
#[path = "../../🧬️schema/🔺️diff/🧪️tests/🔬️asset-capacity-vectors/🦀️.rs"]
mod diff_asset_capacity_vectors;
#[path = "../../🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod mutations_tests;
#[path = "../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod binary_tests;
#[path = "../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod io_text_snapshot_tests;
