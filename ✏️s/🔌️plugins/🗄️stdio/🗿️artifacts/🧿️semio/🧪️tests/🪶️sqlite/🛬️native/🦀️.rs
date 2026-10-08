//! 🛬️ Explicit concrete Semio declaration owners and controlled native construction.
use store::{ArtifactSqliteSnapshot,ArtifactDsl,ArtifactPack,ArtifactCodec};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits,SnapshotEncoding};

fn check<P:ArtifactSqliteSnapshot+ArtifactDsl+ArtifactPack+Default+PartialEq+std::fmt::Debug+'static>(name:&str,subset:&str,codec:ArtifactCodec,failures:&mut Vec<String>){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert!(fixture["owners"].as_array().unwrap().iter().any(|value|value.as_str()==Some(name)));
    let snapshot=P::default();
    let Some(provider)=codec.snapshot_sqlite.as_ref() else{failures.push(format!("{name}: declaration has no capability"));return};
    if provider.snapshot_type!=Some(std::any::TypeId::of::<P>()){failures.push(format!("{name}: declaration type differs"));return;}
    let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:subset.into()};
    for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
        let payload=match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(snapshot.encode_pack()),SnapshotEncoding::Text=>store::io::IoPayload::Text(snapshot.print_dsl())};
        match P::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())){Ok(actual)=>assert_eq!(actual,snapshot,"{name} {encoding:?}"),Err(error)=>{failures.push(format!("{name} {encoding:?}: {error}"));continue;}}
        if let Err(error)=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())){failures.push(format!("{name} declaration {encoding:?}: {error:?}"));}
    }
}

#[test]
fn sqlite_snapshot_000_semio_every_concrete_bare_declaration_owner_has_controlled_native_construction(){
    use crate::standards::v1::subsets;
    crate::declaration(crate::definition().unwrap()).unwrap();
    let mut failures=Vec::new();
    check::<subsets::base::schema::snapshot::SemioSnapshot>("base","*",ArtifactCodec::bare::<subsets::base::schema::snapshot::SemioSnapshot,subsets::base::schema::mutations::SemioMutation>(<subsets::base::schema::snapshot::SemioSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::text::schema::snapshot::SemioTextSnapshot>("text","text",ArtifactCodec::bare::<subsets::text::schema::snapshot::SemioTextSnapshot,subsets::text::schema::mutations::SemioTextMutation>(<subsets::text::schema::snapshot::SemioTextSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::value::schema::snapshot::SemioValueSnapshot>("value","value",ArtifactCodec::bare::<subsets::value::schema::snapshot::SemioValueSnapshot,subsets::value::schema::mutations::SemioValueMutation>(<subsets::value::schema::snapshot::SemioValueSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::audio::schema::snapshot::SemioAudioSnapshot>("audio","audio",ArtifactCodec::bare::<subsets::audio::schema::snapshot::SemioAudioSnapshot,subsets::audio::schema::mutations::SemioAudioMutation>(<subsets::audio::schema::snapshot::SemioAudioSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::video::schema::snapshot::SemioVideoSnapshot>("video","video",ArtifactCodec::bare::<subsets::video::schema::snapshot::SemioVideoSnapshot,subsets::video::schema::mutations::SemioVideoMutation>(<subsets::video::schema::snapshot::SemioVideoSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::image::schema::snapshot::SemioImageSnapshot>("image","image",ArtifactCodec::bare::<subsets::image::schema::snapshot::SemioImageSnapshot,subsets::image::schema::mutations::SemioImageMutation>(<subsets::image::schema::snapshot::SemioImageSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::flow::schema::snapshot::SemioFlowSnapshot>("flow","flow",ArtifactCodec::bare::<subsets::flow::schema::snapshot::SemioFlowSnapshot,subsets::flow::schema::mutations::SemioFlowMutation>(<subsets::flow::schema::snapshot::SemioFlowSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::table::schema::snapshot::SemioTableSnapshot>("table","table",ArtifactCodec::bare::<subsets::table::schema::snapshot::SemioTableSnapshot,subsets::table::schema::mutations::SemioTableMutation>(<subsets::table::schema::snapshot::SemioTableSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::graph::schema::snapshot::SemioGraphSnapshot>("graph","graph",ArtifactCodec::bare::<subsets::graph::schema::snapshot::SemioGraphSnapshot,subsets::graph::schema::mutations::SemioGraphMutation>(<subsets::graph::schema::snapshot::SemioGraphSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::animation::schema::snapshot::SemioAnimationSnapshot>("animation","animation",ArtifactCodec::bare::<subsets::animation::schema::snapshot::SemioAnimationSnapshot,subsets::animation::schema::mutations::SemioAnimationMutation>(<subsets::animation::schema::snapshot::SemioAnimationSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::object::schema::snapshot::SemioObjectSnapshot>("object","object",ArtifactCodec::bare::<subsets::object::schema::snapshot::SemioObjectSnapshot,subsets::object::schema::mutations::SemioObjectMutation>(<subsets::object::schema::snapshot::SemioObjectSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::kit::schema::snapshot::SemioKitSnapshot>("kit","kit",ArtifactCodec::bare::<subsets::kit::schema::snapshot::SemioKitSnapshot,subsets::kit::schema::mutations::SemioKitMutation>(<subsets::kit::schema::snapshot::SemioKitSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::mesh::schema::snapshot::SemioMeshSnapshot>("mesh","mesh",ArtifactCodec::bare::<subsets::mesh::schema::snapshot::SemioMeshSnapshot,subsets::mesh::schema::mutations::SemioMeshMutation>(<subsets::mesh::schema::snapshot::SemioMeshSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::model::schema::snapshot::SemioModelSnapshot>("model","model",ArtifactCodec::bare::<subsets::model::schema::snapshot::SemioModelSnapshot,subsets::model::schema::mutations::SemioModelMutation>(<subsets::model::schema::snapshot::SemioModelSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::cad::schema::snapshot::SemioCadSnapshot>("cad","cad",ArtifactCodec::bare::<subsets::cad::schema::snapshot::SemioCadSnapshot,subsets::cad::schema::mutations::SemioCadMutation>(<subsets::cad::schema::snapshot::SemioCadSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::document::schema::snapshot::SemioDocumentSnapshot>("document","document",ArtifactCodec::bare::<subsets::document::schema::snapshot::SemioDocumentSnapshot,subsets::document::schema::mutations::SemioDocumentMutation>(<subsets::document::schema::snapshot::SemioDocumentSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::drawing::schema::snapshot::SemioDrawingSnapshot>("drawing","drawing",ArtifactCodec::bare::<subsets::drawing::schema::snapshot::SemioDrawingSnapshot,subsets::drawing::schema::mutations::SemioDrawingMutation>(<subsets::drawing::schema::snapshot::SemioDrawingSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::presentation::schema::snapshot::SemioPresentationSnapshot>("presentation","presentation",ArtifactCodec::bare::<subsets::presentation::schema::snapshot::SemioPresentationSnapshot,subsets::presentation::schema::mutations::SemioPresentationMutation>(<subsets::presentation::schema::snapshot::SemioPresentationSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    check::<subsets::brep::schema::snapshot::SemioBrepSnapshot>("brep","brep",ArtifactCodec::bare::<subsets::brep::schema::snapshot::SemioBrepSnapshot,subsets::brep::schema::mutations::SemioBrepMutation>(<subsets::brep::schema::snapshot::SemioBrepSnapshot as ArtifactDsl>::envelope_id()),&mut failures);
    assert!(failures.is_empty(),"{}",failures.join("\n"));
}
