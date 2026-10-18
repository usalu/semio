//! ⏳️ Replay future: the replay phases and transitions and the `HistoryReplayFuture`. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️HistoryReplay
type HistorySegmentLenFuture = Pin<Box<dyn Future<Output = Result<u64, DbError>> + Send + 'static>>;

type HistoryPageReadFuture = Pin<Box<dyn Future<Output = Result<db_io_core::DbIoPages, DbError>> + Send + 'static>>;

type HistorySegmentListFuture = Pin<Box<dyn Future<Output = Result<db_io_core::DbIoU64List, DbError>> + Send + 'static>>;

pub enum HistoryReplayPhase {
    Inventory { future: HistorySegmentListFuture },
    Probe,
    SegmentLen { index: u64, future: HistorySegmentLenFuture },
    PageStart { index: u64, len: u64, offset: u64 },
    PageRead { index: u64, len: u64, offset: u64, requested: u64, future: HistoryPageReadFuture },
    PageClose { index: u64, len: u64, offset: u64, retained: db_io_core::DbIoPages },
    Verify { index: u64 },
    Frame { index: u64 },
    CommittedBody { index: u64 },
    Envelope { index: u64, cursor: HistoryEnvelopeCursor },
    CopyMutation { index: u64, range: std::ops::Range<u64>, copied: u64, result_start: u64 },
    Frontier { index: u64, cursor: HistoryFrontierCursor },
    Publish { index: u64, head_seq: u64, commit_seq: u64, chain_hash: [u8; 32], epoch: u64 },
    Retire,
    InventoryClose,
    FinalizeSuccess,
}

pub enum HistoryReplayTransition {
    InProgress,
    FaultRetire,
    Complete,
}

pub struct HistoryReplayFuture {
    pub storage: Arc<db_storage::DbBackend>,
    pub document: Arc<ArtifactId>,
    pub phase: Option<HistoryReplayPhase>,
    pub transition: HistoryReplayTransition,
    pub pages: HistoryPageSet,
    pub authenticated: Option<db_wal_log::WalAuthenticatedSource<HistoryPageSet>>,
    gate: Option<db_wal_log::WalTransactionGate>,
    previous_tip: db_wal_log::WalPriorChainTip,
    segments: Option<db_io_core::DbIoU64List>,
    segment_ordinal: usize,
    body_index: usize,
    pending_frontier: Option<(u64, u64, [u8; 32], u64)>,
    pub(super) page_count: usize,
    pub reservation: Option<HistoryReplayReservation>,
    reservation_close: Option<HistoryReplayReservationCloseCursor>,
    terminal_page: Option<Vec<u8>>,
    terminal_error: Option<DbError>,
    pub result_len: u64,
    pending_start: usize,
    pub operation_generation: u64,
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub panic_before_transition_commit: bool,
}

impl HistoryReplayFuture {
    pub fn new(storage: Arc<db_storage::DbBackend>, document: ArtifactId, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, mut reservation: HistoryReplayReservation) -> Self {
        let source_pages = std::mem::take(&mut reservation.source_pages);
        reservation.source_page_count = 0;
        let inventory_storage = storage.clone();
        let inventory_document = document.clone();
        Self {
            storage,
            document: Arc::new(document),
            phase: Some(HistoryReplayPhase::Inventory { future: Box::pin(async move { inventory_storage.wal().await.list_segments(&inventory_document).await }) }),
            transition: HistoryReplayTransition::InProgress,
            pages: HistoryPageSet { pages: source_pages, len: 0 },
            authenticated: None,
            gate: Some(db_wal_log::WalTransactionGate::new()),
            previous_tip: db_wal_log::WalPriorChainTip::Genesis,
            segments: None,
            segment_ordinal: 0,
            body_index: 0,
            pending_frontier: None,
            page_count: 0,
            reservation: Some(reservation),
            reservation_close: None,
            terminal_page: None,
            terminal_error: None,
            result_len: 0,
            pending_start: 0,
            operation_generation,
            cancelled,
            panic_before_transition_commit: false,
        }
    }

    pub fn request_close(&mut self, error: DbError) {
        if self.terminal_error.is_none() {
            self.terminal_error = Some(error);
        }
        if !matches!(self.transition, HistoryReplayTransition::Complete) {
            self.transition = HistoryReplayTransition::FaultRetire;
        }
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(source) = self.authenticated.take() {
            self.pages = source.abort_into_source();
            return true;
        }
        if self.terminal_page.take().is_some() {
            return true;
        }
        if self.page_count != 0 {
            self.page_count -= 1;
            drop(self.pages.pages[self.page_count].take());
            return true;
        }
        if !self.pages.pages.is_empty() {
            drop(std::mem::take(&mut self.pages.pages));
            return true;
        }
        if let Some(segments) = self.segments.as_mut() {
            if segments.close_step() {
                return true;
            }
            if !segments.terminal_is_empty() {
                return true;
            }
            self.segments = None;
            return true;
        }
        if self.reservation_close.is_none() {
            if let Some(reservation) = self.reservation.take() {
                self.reservation_close = Some(HistoryReplayReservationCloseCursor::new(reservation));
                return true;
            }
        }
        if self.reservation_close.as_mut().is_some_and(HistoryReplayReservationCloseCursor::close_step) {
            return true;
        }
        if self.reservation_close.as_ref().is_some_and(HistoryReplayReservationCloseCursor::terminal_is_empty) {
            self.reservation_close = None;
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.authenticated.is_none()
            && self.segments.is_none()
            && self.terminal_page.is_none()
            && self.page_count == 0
            && self.pages.pages.is_empty()
            && self.reservation.is_none()
            && self.reservation_close.is_none()
            && self.phase.is_none()
            && matches!(self.transition, HistoryReplayTransition::Complete)
    }

    fn begin_fault(&mut self, error: DbError) {
        self.request_close(error);
    }

    fn fail_pending(&mut self, error: DbError) {
        self.begin_fault(error);
    }

    fn copy_mutation_fragment(pages: &HistoryPageSet, reservation: &mut Option<HistoryReplayReservation>, range: &std::ops::Range<u64>, copied: u64, result_start: u64) -> Result<u64, DbError> {
        let remaining = range.end.checked_sub(range.start).and_then(|len| len.checked_sub(copied)).ok_or(DbError::LimitExceeded("history result copy range"))?;
        if remaining == 0 {
            return Ok(0);
        }
        let source_offset = range.start.checked_add(copied).ok_or(DbError::LimitExceeded("history result source offset"))?;
        let destination_offset = result_start.checked_add(copied).ok_or(DbError::LimitExceeded("history result destination offset"))?;
        let source = pages.page_slice(source_offset, remaining.min(HISTORY_REPLAY_PAGE_BYTES))?;
        let page_index = usize::try_from(destination_offset / HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history result page index"))?;
        let within = usize::try_from(destination_offset % HISTORY_REPLAY_PAGE_BYTES).map_err(|_| DbError::LimitExceeded("history result page offset"))?;
        let reservation = reservation.as_mut().ok_or(DbError::Closed)?;
        let destination = reservation.result_pages.get_mut(page_index).and_then(Option::as_mut).ok_or_else(|| DbError::Corrupt("history result page owner is missing".to_string()))?;
        let take = source.len().min(destination.len().saturating_sub(within));
        if take == 0 {
            return Err(DbError::Corrupt("history result copy made no progress".to_string()));
        }
        destination[within..within + take].copy_from_slice(&source[..take]);
        Ok(take as u64)
    }

    fn finish_view(&mut self) -> Result<ArtifactHistoryView, DbError> {
        let Some(reservation) = self.reservation.as_ref() else {
            return Err(DbError::Closed);
        };
        let retained_bytes = reservation.retained_result_bytes;
        let retained_items = reservation
            .result_pages
            .len()
            .checked_add(reservation.operation_ids.len())
            .and_then(|items| items.checked_add(reservation.entries.len()))
            .and_then(|items| items.checked_add(1))
            .ok_or(DbError::LimitExceeded("history retained result items"));
        let retained_items = retained_items?;
        let Some(reservation) = self.reservation.take() else {
            return Err(DbError::Closed);
        };
        if reservation.scratch.is_some() || !reservation.source_pages.is_empty() {
            self.reservation = Some(reservation);
            return Err(DbError::Internal("history final view retained non-result owners".to_string()));
        }
        Ok(ArtifactHistoryView { entries: reservation.entries, operation_ids: reservation.operation_ids, result_pages: reservation.result_pages, result_len: self.result_len, retained_bytes, retained_items })
    }

    pub fn next(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<ArtifactHistoryView, DbError>> {
        let this = self.get_mut();
        if this.operation_generation == 0 {
            this.request_close(DbError::Closed);
        }
        if this.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            this.request_close(DbError::Closed);
        }
        if matches!(this.transition, HistoryReplayTransition::Complete) {
            return std::task::Poll::Ready(Err(this.terminal_error.take().unwrap_or(DbError::Closed)));
        }
        if matches!(this.transition, HistoryReplayTransition::FaultRetire) {
            if this.phase.take().is_some() {
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            if this.close_step() {
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            this.transition = HistoryReplayTransition::Complete;
            return std::task::Poll::Ready(Err(this.terminal_error.take().unwrap_or(DbError::Closed)));
        }
        let mut next = None;
        let mut fault = None;
        let mut finalize = false;
        let Some(active) = this.phase.as_mut() else {
            this.request_close(DbError::Internal("history replay transition authority was missing".to_string()));
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        };
        match active {
            HistoryReplayPhase::Inventory { future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(error)) => fault = Some(error),
                std::task::Poll::Ready(Ok(segments)) => {
                    this.segments = Some(segments);
                    next = Some(HistoryReplayPhase::Probe);
                }
            },
            HistoryReplayPhase::Probe => {
                if let Some(error) = this.terminal_error.take() {
                    fault = Some(error);
                } else {
                    let segments = this.segments.as_ref().map(db_io_core::DbIoU64List::as_slice).unwrap_or(&[]);
                    match segments.get(this.segment_ordinal).copied() {
                        None => next = Some(HistoryReplayPhase::InventoryClose),
                        Some(segment) if segments.first().and_then(|first| first.checked_add(this.segment_ordinal as u64)) != Some(segment) => fault = Some(DbError::Corrupt("history WAL inventory is not a dense retained suffix".to_string())),
                        Some(segment) => {
                            if this.segment_ordinal == 0 && segment != 0 {
                                this.previous_tip = db_wal_log::WalPriorChainTip::RetainedBoundary;
                            }
                            let storage = this.storage.clone();
                            let document = this.document.clone();
                            next = Some(HistoryReplayPhase::SegmentLen { index: segment, future: Box::pin(async move { storage.wal().await.segment_len(&document, segment).await }) });
                        }
                    }
                }
            }
            HistoryReplayPhase::SegmentLen { index, future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(error)) => fault = Some(this.terminal_error.take().unwrap_or(error)),
                std::task::Poll::Ready(Ok(len)) => {
                    if let Some(error) = this.terminal_error.take() {
                        fault = Some(error);
                    } else if let Some(page_count) = len.checked_add(HISTORY_REPLAY_PAGE_BYTES - 1).map(|bytes| bytes / HISTORY_REPLAY_PAGE_BYTES) {
                        let inventory_bytes = this.segments.as_ref().map_or(0, db_io_core::DbIoU64List::retained_bytes);
                        let simultaneous = this
                            .reservation
                            .as_ref()
                            .and_then(HistoryReplayReservation::retained_bytes)
                            .and_then(|bytes| bytes.checked_add(len))
                            .and_then(|bytes| bytes.checked_add(inventory_bytes))
                            .and_then(|bytes| bytes.checked_add(size_of::<Self>() as u64));
                        if len < protocol::format::HEADER_SIZE as u64 || page_count > HISTORY_REPLAY_SEGMENT_PAGES || simultaneous.is_none_or(|bytes| bytes > HISTORY_REPLAY_OPERATION_BYTES) {
                            fault = Some(DbError::LimitExceeded("history replay page/source byte credit"));
                        } else {
                            this.pages.len = len;
                            next = Some(HistoryReplayPhase::PageStart { index: *index, len, offset: 0 });
                        }
                    } else {
                        fault = Some(DbError::LimitExceeded("history segment page rounding"));
                    }
                }
            },
            HistoryReplayPhase::PageStart { index, len, offset } => {
                if offset == len {
                    if let Some(gate) = this.gate.take() {
                        this.authenticated = Some(db_wal_log::WalAuthenticatedSource::new(std::mem::take(&mut this.pages), gate, *index, this.previous_tip));
                        next = Some(HistoryReplayPhase::Verify { index: *index });
                    } else {
                        fault = Some(DbError::Closed);
                    }
                } else {
                    match len.checked_sub(*offset) {
                        Some(remaining) => {
                            let requested = remaining.min(HISTORY_REPLAY_PAGE_BYTES);
                            let storage = this.storage.clone();
                            let document = this.document.clone();
                            let (index, len, offset) = (*index, *len, *offset);
                            next = Some(HistoryReplayPhase::PageRead {
                                index,
                                len,
                                offset,
                                requested,
                                future: Box::pin(async move { storage.wal().await.read(&document, index, pack::ByteRange { offset, len: requested }).await }),
                            });
                        }
                        None => fault = Some(DbError::LimitExceeded("history page remaining bytes")),
                    }
                }
            }
            HistoryReplayPhase::PageRead { index, len, offset, requested, future } => match future.as_mut().poll(context) {
                std::task::Poll::Pending => {}
                std::task::Poll::Ready(Err(error)) => fault = Some(this.terminal_error.take().unwrap_or(error)),
                std::task::Poll::Ready(Ok(retained)) => {
                    let mut page = Vec::with_capacity(retained.len());
                    for fragment in retained.fragments() {
                        page.extend_from_slice(fragment);
                    }
                    let page = page.into_boxed_slice().into_vec();
                    if let Some(error) = this.terminal_error.take() {
                        this.terminal_page = Some(page);
                        fault = Some(error);
                    } else if page.len() as u64 != *requested || page.capacity() as u64 > HISTORY_REPLAY_PAGE_BYTES || this.page_count >= HISTORY_REPLAY_SEGMENT_PAGES as usize {
                        this.terminal_page = Some(page);
                        fault = Some(DbError::LimitExceeded("history backend page ownership"));
                    } else if let Some(next_offset) = offset.checked_add(*requested) {
                        this.pages.pages[this.page_count] = Some(page);
                        this.page_count += 1;
                        next = Some(HistoryReplayPhase::PageClose { index: *index, len: *len, offset: next_offset, retained });
                    } else {
                        this.terminal_page = Some(page);
                        fault = Some(DbError::LimitExceeded("history page offset"));
                    }
                }
            },
            HistoryReplayPhase::PageClose { index, len, offset, retained } => {
                if retained.terminal_is_empty() {
                    next = Some(HistoryReplayPhase::PageStart { index: *index, len: *len, offset: *offset });
                } else if let Err(error) = retained.close_step() {
                    fault = Some(error);
                }
            }
            HistoryReplayPhase::Verify { index } => {
                let result = db_wal_log::WalCursorControl::new(this.cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_millis(8), 1)
                    .and_then(|mut control| this.authenticated.as_mut().ok_or(DbError::Closed)?.verify_step(&this.document, &mut control));
                match result {
                    Ok(true) => next = Some(HistoryReplayPhase::Frame { index: *index }),
                    Ok(false) => {}
                    Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached" => {}
                    Err(error) => fault = Some(error),
                }
            }
            HistoryReplayPhase::Frame { index } => {
                let result =
                    db_wal_log::WalCursorControl::new(this.cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_millis(8), 1).and_then(|mut control| this.authenticated.as_mut().ok_or(DbError::Closed)?.next_step(&mut control));
                match result {
                    Ok(db_wal_log::WalAuthenticatedStep::Yield) => {}
                    Ok(db_wal_log::WalAuthenticatedStep::Committed) => {
                        this.body_index = 0;
                        this.pending_frontier = None;
                        next = Some(HistoryReplayPhase::CommittedBody { index: *index });
                    }
                    Ok(db_wal_log::WalAuthenticatedStep::Done) => {
                        if let Some(source) = this.authenticated.take() {
                            match source.finish() {
                                Ok((pages, gate, tip)) => {
                                    this.pages = pages;
                                    this.gate = Some(gate);
                                    this.previous_tip = db_wal_log::WalPriorChainTip::Verified(tip);
                                    next = Some(HistoryReplayPhase::Retire);
                                }
                                Err(source) => {
                                    this.authenticated = Some(source);
                                    fault = Some(DbError::Corrupt("history source finished without authentication".to_string()));
                                }
                            }
                        } else {
                            fault = Some(DbError::Closed);
                        }
                    }
                    Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached" => {}
                    Err(error) => fault = Some(error),
                }
            }
            HistoryReplayPhase::CommittedBody { index } => {
                if let Some(source) = this.authenticated.as_ref() {
                    if let Some(frame) = source.committed_frame(this.body_index) {
                        this.body_index += 1;
                        let (offset, len) = (frame.payload_start as u64, (frame.payload_end - frame.payload_start) as u64);
                        match frame.kind {
                            _ if this.pending_frontier.is_some() => fault = Some(DbError::Corrupt("history committed frontier is not terminal".to_string())),
                            db_wal_log::WAL_COMMAND => match HistoryEnvelopeCursor::new(source.source(), offset, len) {
                                Ok(cursor) => next = Some(HistoryReplayPhase::Envelope { index: *index, cursor }),
                                Err(error) => fault = Some(error),
                            },
                            db_wal_log::WAL_FRONTIER => match HistoryFrontierCursor::new(source.source(), offset, len) {
                                Ok(cursor) => next = Some(HistoryReplayPhase::Frontier { index: *index, cursor }),
                                Err(error) => fault = Some(error),
                            },
                            _ => {}
                        }
                    } else if let Some((head_seq, commit_seq, chain_hash, epoch)) = this.pending_frontier.take() {
                        next = Some(HistoryReplayPhase::Publish { index: *index, head_seq, commit_seq, chain_hash, epoch });
                    } else if this.reservation.as_ref().is_some_and(|owner| owner.operation_ids.len() != this.pending_start) {
                        fault = Some(DbError::Corrupt("history committed commands have no frontier".to_string()));
                    } else {
                        match this.authenticated.as_mut().ok_or(DbError::Closed).and_then(|source| source.finish_transaction()) {
                            Ok(()) => next = Some(HistoryReplayPhase::Frame { index: *index }),
                            Err(error) => fault = Some(error),
                        }
                    }
                } else {
                    fault = Some(DbError::Closed);
                }
            }
            HistoryReplayPhase::Envelope { index, cursor } => {
                let step = match this.reservation.as_mut().and_then(|owner| owner.scratch.as_mut()) {
                    Some(scratch) => this.authenticated.as_ref().ok_or(DbError::Closed).and_then(|source| cursor.step(source.source(), scratch, &this.document)),
                    None => Err(DbError::Closed),
                };
                match step {
                    Err(error) => fault = Some(error),
                    Ok(Some(range)) => {
                        let len = range.end.saturating_sub(range.start);
                        match this.reservation.as_ref().map_or(Err(DbError::Closed), |owner| owner.preflight_result_range(this.result_len, len)) {
                            Ok(_) => next = Some(HistoryReplayPhase::CopyMutation { index: *index, range, copied: 0, result_start: this.result_len }),
                            Err(error) => fault = Some(error),
                        }
                    }
                    Ok(None) => {}
                }
            }
            HistoryReplayPhase::CopyMutation { index, range, copied, result_start } => {
                match this.authenticated.as_ref().ok_or(DbError::Closed).and_then(|source| Self::copy_mutation_fragment(source.source(), &mut this.reservation, range, *copied, *result_start)) {
                    Err(error) => fault = Some(error),
                    Ok(0) => {
                        if let Some(len) = range.end.checked_sub(range.start).and_then(|value| u16::try_from(value).ok()) {
                            if let Some(reservation) = this.reservation.as_mut() {
                                reservation.operation_ids.push(HistoryTextRange { start: *result_start as u32, len });
                                this.result_len = *result_start + u64::from(len);
                                next = Some(HistoryReplayPhase::CommittedBody { index: *index });
                            } else {
                                fault = Some(DbError::Closed);
                            }
                        } else {
                            fault = Some(DbError::LimitExceeded("history operation id range"));
                        }
                    }
                    Ok(count) => *copied += count,
                }
            }
            HistoryReplayPhase::Frontier { index, cursor } => {
                let step = match this.reservation.as_mut().and_then(|owner| owner.scratch.as_mut()) {
                    Some(scratch) => this.authenticated.as_ref().ok_or(DbError::Closed).and_then(|source| cursor.step(source.source(), scratch, &this.document)),
                    None => Err(DbError::Closed),
                };
                match step {
                    Err(error) => fault = Some(error),
                    Ok(Some(frontier)) => {
                        this.pending_frontier = Some(frontier);
                        next = Some(HistoryReplayPhase::CommittedBody { index: *index });
                    }
                    Ok(None) => {}
                }
            }
            HistoryReplayPhase::Publish { index, head_seq, commit_seq, chain_hash, epoch } => {
                let operation_end = this.reservation.as_ref().map_or(this.pending_start, |owner| owner.operation_ids.len());
                if operation_end == this.pending_start {
                    next = Some(HistoryReplayPhase::Frame { index: *index });
                } else if this.reservation.as_ref().is_none_or(|owner| owner.entries.len() >= HISTORY_REPLAY_MAX_ENTRIES) {
                    fault = Some(DbError::LimitExceeded("history entry item credit"));
                } else {
                    let count = operation_end - this.pending_start;
                    let start = u16::try_from(this.pending_start);
                    let count = u16::try_from(count);
                    match (start, count, this.reservation.as_mut()) {
                        (Ok(operation_start), Ok(operation_count), Some(reservation)) => {
                            reservation.entries.push(ArtifactHistoryEntry { operation_start, operation_count, head_seq: *head_seq, commit_seq: *commit_seq, chain_hash: *chain_hash, epoch: *epoch });
                            this.pending_start = operation_end;
                            next = Some(HistoryReplayPhase::Frame { index: *index });
                        }
                        _ => fault = Some(DbError::LimitExceeded("history entry range")),
                    }
                }
                if fault.is_none() {
                    if let Err(error) = this.authenticated.as_mut().ok_or(DbError::Closed).and_then(|source| source.finish_transaction()) {
                        fault = Some(error);
                    }
                }
            }
            HistoryReplayPhase::Retire => {
                if this.page_count != 0 {
                    this.page_count -= 1;
                    drop(this.pages.pages[this.page_count].take());
                } else {
                    this.pages.len = 0;
                    this.segment_ordinal += 1;
                    next = Some(HistoryReplayPhase::Probe);
                }
            }
            HistoryReplayPhase::InventoryClose => {
                if let Some(segments) = this.segments.as_mut() {
                    segments.close_step();
                    if segments.terminal_is_empty() {
                        this.segments = None;
                    }
                } else {
                    next = Some(HistoryReplayPhase::FinalizeSuccess);
                }
            }
            HistoryReplayPhase::FinalizeSuccess => {
                if this.reservation.as_mut().and_then(|owner| owner.scratch.take()).is_some() {
                } else if !this.pages.pages.is_empty() {
                    drop(std::mem::take(&mut this.pages.pages));
                } else {
                    finalize = true;
                }
            }
        }
        if std::mem::take(&mut this.panic_before_transition_commit) {
            panic!("history replay transition panic fixture");
        }
        if let Some(error) = fault {
            if this.terminal_error.is_none() {
                this.terminal_error = Some(error);
            }
            this.transition = HistoryReplayTransition::FaultRetire;
        } else if finalize {
            match this.finish_view() {
                Ok(view) => {
                    this.phase = None;
                    this.transition = HistoryReplayTransition::Complete;
                    return std::task::Poll::Ready(Ok(view));
                }
                Err(error) => {
                    this.terminal_error.get_or_insert(error);
                    this.transition = HistoryReplayTransition::FaultRetire;
                }
            }
        } else if let Some(next) = next {
            this.phase = Some(next);
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl Future for HistoryReplayFuture {
    type Output = Result<ArtifactHistoryView, DbError>;

    fn poll(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.next(context)
    }
}

impl Drop for HistoryReplayFuture {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "history replay reached Drop before retained source/result/page retirement");
    }
}
//#endregion 🔖️HistoryReplay
//#endregion 🔖️Ids
