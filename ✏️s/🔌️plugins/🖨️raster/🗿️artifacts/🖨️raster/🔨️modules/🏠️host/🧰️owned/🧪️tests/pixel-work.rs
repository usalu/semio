//! 🧪️ Every semantic pixel route must yield through image preparation and close unpublished work on cancellation.
use super::*;
use semio_s_artifact_raster_raster_mutations_pixels::{apply_filter, fill_region, fill_selection, paint_stroke, transform_image, write_pixel_region};

fn pixel_route(name: &str) -> RasterMutation {
    match name {
        "paint-stroke" => paint_stroke::paint_stroke("paint", "pixels", "brush", paint_stroke::RasterBrush { size: 1.0, hardness: 1.0, opacity: 1.0, color: vec![1.0,0.0,0.0,1.0] }, vec![paint_stroke::RasterStrokePoint { x: 0.5, y: 0.5 }]),
        "fill-region" => fill_region::fill_region("paint", "pixels", fill_region::RasterSeed { x: 0, y: 0 }, 0, [1.0,0.0,0.0,1.0]),
        "apply-filter" => apply_filter::apply_filter("paint", "invert", 0.0),
        "transform-image" => transform_image::rotate_image("paint", true),
        "fill-selection" => fill_selection::fill_selection("paint", "pixels", [1.0,0.0,0.0,1.0]),
        "write-pixel-region" => RasterMutation::WritePixelRegion(write_pixel_region::WritePixelRegion { layer_id: "paint".into(), target: "pixels".into(), x: 0, y: 0, width: 1, height: 1, samples: vec![255,0,0,255] }),
        _ => panic!("unknown neutral pixel route"),
    }
}

fn pixel_source(fixture: &serde_json::Value) -> RasterSnapshot {
    let mut source = empty_raster_document();
    let width = fixture["source"]["width"].as_u64().unwrap() as u32;
    let height = fixture["source"]["height"].as_u64().unwrap() as u32;
    let image = semio_s_artifact_raster_raster_model::semio_image_from_rgba8(width,height,vec![0;width as usize*height as usize*4]);
    source.assets.insert("source".into(),semio_s_artifact_raster_raster_model::mint_raster_image_child("source",&image)).unwrap();
    source.layers.push(RasterLayerNode::Pixel { id: "paint".into(), name: "Paint".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(width), height: Some(height), image_key: Some("source".into()) });
    source
}

fn prepared_mutation(source:&RasterSnapshot,mutation:&RasterMutation)->RasterPreparedMutation{
    let mut apply=RasterOneItemApply::new();
    for _ in 0..100_000{let demand=apply.advance_demands(source,mutation,32768).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let turn=apply.advance_controlled(source,mutation,OperationId(753),semio_framework_job::Generation(1),256,grant,||false);assert!(turn.work<=256);let(post,receipt)=turn.result.unwrap();assert!(receipt.fits(grant));if let Some(post)=post{assert!(apply.terminal_is_empty());return post;}}
    panic!("original mutation preparation failed to complete");
}

#[test]
fn replay_candidate_uses_actual_original_control_and_refuses_foreign_caller(){
 use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;
 let row:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/replay-source.json")).unwrap();let mut fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();fixture["source"]["width"]=1.into();fixture["source"]["height"]=1.into();let source=pixel_source(&fixture);
 let mutation=RasterMutation::RenameLayer(rename_layer::RenameLayer{layer_id:"paint".into(),new_name:row["renamed"].as_str().unwrap().into()});let context=store::os_store::ArtifactReplayPreparationContext{mode:store::os_store::ArtifactReplayPreparationMode::Prefix,operation_index:row["operationIndex"].as_u64().unwrap()as u32,generation:row["generation"].as_u64().unwrap(),base_revision:[row["revisionByte"].as_u64().unwrap()as u8;32]};let mut apply=RasterOneItemApply::new();let mut caller=OriginalWorkFixture::new();let mut foreign=OriginalWorkFixture::new();let mut prepared=None;
 for index in 0..10000{
  let d=apply.advance_demands(&source,&mutation,32768).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  caller.work_turn(grant,256,|cx|{let mut work=0;let result=cx.with_native_receiving::<_,ValueError>(|control|{let turn=apply.advance_replay_controlled(&source,&mutation,context,grant,control);work=turn.work;turn.result});let receipt=result.as_ref().map(|(_,p)|*p).unwrap_or_else(|error|error.retained_progress());cx.consume_fuel(work);cx.consume_receiving_retained(grant,receipt)?;let(post,_)=result?;prepared=post;Ok(())}).unwrap();
  if index==0{foreign.work_turn(grant,256,|cx|{let mut work=0;let result=cx.with_native_receiving::<_,ValueError>(|control|{let turn=apply.advance_replay_controlled(&source,&mutation,context,grant,control);work=turn.work;turn.result});cx.consume_fuel(work);assert_eq!(result.is_err(),row["foreignCallerRefused"].as_bool().unwrap());assert_eq!(work,0);Ok(())}).unwrap();}
  if prepared.is_some(){break}
 }
 assert!(apply.terminal_is_empty());let prepared=prepared.unwrap();assert_eq!(prepared.inverse.as_ref().unwrap().len(),row["inverseRows"].as_u64().unwrap()as usize);assert_eq!(semio_s_artifact_raster_raster_model::layer_name(prepared.snapshot.layers.last().unwrap()),row["renamed"].as_str().unwrap());caller.retire_owned(prepared,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,..test_wallet()});caller.finish();foreign.finish();retire_raster_value_cold(source);retire_raster_value_cold(mutation);
}

#[test]
fn structural_preparation_supplies_genuine_inverse_rows_without_cold_fallback(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/structural-inverse.json")).unwrap();
 let mut fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();fixture["source"]["width"]=1.into();fixture["source"]["height"]=1.into();
 for row in cases["cases"].as_array().unwrap(){
  let mut source=pixel_source(&fixture);if let RasterLayerNode::Pixel{width,height,..}=source.layers.last_mut().unwrap(){*width=None;*height=None;}
  let mutation:RasterMutation=semio_framework_pack_json::from_json_str(&row.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let prepared=prepared_mutation(&source,&mutation);let inverse=prepared.inverse.expect("every document route supplies its retained inverse");assert!(inverse.len()<=mutation.inverse_rows());let mut restored=prepared.snapshot;
  for operation in &inverse{let prepared=prepared_mutation(&restored,operation);retire_raster_value_cold(restored);if let Some(rows)=prepared.inverse{retire_raster_value_cold(rows);}restored=prepared.snapshot;}
  assert_eq!(restored,source,"{}",row["mutation"]);retire_raster_value_cold(restored);retire_raster_value_cold(source);retire_raster_value_cold(mutation);retire_raster_value_cold(inverse);
 }
}

#[test]
fn prepared_pixel_inverses_restore_exact_roots_within_declared_row_bounds(){
    let mut fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();fixture["source"]["width"]=1.into();fixture["source"]["height"]=1.into();
    for row in fixture["inverseCases"].as_array().unwrap(){
        let mut source=pixel_source(&fixture);let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(1,1,vec![0;4]);
        for index in 1..row["assets"].as_u64().unwrap(){let key=format!("unused-{index}");source.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(&key,&image)).unwrap();}
        if row["shared"].as_bool().unwrap(){let mut other=source.layers.last().unwrap().clone();let RasterLayerNode::Pixel{id,..}=&mut other else{unreachable!()};*id="other".into();source.layers.push(other);}
        let mutation=apply_filter::apply_filter("paint",row["filter"].as_str().unwrap(),0.0);assert!(mutation.inverse_rows()>=4);
        let prepared=prepared_mutation(&source,&mutation);let inverse=prepared.inverse.unwrap();assert_eq!(inverse.len(),row["rows"].as_u64().unwrap() as usize);assert!(inverse.len()<=mutation.inverse_rows());let mut restored=prepared.snapshot;
        for operation in &inverse{let prepared=prepared_mutation(&restored,operation);retire_raster_value_cold(restored);if let Some(inverse)=prepared.inverse{retire_raster_value_cold(inverse);}restored=prepared.snapshot;}
        assert_eq!(restored,source,"{}",row["name"]);
        let before=source.assets.get("source").unwrap().local_owner::<semio_s_artifact_raster_raster_model::SemioImageSnapshot>().unwrap();let after=restored.assets.get("source").unwrap().local_owner::<semio_s_artifact_raster_raster_model::SemioImageSnapshot>().unwrap();assert!(std::sync::Arc::ptr_eq(&before,&after));drop((before,after));
        retire_raster_value_cold(restored);retire_raster_value_cold(source);retire_raster_value_cold(mutation);retire_raster_value_cold(inverse);
    }
}

#[test]
fn maximum_stroke_points_are_copied_in_bounded_pages() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let source=pixel_source(&fixture);
    let point_count=fixture["strokePreparation"]["points"].as_u64().unwrap() as usize;
    assert_eq!(point_count,paint_stroke::RASTER_STROKE_MAXIMUM_POINTS);
    let mut mutation=pixel_route("paint-stroke");
    let RasterMutation::PaintStroke(stroke)=&mut mutation else {unreachable!()};
    stroke.points=(0..point_count).map(|index|paint_stroke::RasterStrokePoint{x:index as f64%1024.0+0.5,y:0.5}).collect();
    let mut work=RasterPixelComputeWork::new();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,..test_wallet()};
    let mut copied_points=0;
    for _ in 0..10_000 {
        let demand=work.demands(&source,&mutation,32768).unwrap();
        assert!(demand.copy_bytes<=32768);
        let step=work.advance(&source,&mutation,grant).unwrap();
        assert!(step.progress().fits(grant));
        copied_points=work.copied_points().max(copied_points);
        if copied_points==point_count {break;}
    }
    assert_eq!(copied_points,point_count);
    for _ in 0..200_000 {if work.terminal_is_empty(){break;}work.close_step(exact_grant(work.close_demands(32768).result.unwrap())).unwrap();}
    assert!(work.terminal_is_empty());
    retire_raster_value_cold(source);retire_raster_value_cold(mutation);
}

#[test]
fn maximum_stroke_row_scratch_yields_without_stalling_the_first_pixel() {
    let mut fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();fixture["source"]["width"]=1.into();fixture["source"]["height"]=1.into();
    let source=pixel_source(&fixture);let mut mutation=pixel_route("paint-stroke");
    let RasterMutation::PaintStroke(stroke)=&mut mutation else{unreachable!()};stroke.points=(0..fixture["strokePreparation"]["points"].as_u64().unwrap()).map(|_|paint_stroke::RasterStrokePoint{x:0.5,y:0.5}).collect();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,..test_wallet()};let mut work=RasterPixelComputeWork::new();let mut completed=false;
    for _ in 0..1000{let demand=work.demands(&source,&mutation,32768).unwrap();assert!(demand.copy_bytes<=32768);let step=work.advance(&source,&mutation,grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){let(before,after)=work.take().unwrap();assert_eq!(after.pixels,[255,0,0,255]);drop((before,after));completed=true;break;}}
    assert!(completed);
    for _ in 0..200_000{if work.terminal_is_empty(){break;}work.close_step(exact_grant(work.close_demands(32768).result.unwrap())).unwrap();}assert!(work.terminal_is_empty());
    retire_raster_value_cold(source);retire_raster_value_cold(mutation);
}

#[test]
fn stroke_distance_and_kernel_work_yield_at_actual_primitive_boundaries(){
    use semio_framework_pixels::{RasterImage,editing::{PixelBrush,PixelEditJob,PixelOperation}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let points=fixture["strokePreparation"]["points"].as_u64().unwrap()as usize;
    for operation in [PixelOperation::Stroke(PixelBrush{points:vec![[0.5,0.5];points],size:1.0,opacity:1.0,hardness:1.0,color:[255,0,0,255],erase:false}),PixelOperation::Blur(16)]{
        let source=RasterImage{width:1,height:1,pixels:vec![0,0,0,255]};let mut expected=PixelEditJob::new(source.clone(),operation.clone(),None).unwrap();while !expected.advance(1).unwrap().done{}let expected=expected.result().unwrap().clone();
        let mut job=PixelEditJob::new(source,operation,None).unwrap();job.reserve_scratch().unwrap();
        let stopped=job.advance_controlled(1,32768,17,||true).unwrap();assert_eq!(stopped.completed,0);assert_eq!(job.last_work_units(),0);assert_eq!(job.last_write_bytes(),0);
        let mut turns=0;loop{let progress=job.advance_controlled(1,32768,17,||false).unwrap();assert!(job.last_work_units()<=17);assert!(job.last_write_bytes()<=32768);turns+=1;if progress.done{break;}assert!(turns<1000);}
        assert!(turns>1);assert_eq!(job.result().unwrap(),&expected);
    }
}

#[test]
fn semantic_pixel_output_hashes_and_checks_destination_under_byte_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let source=pixel_source(&fixture);let mutation=pixel_route("apply-filter");
    let mut work=RasterPixelResultWork::new();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,..test_wallet()};
    let mut turns=0;
    loop {
        let step=work.advance(&source,&mutation,grant).unwrap();assert!(step.progress().fits(grant));turns+=1;
        if matches!(step,RetainedCloneStep::Complete(_)){break;}
        assert!(turns<100_000);
    }
    let result=work.take().unwrap();assert!(result.changed);assert!(!result.reuse_destination);
    let image=result.image.as_ref().unwrap();
    assert_eq!(result.key.as_deref(),Some(semio_s_artifact_raster_raster_model::raster_image_content_id("image",image).as_str()));
    assert_eq!(&image.frames[0].rgba8[..4],&[255,255,255,0]);
    assert!(turns>256);
    for _ in 0..200_000{if work.terminal_is_empty(){break;}work.close_step(exact_grant(work.close_demands(32768).result.unwrap())).unwrap();}
    assert!(work.terminal_is_empty());
    retire_raster_value_cold(result);retire_raster_value_cold(source);retire_raster_value_cold(mutation);
}

#[test]
fn canonical_pixel_destination_is_compared_before_reuse() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let mut source=pixel_source(&fixture);let mutation=pixel_route("apply-filter");
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,..test_wallet()};
    let mut first=RasterPixelResultWork::new();
    loop{if matches!(first.advance(&source,&mutation,grant).unwrap(),RetainedCloneStep::Complete(_)){break;}}
    let prepared=first.take().unwrap();let key=prepared.key.as_ref().unwrap().clone();
    for _ in 0..200_000{if first.terminal_is_empty(){break;}first.close_step(exact_grant(first.close_demands(32768).result.unwrap())).unwrap();}
    assert!(first.terminal_is_empty());
    for equal in [true,false]{
        let image=if equal{prepared.image.as_ref().unwrap().clone()}else{semio_s_artifact_raster_raster_model::semio_image_from_rgba8(1,1,vec![0;4])};
        source.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(&key,&image)).unwrap();
        let mut work=RasterPixelResultWork::new();let mut refused=false;let mut completed=false;
        for _ in 0..100_000{
            match work.advance(&source,&mutation,grant){Ok(RetainedCloneStep::Complete(_))=>{let result=work.take().unwrap();assert!(equal);assert!(result.reuse_destination);retire_raster_value_cold(result);completed=true;break;},Ok(step)=>assert!(step.progress().fits(grant)),Err(error)=>{assert!(!equal);assert!(error.to_string().contains("collides"));refused=true;break;}}
        }
        assert_eq!(refused,!equal);
        assert_eq!(completed,equal);
        for _ in 0..200_000{if work.terminal_is_empty(){break;}work.close_step(exact_grant(work.close_demands(32768).result.unwrap())).unwrap();}
        assert!(work.terminal_is_empty());
    }
    retire_raster_value_cold(prepared);retire_raster_value_cold(source);retire_raster_value_cold(mutation);
}

#[test]
fn exclusive_pixel_replacement_uses_the_released_asset_slot() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let mut source=pixel_source(&fixture);
    let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(1,1,vec![0;4]);
    for index in 1..semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY{
        let key=format!("unused-{index}");source.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(&key,&image)).unwrap();
    }
    let mutation=pixel_route("apply-filter");let outcome=::semio_framework_os_kernel::ColdMutation::diff(&mutation,&source);
    assert!(!outcome.messages().iter().any(|message|matches!(message.level,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)));
    let(diff,_)=outcome.into_parts();let next=protocol::apply_diff_cold(&diff,&source).unwrap();
    assert_eq!(next.assets.len(),semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY);assert!(!next.assets.contains_key("source"));assert!(source.assets.contains_key("source"));
    let inverse=mutation.inverse(&source).unwrap();assert_eq!(inverse.len(),4);
    let mut restored=next;
    for step in &inverse{let outcome=::semio_framework_os_kernel::ColdMutation::diff(step,&restored);let(diff,_)=outcome.into_parts();let next=protocol::apply_diff_cold(&diff,&restored).unwrap();protocol::MutationDiff::retire_cold(diff);retire_raster_value_cold(restored);restored=next;}
    assert_eq!(restored,source);
    protocol::MutationDiff::retire_cold(diff);retire_raster_value_cold(restored);retire_raster_value_cold(inverse);retire_raster_value_cold(source);retire_raster_value_cold(mutation);
}

#[test]
fn six_pixel_computations_share_bounded_candidates_and_explicit_cancellation() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,..test_wallet()};
    for route in fixture["routes"].as_array().unwrap(){
        let name=route.as_str().unwrap();let source=pixel_source(&fixture);let mutation=pixel_route(name);
        for cut in [Some(0),Some(32),Some(160),None]{
            let mut work=RasterPixelComputeWork::new();let mut turns=0;
            loop{if cut==Some(turns){break}let step=work.advance(&source,&mutation,grant).unwrap();assert!(step.progress().fits(grant));turns+=1;if matches!(step,RetainedCloneStep::Complete(_)){assert!(cut.is_none());let(before,after)=work.take().unwrap();assert!(before.pixels.iter().all(|byte|*byte==0));assert_eq!(&after.pixels[..4],fixture["firstPixels"][name].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>());drop((before,after));break}assert!(turns<100_000);}
            for _ in 0..200_000{if work.terminal_is_empty(){break}work.close_step(exact_grant(work.close_demands(32768).result.unwrap())).unwrap();}
            assert!(work.terminal_is_empty());
            if cut.is_none(){assert!(turns>=fixture["minimumTurns"].as_u64().unwrap());}
        }
        retire_raster_value_cold(source);retire_raster_value_cold(mutation);
    }
}

#[test]
fn decoded_image_copy_preserves_profiles_metadata_frames_and_replacement_ownership() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔑️content-hash/🧪️tests/🔣️.json")).unwrap();
    let source: semio_s_artifact_raster_raster_model::SemioImageSnapshot = semio_framework_pack_json::from_json_str(&fixture["image"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for replace in [false,true] {
        let mut replacement = replace.then(|| semio_framework_pixels::RasterImage { width:source.width,height:source.height,pixels:vec![255;8] });
        let mut work = RasterImageCopyWork::new();
        let grant = RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:32768,..test_wallet() };
        assert!(work.take().is_none());
        for turn in 0..1000 {
            let starved = work.advance(&source,&mut replacement,RetainedCloneGrant {maximum_items:0,..grant}).unwrap();
            assert_eq!(starved.progress(),RetainedCloneProgress::default());
            let step = work.advance(&source,&mut replacement,grant).unwrap();
            assert!(step.progress().fits(grant));
            if matches!(step,RetainedCloneStep::Complete(_)) { break; }
            assert!(turn < 999);
        }
        let result = work.take().unwrap();
        assert_eq!(result.schema,source.schema);
        assert_eq!(result.metadata,source.metadata);
        assert_eq!(result.icc,source.icc);
        assert_eq!(result.frames[0].delay_ms,source.frames[0].delay_ms);
        if replace { assert!(replacement.is_none());assert_eq!(result.frames[0].rgba8,vec![255;8]); }
        else { assert_eq!(result,source);assert_eq!(semio_s_artifact_raster_raster_model::raster_image_content_id("probe",&result),format!("probe-{}",fixture["digest"].as_str().unwrap())); }
        for _ in 0..1000 {
            if work.terminal_is_empty() { break; }
            work.close_step(exact_grant(work.close_demands(32768).result.unwrap())).unwrap();
        }
        assert!(work.terminal_is_empty());
        drop(result);
    }
}

#[test]
fn pixel_source_preparation_quotes_reservation_copies_and_cancelled_storage() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let source = pixel_source(&fixture);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, ..test_wallet() };
    for cut in fixture["cancelAfterTurns"].as_array().unwrap().iter().map(|value| Some(value.as_u64().unwrap())).chain([None]) {
        let (mut work, receipt) = RasterPixelSourceWork::begin(&source,"paint","pixels",grant).unwrap().unwrap();
        assert!(receipt.fits(grant));
        assert!(work.take_pixels().is_none());
        let mut turns = 0;
        loop {
            if cut == Some(turns) { break; }
            let before = work.progress();
            let empty = work.advance(&source,RetainedCloneGrant { maximum_items:0,..grant }).unwrap();
            assert_eq!(empty.progress(),RetainedCloneProgress::default());
            assert_eq!(work.progress(),before);
            let step = work.advance(&source,grant).unwrap();
            assert!(step.progress().fits(grant));
            turns += 1;
            if matches!(step,RetainedCloneStep::Complete(_)) {
                assert!(cut.is_none());
                let result = work.take_pixels().unwrap();
                assert_eq!(result.pixels.len(),fixture["source"]["width"].as_u64().unwrap() as usize * fixture["source"]["height"].as_u64().unwrap() as usize * 4);
                assert!(result.pixels.iter().all(|byte| *byte == 0));
                drop(result);
                break;
            }
            assert!(turns < 1000);
        }
        for _ in 0..100_000 {
            if work.terminal_is_empty() { break; }
            let demand = work.close_demands(32768).unwrap();
            work.close_step(exact_grant(demand)).result.unwrap();
        }
        assert!(work.terminal_is_empty());
        if cut.is_none() { assert!(turns >= fixture["minimumTurns"].as_u64().unwrap()); }
    }
    retire_raster_value_cold(source);
}

#[test]
fn semantic_pixel_routes_yield_during_large_image_work() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: fixture["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: fixture["grant"]["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: fixture["grant"]["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: fixture["grant"]["maximumDepth"].as_u64().unwrap() as usize };
    for route in fixture["routes"].as_array().unwrap() {
        let source = pixel_source(&fixture);
        let mutation = pixel_route(route.as_str().unwrap());
        let mut work = RasterOneItemApply::new();
        let (pending, empty) = work.advance(&source,&mutation,OperationId(750),semio_framework_job::Generation(1),0,grant).unwrap();
        assert!(pending.is_none());
        assert_eq!(empty,RetainedCloneProgress::default());
        let mut turns = 0;
        let candidate = loop {
            let (candidate, receipt) = work.advance(&source, &mutation, OperationId(750), semio_framework_job::Generation(1), fixture["grant"]["fuel"].as_u64().unwrap(), grant).unwrap();
            assert!(receipt.fits(grant));
            turns += 1;
            assert!(turns < 100_000);
            assert_eq!(source.assets.len(),1);
            if let Some(candidate) = candidate { break candidate; }
        };
        retire_raster_value_cold(candidate);
        retire_raster_value_cold(source);
        retire_raster_value_cold(mutation);
        assert!(work.terminal_is_empty());
        assert!(turns >= fixture["minimumTurns"].as_u64().unwrap(), "{} completed large-image work in {turns} grants", route.as_str().unwrap());
    }
}

#[test]
fn retained_asset_image_hashes_yield_and_removal_inverse_retains_the_exact_root(){
    use semio_s_artifact_raster_raster_mutations_pixels::{add_layer_asset,remove_layer_asset};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/pixel-work.json")).unwrap();let source=empty_raster_document();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,..test_wallet()};
    let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(1024,1024,vec![0;1024*1024*4]);
    let (asset,_)=semio_s_artifact_raster_raster_model::RasterImagePayload::from_owned(image,grant).result.unwrap();
    let add=RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset{asset_id:"large".into(),asset});let mut apply=RasterOneItemApply::new();let mut turns=0;
    let added=loop{let(post,receipt)=apply.advance(&source,&add,OperationId(751),semio_framework_job::Generation(1),1,grant).unwrap();assert!(receipt.fits(grant));turns+=1;assert!(turns<100_000);if let Some(post)=post{break post;}};
    assert!(turns>=fixture["minimumTurns"].as_u64().unwrap());assert_eq!(added.inverse.as_ref().unwrap().len(),1);
    let root=added.snapshot.assets.get("large").unwrap().local_owner::<semio_s_artifact_raster_raster_model::SemioImageSnapshot>().unwrap();
    let remove=RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset{asset_id:"large".into()});let mut apply=RasterOneItemApply::new();
    let removed=loop{let(post,receipt)=apply.advance(&added.snapshot,&remove,OperationId(752),semio_framework_job::Generation(1),1,grant).unwrap();assert!(receipt.fits(grant));if let Some(post)=post{break post;}};
    let RasterMutation::AddLayerAsset(inverse)=&removed.inverse.as_ref().unwrap()[0]else{panic!("removal inverse must restore the image root")};assert!(std::ptr::eq(inverse.asset.image(),root.as_ref()));assert!(!removed.snapshot.assets.contains_key("large"));assert!(added.snapshot.assets.contains_key("large"));
    drop(root);retire_raster_value_cold(removed);retire_raster_value_cold(added);retire_raster_value_cold(add);retire_raster_value_cold(remove);retire_raster_value_cold(source);
}
