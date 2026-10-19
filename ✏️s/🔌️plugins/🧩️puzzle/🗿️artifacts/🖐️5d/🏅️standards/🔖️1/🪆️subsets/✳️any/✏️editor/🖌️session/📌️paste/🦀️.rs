//! 📌️ Paste job of the puzzle 5d editor session: the reserved paste job with its fixed-page state machine. See [`super`].
use semio_framework_job::InteractiveJob;
use semio_framework_plugin::ArtifactReservedJob;

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use semio_framework_plugin::{kernel::ClipboardFragment, kernel::PasteAnchor, kernel::PastePlacement};
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_value::FromValue;
use semio_framework_pack_json::{parse, Value};
use std::collections::HashMap;
use crate::editor::puzzle5d::session::support::{puzzle5d_job_checkpoint, puzzle5d_job_fault, puzzle5d_step_envelope};
use crate::editor::puzzle5d::Puzzle5dPlayApp;
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRANULARITY_PART, PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_RESERVED_PAGE_BYTES, PUZZLE5D_RESERVED_RAW_BYTES, Puzzle5dFastener, Puzzle5dFreshIds, Puzzle5dPart, puzzle5d_paste_placement, puzzle5d_record_from_projection};
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::interaction::{rewrite_grip_ref_local};

//#region 🧵️ReservedJobs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPasteStage {
    Envelope,
    Decode,
    FragmentParts,
    TargetParts,
    TargetFasteners,
    MaterializeParts,
    MaterializeFasteners,
    Complete,
}

/// ♻️ The owners one `Puzzle5dPasteJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPasteOwners {
    pub raw: Vec<u8>,
    pub snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub args: Option<Value>,
    pub fragment_value: Option<Value>,
    pub fragment_parts: Vec<Puzzle5dPart>,
    pub id_map: HashMap<String, String>,
    pub fresh_ids: Puzzle5dFreshIds,
    pub mutations: Vec<Puzzle5dMutation>,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pub pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

pub struct Puzzle5dPasteJob {
    pub raw: Vec<u8>,
    pub raw_cursor: usize,
    pub raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    pub raw_page_len: usize,
    pub progress: u64,
    pub stage: Puzzle5dPasteStage,
    pub snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub args: Option<Value>,
    pub fragment_value: Option<Value>,
    pub fragment_parts: Vec<Puzzle5dPart>,
    pub cursor: usize,
    source_sum: (f64, f64),
    target_sum: (f64, f64),
    target_count: usize,
    pub placement: PastePlacement,
    pub delta: (f64, f64),
    pub id_map: HashMap<String, String>,
    pub fresh_ids: Puzzle5dFreshIds,
    pub mutations: Vec<Puzzle5dMutation>,
    pub completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pub pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    pub completed: bool,
    pub outbox: crate::puzzle_job::JobOutbox,
    pub closing: bool,
    pub owners: crate::puzzle_job::WorkClosing<Puzzle5dPasteOwners>,
}

impl Puzzle5dPasteJob {
    pub fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, args: Option<Value>) -> Self {
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dPasteStage::Envelope,
            snapshot: Some(request.snapshot),
            args,
            fragment_value: None,
            fragment_parts: Vec::new(),
            cursor: 0,
            source_sum: (0.0, 0.0),
            target_sum: (0.0, 0.0),
            target_count: 0,
            placement: PastePlacement::default(),
            delta: (0.0, 0.0),
            id_map: HashMap::new(),
            fresh_ids: Puzzle5dFreshIds::default(),
            mutations: Vec::new(),
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    pub fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_job_checkpoint(self.stage as u8, self.cursor, self.progress)
    }
}

impl Puzzle5dPasteJob {
    pub fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault("puzzle5d paste completion remains rejected");
        }
        match self.stage {
            Puzzle5dPasteStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return outcome;
                }
                self.stage = Puzzle5dPasteStage::Decode;
            }
            Puzzle5dPasteStage::Decode => {
                let Some(args) = self.args.as_ref() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint();
                };
                let Some(fragment_value) = args.get("fragment").cloned() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint();
                };
                let fragment: ClipboardFragment = match puzzle5d_record_from_projection(fragment_value) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                if fragment.media_type != (MediaType { class: MediaClass::Kit, form: MediaForm::Design }) {
                    return puzzle5d_job_fault("puzzle5d paste received an incompatible media type");
                }
                if fragment.dsl_text.len() > PUZZLE5D_RESERVED_RAW_BYTES {
                    return puzzle5d_job_fault("puzzle5d paste fragment exceeds its predecode cap");
                }
                self.fragment_value = match parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                    Ok(value) => Some(value),
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.placement = match puzzle5d_paste_placement(args) {
                    Ok(placement) => placement,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.stage = Puzzle5dPasteStage::FragmentParts;
                self.cursor = 0;
            }
            Puzzle5dPasteStage::FragmentParts => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let part: Puzzle5dPart = match puzzle5d_record_from_projection(row) {
                        Ok(part) => part,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.source_sum.0 += part.part_2d.x;
                    self.source_sum.1 += part.part_2d.y;
                    self.fragment_parts.push(part);
                } else {
                    self.stage = Puzzle5dPasteStage::TargetParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetParts => {
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().parts).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_part(&row.id);
                    self.target_sum.0 += row.part_2d.x;
                    self.target_sum.1 += row.part_2d.y;
                    self.target_count += 1;
                } else {
                    let offset = self.placement.position.map_or((0.0, 0.0), |position| (position[0], position[1]));
                    self.delta = if matches!(self.placement.anchor, PasteAnchor::Original) || self.fragment_parts.is_empty() || self.target_count == 0 {
                        offset
                    } else {
                        (self.target_sum.0 / self.target_count as f64 - self.source_sum.0 / self.fragment_parts.len() as f64 + offset.0, self.target_sum.1 / self.target_count as f64 - self.source_sum.1 / self.fragment_parts.len() as f64 + offset.1)
                    };
                    self.stage = Puzzle5dPasteStage::TargetFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetFasteners => {
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().fasteners).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_fastener(&row.id);
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeParts => {
                if let Some(part) = self.fragment_parts.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fresh_id = self.fresh_ids.next_part();
                    self.id_map.insert(part.id.clone(), fresh_id.clone());
                    let mut next = part;
                    next.id = fresh_id;
                    next.part_2d.x += self.delta.0;
                    next.part_2d.y += self.delta.1;
                    next.part_3d.origin[0] += self.delta.0;
                    next.part_3d.origin[1] += self.delta.1;
                    let typed = match <crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart as FromValue>::from_value(next.to_value()) {
                        Ok(typed) => typed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(typed, None));
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeFasteners => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("fasteners")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fastener: Puzzle5dFastener = match puzzle5d_record_from_projection(row) {
                        Ok(fastener) => fastener,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                        self.fresh_ids.next_fastener(),
                        rewrite_grip_ref_local(&fastener.source, &self.id_map),
                        rewrite_grip_ref_local(&fastener.target, &self.id_map),
                        fastener.fastener_kind,
                        fastener.gap,
                        fastener.shift,
                        fastener.rise,
                        fastener.rotation,
                        fastener.turn,
                        fastener.tilt,
                        fastener.x + self.delta.0,
                        fastener.y + self.delta.1, None,
                    ));
                } else {
                    self.stage = Puzzle5dPasteStage::Complete;
                }
            }
            Puzzle5dPasteStage::Complete => {
                if !self.raw.is_empty() {
                    return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw));
                }
                if !self.completed {
                    // 🕹️ The pasted parts BECOME the selection: a paste whose result is invisible until the
                    // user hunts for it is indistinguishable from one that landed nowhere, and the fresh ids
                    // are the only handle on it. Sorted, so one fragment always re-selects in one order.
                    let mut fresh: Vec<String> = self.id_map.values().cloned().collect();
                    fresh.sort();
                    let emit = Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, fresh)], ..Default::default() };
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault("puzzle5d paste lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle5d_job_fault(message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}

impl Puzzle5dPasteJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }
}

impl InteractiveJob for Puzzle5dPasteJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dPasteOwners {
            raw: std::mem::take(&mut self.raw),
            snapshot: self.snapshot.take(),
            args: self.args.take(),
            fragment_value: self.fragment_value.take(),
            fragment_parts: std::mem::take(&mut self.fragment_parts),
            id_map: std::mem::take(&mut self.id_map),
            fresh_ids: std::mem::take(&mut self.fresh_ids),
            mutations: std::mem::take(&mut self.mutations),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.begin_close();
            crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
        })
    }

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

impl ArtifactReservedJob for Puzzle5dPasteJob {}
//#endregion 🧵️ReservedJobs
