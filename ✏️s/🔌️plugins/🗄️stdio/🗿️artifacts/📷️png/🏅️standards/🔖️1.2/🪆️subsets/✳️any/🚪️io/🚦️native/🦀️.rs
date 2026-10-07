//! 🚦️ Precise PNG native admission and canonical publication.
use super::*;
use crate::store::sqlite_snapshot::SqliteDatabaseLimits;
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError {ValueError::new(ValueRefusalKind::InvalidValue,message)}
pub fn decode_png(data:&[u8],limits:SqliteDatabaseLimits,control:&mut NativeDecodeControl<'_>)->Result<PngSnapshot,ValueError> {
 if data.len()>limits.max_file_bytes {return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG input exceeds file limit"));}
 let image=decode_png_image_controlled(data,control).map_err(PngReadError::into_value)?;
 if image.samples.len().checked_add(image.text_chunks.len()).and_then(|n|n.checked_add(image.ancillary_chunks.len())).is_none_or(|n|n>limits.max_rows) {return Err(ValueError::new(ValueRefusalKind::WorkLimit,"PNG owned image exceeds row limit"));}
 Ok(PngSnapshot {schema:control.copy_text(crate::STDIO_PNG_DOCUMENT_SCHEMA)?,image})
}
pub fn forecast_png(snapshot:&PngSnapshot,limits:SqliteDatabaseLimits,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError> {snapshot.validate().map_err(invalid)?;let bytes=encode_png_image_controlled(&snapshot.image,limits.max_file_bytes,control)?;Ok(bytes.len())}
pub fn encode_png(snapshot:&PngSnapshot,limits:SqliteDatabaseLimits,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError> {snapshot.validate().map_err(invalid)?;encode_png_image_controlled(&snapshot.image,limits.max_file_bytes,control)}
