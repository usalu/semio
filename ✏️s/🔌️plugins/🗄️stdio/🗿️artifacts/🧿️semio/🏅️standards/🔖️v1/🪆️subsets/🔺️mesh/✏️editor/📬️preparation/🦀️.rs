//! 📬️ Paged structural edit for one mesh vertex move.

use super::*;
use {semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3, crate::schema::mutations::move_vertex};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_plugin::plugin_app_close_prelude::store::{self as app_store, PagedOneItemEdit, PagedOneItemEditStep, PagedOneItemPost, PagedOneItemPostMode, PagedOneItemPostRead};
use semio_framework_value::retained_clone::{RetainedCloneRef, RetainedCloneStep};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, RetirementDemand};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::sync::Arc;
use std::mem::size_of;

const PREFIX: &str = "stdio-semio-mesh-set-vertex";

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<SemioMeshSnapshot, SemioMeshMutation>> {
    None
}

#[derive(Default)]
pub(super) struct MeshVertexEdit {
    last_work: u64,
    closing: bool,
    mesh_index: usize,
    primitive_index: usize,
    mesh_matches: usize,
    primitive_matches: usize,
    located: Option<(usize, usize)>,
    scanned: bool,
}

impl MeshVertexEdit {
    fn target(mutation: &SemioMeshMutation) -> Result<&move_vertex::MoveVertex, ValueError> {
        let SemioMeshMutation::MoveVertex(value) = mutation else { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "stdio-semio-mesh-set-vertex-mutation")) };
        Ok(value)
    }

    fn scan(&mut self, post: &SemioMeshSnapshot, target: &move_vertex::MoveVertex, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioMeshMutation>, ValueError> {
        let unit = |bytes: usize| RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() };
        let Some(mesh) = post.meshes.get(self.mesh_index) else {
            self.last_work+=1;
            if self.mesh_matches != 1 || self.primitive_matches != 1 {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{PREFIX}-target-count-{}-{}", self.mesh_matches, self.primitive_matches)));
            }
            self.scanned = true;
            return Ok(PagedOneItemEditStep::Progress(unit(0)));
        };
        if mesh.id != target.mesh_id {
            let bytes = mesh.id.len();
            if grant.maximum_copy_bytes < bytes {
                return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
            }
            self.last_work+=1;
            self.mesh_index += 1;
            return Ok(PagedOneItemEditStep::Progress(unit(bytes)));
        }
        let Some(primitive) = mesh.primitives.get(self.primitive_index) else {
            self.last_work+=1;
            self.mesh_matches += 1;
            self.mesh_index += 1;
            self.primitive_index = 0;
            return Ok(PagedOneItemEditStep::Progress(unit(0)));
        };
        let bytes = primitive.id.len();
        if grant.maximum_copy_bytes < bytes {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        self.last_work+=1;
        if primitive.id == target.primitive_id {
            self.primitive_matches += 1;
            self.located = Some((self.mesh_index, self.primitive_index));
        }
        self.primitive_index += 1;
        Ok(PagedOneItemEditStep::Progress(unit(bytes)))
    }

    fn apply(&mut self, post: &mut SemioMeshSnapshot, target: &move_vertex::MoveVertex, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<SemioMeshMutation>, ValueError> {
        let copied = size_of::<SemioPoint3>();
        let retained = target.mesh_id.len().checked_add(target.primitive_id.len()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "stdio-semio-mesh-set-vertex-inverse-overflow"))?;
        if grant.maximum_copy_bytes < copied || grant.maximum_capacity_bytes < retained {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        self.last_work+=1;
        let (mesh_index, primitive_index) = self.located.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "stdio-semio-mesh-set-vertex-located"))?;
        let slot = post
            .meshes
            .get_mut(mesh_index)
            .and_then(|mesh| mesh.primitives.get_mut(primitive_index))
            .and_then(|primitive| primitive.positions.get_mut(target.vertex_index))
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "stdio-semio-mesh-set-vertex-vertex-index"))?;
        let old_point = std::mem::replace(slot, target.new_point);
        let inverse = SemioMeshMutation::MoveVertex(move_vertex::MoveVertex { mesh_id: target.mesh_id.clone(), primitive_id: target.primitive_id.clone(), vertex_index: target.vertex_index, new_point: old_point });
        Ok(PagedOneItemEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, retained_capacity_bytes: retained, released_bytes: 0 }, inverse))
    }
}

impl PagedOneItemEdit<SemioMeshSnapshot, SemioMeshMutation> for MeshVertexEdit {
    const PREFIX: &'static str = PREFIX;

    fn recognizes(mutation: &SemioMeshMutation) -> bool {
        matches!(mutation, SemioMeshMutation::MoveVertex(_))
    }

    fn preflight(mutation: &SemioMeshMutation) -> Result<usize, String> {
        let value = Self::target(mutation).map_err(|_| format!("{PREFIX}-mutation"))?;
        value.mesh_id.len().checked_add(value.primitive_id.len()).and_then(|bytes| bytes.checked_add(size_of::<usize>() + size_of::<SemioPoint3>())).ok_or_else(|| format!("{PREFIX}-payload-overflow"))
    }

    fn post_mode(&self) -> PagedOneItemPostMode {
        PagedOneItemPostMode::Mutable
    }

    fn advance_demands(&self, _before: RetainedCloneRef<'_, SemioMeshSnapshot>, _mutation: RetainedCloneRef<'_, SemioMeshMutation>, _post: PagedOneItemPostRead<'_, SemioMeshSnapshot>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn advance(&mut self, _before: RetainedCloneRef<'_, SemioMeshSnapshot>, mutation: RetainedCloneRef<'_, SemioMeshMutation>, post: PagedOneItemPost<'_, SemioMeshSnapshot>, _body: usize, grant: RetainedCloneGrant, receiving: &mut NativeDecodeControl<'_>) -> Result<PagedOneItemEditStep<SemioMeshMutation>, ValueError> {
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

    fn close_demands(&self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<SemioMeshMutation>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn close_step(&mut self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<SemioMeshMutation>, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
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
    use crate::schema::snapshot::{SemioMaterial, SemioMesh, SemioPrimitive, SemioTexture};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../../../../../../../📇️registry/🧬️contract/✏️editing/🧪️tests/🧫️fixtures/🧵️retained-native/🔣️.json")).expect("retained native fixture")
    }

    fn snapshot() -> SemioMeshSnapshot {
        let vertex_count = fixture()["structuralCopy"]["meshVertexCount"].as_u64().expect("mesh vertex count") as usize;
        SemioMeshSnapshot {
            schema: SEMIO_MESH_DOCUMENT_SCHEMA.into(),
            meshes: vec![
                SemioMesh { id: "other".into(), primitives: vec![SemioPrimitive { id: "primitive".into(), positions: vec![SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }], ..Default::default() }] },
                SemioMesh { id: "mesh".into(), primitives: vec![SemioPrimitive { id: "primitive".into(), positions: (0..vertex_count).map(|index| SemioPoint3 { x: index as f64, y: 1.0, z: 2.0 }).collect(), ..Default::default() }] },
            ],
            materials: vec![SemioMaterial { id: "material".into(), ..Default::default() }],
            textures: vec![SemioTexture { id: "texture".into(), mime: "application/octet-stream".into(), bytes: vec![7; 64] }],
        }
    }

    fn mutation(snapshot: &SemioMeshSnapshot, mesh_id: &str) -> SemioMeshMutation {
        SemioMeshMutation::MoveVertex(move_vertex::MoveVertex { mesh_id: mesh_id.into(), primitive_id: "primitive".into(), vertex_index: snapshot.meshes[1].primitives[0].positions.len() - 1, new_point: SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 } })
    }

    fn grant() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 4_096, maximum_release_bytes: 4_096, maximum_depth: 8 }
    }


    type Fixture=semio_os_store_one_item::testkit::PagedEditFixture<SemioMeshSnapshot,SemioMeshMutation,MeshVertexEdit>;
    fn close_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:16_777_216,maximum_release_bytes:16_777_216,maximum_depth:4096}}
    fn setup(post:SemioMeshSnapshot,mutation:SemioMeshMutation)->Fixture{
        use semio_os_store_one_item::testkit::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
        let before=semio_framework_value::testing::admit_owned_source(post.clone(),()).0;
        let mutation=semio_framework_value::testing::admit_owned_source(mutation,()).0;
        let caller=OriginalJobFixtureCaller::admit(OperationId(44791),Generation(8),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:16_777_216,maximum_capacity_bytes:16_777_216,maximum_release_bytes:16_777_216,maximum_depth:4096},||Some(1),u64::MAX,1<<20);
        Fixture::new(caller,OriginalJobDecodeOwner::empty(),before,mutation,post,MeshVertexEdit::default())
    }
    fn retire<T:semio_framework_value::RetireOwned+'static>(fixture:&mut Fixture,value:T){let mut value=Some(value);let mut active=None;assert!(fixture.retire_owned(&mut value,&mut active,close_grant(),1_000_000).expect("funded fixture output retirement"));assert!(value.is_none()&&active.is_none());}

    #[test]
    fn edit_scans_one_unit_per_turn_and_moves_exactly_the_addressed_vertex(){
        let source=snapshot();let mut fixture=setup(source.clone(),mutation(&source,"mesh"));let mut turns=0;
        let inverse=loop{let outcome=fixture.advance(grant()).expect("original mesh caller");assert_eq!(outcome.work,fixture.last_work());assert!(outcome.work<=1);match outcome.edit.expect("mesh edit"){
            PagedOneItemEditStep::Progress(p)=>assert!(p.fits(grant())&&p.copied_items<=1),
            PagedOneItemEditStep::Complete(p,inverse)=>{assert!(p.fits(grant()));break inverse},
            _=>panic!("mesh completes with one original inverse"),
        }turns+=1;assert!(turns<10_000);};
        let post=fixture.post().unwrap();assert_eq!(post.textures,source.textures);assert_eq!(post.materials,source.materials);assert_eq!(post.meshes[0],source.meshes[0]);assert_eq!(post.meshes[1].primitives[0].positions.last(),Some(&SemioPoint3{x:-1.0,y:-2.0,z:-3.0}));assert_eq!(post.meshes[1].primitives[0].positions.len(),source.meshes[1].primitives[0].positions.len());
        let SemioMeshMutation::MoveVertex(value)=&inverse else{panic!("mesh inverse")};assert_eq!(value.new_point,*source.meshes[1].primitives[0].positions.last().unwrap());retire(&mut fixture,inverse);assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }
    #[test]
    fn edit_refuses_an_absent_target_before_touching_any_vertex(){
        let source=snapshot();let mut fixture=setup(source.clone(),mutation(&source,"absent"));
        let error=loop{let outcome=fixture.advance(grant()).expect("original mesh caller");match outcome.edit{
            Ok(PagedOneItemEditStep::Progress(_))=>{},Ok(_)=>panic!("absent mesh cannot complete"),Err(error)=>{assert_eq!(outcome.work,1);break error},
        }};
        assert!(format!("{error:?}").contains("target-count-0-0"));assert_eq!(fixture.post().unwrap(),&source);
        let reset=fixture.advance(RetainedCloneGrant{maximum_items:0,..grant()}).unwrap();assert_eq!(reset.work,0);assert_eq!(reset.progress,Default::default());retire(&mut fixture,error);assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }
    #[test]
    fn edit_waits_without_progress_when_the_grant_cannot_cover_one_turn(){
        let source=snapshot();let mut fixture=setup(source.clone(),mutation(&source,"mesh"));
        let starved=fixture.advance(RetainedCloneGrant{maximum_copy_bytes:0,..grant()}).unwrap();assert_eq!(starved.work,0);assert_eq!(starved.progress,Default::default());assert_eq!(fixture.post().unwrap(),&source);
        let paid=fixture.advance(grant()).unwrap();assert_eq!(paid.work,1);let denied=fixture.advance_with_work(grant(),0).unwrap();assert_eq!(denied.work,0);assert_eq!(denied.progress,Default::default());
        assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }

}
