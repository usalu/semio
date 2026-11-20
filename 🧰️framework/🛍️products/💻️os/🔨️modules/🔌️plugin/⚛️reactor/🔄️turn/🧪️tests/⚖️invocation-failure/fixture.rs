//! 🧪️ Test driver for the original ingress custody slots.
use super::{retire_command_ingress, CommandIngressLoan, CommandIngressOwner, RetainedExtensionInvocation, COMMAND_INGRESS, RETAINED_EXTENSION_INVOCATIONS};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::RetainedCloneProgress;
use semio_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey;

pub struct CommandIngressFixture<'a>(&'a mut CommandIngressLoan);

impl CommandIngressFixture<'_> {
    /// 🔍️ Observes whether the borrowed original lane still holds its owner.
    pub fn has_owner(&self) -> bool {
        self.0.state().is_some()
    }

    /// 🧹️ Advances the actual retirement reducer for the borrowed ingress lane.
    pub fn retire_step(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        retire_command_ingress(self.0.state_slot().expect("fixture lane is admitted"), cx)
    }
}

/// 🔒️ Admits the actual ingress slot and settles its return after the supplied retirement law runs.
pub fn with_command_ingress(
    slot: usize,
    key: NativeCloseKey,
    state: CommandIngressOwner,
    maximum_turns: u64,
    cx: &mut StepContext<'_>,
    run: impl FnOnce(&mut CommandIngressFixture<'_>, &mut StepContext<'_>) -> Result<(), OriginalTurnFailure>,
) -> Result<(), OriginalTurnFailure> {
    let mut loan = CommandIngressLoan::empty(slot);
    assert!(loan.lend(cx)?);
    assert!(loan.admit_entry(key, cx)?);
    let copied_bytes = std::mem::size_of::<Option<CommandIngressOwner>>();
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() })?;
    cx.consume_fuel(1);
    loan.put_state(Some(state));
    run(&mut CommandIngressFixture(&mut loan), cx)?;
    assert!(loan.state().is_none());
    loan.clear_empty_entry(cx)?;
    loan.restore()?;
    for _ in 0..maximum_turns {
        if COMMAND_INGRESS.with(|owners| owners.borrow_mut()[slot].settle_step(cx))? {
            return Ok(());
        }
    }
    panic!("the original ingress slot did not settle within the fixture limit");
}

/// 📤️ Takes the exact owner the request receiver parked for the next step.
pub fn take_extension_invocation(index: usize) -> Option<RetainedExtensionInvocation> {
    RETAINED_EXTENSION_INVOCATIONS.with(|owners| owners.borrow_mut()[index].take())
}
