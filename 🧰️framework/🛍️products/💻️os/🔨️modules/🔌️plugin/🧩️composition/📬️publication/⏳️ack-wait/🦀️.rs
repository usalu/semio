//! ⏳️ Ack wait of a mounted typed operation: whether the guest owes the operation a step or only the host's acknowledgement can move it. See [`super`].

use super::MountedTypedCommandFullOperationStage;

impl MountedTypedCommandFullOperationStage {
    /// ⏳️ Whether a step of the guest can move an operation in this stage; an `AwaitingAck` operation whose result page is already out waits for the host's acknowledgement event and must not keep the reactor in more-work.
    pub const fn owes_guest_step(self, result_page_presented: bool) -> bool {
        !matches!(self, Self::AwaitingAck) || !result_page_presented
    }
}

#[cfg(test)]
mod ack_wait_tests {
    use super::*;

    /// ⚖️ LAW: a runnable typed operation that can make no progress in a unit does not owe a step; only an unacknowledged presented page is such a wait.
    #[test]
    fn an_operation_waiting_for_the_host_acknowledgement_owes_the_guest_no_step() {
        let contract: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("the ack wait contract fixture");
        for row in contract["rows"].as_array().unwrap() {
            let stage = match row["stage"].as_str().unwrap() {
                "Worker" => MountedTypedCommandFullOperationStage::Worker,
                "Publishing" => MountedTypedCommandFullOperationStage::Publishing,
                "AwaitingAck" => MountedTypedCommandFullOperationStage::AwaitingAck,
                "Retiring" => MountedTypedCommandFullOperationStage::Retiring,
                other => panic!("unknown stage {other}"),
            };
            assert_eq!(stage.owes_guest_step(row["resultPagePresented"].as_bool().unwrap()), row["owesGuestStep"].as_bool().unwrap(), "{row}");
        }
    }
}
