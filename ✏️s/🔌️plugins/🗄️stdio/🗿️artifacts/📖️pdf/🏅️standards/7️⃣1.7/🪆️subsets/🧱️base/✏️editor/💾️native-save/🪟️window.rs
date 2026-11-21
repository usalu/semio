//! 🪟️ Combined Original source and segmented output window for the native PDF export producer.

use std::io::{self, Write};
use semio_plugin::__job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError};
use semio_plugin::ArtifactOutputChunks;

/// 🔗️ Lends output room and one combined physical copy budget to a single emitter turn.
/// The emitter supplies exact source reads even if it refused after a partial write.
pub fn write_pdf_original_turn<R>(
    page: &mut Vec<u8>,
    cx: &mut StepContext<'_>,
    write: impl FnOnce(&mut dyn Write, usize) -> (R, usize),
) -> Result<Option<R>, OriginalTurnFailure> {
    cx.original_source_parent()?;
    let cap = ArtifactOutputChunks::CHUNK_BYTES;
    if page.len() > cap || page.capacity() < cap {
        return Err(ValueError::invariant("PDF export lost its admitted output page").into());
    }
    let grant = cx.retained_grant();
    if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
        return Ok(None);
    }
    let combined = grant.maximum_copy_bytes
        .min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
    let room = (cap - page.len()).min(combined);
    if room == 0 {
        return Ok(None);
    }
    let mut output = OutputWindow { page, remaining: room };
    let (result, read) = write(&mut output, combined);
    let written = room - output.remaining;
    let copied = read.checked_add(written).filter(|total| *total <= combined)
        .ok_or_else(|| ValueError::invariant("PDF emitter exceeded its combined Original copy window"))?;
    if copied != 0 {
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() })?;
        cx.consume_fuel(copied as u64);
    }
    Ok(Some(result))
}

struct OutputWindow<'a> {
    page: &'a mut Vec<u8>,
    remaining: usize,
}

impl Write for OutputWindow<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() { return Ok(0); }
        let copied = bytes.len().min(self.remaining);
        if copied == 0 { return Err(io::ErrorKind::WouldBlock.into()); }
        self.page.extend_from_slice(&bytes[..copied]);
        self.remaining -= copied;
        Ok(copied)
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
