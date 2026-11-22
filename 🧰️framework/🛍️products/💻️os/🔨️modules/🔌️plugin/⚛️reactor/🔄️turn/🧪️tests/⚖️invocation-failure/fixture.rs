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

    /// 🧬️ Borrows the original failed command cause while the ingress lane retains it.
    pub fn failed_original(&self) -> Option<&OriginalTurnFailure> {
        let CommandIngressOwner::Generic { command: Some((_, command)), .. } = self.0.state()? else {
            return None;
        };
        let semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded { failure, .. } = command else {
            return None;
        };
        failure.as_ref()
    }

    /// 🔍️ Observes whether the borrowed generic lane has begun retiring its exchange output.
    pub fn output_retiring(&self) -> bool {
        matches!(self.0.state(), Some(CommandIngressOwner::Generic { output_close: Some(_), .. }))
    }

    /// 🛑️ Asks the actual exchange guard whether the borrowed lane may still execute its command.
    pub fn exchange_may_run(&self, closing_instances: &[u32]) -> bool {
        self.0.state().is_some_and(|owner| owner.exchange_may_run(closing_instances))
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
