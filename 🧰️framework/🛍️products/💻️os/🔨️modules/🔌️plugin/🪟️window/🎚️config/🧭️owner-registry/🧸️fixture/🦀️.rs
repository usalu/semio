//! 🧸️ Fixture lifetime of the window config owner registry: real declared fixture partitions opened and closed through original turns. See [`super`].

use super::*;

/// 🧪️ The original caller and decoder that birthed the fixture partitions: the partitions hold work on this decoder until the registry closes.
pub(super) struct FixtureLifetime {
    caller: semio_framework_job::OriginalJobFixtureCaller,
    decoder: semio_framework_job::OriginalJobDecodeOwner,
    bank: semio_framework_job::RetainedCloneGrant,
}

const FIXTURE_BANK: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: 1 << 30, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 4096 };

const FIXTURE_TURN: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 16 * 1024 * 1024, maximum_release_bytes: 16 * 1024 * 1024, maximum_depth: 4096 };

impl WindowConfigOwnerRegistry {
    /// 🧪️ Opens a real declared fixture partition and returns its original issued authority; the birthing caller and decoder stay with the registry until [`Self::fixture_close`].
    pub async fn fixture_partition_authority(&mut self, kind: &str, id: &str) -> Result<WindowConfigAuthorityIdentity, Fault> {
        let owner = self.owners.get_mut(kind).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.fixture-owner"), "fixture partition requires its declared original owner"))?;
        let fixture_fault = |error: semio_framework_job::OriginalTurnFailure| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.fixture-turn"), format!("{error:?}"));
        let fixture = self.fixture.get_or_insert_with(|| {
            let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(23), semio_framework_job::Generation(1), FIXTURE_BANK, semio_framework_job::default_now_us, u64::MAX, 1 << 28);
            caller.admit_return_table();
            let decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
            FixtureLifetime { caller, decoder, bank: FIXTURE_BANK }
        });
        let mut authority = None;
        let mut ready = false;
        for _ in 0..262144 { if fixture.caller.turn_with_decoder(FIXTURE_TURN, &mut fixture.decoder, |cx| { let ready = owner.ensure_partition(id, cx)?; cx.settle_receiving_scope().map_err(semio_framework_job::OriginalTurnFailure::from)?; Ok(ready) }).map_err(fixture_fault)? { ready = true; break; } }
        if !ready { return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.fixture-turn"), "fixture partition did not open within its bounded turns")); }
        fixture.caller.turn_with_decoder(FIXTURE_TURN, &mut fixture.decoder, |cx| { let captured = owner.capture_original_header(id, cx, &mut authority)?; cx.settle_receiving_scope().map_err(semio_framework_job::OriginalTurnFailure::from)?; Ok(captured) }).map_err(fixture_fault)?;
        Ok(authority.ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.fixture-turn"), "fixture capture produced no authority"))?.snapshot.authority)
    }

    /// 🧪️ Closes every fixture partition, then the birthing decoder and caller, through real original turns.
    pub fn fixture_close(&mut self) {
        let Some(mut fixture) = self.fixture.take() else { return };
        for _ in 0..65536 {
            if self.terminal_is_empty() { break; }
            let mut failure = None;
            fixture.caller.turn_with_decoder(FIXTURE_TURN, &mut fixture.decoder, |cx| { if let Err(error) = self.close_step_context(FIXTURE_TURN, cx) { failure = Some(format!("{error:?}")); } cx.settle_receiving_scope().map_err(semio_framework_job::OriginalTurnFailure::from)?; Ok(()) }).expect("fixture registry close turn");
            assert!(failure.is_none(), "fixture registry close failed: {failure:?}");
        }
        assert!(self.terminal_is_empty(), "fixture registry did not close within its bounded turns");
        for _ in 0..65536 { if fixture.decoder.terminal_is_empty() { break; } fixture.caller.turn(FIXTURE_TURN, |cx| cx.with_original_retirement(|cx| { fixture.decoder.close_on_source(cx)?; Ok(()) })).expect("fixture decoder close turn"); }
        assert!(fixture.decoder.terminal_is_empty(), "fixture decoder did not close within its bounded turns");
        fixture.caller.retire(fixture.bank);
    }
}
