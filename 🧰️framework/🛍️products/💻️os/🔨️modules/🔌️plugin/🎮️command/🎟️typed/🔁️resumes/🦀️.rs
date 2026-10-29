//! 🔁️ Task resumes of the instance (a command the runtime itself originates, like host events): the follow-up commands resolved `AsyncTask`s hand back wait here until the instance has delivered each one through its own original typed source across funded turns.
//!
//! 🧵️ The reactor drains one resolved task per round and queues it here ([`TaskResumes::push`]); the instance then advances the queue front one funded step per publication unit ([`TaskResumes::plan`]) until the answering receipt is owed to the host ([`TaskResumes::take_outcome`]).
use semio_framework::kernel::InvocationResult;
use semio_framework::{Fault, FaultCode, FaultOrigin};
use semio_framework_plugin_decl_emission::ActionMeta;
use std::collections::VecDeque;

/// 📏️ The most resolved tasks one instance holds before it refuses the next resume by name.
pub const TASK_RESUME_QUEUE_SLOTS: usize = 1_024;

/// 📬️ One resolved task follow-up: the `OpBinary` bytes of the app's typed command and the originating action metadata.
struct TaskResume {
    command: Option<Vec<u8>>,
    meta: Option<ActionMeta>,
}

/// 🪜️ What one funded publication unit does for the queue front.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskResumeStep {
    /// 💤️ Nothing to do: the queue is empty, the receipt waits for the host, or the source is busy.
    Idle,
    /// 🪦️ The front is retiring: close the source one paid step.
    Close,
    /// 🏃️ Produce, admit and mount the front through the original typed source.
    Drive,
}

/// 🔁️ The queued task resumes of one instance and the delivery in flight; `R` is the receipt a delivery leaves for the host.
pub struct TaskResumes<R = Result<InvocationResult, Fault>> {
    queue: VecDeque<TaskResume>,
    started: bool,
    closing: bool,
    failure: Option<R>,
    outcome: Option<R>,
    turn: bool,
}

impl<R> TaskResumes<R> {
    /// 🎹️ An empty queue with nothing in flight.
    pub fn new() -> Self {
        Self { queue: VecDeque::new(), started: false, closing: false, failure: None, outcome: None, turn: false }
    }

    /// 📥️ Queues one resolved task follow-up behind the earlier ones; a full queue refuses by name.
    pub fn push(&mut self, command: Vec<u8>, meta: ActionMeta) -> Result<(), Fault> {
        if self.queue.len() >= TASK_RESUME_QUEUE_SLOTS {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.task-resume-queue"), format!("{TASK_RESUME_QUEUE_SLOTS} resolved task follow-ups already wait for delivery")));
        }
        self.queue.push_back(TaskResume { command: Some(command), meta: Some(meta) });
        Ok(())
    }

    /// ⏳️ Whether the queue, the delivery in flight or the unclaimed receipt still owes the instance a unit.
    pub fn pending(&self) -> bool {
        !self.queue.is_empty() || self.started || self.closing || self.outcome.is_some()
    }

    /// 🏃️ Whether a publication unit can advance the queue now: the receipt of the last delivery is claimed first.
    pub fn runnable(&self) -> bool {
        self.outcome.is_none() && (!self.queue.is_empty() || self.started || self.closing)
    }

    /// ⚖️ Alternates the instance's publication units between the queue and every other owner so neither starves the other.
    pub fn take_turn(&mut self) -> bool {
        self.turn = !self.turn;
        self.turn
    }

    /// 🔒️ Whether a delivery holds the shared typed source: every other lane waits for it instead of reading a half-finished source.
    pub fn in_flight(&self) -> bool {
        self.started || self.closing
    }

    /// 🧾️ Plans one funded unit: `cancelled` retires the whole queue once the source this queue holds is closed (another lane's source is left to that lane), `source_pending` is whether the typed source still holds an owner
    /// and `lane_busy` whether another lane is about to take it.
    pub fn plan(&mut self, cancelled: bool, source_pending: bool, lane_busy: bool) -> TaskResumeStep {
        if self.outcome.is_some() {
            return TaskResumeStep::Idle;
        }
        if cancelled {
            if source_pending && (self.started || self.closing) {
                return TaskResumeStep::Close;
            }
            self.queue.clear();
            self.started = false;
            self.closing = false;
            self.failure = None;
            return TaskResumeStep::Idle;
        }
        if self.closing {
            if source_pending {
                return TaskResumeStep::Close;
            }
            self.queue.pop_front();
            self.started = false;
            self.closing = false;
            self.outcome = self.failure.take();
            return TaskResumeStep::Idle;
        }
        if self.queue.is_empty() {
            return TaskResumeStep::Idle;
        }
        if !self.started {
            if source_pending || lane_busy {
                return TaskResumeStep::Idle;
            }
            self.started = true;
        }
        TaskResumeStep::Drive
    }

    /// 🫴️ Lends the front's command bytes and metadata for one drive step; what the step did not consume goes back through [`Self::restore`].
    pub fn lend(&mut self) -> (Option<Vec<u8>>, Option<ActionMeta>) {
        self.queue.front_mut().map_or((None, None), |front| (front.command.take(), front.meta.take()))
    }

    /// 🫳️ Returns what a drive step left unconsumed to the queue front.
    pub fn restore(&mut self, command: Option<Vec<u8>>, meta: Option<ActionMeta>) {
        if let Some(front) = self.queue.front_mut() {
            front.command = command;
            front.meta = meta;
        }
    }

    /// ✅️ The front was mounted: its receipt is owed to the host and the next resume may start.
    pub fn delivered(&mut self, receipt: R) {
        self.queue.pop_front();
        self.started = false;
        self.outcome = Some(receipt);
    }

    /// 🛑️ The front failed: the source closes before the next resume starts, then `answer` (the fault owed for this task, if the failure can be
    /// answered) becomes the receipt owed to the host.
    pub fn failed(&mut self, answer: Option<R>) {
        self.closing = true;
        self.failure = answer;
    }

    /// 🧾️ Claims the receipt of the last delivery.
    pub fn take_outcome(&mut self) -> Option<R> {
        self.outcome.take()
    }

    /// 🔢️ How many follow-ups still wait, the one in flight included.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// 🪫️ Whether no follow-up waits.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl<R> Default for TaskResumes<R> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAW: &str = include_str!("../🧪️tests/🧫️fixtures/🔁️task-resumes/🔣️.json");

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 7, view_state: None }
    }

    fn step(text: &str) -> TaskResumeStep {
        match text {
            "idle" => TaskResumeStep::Idle,
            "close" => TaskResumeStep::Close,
            "drive" => TaskResumeStep::Drive,
            other => panic!("step {other}"),
        }
    }

    fn run(name: &str, steps: &[serde_json::Value]) {
        let mut queue = TaskResumes::<u64>::new();
        let mut lent: (Option<Vec<u8>>, Option<ActionMeta>) = (None, None);
        for (index, row) in steps.iter().enumerate() {
            let context = format!("{name} step {index}: {row}");
            let flag = |key: &str| row[key].as_bool().unwrap_or_else(|| panic!("{context}: {key}"));
            let expect = &row["expect"];
            match row["op"].as_str().unwrap() {
                "push" => queue.push(row["id"].as_str().unwrap().as_bytes().to_vec(), meta(row["id"].as_str().unwrap())).expect("queue has room"),
                "plan" => assert_eq!(queue.plan(flag("cancelled"), flag("source"), flag("busy")), step(expect.as_str().unwrap()), "{context}"),
                "lend" => {
                    lent = queue.lend();
                    assert_eq!(lent.0.as_ref().map(|bytes| String::from_utf8(bytes.clone()).unwrap()), expect.as_str().map(str::to_string), "{context}");
                    assert_eq!(lent.1.is_some(), expect.is_string(), "{context}");
                }
                "restore" => queue.restore(lent.0.take(), lent.1.take()),
                "delivered" => queue.delivered(row["receipt"].as_u64().unwrap()),
                "failed" => queue.failed(row["answer"].as_u64()),
                "outcome" => assert_eq!(queue.take_outcome(), expect.as_u64(), "{context}"),
                "len" => assert_eq!(queue.len() as u64, expect.as_u64().unwrap(), "{context}"),
                "inflight" => assert_eq!(queue.in_flight(), expect.as_bool().unwrap(), "{context}"),
                "runnable" => assert_eq!(queue.runnable(), expect.as_bool().unwrap(), "{context}"),
                "pending" => assert_eq!(queue.pending(), expect.as_bool().unwrap(), "{context}"),
                other => panic!("{context}: unknown op {other}"),
            }
        }
    }

    #[test]
    fn the_language_agnostic_law_holds_for_every_scenario() {
        let law: serde_json::Value = serde_json::from_str(LAW).unwrap();
        let scenarios = law["scenarios"].as_array().unwrap();
        assert!(scenarios.len() >= 10);
        for scenario in scenarios {
            run(scenario["name"].as_str().unwrap(), scenario["steps"].as_array().unwrap());
        }
    }

    #[test]
    fn a_full_queue_refuses_the_next_resume_by_name_and_keeps_the_rest() {
        let mut queue = TaskResumes::<u64>::new();
        for index in 0..TASK_RESUME_QUEUE_SLOTS {
            queue.push(vec![0], meta(&index.to_string())).expect("room");
        }
        let fault = queue.push(vec![1], meta("late")).expect_err("a full queue refuses");
        assert_eq!(fault.code.0, "interactive-job.task-resume-queue");
        assert_eq!(queue.len(), TASK_RESUME_QUEUE_SLOTS);
    }

    #[test]
    fn cancelling_after_every_turn_count_retires_the_queue_through_the_source_closure() {
        for turns in 0..8 {
            let mut queue = TaskResumes::<u64>::new();
            for id in 0..3 {
                queue.push(vec![id], meta("a")).unwrap();
            }
            let mut source_pending = false;
            for _ in 0..turns {
                if queue.plan(false, source_pending, false) == TaskResumeStep::Drive {
                    source_pending = true;
                }
            }
            let mut closures = 0;
            loop {
                match queue.plan(true, source_pending, false) {
                    TaskResumeStep::Close => {
                        closures += 1;
                        source_pending = closures < 3;
                    }
                    TaskResumeStep::Idle => break,
                    TaskResumeStep::Drive => panic!("a cancelled queue never drives"),
                }
            }
            assert!(queue.is_empty() && !queue.pending() && !queue.in_flight(), "cancelled after {turns} turns");
        }
    }
}
