//! 📬️ Paged structural edit for one boundary-representation vertex move.

use super::*;
use crate::{
    standards::v1::subsets::base::schema::geometry::SemioPoint3,
};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_framework_plugin::plugin_app_close_prelude::store::{self as app_store, PagedOneItemEdit, PagedOneItemEditStep};
use std::sync::Arc;
use std::mem::size_of;

const PREFIX: &str = "stdio-semio-brep-set-vertex";

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<SemioBrepSnapshot, SemioBrepMutation>> {
    Some(NativeEditPreparationRoute::new(BrepVertexEdit::recognizes, Arc::new(app_store::PagedOneItemPreparationFactory::<SemioBrepSnapshot, SemioBrepMutation, BrepVertexEdit>::default())))
}

#[derive(Default)]
pub(super) struct BrepVertexEdit {
    vertex_index: usize,
    matches: usize,
    located: Option<usize>,
    scanned: bool,
}

impl BrepVertexEdit {
    fn target(mutation: &SemioBrepMutation) -> Result<&MoveVertex, ValueError> {
        let SemioBrepMutation::MoveVertex(value) = mutation else { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "stdio-semio-brep-set-vertex-mutation")) };
        Ok(value)
    }

    fn scan(&mut self, post: &SemioBrepSnapshot, target: &MoveVertex, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioBrepMutation>, ValueError> {
        let unit = |bytes: usize| RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() };
        let Some(vertex) = post.vertices.get(self.vertex_index) else {
            if self.matches != 1 {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{PREFIX}-target-count-{}", self.matches)));
            }
            self.scanned = true;
            return Ok(PagedOneItemEditStep::Progress(unit(0)));
        };
        let bytes = vertex.id.len();
        if grant.maximum_copy_bytes < bytes {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        if vertex.id == target.vertex_id {
            self.matches += 1;
            self.located = Some(self.vertex_index);
        }
        self.vertex_index += 1;
        Ok(PagedOneItemEditStep::Progress(unit(bytes)))
    }

    fn apply(&mut self, post: &mut SemioBrepSnapshot, target: &MoveVertex, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioBrepMutation>, ValueError> {
        let copied = size_of::<SemioPoint3>();
        let retained = target.vertex_id.len();
        if grant.maximum_copy_bytes < copied || grant.maximum_capacity_bytes < retained {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        let index = self.located.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "stdio-semio-brep-set-vertex-located"))?;
        let vertex = post.vertices.get_mut(index).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "stdio-semio-brep-set-vertex-vertex-index"))?;
        let old_point = std::mem::replace(&mut vertex.point, target.new_point);
        let inverse = SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: target.vertex_id.clone(), new_point: old_point });
        Ok(PagedOneItemEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, retained_capacity_bytes: retained, released_bytes: 0 }, inverse))
    }
}

impl PagedOneItemEdit<SemioBrepSnapshot, SemioBrepMutation> for BrepVertexEdit {
    const PREFIX: &'static str = PREFIX;

    fn recognizes(mutation: &SemioBrepMutation) -> bool {
        matches!(mutation, SemioBrepMutation::MoveVertex(_))
    }

    fn preflight(mutation: &SemioBrepMutation) -> Result<usize, String> {
        let value = Self::target(mutation).map_err(|_| format!("{PREFIX}-mutation"))?;
        value.vertex_id.len().checked_add(size_of::<SemioPoint3>()).ok_or_else(|| format!("{PREFIX}-payload-overflow"))
    }

    fn advance(&mut self, post: &mut SemioBrepSnapshot, mutation: &SemioBrepMutation, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioBrepMutation>, ValueError> {
        let target = Self::target(mutation)?;
        if self.scanned { self.apply(post, target, grant) } else { self.scan(post, target, grant) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
        use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepVertex};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🧫️fixtures/🧵️retained-native/🔣️.json"))).expect("retained native fixture")
    }

    fn snapshot() -> SemioBrepSnapshot {
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

    fn mutation(vertex_id: &str) -> SemioBrepMutation {
        SemioBrepMutation::MoveVertex(MoveVertex { vertex_id: vertex_id.into(), new_point: SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 } })
    }

    fn grant() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 4_096, maximum_release_bytes: 4_096, maximum_depth: 8 }
    }

    #[test]
    fn edit_scans_one_vertex_per_turn_and_moves_exactly_the_addressed_vertex() {
        let source = snapshot();
        let target = source.vertices.last().expect("target vertex").id.clone();
        let mutation = mutation(&target);
        let mut post = source.clone();
        let mut edit = BrepVertexEdit::default();
        let mut turns = 0;
        let (progress, inverse) = loop {
            match edit.advance(&mut post, &mutation, grant()).expect("brep edit turn") {
                PagedOneItemEditStep::Progress(progress) => assert!(progress.fits(grant()) && progress.copied_items <= 1),
                PagedOneItemEditStep::Complete(progress, inverse) => break (progress, inverse),
            }
            turns += 1;
            assert!(turns < 50_000);
        };
        assert!(progress.fits(grant()));
        assert!(turns > source.vertices.len());
        assert_eq!(post.edges, source.edges);
        assert_eq!(post.next_label, source.next_label);
        assert_eq!(post.vertices.last().expect("post target").point, SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 });
        assert_eq!(post.vertices[..post.vertices.len() - 1], source.vertices[..source.vertices.len() - 1]);
        let SemioBrepMutation::MoveVertex(inverse) = inverse else { panic!("brep inverse") };
        assert_eq!(inverse.new_point, source.vertices.last().expect("source target").point);
    }

    #[test]
    fn edit_refuses_an_absent_target_before_touching_any_vertex() {
        let source = snapshot();
        let mutation = mutation("absent");
        let mut post = source.clone();
        let mut edit = BrepVertexEdit::default();
        let error = loop {
            match edit.advance(&mut post, &mutation, grant()) {
                Ok(PagedOneItemEditStep::Progress(_)) => {}
                Ok(PagedOneItemEditStep::Complete(..)) => panic!("absent vertex must not complete"),
                Err(error) => break error,
            }
        };
        assert!(format!("{error:?}").contains("target-count-0"));
        assert_eq!(post, source);
    }

    #[test]
    fn edit_waits_without_progress_when_the_grant_cannot_cover_one_turn() {
        let source = snapshot();
        let mutation = mutation("v-0");
        let mut post = source.clone();
        let mut edit = BrepVertexEdit::default();
        let starved = RetainedCloneGrant { maximum_copy_bytes: 0, ..grant() };
        match edit.advance(&mut post, &mutation, starved).expect("starved brep edit turn") {
            PagedOneItemEditStep::Progress(progress) => assert_eq!(progress, RetainedCloneProgress::default()),
            PagedOneItemEditStep::Complete(..) => panic!("starved grant must not complete"),
        }
        assert_eq!(post, source);
    }
}
