//! 🔺️ SemioPresentationDiff — handcrafted sparse diff over `SemioPresentationSnapshot`
//! (`masters`/`layouts`/`slides`). No `snapshot: Option<SemioPresentationSnapshot>` full-replace
//! slot — even `SetSnapshot`'s diff is the sparse field-by-field `SemioPresentationDiff::between`.
//!
//! Collection key kinds (per the recipe's "Key kinds per collection" rule): `masters`/`layouts`
//! are id-keyed (`NamedTripleDiff`, referenced BY id from `layouts.master_id`/`slides.layout_id`,
//! like docx's name-keyed `styles`); `slides` is INDEX-keyed (`IndexedTripleDiff`, presentation
//! order is significant, like pdf page order), and a slot whose slide changes identity carries the
//! new `id` in `SlideDiff::id` — so a reorder or a whole-deck replacement lands every slide's own
//! identity at its new index instead of leaving the old identifiers behind the new content. `shapes` (owned by masters/layouts/slides alike), `notes`, and `TextBox`/table
//! `blocks` are all index-keyed too.
//!
//! `document::DocBlock` is reused verbatim for text content (`TextBox.blocks`, table cell
//! `blocks`, `Slide.notes`) but is OWNED by the `document` subset, out of this file's write scope
//! — it has no field-level diff type of its own exposed yet, so this file diffs `Vec<DocBlock>`
//! items as WHOLE VALUES (`D = T = DocBlock`; "modified" carries the complete replacement block).
//! This is honest per the recipe's weak/strong-entity split: from this subset's point of view
//! `DocBlock` is a value struct it does not own the internals of, so it is never sub-diffed here.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedAdded;

use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
use crate::schema::snapshot::SemioPresentationSnapshot;
use crate::schema::snapshot::{PlaceholderKind, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage, SlideShape, SlideTableCell, SlideTableRow};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

#[path = "🧬️diff-types/🦀️.rs"]
mod diff_types;
pub use diff_types::*;

#[path = "⚙️indexed-and-named-engine/🦀️.rs"]
mod indexed_and_named_engine;
use indexed_and_named_engine::*;

#[path = "🖼️frame-image-and-doc-block-logic/🦀️.rs"]
mod frame_image_and_doc_block_logic;
use frame_image_and_doc_block_logic::*;

#[path = "🔷️shape-and-table-logic/🦀️.rs"]
mod shape_and_table_logic;
use shape_and_table_logic::*;

#[path = "📑️master-layout-and-slide-logic/🦀️.rs"]
mod master_layout_and_slide_logic;
use master_layout_and_slide_logic::*;

#[path = "🔌️mutation-diff-and-algebra/🦀️.rs"]
mod mutation_diff_and_algebra;

#[path = "🛠️mutation-diff-helpers/🦀️.rs"]
mod mutation_diff_helpers;
pub use mutation_diff_helpers::*;

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` — same grammar style docx/gif/svg's own hand-rolled
/// codecs use (bracket-depth-aware split via the shared `engine::triples` primitives, hex for
/// strings/bytes, `[0]`/`[1,x]` for `Option<T>`, single uppercase tag letters for data-carrying
/// enums). `IndexedTripleDiff`/`NamedTripleDiff`'s own `enc_indexed_triple`/`enc_named_triple`
/// (shared `engine::triples`) drive every collection instantiation below.
//#region 🔖️Primitives

//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs

//#endregion 🔖️ValueCodecs

//#region 🔖️GenericTripleCodecs

//#endregion 🔖️GenericTripleCodecs

//#region 🔖️DiffValueCodecs

//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel

//#region 🔖️BinaryPrimitives

//#endregion 🔖️BinaryPrimitives

//#endregion 🔖️TopLevel

#[cfg(any(test, feature = "testkit"))]
#[path = "🌱️demo-cases/🦀️.rs"]
mod demo_cases;
#[cfg(any(test, feature = "testkit"))]
pub use demo_cases::*;

//#region 🧪️Tests

//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec
