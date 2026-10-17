//! 🔄️ Checkpoint restore cursor of the LDLT job.

use super::*;

pub struct LdltRestoreCursor {
    pub operation: Operation,
    pub payload: Option<RetainedJobPayload>,
    pub total_pages: usize,
    pub page_slot: usize,
    pub close_due: bool,
    pub expected_field: u16,
    pub page_entry: usize,
    pub owner_cursor: NumericalOwnerRestoreCursor,
    pub control: [u64; 32],
    pub state: Option<LdltCheckpoint>,
    pub fault: Option<NumericalCheckpointFault>,
}

impl LdltRestoreCursor {
    pub fn new(operation: Operation, payload: RetainedJobPayload) -> Self {
        let total_pages = payload.page_count();
        Self { operation, payload: Some(payload), total_pages, page_slot: 0, close_due: false, expected_field: 0, page_entry: 0, owner_cursor: NumericalOwnerRestoreCursor::default(), control: [0; 32], state: None, fault: None }
    }

    fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault> {
        let page = parse_numerical_page(bytes, b"FEMLCP1\0")?;
        if page.kind != 11 || page.field != self.expected_field || (self.owner_cursor.shape.is_none() && (page.owner != 0 || page.item != 0)) {
            return Err(NumericalCheckpointFault::Field);
        }
        if page.field == 0 {
            let count = declared_owner_length(&page, 32)?;
            if count != 26 || page.bytes.len() != 8 + count * 8 {
                return Err(NumericalCheckpointFault::Truncated);
            }
            if self.page_entry == 0 {
                self.page_entry = 1;
                return Ok(false);
            }
            let item = self.page_entry - 1;
            if item < count {
                self.control[item] = read_checkpoint_u64(page.bytes, 8 + item * 8)?;
                self.page_entry += 1;
                return Ok(false);
            }
            let value = |index| self.control[index];
            let identity = NumericalCheckpointIdentity { operation: value(0), revision: value(1), generation: value(2), seed: value(3) };
            if !identity.matches(self.operation) {
                return Err(NumericalCheckpointFault::Stale);
            }
            let n = value(4) as usize;
            if n > LDLT_MAXIMUM_ORDER {
                return Err(NumericalCheckpointFault::Envelope);
            }
            let stage = match value(6) {
                0 => LdltColumnStage::ReserveColumn,
                1 => LdltColumnStage::SourceEntry,
                2 => LdltColumnStage::ContributorLookup,
                3 => LdltColumnStage::ContributorEntry,
                4 => LdltColumnStage::PivotRead,
                5 => LdltColumnStage::DiagonalCommit,
                6 => LdltColumnStage::EmitRow,
                7 => LdltColumnStage::PublishColumn,
                8 => LdltColumnStage::CompleteColumn,
                _ => return Err(NumericalCheckpointFault::Field),
            };
            self.state = Some(LdltCheckpoint {
                identity,
                a: CscSym { n, colptr: Vec::new(), rowind: Vec::new(), vals: Vec::new() },
                l_cols: Vec::new(),
                d: Vec::new(),
                row_lists: Vec::new(),
                column: value(5) as usize,
                cursor: LdltColumnCursor {
                    stage,
                    source: value(7) as usize,
                    contributor: value(8) as usize,
                    entry: value(9) as usize,
                    emit_row: value(10) as usize,
                    active_column: value(11) as usize,
                    lookup_lower: value(21) as usize,
                    lookup_upper: value(22) as usize,
                    lookup_mid: value(23) as usize,
                    lookup_comparison: value(24) as i64 as i8,
                    lookup_initialized: value(25) != 0,
                    factor: f64::from_bits(value(12)),
                    pivot: f64::from_bits(value(13)),
                },
                workspace: LdltColumnWorkspace { values: Vec::new(), marks: Vec::new(), generation: value(20) as u32, candidate: Vec::new() },
                admission_fault: value(14) != 0,
                reserve_lane: value(15) as u8,
                reserve_cursor: value(16) as usize,
                checkpoint_due: false,
                publication_stage: value(17) as u8,
                publication_outer: value(18) as usize,
                publication_inner: value(19) as usize,
            });
            self.expected_field = 1;
            return Ok(true);
        }
        let state = self.state.as_mut().ok_or(NumericalCheckpointFault::Field)?;
        let n = state.a.n;
        let row_base = 520u16;
        let complete = match page.field {
            1 => restore_u32_entry(&mut state.a.colptr, &page, n + 1, &mut self.page_entry, &mut self.owner_cursor)?,
            2 => restore_u32_entry(&mut state.a.rowind, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor)?,
            3 => restore_f64_entry(&mut state.a.vals, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor)?,
            4 => {
                let length = declared_owner_length(&page, n)?;
                if page.bytes.len() != 8 {
                    return Err(NumericalCheckpointFault::Field);
                }
                if self.page_entry == 0 {
                    state.l_cols.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
                    validate_restored_owner(&state.l_cols)?;
                    self.page_entry = 1;
                    return Ok(false);
                }
                if state.l_cols.len() < length {
                    state.l_cols.push(Vec::new());
                    self.page_entry += 1;
                    return Ok(false);
                }
                true
            }
            field if field >= 5 && field < 5 + state.l_cols.len() as u16 => restore_pair_entry(&mut state.l_cols[(field - 5) as usize], &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            517 => restore_f64_entry(&mut state.d, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            518 => {
                let length = declared_owner_length(&page, n)?;
                if page.bytes.len() != 8 {
                    return Err(NumericalCheckpointFault::Field);
                }
                if self.page_entry == 0 {
                    state.row_lists.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
                    validate_restored_owner(&state.row_lists)?;
                    self.page_entry = 1;
                    return Ok(false);
                }
                if state.row_lists.len() < length {
                    state.row_lists.push(Vec::new());
                    self.page_entry += 1;
                    return Ok(false);
                }
                true
            }
            field if field >= row_base && field < row_base + state.row_lists.len() as u16 => restore_usize_entry(&mut state.row_lists[(field - row_base) as usize], &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            1032 => restore_f64_entry(&mut state.workspace.values, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            1033 => restore_u32_entry(&mut state.workspace.marks, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            1034 => restore_pair_entry(&mut state.workspace.candidate, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            _ => return Err(NumericalCheckpointFault::Field),
        };
        if complete && self.owner_cursor.shape.is_none() {
            self.expected_field = match page.field {
                4 if state.l_cols.is_empty() => 517,
                4 => 5,
                field if field >= 5 && field + 1 < 5 + state.l_cols.len() as u16 => field + 1,
                field if field >= 5 && field < 5 + state.l_cols.len() as u16 => 517,
                517 => 518,
                518 if state.row_lists.is_empty() => 1032,
                518 => row_base,
                field if field >= row_base && field + 1 < row_base + state.row_lists.len() as u16 => field + 1,
                field if field >= row_base && field < row_base + state.row_lists.len() as u16 => 1032,
                1032 | 1033 => page.field + 1,
                1034 => u16::MAX,
                field => field + 1,
            };
        }
        Ok(complete)
    }

    pub fn step(&mut self, context: &mut StepContext<'_>) -> Result<Option<LdltJob>, NumericalCheckpointFault> {
        if let Some(fault) = self.fault { return Err(fault); }
        let outcome = self.advance(context);
        if let Err(fault) = outcome { self.fault = Some(fault); }
        outcome
    }

    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<Option<LdltJob>, NumericalCheckpointFault> {
        if context.is_cancelled() {
            return Err(NumericalCheckpointFault::Cancelled);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Err(NumericalCheckpointFault::Stale);
        }
        if context.should_yield() {
            return Ok(None);
        }
        context.consume_fuel(1);
        if self.close_due {
            let payload = self.payload.as_mut().ok_or(NumericalCheckpointFault::Truncated)?;
            if numerical_restore_close_due(payload, context, &mut self.page_slot, &mut self.page_entry, &mut self.close_due, self.total_pages)? {
                if self.expected_field != u16::MAX {
                    return Err(NumericalCheckpointFault::Truncated);
                }
                self.payload = None;
                let state = self.state.take().ok_or(NumericalCheckpointFault::Truncated)?;
                return Ok(Some(LdltJob { operation: self.operation, state, output_writer: None, output_page_cursor: 0, checkpoint_writer: None, checkpoint_cursor: NumericalPageCursor::new(), desk: NumericalOutcomeDesk::default() }));
            }
            return Ok(None);
        }
        let payload = self.payload.take().ok_or(NumericalCheckpointFault::Truncated)?;
        let decoded = payload.page(self.page_slot).ok_or(NumericalCheckpointFault::Truncated).and_then(|source| self.decode_page_entry(source));
        self.payload = Some(payload);
        match decoded {
            Ok(true) => self.close_due = true,
            Ok(false) => {}
            Err(fault) => return Err(fault),
        }
        Ok(None)
    }

    pub(super) fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        numerical_restore_frontier(&self.payload)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let frontier = self.close_frontier()?;
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        if !numerical_close_gate(grant, frontier) {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        if let Some(step) = numerical_restore_payload_close(&mut self.payload, grant) {
            return step?.admit(grant, self.terminal_is_empty());
        }
        let Some(state) = self.state.as_mut() else { return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }) };
        let step = match numerical_owner_turn(close_ldlt_checkpoint_step(state, grant.maximum_release_bytes)) {
            Some(step) => step,
            None => {
                self.state = None;
                InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }
            }
        };
        step.admit(grant, self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.is_none() && self.state.is_none()
    }
}
