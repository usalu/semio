//! 🪙️ Real fixture turns preserve the native Work, construction and retirement receipts.
use semio_framework_value::{
    AuthoringWallet, NativeDecodeControl, OriginalRetirementPoll, RetainedAllocationEffect,
    RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome,
    ValueError, ORIGINAL_INVOCATION_WORK,
};
use semio_framework_value_native_owned::native_decoding::{
    OriginalConstructionAccept, OriginalConstructionAcceptComplete, OriginalConstructionAuthority,
    OriginalConstructionGrant, OriginalConstructionGrantComplete, OriginalConstructionRecipient,
    OriginalReceivingPrimary, OriginalSourceTurnIdentity, OriginalWorkAuthority,
    OriginalWorkPartialCheck, OriginalWorkPartialCheckComplete, OriginalWorkQuote,
    OriginalWorkQuoteComplete, OriginalWorkRecipient,
};

pub(super) struct OperationBank {
    pub(super) left: RetainedCloneGrant,
    fuel: u64,
    quantum: RetainedCloneGrant,
    source: OriginalSourceTurnIdentity,
    work: OriginalWorkAuthority<OperationWorkDriver>,
    physical: OriginalConstructionAuthority<OperationPhysicalDriver>,
}

struct OperationWorkDriver;
impl OriginalWorkRecipient for OperationWorkDriver {
    type Context<'turn, 'source> = (u64, OriginalSourceTurnIdentity) where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &Self::Context<'turn, 'source>) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, context: &Self::Context<'turn, 'source>) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { Some(context.1) }
    fn admits<'reply, 'turn, 'source>(&mut self, context: &mut Self::Context<'turn, 'source>, quote: OriginalWorkQuote<'reply>, observed: u64, additional: u64) -> OriginalWorkQuoteComplete<'reply> where 'source: 'turn {
        quote.complete(Ok(observed.checked_add(additional).is_some_and(|work| work <= context.0)))
    }
    fn settle_partial<'reply, 'turn, 'source>(&mut self, _: &mut Self::Context<'turn, 'source>, check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
    fn recover_partial<'reply, 'turn, 'source>(&mut self, _: &mut Self::Context<'turn, 'source>, check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.pending() }
}
struct OperationPhysicalLoan {
    submitted: RetainedCloneProgress,
    left: RetainedCloneGrant,
    source: OriginalSourceTurnIdentity,
}
struct OperationPhysicalDriver;
impl OriginalConstructionRecipient for OperationPhysicalDriver {
    type Context<'turn, 'source> = OperationPhysicalLoan where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &OperationPhysicalLoan) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, loan: &OperationPhysicalLoan) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { Some(loan.source) }
    fn grant<'reply, 'turn, 'source>(&mut self, loan: &mut OperationPhysicalLoan, quote: OriginalConstructionGrant<'reply>) -> OriginalConstructionGrantComplete<'reply> where 'source: 'turn { quote.complete(Ok(loan.left)) }
    fn accept<'reply, 'turn, 'source>(&mut self, loan: &mut OperationPhysicalLoan, receipt: OriginalConstructionAccept<'reply>) -> OriginalConstructionAcceptComplete<'reply> where 'source: 'turn {
        let progress = receipt.progress();
        assert!(progress.fits(receipt.grant()));
        debit_grant(&mut loan.left, progress);
        receipt.complete(loan.submitted.checked_add(progress).map(|total| loan.submitted = total))
    }
    fn submitted<'turn, 'source>(&self, loan: &OperationPhysicalLoan) -> RetainedCloneProgress where 'source: 'turn { loan.submitted }
}

fn debit_grant(left: &mut RetainedCloneGrant, progress: RetainedCloneProgress) {
    assert!(progress.fits(*left));
    left.maximum_items -= progress.copied_items;
    left.maximum_copy_bytes -= progress.copied_bytes;
    left.maximum_capacity_bytes -= progress.retained_capacity_bytes;
    left.maximum_release_bytes -= progress.released_bytes;
}

impl OperationBank {
    pub(super) fn new(quantum: RetainedCloneGrant, left: RetainedCloneGrant, fuel: u64, source: OriginalSourceTurnIdentity) -> Self {
        Self {
            left, fuel, quantum, source,
            work: OriginalWorkAuthority::new(OperationWorkDriver),
            physical: OriginalConstructionAuthority::new(OperationPhysicalDriver),
        }
    }

    pub(super) fn fixture(quantum: RetainedCloneGrant, source: OriginalSourceTurnIdentity) -> Self {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();
        let stock = &fixture["originalStock"];
        Self::new(quantum, RetainedCloneGrant {
            maximum_items: stock["items"].as_u64().unwrap() as usize,
            maximum_copy_bytes: stock["copyBytes"].as_u64().unwrap() as usize,
            maximum_capacity_bytes: stock["capacityBytes"].as_u64().unwrap() as usize,
            maximum_release_bytes: stock["releaseBytes"].as_u64().unwrap() as usize,
            maximum_depth: stock["depth"].as_u64().unwrap() as usize,
        }, stock["fuel"].as_u64().unwrap(), source)
    }

    pub(super) fn grant(&self) -> RetainedCloneGrant {
        let grant = self.quantum.restricted_by(self.left);
        RetainedCloneGrant { maximum_items: if self.fuel == 0 { 0 } else { grant.maximum_items }, ..grant }
    }

    pub(super) fn debit(&mut self, progress: RetainedCloneProgress, work: u128) {
        debit_grant(&mut self.left, progress);
        self.fuel = self.fuel.checked_sub(u64::try_from(work).unwrap()).unwrap();
    }

    pub(super) fn native(
        &mut self, native: &mut NativeDecodeControl<'_>, grant: RetainedCloneGrant, maximum_work: u64,
        operation: impl FnOnce(&mut NativeDecodeControl<'_>, RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        let mut work = (self.fuel.min(maximum_work), self.source);
        let mut physical = OperationPhysicalLoan { submitted: Default::default(), left: grant, source: self.source };
        let before = native.actual_work();
        let turn = self.work.with_loan(&mut work, |work| native.with_work_recipient(work, |native| {
            self.physical.with_loan(&mut physical, |physical| {
                native.with_construction_recipient(physical, |native| Ok::<_, ValueError>(operation(native, grant)))
            })?
        })).unwrap().unwrap();
        let progress = turn.result.as_ref().copied().unwrap_or_else(|error| error.retained_progress());
        self.debit(progress, native.actual_work().checked_sub(before).unwrap());
        turn
    }

    pub(super) fn normal(
        &mut self, native: &mut NativeDecodeControl<'_>, work: u64,
        operation: impl FnOnce(&mut NativeDecodeControl<'_>, RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        self.native(native, self.grant(), work.saturating_add(ORIGINAL_INVOCATION_WORK), operation)
    }

    pub(super) fn close_native(
        &mut self, native: &mut NativeDecodeControl<'_>,
        operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll,
    ) -> RetainedWorkOutcome<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        let maximum = self.fuel.saturating_sub(ORIGINAL_INVOCATION_WORK);
        let mut closed = None;
        self.normal(native, maximum, |native, grant| {
            let turn = match native.with_effect(grant, maximum, |_, effect| operation(effect)) {
                Ok(Some(semio_framework_value::RetainedWorkPoll::Complete(turn))) => turn,
                Ok(Some(semio_framework_value::RetainedWorkPoll::Pending { work })) => RetainedWorkOutcome {
                    result: Err(ValueError::invariant("fixture original close stayed pending").into()), work,
                },
                Ok(None) => RetainedWorkOutcome {
                    result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0,
                },
                Err(error) => RetainedWorkOutcome { result: Err(error.into()), work: 0 },
            };
            let progress = turn.result.as_ref().map_or_else(|error| error.retained_progress(), |step| step.progress());
            let work = turn.work;
            closed = Some(turn);
            RetainedWorkOutcome { result: Ok(progress), work }
        }).result.unwrap();
        closed.unwrap()
    }

    pub(super) fn close(&mut self, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll) -> RetainedWorkOutcome<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        let mut wallet = AuthoringWallet::new(self.left, u128::from(self.fuel));
        let (turn, _) = wallet.run_settled(self.grant(), self.fuel, operation).unwrap().unwrap();
        let progress = turn.result.as_ref().map_or_else(|error| error.retained_progress(), |step| step.progress());
        self.debit(progress, u128::from(self.fuel) - wallet.remaining_work());
        turn
    }
}