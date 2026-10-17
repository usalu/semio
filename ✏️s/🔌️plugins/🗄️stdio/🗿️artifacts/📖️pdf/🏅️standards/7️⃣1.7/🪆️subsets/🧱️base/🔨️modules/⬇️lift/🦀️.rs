//! ⬇️ Lift: the COS object graph → the typed document lanes (ISO 32000-1 §7.7 document
//! structure, §7.7.3 page tree with inheritance, §7.8.3 resources, §9 fonts, §8.9–8.10
//! XObjects, §8.4.5 graphics states, §8.7 shadings and patterns, §12 interactive features,
//! §14.3 metadata). Resource names are canonicalized into document-level ids: the first name a
//! resource object is bound under becomes its id, and every content stream that binds the same
//! object under another name is rewritten to the id — so `lift ∘ lower` is the identity on the
//! typed lanes and content operators can name resources directly.

use crate::standards::v1_7::subsets::base::modules::colour::{array_n, extra_entries, lift_colour_space, lift_ext_g_state, lift_function, lift_shading, matrix_of, numbers_of, rect_of};
use crate::standards::v1_7::subsets::base::modules::content::{parse_content, FontTable};
use crate::standards::v1_7::subsets::base::modules::fonts::{base_encoding_from_name, cmap, FontCodec};
use crate::standards::v1_7::subsets::base::io::text_string::{decode_text_string};
use crate::standards::v1_7::subsets::base::modules::images::lift_image;
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_f64, dict_get, dict_i64, dict_name};
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::{HashMap, HashSet};
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{NativePdfArtifactResources,PdfArtifactResourcePort};
use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfGraphPath, PdfStreamRoleValue};
use crate::standards::v1_7::subsets::base::schema::content_mapping::{ResourceMap, rename_content};

#[path = "🧩️lifter-and-document/🦀️.rs"]
mod lifter_and_document;
pub use lifter_and_document::*;

#[path = "🪟️resources-and-fonts/🦀️.rs"]
mod resources_and_fonts;
use resources_and_fonts::*;

#[path = "🧭️navigation-and-annotations/🦀️.rs"]
mod navigation_and_annotations;
use navigation_and_annotations::*;

#[path = "📚️catalog-and-page-mode/🦀️.rs"]
mod catalog_and_page_mode;
pub use catalog_and_page_mode::*;

#[allow(unused_imports)]
use lift_function as _lift_function_is_used_by_colour;
