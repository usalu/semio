//! 🛫️ Controlled TIFF carrier writer over canonical per-IFD storage.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind,native_encoding::NativeEncodeControl};

fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

fn census(snapshot:&TiffSnapshot,control:&mut NativeEncodeControl<'_>,maximum_rows:usize)->Result<(),ValueError>{
    let mut rows=1usize;let workload=snapshot.ifds.iter().try_fold(snapshot.ifds.len(),|total,ifd|total.checked_add(ifd.entries.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"TIFF semantic workload overflow")))?;control.begin_stage(workload)?;for ifd in &snapshot.ifds{rows=rows.checked_add(1).and_then(|value|value.checked_add(ifd.blocks.iter().map(|block|block.samples.len()+1).sum())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"TIFF semantic row count overflow"))?;for tag in &ifd.entries{rows=rows.checked_add(1).and_then(|value|value.checked_add(usize::try_from(tag.values.count()).ok()?)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"TIFF semantic row count overflow"))?;if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"TIFF semantic row limit"));}control.step()?;}control.step()?;}Ok(())
}

fn encode(snapshot:&TiffSnapshot,control:&mut NativeEncodeControl<'_>,maximum_rows:usize,maximum_file_bytes:usize)->Result<Vec<u8>,ValueError>{
    census(snapshot,control,maximum_rows)?;control.checkpoint()?;let native=super::owned_samples::lower(snapshot,TiffNativeOptions::default(),control)?;let encoded=super::write_native(&native,control).map_err(invalid)?;if encoded.len()>maximum_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"TIFF native output file limit"));}let mut output=control.allocate_vec::<u8>(encoded.len())?;control.begin_stage(encoded.len())?;for chunk in encoded.chunks(65536){output.extend_from_slice(chunk);control.advance(chunk.len())?;}Ok(output)
}

/// 📤️ Emits the TIFF carrier with admitted snapshot traversal and physical copy progress.
pub fn encode_tiff_controlled(snapshot:&TiffSnapshot,control:&mut NativeEncodeControl<'_>,maximum_rows:usize,maximum_file_bytes:usize)->Result<Vec<u8>,ValueError>{encode(snapshot,control,maximum_rows,maximum_file_bytes)}
