//! 🌊 Controlled PDF cross-reference stream sections.

#[path = "../../🦀️.rs"]
mod row;
pub use row::XrefStreamRowCursor;

#[path = "../../🧭️section/🦀️.rs"]
mod section;
pub use section::{XrefStreamError, XrefStreamEvent, XrefStreamReceipt, XrefStreamSectionCursor};

#[path = "../../📦️object-stream/🦀️.rs"]
mod object_stream;
pub use object_stream::{ObjectStreamCursor, ObjectStreamEvent, ObjectStreamReceipt, ObjectStreamHeaderCursor, ObjectStreamHeaderEvent, ObjectStreamHeaderReceipt, ObjectStreamMemberCursor};

#[path = "../../📇️index/🦀️.rs"]
mod index;
pub use index::{RevisionIndex, XrefIndex, XrefIndexLookup, XrefIndexReceipt, XrefIndexScan, XrefIndexStatus, XrefRow};

#[path = "../../../📑️classic/🦀️.rs"]
mod classic;
pub use classic::{ClassicXrefCursor, ClassicXrefEvent, ClassicXrefReceipt};

#[path = "../../../📋️section/🦀️.rs"]
mod classic_section;
pub use classic_section::{ClassicSectionCursor, ClassicSectionEvent, ClassicSectionReceipt};

#[path = "../../../🔎️footer/🦀️.rs"]
mod footer;
pub use footer::{FooterCursor, FooterStep};

#[path = "../../../🧭️revisions/🦀️.rs"]
mod revisions;
pub use revisions::{XrefRevisionCursor, XrefRevisionEvent, XrefRevisionReceipt};

#[path = "../../../🔎️offset-validation/🦀️.rs"]
mod offset_validation;
pub use offset_validation::{XrefOffsetCursor, XrefOffsetStatus};

#[path = "../../../🧵️assembly/🦀️.rs"]
mod assembly;
pub use assembly::{XrefAssemblyCursor, XrefAssemblyEvent, XrefAssemblyReceipt};

#[path = "../../../📨️trailer-fold/🦀️.rs"]
mod trailer_fold;
pub use trailer_fold::{StructuredTrailer, XrefTrailerCursor};

#[path = "../../../🗃️graph-materialization/🦀️.rs"]
mod graph_materialization;
pub use graph_materialization::{RetainedGraphBuilder, GraphMaterializationEvent, GraphMaterializationReceipt};

#[path = "../../../📚️file-order/🦀️.rs"]
mod file_order;
pub use file_order::{XrefFileOrderCursor, XrefFileOrderEvent, XrefFileOrderReceipt};

#[path = "../../../📦️normal-projection/🦀️.rs"]
mod normal_projection;
pub use normal_projection::{NormalProjection, NormalProjectionCursor, NormalProjectionEvent, NormalProjectionReceipt};

#[path = "../../../🛤️structured-file/🦀️.rs"]
mod structured_file;
pub use structured_file::{StructuredXrefCursor, StructuredXrefEvent, StructuredXrefReceipt};

/// 🗂️ The original location of an indirect document object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefEntry {
    Normal { offset: usize, gen: u16 },
    Compressed { stream_num: u32, index: u32 },
}
