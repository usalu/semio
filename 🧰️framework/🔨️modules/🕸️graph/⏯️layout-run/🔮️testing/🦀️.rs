//! 🔮️ Layout run testing: the third-party Fruchterman-Reingold oracle, stress metric and the ledger-style run driver.

use super::{LayoutRunGraph, LayoutRunPoint, LayoutRunStage};
use fdg_sim::force::fruchterman_reingold;
use fdg_sim::glam::Vec3;
use fdg_sim::{Dimensions, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};
use semio_framework_job::{drive_step, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobOutcomeView, OperationId, OriginalJobFixtureCaller, OriginalJobReturn, OriginalJobReturnOrigin, RetainedCloneGrant};
use semio_framework_job_payload::{RetainedJobPayload};
use semio_framework_tool_run::{ToolRunProgress, ToolRunStep, ToolRunTick, ToolRunTraceOp, ToolRunTraceStore, ToolRunVerdict};
use serde::Deserialize;
use std::collections::{BTreeSet, VecDeque};

/// 🎛️ The `fdg-sim` Fruchterman-Reingold parameters a fixture's `oracle` section carries.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutRunOracleParameters {
    pub fdg_scale: f32,
    pub fdg_cooloff: f32,
    pub fdg_dt: f32,
    pub fdg_updates: u64,
}

/// 🦘️ All-pairs hop distances over the undirected edges (`u32::MAX` = disconnected).
pub fn layout_run_hop_distances(graph: &LayoutRunGraph) -> Vec<Vec<u32>> {
    let count = graph.nodes.len();
    let mut adjacency = vec![Vec::new(); count];
    for edge in &graph.edges {
        adjacency[edge.source as usize].push(edge.target as usize);
        adjacency[edge.target as usize].push(edge.source as usize);
    }
    (0..count)
        .map(|source| {
            let mut distance = vec![u32::MAX; count];
            let mut queue = VecDeque::from([source]);
            distance[source] = 0;
            while let Some(node) = queue.pop_front() {
                for &next in &adjacency[node] {
                    if distance[next] == u32::MAX {
                        distance[next] = distance[node] + 1;
                        queue.push_back(next);
                    }
                }
            }
            distance
        })
        .collect()
}

/// 📏️ Scale-invariant normalized stress of `positions` against `hops`; disconnected pairs are skipped.
pub fn layout_run_normalized_stress(positions: &[[f64; 2]], hops: &[Vec<u32>]) -> f64 {
    let mut pairs = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            if hops[i][j] == u32::MAX {
                continue;
            }
            let hop = f64::from(hops[i][j]);
            let euclid = ((positions[i][0] - positions[j][0]).powi(2) + (positions[i][1] - positions[j][1]).powi(2)).sqrt();
            pairs.push((euclid, hop));
        }
    }
    if pairs.is_empty() {
        return 0.0;
    }
    let scale = pairs.iter().map(|(euclid, hop)| euclid / hop).sum::<f64>() / pairs.iter().map(|(euclid, hop)| euclid * euclid / (hop * hop)).sum::<f64>();
    pairs.iter().map(|(euclid, hop)| ((scale * euclid - hop) / hop).powi(2)).sum::<f64>() / pairs.len() as f64
}

/// 🌀️ The `fdg-sim` Fruchterman-Reingold layout of `graph` started from `initial`.
pub fn layout_run_fdg_layout(graph: &LayoutRunGraph, initial: &[LayoutRunPoint], parameters: LayoutRunOracleParameters) -> Vec<[f64; 2]> {
    let mut force_graph: ForceGraph<(), ()> = ForceGraph::default();
    let indices: Vec<_> = (0..graph.nodes.len()).map(|index| force_graph.add_force_node(format!("{index}"), ())).collect();
    for edge in &graph.edges {
        force_graph.add_edge(indices[edge.source as usize], indices[edge.target as usize], ());
    }
    let mut simulation = Simulation::from_graph(force_graph, SimulationParameters::new(200.0, Dimensions::Two, fruchterman_reingold(parameters.fdg_scale, parameters.fdg_cooloff)));
    for (index, node) in indices.iter().enumerate() {
        let weight = &mut simulation.get_graph_mut()[*node];
        let location = Vec3::new(initial[index].x as f32, initial[index].y as f32, 0.0);
        weight.location = location;
        weight.old_location = location;
        weight.velocity = Vec3::ZERO;
    }
    for _ in 0..parameters.fdg_updates {
        simulation.update(parameters.fdg_dt);
    }
    indices.iter().map(|node| simulation.get_graph()[*node].location).map(|location| [f64::from(location.x), f64::from(location.y)]).collect()
}

/// 🪜️ `petgraph` longest-path layering of the directed `edges` over `count` nodes: every source sits on layer 0 and
/// every other node one layer past its deepest predecessor (`None` = cyclic).
pub fn layout_run_longest_path_layers(count: usize, edges: &[(u32, u32)]) -> Option<Vec<u32>> {
    let mut graph = fdg_sim::petgraph::graph::DiGraph::<(), ()>::with_capacity(count, edges.len());
    let nodes: Vec<_> = (0..count).map(|_| graph.add_node(())).collect();
    for (source, target) in edges {
        graph.add_edge(nodes[*source as usize], nodes[*target as usize], ());
    }
    let order = fdg_sim::petgraph::algo::toposort(&graph, None).ok()?;
    let mut layers = vec![0_u32; count];
    for node in order {
        let layer = layers[node.index()];
        for next in graph.neighbors(node) {
            layers[next.index()] = layers[next.index()].max(layer + 1);
        }
    }
    Some(layers)
}

/// 📼️ Everything a driven run published, folded like the ToolRun ledger folds ticks.
#[derive(Debug, Default)]
pub struct LayoutRunRecording {
    pub ops: Vec<Vec<u8>>,
    pub entities: Vec<(u32, u64)>,
    pub upserts: Vec<(u64, ToolRunVerdict, u16)>,
    pub trace: Option<ToolRunTraceStore>,
    pub steps: Vec<ToolRunStep>,
    pub progress: Option<ToolRunProgress>,
    pub ticks: u32,
    pub checkpoints: u32,
    pub step_micros: Vec<u64>,
    pub complete: bool,
}

impl LayoutRunRecording {
    /// 🧮️ The provisional entity set after every retract.
    pub fn entity_set(&self) -> BTreeSet<u64> {
        self.entities.iter().map(|(_, entity)| *entity).collect()
    }

    fn apply(&mut self, tick: ToolRunTick) -> Result<(), String> {
        self.ticks += 1;
        if let Some(length) = tick.retract_to {
            self.ops.truncate(length as usize);
            self.entities.retain(|(mark, _)| *mark <= length);
        }
        self.ops.extend(tick.append_ops);
        let end = self.ops.len() as u32;
        self.entities.extend(tick.append_entities.into_iter().map(|entity| (end, entity)));
        let iterating = tick.progress.as_ref().is_some_and(|progress| progress.stage != LayoutRunStage::Initialize.index());
        let store = self.trace.get_or_insert_with(|| ToolRunTraceStore::new(tick.identity));
        for page in &tick.trace {
            store.apply_page(page).map_err(|rejection| format!("{rejection:?}"))?;
            if iterating {
                self.upserts.extend(page.ops.iter().filter_map(|op| if let ToolRunTraceOp::Upsert { key, verdict, reason, .. } = op { Some((*key, *verdict, *reason)) } else { None }));
            }
        }
        self.steps.extend(tick.steps);
        if tick.progress.is_some() {
            self.progress = tick.progress;
        }
        Ok(())
    }
}

fn frozen_now() -> Option<u64> {
    Some(0)
}

fn payload_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).filter_map(|index| payload.page(index)).flatten().copied().collect()
}

fn wallet() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 18, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 }
}

const SOURCE: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 28, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 };

fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), SOURCE, frozen_now, u64::MAX, 1 << 28)
}

/// ▶️ Drives `job` through `drive_step` to `Complete` with `fuel` per step and a frozen clock, so the slicing is
/// the job's own (one iteration per step) and every recorded step time is a whole, never deadline-cut step.
pub fn layout_run_drive<J: InteractiveJob + ?Sized>(job: &mut J, fuel: u64) -> Result<LayoutRunRecording, String> {
    let mut recording = LayoutRunRecording::default();
    let grant = wallet();
    let mut caller = caller();
    caller.birth_returns(SOURCE);
    let mut origin = OriginalJobReturnOrigin::empty();
    let mut verdict = None;
    let result = (|| {
        let mut ready = false;
        for _ in 0..4096 {
            ready = caller.turn(SOURCE, |cx| cx.with_original_work_loan(16, cx.original_absolute_deadline_us(), |cx| origin.admit(cx)))
                .map_err(|error| format!("{error:?}"))?;
            if ready {
                break;
            }
        }
        if !ready {
            return Err("layout run return origin was not admitted".into());
        }
        while !recording.complete {
            let started = std::time::Instant::now();
            let mut returned = None;
            let mut continuation = None;
            caller.refreshed_turn(grant, |cx| cx.with_original_work_loan(fuel, cx.original_absolute_deadline_us(), |cx| {
                match drive_step(job, cx, origin.receiver()?, "layout-run-testing", InteractiveStage::InteractiveStep, &mut verdict)? {
                    OriginalJobReturn::Preparing | OriginalJobReturn::Pending => {}
                    OriginalJobReturn::Outcome(reply) => returned = Some(reply.into_descriptor()),
                    OriginalJobReturn::Continuation(failure) => continuation = Some(failure),
                }
                Ok(())
            })).map_err(|error| format!("{error:?}"))?;
            recording.step_micros.push(started.elapsed().as_micros() as u64);
            if continuation.is_some() {
                while continuation.is_some() {
                    caller.refreshed_turn(grant, |cx| origin.receive_continuation(cx, &mut continuation)).map_err(|error| format!("{error:?}"))?;
                }
                let message = origin.continuation().and_then(|failure| failure.view().callback)
                    .and_then(|failure| failure.primary_fault()).map(|failure| failure.message.to_string())
                    .unwrap_or_else(|| "layout run failed in its original turn".into());
                return Err(message);
            }
            if returned.is_none() {
                continue;
            }
            while returned.is_some() {
                caller.refreshed_turn(grant, |cx| origin.receive_outcome(cx, &mut returned)).map_err(|error| format!("{error:?}"))?;
            }
            let descriptor = origin.outcome().expect("received layout run outcome");
            let observed = match job.borrow_outcome(descriptor).map_err(|error| format!("{error:?}"))? {
                JobOutcomeView::PreviewReady { payload, .. } => ToolRunTick::decode(&payload_bytes(payload)).map_err(|error| format!("{error:?}")).and_then(|tick| recording.apply(tick)),
                JobOutcomeView::CheckpointReady { .. } => {
                    recording.checkpoints += 1;
                    Ok(())
                }
                JobOutcomeView::Complete { .. } => {
                    recording.complete = true;
                    Ok(())
                }
                JobOutcomeView::Yield { .. } => Ok(()),
                JobOutcomeView::Cancelled { .. } => Err("layout run cancelled".to_string()),
                JobOutcomeView::Fault { detail, .. } => Err(String::from_utf8_lossy(&payload_bytes(detail)).into_owned()),
            };
            while origin.outcome().is_some() {
                caller.refreshed_turn(grant, |cx| origin.acknowledge_outcome(cx)).map_err(|error| format!("{error:?}"))?;
            }
            observed?;
        }
        Ok(())
    })();
    for _ in 0..4096 {
        if origin.terminal_is_empty() {
            break;
        }
        caller.turn(SOURCE, |cx| origin.close_step(cx).map(|_| ())).map_err(|error| format!("{error:?}"))?;
    }
    assert!(origin.terminal_is_empty(), "layout run return origin did not close");
    if result.is_err() {
        close_on_caller(job, &mut caller);
    }
    caller.retire(SOURCE);
    result.map(|()| recording)
}

/// 🧹️ Closes a job the way the ledger does, one owned buffer per `close_step`.
pub fn layout_run_close<J: InteractiveJob + ?Sized>(job: &mut J) {
    let mut caller = caller();
    close_on_caller(job, &mut caller);
    caller.retire(SOURCE);
}

fn close_on_caller<J: InteractiveJob + ?Sized>(job: &mut J, caller: &mut OriginalJobFixtureCaller) {
    job.begin_close();
    for _ in 0..4096 {
        let step = caller.turn(wallet(), |cx| cx.with_original_retirement(|cx| job.close_step(cx))).expect("layout close original failure custody");
        if matches!(step, InteractiveJobCloseStep::Complete { .. }) {
            return;
        }
    }
    panic!("layout run job did not close");
}
