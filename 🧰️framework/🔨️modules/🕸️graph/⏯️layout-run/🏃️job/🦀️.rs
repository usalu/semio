//! 🏃️ Layout run job: the Barnes-Hut quadtree and the stepped, cancellable `InteractiveJob` that drives iterations.

use super::*;

//#region 🔖️Quadtree
#[derive(Clone, Copy, Debug)]
struct LayoutRunCell {
    pub(crate) center: [f64; 2],
    half: f64,
    mass: f64,
    moment: [f64; 2],
    children: [u32; 4],
    body: u32,
    pub(crate) count: u32,
    depth: u32,
}

impl LayoutRunCell {
    fn empty(center: [f64; 2], half: f64, depth: u32) -> Self {
        Self { center, half, mass: 0.0, moment: [0.0, 0.0], children: [0; 4], body: NONE, count: 0, depth }
    }

    fn is_leaf(&self) -> bool {
        self.children == [0; 4]
    }

    fn quadrant(&self, point: [f64; 2]) -> usize {
        usize::from(point[0] >= self.center[0]) | (usize::from(point[1] >= self.center[1]) << 1)
    }
}

/// 🌳️ Arena Barnes-Hut quadtree; cleared and rebuilt every iteration with retained capacity.
#[derive(Debug, Default)]
pub(crate) struct LayoutRunTree {
    pub(crate) cells: Vec<LayoutRunCell>,
    next: Vec<u32>,
    stack: Vec<u32>,
}

impl LayoutRunTree {
    pub(crate) fn reset(&mut self, bounds: [f64; 4], bodies: usize) {
        self.cells.clear();
        self.next.clear();
        self.next.resize(bodies, NONE);
        let half = ((bounds[2] - bounds[0]).max(bounds[3] - bounds[1]) * 0.5).max(1.0) + 1.0;
        self.cells.push(LayoutRunCell::empty([(bounds[0] + bounds[2]) * 0.5, (bounds[1] + bounds[3]) * 0.5], half, 0));
    }

    fn child(&mut self, cell: usize, quadrant: usize) -> usize {
        let existing = self.cells[cell].children[quadrant];
        if existing != 0 {
            return existing as usize;
        }
        let parent = self.cells[cell];
        let offset = parent.half * 0.5;
        let center = [parent.center[0] + if quadrant & 1 == 1 { offset } else { -offset }, parent.center[1] + if quadrant & 2 == 2 { offset } else { -offset }];
        let index = self.cells.len();
        self.cells.push(LayoutRunCell::empty(center, offset, parent.depth + 1));
        self.cells[cell].children[quadrant] = index as u32;
        index
    }

    pub(crate) fn insert(&mut self, body: u32, positions: &[[f64; 2]], masses: &[f64]) {
        let point = positions[body as usize];
        let mass = masses[body as usize];
        let mut cell = 0usize;
        loop {
            let current = &mut self.cells[cell];
            current.mass += mass;
            current.moment[0] += mass * point[0];
            current.moment[1] += mass * point[1];
            current.count += 1;
            if current.is_leaf() {
                if current.count == 1 {
                    current.body = body;
                    self.next[body as usize] = NONE;
                    return;
                }
                if current.depth >= LAYOUT_RUN_TREE_DEPTH_MAX {
                    self.next[body as usize] = current.body;
                    current.body = body;
                    return;
                }
                let existing = current.body;
                current.body = NONE;
                let quadrant = current.quadrant(positions[existing as usize]);
                let child = self.child(cell, quadrant);
                let existing_mass = masses[existing as usize];
                let existing_point = positions[existing as usize];
                let target = &mut self.cells[child];
                target.mass = existing_mass;
                target.moment = [existing_mass * existing_point[0], existing_mass * existing_point[1]];
                target.count = 1;
                target.body = existing;
            }
            let quadrant = self.cells[cell].quadrant(point);
            cell = self.child(cell, quadrant);
        }
    }

    pub(crate) fn repulsion(&mut self, body: u32, positions: &[[f64; 2]], radii: &[f64], strength: f64, falloff: LayoutRunFalloff, theta_squared: f64) -> [f64; 2] {
        let point = positions[body as usize];
        let radius = radii[body as usize];
        let mut force = [0.0, 0.0];
        self.stack.clear();
        self.stack.push(0);
        while let Some(index) = self.stack.pop() {
            let cell = self.cells[index as usize];
            if cell.count == 0 {
                continue;
            }
            if cell.is_leaf() {
                let mut other = cell.body;
                while other != NONE {
                    if other != body {
                        let push = pairwise_repulsion(point, positions[other as usize], radius, radii[other as usize], strength, falloff);
                        force[0] += push[0];
                        force[1] += push[1];
                    }
                    other = self.next[other as usize];
                }
                continue;
            }
            let dx = cell.moment[0] / cell.mass - point[0];
            let dy = cell.moment[1] / cell.mass - point[1];
            let distance_squared = dx * dx + dy * dy;
            let inside = (point[0] - cell.center[0]).abs() <= cell.half && (point[1] - cell.center[1]).abs() <= cell.half;
            let size = 2.0 * cell.half;
            if !inside && size * size < theta_squared * distance_squared {
                let distance = distance_squared.sqrt().max(MIN_DISTANCE);
                let magnitude = strength * radius * cell.mass / falloff.divisor(distance);
                force[0] -= dx / distance * magnitude;
                force[1] -= dy / distance * magnitude;
            } else {
                self.stack.extend(cell.children.iter().copied().filter(|child| *child != 0));
            }
        }
        force
    }
}

#[inline]
pub(crate) fn pairwise_repulsion(point: [f64; 2], other: [f64; 2], radius: f64, other_radius: f64, strength: f64, falloff: LayoutRunFalloff) -> [f64; 2] {
    let dx = other[0] - point[0];
    let dy = other[1] - point[1];
    let distance = (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);
    let magnitude = strength * (radius * other_radius).max(1.0) / falloff.divisor(distance);
    [-dx / distance * magnitude, -dy / distance * magnitude]
}

fn split_mix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub(crate) fn unit_interval(state: &mut u64) -> f64 {
    (split_mix64(state) >> 11) as f64 / (1u64 << 53) as f64
}
//#endregion 🔖️Quadtree

//#region 🔖️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LayoutRunPhase {
    Initialize { cursor: u32 },
    Tree { cursor: u32 },
    Repulse { cursor: u32 },
    Attract { cursor: u32 },
    Integrate { cursor: u32 },
    Compact { cursor: u32, settle: bool },
    Done,
}

/// ⏯️ The stepped layout run. Build with `new` or `resume`, box it as the ledger's `ToolRunJob`.
pub struct LayoutRunJob<E: LayoutRunOpEncoder> {
    encoder: E,
    pub(crate) config: LayoutRunConfig,
    writer: ToolRunTickWriter,
    pub(crate) graph_digest: u64,
    pub(crate) entities: Vec<u64>,
    radii: Vec<f64>,
    pub(crate) pinned: Vec<bool>,
    origins: Vec<[f64; 2]>,
    anchors: Vec<[f64; 2]>,
    pub(crate) edges: Vec<(u32, u32, f64)>,
    pub(crate) positions: Vec<[f64; 2]>,
    velocities: Vec<[f64; 2]>,
    forces: Vec<[f64; 2]>,
    displacements: Vec<f64>,
    emitted: Vec<[f64; 2]>,
    epoch_entities: Vec<bool>,
    pub(crate) reasons: Vec<u16>,
    diverged: Vec<bool>,
    tree: LayoutRunTree,
    pub(crate) center: [f64; 2],
    bounds: [f64; 4],
    phase: LayoutRunPhase,
    checkpoint_owed: bool,
    resumed: bool,
    pub(crate) iteration: u32,
    pub(crate) settle_streak: u32,
    energy: f64,
    max_displacement: f64,
    settled: u32,
    diverged_count: u32,
    emit_cursor: u32,
    pub(crate) checkpoints: u32,
    stop: Option<LayoutRunStop>,
    closing: bool,
    publication: RetainedJobPublication,
    staged: Option<(JobPublicationKind, Vec<u8>)>,
    pub(crate) delivered: bool,
}

impl<E: LayoutRunOpEncoder> LayoutRunJob<E> {
    /// 🌱️ A fresh run: unplaced nodes are seeded from `config.seed` around their anchor or the placed centroid.
    pub fn new(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E) -> Result<Self, LayoutRunStartError> {
        graph.validate().map_err(LayoutRunStartError::Graph)?;
        config.validate().map_err(LayoutRunStartError::Config)?;
        let mut job = Self::allocate(identity, 0, graph, config, encoder);
        job.seed(graph);
        job.center = config.center.map_or_else(|| job.centroid(), |center| [center.x, center.y]);
        job.refresh_bounds();
        Ok(job)
    }

    /// ⏩️ Continues from a checkpoint of the same graph with (possibly changed) settings. `positions` are the nodes'
    /// current positions (the ToolRun overlay), `provisional_len` the ledger's provisional op count.
    pub fn resume(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E, positions: &[LayoutRunPoint], checkpoint: &[u8], provisional_len: u32) -> Result<Self, LayoutRunResumeError> {
        graph.validate().map_err(LayoutRunResumeError::Graph)?;
        config.validate().map_err(LayoutRunResumeError::Config)?;
        let checkpoint = LayoutRunCheckpoint::decode(checkpoint).ok_or(LayoutRunResumeError::Malformed)?;
        if checkpoint.node_count as usize != graph.nodes.len() || checkpoint.graph_digest != graph.digest() || checkpoint.provisional_len > provisional_len {
            return Err(LayoutRunResumeError::Foreign);
        }
        if positions.len() != graph.nodes.len() || positions.iter().any(|point| !(point.x.is_finite() && point.y.is_finite())) {
            return Err(LayoutRunResumeError::Positions);
        }
        let mut job = Self::allocate(identity, provisional_len, graph, config, encoder);
        for (index, point) in positions.iter().enumerate() {
            let position = if job.pinned[index] { job.origins[index] } else { [point.x, point.y] };
            job.positions[index] = position;
            job.emitted[index] = position;
            job.epoch_entities[index] = job.moved(index);
        }
        job.iteration = checkpoint.iteration;
        job.settle_streak = checkpoint.settle_streak;
        job.center = config.center.map_or(checkpoint.center, |center| [center.x, center.y]);
        job.resumed = true;
        job.refresh_bounds();
        Ok(job)
    }

    fn allocate(identity: ToolRunIdentity, provisional_len: u32, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E) -> Self {
        let count = graph.nodes.len();
        let point = |point: Option<LayoutRunPoint>| point.map_or([f64::NAN, f64::NAN], |point| [point.x, point.y]);
        let origins: Vec<[f64; 2]> = graph.nodes.iter().map(|node| point(node.origin)).collect();
        Self {
            encoder,
            config,
            writer: ToolRunTickWriter::with_provisional_base(identity, provisional_len),
            graph_digest: graph.digest(),
            entities: graph.nodes.iter().map(|node| node.entity).collect(),
            radii: graph.nodes.iter().map(|node| node.radius).collect(),
            pinned: graph.nodes.iter().map(|node| node.pinned).collect(),
            anchors: graph.nodes.iter().map(|node| point(node.anchor)).collect(),
            edges: graph.edges.iter().map(|edge| (edge.source, edge.target, edge.weight)).collect(),
            positions: origins.clone(),
            emitted: origins.clone(),
            origins,
            velocities: vec![[0.0, 0.0]; count],
            forces: vec![[0.0, 0.0]; count],
            displacements: vec![f64::INFINITY; count],
            epoch_entities: vec![false; count],
            reasons: vec![u16::MAX; count],
            diverged: vec![false; count],
            tree: LayoutRunTree::default(),
            center: [0.0, 0.0],
            bounds: [0.0; 4],
            phase: LayoutRunPhase::Initialize { cursor: 0 },
            checkpoint_owed: false,
            resumed: false,
            iteration: 0,
            settle_streak: 0,
            energy: 0.0,
            max_displacement: 0.0,
            settled: 0,
            diverged_count: 0,
            emit_cursor: 0,
            checkpoints: 0,
            stop: None,
            closing: false,
            publication: RetainedJobPublication::new(),
            staged: None,
            delivered: false,
        }
    }

    fn seed(&mut self, graph: &LayoutRunGraph) {
        let placed: Vec<[f64; 2]> = self.origins.iter().copied().filter(|origin| origin[0].is_finite()).collect();
        let fallback = self.config.center.map_or([0.0, 0.0], |center| [center.x, center.y]);
        let anchor = if placed.is_empty() { fallback } else { [placed.iter().map(|point| point[0]).sum::<f64>() / placed.len() as f64, placed.iter().map(|point| point[1]).sum::<f64>() / placed.len() as f64] };
        let unplaced = graph.nodes.len() - placed.len();
        let side = self.config.ideal_edge_length * (unplaced.max(1) as f64).sqrt();
        let mut state = self.config.seed;
        for index in 0..graph.nodes.len() {
            if self.positions[index][0].is_finite() {
                continue;
            }
            let around = if self.anchors[index][0].is_finite() { self.anchors[index] } else { anchor };
            let u = unit_interval(&mut state);
            let v = unit_interval(&mut state);
            self.positions[index] = [around[0] + (u - 0.5) * side, around[1] + (v - 0.5) * side];
        }
    }

    fn centroid(&self) -> [f64; 2] {
        if self.positions.is_empty() {
            return [0.0, 0.0];
        }
        let count = self.positions.len() as f64;
        [self.positions.iter().map(|point| point[0]).sum::<f64>() / count, self.positions.iter().map(|point| point[1]).sum::<f64>() / count]
    }

    fn refresh_bounds(&mut self) {
        self.bounds = self.positions.iter().fold([f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY], |bounds, point| [bounds[0].min(point[0]), bounds[1].min(point[1]), bounds[2].max(point[0]), bounds[3].max(point[1])]);
    }

    pub(crate) fn moved(&self, index: usize) -> bool {
        !self.pinned[index] && (self.origins[index][0].is_nan() || self.positions[index] != self.origins[index])
    }

    pub fn identity(&self) -> ToolRunIdentity {
        self.writer.identity()
    }

    pub fn stage(&self) -> LayoutRunStage {
        match self.phase {
            LayoutRunPhase::Initialize { .. } => LayoutRunStage::Initialize,
            LayoutRunPhase::Compact { settle: true, .. } | LayoutRunPhase::Done => LayoutRunStage::Settle,
            _ => LayoutRunStage::Iterate,
        }
    }

    pub fn iteration(&self) -> u32 {
        self.iteration
    }

    pub fn stop(&self) -> Option<LayoutRunStop> {
        self.stop
    }

    pub fn checkpoints(&self) -> u32 {
        self.checkpoints
    }

    pub fn positions(&self) -> Vec<LayoutRunPoint> {
        self.positions.iter().map(|point| LayoutRunPoint::new(point[0], point[1])).collect()
    }

    pub fn positions_digest(&self) -> u64 {
        layout_run_positions_digest(&self.positions)
    }

    /// 🧮️ Counter values in `LayoutRunCounter::ALL` order.
    pub fn counters(&self) -> [u64; 4] {
        [u64::from(self.iteration), (self.energy * 1000.0).round() as u64, (self.max_displacement * 1000.0).round() as u64, u64::from(self.settled)]
    }

    pub fn checkpoint(&self) -> LayoutRunCheckpoint {
        LayoutRunCheckpoint { node_count: self.positions.len() as u32, graph_digest: self.graph_digest, iteration: self.iteration, settle_streak: self.settle_streak, provisional_len: self.writer.provisional_len(), center: self.center }
    }

    fn cooling(&self) -> f64 {
        (1.0 - f64::from(self.iteration) / f64::from(self.config.max_iterations)).max(self.config.cooling_floor)
    }

    fn uses_tree(&self) -> bool {
        self.positions.len() > self.config.pairwise_max_bodies as usize && self.config.barnes_hut_theta > 0.0
    }

    fn compact_threshold(&self) -> u32 {
        self.config.compact_ops.max(2 * self.positions.len() as u32).min(TOOL_RUN_PROVISIONAL_OPS_MAX)
    }

    fn trace_reason(&self, index: usize) -> LayoutRunReason {
        if self.pinned[index] {
            LayoutRunReason::Pinned
        } else if self.diverged[index] {
            LayoutRunReason::Diverged
        } else if self.displacements[index] < self.config.settle_displacement {
            LayoutRunReason::Settled
        } else {
            LayoutRunReason::Moving
        }
    }

    pub(crate) fn final_reason(&self, index: usize) -> LayoutRunReason {
        match self.trace_reason(index) {
            LayoutRunReason::Moving => LayoutRunReason::Unsettled,
            reason => reason,
        }
    }

    fn upsert(&mut self, index: usize, reason: LayoutRunReason) {
        self.reasons[index] = reason.code();
        self.writer.upsert(index as u64, reason.verdict(), reason.code(), ToolRunTraceSubject::Entity { entity: self.entities[index] });
    }

    fn append_move(&mut self, index: usize) -> Result<(), String> {
        let position = self.positions[index];
        let op = self.encoder.encode_move(index as u32, self.entities[index], LayoutRunPoint::new(position[0], position[1])).map_err(|error| error.0)?;
        if op.len() > LAYOUT_RUN_OP_BYTES_MAX {
            return Err(format!("layout move op of node {index} takes {} bytes, above {LAYOUT_RUN_OP_BYTES_MAX}", op.len()));
        }
        self.writer.append_op(op).map_err(|error| format!("{error:?}"))?;
        if !self.epoch_entities[index] {
            self.epoch_entities[index] = true;
            self.writer.append_entity(self.entities[index]);
        }
        self.emitted[index] = position;
        Ok(())
    }

    pub(crate) fn progress(&mut self, state: ToolRunState) {
        let counters = self.counters();
        self.writer.progress(ToolRunProgress {
            identity: self.writer.identity(),
            sequence: 0,
            state,
            stage: self.stage().index(),
            completed: u64::from(self.iteration),
            total: Some(u64::from(self.config.max_iterations.max(self.iteration))),
            counters: LayoutRunCounter::ALL.iter().map(|counter| ToolRunCounter { counter: counter.index(), value: counters[counter.index() as usize] }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        });
    }

    fn step_args(&self) -> [ToolRunStepArg; 3] {
        [ToolRunStepArg::Unsigned(u64::from(self.iteration)), ToolRunStepArg::Float(self.max_displacement), ToolRunStepArg::Float(self.energy)]
    }

    fn publish(&mut self, kind: JobPublicationKind, bytes: Vec<u8>) -> LayoutRunTurn {
        self.staged = Some((kind, bytes));
        LayoutRunTurn::Staged
    }

    fn flush(&mut self) -> LayoutRunTurn {
        let Some(tick) = self.writer.finish() else { return LayoutRunTurn::Yield };
        match tick.encode() {
            Ok(bytes) if bytes.len() <= JOB_PAYLOAD_PAGE_BYTES => self.publish(JobPublicationKind::Preview, bytes),
            Ok(bytes) => self.fault(&format!("layout run tick takes {} bytes, above one payload page", bytes.len())),
            Err(error) => self.fault(&format!("layout run tick does not encode: {error:?}")),
        }
    }

    fn fault(&mut self, message: &str) -> LayoutRunTurn {
        self.publish(JobPublicationKind::Fault, message.as_bytes()[..message.len().min(JOB_PAYLOAD_PAGE_BYTES)].to_vec())
    }

    fn initialize(&mut self, _cx: &mut StepContext<'_>, cursor: u32) -> LayoutRunTurn {
        let count = self.positions.len();
        if cursor == 0 {
            let (reason, args) = if self.resumed { (LayoutRunReason::Resumed, vec![ToolRunStepArg::Unsigned(u64::from(self.iteration))]) } else { (LayoutRunReason::Initialized, vec![ToolRunStepArg::Unsigned(count as u64), ToolRunStepArg::Unsigned(self.edges.len() as u64)]) };
            if self.writer.step(ToolRunStepKind::Info, LayoutRunStage::Initialize.index(), reason.code(), None, &args).is_err() {
                return self.fault("layout run step arguments exceed the limit");
            }
        }
        let mut index = cursor as usize;
        if !self.resumed {
            while index < count && self.writer.pending_bytes() < LAYOUT_RUN_TICK_FLUSH_BYTES {
                let reason = if self.pinned[index] { LayoutRunReason::Pinned } else { LayoutRunReason::Moving };
                self.upsert(index, reason);
                index += 1;
            }
        }
        self.progress(ToolRunState::Running);
        if self.resumed || index == count {
            self.phase = if count == 0 {
                self.settle_phase(LayoutRunStop::Converged)
            } else if self.iteration >= self.config.max_iterations {
                self.settle_phase(LayoutRunStop::IterationLimit)
            } else {
                LayoutRunPhase::Tree { cursor: 0 }
            };
        } else {
            self.phase = LayoutRunPhase::Initialize { cursor: index as u32 };
        }
        self.flush()
    }

    fn settle_phase(&mut self, stop: LayoutRunStop) -> LayoutRunPhase {
        self.stop = Some(stop);
        LayoutRunPhase::Compact { cursor: 0, settle: true }
    }

    fn iterate(&mut self, cx: &mut StepContext<'_>) -> Option<LayoutRunTurn> {
        let count = self.positions.len();
        let cool = self.cooling();
        loop {
            if cx.is_cancelled() {
                return Some(LayoutRunTurn::Cancelled);
            }
            match self.phase {
                LayoutRunPhase::Tree { cursor } => {
                    if !self.uses_tree() {
                        self.forces.iter_mut().for_each(|force| *force = [0.0, 0.0]);
                        self.phase = LayoutRunPhase::Repulse { cursor: 0 };
                        continue;
                    }
                    if cursor == 0 {
                        self.forces.iter_mut().for_each(|force| *force = [0.0, 0.0]);
                        self.tree.reset(self.bounds, count);
                    }
                    let end = (cursor as usize + LAYOUT_RUN_LINEAR_CHUNK).min(count);
                    for body in cursor as usize..end {
                        self.tree.insert(body as u32, &self.positions, &self.radii);
                    }
                    self.phase = if end == count { LayoutRunPhase::Repulse { cursor: 0 } } else { LayoutRunPhase::Tree { cursor: end as u32 } };
                }
                LayoutRunPhase::Repulse { cursor } => {
                    let strength = self.config.repulsion_strength * cool;
                    let end = (cursor as usize + LAYOUT_RUN_REPULSION_CHUNK).min(count);
                    if self.uses_tree() {
                        let theta_squared = self.config.barnes_hut_theta * self.config.barnes_hut_theta;
                        for body in cursor as usize..end {
                            let push = self.tree.repulsion(body as u32, &self.positions, &self.radii, strength, self.config.repulsion_falloff, theta_squared);
                            self.forces[body][0] += push[0];
                            self.forces[body][1] += push[1];
                        }
                    } else {
                        for body in cursor as usize..end {
                            for other in body + 1..count {
                                let push = pairwise_repulsion(self.positions[body], self.positions[other], self.radii[body], self.radii[other], strength, self.config.repulsion_falloff);
                                self.forces[body][0] += push[0];
                                self.forces[body][1] += push[1];
                                self.forces[other][0] -= push[0];
                                self.forces[other][1] -= push[1];
                            }
                        }
                    }
                    self.phase = if end == count { LayoutRunPhase::Attract { cursor: 0 } } else { LayoutRunPhase::Repulse { cursor: end as u32 } };
                }
                LayoutRunPhase::Attract { cursor } => {
                    let strength = self.config.spring_strength * cool;
                    let ideal = self.config.ideal_edge_length;
                    let end = (cursor as usize + LAYOUT_RUN_LINEAR_CHUNK).min(self.edges.len());
                    for edge in cursor as usize..end {
                        let (source, target, weight) = self.edges[edge];
                        let (a, b) = (self.positions[source as usize], self.positions[target as usize]);
                        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
                        let distance = (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);
                        let magnitude = strength * self.config.spring_law.stretch(distance, ideal) * weight;
                        let pull = [dx / distance * magnitude, dy / distance * magnitude];
                        self.forces[source as usize][0] += pull[0];
                        self.forces[source as usize][1] += pull[1];
                        self.forces[target as usize][0] -= pull[0];
                        self.forces[target as usize][1] -= pull[1];
                    }
                    self.phase = if end == self.edges.len() { LayoutRunPhase::Integrate { cursor: 0 } } else { LayoutRunPhase::Attract { cursor: end as u32 } };
                }
                LayoutRunPhase::Integrate { cursor } => {
                    if cursor == 0 {
                        self.energy = 0.0;
                        self.max_displacement = 0.0;
                        self.settled = 0;
                        self.diverged_count = 0;
                        self.bounds = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
                    }
                    let end = (cursor as usize + LAYOUT_RUN_LINEAR_CHUNK).min(count);
                    for body in cursor as usize..end {
                        self.integrate(body, cool);
                    }
                    if end < count {
                        self.phase = LayoutRunPhase::Integrate { cursor: end as u32 };
                    } else {
                        return Some(self.finish_iteration(cx));
                    }
                }
                _ => return None,
            }
            if cx.deadline_exceeded() {
                return Some(LayoutRunTurn::Yield);
            }
        }
    }

    fn integrate(&mut self, body: usize, cool: f64) {
        let point = self.positions[body];
        if self.pinned[body] {
            self.positions[body] = self.origins[body];
            self.velocities[body] = [0.0, 0.0];
            self.displacements[body] = 0.0;
            self.diverged[body] = false;
        } else {
            let gravity = self.config.gravity * cool;
            let anchor_strength = self.config.anchor_strength * cool;
            let mut force = self.forces[body];
            force[0] += (self.center[0] - point[0]) * gravity;
            force[1] += (self.center[1] - point[1]) * gravity;
            let anchor = self.anchors[body];
            if anchor[0].is_finite() {
                let (dx, dy) = (anchor[0] - point[0], anchor[1] - point[1]);
                let pull = anchor_strength * self.config.spring_law.anchor_scale((dx * dx + dy * dy).sqrt(), self.config.ideal_edge_length);
                force[0] += dx * pull;
                force[1] += dy * pull;
            }
            let dt = self.config.time_step * cool.sqrt();
            let damping = self.config.velocity_damping;
            let mut velocity = [(self.velocities[body][0] + force[0] * dt) * damping, (self.velocities[body][1] + force[1] * dt) * damping];
            let speed = (velocity[0] * velocity[0] + velocity[1] * velocity[1]).sqrt();
            if speed > self.config.max_speed {
                let scale = self.config.max_speed / speed;
                velocity = [velocity[0] * scale, velocity[1] * scale];
            }
            let next = [point[0] + velocity[0] * dt, point[1] + velocity[1] * dt];
            if next[0].is_finite() && next[1].is_finite() && velocity[0].is_finite() && velocity[1].is_finite() {
                let (dx, dy) = (next[0] - point[0], next[1] - point[1]);
                self.positions[body] = next;
                self.velocities[body] = velocity;
                self.displacements[body] = (dx * dx + dy * dy).sqrt();
                self.diverged[body] = false;
            } else {
                self.velocities[body] = [0.0, 0.0];
                self.displacements[body] = 0.0;
                self.diverged[body] = true;
                self.diverged_count += 1;
            }
        }
        let velocity = self.velocities[body];
        self.energy += 0.5 * (velocity[0] * velocity[0] + velocity[1] * velocity[1]);
        self.max_displacement = self.max_displacement.max(self.displacements[body]);
        if !self.diverged[body] && self.displacements[body] < self.config.settle_displacement {
            self.settled += 1;
        }
        let point = self.positions[body];
        self.bounds = [self.bounds[0].min(point[0]), self.bounds[1].min(point[1]), self.bounds[2].max(point[0]), self.bounds[3].max(point[1])];
    }

    fn finish_iteration(&mut self, cx: &mut StepContext<'_>) -> LayoutRunTurn {
        self.iteration += 1;
        cx.consume_fuel(1);
        self.settle_streak = if self.diverged_count == 0 && self.max_displacement < self.config.settle_displacement { self.settle_streak + 1 } else { 0 };
        if self.diverged_count > 0 && self.writer.step(ToolRunStepKind::Danger, LayoutRunStage::Iterate.index(), LayoutRunReason::Diverged.code(), None, &[ToolRunStepArg::Unsigned(u64::from(self.diverged_count))]).is_err() {
            return self.fault("layout run step arguments exceed the limit");
        }
        let threshold = self.compact_threshold();
        let count = self.positions.len();
        let mut index = self.emit_cursor as usize % count;
        let mut compact = false;
        for _ in 0..count {
            if self.writer.pending_bytes() >= LAYOUT_RUN_TICK_FLUSH_BYTES {
                break;
            }
            let reason = self.trace_reason(index);
            let emitted = self.emitted[index];
            let drift = if emitted[0].is_nan() { f64::INFINITY } else { ((self.positions[index][0] - emitted[0]).powi(2) + (self.positions[index][1] - emitted[1]).powi(2)).sqrt() };
            let moves = !self.pinned[index] && drift > 0.0 && drift >= self.config.emit_displacement;
            if moves && self.writer.provisional_len() >= threshold {
                compact = true;
                break;
            }
            if moves {
                if let Err(message) = self.append_move(index) {
                    return self.fault(&message);
                }
            }
            if moves || reason.code() != self.reasons[index] {
                self.upsert(index, reason);
            }
            index = (index + 1) % count;
        }
        self.emit_cursor = index as u32;
        self.progress(ToolRunState::Running);
        self.phase = if self.settle_streak >= self.config.settle_iterations {
            self.settle_phase(LayoutRunStop::Converged)
        } else if self.iteration >= self.config.max_iterations {
            self.settle_phase(LayoutRunStop::IterationLimit)
        } else if compact || self.iteration.is_multiple_of(self.config.checkpoint_iterations) || self.writer.provisional_len() >= threshold {
            LayoutRunPhase::Compact { cursor: 0, settle: false }
        } else {
            LayoutRunPhase::Tree { cursor: 0 }
        };
        self.flush()
    }

    fn compact(&mut self, _cx: &mut StepContext<'_>, cursor: u32, settle: bool) -> LayoutRunTurn {
        let count = self.positions.len();
        if cursor == 0 {
            self.writer.retract_to(0);
            self.epoch_entities.iter_mut().for_each(|flag| *flag = false);
            if !settle {
                self.velocities.iter_mut().for_each(|velocity| *velocity = [0.0, 0.0]);
            }
        }
        let mut index = cursor as usize;
        while index < count && self.writer.pending_bytes() < LAYOUT_RUN_TICK_FLUSH_BYTES {
            if self.moved(index) {
                if let Err(message) = self.append_move(index) {
                    return self.fault(&message);
                }
            } else {
                self.emitted[index] = self.positions[index];
            }
            if settle {
                let reason = self.final_reason(index);
                self.upsert(index, reason);
            }
            index += 1;
        }
        if index < count {
            self.phase = LayoutRunPhase::Compact { cursor: index as u32, settle };
            self.progress(ToolRunState::Running);
            return self.flush();
        }
        if settle {
            let (kind, reason) = match self.stop {
                Some(LayoutRunStop::IterationLimit) => (ToolRunStepKind::Warning, LayoutRunReason::IterationLimit),
                _ => (ToolRunStepKind::Success, LayoutRunReason::Converged),
            };
            if self.writer.step(kind, LayoutRunStage::Settle.index(), reason.code(), None, &self.step_args()).is_err() {
                return self.fault("layout run step arguments exceed the limit");
            }
            self.phase = LayoutRunPhase::Done;
            self.progress(ToolRunState::Complete);
        } else {
            self.checkpoints += 1;
            self.phase = LayoutRunPhase::Tree { cursor: 0 };
            self.checkpoint_owed = true;
            self.progress(ToolRunState::Running);
        }
        self.flush()
    }

    fn release_next(&mut self) -> Option<usize> {
        fn take<T>(values: &mut Vec<T>) -> Option<usize> {
            (values.capacity() != 0).then(|| {
                let bytes = values.capacity() * size_of::<T>();
                *values = Vec::new();
                bytes
            })
        }
        take(&mut self.positions)
            .or_else(|| take(&mut self.velocities))
            .or_else(|| take(&mut self.forces))
            .or_else(|| take(&mut self.displacements))
            .or_else(|| take(&mut self.emitted))
            .or_else(|| take(&mut self.origins))
            .or_else(|| take(&mut self.anchors))
            .or_else(|| take(&mut self.radii))
            .or_else(|| take(&mut self.entities))
            .or_else(|| take(&mut self.pinned))
            .or_else(|| take(&mut self.epoch_entities))
            .or_else(|| take(&mut self.reasons))
            .or_else(|| take(&mut self.diverged))
            .or_else(|| take(&mut self.edges))
            .or_else(|| take(&mut self.tree.cells))
            .or_else(|| take(&mut self.tree.next))
            .or_else(|| take(&mut self.tree.stack))
    }

    fn next_release_bytes(&self) -> usize {
        fn bytes<T>(values: &[T], capacity: usize) -> usize {
            capacity * size_of::<T>() + values.len() * 0
        }
        [
            bytes(&self.positions, self.positions.capacity()),
            bytes(&self.velocities, self.velocities.capacity()),
            bytes(&self.forces, self.forces.capacity()),
            bytes(&self.displacements, self.displacements.capacity()),
            bytes(&self.emitted, self.emitted.capacity()),
            bytes(&self.origins, self.origins.capacity()),
            bytes(&self.anchors, self.anchors.capacity()),
            bytes(&self.radii, self.radii.capacity()),
            bytes(&self.entities, self.entities.capacity()),
            bytes(&self.pinned, self.pinned.capacity()),
            bytes(&self.epoch_entities, self.epoch_entities.capacity()),
            bytes(&self.reasons, self.reasons.capacity()),
            bytes(&self.diverged, self.diverged.capacity()),
            bytes(&self.edges, self.edges.capacity()),
            bytes(&self.tree.cells, self.tree.cells.capacity()),
            bytes(&self.tree.next, self.tree.next.capacity()),
            bytes(&self.tree.stack, self.tree.stack.capacity()),
        ]
        .into_iter()
        .find(|bytes| *bytes != 0)
        .unwrap_or(0)
    }

    fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if let Some((_, bytes)) = self.staged.as_ref() {
            return Ok(RetirementDemand { release_bytes: bytes.capacity(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { release_bytes: self.next_release_bytes(), depth: usize::from(!self.owns_nothing()), ..Default::default() })
    }

    fn retire_delivered<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        let step = self.publication.close_step(cx.retained_grant()).map_err(|error| OriginalTurnFailure::debit_original(error, cx))?;
        cx.consume_retained(step.progress())?;
        if self.publication.terminal_is_empty() {
            if let Some((_, bytes)) = self.staged.as_ref() {
                let progress = RetainedCloneProgress { copied_items: 1, released_bytes: bytes.capacity(), ..Default::default() };
                if !progress.fits(cx.retained_grant()) {
                    return Ok(None);
                }
                cx.consume_retained(progress)?;
                self.staged = None;
            }
            self.delivered = false;
        }
        Ok(None)
    }

    fn owns_nothing(&self) -> bool {
        self.publication.terminal_is_empty()
            && self.staged.is_none()
            && self.positions.capacity() == 0
            && self.velocities.capacity() == 0
            && self.forces.capacity() == 0
            && self.displacements.capacity() == 0
            && self.emitted.capacity() == 0
            && self.origins.capacity() == 0
            && self.anchors.capacity() == 0
            && self.radii.capacity() == 0
            && self.entities.capacity() == 0
            && self.pinned.capacity() == 0
            && self.epoch_entities.capacity() == 0
            && self.reasons.capacity() == 0
            && self.diverged.capacity() == 0
            && self.edges.capacity() == 0
            && self.tree.cells.capacity() == 0
            && self.tree.next.capacity() == 0
            && self.tree.stack.capacity() == 0
    }
}

/// 🔁️ What one compute turn produced before its outcome is published or reported.
enum LayoutRunTurn {
    Yield,
    Cancelled,
    Complete,
    Staged,
}

impl<E: LayoutRunOpEncoder> InteractiveJob for LayoutRunJob<E> {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.delivered {
            return self.retire_delivered(cx);
        }
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if let Some((kind, bytes)) = self.staged.as_ref() {
            let outcome = self.publication.advance_from_source(*kind, bytes, cx)?;
            self.delivered = outcome.is_some();
            return Ok(outcome);
        }
        if std::mem::take(&mut self.checkpoint_owed) {
            let bytes = self.checkpoint().encode().to_vec();
            self.staged = Some((JobPublicationKind::Checkpoint { applied_progress: u64::from(self.iteration) }, bytes));
            return Ok(None);
        }
        let turn = match self.phase {
            LayoutRunPhase::Initialize { cursor } => self.initialize(cx, cursor),
            LayoutRunPhase::Compact { cursor, settle } => self.compact(cx, cursor, settle),
            LayoutRunPhase::Done => LayoutRunTurn::Complete,
            _ => self.iterate(cx).unwrap_or(LayoutRunTurn::Yield),
        };
        match turn {
            LayoutRunTurn::Yield | LayoutRunTurn::Staged => Ok(None),
            LayoutRunTurn::Cancelled => JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into),
            LayoutRunTurn::Complete => JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into),
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.writer = ToolRunTickWriter::new(self.writer.identity());
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if !self.closing { return Ok(InteractiveJobCloseStep::Blocked); }
        if self.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }); }
        if cx.should_yield() || grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        let demand = self.close_demands()?;
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        cx.consume_fuel(1);
        let progress = if !self.publication.terminal_is_empty() {
            self.publication.close_step(grant).map_err(|error| OriginalTurnFailure::debit_original(error, cx))?.progress()
        } else if let Some((_, bytes)) = self.staged.take() {
            let released_bytes = bytes.capacity();
            drop(bytes);
            self.delivered = false;
            RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }
        } else {
            let Some(released_bytes) = self.release_next() else { return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }); };
            RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }
        };
        cx.consume_retained(progress)?;
        Ok(InteractiveJobCloseStep::Pending { progress })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.owns_nothing()
    }
}

/// ⏩️ Where a ToolRun ledger resumes a layout run: its checkpoint, the nodes' current overlay positions and its
/// provisional op count.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutRunResume<'a> {
    pub checkpoint: &'a [u8],
    pub positions: Vec<LayoutRunPoint>,
    pub provisional_len: u32,
}

/// 🧵️ The consumer entry every layout tool's `build_tool_run_job` calls: resumes while the checkpoint and positions
/// still describe this graph, starts fresh otherwise; graph and config errors refuse either way.
pub fn layout_run_job<E: LayoutRunOpEncoder>(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: impl Fn() -> E, resume: Option<LayoutRunResume<'_>>) -> Result<LayoutRunJob<E>, LayoutRunStartError> {
    if let Some(resume) = resume {
        match LayoutRunJob::resume(identity, graph, config, encoder(), &resume.positions, resume.checkpoint, resume.provisional_len) {
            Ok(job) => return Ok(job),
            Err(LayoutRunResumeError::Graph(error)) => return Err(LayoutRunStartError::Graph(error)),
            Err(LayoutRunResumeError::Config(error)) => return Err(LayoutRunStartError::Config(error)),
            Err(LayoutRunResumeError::Malformed | LayoutRunResumeError::Foreign | LayoutRunResumeError::Positions) => {}
        }
    }
    LayoutRunJob::new(identity, graph, config, encoder())
}

/// 🪪️ A consumer node id's entity: the first eight little-endian digest bytes of the id, the same key the ToolRun
/// provisional entity set and the `provisional` instance stamp use.
pub fn layout_run_entity(id: &str) -> u64 {
    u64::from_le_bytes(semio_framework_hash::hash(id.as_bytes()).as_bytes()[..8].try_into().expect("eight digest bytes"))
}

/// 📍️ A resume's overlay positions: every node's origin (unplaced = NaN, which refuses the resume) with the
/// provisional `moves` (node index, position) folded in order.
pub fn layout_run_overlay_positions(graph: &LayoutRunGraph, moves: impl IntoIterator<Item = (u32, LayoutRunPoint)>) -> Vec<LayoutRunPoint> {
    let mut positions: Vec<LayoutRunPoint> = graph.nodes.iter().map(|node| node.origin.unwrap_or(LayoutRunPoint::new(f64::NAN, f64::NAN))).collect();
    for (node, position) in moves {
        if let Some(slot) = positions.get_mut(node as usize) {
            *slot = position;
        }
    }
    positions
}
//#endregion 🔖️Job
