//! 🧫️ Grants the companion crate access to original export owners.
use super::*;

pub struct WorkState<'a> {
    pub read: Option<&'a store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>>,
    pub job: Option<&'a dyn OwnedSerializerJob>,
    pub request: Option<&'a OwnedSerializerRequest>,
    pub labels: Option<(&'a str, &'a str)>,
    pub raw: Option<&'a OwnedSerializerOutput>,
    pub pending_page: Option<&'a [u8]>,
    pub chunks: Option<&'a ArtifactOutputChunks>,
    pub stage: &'static str,
}

pub fn state(work: &Work) -> WorkState<'_> {
    let owners = work.download.original();
    WorkState {
        read: work.read.as_ref(),
        job: work.job.as_deref(),
        request: work.request.as_ref(),
        labels: owners.map(|owner| (owner.filename.as_str(), owner.mime.as_str())),
        raw: owners.and_then(|owner| owner.raw.as_ref()),
        pending_page: owners.and_then(|owner| owner.pending_page.as_deref()),
        chunks: owners.and_then(|owner| owner.chunks.as_ref()),
        stage: work.phase,
    }
}

pub fn work_with_read(read: store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>) -> Work {
    let mut work = Work::new();
    work.read = Some(read);
    work
}

pub fn work_with_serializer(job: Box<dyn OwnedSerializerJob>, fault: bool) -> Work {
    let mut work = Work::new();
    work.job = Some(job);
    work.phase = if fault { "handoff-fault" } else { "handoff-output" };
    work
}

pub fn work_with_output(output: OwnedSerializerOutput) -> Work {
    let mut work = Work::new();
    work.download.original_mut().unwrap().raw = Some(output);
    work.phase = "output";
    work
}

pub fn header_demands(work: &Work, payload: &export_document::ExportDocument) -> Result<RetirementDemand, ValueError> {
    let mut buffer = [0; export_document::EXPORT_STEM_BYTES];
    Ok(work.header(payload, &mut buffer)?.demands())
}

pub fn prepare_header(work: &mut Work, payload: &export_document::ExportDocument, generation: u64, revision: [u8; 32], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    let mut buffer = [0; export_document::EXPORT_STEM_BYTES];
    let header = work.header(payload, &mut buffer)?;
    work.prepare_header(header, generation, revision, cx)
}

pub fn handoff(work: &mut Work, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, OriginalTurnFailure> {
    work.handoff(cx)
}

pub fn output_demands(work: &Work) -> Result<RetirementDemand, ValueError> {
    work.output_demands()
}

pub fn advance_output(work: &mut Work, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    work.advance_output(cx)
}
