//! 🎞️ Drives the authenticated PNG decoder and the funded PDF edit as one format continuation.

use super::{ImagePlacement, PdfMutation, PdfSnapshot, PngGraphEdit, check_image_placement};
use semio_os_kernel::os_store::one_item_publication::ArtifactMutationPages;
use semio_plugin::__job::{OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_plugin::__value::{RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_plugin::original_media::{OriginalGuestMediaInput, OriginalMediaLoan, RawMediaTarget, RAW_MEDIA_BYTES};
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::png_source::{PNG_GRAPH_MAXIMUM_BYTES, PNG_GRAPH_MAXIMUM_PIXELS};
use semio_framework_pixels::png_decoding::PngDecodeLimits;

type Work = super::RawPngWork<RawMediaTarget, RAW_MEDIA_BYTES>;
type Rows = ArtifactMutationPages<PdfMutation>;

pub struct PngFormatWork {
    caller: OriginalSourceBinding,
    decoder: Work,
    placement: Option<ImagePlacement>,
    size: (u32, u32),
    scanned: usize,
    alpha: bool,
    graph: Option<PngGraphEdit>,
    donated: bool,
    decoder_closed: bool,
}

impl PngFormatWork {
    /// 🪙 Allocates the fixed format owner under its exact picker caller before taking decoder bytes.
    pub fn admit(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if slot.is_some() { return Ok(true); }
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>(),
            retained_capacity_bytes: size_of::<Self>(), ..Default::default()
        };
        if cx.should_yield() || cx.is_cancelled() || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
            return Ok(false);
        }
        let owner = Box::new(Self {
            caller: OriginalSourceBinding::borrow(cx), decoder: Work::empty(), placement: None, size: (0, 0),
            scanned: 0, alpha: false, graph: None, donated: false, decoder_closed: false,
        });
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        *slot = Some(owner);
        Ok(true)
    }

    /// 📦 Produces the Store's actual paged input from the retained PNG and the current published PDF.
    pub fn advance(
        &mut self, input: &mut OriginalGuestMediaInput<RawMediaTarget, RAW_MEDIA_BYTES>, live: OriginalMediaLoan<'_>,
        snapshot: &PdfSnapshot, rows: &mut Option<Rows>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        self.check_caller(cx)?;
        if self.donated { return Ok(true); }
        if cx.is_cancelled() { return Ok(false); }
        if self.placement.is_none() {
            let limits = PngDecodeLimits {
                max_pixels: PNG_GRAPH_MAXIMUM_PIXELS, max_bytes: PNG_GRAPH_MAXIMUM_BYTES, max_chunks: PNG_GRAPH_MAXIMUM_BYTES / 12,
            };
            let demand = self.decoder.demand()?;
            let ceiling = cx.original_turn_ceiling().0;
            if demand.capacity_bytes > ceiling.maximum_capacity_bytes || demand.copy_bytes > ceiling.maximum_copy_bytes {
                return Err(ValueError::literal(ValueRefusalKind::AllocationFailed, "PNG working memory exceeds this Original caller's funded turn").into());
            }
            let _ = self.decoder.advance(input, live, limits, cx)?;
            let Some((target, raster)) = self.decoder.raster(input, live, cx)? else { return Ok(false); };
            let count = (raster.pixels.len() / 4 - self.scanned).min(cx.fuel_remaining().min(1024) as usize);
            let end = self.scanned + count;
            self.alpha = raster.pixels[self.scanned * 4..end * 4].chunks_exact(4).any(|sample| sample[3] != 255);
            self.scanned = end;
            cx.consume_fuel(count as u64);
            if self.alpha || end * 4 == raster.pixels.len() {
                let placement = placement(target)?;
                check_image_placement(snapshot, placement)?;
                self.placement = Some(placement);
                self.size = (raster.width, raster.height);
            }
            return Ok(false);
        }
        let Some(source) = self.decoder.source(input, live, cx)? else { return Ok(false); };
        let Some(graph) = self.graph.as_mut() else {
            PngGraphEdit::admit(source, *live.base_revision, self.placement.expect("PNG placement was checked"), self.size, self.alpha, &mut self.graph, cx)?;
            return Ok(false);
        };
        if graph.advance(source, snapshot, live.base_revision, cx)? && graph.take_pages(live.base_revision, rows, cx)? {
            self.donated = true;
        }
        Ok(self.donated)
    }

    /// ♻️ Closes private rows before decoder custody and the receiver's Original grant return.
    pub fn close(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let Some(owner) = slot.as_mut() else { return Ok(true); };
        owner.check_caller(cx)?;
        if let Some(graph) = owner.graph.as_mut().filter(|graph| !graph.terminal_is_empty()) {
            graph.close(cx)?;
            return Ok(false);
        }
        if !owner.decoder_closed {
            owner.decoder_closed = matches!(owner.decoder.close(cx)?, semio_plugin::__job::InteractiveJobCloseStep::Complete { .. });
            return Ok(false);
        }
        let progress = RetainedCloneProgress {
            copied_items: 1, copied_bytes: size_of::<Option<Box<Self>>>(),
            released_bytes: size_of::<Self>(), ..Default::default()
        };
        if cx.retirement_should_yield() || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        drop(slot.take());
        Ok(true)
    }

    pub fn close_demand(slot: &Option<Box<Self>>, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some(owner) = slot.as_ref() else { return Ok(Default::default()); };
        if let Some(graph) = owner.graph.as_ref().filter(|graph| !graph.terminal_is_empty()) {
            return graph.close_demand(body);
        }
        if !owner.decoder_closed { return owner.decoder.close_demand(body); }
        Ok(RetirementDemand { copy_bytes: size_of::<Option<Box<Self>>>(), release_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }

    fn check_caller(&self, cx: &StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.caller.matches(cx) { Ok(()) } else { Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PNG format changed its Original caller").into()) }
    }
}

fn placement(target: RawMediaTarget) -> Result<ImagePlacement, ValueError> {
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "invalid retained PNG placement");
    let page = usize::try_from(target[1]).map_err(|_| invalid())?;
    match target {
        [0, _, a, b, c, d, x, y] => Ok(ImagePlacement::Insert { page, matrix: [a, b, c, d, x, y].map(f64::from_bits) }),
        [1, _, operator, 0, 0, 0, 0, 0] => Ok(ImagePlacement::Replace { page, operator: usize::try_from(operator).map_err(|_| invalid())? }),
        _ => Err(invalid()),
    }
}
