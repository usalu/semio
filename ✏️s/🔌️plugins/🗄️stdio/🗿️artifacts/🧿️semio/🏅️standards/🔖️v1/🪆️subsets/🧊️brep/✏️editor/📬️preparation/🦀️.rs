//! 📬️ Paged structural preparation for one boundary-representation vertex move.

use super::*;
use crate::{
    retained_native_preparation::{StructuralCopyStep, StructuralMutationCopy, StructuralPreparationFactory},
    standards::v1::subsets::{
        base::schema::geometry::{SemioPoint2, SemioPoint3},
        brep::schema::snapshot::{BrepCoedge, BrepCurve, BrepCurve2, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex},
    },
    SemioMutationRetirementFactory, SemioSnapshotRetirementFactory,
};
use semio_framework_plugin::plugin_app_close_prelude::store as app_store;
use semio_s_artifact_stdio_contract::editing::{NativeEditPreparationRoute, RetainedTextCopy};
use std::{marker::PhantomData, mem::size_of, sync::Arc};

const PREFIX: &str = "stdio-semio-brep-set-vertex";

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<SemioBrepSnapshot, SemioBrepMutation>> {
    Some(NativeEditPreparationRoute::new(
        recognizes,
        Arc::new(StructuralPreparationFactory::new(
            PREFIX,
            recognizes,
            preflight,
            || Box::<BrepStructuralCopy>::default(),
            Arc::new(SemioMutationRetirementFactory::<SemioBrepMutation>(PhantomData)),
            Arc::new(SemioSnapshotRetirementFactory::<SemioBrepSnapshot>(PhantomData)),
        )),
    ))
}

fn recognizes(mutation: &SemioBrepMutation) -> bool {
    matches!(mutation, SemioBrepMutation::MoveVertex(_))
}

fn preflight(mutation: &SemioBrepMutation) -> Result<usize, String> {
    let SemioBrepMutation::MoveVertex(value) = mutation else { return Err(format!("{PREFIX}-mutation")) };
    value.vertex_id.len().checked_add(size_of::<SemioPoint3>()).ok_or_else(|| format!("{PREFIX}-payload-overflow"))
}

#[derive(Default)]
struct PodCopy<T> {
    values: Vec<T>,
    reserved: bool,
    complete: bool,
}

impl<T: Copy> PodCopy<T> {
    fn advance(&mut self, source: &[T], maximum_items: usize, maximum_bytes: usize) -> Result<(usize, usize), String> {
        if !self.reserved {
            self.values.try_reserve_exact(source.len()).map_err(|_| format!("{PREFIX}-vector-allocation"))?;
            self.reserved = true;
            self.complete = source.is_empty();
            return Ok((1, 0));
        }
        if self.complete {
            return Ok((0, 0));
        }
        let element_bytes = size_of::<T>().max(1);
        let count = maximum_items.min(maximum_bytes / element_bytes).min(source.len().saturating_sub(self.values.len()));
        if count == 0 {
            return Ok((0, 0));
        }
        let start = self.values.len();
        self.values.extend_from_slice(&source[start..start + count]);
        self.complete = self.values.len() == source.len();
        Ok((count, count.saturating_mul(element_bytes)))
    }

    fn take(&mut self) -> Option<Vec<T>> {
        self.complete.then(|| {
            self.complete = false;
            self.reserved = false;
            std::mem::take(&mut self.values)
        })
    }

    fn take_partial(&mut self) -> Vec<T> {
        self.complete = false;
        self.reserved = false;
        std::mem::take(&mut self.values)
    }

    fn terminal_is_empty(&self) -> bool {
        self.values.is_empty() && self.values.capacity() == 0 && !self.reserved && !self.complete
    }
}

fn text_close(copy: &mut RetainedTextCopy, grant: app_store::ArtifactStoreOneItemGrant) -> app_store::SnapshotRetirementStep {
    match copy.close_step(grant.maximum_items, grant.maximum_bytes) {
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => app_store::SnapshotRetirementStep::Pending { released_items, released_bytes },
        semio_framework_job::InteractiveJobCloseStep::Blocked => app_store::SnapshotRetirementStep::Blocked,
        semio_framework_job::InteractiveJobCloseStep::Complete => app_store::SnapshotRetirementStep::Complete,
    }
}

struct TextFields<const N: usize> {
    index: usize,
    copies: [RetainedTextCopy; N],
    values: [Option<String>; N],
}

impl<const N: usize> Default for TextFields<N> {
    fn default() -> Self {
        Self { index: 0, copies: std::array::from_fn(|_| RetainedTextCopy::default()), values: std::array::from_fn(|_| None) }
    }
}

impl<const N: usize> TextFields<N> {
    fn advance(&mut self, sources: [&str; N], maximum_bytes: usize) -> Result<(usize, bool), String> {
        if self.index == N {
            return Ok((0, true));
        }
        let copy = &mut self.copies[self.index];
        let bytes = copy.advance(sources[self.index], maximum_bytes)?.unwrap_or(0);
        if copy.is_complete() {
            self.values[self.index] = copy.take();
            self.index += 1;
        }
        Ok((bytes, self.index == N))
    }

    fn take(&mut self) -> Option<[String; N]> {
        (self.index == N).then(|| std::array::from_fn(|index| self.values[index].take().expect("completed text field")))
    }

    fn take_partial(&mut self) -> [String; N] {
        self.index = N;
        std::array::from_fn(|index| self.values[index].take().unwrap_or_default())
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> app_store::SnapshotRetirementStep {
        for copy in &mut self.copies {
            let step = text_close(copy, grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return step;
            }
        }
        app_store::SnapshotRetirementStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.copies.iter().all(RetainedTextCopy::terminal_is_empty) && self.values.iter().all(Option::is_none)
    }
}

#[derive(Default)]
struct StringListCopy {
    values: Vec<String>,
    index: usize,
    current: RetainedTextCopy,
    complete: bool,
}

impl StringListCopy {
    fn advance(&mut self, source: &[String], maximum_bytes: usize) -> Result<(usize, bool), String> {
        let Some(value) = source.get(self.index) else {
            self.complete = true;
            return Ok((0, true));
        };
        let bytes = self.current.advance(value, maximum_bytes)?.unwrap_or(0);
        if self.current.is_complete() {
            self.values.push(self.current.take().ok_or_else(|| format!("{PREFIX}-string-list"))?);
            self.index += 1;
        }
        Ok((bytes, false))
    }

    fn take(&mut self) -> Option<Vec<String>> {
        self.complete.then(|| {
            self.complete = false;
            std::mem::take(&mut self.values)
        })
    }

    fn take_partial(&mut self) -> Vec<String> {
        self.complete = false;
        std::mem::take(&mut self.values)
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> app_store::SnapshotRetirementStep {
        text_close(&mut self.current, grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.current.terminal_is_empty() && self.values.is_empty() && self.values.capacity() == 0 && !self.complete
    }
}

#[derive(Default)]
struct TextBoolListCopy<T> {
    values: Vec<T>,
    index: usize,
    current: RetainedTextCopy,
    complete: bool,
}

impl<T> TextBoolListCopy<T> {
    fn advance(&mut self, text: &str, flag: bool, source_len: usize, build: fn(String, bool) -> T, maximum_bytes: usize) -> Result<(usize, bool), String> {
        if self.index == source_len {
            self.complete = true;
            return Ok((0, true));
        }
        let bytes = self.current.advance(text, maximum_bytes)?.unwrap_or(0);
        if self.current.is_complete() {
            self.values.push(build(self.current.take().ok_or_else(|| format!("{PREFIX}-text-bool-list"))?, flag));
            self.index += 1;
        }
        Ok((bytes, false))
    }

    fn take(&mut self) -> Option<Vec<T>> {
        self.complete.then(|| {
            self.complete = false;
            std::mem::take(&mut self.values)
        })
    }

    fn take_partial(&mut self) -> Vec<T> {
        self.complete = false;
        std::mem::take(&mut self.values)
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> app_store::SnapshotRetirementStep {
        text_close(&mut self.current, grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.current.terminal_is_empty() && self.values.is_empty() && self.values.capacity() == 0 && !self.complete
    }
}

#[derive(Default)]
struct Curve3Copy {
    points: PodCopy<SemioPoint3>,
    weights: PodCopy<f64>,
    knots: PodCopy<f64>,
    phase: u8,
    output: Option<BrepCurve>,
}

impl Curve3Copy {
    fn advance(&mut self, source: &BrepCurve, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        let BrepCurve::Nurbs { control_points, weights, degree, knots } = source else {
            self.output = Some(source.clone());
            return Ok(StructuralCopyStep::Complete);
        };
        match self.phase {
            0 => {
                let (items, bytes) = self.points.advance(control_points, maximum_items, maximum_bytes)?;
                if self.points.complete {
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            1 => {
                let (items, bytes) = self.weights.advance(weights, maximum_items, maximum_bytes)?;
                if self.weights.complete {
                    self.phase = 2;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            2 => {
                let (items, bytes) = self.knots.advance(knots, maximum_items, maximum_bytes)?;
                if self.knots.complete {
                    self.phase = 3;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            3 => {
                self.output = Some(BrepCurve::Nurbs {
                    control_points: self.points.take().ok_or_else(|| format!("{PREFIX}-curve-points"))?,
                    weights: self.weights.take().ok_or_else(|| format!("{PREFIX}-curve-weights"))?,
                    degree: *degree,
                    knots: self.knots.take().ok_or_else(|| format!("{PREFIX}-curve-knots"))?,
                });
                self.phase = 4;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take_partial(&mut self) -> BrepCurve {
        self.output.take().unwrap_or_else(|| BrepCurve::Nurbs { control_points: self.points.take_partial(), weights: self.weights.take_partial(), degree: 0, knots: self.knots.take_partial() })
    }

    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.points.terminal_is_empty() && self.weights.terminal_is_empty() && self.knots.terminal_is_empty()
    }
}

#[derive(Default)]
struct Curve2Copy {
    points: PodCopy<SemioPoint2>,
    weights: PodCopy<f64>,
    knots: PodCopy<f64>,
    phase: u8,
    output: Option<BrepCurve2>,
}

impl Curve2Copy {
    fn advance(&mut self, source: &BrepCurve2, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        let BrepCurve2::Nurbs { control_points, weights, degree, knots } = source else {
            self.output = Some(source.clone());
            return Ok(StructuralCopyStep::Complete);
        };
        match self.phase {
            0 => {
                let (items, bytes) = self.points.advance(control_points, maximum_items, maximum_bytes)?;
                if self.points.complete {
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            1 => {
                let (items, bytes) = self.weights.advance(weights, maximum_items, maximum_bytes)?;
                if self.weights.complete {
                    self.phase = 2;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            2 => {
                let (items, bytes) = self.knots.advance(knots, maximum_items, maximum_bytes)?;
                if self.knots.complete {
                    self.phase = 3;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            3 => {
                self.output = Some(BrepCurve2::Nurbs {
                    control_points: self.points.take().ok_or_else(|| format!("{PREFIX}-pcurve-points"))?,
                    weights: self.weights.take().ok_or_else(|| format!("{PREFIX}-pcurve-weights"))?,
                    degree: *degree,
                    knots: self.knots.take().ok_or_else(|| format!("{PREFIX}-pcurve-knots"))?,
                });
                self.phase = 4;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take_partial(&mut self) -> BrepCurve2 {
        self.output.take().unwrap_or_else(|| BrepCurve2::Nurbs { control_points: self.points.take_partial(), weights: self.weights.take_partial(), degree: 0, knots: self.knots.take_partial() })
    }

    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.points.terminal_is_empty() && self.weights.terminal_is_empty() && self.knots.terminal_is_empty()
    }
}

#[derive(Default)]
struct SurfaceCopy {
    points: PodCopy<SemioPoint3>,
    weights: PodCopy<f64>,
    knots_u: PodCopy<f64>,
    knots_v: PodCopy<f64>,
    phase: u8,
    output: Option<BrepSurface>,
}

impl SurfaceCopy {
    fn advance(&mut self, source: &BrepSurface, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        let BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } = source else {
            self.output = Some(source.clone());
            return Ok(StructuralCopyStep::Complete);
        };
        let (items, bytes) = match self.phase {
            0 => self.points.advance(control_points, maximum_items, maximum_bytes)?,
            1 => self.weights.advance(weights, maximum_items, maximum_bytes)?,
            2 => self.knots_u.advance(knots_u, maximum_items, maximum_bytes)?,
            3 => self.knots_v.advance(knots_v, maximum_items, maximum_bytes)?,
            4 => {
                self.output = Some(BrepSurface::Nurbs {
                    control_points: self.points.take().ok_or_else(|| format!("{PREFIX}-surface-points"))?,
                    weights: self.weights.take().ok_or_else(|| format!("{PREFIX}-surface-weights"))?,
                    u_count: *u_count,
                    v_count: *v_count,
                    degree_u: *degree_u,
                    degree_v: *degree_v,
                    knots_u: self.knots_u.take().ok_or_else(|| format!("{PREFIX}-surface-knots-u"))?,
                    knots_v: self.knots_v.take().ok_or_else(|| format!("{PREFIX}-surface-knots-v"))?,
                });
                self.phase = 5;
                return Ok(StructuralCopyStep::Complete);
            }
            _ => return Ok(StructuralCopyStep::Complete),
        };
        let complete = match self.phase {
            0 => self.points.complete,
            1 => self.weights.complete,
            2 => self.knots_u.complete,
            3 => self.knots_v.complete,
            _ => false,
        };
        if complete {
            self.phase += 1;
        }
        Ok(StructuralCopyStep::Progress { items, bytes })
    }

    fn take_partial(&mut self) -> BrepSurface {
        self.output.take().unwrap_or_else(|| BrepSurface::Nurbs {
            control_points: self.points.take_partial(),
            weights: self.weights.take_partial(),
            u_count: 0,
            v_count: 0,
            degree_u: 0,
            degree_v: 0,
            knots_u: self.knots_u.take_partial(),
            knots_v: self.knots_v.take_partial(),
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.output.is_none() && self.points.terminal_is_empty() && self.weights.terminal_is_empty() && self.knots_u.terminal_is_empty() && self.knots_v.terminal_is_empty()
    }
}

#[derive(Default)]
struct VertexCopy {
    fields: TextFields<1>,
    output: Option<BrepVertex>,
}

impl VertexCopy {
    fn advance(&mut self, source: &BrepVertex, replacement: Option<SemioPoint3>, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        let (bytes, complete) = self.fields.advance([&source.id], maximum_bytes)?;
        if complete {
            let [id] = self.fields.take().ok_or_else(|| format!("{PREFIX}-vertex-fields"))?;
            self.output = Some(BrepVertex { id, point: replacement.unwrap_or(source.point), tol: source.tol });
            return Ok(StructuralCopyStep::Complete);
        }
        Ok(StructuralCopyStep::Progress { items: 1, bytes })
    }

    fn take_partial(&mut self) -> BrepVertex {
        self.output.take().unwrap_or_else(|| {
            let [id] = self.fields.take_partial();
            BrepVertex { id, ..Default::default() }
        })
    }
}

#[derive(Default)]
struct EdgeCopy {
    phase: u8,
    fields: TextFields<3>,
    curve: Curve3Copy,
    output: Option<BrepEdge>,
}

impl EdgeCopy {
    fn advance(&mut self, source: &BrepEdge, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        match self.phase {
            0 => {
                let (bytes, complete) = self.fields.advance([&source.id, &source.start_vertex, &source.end_vertex], maximum_bytes)?;
                if complete {
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            1 => match self.curve.advance(&source.curve, maximum_items, maximum_bytes)? {
                StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                StructuralCopyStep::Complete => {
                    self.phase = 2;
                    Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                }
            },
            2 => {
                let [id, start_vertex, end_vertex] = self.fields.take().ok_or_else(|| format!("{PREFIX}-edge-fields"))?;
                self.output = Some(BrepEdge { id, start_vertex, end_vertex, curve: self.curve.output.take().ok_or_else(|| format!("{PREFIX}-edge-curve"))?, tol: source.tol });
                self.phase = 3;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take_partial(&mut self) -> BrepEdge {
        self.output.take().unwrap_or_else(|| {
            let [id, start_vertex, end_vertex] = self.fields.take_partial();
            BrepEdge { id, start_vertex, end_vertex, curve: self.curve.take_partial(), tol: 0.0 }
        })
    }
}

#[derive(Default)]
struct LoopCopy {
    phase: u8,
    fields: TextFields<1>,
    edges: TextBoolListCopy<BrepLoopEdge>,
    output: Option<BrepLoop>,
}

impl LoopCopy {
    fn advance(&mut self, source: &BrepLoop, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        if self.phase == 0 {
            let (bytes, complete) = self.fields.advance([&source.id], maximum_bytes)?;
            if complete {
                self.phase = 1;
            }
            return Ok(StructuralCopyStep::Progress { items: 1, bytes });
        }
        if self.phase == 1 {
            if let Some(edge) = source.edges.get(self.edges.index) {
                let (bytes, _) = self.edges.advance(&edge.edge, edge.orientation, source.edges.len(), |edge, orientation| BrepLoopEdge { edge, orientation }, maximum_bytes)?;
                return Ok(StructuralCopyStep::Progress { items: 1, bytes });
            }
            self.edges.complete = true;
            self.phase = 2;
            return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
        }
        if self.phase == 2 {
            let [id] = self.fields.take().ok_or_else(|| format!("{PREFIX}-loop-fields"))?;
            self.output = Some(BrepLoop { id, edges: self.edges.take().ok_or_else(|| format!("{PREFIX}-loop-edges"))? });
            self.phase = 3;
        }
        Ok(StructuralCopyStep::Complete)
    }

    fn take_partial(&mut self) -> BrepLoop {
        self.output.take().unwrap_or_else(|| {
            let [id] = self.fields.take_partial();
            BrepLoop { id, edges: self.edges.take_partial() }
        })
    }
}

#[derive(Default)]
struct FaceCopy {
    phase: u8,
    fields: TextFields<2>,
    inner_loops: StringListCopy,
    surface: SurfaceCopy,
    output: Option<BrepFace>,
}

impl FaceCopy {
    fn advance(&mut self, source: &BrepFace, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        match self.phase {
            0 => {
                let (bytes, complete) = self.fields.advance([&source.id, &source.outer_loop], maximum_bytes)?;
                if complete {
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            1 => {
                let (bytes, complete) = self.inner_loops.advance(&source.inner_loops, maximum_bytes)?;
                if complete {
                    self.phase = 2;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            2 => match self.surface.advance(&source.surface, maximum_items, maximum_bytes)? {
                StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                StructuralCopyStep::Complete => {
                    self.phase = 3;
                    Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                }
            },
            3 => {
                let [id, outer_loop] = self.fields.take().ok_or_else(|| format!("{PREFIX}-face-fields"))?;
                self.output = Some(BrepFace {
                    id,
                    outer_loop,
                    inner_loops: self.inner_loops.take().ok_or_else(|| format!("{PREFIX}-face-inner-loops"))?,
                    surface: self.surface.output.take().ok_or_else(|| format!("{PREFIX}-face-surface"))?,
                    orientation: source.orientation,
                    tol: source.tol,
                });
                self.phase = 4;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take_partial(&mut self) -> BrepFace {
        self.output.take().unwrap_or_else(|| {
            let [id, outer_loop] = self.fields.take_partial();
            BrepFace { id, outer_loop, inner_loops: self.inner_loops.take_partial(), surface: self.surface.take_partial(), orientation: false, tol: 0.0 }
        })
    }
}

#[derive(Default)]
struct ShellCopy {
    phase: u8,
    fields: TextFields<1>,
    faces: TextBoolListCopy<BrepShellFace>,
    output: Option<BrepShell>,
}

impl ShellCopy {
    fn advance(&mut self, source: &BrepShell, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        if self.phase == 0 {
            let (bytes, complete) = self.fields.advance([&source.id], maximum_bytes)?;
            if complete {
                self.phase = 1;
            }
            return Ok(StructuralCopyStep::Progress { items: 1, bytes });
        }
        if self.phase == 1 {
            if let Some(face) = source.faces.get(self.faces.index) {
                let (bytes, _) = self.faces.advance(&face.face, face.orientation, source.faces.len(), |face, orientation| BrepShellFace { face, orientation }, maximum_bytes)?;
                return Ok(StructuralCopyStep::Progress { items: 1, bytes });
            }
            self.faces.complete = true;
            self.phase = 2;
            return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
        }
        if self.phase == 2 {
            let [id] = self.fields.take().ok_or_else(|| format!("{PREFIX}-shell-fields"))?;
            self.output = Some(BrepShell { id, faces: self.faces.take().ok_or_else(|| format!("{PREFIX}-shell-faces"))? });
            self.phase = 3;
        }
        Ok(StructuralCopyStep::Complete)
    }

    fn take_partial(&mut self) -> BrepShell {
        self.output.take().unwrap_or_else(|| {
            let [id] = self.fields.take_partial();
            BrepShell { id, faces: self.faces.take_partial() }
        })
    }
}

#[derive(Default)]
struct SolidCopy {
    phase: u8,
    fields: TextFields<1>,
    shells: TextBoolListCopy<BrepSolidShell>,
    output: Option<BrepSolid>,
}

impl SolidCopy {
    fn advance(&mut self, source: &BrepSolid, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        if self.phase == 0 {
            let (bytes, complete) = self.fields.advance([&source.id], maximum_bytes)?;
            if complete {
                self.phase = 1;
            }
            return Ok(StructuralCopyStep::Progress { items: 1, bytes });
        }
        if self.phase == 1 {
            if let Some(shell) = source.shells.get(self.shells.index) {
                let (bytes, _) = self.shells.advance(&shell.shell, shell.is_void, source.shells.len(), |shell, is_void| BrepSolidShell { shell, is_void }, maximum_bytes)?;
                return Ok(StructuralCopyStep::Progress { items: 1, bytes });
            }
            self.shells.complete = true;
            self.phase = 2;
            return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
        }
        if self.phase == 2 {
            let [id] = self.fields.take().ok_or_else(|| format!("{PREFIX}-solid-fields"))?;
            self.output = Some(BrepSolid { id, shells: self.shells.take().ok_or_else(|| format!("{PREFIX}-solid-shells"))? });
            self.phase = 3;
        }
        Ok(StructuralCopyStep::Complete)
    }

    fn take_partial(&mut self) -> BrepSolid {
        self.output.take().unwrap_or_else(|| {
            let [id] = self.fields.take_partial();
            BrepSolid { id, shells: self.shells.take_partial() }
        })
    }
}

#[derive(Default)]
struct CoedgeCopy {
    phase: u8,
    fields: TextFields<5>,
    pcurve: Curve2Copy,
    pcurve_present: bool,
    output: Option<BrepCoedge>,
}

impl CoedgeCopy {
    fn advance(&mut self, source: &BrepCoedge, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        match self.phase {
            0 => {
                let (bytes, complete) = self.fields.advance([&source.id, &source.edge, &source.loop_id, &source.next, &source.prev], maximum_bytes)?;
                if complete {
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            1 => match source.pcurve.as_ref() {
                Some(pcurve) => {
                    self.pcurve_present = true;
                    match self.pcurve.advance(pcurve, maximum_items, maximum_bytes)? {
                        StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                        StructuralCopyStep::Complete => {
                            self.phase = 2;
                            Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                        }
                    }
                }
                None => {
                    self.phase = 2;
                    Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                }
            },
            2 => {
                let [id, edge, loop_id, next, prev] = self.fields.take().ok_or_else(|| format!("{PREFIX}-coedge-fields"))?;
                self.output = Some(BrepCoedge { id, edge, forward: source.forward, pcurve: self.pcurve_present.then(|| self.pcurve.output.take().expect("completed pcurve")), prange: source.prange, loop_id, next, prev });
                self.phase = 3;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take_partial(&mut self) -> BrepCoedge {
        self.output.take().unwrap_or_else(|| {
            let [id, edge, loop_id, next, prev] = self.fields.take_partial();
            BrepCoedge { id, edge, forward: false, pcurve: self.pcurve_present.then(|| self.pcurve.take_partial()), prange: (0.0, 0.0), loop_id, next, prev }
        })
    }
}

#[derive(Default)]
struct BrepStructuralCopy {
    phase: u8,
    schema_copy: RetainedTextCopy,
    schema: Option<String>,
    vertices: Vec<BrepVertex>,
    vertex_index: usize,
    vertex: Option<VertexCopy>,
    edges: Vec<BrepEdge>,
    edge_index: usize,
    edge: Option<EdgeCopy>,
    loops: Vec<BrepLoop>,
    loop_index: usize,
    loop_copy: Option<LoopCopy>,
    faces: Vec<BrepFace>,
    face_index: usize,
    face: Option<FaceCopy>,
    shells: Vec<BrepShell>,
    shell_index: usize,
    shell: Option<ShellCopy>,
    solids: Vec<BrepSolid>,
    solid_index: usize,
    solid: Option<SolidCopy>,
    coedges: Vec<BrepCoedge>,
    coedge_index: usize,
    coedge: Option<CoedgeCopy>,
    matches: usize,
    old_point: Option<SemioPoint3>,
    result: Option<(SemioBrepSnapshot, SemioBrepMutation)>,
    orphan_inverse: Option<SemioBrepMutation>,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    closing: bool,
}

impl BrepStructuralCopy {
    fn target(mutation: &SemioBrepMutation) -> Result<&MoveVertex, String> {
        let SemioBrepMutation::MoveVertex(value) = mutation else { return Err(format!("{PREFIX}-mutation")) };
        Ok(value)
    }

    fn finish_active_close(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<Option<app_store::SnapshotRetirementStep>, String> {
        if let Some(value) = self.vertex.as_mut() {
            let step = value.fields.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            self.vertices.push(value.take_partial());
            self.vertex = None;
            return Ok(Some(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if let Some(value) = self.edge.as_mut() {
            let step = value.fields.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            self.edges.push(value.take_partial());
            self.edge = None;
            return Ok(Some(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if let Some(value) = self.loop_copy.as_mut() {
            let step = value.fields.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            let step = value.edges.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            self.loops.push(value.take_partial());
            self.loop_copy = None;
            return Ok(Some(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if let Some(value) = self.face.as_mut() {
            let step = value.fields.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            let step = value.inner_loops.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            self.faces.push(value.take_partial());
            self.face = None;
            return Ok(Some(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if let Some(value) = self.shell.as_mut() {
            let step = value.fields.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            let step = value.faces.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            self.shells.push(value.take_partial());
            self.shell = None;
            return Ok(Some(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if let Some(value) = self.solid.as_mut() {
            let step = value.fields.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            let step = value.shells.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            self.solids.push(value.take_partial());
            self.solid = None;
            return Ok(Some(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if let Some(value) = self.coedge.as_mut() {
            let step = value.fields.close_step(grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(Some(step));
            }
            self.coedges.push(value.take_partial());
            self.coedge = None;
            return Ok(Some(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        Ok(None)
    }
}

impl StructuralMutationCopy<SemioBrepSnapshot, SemioBrepMutation> for BrepStructuralCopy {
    fn advance(&mut self, base: &SemioBrepSnapshot, mutation: &SemioBrepMutation, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        let target = Self::target(mutation)?;
        match self.phase {
            0 => {
                let bytes = self.schema_copy.advance(&base.schema, maximum_bytes)?.unwrap_or(0);
                if self.schema_copy.is_complete() {
                    self.schema = self.schema_copy.take();
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            1 => {
                let Some(source) = base.vertices.get(self.vertex_index) else {
                    self.phase = 2;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let replacement = if self.vertex.is_none() && source.id == target.vertex_id {
                    self.matches += 1;
                    self.old_point = Some(source.point);
                    Some(target.new_point)
                } else if source.id == target.vertex_id {
                    Some(target.new_point)
                } else {
                    None
                };
                let cursor = self.vertex.get_or_insert_with(VertexCopy::default);
                match cursor.advance(source, replacement, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.vertices.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-vertex-result"))?);
                        self.vertex = None;
                        self.vertex_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            2 => {
                let Some(source) = base.edges.get(self.edge_index) else {
                    self.phase = 3;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.edge.get_or_insert_with(EdgeCopy::default);
                match cursor.advance(source, maximum_items, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.edges.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-edge-result"))?);
                        self.edge = None;
                        self.edge_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            3 => {
                let Some(source) = base.loops.get(self.loop_index) else {
                    self.phase = 4;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.loop_copy.get_or_insert_with(LoopCopy::default);
                match cursor.advance(source, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.loops.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-loop-result"))?);
                        self.loop_copy = None;
                        self.loop_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            4 => {
                let Some(source) = base.faces.get(self.face_index) else {
                    self.phase = 5;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.face.get_or_insert_with(FaceCopy::default);
                match cursor.advance(source, maximum_items, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.faces.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-face-result"))?);
                        self.face = None;
                        self.face_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            5 => {
                let Some(source) = base.shells.get(self.shell_index) else {
                    self.phase = 6;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.shell.get_or_insert_with(ShellCopy::default);
                match cursor.advance(source, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.shells.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-shell-result"))?);
                        self.shell = None;
                        self.shell_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            6 => {
                let Some(source) = base.solids.get(self.solid_index) else {
                    self.phase = 7;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.solid.get_or_insert_with(SolidCopy::default);
                match cursor.advance(source, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.solids.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-solid-result"))?);
                        self.solid = None;
                        self.solid_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            7 => {
                let Some(source) = base.coedges.get(self.coedge_index) else {
                    self.phase = 8;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.coedge.get_or_insert_with(CoedgeCopy::default);
                match cursor.advance(source, maximum_items, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.coedges.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-coedge-result"))?);
                        self.coedge = None;
                        self.coedge_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            8 => {
                if self.matches != 1 {
                    return Err(format!("{PREFIX}-target-count-{}", self.matches));
                }
                let inverse = SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: target.vertex_id.clone(), new_point: self.old_point.ok_or_else(|| format!("{PREFIX}-inverse"))? });
                self.result = Some((
                    SemioBrepSnapshot {
                        schema: self.schema.take().ok_or_else(|| format!("{PREFIX}-schema"))?,
                        vertices: std::mem::take(&mut self.vertices),
                        edges: std::mem::take(&mut self.edges),
                        loops: std::mem::take(&mut self.loops),
                        faces: std::mem::take(&mut self.faces),
                        shells: std::mem::take(&mut self.shells),
                        solids: std::mem::take(&mut self.solids),
                        coedges: std::mem::take(&mut self.coedges),
                        next_label: base.next_label,
                    },
                    inverse,
                ));
                self.phase = 9;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take_result(&mut self) -> Option<(SemioBrepSnapshot, SemioBrepMutation)> {
        self.result.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, String> {
        if !self.closing {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(step) = self.finish_active_close(grant)? {
            return Ok(step);
        }
        let schema = text_close(&mut self.schema_copy, grant);
        if schema != app_store::SnapshotRetirementStep::Complete {
            return Ok(schema);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(format!("{PREFIX}-retirement-witness"));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(inverse) = self.orphan_inverse.take() {
            self.retirement = Some(app_store::retirement::owned_retirement(inverse));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let has_snapshot =
            self.result.is_some() || self.schema.is_some() || !self.vertices.is_empty() || !self.edges.is_empty() || !self.loops.is_empty() || !self.faces.is_empty() || !self.shells.is_empty() || !self.solids.is_empty() || !self.coedges.is_empty();
        if has_snapshot {
            let snapshot = if let Some((snapshot, inverse)) = self.result.take() {
                self.orphan_inverse = Some(inverse);
                snapshot
            } else {
                SemioBrepSnapshot {
                    schema: self.schema.take().unwrap_or_default(),
                    vertices: std::mem::take(&mut self.vertices),
                    edges: std::mem::take(&mut self.edges),
                    loops: std::mem::take(&mut self.loops),
                    faces: std::mem::take(&mut self.faces),
                    shells: std::mem::take(&mut self.shells),
                    solids: std::mem::take(&mut self.solids),
                    coedges: std::mem::take(&mut self.coedges),
                    next_label: 0,
                }
            };
            self.retirement = Some(app_store::retirement::owned_retirement(snapshot));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.schema_copy.terminal_is_empty()
            && self.schema.is_none()
            && self.vertices.is_empty()
            && self.vertex.is_none()
            && self.edges.is_empty()
            && self.edge.is_none()
            && self.loops.is_empty()
            && self.loop_copy.is_none()
            && self.faces.is_empty()
            && self.face.is_none()
            && self.shells.is_empty()
            && self.shell.is_none()
            && self.solids.is_empty()
            && self.solid.is_none()
            && self.coedges.is_empty()
            && self.coedge.is_none()
            && self.result.is_none()
            && self.orphan_inverse.is_none()
            && self.retirement.is_none()
    }
}

fn canonical_scalar(value: app_store::ArtifactCanonicalJsonNode<'_>) -> app_store::ArtifactCanonicalJsonValue<'_> {
    app_store::ArtifactCanonicalJsonValue::Scalar(value)
}

fn canonical_object<'a, const N: usize>(mut fields: [(&'a str, app_store::ArtifactCanonicalJsonValue<'a>); N]) -> app_store::ArtifactCanonicalJsonValue<'a> {
    fields.sort_unstable_by(|left, right| left.0.cmp(right.0));
    app_store::ArtifactCanonicalJsonValue::Object(app_store::ArtifactCanonicalJsonObject::new(fields.into_iter()))
}

impl app_store::ArtifactCanonicalJson for SemioBrepMutation {
    fn canonical_json_borrowed_root(&self) -> Result<Option<app_store::ArtifactCanonicalJsonValue<'_>>, String> {
        let SemioBrepMutation::MoveVertex(value) = self else { return Err(format!("{PREFIX}-canonical-mutation")) };
        let point = canonical_object([
            ("x", canonical_scalar(app_store::ArtifactCanonicalJsonNode::F64(value.new_point.x))),
            ("y", canonical_scalar(app_store::ArtifactCanonicalJsonNode::F64(value.new_point.y))),
            ("z", canonical_scalar(app_store::ArtifactCanonicalJsonNode::F64(value.new_point.z))),
        ]);
        Ok(Some(canonical_object([("kind", canonical_scalar(app_store::ArtifactCanonicalJsonNode::String("moveVertex"))), ("newPoint", point), ("vertexId", canonical_scalar(app_store::ArtifactCanonicalJsonNode::String(&value.vertex_id)))])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🧫️fixtures/🧵️retained-native/🔣️.json"))).expect("retained native fixture")
    }

    fn large_snapshot() -> SemioBrepSnapshot {
        let values = fixture();
        let values = &values["structuralCopy"];
        let vertex_count = values["brepVertexCount"].as_u64().expect("brep vertex count") as usize;
        let nurbs_count = values["nurbsPointCount"].as_u64().expect("nurbs point count") as usize;
        SemioBrepSnapshot {
            schema: SEMIO_BREP_DOCUMENT_SCHEMA.into(),
            vertices: (0..vertex_count).map(|index| BrepVertex { id: format!("v-{index}"), point: SemioPoint3 { x: index as f64, y: 0.0, z: 0.0 }, tol: 0.000_001 }).collect(),
            edges: vec![BrepEdge {
                id: "edge".into(),
                start_vertex: "v-0".into(),
                end_vertex: format!("v-{}", vertex_count - 1),
                curve: BrepCurve::Nurbs {
                    control_points: (0..nurbs_count).map(|index| SemioPoint3 { x: index as f64, y: 1.0, z: 2.0 }).collect(),
                    weights: vec![1.0; nurbs_count],
                    degree: 3,
                    knots: (0..nurbs_count).map(|index| index as f64).collect(),
                },
                tol: 0.000_001,
            }],
            loops: Vec::new(),
            faces: Vec::new(),
            shells: Vec::new(),
            solids: Vec::new(),
            coedges: Vec::new(),
            next_label: 91,
        }
    }

    fn mutation(snapshot: &SemioBrepSnapshot) -> SemioBrepMutation {
        SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: snapshot.vertices.last().expect("target vertex").id.clone(), new_point: SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 } })
    }

    #[test]
    fn structural_copy_pages_vertices_and_nurbs_without_cloning_aggregate_turns() {
        let source = large_snapshot();
        let mutation = mutation(&source);
        let mut copy = BrepStructuralCopy::default();
        let mut turns = 0;
        loop {
            match copy.advance(&source, &mutation, 32, 4_096).expect("brep structural copy") {
                StructuralCopyStep::Progress { items, bytes } => {
                    assert!(items <= 32);
                    assert!(bytes <= 4_096);
                }
                StructuralCopyStep::Complete => break,
            }
            turns += 1;
            assert!(turns < 50_000);
        }
        let (post, inverse) = copy.take_result().expect("completed brep result");
        assert!(turns > 8_192);
        assert_eq!(post.edges, source.edges);
        assert_eq!(post.next_label, source.next_label);
        assert_eq!(post.vertices.last().expect("post target").point, SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 });
        let SemioBrepMutation::MoveVertex(inverse) = inverse else { panic!("brep inverse") };
        assert_eq!(inverse.new_point, source.vertices.last().expect("source target").point);
        copy.begin_close();
        assert_eq!(copy.close_step(app_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }).expect("empty close"), app_store::SnapshotRetirementStep::Complete);
        assert!(copy.terminal_is_empty());
    }

    #[test]
    fn structural_copy_cancellation_retires_partial_vertex_owners() {
        let source = large_snapshot();
        let mutation = mutation(&source);
        let mut copy = BrepStructuralCopy::default();
        for _ in 0..4_096 {
            if matches!(copy.advance(&source, &mutation, 1, 4_096).expect("brep structural copy"), StructuralCopyStep::Complete) {
                break;
            }
        }
        copy.begin_close();
        let mut close_turns = 0;
        while !copy.terminal_is_empty() {
            copy.close_step(app_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }).expect("bounded brep close");
            close_turns += 1;
            assert!(close_turns < 20_000);
        }
        assert!(close_turns > 1);
    }
}
