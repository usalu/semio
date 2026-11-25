//! 🧱️ Lifter of the COS object graph: the lifter state and document lanes, the resource and font lanes, navigation and annotations, the catalog and page mode (ISO 32000-1 §7.7, §7.8.3, §9, §12, §14.3).
//! The resumable document lift above it lives in `semio-s-artifact-stdio-pdf-lift`.

use crate::standards::v1_7::subsets::base::schema::snapshot::*;

use semio_s_artifact_stdio_pdf_page_tree::{PageAttribute, PageTreeLeaf};

#[path = "../🧩️lifter-and-document/🦀️.rs"]
mod lifter_and_document;
pub use lifter_and_document::*;

#[path = "../🪟️resources-and-fonts/🦀️.rs"]
mod resources_and_fonts;
pub use resources_and_fonts::{FontLift, FontStart};

#[path = "../🧭️navigation-and-annotations/🦀️.rs"]
mod navigation_and_annotations;

#[path = "../📚️catalog-and-page-mode/🦀️.rs"]
mod catalog_and_page_mode;
pub use catalog_and_page_mode::*;
