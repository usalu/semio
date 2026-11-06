//! 📬️ Paged structural edit for one boundary-representation vertex move.

use super::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_plugin::plugin_app_close_prelude::store::{self as app_store, PagedOneItemEdit, PagedOneItemEditStep, PagedOneItemPost, PagedOneItemPostMode, PagedOneItemPostRead};
use semio_framework_value::retained_clone::{RetainedCloneRef, RetainedCloneStep};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, RetirementDemand};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::sync::Arc;
use std::mem::size_of;

const PREFIX: &str = "stdio-semio-brep-set-vertex";

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<SemioBrepSnapshot, SemioBrepMutation>> {
    None
}

#[derive(Default)]
pub(super) struct BrepVertexEdit {
    last_work: u64,
    closing: bool,
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
            self.last_work+=1;
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
        self.last_work+=1;
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
        self.last_work+=1;
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

    fn post_mode(&self) -> PagedOneItemPostMode {
        PagedOneItemPostMode::Mutable
    }

    fn advance_demands(&self, _before: RetainedCloneRef<'_, SemioBrepSnapshot>, _mutation: RetainedCloneRef<'_, SemioBrepMutation>, _post: PagedOneItemPostRead<'_, SemioBrepSnapshot>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn advance(&mut self, _before: RetainedCloneRef<'_, SemioBrepSnapshot>, mutation: RetainedCloneRef<'_, SemioBrepMutation>, post: PagedOneItemPost<'_, SemioBrepSnapshot>, _body: usize, grant: RetainedCloneGrant, receiving: &mut NativeDecodeControl<'_>) -> Result<PagedOneItemEditStep<SemioBrepMutation>, ValueError> {
        self.last_work=0;
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PagedOneItemEditStep::Progress(Default::default()))}
        if !receiving.prepare_work(1)?{return Ok(PagedOneItemEditStep::Progress(Default::default()))}
        let result=(||{
        let mutation = mutation.get();
        let target = Self::target(mutation)?;
        let post = post.mutable()?;
        if self.scanned { self.apply(post, target, grant) } else { self.scan(post, target, grant) }
        })();
        receiving.commit_work(self.last_work)?;
        result
    }

    fn last_turn_work(&self)->u64{self.last_work}

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_demands(&self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<SemioBrepMutation>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn close_step(&mut self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<SemioBrepMutation>, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.closing = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
        use crate::schema::snapshot::{BrepCurve, BrepEdge, BrepVertex};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../../../../../../../📇️registry/🧬️contract/✏️editing/🧪️tests/🧫️fixtures/🧵️retained-native/🔣️.json")).expect("retained native fixture")
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


    type Fixture=semio_os_store_one_item::testkit::PagedEditFixture<SemioBrepSnapshot,SemioBrepMutation,BrepVertexEdit>;
    fn close_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:16_777_216,maximum_release_bytes:16_777_216,maximum_depth:4096}}
    fn setup(post:SemioBrepSnapshot,mutation:SemioBrepMutation)->Fixture{
        use semio_os_store_one_item::testkit::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
        let before=semio_framework_value::testing::admit_owned_source(post.clone(),()).0;
        let mutation=semio_framework_value::testing::admit_owned_source(mutation,()).0;
        let caller=OriginalJobFixtureCaller::admit(OperationId(44791),Generation(8),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:16_777_216,maximum_capacity_bytes:16_777_216,maximum_release_bytes:16_777_216,maximum_depth:4096},||Some(1),u64::MAX,1<<20);
        Fixture::new(caller,OriginalJobDecodeOwner::empty(),before,mutation,post,BrepVertexEdit::default())
    }
    fn retire<T:semio_framework_value::RetireOwned+'static>(fixture:&mut Fixture,value:T){let mut value=Some(value);let mut active=None;assert!(fixture.retire_owned(&mut value,&mut active,close_grant(),1_000_000).expect("funded fixture output retirement"));assert!(value.is_none()&&active.is_none());}

    #[test]
    fn edit_scans_one_vertex_per_turn_and_moves_exactly_the_addressed_vertex(){
        let source=snapshot();let id=source.vertices.last().unwrap().id.clone();let mut fixture=setup(source.clone(),mutation(&id));let mut turns=0;
        let inverse=loop{let outcome=fixture.advance(grant()).expect("original brep caller");assert_eq!(outcome.work,fixture.last_work());assert!(outcome.work<=1);match outcome.edit.expect("brep edit"){
            PagedOneItemEditStep::Progress(p)=>assert!(p.fits(grant())&&p.copied_items<=1),PagedOneItemEditStep::Complete(p,inverse)=>{assert!(p.fits(grant()));break inverse},_=>panic!("brep completes with one original inverse"),
        }turns+=1;assert!(turns<10_000);};
        let post=fixture.post().unwrap();assert_eq!(post.edges,source.edges);assert_eq!(post.faces,source.faces);assert_eq!(post.next_label,source.next_label);assert_eq!(post.vertices.last().unwrap().point,SemioPoint3{x:-1.0,y:-2.0,z:-3.0});assert_eq!(post.vertices.len(),source.vertices.len());
        let SemioBrepMutation::MoveVertex(value)=&inverse else{panic!("brep inverse")};assert_eq!(value.new_point,source.vertices.last().unwrap().point);retire(&mut fixture,inverse);assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }
    #[test]
    fn edit_refuses_an_absent_target_before_touching_any_vertex(){
        let source=snapshot();let mut fixture=setup(source.clone(),mutation("absent"));let error=loop{let outcome=fixture.advance(grant()).unwrap();match outcome.edit{Ok(PagedOneItemEditStep::Progress(_))=>{},Ok(_)=>panic!("absent brep cannot complete"),Err(error)=>{assert_eq!(outcome.work,1);break error}}};
        assert!(format!("{error:?}").contains("target-count-0"));assert_eq!(fixture.post().unwrap(),&source);let reset=fixture.advance(RetainedCloneGrant{maximum_items:0,..grant()}).unwrap();assert_eq!(reset.work,0);assert_eq!(reset.progress,Default::default());retire(&mut fixture,error);assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }
    #[test]
    fn edit_waits_without_progress_when_the_grant_cannot_cover_one_turn(){
        let source=snapshot();let id=source.vertices.last().unwrap().id.clone();let mut fixture=setup(source.clone(),mutation(&id));
        let starved=fixture.advance(RetainedCloneGrant{maximum_copy_bytes:0,..grant()}).unwrap();assert_eq!(starved.work,0);assert_eq!(starved.progress,Default::default());assert_eq!(fixture.post().unwrap(),&source);let paid=fixture.advance(grant()).unwrap();assert_eq!(paid.work,1);
        let denied=fixture.advance_with_work(grant(),0).unwrap();assert_eq!(denied.work,0);assert_eq!(denied.progress,Default::default());assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }

}
