//! 🧪️ The actual ephemeral owner admits the full schema stroke count within its existing physical envelope.
use super::*;
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::{PaintStroke,RasterBrush,RasterStrokePoint,RASTER_STROKE_MAXIMUM_POINTS};
#[test]
fn full_stroke_point_limit_fits_the_actual_ephemeral_store_envelope(){
 let law:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("fixture.json")).unwrap();assert_eq!(RASTER_STROKE_MAXIMUM_POINTS,law["maximumPoints"].as_u64().unwrap()as usize);assert_eq!(size_of::<RasterStrokePoint>(),law["pointBytes"].as_u64().unwrap()as usize);assert_eq!(protocol::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,law["maximumItemBytes"].as_u64().unwrap()as usize);
 for row in law["cases"].as_array().unwrap(){let mut caller=semio_plugin::artifact_app_laws::OriginalWorkFixture::new();let grant=semio_plugin::artifact_app_laws::OriginalWorkFixture::stock();let seed=caller.receive_native::<_,semio_framework_value::SharedUtf8>("actor:stroke".to_owned(),grant);let points=vec![RasterStrokePoint{x:1.0,y:2.0};row["points"].as_u64().unwrap()as usize];let leaf=PaintStroke{layer_id:"p".into(),target:"pixels".into(),tool:"brush".into(),brush:RasterBrush{size:1.0,hardness:1.0,opacity:1.0,color:vec![1.0,0.0,0.0,1.0]},points,selection:None};let value=RasterCompositeWindowTransientMutation::Snapshot{transient:RasterCompositeWindowTransient{stroke:Some(Box::new(RasterStrokeToolState{gesture:"press".into(),states:vec!["painting".into()],authoring_seed:seed,transaction:protocol::TransactionRef{id:"stroke".into(),tool:"paintStroke".into()},stroke:leaf,preview:None})),closed:None}};let result=RasterCompositeWindowTransientMutation::footprint(&value);assert_eq!(result.is_ok(),row["admitted"].as_bool().unwrap());if let Ok(footprint)=result{assert!(footprint.is_admissible());assert!(footprint.retained_bytes>65536);assert!(footprint.retained_bytes<1048576);}caller.retire_owned(value,grant);caller.finish();}
}

#[test]
fn preview_alias_preserves_original_snapshot_until_its_last_lease_closes() {
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;
    use semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot;
    use std::sync::Arc;

    let law: semio_framework_value::serde_json::Value =
        semio_framework_value::serde_json::from_str(include_str!("fixture.json")).unwrap();
    let row = &law["preview"];
    let revision = [row["baseRevisionByte"].as_u64().unwrap() as u8; 32];
    let other_revision = [row["differentRevisionByte"].as_u64().unwrap() as u8; 32];
    let id = row["documentId"].as_str().unwrap();
    let mut caller = OriginalWorkFixture::new();
    let grant = OriginalWorkFixture::stock();
    let mut snapshot = empty_raster_snapshot();
    snapshot.id = id.into();
    let owner = Arc::new(snapshot);
    let address = Arc::as_ptr(&owner);
    let preview = RasterStrokePreview { base_revision: revision, snapshot: ArcLease::new(owner) };
    let alias = preview.clone();
    assert_eq!(preview, alias);
    let seed = caller.receive_native::<_, semio_framework_value::SharedUtf8>("actor:stroke".to_owned(), grant);
    let window = RasterCompositeWindowTransient {
        stroke: Some(Box::new(RasterStrokeToolState {
            gesture: "press".into(),
            states: vec!["painting".into()],
            authoring_seed: seed,
            transaction: protocol::TransactionRef { id: "stroke".into(), tool: "paintStroke".into() },
            stroke: PaintStroke {
                layer_id: "p".into(),
                target: "pixels".into(),
                tool: "brush".into(),
                brush: RasterBrush { size: 1.0, hardness: 1.0, opacity: 1.0, color: vec![1.0, 0.0, 0.0, 1.0] },
                points: Vec::new(),
                selection: None,
            },
            preview: Some(alias),
        })),
        closed: None,
    };

    caller.retire_owned(preview, grant);
    let rendered = cached_stroke_preview(&window, &revision).expect("the window still owns its preview");
    assert_eq!(rendered as *const _, address);
    assert_eq!(rendered.id, id);
    assert!(cached_stroke_preview(&window, &other_revision).is_none());
    caller.retire_owned(window, grant);
    caller.finish();
}
