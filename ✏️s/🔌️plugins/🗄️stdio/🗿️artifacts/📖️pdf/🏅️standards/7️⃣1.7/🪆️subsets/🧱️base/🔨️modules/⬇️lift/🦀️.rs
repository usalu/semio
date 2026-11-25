//! ⬇️ Lift: the COS object graph → the typed document lanes (ISO 32000-1 §7.7 document
//! structure, §7.7.3 page tree with inheritance, §7.8.3 resources, §9 fonts, §8.9–8.10
//! XObjects, §8.4.5 graphics states, §8.7 shadings and patterns, §12 interactive features,
//! §14.3 metadata). Resource names are canonicalized into document-level ids: the first name a
//! resource object is bound under becomes its id, and every content stream that binds the same
//! object under another name is rewritten to the id — so `lift ∘ lower` is the identity on the
//! typed lanes and content operators can name resources directly.

use crate::standards::v1_7::subsets::base::io::error::PdfEngineError;
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_name, CosGrant};
use crate::standards::v1_7::subsets::base::schema::{graph_source::ObjectSource, snapshot::*, stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfGraphPath, PdfStreamRoleValue}};

pub use semio_s_artifact_stdio_pdf_page_tree::{PageAttribute, PageInheritance, PageTreeCursor, PageTreeEvent, PageTreeLeaf, PageTreeReceipt};

pub use semio_s_artifact_stdio_pdf_lifter::{Category, DocumentLiftState, FontLift, FontStart, Lifter};
pub use semio_s_artifact_stdio_pdf_page_content::{operator_bytes, project_content_bytes, PageContentBarrier, PageContentCursor, PageContentEvent, PageContentReceipt, PageProgramCursor};
use semio_s_artifact_stdio_pdf_lifter::{lift_viewer_preferences, page_layout_from_name, page_mode_from_name, CATALOG_KEYS};

#[path = "🧩️lifter-and-document/🧭️document-lift/🦀️.rs"]
mod document_lift;
pub use document_lift::*;
