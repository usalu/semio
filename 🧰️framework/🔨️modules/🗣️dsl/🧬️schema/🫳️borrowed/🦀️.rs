//! 🖨️ Borrowed schema measuring belongs to actual Record text emission.
use crate::{BorrowedRecordSpec, ValueError};
use crate::native_encoding::FieldProjectionSource;
use semio_framework_value::NativeEncodeControl;
/// 📏️ Exact canonical Document Text demand from a retained source and borrowed schema.
pub fn measure_print_borrowed<T:FieldProjectionSource>(source:&T,spec:&BorrowedRecordSpec,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{crate::controlled_encoding::measure(source,spec,maximum,control)}
