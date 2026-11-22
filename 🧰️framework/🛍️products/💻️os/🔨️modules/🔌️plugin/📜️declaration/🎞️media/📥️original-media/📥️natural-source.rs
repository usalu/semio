//! 🧳 Holds the unpublished natural file below its exact Original command Source.

use super::{
    command_owner::{PendingPage, copy_demand, moved, paid},
    guest::{run, validate_source},
    natural_source_gate::{
        NaturalSourceBegin, NaturalSourceCommand,
        NaturalSourceDeclaration, NaturalSourceLoan, NaturalSourceOffer, NaturalSourceOperation,
        NaturalSourcePage, PreparedNaturalSource, Stage,
    },
    receiver::PagedMediaSource,
};
use semio_framework_job::{OriginalSourceBinding, OriginalTurnFailure, StepContext, close_original_effect_step};
use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage};
use semio_framework_value::{
    RetainedCloneProgress, paged::PagedBytes, retirement::controlled::ControlledRetirement,
};

pub use super::natural_source_gate::NaturalSourceStatus;

pub struct NaturalSourceGuest<const N: usize> {
    grant: OriginalSourceBinding,
    issued: PreparedNaturalSource<'static>,
    seq: u64,
    stage: Stage,
    command: Option<NaturalSourceCommand<'static>>,
    source: Option<PagedMediaSource<N>>,
    pending: Option<PendingPage>,
    next: u32,
    terminal_receipt_sent: bool,
}

type Input<const N: usize> = ControlledRetirement<PagedBytes<N>>;

/// 🧩 The original paid source passed from fixed-page ingress to its natural file decoder.
pub type NaturalFilePagedSource = Input<{ super::natural_source_gate::NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES as usize }>;

/// 🚪 One native natural source receiver under the transport's total extent ceiling.
pub type NaturalFileSourceGuest = NaturalSourceGuest<{ super::natural_source_gate::NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES as usize }>;

impl<const N: usize> NaturalSourceGuest<N> {
    /// 🎟️ Issues directly from the mounted declaration and fresh allocation before source body reads.
    pub fn prepare(
        live: NaturalSourceOperation, fresh: bool, declaration: NaturalSourceDeclaration<'static>,
        maximum: u32, seq: u64, selected: NaturalSourceOffer<'_>, slot: &mut Option<Self>, cx: &mut StepContext<'_>,
    ) -> Result<NaturalSourceStatus, OriginalTurnFailure> {
        use NaturalSourceStatus::*;
        let grant = OriginalSourceBinding::borrow(cx);
        validate_source(grant, cx)?;
        let Some(issued) = PreparedNaturalSource::issue(live, fresh, declaration, maximum, selected)
            .filter(|loan| loan.loan().extent as usize <= N && seq != 0) else { return Ok(Refused); };
        if let Some(existing) = slot.as_ref() {
            validate_source(existing.grant, cx)?;
            return Ok(if existing.stage == Stage::Prepared && existing.seq == seq && existing.loan() == issued.loan() {
                Accepted
            } else { Stale });
        }
        Ok(run(cx, |effect| {
            let demand = copy_demand::<Option<Self>>();
            if !paid(effect, demand, 1)? { return Ok((Retry, Default::default())); }
            *slot = Some(Self {
                grant, issued, seq, stage: Stage::Prepared, command: None, source: None, pending: None, next: 0,
                terminal_receipt_sent: false,
            });
            Ok((Accepted, moved(demand)))
        })?.unwrap_or(Retry))
    }

    pub fn loan(&self) -> NaturalSourceLoan<'static> { self.issued.loan() }
    pub fn sequence(&self) -> u64 { self.seq }

    pub fn input_ready(&self) -> bool { self.stage == Stage::Sealed }

    pub fn format_runnable(&self) -> bool {
        matches!(self.stage, Stage::Sealed | Stage::Lent | Stage::Closing) || self.completion_pending()
    }

    pub fn page_pending(&self) -> bool { self.stage == Stage::Receive && self.pending.is_some() }
    pub fn cancellation_pending(&self) -> bool {
        self.stage == Stage::Closing || self.stage == Stage::Cancelled && !self.terminal_receipt_sent
    }
    pub fn completion_pending(&self) -> bool {
        matches!(self.stage, Stage::Published | Stage::Failed) && !self.terminal_receipt_sent
    }

    /// 📬 Emits one terminal receipt from the exact Original Source after retirement or format settlement.
    pub fn terminal_receipt(&mut self, cx: &mut StepContext<'_>) -> Result<Option<NaturalSourceStatus>, OriginalTurnFailure> {
        validate_source(self.grant, cx)?;
        if self.terminal_receipt_sent { return Ok(None); }
        let status = match self.stage {
            Stage::Cancelled | Stage::Published => NaturalSourceStatus::Accepted,
            Stage::Failed => NaturalSourceStatus::Refused,
            _ => return Ok(None),
        };
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<bool>(), ..Default::default() };
        if cx.retirement_should_yield() || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0
            || !progress.fits(cx.retained_grant()) { return Ok(None); }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        self.terminal_receipt_sent = true;
        Ok(Some(status))
    }

    fn authenticated(
        &self, control: NaturalSourceBegin<'_>, current: NaturalSourceOperation, fresh: bool, cx: &StepContext<'_>,
    ) -> Result<Option<NaturalSourceCommand<'static>>, OriginalTurnFailure> {
        validate_source(self.grant, cx)?;
        Ok(if self.seq == control.seq { self.issued.bind(control, current, fresh) } else { None })
    }

    /// 🌱 Starts paid custody once the host echoes the issued loan and full selected extent.
    pub fn begin(
        &mut self, control: NaturalSourceBegin<'_>, current: NaturalSourceOperation, fresh: bool, cx: &mut StepContext<'_>,
    ) -> Result<NaturalSourceStatus, OriginalTurnFailure> {
        use NaturalSourceStatus::*;
        let Some(command) = self.authenticated(control, current, fresh, cx)? else { return Ok(Stale); };
        if self.stage == Stage::Receive && self.next == 0 && self.pending.is_none() { return Ok(Accepted); }
        if self.stage != Stage::Prepared { return Ok(Stale); }
        Ok(run(cx, |effect| {
            let demand = copy_demand::<(Option<NaturalSourceCommand<'static>>, Option<PagedMediaSource<N>>, Stage)>();
            if !paid(effect, demand, 1)? { return Ok((Retry, Default::default())); }
            let progress = moved(demand);
            self.source = Some(PagedMediaSource::new(self.loan().extent as usize, N)
                .map_err(|error| error.with_retained_progress(progress))?);
            self.command = Some(command);
            self.stage = Stage::Receive;
            Ok((Accepted, progress))
        })?.unwrap_or(Retry))
    }

    /// 📩 Takes one funded ingress block and keeps it until its original backing has retired.
    pub fn stage_page(
        &mut self, incoming: &mut Option<(CommandPageCursor, FixedCommandPage)>, current: NaturalSourceOperation,
        fresh: bool, cx: &mut StepContext<'_>,
    ) -> Result<NaturalSourceStatus, OriginalTurnFailure> {
        use NaturalSourceStatus::*;
        validate_source(self.grant, cx)?;
        let Some((cursor, bytes)) = incoming.as_ref() else { return Ok(Retry); };
        if self.stage != Stage::Receive || current != self.loan().operation || !fresh { return Ok(Stale); }
        let command = self.command.as_ref().expect("receiving source has a bound command");
        let index = cursor.page_index;
        if !command.admits_page(page(cursor), bytes.len() as u32, self.next, current, fresh) || self.pending.is_some() {
            return Ok(Refused);
        }
        let source = self.source.as_ref().expect("receiving source has paid custody");
        source.demand(index, bytes.as_slice(), 0)?;
        Ok(run(cx, |effect| {
            let demand = copy_demand::<Option<PendingPage>>();
            if !paid(effect, demand, 1)? { return Ok((Retry, Default::default())); }
            let progress = moved(demand);
            let (header, bytes) = incoming.take().expect("funded page offer");
            match ControlledRetirement::new(bytes) {
                Ok(body) => self.pending = Some(PendingPage { header, body, copied: 0, complete: false }),
                Err((error, bytes)) => {
                    *incoming = Some((header, bytes));
                    return Err(error.with_retained_progress(progress));
                }
            }
            Ok((Pending, progress))
        })?.unwrap_or(Retry))
    }

    /// 🧵 Pays for copying and retirement before advancing the acknowledged frontier.
    pub fn poll_page(
        &mut self, current: NaturalSourceOperation, fresh: bool, cx: &mut StepContext<'_>,
    ) -> Result<Option<u32>, OriginalTurnFailure> {
        validate_source(self.grant, cx)?;
        if self.stage != Stage::Receive || current != self.loan().operation || !fresh { return Ok(None); }
        let Some(page) = self.pending.as_mut() else { return Ok(None); };
        if !page.complete {
            let source = self.source.as_mut().expect("pending page has paid custody");
            run(cx, |effect| {
                let bytes = page.body.original().expect("uncopied page retains its source");
                let progress = source.append(page.header.page_index, bytes.as_slice(), &mut page.copied, effect)?;
                page.complete = page.copied == bytes.len();
                Ok(((), progress))
            })?;
        }
        if !page.complete { return Ok(None); }
        if !page.body.terminal_is_empty() {
            let body = cx.retained_grant().maximum_copy_bytes;
            close_original_effect_step(cx, page.body.next_demand(body)?, |effect| page.body.step(body, effect))?;
        }
        if !page.body.terminal_is_empty() { return Ok(None); }
        Ok(run(cx, |effect| {
            let demand = copy_demand::<(Option<PendingPage>, u32)>();
            if !paid(effect, demand, 1)? { return Ok((None, Default::default())); }
            let page = self.pending.take().expect("paid retired natural source page");
            self.next += 1;
            Ok((Some(page.header.page_index), moved(demand)))
        })?.flatten())
    }

    /// 🧾 Leaves Complete pending through the independent format publication and its cleanup.
    pub fn complete(
        &mut self, control: NaturalSourceBegin<'_>, current: NaturalSourceOperation, fresh: bool, cx: &mut StepContext<'_>,
    ) -> Result<NaturalSourceStatus, OriginalTurnFailure> {
        use NaturalSourceStatus::*;
        if self.authenticated(control, current, fresh || matches!(self.stage, Stage::Published | Stage::Failed), cx)?.is_none() {
            return Ok(Stale);
        }
        let result = self.stage.complete(self.loan().extent, self.next, self.pending.is_some());
        if self.stage == Stage::Receive && result == Pending && !self.change_stage(Stage::Sealed, false, cx)? {
            return Ok(Retry);
        }
        Ok(result)
    }

    /// 📦 Transfers the same sealed paged owner under its current authenticated operation.
    pub fn take_source(
        &mut self, current: NaturalSourceOperation, fresh: bool, output: &mut Option<Input<N>>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        validate_source(self.grant, cx)?;
        if self.stage != Stage::Sealed || !fresh || self.loan().operation != current || output.is_some() { return Ok(false); }
        let source = self.source.as_mut().expect("sealed natural source exists");
        Ok(run(cx, |effect| {
            let metadata = size_of::<(Option<Input<N>>, Stage)>();
            let Some((owner, progress)) = source.take(metadata, effect)? else { return Ok((false, Default::default())); };
            *output = Some(owner);
            self.stage = Stage::Lent;
            Ok((true, progress))
        })?.unwrap_or(false))
    }

    /// 🏁 Sets the terminal receipt after the caller settles its actual format result and closes the format owner.
    pub fn finish_format(
        &mut self, current: NaturalSourceOperation, published: bool, cx: &mut StepContext<'_>,
    ) -> Result<bool, OriginalTurnFailure> {
        validate_source(self.grant, cx)?;
        if self.loan().operation != current { return Ok(false); }
        let settled = if published { Stage::Published } else { Stage::Failed };
        if self.stage == settled { return Ok(true); }
        if self.stage != Stage::Lent { return Ok(false); }
        self.change_stage(settled, true, cx)
    }

    /// 🛑 Retires all remaining source custody and waits for the supplied format owner's terminal result.
    pub fn cancel(
        &mut self, control: NaturalSourceBegin<'_>, current: NaturalSourceOperation, format_closed: bool, cx: &mut StepContext<'_>,
    ) -> Result<NaturalSourceStatus, OriginalTurnFailure> {
        use NaturalSourceStatus::*;
        if self.authenticated(control, current, true, cx)?.is_none() || matches!(self.stage, Stage::Published | Stage::Failed) {
            return Ok(Stale);
        }
        if self.stage == Stage::Cancelled { return Ok(Accepted); }
        if !self.retire(format_closed, cx)? {
            return Ok(if self.stage == Stage::Closing { Pending } else { Retry });
        }
        Ok(Accepted)
    }

    /// ♻️ Joins instance teardown only after the app's format resources are terminal.
    pub fn retire(&mut self, format_closed: bool, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        validate_source(self.grant, cx)?;
        if matches!(self.stage, Stage::Cancelled | Stage::Published | Stage::Failed) { return Ok(true); }
        if self.stage != Stage::Closing && !self.change_stage(Stage::Closing, true, cx)? { return Ok(false); }
        if let Some(page) = self.pending.as_mut().filter(|page| !page.body.terminal_is_empty()) {
            let body = cx.retained_grant().maximum_copy_bytes;
            close_original_effect_step(cx, page.body.next_demand(body)?, |effect| page.body.step(body, effect))?;
            return Ok(false);
        }
        if let Some(source) = self.source.as_mut().filter(|source| !source.terminal_is_empty()) {
            let body = cx.retained_grant().maximum_copy_bytes;
            close_original_effect_step(cx, source.close_demand(body)?, |effect| source.close_step(body, effect).expect("quoted natural source close"))?;
            return Ok(false);
        }
        if !format_closed { return Ok(false); }
        self.change_stage(Stage::Cancelled, true, cx)
    }

    fn change_stage(&mut self, stage: Stage, retiring: bool, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Stage>(), ..Default::default() };
        let ready = if retiring { !cx.retirement_should_yield() } else { !(cx.is_cancelled() || cx.should_yield()) };
        if !ready || cx.fuel_remaining() == 0 || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
            return Ok(false);
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        self.stage = stage;
        Ok(true)
    }
}

fn page(cursor: &CommandPageCursor) -> NaturalSourcePage {
    NaturalSourcePage {
        owner: cursor.owner, generation: cursor.generation, command_index: cursor.command_index,
        command_count: cursor.command_count, instance: cursor.instance, seq: cursor.seq, kind: cursor.kind,
        page_index: cursor.page_index, page_count: cursor.page_count, item_count: cursor.item_count,
        metadata: cursor.metadata,
    }
}
