
use super::*;
use std::cell::Cell;

struct AlwaysAccepts;
impl CommandSink for AlwaysAccepts {
    fn try_send(&self, _command: Command) -> Result<(), SinkFull> {
        Ok(())
    }
}

struct FailsAfter {
    calls: Cell<u32>,
    accepts: u32,
}
impl CommandSink for FailsAfter {
    fn try_send(&self, _command: Command) -> Result<(), SinkFull> {
        let seen = self.calls.get();
        self.calls.set(seen + 1);
        if seen < self.accepts { Ok(()) } else { Err(SinkFull) }
    }
}

fn command(id: u64) -> Command {
    Command { id: CommandId(id), correlation: CorrelationId(id), payload: ui_contract::UiValue::Number(id as f64) }
}

#[test]
fn full_local_capacity_returns_full_synchronously_without_dropping_the_command() {
    let mut gateway = CommandGateway::new(1, AlwaysAccepts);
    let first = gateway.try_submit(command(1)).expect("first fits within capacity");
    assert_eq!(first.command_id, CommandId(1));

    let overflow = gateway.try_submit(command(2));
    assert_eq!(overflow, Err(GatewayError::Full { command_id: CommandId(2) }));
    assert_eq!(gateway.len(), 1, "the rejected command must not have been silently tracked as sent");
    assert_eq!(gateway.status(CommandId(2)), None, "a refused submission has no ticket to look up");
}

#[test]
fn full_backing_sink_returns_full_synchronously_without_dropping_the_command() {
    let sink = FailsAfter { calls: Cell::new(0), accepts: 1 };
    let mut gateway = CommandGateway::new(10, sink);
    gateway.try_submit(command(1)).expect("sink accepts the first command");

    let refused = gateway.try_submit(command(2));
    assert_eq!(refused, Err(GatewayError::Full { command_id: CommandId(2) }));
    assert_eq!(gateway.len(), 1, "a sink-refused command is never enqueued as if it were sent");
}

#[test]
fn ticket_round_trips_to_acknowledged_and_to_rejected() {
    let mut gateway = CommandGateway::new(10, AlwaysAccepts);
    let acked = gateway.try_submit(command(1)).expect("submits");
    let rejected = gateway.try_submit(command(2)).expect("submits");

    assert_eq!(gateway.status(acked.command_id), Some(OptimisticStatus::Pending));
    assert!(gateway.acknowledge(acked.command_id));
    assert_eq!(gateway.status(acked.command_id), Some(OptimisticStatus::Acknowledged));

    assert_eq!(gateway.status(rejected.command_id), Some(OptimisticStatus::Pending));
    assert!(gateway.reject(rejected.command_id));
    assert_eq!(gateway.status(rejected.command_id), Some(OptimisticStatus::Rejected));

    assert_eq!(gateway.len(), 0, "resolved tickets free their capacity slot");
    assert!(!gateway.acknowledge(CommandId(404)), "resolving an unknown id is a no-op, not a panic");
}

#[test]
fn resolving_a_ticket_frees_capacity_for_a_new_submission() {
    let mut gateway = CommandGateway::new(1, AlwaysAccepts);
    let first = gateway.try_submit(command(1)).expect("fits");
    assert_eq!(gateway.try_submit(command(2)), Err(GatewayError::Full { command_id: CommandId(2) }));

    assert!(gateway.acknowledge(first.command_id));
    gateway.try_submit(command(2)).expect("capacity freed by the acknowledgement");
}
