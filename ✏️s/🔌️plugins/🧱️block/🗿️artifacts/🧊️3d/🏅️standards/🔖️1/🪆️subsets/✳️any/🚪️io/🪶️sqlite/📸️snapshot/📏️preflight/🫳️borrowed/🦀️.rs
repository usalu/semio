//! 🫳️ Literal Block3d metadata for canonical retained owner wire measurement.
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout};
const fn optional(mut f:F)->F{f.optional=true;f}
const fn defines(mut f:F,key:&'static str)->F{f.defines=Some(key);f}
fn text()->H{H::Text}
fn kind()->R{const FS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"name",H::Text),F::new(2,"label",H::Text),optional(F::new(3,"variant",H::Text)),F::new(4,"description",H::Text),optional(F::new(5,"icon",H::Text)),optional(F::new(6,"unit",H::Text))];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn kind_shape()->H{H::Record(kind)}
fn attribute()->R{const FS:&[F]=&[F::new(0,"key",H::Text),F::new(1,"value",H::Text),optional(F::new(2,"definition",H::Text))];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn representation()->R{const FS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"name",H::Text),optional(F::new(2,"mesh-url",H::Text)),F::new(3,"tags",H::List(text)),optional(F::new(4,"lod",H::Text)),F::new(5,"description",H::Text),F::new(6,"attributes",H::Table(attribute))];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn target()->R{const FS:&[F]=&[F::new(0,"artifact-id",H::Text),F::new(1,"artifact-kind",H::Text),F::new(2,"standard",H::Text),F::new(3,"subset",H::Text)];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn child()->R{const FS:&[F]=&[F::new(0,"child_id",H::Text),F::new(1,"target",H::Record(target))];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn extra()->R{const FS:&[F]=&[defines(F::new(0,"id",H::Text),"vortex_kind"),F::new(1,"name",H::Text),F::new(2,"label",H::Text),F::new(3,"color",H::Text),F::new(4,"default-cable-kind",H::Text)];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn vortex()->R{const FS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"vortex-kind",H::Ref("vortex_kind")),F::new(2,"position",H::Coord(3)),F::new(3,"direction",H::Dir),F::new(4,"radius",H::Float),optional(F::new(5,"label",H::Text))];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn compatibility()->R{const FS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"source",H::Text),F::new(2,"target",H::Text),F::new(3,"bidirectional",H::Bool)];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn author()->R{const FS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"name",H::Text),optional(F::new(2,"email",H::Text))];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn camera()->R{const FS:&[F]=&[F::new(0,"position",H::Coord(3)),F::new(1,"target",H::Coord(3)),F::new(2,"zoom",H::Float)];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn camera_shape()->H{H::Record(camera)}
fn meta()->R{const FS:&[F]=&[F::new(0,"description",H::Text)];R{keyword:None,layout:RecordLayout::Inline,fields:FS}}
fn meta_shape()->H{H::Record(meta)}
pub(super)fn spec()->R{const FS:&[F]=&[F::new(0,"schema",H::Text),F::new(1,"object-kind",H::Block(kind_shape)),F::new(2,"representations",H::Table(representation)),F::new(3,"catalog",H::Record(child)),F::new(4,"vortex-kind-extra",H::Table(extra)),F::new(5,"vortices",H::Table(vortex)),F::new(6,"compatibility",H::Table(compatibility)),F::new(7,"attributes",H::Table(attribute)),F::new(8,"authors",H::Table(author)),F::new(9,"camera3d",H::Block(camera_shape)),F::new(10,"meta",H::Block(meta_shape))];R{keyword:None,layout:RecordLayout::Lines,fields:FS}}
