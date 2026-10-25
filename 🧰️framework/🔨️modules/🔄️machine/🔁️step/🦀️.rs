//! 🔁️ One settled transition, packaged for a host.
//!
//! The working [`crate::Snapshot`] is born and dies inside [`step`]: a host reads a
//! [`crate::PersistedSnapshot`] out of whatever state lane it owns, gets a [`MachineStep`]
//! back, and writes the new one in. Nothing mutable crosses the call boundary, so machine
//! state is never a durable field and never a `thread_local!`.

use super::{
    kernel::{self, Command, Snapshot, StepReport},
    persist::{self, Migration, PersistedSnapshot, RestoreError},
};
use crate::{Configuration, InspectionEvent, Inspector, Machine, NodeId};

//#region 🔖️StepInspector

/// 🔎️ Accumulates the entered/exited node sets across every microstep of one macrostep.
struct StepInspector {
    entered: Vec<NodeId>,
    exited: Vec<NodeId>,
}

impl Default for StepInspector {
    fn default() -> Self {
        Self { entered: Vec::new(), exited: Vec::new() }
    }
}

impl<M: Machine> Inspector<M> for StepInspector {
    fn observe(&mut self, event: InspectionEvent<'_, M>) {
        if let InspectionEvent::Microstep { exited, entered } = event {
            self.exited.extend_from_slice(exited);
            self.entered.extend_from_slice(entered);
        }
    }
}

//#endregion 🔖️StepInspector

//#region 🔖️MachineStep

/// 🖥️ Everything a host needs to project one settled transition into its own state lanes.
///
/// `entered`/`exited` are the union across the macrostep's microsteps, in execution order — a
/// node touched twice within one macrostep appears twice, because that is what happened.
/// `active` is the settled configuration and is the field to project from.
pub struct MachineStep<M: Machine> {
    pub entered: Vec<&'static str>,
    pub exited: Vec<&'static str>,
    pub active: Vec<&'static str>,
    pub commands: Vec<Command<M>>,
    pub report: StepReport,
    pub persisted: PersistedSnapshot,
}

impl<M: Machine> MachineStep<M> {
    fn of(snapshot: &Snapshot<M>, entered: Vec<NodeId>, exited: Vec<NodeId>, commands: Vec<Command<M>>, report: StepReport) -> Self {
        Self {
            entered: stable_ids::<M>(&entered),
            exited: stable_ids::<M>(&exited),
            active: snapshot.configuration.iter_ones().map(|id| M::definition().nodes[id.0 as usize].stable_id).collect(),
            commands,
            report,
            persisted: persist::persist(snapshot),
        }
    }

    /// 🎛️ Whether the settled configuration contains the state with this stable id.
    pub fn is_active(&self, stable_id: &str) -> bool {
        self.active.iter().any(|id| *id == stable_id)
    }
}

fn stable_ids<M: Machine>(ids: &[NodeId]) -> Vec<&'static str> {
    let definition = M::definition();
    ids.iter().map(|id| definition.nodes[id.0 as usize].stable_id).collect()
}

//#endregion 🔖️MachineStep

//#region 🔖️StepEntryPoints

/// 🌱️ Builds the initial configuration — the host's first call, when its lane holds no
/// [`PersistedSnapshot`] yet.
pub fn start<M: Machine>(input: M::Input) -> MachineStep<M> {
    let mut commands = Vec::new();
    let snapshot = kernel::init::<M>(input, &mut commands);
    MachineStep::of(&snapshot, Vec::new(), Vec::new(), commands, StepReport::default())
}

/// 🪜️ Restores from `prior`, runs one macrostep, and persists the result — the whole
/// read-transition-write cycle, with the live [`Snapshot`] confined to this frame.
pub fn step<M: Machine, Mg: Migration>(prior: &PersistedSnapshot, context: M::Context, event: M::Event, migrations: &[&Mg]) -> Result<MachineStep<M>, RestoreError> {
    let mut snapshot = persist::restore::<M, Mg>(prior, context, migrations)?;
    let mut commands = Vec::new();
    let mut inspector = StepInspector::default();
    let report = kernel::macrostep::<M>(&mut snapshot, event, &mut commands, &mut inspector);
    Ok(MachineStep::of(&snapshot, inspector.entered, inspector.exited, commands, report))
}

//#endregion 🔖️StepEntryPoints
