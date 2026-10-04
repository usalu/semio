//! 📷️ Controlled exact PNG byte ownership.

use super::*;
use crate::store::sqlite_snapshot::SqliteDatabaseLimits;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};

fn invalid(message: impl Into<String>) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}

pub fn decode_png(data: &[u8], limits: SqliteDatabaseLimits, control: &mut NativeDecodeControl<'_>) -> Result<PngSnapshot, ValueError> {
    if data.len() > limits.max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "PNG input exceeds file limit"));
    }
    let layout = png_layout_bytes(data).map_err(invalid)?;
    if layout.chunks.len() > limits.max_rows {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit, "PNG chunk count exceeds row limit"));
    }
    control.begin_stage(data.len())?;
    let bytes = control.copy_bytes(data)?;
    control.advance(data.len())?;
    control.checkpoint()?;
    Ok(PngSnapshot { schema: control.copy_text(crate::STDIO_PNG_DOCUMENT_SCHEMA)?, bytes })
}

pub fn forecast_png(snapshot: &PngSnapshot, limits: SqliteDatabaseLimits, control: &mut NativeEncodeControl<'_>) -> Result<usize, ValueError> {
    let layout = png_layout(snapshot).map_err(invalid)?;
    if snapshot.bytes.len() > limits.max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "PNG output exceeds file limit"));
    }
    if layout.chunks.len() > limits.max_rows {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit, "PNG chunk count exceeds row limit"));
    }
    control.begin_stage(snapshot.bytes.len())?;
    control.advance(snapshot.bytes.len())?;
    control.checkpoint()?;
    Ok(snapshot.bytes.len())
}

pub fn encode_png(snapshot: &PngSnapshot, limits: SqliteDatabaseLimits, control: &mut NativeEncodeControl<'_>) -> Result<Vec<u8>, ValueError> {
    forecast_png(snapshot, limits, control)?;
    control.copy_bytes(&snapshot.bytes)
}
