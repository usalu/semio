//! 🫳️ Exact authored Collection metadata preserves Statements body variants.
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout};
const fn optional(mut field:F)->F{field.optional=true;field}
fn folder()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text),optional(F::new(1,"parent-id",H::Text)),F::new(2,"name",H::Text)];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
fn document()->R{const FIELDS:&[F]=&[F::new(0,"schema",H::Text),F::new(1,"document-id",H::Text)];R{keyword:Some("document"),layout:RecordLayout::Inline,fields:FIELDS}}
fn blob()->R{const FIELDS:&[F]=&[F::new(0,"hash",H::Text),F::new(1,"size",H::UInt),F::new(2,"media-type",H::Text)];R{keyword:Some("blob"),layout:RecordLayout::Inline,fields:FIELDS}}
fn entry()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text),optional(F::new(1,"folder-id",H::Text)),F::new(2,"name",H::Text),F::new(3,"kind-id",H::Text),F::new(4,"body",H::Statements(&[("document",document),("blob",blob)]))];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
fn entry_shape()->H{H::Record(entry)}
pub(super)fn spec()->R{const FIELDS:&[F]=&[F::new(0,"schema",H::Text),F::new(1,"name",H::Text),F::new(2,"folders",H::Table(folder)),F::new(3,"entries",H::List(entry_shape))];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
