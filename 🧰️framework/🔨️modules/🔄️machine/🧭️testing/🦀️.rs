//! 🧭️ Model-based testing — reachability exploration, invariants, and inline conformance fixtures.
//!
//! Fixtures are plain Rust structs/consts rather than a separate JSON format or
//! separate test files, per this workspace's "extend existing test files" rule.

use super::kernel::{init, macrostep, Command, Status};
use crate::{Configuration, Machine, NullInspector, Snapshot};

//#region 🔖️Model

/// 🧭️ A set of representative events tried from every reachable configuration.
pub struct Model<M: Machine> {
    events: Vec<M::Event>,
}

impl<M: Machine> Model<M> {
    /// 🧭️ A model that explores with exactly these representative events.
    pub fn new(events: Vec<M::Event>) -> Self {
        Self { events }
    }
}

//#endregion 🔖️Model

//#region 🔖️Paths

fn active_stable_ids<M: Machine>(snapshot: &Snapshot<M>) -> Vec<&'static str> {
    let def = M::definition();
    snapshot.configuration.iter_ones().map(|id| def.nodes[id.0 as usize].stable_id).collect()
}

//#endregion 🔖️Paths

//#region 🔖️Coverage

/// 🧭️ What a BFS [`explore`] found: distinct configurations visited and every stable
/// state id reached across them.
#[derive(Debug, Default)]
pub struct Coverage {
    pub visited_configurations: usize,
    pub reached_stable_ids: Vec<&'static str>,
}

/// 🧭️ Breadth-first walk over reachable configurations, trying every event in
/// `model` from each newly-discovered configuration. Approximates reachability by
/// configuration only — guard outcomes that depend on context may under-approximate.
pub fn explore<M: Machine>(model: &Model<M>, input: M::Input) -> Coverage
where
    M::Context: Clone,
{
    let mut sink: Vec<Command<M>> = Vec::new();
    let root = init::<M>(input, &mut sink);
    let mut visited: Vec<M::Config> = Vec::new();
    let mut frontier: Vec<Snapshot<M>> = vec![root];
    let mut reached_ids: Vec<&'static str> = Vec::new();

    while let Some(snapshot) = frontier.pop() {
        if visited.contains(&snapshot.configuration) {
            continue;
        }
        for stable in active_stable_ids(&snapshot) {
            if !reached_ids.contains(&stable) {
                reached_ids.push(stable);
            }
        }
        visited.push(snapshot.configuration.clone());

        for event in &model.events {
            let mut next = Snapshot::from_parts(snapshot.configuration.clone(), snapshot.context.clone(), Status::Running, snapshot.history_entries().to_vec());
            let mut local_sink: Vec<Command<M>> = Vec::new();
            let mut inspector = NullInspector;
            macrostep(&mut next, event.clone(), &mut local_sink, &mut inspector);
            frontier.push(next);
        }
    }

    Coverage { visited_configurations: visited.len(), reached_stable_ids: reached_ids }
}

//#endregion 🔖️Coverage

//#region ⚠️ Errors

/// ⚠️ Why an [`Invariant`] check or [`run_conformance`] fixture failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsmError {
    /// 🧭️ A model-checked invariant or conformance step reported a violation.
    Violation(String),
}

impl std::fmt::Display for FsmError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Violation(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for FsmError {}

//#endregion ⚠️ Errors

//#region 🔖️Invariants

/// 🧭️ A named property that must hold of every [`Snapshot`] visited during exploration.
pub struct Invariant<M: Machine> {
    pub name: &'static str,
    pub check: fn(&Snapshot<M>) -> Result<(), FsmError>,
}

/// 🧭️ Runs every invariant against `snapshot`, returning one formatted message per violation.
pub fn check_invariants<M: Machine>(snapshot: &Snapshot<M>, invariants: &[Invariant<M>]) -> Vec<String> {
    invariants.iter().filter_map(|inv| (inv.check)(snapshot).err().map(|reason| format!("{}: {}", inv.name, reason))).collect()
}

//#endregion 🔖️Invariants

//#region 🔖️Conformance

/// 🧭️ One step of an inline conformance fixture: send `event`, then assert every
/// stable id in `expect_active` is part of the settled configuration.
pub struct ConformanceStep<M: Machine> {
    pub event: M::Event,
    pub expect_active: &'static [&'static str],
}

/// 🧭️ Runs `steps` against a freshly-initialized machine, failing fast with a
/// descriptive message naming the offending step and the actual active configuration.
pub fn run_conformance<M: Machine>(input: M::Input, steps: &[ConformanceStep<M>]) -> Result<(), FsmError> {
    let mut sink: Vec<Command<M>> = Vec::new();
    let mut snapshot = init::<M>(input, &mut sink);
    for (index, step) in steps.iter().enumerate() {
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, step.event.clone(), &mut sink, &mut inspector);
        for expected in step.expect_active {
            if !snapshot.matches(expected) {
                return Err(FsmError::Violation(format!("conformance step {index}: expected active state '{expected}', got {:?}", active_stable_ids(&snapshot))));
            }
        }
    }
    Ok(())
}

//#endregion 🔖️Conformance

//#region 🔖️ToggleModel

#[cfg(test)]
include!("../🧪️tests/🔀️toggle-machine/🦀️.rs");

//#endregion 🔖️ToggleModel

//#region 🧪️Tests

#[cfg(test)]
include!("../🧪️tests/🔬️testing-unit/🦀️.rs");

//#endregion 🧪️Tests
