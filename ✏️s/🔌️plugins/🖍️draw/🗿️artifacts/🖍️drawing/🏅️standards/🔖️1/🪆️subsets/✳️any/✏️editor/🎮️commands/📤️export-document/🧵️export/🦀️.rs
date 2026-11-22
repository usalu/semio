//! 📤️ Registered captured serializers publish through the existing sealed download completion.
use crate::{DRAWING_BOUNDED_PAYLOAD_SCHEMA, DRAWING_EXPORT_PUBLICATION_CONTRACTS, DRAWING_EXPORT_TOOL_IDS, DrawingCommand, DrawingPlayApp};
use semio_s_artifact_draw_drawing_editor_commands::export_document;
use semio_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep};
use semio_plugin::app::{ArtifactDownloadOutput,ArtifactOutputChunks};
use semio_os_kernel::io::io_mechanism::{OwnedSerializerRequest,OwnedSerializerJob,OwnedSerializerStep,OwnedSerializerOutput,io_begin_owned_serialization,io_owned_serialization_demand};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{DslValue, OriginalRetirementFailure, RetirementDemand, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll,retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA;
use semio_framework::Fault;
use semio_framework_job::{OriginalTurnFailure, StepContext};
type App=semio_plugin::EditorApp<DrawingPlayApp>;
const OUTPUT_LIMIT:usize=ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES;
pub(super) const EXECUTION_CONTRACT: semio_framework::ToolExecutionContract =
    semio_framework::ToolExecutionContract::resumable(4_096, 4_096, 1, OUTPUT_LIMIT, 7_500, 1, 1);
const PAGE_INPUT:usize=ArtifactOutputChunks::CHUNK_BYTES/4*3;
const ENCODING:&str="base64";

struct Header<'a> {
 stem: &'a str,
 format: &'a str,
 media: (&'static str, &'static str, &'static str),
 width: Option<u32>,
 height: Option<u32>,
 transparent: bool,
}

impl<'a> Header<'a> {
 fn new(payload: &'a export_document::ExportDocument, stem: &'a str) -> Result<Self, ValueError> {
  let media = match payload.format.as_str() {
   "png" => ("s.stdio.png", "1.2", "image/png"),
   "svg" => ("s.stdio.svg", "1.1", "image/svg+xml"),
   "pdf" => ("s.stdio.pdf", "1.4", "application/pdf"),
   _ => return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Drawing exports to png, pdf or svg")),
  };
  Ok(Self { stem, format: &payload.format, media, width: payload.width, height: payload.height, transparent: payload.transparent.unwrap_or(true) })
 }

 fn route_text(&self) -> [&'static str; 6] {
  let source = semio_s_artifact_draw_drawing_model::DRAWING_DIALECT;
  [source.artifact_kind, source.standard.0, source.subset.0, self.media.0, self.media.1, "*"]
 }

 fn options(&self) -> impl Iterator<Item = (&'static str, DslValue)> {
  [
   self.width.map(|width| ("width", DslValue::uint(u64::from(width)))),
   self.height.map(|height| ("height", DslValue::uint(u64::from(height)))),
   Some(("transparent", DslValue::Bool(self.transparent))),
  ].into_iter().flatten()
 }

 fn demands(&self) -> RetirementDemand {
  let text_bytes = self.stem.len() + 1 + self.format.len() + self.media.2.len()
   + self.route_text().into_iter().map(str::len).sum::<usize>()
   + self.options().map(|(key, _)| key.len()).sum::<usize>();
  let options_bytes = self.options().count() * size_of::<(String, DslValue)>();
  RetirementDemand {
   copy_bytes: text_bytes + options_bytes + size_of::<OwnedSerializerRequest>(),
   capacity_bytes: text_bytes + options_bytes,
   depth: 1,
   ..Default::default()
  }
 }

 fn copy_text(dest: &mut String, parts: &[&str], receipt: &mut RetainedCloneProgress) -> Result<(), ValueError> {
  let length = parts.iter().map(|part| part.len()).sum();
  dest.try_reserve_exact(length).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "Drawing export header text allocation refused"))?;
  receipt.retained_capacity_bytes += dest.capacity();
  for part in parts { dest.push_str(part); }
  receipt.copied_bytes += length;
  Ok(())
 }

 fn prepare(&self, owners: &mut DownloadOwners, receipt: &mut RetainedCloneProgress) -> Result<(), ValueError> {
  Self::copy_text(&mut owners.filename, &[self.stem, ".", self.format], receipt)?;
  Self::copy_text(&mut owners.mime, &[self.media.2], receipt)?;
  let empty_dialect = || ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() };
  let ((from, to), options) = owners.metadata.insert(((empty_dialect(), empty_dialect()), DslValue::Object(Vec::new())));
  let route = [&mut from.artifact_kind, &mut from.standard, &mut from.subset, &mut to.artifact_kind, &mut to.standard, &mut to.subset];
  for (dest, value) in route.into_iter().zip(self.route_text()) { Self::copy_text(dest, &[value], receipt)?; }
  let DslValue::Object(options) = options else { unreachable!() };
  options.try_reserve_exact(self.options().count())
   .map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "Drawing export header options allocation refused"))?;
  receipt.retained_capacity_bytes += options.capacity() * size_of::<(String, DslValue)>();
  for (key, value) in self.options() {
   let mut owned = String::new();
   Self::copy_text(&mut owned, &[key], receipt)?;
   options.push((owned, value));
   receipt.copied_bytes += size_of::<(String, DslValue)>();
  }
  Ok(())
 }
}
#[derive(semio_framework_value::RetireOwned)]
struct DownloadOwners{chunks:Option<ArtifactOutputChunks>,raw:Option<OwnedSerializerOutput>,pending_page:Option<Vec<u8>>,filename:String,mime:String,metadata:Option<((ArtifactDialect,ArtifactDialect),DslValue)>,refusal:Option<ValueError>}
pub struct Work{read:Option<store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>>,read_close:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,request:Option<OwnedSerializerRequest>,job:Option<Box<dyn OwnedSerializerJob>>,download:ControlledRetirement<DownloadOwners>,phase:&'static str,page:usize,cursor:usize,complete:bool,closing:bool}
impl Work{
 pub fn new()->Self{let download=ControlledRetirement::new(DownloadOwners{chunks:None,raw:None,pending_page:None,filename:String::new(),mime:String::new(),metadata:None,refusal:None}).unwrap_or_else(|_|unreachable!());Self{read:None,read_close:None,request:None,job:None,download,phase:"header",page:0,cursor:0,complete:false,closing:false}}
 fn header<'a>(&self, payload: &'a export_document::ExportDocument, buffer: &'a mut [u8; export_document::EXPORT_STEM_BYTES]) -> Result<Header<'a>, ValueError> {
  let source = self.read.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Drawing export source read missing"))?;
  Header::new(payload, export_document::export_stem_in(source.get(), buffer))
 }
 fn prepare_header(&mut self, header: Header<'_>, generation: u64, revision: [u8; 32], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
  if !cx.retained_work_can_enter(header.demands()) { return Ok(false); }
  let mut receipt = RetainedCloneProgress { copied_items: 1, ..Default::default() };
  let owners = self.download.original_mut().unwrap();
  let result = header.prepare(owners, &mut receipt);
  if result.is_ok() {
   let (route, options) = owners.metadata.take().unwrap();
   self.request = Some(OwnedSerializerRequest { source: self.read.take().unwrap().into_erased(), route, generation, revision, options, children: None });
   receipt.copied_bytes += size_of::<OwnedSerializerRequest>();
   self.phase = "admit";
  } else {
   self.phase = "header-refused";
  }
  cx.consume_retained(receipt)?;
  cx.consume_fuel(1);
  result?;
  Ok(true)
 }
 fn handoff(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, semio_framework_job::OriginalTurnFailure> {
  if !cx.retained_work_can_enter(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }) {
   return Ok(ArtifactCommandWorkStep::Pending { stage: self.phase, preview: b"" });
  }
  let grant = cx.retained_grant();
  let job = self.job.as_mut().unwrap();
  if self.phase == "handoff-fault" {
   let handoff = job.take_fault(grant)?.ok_or_else(|| Fault::from("Drawing serializer failed without original diagnostic"))?;
   cx.consume_retained(handoff.progress)?;
   cx.consume_fuel(1);
   return Err(handoff.fault.into());
  }
  let handoff = job.take_output(grant)?.ok_or_else(|| Fault::from("Drawing serializer completed without original output"))?;
  self.download.original_mut().unwrap().raw = Some(handoff.output);
  self.phase = "output";
  cx.consume_retained(handoff.progress)?;
  cx.consume_fuel(1);
  Ok(ArtifactCommandWorkStep::Progress { stage: self.phase, preview: b"" })
 }
 fn output_input(raw: &OwnedSerializerOutput, page: usize, cursor: usize) -> Option<&[u8]> {
  if page < raw.pages.len() {
   return raw.pages.get(page).map(Vec::as_slice);
  }
  let end = (cursor + PAGE_INPUT).min(raw.contiguous.len());
  (cursor < end).then(|| &raw.contiguous[cursor..end])
 }
 fn output_demands(&self) -> Result<RetirementDemand, ValueError> {
  let owners = self.download.original().unwrap();
  if owners.chunks.is_none() {
   return ArtifactOutputChunks::original_birth_demands(OUTPUT_LIMIT);
  }
  if owners.pending_page.is_some() {
   return Ok(ArtifactOutputChunks::original_push_demands());
  }
  let raw = owners.raw.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Drawing export output ownership missing"))?;
  let Some(input) = Self::output_input(raw, self.page, self.cursor) else {
   return Ok(ArtifactOutputChunks::original_seal_demands());
  };
  if input.is_empty() || input.len() > PAGE_INPUT {
   return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Drawing export source page exceeds its wire extent"));
  }
  let capacity_bytes = input.len().div_ceil(3) * 4;
  Ok(RetirementDemand { copy_bytes: input.len() + capacity_bytes, capacity_bytes, depth: 1, ..Default::default() })
 }
 fn advance_output(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
  let demand = self.output_demands()?;
  if cx.should_yield() || cx.is_cancelled() || !cx.retained_work_can_enter(demand) { return Ok(()); }
  let owners = self.download.original_mut().unwrap();
  let Some(chunks) = owners.chunks.as_ref() else {
   ArtifactOutputChunks::original_birth_admit(OUTPUT_LIMIT, &mut owners.chunks, cx)?;
   return Ok(());
  };
  if owners.pending_page.is_some() {
   chunks.push_original(&mut owners.pending_page, None, cx)?;
   return Ok(());
  }
  let raw = owners.raw.as_ref().unwrap();
  let Some(input) = Self::output_input(raw, self.page, self.cursor) else {
   if chunks.seal_original(cx)? { self.phase = "publish"; }
   return Ok(());
  };
  let mut encoded = Vec::new();
  encoded.try_reserve_exact(demand.capacity_bytes).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "Drawing export base64 page allocation refused"))?;
  for word in input.chunks(3) { encoded.extend_from_slice(&base64_codec::base64_standard_word(word)); }
  let retained_capacity_bytes = encoded.capacity();
  owners.pending_page = Some(encoded);
  if self.page < raw.pages.len() { self.page += 1; } else { self.cursor += input.len(); }
  cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes, ..Default::default() })?;
  cx.consume_fuel(1);
  Ok(())
 }
 fn download_demands() -> RetirementDemand {
  RetirementDemand { copy_bytes: size_of::<ArtifactDownloadOutput>() + ENCODING.len(), capacity_bytes: ENCODING.len(), depth: 1, ..Default::default() }
 }
 fn source_authority_matches(&self,generation:u64,revision:[u8;32])->bool{self.read.as_ref().is_some_and(|read|read.commit_authority_matches(generation,revision))||self.request.as_ref().is_some_and(|request|request.source.commit_authority_matches(generation,revision))||self.job.as_ref().is_some_and(|job|job.source_authority_matches(generation,revision))}
 pub fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if self.request.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<OwnedSerializerRequest>(),depth:1,..Default::default()});}if !self.download.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:self.download.next_copy_byte_demand()?,capacity_bytes:self.download.next_capacity_byte_demand(body)?,release_bytes:self.download.next_release_byte_demand()?,depth:self.download.next_depth_demand()?});}if let Some(job)=&self.job{if let Some(release_bytes)=job.terminal_frame_release_bytes(){return Ok(semio_framework_value::RetirementDemand{release_bytes,depth:1,..Default::default()});}return Ok(semio_framework_value::RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?});}if let Some(read)=&self.read_close{return store::os_store::snapshot_read::artifact_retirement_box_demands(read,body);}store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.read)}
}
impl ArtifactCommandWork<App> for Work{
 fn tool_id(&self)->&'static str{"exportDocument"}
 fn work_demands(&self, input: &ArtifactCommandInputs<'_,App>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
  if let Some(request) = self.request.as_ref().filter(|_| self.phase == "admit") {
   let birth = io_owned_serialization_demand(request)?;
   return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth, ..Default::default() });
  }
  match self.phase {
   "header" if self.read.is_some() => {
    let DrawingCommand::ExportDocument(payload) = input.command else {
     return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Drawing export command required"));
    };
    let mut stem = [0; export_document::EXPORT_STEM_BYTES];
    Ok(self.header(payload, &mut stem)?.demands())
   }
   "handoff-output" | "handoff-fault" => Ok(RetirementDemand { depth: 1, ..Default::default() }),
   "output" => self.output_demands(),
   "publish" => Ok(Self::download_demands()),
   _ => Ok(RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() }),
  }
 }
 fn extent(&self,command:&DrawingCommand,_snapshot:&DrawingSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>)->Option<usize>{(!self.complete&&matches!(command,DrawingCommand::ExportDocument(_))).then_some(1)}
 fn step(&mut self,input:&ArtifactCommandInputs<'_,App>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<App>, semio_framework_job::OriginalTurnFailure> {
        if self.read.is_none()&&self.phase=="header"&&self.request.is_none()&&!self.complete&&!self.closing{self.read=input.snapshot_read.take_current(input.operation.generation,input.operation.canonical_base_revision,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:self.phase,preview:b""})}
        if self.complete || self.closing || cx.is_cancelled() { return Err(Fault::from("Drawing export cancelled or terminal").into()); }
        let generation = input.operation.generation;
        let revision = input.operation.canonical_base_revision;
        if self.phase != "handoff-fault" && !self.source_authority_matches(generation, revision) { return Err(Fault::from("Drawing export source authority changed").into()); }
        if cx.should_yield() { return Ok(ArtifactCommandWorkStep::Pending { stage: self.phase, preview: b"" }); }
        let DrawingCommand::ExportDocument(payload) = input.command else { return Err(Fault::from("Drawing export command required").into()); };
        if self.phase == "header-refused" {
            return Err(ValueError::literal(ValueRefusalKind::AllocationFailed, "Drawing export header allocation refused").into());
        }
        if self.phase == "header" {
            let mut stem = [0; export_document::EXPORT_STEM_BYTES];
            let header = self.header(payload, &mut stem)?;
            let applied = self.prepare_header(header, generation, revision, cx)?;
            let stage = self.phase;
            return Ok(if applied {
                ArtifactCommandWorkStep::Progress { stage, preview: b"" }
            } else {
                ArtifactCommandWorkStep::Pending { stage, preview: b"" }
            });
        }
        if matches!(self.phase, "handoff-output" | "handoff-fault") { return self.handoff(cx); }
        if self.phase == "output" {
            let before = cx.retained_progress();
            self.advance_output(cx)?;
            let stage = self.phase;
            return Ok(if cx.retained_progress() == before {
                ArtifactCommandWorkStep::Pending { stage, preview: b"" }
            } else {
                ArtifactCommandWorkStep::Progress { stage, preview: b"" }
            });
        }
        if self.phase == "publish" {
            let demand = Self::download_demands();
            if !cx.retained_work_can_enter(demand) {
                return Ok(ArtifactCommandWorkStep::Pending { stage: self.phase, preview: b"" });
            }
            let mut encoding = String::new();
            encoding.try_reserve_exact(demand.capacity_bytes).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "Drawing export encoding allocation refused"))?;
            encoding.push_str(ENCODING);
            let retained_capacity_bytes = encoding.capacity();
            let owners = self.download.original_mut().unwrap();
            let download = ArtifactDownloadOutput::new(std::mem::take(&mut owners.filename), std::mem::take(&mut owners.mime), Some(encoding), owners.chunks.take().unwrap())?;
            self.complete = true;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(ArtifactCommandWorkStep::CompleteDownload { download, ephemeral: Default::default() });
        }
        let original: Result<_, Fault> = (|| {
  if self.phase == "admit" {
   let grant = cx.retained_grant();
   let progress = match io_begin_owned_serialization(self.request.take().unwrap(), grant) {
    Ok(admission) => {
     self.job = Some(admission.job);
     self.phase = "produce";
     admission.progress
    }
    Err(refusal) => {
     self.request = Some(refusal.request);
     self.download.original_mut().unwrap().refusal = Some(refusal.error);
     self.phase = "refused";
     cx.consume_retained(refusal.progress).map_err(|error| Fault::from(error.into_message()))?;
     return Err(Fault::from(self.download.original().unwrap().refusal.as_ref().unwrap().message.as_ref()));
    }
   };
   cx.consume_retained(progress).map_err(|error| Fault::from(error.into_message()))?;
   cx.consume_fuel(1);
   return Ok(ArtifactCommandWorkStep::Progress { stage: self.phase, preview: b"" });
  }
  if self.phase=="refused"{return Err(Fault::from(self.download.original().unwrap().refusal.as_ref().unwrap().message.as_ref()));}
  if self.phase == "produce" {
      let before = (cx.fuel_remaining(), cx.retained_progress());
      match self.job.as_mut().unwrap().advance_serialization(cx) {
          OwnedSerializerStep::Pending => {
              let stage = cx.stage();
              let step = if before == (cx.fuel_remaining(), cx.retained_progress()) {
                  ArtifactCommandWorkStep::Pending { stage, preview: b"" }
              } else {
                  ArtifactCommandWorkStep::Progress { stage, preview: b"" }
              };
              return Ok(step);
          }
          OwnedSerializerStep::Complete => self.phase = "handoff-output",
          OwnedSerializerStep::Fault => self.phase = "handoff-fault",
          OwnedSerializerStep::Cancelled => return Err(Fault::from("Drawing export cancelled")),
      }
      return Ok(ArtifactCommandWorkStep::Progress { stage: self.phase, preview: b"" });
  }
  Err(Fault::from("Drawing export transport stage invalid"))
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
 fn begin_close(&mut self){self.closing=true;if let Some(job)=&mut self.job{job.begin_close();}}
 fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
  use semio_framework_job::InteractiveJobCloseStep;
  if !self.closing { return Ok(InteractiveJobCloseStep::Blocked); }
  if self.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
  if self.request.is_none() && self.download.terminal_is_empty() {
   if let Some(job) = self.job.as_mut() {
    if job.terminal_frame_release_bytes().is_none() { return job.close_step(cx); }
   }
  }
  let demand = self.close_demands(cx.retained_grant().maximum_copy_bytes)?;
  semio_framework_job::close_original_effect_step(cx, demand, |effect| {
   if self.request.is_some() || self.download.terminal_is_empty() && self.job.is_some() {
    let before = effect.performed_work();
    let result = (|| -> Result<RetainedCloneStep, ValueError> {
     if !effect.performed(1)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
     if let Some(request) = self.request.take() {
      let refused = || ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "Drawing export request could not be retired");
      if request.children.is_some() {
       self.request = Some(request);
       return Err(refused());
      }
      let OwnedSerializerRequest { source, route, generation, revision, options, children } = request;
      match source.try_into_typed::<DrawingSnapshot>() {
       Ok(read) => {
        self.read = Some(read);
        self.download.original_mut().unwrap().metadata = Some((route, options));
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
       }
       Err(source) => {
        self.request = Some(OwnedSerializerRequest { source, route, generation, revision, options, children });
        return Err(refused());
       }
      }
     }
     drop(self.job.take());
     Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }))
    })();
    return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: result.map_err(OriginalRetirementFailure::from_primary), work: effect.performed_work() - before });
   }
   if !self.download.terminal_is_empty() {
    return self.download.step(effect.grant().maximum_copy_bytes, effect);
   }
   if self.read_close.is_some() {
    return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.read_close, effect.grant().maximum_copy_bytes, effect);
   }
   let turn = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.read, &mut self.read_close, effect);
   RetainedWorkPoll::Complete(RetainedWorkOutcome { result: turn.result.map_err(OriginalRetirementFailure::from_primary), work: turn.work })
  })
 }
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
 fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
 fn terminal_is_empty(&self)->bool{self.closing&&self.download.terminal_is_empty()&&self.read.is_none()&&self.read_close.is_none()&&self.request.is_none()&&self.job.is_none()}
}

pub struct DrawingExportCommandJobFactory{keys:Vec<semio_framework::ToolFactoryKey>}
impl DrawingExportCommandJobFactory{pub fn new(controller:&str)->Self{Self{keys:vec![semio_framework::ToolFactoryKey::new(controller,"exportDocument")]}}}
impl semio_framework::ToolJobFactory for DrawingExportCommandJobFactory {
 type Payload=semio_plugin::retained_command::ArtifactRetainedCommandPayload<App>;
 type Job=semio_plugin::retained_command::ArtifactRetainedCommandJob<App>;
 fn keys(&self)->&[semio_framework::ToolFactoryKey]{&self.keys}
 fn payload_schema_id(&self)->&str{DRAWING_BOUNDED_PAYLOAD_SCHEMA}
 fn classification(&self)->semio_framework::InteractiveJobClassification{semio_framework::InteractiveJobClassification::Migrated}
 fn execution_contract(&self)->semio_framework::ToolExecutionContract{EXECUTION_CONTRACT}
 semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes:4096,checkpoint_resume:false);
}
impl semio_plugin::ArtifactOwnedToolJobFactory for DrawingExportCommandJobFactory{type Owner=App;const TOOL_IDS:&'static[&'static str]=DRAWING_EXPORT_TOOL_IDS;const DOCUMENT_SCHEMA:&'static str=DRAWING_DOCUMENT_SCHEMA;const PUBLICATION_CONTRACTS:&'static[semio_plugin::ArtifactToolPublicationContract]=DRAWING_EXPORT_PUBLICATION_CONTRACTS;}
pub fn build(request:&mut Option<semio_plugin::ArtifactOwnedToolJobRequest<App>>,output:&mut Option<semio_plugin::ErasedToolJob>,cx:&mut semio_plugin::__job::StepContext<'_>)->Result<bool,semio_plugin::__job::OriginalTurnFailure>{
 semio_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,DrawingCommand::command_id,4096,4096,false,|_|Work::new())
}

#[cfg(feature = "testkit")]
#[path = "🧪️testkit/🦀️.rs"]
pub mod testkit;
