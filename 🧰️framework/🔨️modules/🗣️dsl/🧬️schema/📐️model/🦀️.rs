//! 📐️ Authored Record value, shape, field projection and typed receiving ownership.
#[cfg(test)]
extern crate self as semio_framework_dsl_record;
pub use semio_framework_diagnostic::{Limits,TextError,TextSpan};
pub use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_value::{DslValue,Number};
pub use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
#[path="../🪆️binding/🦀️.rs"]
mod binding;
pub use binding::*;
#[cfg(test)]
#[path="../🛬️receive/🦀️.rs"]
mod native_receive;
#[path="../🫴️receiving/🦀️.rs"]
pub mod receiving;
pub use receiving::{DslReceivingField,DslReceivingRecord,DslReceivingVariants,DslOwnedReceivingField,DslOwnedReceivingRecord,DslOwnedReceivingVariants,OwnedFieldReceiving};
#[path="../🛫️encode/🦀️.rs"]
pub mod native_encoding;
pub use semio_framework_dsl_record_ownership::{
    RecordBytes,RecordLayout,Shape,FieldSpec,RecordSpec,GrammarSpec,WireNode,WireEdgeLabel,WireValue,
    FieldValue,RecordFields,RecordValue,Cst,ExprOp,ExprValue,shape_type_name,
    BorrowedShape,BorrowedFieldSpec,BorrowedRecordSpec,NativeSchemaControl,RecordSpecProducer,producer,retirement,
};
#[path="../🫳️borrowed/🏭️authored/🦀️.rs"]
mod authored_borrowed_schema;
pub use authored_borrowed_schema::{BorrowedRecordSpecProducer,BorrowedDslField,BorrowedDslRecord,BorrowedDslVariants,borrowed_field_shape,borrowed_record,borrowed_unit};
