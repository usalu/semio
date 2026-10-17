//! ⬆️ Lower: the typed document lanes → a COS object graph (the inverse of `⬇️lift`). Every
//! document collection becomes indirect objects; every page and form regenerates its
//! `/Resources` from the names its operators use; the catalog, info dictionary and trailer
//! entries are assembled last. Object numbers are allocated densely from `first_number`, so a
//! reconciling write can keep a retained graph's own numbers untouched.

use crate::standards::v1_7::subsets::base::modules::colour::{entry, lower_colour_space, lower_ext_g_state, lower_function, lower_shading, push_opt, raw_stream, stream};
use crate::standards::v1_7::subsets::base::modules::content::{content_references, print_content, FontTable};
use crate::standards::v1_7::subsets::base::modules::fonts::{base_encoding_name, cmap, FontCodec};
use crate::standards::v1_7::subsets::base::io::text_string::{encode_text_string};
use crate::standards::v1_7::subsets::base::modules::images::lower_image;
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::modules::xref::ObjectSink;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{NativePdfArtifactResources,PdfArtifactResourcePort};

#[path = "🎛️options-and-lowering/🦀️.rs"]
mod options_and_lowering;
pub use options_and_lowering::*;

#[path = "🔤️fonts-and-resources/🦀️.rs"]
mod fonts_and_resources;

#[path = "🧭️pages-and-navigation/🦀️.rs"]
mod pages_and_navigation;

#[path = "📚️catalog-and-info/🦀️.rs"]
mod catalog_and_info;
pub use catalog_and_info::*;

#[allow(dead_code)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn _unused(_: &BTreeMap<u8, u8>, _: PdfEngineError, _: &dyn Fn(&PdfFunction)) {
    let _ = lower_function;
}
