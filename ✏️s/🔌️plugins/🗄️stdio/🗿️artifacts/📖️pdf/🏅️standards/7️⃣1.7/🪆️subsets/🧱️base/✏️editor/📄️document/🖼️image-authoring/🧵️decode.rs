//! 🧵️ Privately decodes a revision-pinned PNG and emits one PDF page-content transaction.

use super::*;
use semio_framework_pixels::image_decoding::{ImageDecodeInput, ImageDecodeJob};
use semio_plugin::{ArtifactApp, Emit, NoConfigMutation, NoDraftMutation};
use semio_plugin::__job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
use semio_plugin::__value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_plugin::__value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin::__value::retirement::controlled::ControlledRetirement;
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};

pub const PNG_PREFIX: &str = "data:image/png;base64,";
pub const MAX_FILE_BYTES: u32 = 24_546;
pub const MAX_DATA_URL: usize = PNG_PREFIX.len() + 4 * (MAX_FILE_BYTES as usize).div_ceil(3);

/// 📦 Binds the immutable command payload to its target at picker opening.
pub struct PngInput<'a> {
    pub placement: ImagePlacement,
    pub revision: &'a str,
    pub payload: &'a str,
}

/// 🔐 Encodes the full operation-captured artifact revision without allocating in work turns.
pub fn revision_bytes(operation: &semio_plugin::AppOperationContext) -> [u8; 64] {
    let alphabet = b"0123456789abcdef";
    let mut result = [0; 64];
    for (index, value) in operation.canonical_base_revision.iter().copied().enumerate() {
        result[index * 2] = alphabet[usize::from(value >> 4)];
        result[index * 2 + 1] = alphabet[usize::from(value & 15)];
    }
    result
}

type Decoder = Option<ImageDecodeJob>;

/// 🧵 Retains input-owned PNG decoding and its unpublished edit until the invoking job settles.
pub struct PngDecodeWork<A: ArtifactApp> {
    tool: &'static str,
    read: for<'a> fn(&'a A::Command) -> Option<PngInput<'a>>,
    decoder: Option<ControlledRetirement<Decoder>>,
    closing: bool,
}

impl<A: ArtifactApp> PngDecodeWork<A> {
    /// 🌱 Begins with a command reader and an empty decoder workspace.
    pub fn new(tool: &'static str, read: for<'a> fn(&'a A::Command) -> Option<PngInput<'a>>) -> Self {
        Self { tool, read, decoder: None, closing: false }
    }
}

fn refusal() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Local PNG import could not decode its source")
}

impl<A> ArtifactCommandWork<A> for PngDecodeWork<A>
where
    A: ArtifactApp<Snapshot = PdfSnapshot, Mutation = PdfMutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation>,
{
    fn tool_id(&self) -> &'static str { self.tool }

    fn extent(&self, command: &A::Command, _: &PdfSnapshot, _: &semio_plugin::__kernel::InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<A>>) -> Option<usize> {
        (self.read)(command).map(|_| import_rows())
    }

    fn work_demands(&self, _: &ArtifactCommandInputs<'_, A>, _: usize) -> Result<RetirementDemand, ValueError> {
        let Some(job) = self.decoder.as_ref().and_then(ControlledRetirement::original).and_then(Option::as_ref) else {
            return Ok(RetirementDemand { copy_bytes: size_of::<Decoder>(), depth: 1, ..Default::default() });
        };
        if job.progress().done {
            return Ok(RetirementDemand { capacity_bytes: 192 * 1024, depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand {
            copy_bytes: job.next_copy_byte_demand().map_err(|_| refusal())?,
            capacity_bytes: job.next_capacity_byte_demand().map_err(|_| refusal())?,
            depth: job.next_depth_demand().map_err(|_| refusal())?,
            ..Default::default()
        })
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, OriginalTurnFailure> {
        let pending = |stage| ArtifactCommandWorkStep::Progress { stage, preview: b"" };
        if cx.is_cancelled() || self.closing { return Err(fault("Image import cancelled").into()); }
        let Some(PngInput { placement, revision: pinned, payload }) = (self.read)(input.command) else {
            return Err(fault("PDF image completion command mismatch").into());
        };
        if pinned.as_bytes() != revision_bytes(input.operation) { return Err(fault("PDF changed while the image picker was open").into()); }
        if self.decoder.is_none() {
            check_image_placement(input.snapshot, placement)?;
            let mut decoder = ControlledRetirement::new(None::<ImageDecodeJob>)
                .map_err(|_| fault("PDF image decoder owner admission refused"))?;
            match ImageDecodeJob::new(ImageDecodeInput {
                mime: "image/png", data: payload, max_source_bytes: MAX_DATA_URL,
                max_bytes: MAX_FILE_BYTES as usize, max_pixels: MAX_IMAGE_IMPORT_PIXELS, max_chunks: 1024,
            }) {
                Ok(job) => *decoder.original_mut().expect("live decoder owner") = Some(job),
                Err(_) => {
                    self.decoder = Some(decoder);
                    return Err(refusal().into());
                }
            }
            self.decoder = Some(decoder);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Decoder>(), ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(pending("image-admission"));
        }
        let job = self.decoder.as_mut().and_then(ControlledRetirement::original_mut)
            .and_then(Option::as_mut).ok_or_else(refusal)?;
        if job.progress().done {
            let raster = job.result().map_err(|_| refusal())?;
            let extent = raster.pixels.len();
            if cx.fuel_remaining() < extent as u64 { return Ok(pending("image-samples")); }
            let artwork = place_sampled_image(
                input.snapshot, placement,
                SampledRgba { width: raster.width, height: raster.height, pixels: &raster.pixels },
            )?;
            cx.consume_fuel(extent as u64);
            return Ok(ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: artwork, ..Default::default() }));
        }
        while !cx.should_yield() && !job.progress().done {
            let remaining = cx.retained_grant();
            if remaining.maximum_items == 0 { break; }
            let grant = RetainedCloneGrant { maximum_items: 1, ..remaining };
            let advanced = cx.with_original_effect(grant, |_, effect| job.advance(payload, effect))?;
            let Some((result, performed)) = advanced else { break; };
            cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX).max(1));
            let (_, receipt) = result.map_err(|_| refusal())?;
            cx.consume_retained(receipt)?;
            if receipt.copied_items == 0 { break; }
        }
        Ok(pending(job.progress().phase))
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let Some(owner) = self.decoder.as_mut() else {
            return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() });
        };
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = owner.next_demand(body)?;
        semio_plugin::__job::close_original_effect_step(cx, demand, |effect| owner.step(body, effect))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.decoder.as_ref().map_or(Ok(0), ControlledRetirement::next_copy_byte_demand)
    }

    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> {
        self.decoder.as_ref().map_or(Ok(0), |owner| owner.next_capacity_byte_demand(copy))
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        self.decoder.as_ref().map_or(Ok(0), ControlledRetirement::next_release_byte_demand)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        self.decoder.as_ref().map_or(Ok(0), ControlledRetirement::next_depth_demand)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.decoder.as_ref().is_none_or(ControlledRetirement::terminal_is_empty)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
}

/// 🔢 Bounds the durable mutation rows for one new image, optional mask, and placement.
pub fn import_rows() -> usize {
    semio_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(3).work_items()
}

