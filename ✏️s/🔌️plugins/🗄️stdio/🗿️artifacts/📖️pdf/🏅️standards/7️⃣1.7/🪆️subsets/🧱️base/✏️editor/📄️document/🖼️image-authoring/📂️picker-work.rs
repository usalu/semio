//! 📂️ Issues the PDF retained PickImage from its original command snapshot and authority.

use super::{check_image_placement, revision_bytes, ImagePlacement, PngTarget, IMPORT, INSERT_OPERATOR, PdfMutation, PdfSnapshot};
use semio_plugin::__value::retained_clone::RetainedCloneProgress;
use semio_plugin::original_media::{OriginalMediaLoan, RawMediaIntent, ORIGINAL_MEDIA_COMMAND_KIND, issue::issue_args};
use semio_plugin::__job::{OriginalTurnFailure, StepContext};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::{ArtifactApp, Effect, Emit, Fault, NoConfigMutation, NoDraftMutation, RequestId};

use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::png_source::PNG_GRAPH_MAXIMUM_BYTES;

type Pick<A> = for<'a> fn(&'a <A as ArtifactApp>::Command) -> Result<(PngTarget, &'a str), Fault>;

pub struct PngPickerWork<A: ArtifactApp> {
    tool: &'static str,
    read: Pick<A>,
}

impl<A: ArtifactApp> PngPickerWork<A> {
    pub fn new(tool: &'static str, read: Pick<A>) -> Self { Self { tool, read } }
}

impl<A> ArtifactCommandWork<A> for PngPickerWork<A>
where
    A: ArtifactApp<Snapshot = PdfSnapshot, Mutation = PdfMutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation>,
{
    fn tool_id(&self) -> &'static str { self.tool }

    fn extent(
        &self, _: &A::Command, _: &PdfSnapshot, _: &semio_plugin::__kernel::InteractionState,
        _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<A>>,
    ) -> Option<usize> { Some(1) }

    fn reports_progress(&self) -> bool { false }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, cx: &mut StepContext<'_>)
        -> Result<ArtifactCommandWorkStep<A>, OriginalTurnFailure>
    {
        let (picked, guard) = (self.read)(input.command)?;
        let target = PngTarget { page: crate::document::resolve_page(input.snapshot.pages.len(), u64::from(picked.page))? as u32, operator: picked.operator, x: picked.x, y: picked.y, width: picked.width, height: picked.height };
        let operation = input.operation;
        if target.operator != INSERT_OPERATOR && guard.as_bytes() != &revision_bytes(operation)[..16] {
            return Err(Fault::from("Selected PDF image changed before the picker opened").into());
        }
        let placement = target.placement();
        check_image_placement(input.snapshot, placement)?;
        let snapshot = OriginalMediaLoan { operation_id: operation.operation_id, generation: operation.generation,
            instance_id: operation.app_instance_id, document_id: &operation.parent_document_id, base_revision: &operation.canonical_base_revision };
        let charge = RetainedCloneProgress { copied_items: 4, copied_bytes: 8_192, retained_capacity_bytes: 4_096, ..Default::default() };
        if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() < 1030 || cx.retained_grant().maximum_depth < 2 || !charge.fits(cx.retained_grant()) {
            return Ok(ArtifactCommandWorkStep::Pending { stage: "picker-admission", preview: b"" });
        }
        let mut original = None;
        if !RawMediaIntent::issue(snapshot, placement_words(placement), ORIGINAL_MEDIA_COMMAND_KIND, PNG_GRAPH_MAXIMUM_BYTES, &mut original, cx)? {
            return Ok(ArtifactCommandWorkStep::Pending { stage: "picker-admission", preview: b"" });
        }
        let args = issue_args(original.as_ref().expect("admitted PDF placement has a typed intent"), cx)?
            .ok_or_else(|| Fault::from("PDF picker exhausted its reserved emission grant"))?;
        let accepted = "image/png,.png".to_owned();
        let import_action = IMPORT.to_owned();
        let lexical = accepted.capacity() + import_action.capacity();
        let emit = Emit::effect(Effect::RequestFileOpen {
            req: RequestId(0x5044494d), accept: accepted, read_as: None, import_action,
            multiple: false, args: Some(args), max_file_bytes: Some(PNG_GRAPH_MAXIMUM_BYTES as u32),
        });
        let emission = lexical + emit.effects.capacity() * size_of::<Effect>();
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: emission,
            retained_capacity_bytes: emission, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
}

/// 🧮️ Keeps placement exact through the fixed guest-only carrier.
fn placement_words(placement: ImagePlacement) -> [u64; 8] {
    let mut words = [0; 8];
    match placement {
        ImagePlacement::Insert { page, matrix } => {
            words[1] = page as u64;
            for (word, value) in words[2..].iter_mut().zip(matrix) { *word = value.to_bits(); }
        }
        ImagePlacement::Replace { page, operator } => {
            words[0] = 1;
            words[1] = page as u64;
            words[2] = operator as u64;
        }
    }
    words
}
