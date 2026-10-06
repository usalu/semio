//! 🫳️ Exact authored Space record metadata and borrowed native wire measurement.
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout};
const fn optional(mut field:F)->F{field.optional=true;field}
fn text()->H{H::Text}
fn user()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"name",H::Text),optional(F::new(2,"avatar",H::Text)),F::new(3,"role",H::Enum(&[("author",0),("spectator",1)]))];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
fn collection()->R{const FIELDS:&[F]=&[F::new(0,"id",H::Text),F::new(1,"name",H::Text),F::new(2,"document-id",H::Text)];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
fn extension()->R{const FIELDS:&[F]=&[F::new(0,"extension-id",H::Text),F::new(1,"version",H::Text),F::new(2,"source-uri",H::Text),F::new(3,"package-hash",H::Text),F::new(4,"enabled",H::Bool)];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
fn extension_shape()->H{H::Record(extension)}
pub(super)fn spec()->R{const FIELDS:&[F]=&[F::new(0,"schema",H::Text),F::new(1,"name",H::Text),F::new(2,"kind",H::Enum(&[("atelier",0),("studio",1),("archive",2)])),F::new(3,"visibility",H::Enum(&[("private",0),("public",1)])),F::new(4,"users",H::Table(user)),F::new(5,"collections",H::Table(collection)),F::new(6,"programs",H::List(text)),F::new(7,"extensions",H::List(extension_shape))];R{keyword:None,layout:RecordLayout::Inline,fields:FIELDS}}
