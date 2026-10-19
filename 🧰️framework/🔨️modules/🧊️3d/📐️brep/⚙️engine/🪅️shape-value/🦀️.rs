//! 🪅️ Owned standalone B-Rep geometry value: a minimal [`Body`] holding exactly the entity closure of one shape's roots, plus the root references and every [`PersistentLabel`], so a pure inference can pass geometry between widgets without any session handle.
use semio_framework_3d_brep_kernel_contract::{map_err, BrepError, GeometryHandle, GeometryKind};
use semio_framework_3d_brep_representation::contract::MeshTransfer;
extern crate semio_framework_value_derive as value_derive;
use semio_framework_3d_brep_representation::topology::Wire;
use semio_framework_3d_brep_tessellation::{TessellationJob, TessellationProgress, TessellationReport, TessellationStep};
use semio_framework_3d_brep_representation::arena::{EdgeId, FaceId, ShellId, SolidId, VertexId};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_representation::topology::history::{LabelSource, PersistentLabel};
use semio_framework_3d_brep_representation::topology::Body;
#[path="🎟️original/🦀️.rs"]
mod original;

/// 🧵️ An ordered chain of oriented edges, labelled because a wire has no arena identity of its own.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(crate = "::semio_framework_value")]
pub struct ShapeWire {
    pub members: Vec<(EdgeId, bool)>,
    pub vertices: Vec<VertexId>,
    pub closed: bool,
    pub label: PersistentLabel,
}

/// 🎯️ The root of a [`ShapeValue`]: arena ids into its own [`Body`] for topological kinds, an owned labelled value for the kinds with no arena identity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(crate = "::semio_framework_value")]
#[value(rename_all = "camelCase")]
pub enum ShapeRoot {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire(ShapeWire),
    Face(FaceId),
    Shell(ShellId),
    Solid(SolidId),
    Compound { solids: Vec<SolidId>, label: PersistentLabel },
    Curve { curve: Curve3, label: PersistentLabel },
    Surface { surface: Surface, label: PersistentLabel },
}

/// 🏷️ One labelled sub-element of a [`ShapeValue`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct ShapeComponent {
    pub kind: GeometryKind,
    pub label: PersistentLabel,
}

/// 📦️ An owned, session-free B-Rep shape: encodes canonically through `ToValue`, compares by value, and re-enters any [`Brep`] through [`Brep::import_shape`].
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(crate = "::semio_framework_value")]
pub struct ShapeValue {
    pub root: ShapeRoot,
    pub body: Body,
}

/// 📥️ What [`Brep::import_shape_mapped`] registered: the root handle plus the label shift every sub-element label of the imported value received.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportedShape {
    pub handle: GeometryHandle,
    pub label_offset: u64,
}

impl ImportedShape {
    /// 🏷️ The session label a label of the imported [`ShapeValue`] resolves to (identity in a fresh session).
    pub fn session_label(&self, label: PersistentLabel) -> PersistentLabel {
        PersistentLabel(label.0 + self.label_offset)
    }
}

impl ShapeValue {
    /// 🧭️ The geometry kind of the root.
    pub fn kind(&self) -> GeometryKind {
        match &self.root {
            ShapeRoot::Vertex(_) => GeometryKind::Vertex,
            ShapeRoot::Edge(_) => GeometryKind::Edge,
            ShapeRoot::Wire(_) => GeometryKind::Wire,
            ShapeRoot::Face(_) => GeometryKind::Face,
            ShapeRoot::Shell(_) => GeometryKind::Shell,
            ShapeRoot::Solid(_) => GeometryKind::Solid,
            ShapeRoot::Compound { .. } => GeometryKind::Compound,
            ShapeRoot::Curve { .. } => GeometryKind::Curve,
            ShapeRoot::Surface { .. } => GeometryKind::Surface,
        }
    }

    /// 🏷️ The persistent label of the root.
    pub fn label(&self) -> Option<PersistentLabel> {
        match &self.root {
            ShapeRoot::Vertex(id) => self.body.vertices.get(*id).map(|vertex| vertex.label),
            ShapeRoot::Edge(id) => self.body.edges.get(*id).map(|edge| edge.label),
            ShapeRoot::Wire(wire) => Some(wire.label),
            ShapeRoot::Face(id) => self.body.faces.get(*id).map(|face| face.label),
            ShapeRoot::Shell(id) => self.body.shells.get(*id).map(|shell| shell.label),
            ShapeRoot::Solid(id) => self.body.solids.get(*id).map(|solid| solid.label),
            ShapeRoot::Compound { label, .. } | ShapeRoot::Curve { label, .. } | ShapeRoot::Surface { label, .. } => Some(*label),
        }
    }

    /// 🏷️ Every labelled element of `kind` in the closure, in deterministic arena order, root included when it has that kind.
    pub fn components(&self, kind: GeometryKind) -> Vec<ShapeComponent> {
        let labels: Vec<PersistentLabel> = match kind {
            GeometryKind::Vertex => self.body.vertices.iter().map(|(_, vertex)| vertex.label).collect(),
            GeometryKind::Edge => self.body.edges.iter().map(|(_, edge)| edge.label).collect(),
            GeometryKind::Face => self.body.faces.iter().map(|(_, face)| face.label).collect(),
            GeometryKind::Shell => self.body.shells.iter().map(|(_, shell)| shell.label).collect(),
            GeometryKind::Solid => self.body.solids.iter().map(|(_, solid)| solid.label).collect(),
            GeometryKind::Wire | GeometryKind::Compound | GeometryKind::Curve | GeometryKind::Surface => self.label().filter(|_| self.kind() == kind).into_iter().collect(),
        };
        labels.into_iter().map(|label| ShapeComponent { kind, label }).collect()
    }

    /// 🔑️ Content hash of the canonical encoding: equal exactly when the values are equal.
    pub fn content_hash(&self) -> String {
        semio_framework_hash::hash_bytes(semio_framework_pack_json::to_json_string(self).as_bytes())
    }

    /// 🩺️ Refuses a value whose ids dangle, so a decoded value can never panic an import.
    pub fn check(&self) -> Result<(), BrepError> {
        let body = &self.body;
        let broken = |what: String| BrepError::InvalidInput(format!("shape value is inconsistent: {what}"));
        for (id, edge) in body.edges.iter() {
            if !body.curves3.contains(edge.curve) || !body.vertices.contains(edge.v0) || !body.vertices.contains(edge.v1) {
                return Err(broken(format!("edge {id}")));
            }
        }
        for (id, coedge) in body.coedges.iter() {
            if !body.edges.contains(coedge.edge) || !body.loops.contains(coedge.loop_id) || !body.coedges.contains(coedge.next) || !body.coedges.contains(coedge.prev) || coedge.pcurve.is_some_and(|pcurve| !body.curves2.contains(pcurve)) {
                return Err(broken(format!("coedge {id}")));
            }
        }
        for (id, ring) in body.loops.iter() {
            if !body.coedges.contains(ring.first) || !body.faces.contains(ring.face) {
                return Err(broken(format!("loop {id}")));
            }
        }
        for (id, face) in body.faces.iter() {
            if !body.surfaces.contains(face.surface) || face.outer.into_iter().chain(face.inners.iter().copied()).any(|ring| !body.loops.contains(ring)) {
                return Err(broken(format!("face {id}")));
            }
        }
        for (id, shell) in body.shells.iter() {
            if shell.faces.iter().any(|face| !body.faces.contains(*face)) {
                return Err(broken(format!("shell {id}")));
            }
        }
        for (id, solid) in body.solids.iter() {
            if !body.shells.contains(solid.outer) || solid.inners.iter().any(|shell| !body.shells.contains(*shell)) {
                return Err(broken(format!("solid {id}")));
            }
        }
        let resolved = match &self.root {
            ShapeRoot::Vertex(id) => body.vertices.contains(*id),
            ShapeRoot::Edge(id) => body.edges.contains(*id),
            ShapeRoot::Wire(wire) => wire.members.iter().all(|(edge, _)| body.edges.contains(*edge)) && wire.vertices.iter().all(|vertex| body.vertices.contains(*vertex)),
            ShapeRoot::Face(id) => body.faces.contains(*id),
            ShapeRoot::Shell(id) => body.shells.contains(*id),
            ShapeRoot::Solid(id) => body.solids.contains(*id),
            ShapeRoot::Compound { solids, .. } => solids.iter().all(|solid| body.solids.contains(*solid)),
            ShapeRoot::Curve { .. } | ShapeRoot::Surface { .. } => true,
        };
        let highest = self.label().map_or(0, |label| label.0 + 1);
        if !resolved || self.label().is_none() || body.labels.next() < highest {
            return Err(broken("root".into()));
        }
        Ok(())
    }

    /// ⏱️ A resumable, cancellable tessellation of this value that needs no session.
    pub fn tessellate_job(&self, deflection: f64) -> Result<ShapeTessellationJob, BrepError> {
        self.check()?;
        let job=match &self.root {
            ShapeRoot::Vertex(id)=>TessellationJob::for_vertex(&self.body,*id,deflection),
            ShapeRoot::Solid(_)|ShapeRoot::Face(_)|ShapeRoot::Shell(_)|ShapeRoot::Compound {..}|ShapeRoot::Wire(_)=>Ok(TessellationJob::new(deflection)),
            _=>return Err(BrepError::InvalidInput(format!("cannot tessellate {:?}",self.kind()))),
        };
        job.map(|job| ShapeTessellationJob { shape: self.clone(), job }).map_err(|error| map_err(&error))
    }

    /// 🔺️ One-shot tessellation of this value.
    pub fn tessellate(&self, deflection: f64) -> Result<MeshTransfer, BrepError> {
        let mut job = self.tessellate_job(deflection)?;
        loop {
            match job.step(usize::MAX)? {
                TessellationStep::Done(_) => return job.into_mesh().map(|(mesh, _)| mesh).ok_or_else(|| BrepError::Operation("tessellation produced no mesh".into())),
                TessellationStep::Cancelled(_) => return Err(BrepError::Operation("tessellation cancelled".into())),
                TessellationStep::Working(_) => {}
            }
        }
    }
}

/// ⏱️ A [`TessellationJob`] that owns the [`ShapeValue`] it steps against.
pub struct ShapeTessellationJob {
    shape: ShapeValue,
    job: TessellationJob,
}

impl ShapeTessellationJob {
    /// ⏱️ Advances by at most `budget` units.
    pub fn step(&mut self, budget: usize) -> Result<TessellationStep, BrepError> {
        use semio_framework_3d_brep_tessellation::TessellationInput;
        let input=match &self.shape.root {
            ShapeRoot::Vertex(id)=>TessellationInput::Vertex(*id),ShapeRoot::Solid(id)=>TessellationInput::Solid(*id),ShapeRoot::Face(id)=>TessellationInput::Face(*id),ShapeRoot::Shell(id)=>TessellationInput::Shell(*id),ShapeRoot::Compound {solids,..}=>TessellationInput::Solids(solids),ShapeRoot::Wire(wire)=>TessellationInput::Wire(&wire.members),_=>return Err(BrepError::InvalidInput("invalid tessellation root".into())),
        };
        self.job.step(&self.shape.body,input,budget).map_err(|error|map_err(&error))
    }

    /// 📈️ Progress right now.
    pub fn progress(&self) -> TessellationProgress {
        self.job.progress()
    }

    /// 🛑️ Retires the job at the next observable boundary.
    pub fn cancel(&mut self) {
        self.job.cancel();
    }

    /// ✅️ True once the job reached a terminal phase.
    pub fn is_terminal(&self) -> bool {
        self.job.is_terminal()
    }

    /// 📤️ Takes the finished mesh, `None` unless the job completed.
    pub fn into_mesh(self) -> Option<(MeshTransfer, TessellationReport)> {
        self.job.into_mesh()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
