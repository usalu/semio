//! 🪟️ Controlled exact-byte BMP native boundary.

use crate::standards::v_v3::subsets::any::schema::snapshot::BmpSnapshot;
use crate::store;
use semio_framework_value::{
    native_decoding::NativeDecodeProgress, native_encoding::NativeEncodeProgress, NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind,
};
use store::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotControl, SqliteSnapshotPhase};

fn invalid(message: impl Into<String>) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}

fn copy_bytes(source: &[u8], native: &mut NativeDecodeControl<'_>) -> Result<Vec<u8>, ValueError> {
    native.begin_stage(source.len())?;
    let mut output = native.allocate_vec(source.len())?;
    for chunk in source.chunks(65_536) {
        output.extend_from_slice(chunk);
        native.advance(chunk.len())?;
    }
    Ok(output)
}

fn hex_body(text: &str, native: &mut NativeDecodeControl<'_>) -> Result<Vec<u8>, ValueError> {
    native.begin_stage(text.len())?;
    let mut digits = 0usize;
    for character in text.chars() {
        if !character.is_whitespace() {
            if !character.is_ascii_hexdigit() {
                return Err(invalid("BMP native text contains non-ASCII hexadecimal input"));
            }
            digits = digits.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "BMP hex size overflow"))?;
        }
        native.advance(character.len_utf8())?;
    }
    if !digits.is_multiple_of(2) {
        return Err(invalid("BMP native text has an incomplete hexadecimal byte"));
    }
    native.begin_stage(text.len())?;
    let mut output = native.allocate_vec(digits / 2)?;
    let mut high = None;
    for character in text.chars() {
        if character.is_whitespace() {
            native.advance(character.len_utf8())?;
            continue;
        }
        let digit = character.to_digit(16).ok_or_else(|| invalid("BMP native text contains invalid hexadecimal input"))? as u8;
        if let Some(first) = high.take() {
            output.push(first << 4 | digit);
        } else {
            high = Some(digit);
        }
        native.advance(character.len_utf8())?;
    }
    Ok(output)
}

pub(crate) fn decode(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<BmpSnapshot, ValueError> {
    let limits = control.limits();
    let length = match payload {
        store::io_schema::IoPayload::Binary(bytes) => bytes.len(),
        store::io_schema::IoPayload::Text(text) => text.len(),
    };
    if length > limits.max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "BMP native input exceeds file limit"));
    }
    control.check_rows(1)?;
    control.allocation_stage(SqliteSnapshotPhase::DecodeNative, |remaining, checkpoint| {
        let mut progress = |event: NativeDecodeProgress| checkpoint(event.completed, event.total);
        let mut native = NativeDecodeControl::new(remaining, &mut progress);
        let result = (|| {
            let bytes = match payload {
                store::io_schema::IoPayload::Binary(bytes) => {
                    let body = store::semio_format::unwrap_binary_controlled(bytes, "stdio.bmp", store::semio_format::Component::Pack, 1, &mut native).map_err(store::semio_format::SemioError::into_value_error)?;
                    crate::standards::v_v3::subsets::any::io::bmp_layout_bytes(body).map_err(invalid)?;
                    copy_bytes(body, &mut native)?
                }
                store::io_schema::IoPayload::Text(text) => {
                    let body = if text.starts_with("semio ") {
                        store::semio_format::split_text_preamble_controlled(text, "stdio.bmp", store::semio_format::Component::Dsl, 1, &mut native).map_err(store::semio_format::SemioError::into_value_error)?
                    } else {
                        text
                    };
                    let bytes = hex_body(body, &mut native)?;
                    crate::standards::v_v3::subsets::any::io::bmp_layout_bytes(&bytes).map_err(invalid)?;
                    bytes
                }
            };
            Ok(BmpSnapshot { schema: native.copy_text(crate::STDIO_BMP_DOCUMENT_SCHEMA)?, bytes })
        })();
        (result, native.owned_bytes())
    })?
}

fn output_body_bytes(snapshot: &BmpSnapshot, encoding: SnapshotEncoding) -> Result<usize, ValueError> {
    crate::standards::v_v3::subsets::any::io::bmp_layout(snapshot).map_err(invalid)?;
    match encoding {
        SnapshotEncoding::Binary => Ok(snapshot.bytes.len()),
        SnapshotEncoding::Text => snapshot.bytes.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "BMP hexadecimal output size overflow")),
    }
}

pub(crate) fn preflight(snapshot: &BmpSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    let body = output_body_bytes(snapshot, encoding)?;
    let component = match encoding {
        SnapshotEncoding::Binary => store::semio_format::Component::Pack,
        SnapshotEncoding::Text => store::semio_format::Component::Dsl,
    };
    let prefix = store::semio_format::declared_envelope_prefix_len("stdio.bmp", component, 1)?;
    let total = prefix.checked_add(body).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "BMP native output size overflow"))?;
    if total > control.limits().max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "BMP native output exceeds file limit"));
    }
    control.check_rows(1)?;
    control.checkpoint(SqliteSnapshotPhase::EncodeNative, 1, 1)
}

pub(crate) fn encode(snapshot: &BmpSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
    preflight(snapshot, encoding, control)?;
    control.allocation_stage(SqliteSnapshotPhase::EncodeNative, |remaining, checkpoint| {
        let mut progress = |event: NativeEncodeProgress| checkpoint(event.completed, event.total);
        let mut native = NativeEncodeControl::new(remaining, &mut progress);
        let result = (|| {
            match encoding {
                SnapshotEncoding::Binary => Ok(store::io_schema::IoPayload::Binary(store::semio_format::wrap_binary_controlled("stdio.bmp", store::semio_format::Component::Pack, 1, &snapshot.bytes, &mut native)?)),
                SnapshotEncoding::Text => {
                    let count = snapshot.bytes.len().checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "BMP hexadecimal output size overflow"))?;
                    native.begin_stage(count)?;
                    let mut body = native.allocate_vec(count)?;
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    for chunk in snapshot.bytes.chunks(65_536) {
                        for byte in chunk {
                            body.extend_from_slice(&[HEX[(byte >> 4) as usize], HEX[(byte & 15) as usize]]);
                        }
                        native.advance(chunk.len() * 2)?;
                    }
                    let text = String::from_utf8(body).map_err(|error| ValueError::new(ValueRefusalKind::InvariantViolated, error.to_string()))?;
                    Ok(store::io_schema::IoPayload::Text(store::semio_format::wrap_text_controlled("stdio.bmp", store::semio_format::Component::Dsl, 1, &text, &mut native)?))
                }
            }
        })();
        (result, native.owned_bytes())
    })?
}
