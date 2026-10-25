//! 📋️ Clipboard jobs of the puzzle 5d editor session: the selection scan and the reserved copy and cut jobs. See [`super`].
use semio_framework_job::InteractiveJob;
use semio_framework_plugin::ArtifactReservedJob;

use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::editor::puzzle5d::config::Puzzle5dConfigMutation;
use semio_framework_plugin::{kernel::ClipboardFragment, kernel::Effect};
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_pack_json::Value;
use std::collections::HashSet;
use crate::editor::puzzle5d::session::support::{puzzle5d_job_checkpoint, puzzle5d_job_fault, puzzle5d_selection_ids, puzzle5d_step_envelope};
use crate::editor::puzzle5d::Puzzle5dPlayApp;
use crate::editor::puzzle5d::entities::{PUZZLE5D_PLAY_APP_ID, PUZZLE5D_RESERVED_OUTPUT_BYTES, PUZZLE5D_RESERVED_PAGE_BYTES, PUZZLE5D_SCHEMA, Puzzle5dFastener, Puzzle5dPart, puzzle5d_editor_projection, puzzle5d_record_from_projection};
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::interaction::{owning_part_id_local};

//#region 🧵️ReservedJobs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dSelectionStage {
    Endpoints,
    Fasteners,
    Parts,
    Complete,
}

/// ♻️ The scan's retained owners, staged whole when its job closes.
#[derive(Default, semio_framework_value::RetireOwned)]
struct Puzzle5dSelectionScanOwners {
    pub snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub projection: Option<Value>,
    pub part_ids: HashSet<String>,
    explicit_fastener_ids: HashSet<String>,
    pub parts: Vec<Puzzle5dPart>,
    pub fasteners: Vec<Puzzle5dFastener>,
}

struct Puzzle5dSelectionScan {
    pub snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub projection: Option<Value>,
    pub part_ids: HashSet<String>,
    explicit_fastener_ids: HashSet<String>,
    pub stage: Puzzle5dSelectionStage,
    pub cursor: usize,
    pub parts: Vec<Puzzle5dPart>,
    pub fasteners: Vec<Puzzle5dFastener>,
}

impl Puzzle5dSelectionScan {
    fn into_owners(&mut self) -> Puzzle5dSelectionScanOwners {
        Puzzle5dSelectionScanOwners { snapshot: self.snapshot.take(), projection: self.projection.take(), part_ids: std::mem::take(&mut self.part_ids), explicit_fastener_ids: std::mem::take(&mut self.explicit_fastener_ids), parts: std::mem::take(&mut self.parts), fasteners: std::mem::take(&mut self.fasteners) }
    }

    pub fn new(snapshot: std::sync::Arc<Puzzle5dPlaySnapshot>, interaction: &semio_framework::InteractionState) -> Self {
        let (part_ids, explicit_fastener_ids) = puzzle5d_selection_ids(interaction);
        Self { snapshot: Some(snapshot), projection: None, part_ids, explicit_fastener_ids, stage: Puzzle5dSelectionStage::Endpoints, cursor: 0, parts: Vec::new(), fasteners: Vec::new() }
    }

    /// 🗂️ One cursored row of the scanned projection, which is derived ONCE and then cached. Re-deriving
    /// it per step — and cloning the whole row array with it — cost O(document) on every one of a
    /// selection's ~2N+M cursor turns and blew the interactive ceiling on a real document:
    /// `framework route 'copy' overran the 8 ms step ceiling for 4 consecutive steps, worst 70521us`
    /// on the 180-part Nakagin, which is what made `copy`/`cut` fault the moment the shipped examples
    /// stopped loading empty. The cache is retired on its own rung of the close ladder.
    pub fn row(&mut self, key: &str, index: usize) -> Option<Value> {
        if self.projection.is_none() {
            let projection = self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot))?;
            self.projection = Some(projection);
        }
        self.projection.as_ref()?.get(key).and_then(Value::as_array).and_then(|rows| rows.get(index)).cloned()
    }

    pub fn step(&mut self) -> Result<bool, String> {
        match self.stage {
            Puzzle5dSelectionStage::Endpoints => {
                let cursor = self.cursor;
                if let Some(row) = self.row("fasteners", cursor) {
                    self.cursor += 1;
                    if row.get("id").and_then(Value::as_str).is_some_and(|id| self.explicit_fastener_ids.contains(id)) {
                        if let Some(source) = row.get("source").and_then(Value::as_str) {
                            self.part_ids.insert(owning_part_id_local(source).to_string());
                        }
                        if let Some(target) = row.get("target").and_then(Value::as_str) {
                            self.part_ids.insert(owning_part_id_local(target).to_string());
                        }
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Fasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dSelectionStage::Fasteners => {
                let cursor = self.cursor;
                if let Some(row) = self.row("fasteners", cursor) {
                    self.cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).map(owning_part_id_local);
                    let target = row.get("target").and_then(Value::as_str).map(owning_part_id_local);
                    let selected = row.get("id").and_then(Value::as_str).is_some_and(|id| self.explicit_fastener_ids.contains(id))
                        || source.zip(target).is_some_and(|(source, target)| !self.part_ids.is_empty() && self.part_ids.contains(source) && self.part_ids.contains(target));
                    if selected {
                        self.fasteners.push(puzzle5d_record_from_projection(row).map_err(|error| error.to_string())?);
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Parts;
                    self.cursor = 0;
                }
            }
            Puzzle5dSelectionStage::Parts => {
                let cursor = self.cursor;
                if let Some(row) = self.row("parts", cursor) {
                    self.cursor += 1;
                    if row.get("id").and_then(Value::as_str).is_some_and(|id| self.part_ids.contains(id)) {
                        self.parts.push(puzzle5d_record_from_projection(row).map_err(|error| error.to_string())?);
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Complete;
                }
            }
            Puzzle5dSelectionStage::Complete => return Ok(true),
        }
        Ok(self.stage == Puzzle5dSelectionStage::Complete)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dClipboardStage {
    Envelope,
    Select,
    EncodeParts,
    EncodeFasteners,
    Complete,
}

struct Puzzle5dClipboardWork {
    pub raw: Vec<u8>,
    pub raw_cursor: usize,
    pub raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    pub raw_page_len: usize,
    pub progress: u64,
    pub stage: Puzzle5dClipboardStage,
    pub scan: Puzzle5dSelectionScan,
    encode_cursor: usize,
    pub dsl_text: String,
}

enum Puzzle5dClipboardWorkStep {
    Outcome(crate::puzzle_job::JobTurn),
    Pending,
    Complete,
}

impl Puzzle5dClipboardWork {
    pub fn new(raw: Vec<u8>, snapshot: std::sync::Arc<Puzzle5dPlaySnapshot>, interaction: &semio_framework::InteractionState) -> Self {
        Self {
            raw,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dClipboardStage::Envelope,
            scan: Puzzle5dSelectionScan::new(snapshot, interaction),
            encode_cursor: 0,
            dsl_text: String::new(),
        }
    }

    fn step_work(&mut self, cx: &mut StepContext<'_>) -> Result<Puzzle5dClipboardWorkStep, String> {
        match self.stage {
            Puzzle5dClipboardStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return Ok(Puzzle5dClipboardWorkStep::Outcome(outcome));
                }
                self.stage = Puzzle5dClipboardStage::Select;
            }
            Puzzle5dClipboardStage::Select => {
                if !self.scan.step()? {
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text = format!("{{\"schema\":{},\"parts\":[", semio_framework_pack_json::to_json_string(&PUZZLE5D_SCHEMA));
                self.stage = Puzzle5dClipboardStage::EncodeParts;
                self.encode_cursor = 0;
            }
            Puzzle5dClipboardStage::EncodeParts => {
                if let Some(part) = self.scan.parts.get(self.encode_cursor) {
                    if self.encode_cursor != 0 {
                        self.dsl_text.push(',');
                    }
                    self.dsl_text.push_str(&semio_framework_pack_json::to_json_string(part));
                    self.encode_cursor += 1;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text.push_str("],\"fasteners\":[");
                self.stage = Puzzle5dClipboardStage::EncodeFasteners;
                self.encode_cursor = 0;
            }
            Puzzle5dClipboardStage::EncodeFasteners => {
                if let Some(fastener) = self.scan.fasteners.get(self.encode_cursor) {
                    if self.encode_cursor != 0 {
                        self.dsl_text.push(',');
                    }
                    self.dsl_text.push_str(&semio_framework_pack_json::to_json_string(fastener));
                    self.encode_cursor += 1;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text.push_str("]}");
                if self.dsl_text.len() > PUZZLE5D_RESERVED_OUTPUT_BYTES {
                    return Err("puzzle5d clipboard fragment exceeds output cap".into());
                }
                self.stage = Puzzle5dClipboardStage::Complete;
            }
            Puzzle5dClipboardStage::Complete => return Ok(Puzzle5dClipboardWorkStep::Complete),
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        Ok(if self.stage == Puzzle5dClipboardStage::Complete { Puzzle5dClipboardWorkStep::Complete } else { Puzzle5dClipboardWorkStep::Pending })
    }

    pub fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_job_checkpoint(self.stage as u8, self.encode_cursor.max(self.scan.cursor), self.progress)
    }

    pub fn fragment(&self) -> Option<ClipboardFragment> {
        (!self.scan.parts.is_empty()).then(|| ClipboardFragment {
            schema: PUZZLE5D_SCHEMA.to_string(),
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
            dsl_text: self.dsl_text.clone(),
            pack_bytes: None,
            source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
            label: format!("{} part(s)", self.scan.parts.len()),
        })
    }
}

/// 📋️ Which clipboard verb one [`Puzzle5dClipboardJob`] answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle5dClipboardVerb {
    Copy,
    Cut,
}

/// ♻️ The owners one `Puzzle5dClipboardJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Puzzle5dClipboardOwners {
    pub raw: Vec<u8>,
    pub dsl_text: String,
    pub scan: Puzzle5dSelectionScanOwners,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

pub struct Puzzle5dClipboardJob {
    pub verb: Puzzle5dClipboardVerb,
    pub work: Puzzle5dClipboardWork,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    pub completed: bool,
    pub outbox: crate::puzzle_job::JobOutbox,
    pub closing: bool,
    pub owners: crate::puzzle_job::WorkClosing<Puzzle5dClipboardOwners>,
}

impl Puzzle5dClipboardJob {
    pub fn new(verb: Puzzle5dClipboardVerb, request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, interaction: &semio_framework::InteractionState) -> Self {
        Self { verb, work: Puzzle5dClipboardWork::new(request.raw_wire, request.snapshot, interaction), completion: Some(request.completion), rejection: None, completed: false, outbox: Default::default(), closing: false, owners: Default::default() }
    }

    pub fn label(&self) -> &'static str {
        match self.verb {
            Puzzle5dClipboardVerb::Copy => "puzzle5d copy",
            Puzzle5dClipboardVerb::Cut => "puzzle5d cut",
        }
    }

    pub fn emit(&self) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
        let effects = self.work.fragment().map(|fragment| vec![Effect::ClipboardWrite { fragment }]).unwrap_or_default();
        match self.verb {
            Puzzle5dClipboardVerb::Copy => Emit { effects, ..Default::default() },
            Puzzle5dClipboardVerb::Cut => {
                let locked: HashSet<&str> = self.work.scan.parts.iter().filter(|part| part.part_2d.locked.unwrap_or(false)).map(|part| part.id.as_str()).collect();
                let mut mutations = self
                    .work
                    .scan
                    .fasteners
                    .iter()
                    .filter(|fastener| !locked.contains(owning_part_id_local(&fastener.source)) && !locked.contains(owning_part_id_local(&fastener.target)))
                    .map(|fastener| crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener.id.clone()))
                    .collect::<Vec<_>>();
                mutations.extend(self.work.scan.parts.iter().filter(|part| !locked.contains(part.id.as_str())).map(|part| crate::standards::v1::subsets::any::schema::mutations::delete_part(part.id.clone())));
                Emit { artifact_mutations: mutations, effects, ..Default::default() }
            }
        }
    }

    pub fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.rejection.is_some() {
            return puzzle5d_job_fault(format!("{} completion remains rejected", self.label()));
        }
        if self.completed {
            return crate::puzzle_job::JobTurn::Complete;
        }
        match self.work.step_work(cx) {
            Ok(Puzzle5dClipboardWorkStep::Outcome(outcome)) => return outcome,
            Ok(Puzzle5dClipboardWorkStep::Pending) => return self.work.checkpoint(),
            Err(error) => return puzzle5d_job_fault(error),
            Ok(Puzzle5dClipboardWorkStep::Complete) => {}
        }
        if !self.work.raw.is_empty() {
            return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.work.raw));
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault(format!("{} lost its completion authority", self.label())) };
        if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
            let message = rejected.fault.message.clone();
            self.rejection = Some(rejected);
            return puzzle5d_job_fault(message);
        }
        self.completed = true;
        crate::puzzle_job::JobTurn::Complete
    }
}

impl Puzzle5dClipboardJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

impl InteractiveJob for Puzzle5dClipboardJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dClipboardOwners {
            raw: std::mem::take(&mut self.work.raw),
            dsl_text: std::mem::take(&mut self.work.dsl_text),
            scan: self.work.scan.into_owners(),
            completion: self.completion.take(),
            rejection: self.rejection.take(),
        });
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { semio_framework_job::close_original_nested_step(cx, |cx, _grant| {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, cx)
    }) }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle5dClipboardJob {}
//#endregion 🧵️ReservedJobs
