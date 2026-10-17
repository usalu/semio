//! 🔺️ PdfDiff (1.7) — sparse diff over the typed document model. Ticket
//! 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE widened it with the model: `pages` is an index-keyed
//! triple of per-field page patches whose `content` and `annotations` are themselves
//! index-keyed triples (an inserted operator is one added row, not a re-sent stream); every
//! document collection (`fonts`, `images`, `forms`, graphics states, shadings, patterns, named
//! colour spaces and property lists, embedded files) is an id-keyed triple of whole values;
//! `outlines`, `named_destinations`, `page_labels`, `output_intents` are index-keyed triples;
//! the catalog scalars are tri-state (`Clear`/`Set`); `info` is a field-wise tri-state patch; and the
//! retained COS lanes keep their recursive `PdfValueDiff` patches (`objects` keyed by `ObjRef`,
//! `trailer`/`catalog_extra` name-keyed).
//!
//! Every triple follows the recipe: `removed`/`modified` address BASE state (removals processed
//! descending), `added` addresses FINAL state (ascending insert).
//!
//! Codec: the derive-owned value encoding is the wire — one JSON line as text, the container-less
//! pack record body as binary — so every lane rides one codec instead of a hand-rolled one per
//! field (see `🔖️DiffCodec`).

use crate::standards::v1_7::subsets::base::schema::graph_projection::carry_graph_edit;
use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, validate_role_transition};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{ApplyCapability, MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

#[path = "🔀️tri-state-indexed-and-keyed/🦀️.rs"]
mod tri_state_indexed_and_keyed;
pub use tri_state_indexed_and_keyed::*;

#[path = "📄️page-info-and-pages/🦀️.rs"]
mod page_info_and_pages;
pub use page_info_and_pages::*;

#[path = "🌳️value-trees/🦀️.rs"]
mod value_trees;
pub use value_trees::*;

#[path = "🧭️path-addressing-and-validation/🦀️.rs"]
mod path_addressing_and_validation;
pub use path_addressing_and_validation::*;

#[path = "🧾️pdf-diff-aggregate/🦀️.rs"]
mod pdf_diff_aggregate;
pub use pdf_diff_aggregate::*;

#[path = "🛠️mutation-diff-builders/🦀️.rs"]
mod mutation_diff_builders;
pub use mutation_diff_builders::*;

//#region 🔖️DiffCodec

//#endregion 🔖️DiffCodec

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;
pub use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
pub use crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter;
//#endregion 🔁️Re-exports
