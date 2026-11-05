//! 🎭️ Retained masks consume authoritative selection and publish raw image children atomically.
use semio_s_artifact_raster_raster_model::{RasterLayerNode,RasterLayerMask,RasterTransform};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_editor::commands::mask_from_selection::*;
use semio_s_artifact_raster_raster_editor::{RasterConfig,RasterConfigMutation};
use semio_s_artifact_raster_raster_editor::selection::RasterPixelSelection;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand};
use protocol::{ColdMutation,Mutation};
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan;
use semio_framework_value::ToValue;
use semio_s_artifact_raster_raster_editor::selection::RasterSelectionSpans;

#[test]
fn mask_action_moves_business_strings_and_retains_reserved_window_metadata() {
    use semio_framework_value::DslValue;
    let mut args=Some(semio_framework_pack_json::from_json_str::<DslValue>(&fixture()["scopedAction"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let DslValue::Object(entries)=args.as_ref().unwrap()else{panic!("fixture requires an object")};
    let pointer=|name:&str|entries.iter().find_map(|(key,value)|if key==name{if let DslValue::String(text)=value{Some(text.as_ptr())}else{None}}else{None}).unwrap();let layer=pointer("layerId");let image=pointer("expectedImageKey");
    let command=MaskFromSelection::from_original_action_value(&mut args).unwrap();assert_eq!(command.layer_id.as_ptr(),layer);assert_eq!(command.expected_image_key.as_ref().unwrap().as_ptr(),image);
    let DslValue::Object(entries)=args.as_ref().unwrap()else{panic!("original envelope must remain owned")};
    assert!(entries.iter().any(|(key,value)|key=="windowId"&&matches!(value,DslValue::String(text) if text=="raster-composite")));
    assert!(entries.iter().filter(|(key,_)|key!="windowId").all(|(_,value)|matches!(value,DslValue::Null)));
}

#[test]
fn invalid_mask_action_preserves_the_original_argument_owner() {
    use semio_framework_value::DslValue;
    for extra in [("selection",DslValue::Array(Vec::new())),("windowId",DslValue::String("duplicate".into()))]{
        let mut value=semio_framework_pack_json::from_json_str::<DslValue>(&fixture()["scopedAction"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let DslValue::Object(entries)=&mut value else{unreachable!()};entries.push((extra.0.into(),extra.1));
        let mut args=Some(value);let before=args.clone();assert!(MaskFromSelection::from_original_action_value(&mut args).is_err());assert_eq!(args,before);
    }
}

#[test]
fn oversized_mask_business_fields_refuse_before_owned_work_or_action_handoff() {
    use semio_framework_value::DslValue;
    let maximum=fixture()["maximumFieldUtf8Bytes"].as_u64().unwrap() as usize;let (document,config,command)=source(1,1);
    for field in ["layerId","expectedImageKey"]{
        let mut invalid=command.clone();if field=="layerId"{invalid.layer_id="x".repeat(maximum+1);}else{invalid.expected_image_key=Some("x".repeat(maximum+1));}
        assert!(MaskSelectionWork::begin_demands(&invalid,&document,&config).is_err());
        let mut value=semio_framework_pack_json::from_json_str::<DslValue>(&fixture()["scopedAction"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let DslValue::Object(entries)=&mut value else{unreachable!()};let (_,cell)=entries.iter_mut().find(|(key,_)|key==field).unwrap();*cell=DslValue::String("x".repeat(maximum+1));let mut args=Some(value);let before=args.clone();
        assert!(MaskFromSelection::from_original_action_value(&mut args).is_err());assert_eq!(args,before);
    }
    retire(document);
}

#[derive(Clone)]
struct ConfigFixture(Option<RasterConfig>);
impl std::ops::Deref for ConfigFixture {type Target=RasterConfig;fn deref(&self)->&RasterConfig{self.0.as_ref().unwrap()}}
impl std::ops::DerefMut for ConfigFixture {fn deref_mut(&mut self)->&mut RasterConfig{self.0.as_mut().unwrap()}}
impl Drop for ConfigFixture {fn drop(&mut self){if let Some(config)=self.0.take(){let mut caller=OriginalWorkFixture::new();caller.retire_owned(config,OriginalWorkFixture::stock());caller.finish();}}}
fn spans(rows:Vec<RasterSelectionSpan>)->RasterSelectionSpans{let mut caller=OriginalWorkFixture::new();let output=caller.receive_native(rows.to_value(),OriginalWorkFixture::stock());caller.finish();output}
fn decoded_spans(value:&serde_json::Value)->Result<RasterSelectionSpans,semio_framework_job::OriginalTurnFailure>{let input=semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject)?;let mut caller=OriginalWorkFixture::new();let output=caller.try_receive_native(input,OriginalWorkFixture::stock());caller.finish();output}

fn fixture()->serde_json::Value {serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn source(width:u32,height:u32)->(RasterSnapshot,ConfigFixture,MaskFromSelection) {
    let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();
    document.layers.push(semio_s_artifact_raster_raster_model::create_pixel_layer("Paint",width,height));
    let layer_id=semio_s_artifact_raster_raster_model::layer_node_id(&document.layers[0]).to_owned();
    let mut config=RasterConfig::default();
    config.pixel_selection=Some(RasterPixelSelection {layer_id:layer_id.clone(),target:"pixels".into(),width,height,spans:Default::default()});
    (document,ConfigFixture(Some(config)),MaskFromSelection {layer_id,expected_image_key:None})
}
fn grant(demand:RetirementDemand)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(32768),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)}}
fn close(work:&mut MaskSelectionWork) {
    for turn in 0..100_000 {
        if work.terminal_is_empty(){return;}
        let quoted=grant(work.close_demands(32768).unwrap());
        let empty=work.close_step(RetainedCloneGrant::default());assert_eq!(empty.work,0);assert_eq!(empty.result.unwrap().progress(),Default::default());
        let step=work.close_step(quoted).result.unwrap();assert!(step.progress().fits(quoted));assert!(turn<99_999);
    }
    assert!(work.terminal_is_empty());
}
fn finish(command:&MaskFromSelection,document:&RasterSnapshot,config:&RasterConfig)->semio_framework_plugin::Emit<semio_s_artifact_raster_raster_mutations::RasterMutation,RasterConfigMutation> {
    let quoted=grant(MaskSelectionWork::begin_demands(command,document,config).unwrap());
    let (mut work,receipt)=MaskSelectionWork::begin(command,document,config,quoted).unwrap().unwrap();assert!(receipt.fits(quoted));
    for turn in 0..100_000 {
        let quoted=grant(work.demands(32768).unwrap());
        let empty=work.advance(document,config,RetainedCloneGrant::default()).unwrap();assert_eq!(empty.progress(),Default::default());
        let step=work.advance(document,config,quoted).unwrap();assert!(step.progress().fits(quoted));
        if matches!(step,RetainedCloneStep::Complete(_)){let emit=work.take_emit().unwrap();close(&mut work);return emit;}
        assert!(work.take_emit().is_none());assert!(turn<99_999);
    }
    panic!("mask work failed to yield a terminal result")
}
fn retire(document:RasterSnapshot){semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);}
fn refused_work(command:&MaskFromSelection,document:&RasterSnapshot,config:&RasterConfig) {
    let quoted=grant(MaskSelectionWork::begin_demands(command,document,config).unwrap());let (mut work,_)=MaskSelectionWork::begin(command,document,config,quoted).unwrap().unwrap();
    let mut refused=false;
    for turn in 0..100_000 {let quoted=grant(work.demands(32768).unwrap());match work.advance(document,config,quoted){Ok(step)=>{assert!(step.progress().fits(quoted));assert!(!matches!(step,RetainedCloneStep::Complete(_)));},Err(_)=>{refused=true;break;}}assert!(work.take_emit().is_none());assert!(turn<99_999);}
    assert!(refused);assert!(work.take_emit().is_none());close(&mut work);
}
fn refused_guard(command:&MaskFromSelection,document:&RasterSnapshot,config:&RasterConfig) {
    if MaskSelectionWork::begin_demands(command,document,config).is_ok(){refused_work(command,document,config);}
}

#[test]
fn selection_mask_preserves_soft_coverage_and_round_trips_history() {
    let fixture=fixture();let (mut document,mut config,command)=source(3,2);
    config.pixel_selection.as_mut().unwrap().spans=decoded_spans(&fixture["selection"]).unwrap();
    let before=document.clone();let emit=finish(&command,&document,&config);let mut inverses=Vec::new();
    for mutation in &emit.artifact_mutations {
        inverses.push(mutation.inverse(&document).unwrap());
        let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));
    }
    let RasterLayerNode::Pixel {mask:Some(mask),image_key,..}=&document.layers[0] else{panic!("mask must be attached")};assert!(image_key.is_none());
    let image=document.assets.get(mask.image_key.as_ref().unwrap()).unwrap().local_owner::<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot>().unwrap();
    assert_eq!((image.width,image.height),(3,2));assert_eq!(serde_json::to_value(&image.frames[0].rgba8).unwrap(),fixture["expectedRgba"]);
    for inverse in inverses.into_iter().rev().flatten(){let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&inverse).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));inverse.retire_cold();}
    assert_eq!(document,before);for mutation in emit.artifact_mutations{mutation.retire_cold();}retire(document);retire(before);
}

#[test]
fn persistent_spans_preserve_sparse_adjacent_and_paged_prefix_coverage() {
    for case in fixture()["spanRegressions"].as_array().unwrap() {
        let width=case["width"].as_u64().unwrap() as u32;let height=case["height"].as_u64().unwrap() as u32;
        let (mut document,mut config,command)=source(width,height);
        config.pixel_selection.as_mut().unwrap().spans=if let Some(pattern)=case.get("pattern") {
            let cycle=pattern["coverageCycle"].as_array().unwrap();
            spans((0..pattern["count"].as_u64().unwrap()).map(|index|RasterSelectionSpan {start:(pattern["start"].as_u64().unwrap()+index*pattern["stride"].as_u64().unwrap()) as u32,length:pattern["length"].as_u64().unwrap() as u32,coverage:cycle[index as usize%cycle.len()].as_u64().unwrap() as u32}).collect())
        }else{decoded_spans(&case["spans"]).unwrap()};
        let emit=finish(&command,&document,&config);
        for mutation in emit.artifact_mutations {let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));mutation.retire_cold();}
        let RasterLayerNode::Pixel {mask:Some(mask),..}=&document.layers[0] else{panic!("regression mask must be attached")};
        let image=document.assets.get(mask.image_key.as_ref().unwrap()).unwrap().local_owner::<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot>().unwrap();
        let expected:Vec<u8>=case["expectedAlpha"].as_array().unwrap().iter().flat_map(|value|[255,255,255,value.as_u64().unwrap() as u8]).collect();
        assert_eq!(image.frames[0].rgba8,expected,"{}",case["name"]);retire(document);
    }
}

#[test]
fn mask_requires_exact_completed_pixel_selection_and_source() {
    let fixture=fixture();let (document,config,command)=source(3,2);
    for case in fixture["guards"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()) {
        let mut config=config.clone();let mut command=command.clone();
        match case {"missing"=>config.pixel_selection=None,"layer"=>config.pixel_selection.as_mut().unwrap().layer_id="other".into(),"target"=>config.pixel_selection.as_mut().unwrap().target="mask".into(),"width"=>config.pixel_selection.as_mut().unwrap().width=2,"height"=>config.pixel_selection.as_mut().unwrap().height=1,_=>command.expected_image_key=Some("stale".into())}
        refused_guard(&command,&document,&config);
    }
    for (index,invalid) in fixture["invalid"].as_array().unwrap().iter().enumerate(){
        let decoded=decoded_spans(invalid);
        if index==0{let spans=decoded.expect("ordered valid coverage remains independent of image extent");assert_eq!(spans.maximum_end(),7);let mut config=config.clone();config.pixel_selection.as_mut().unwrap().spans=spans;let before=document.clone();refused_work(&command,&document,&config);assert_eq!(document,before);retire(before);}else{assert!(decoded.is_err(),"invalid coverage must refuse at original receiving");}
    }
    let emit=finish(&command,&document,&config);assert!(!emit.artifact_mutations.is_empty());for mutation in emit.artifact_mutations{mutation.retire_cold();}retire(document);
}

#[test]
fn selection_mask_initialization_and_cancelled_storage_are_granted() {
    let (document,mut config,command)=source(256,256);config.pixel_selection.as_mut().unwrap().spans=spans(vec![RasterSelectionSpan {start:0,length:65536,coverage:255}]);
    assert!(MaskSelectionWork::begin(&command,&document,&config,RetainedCloneGrant::default()).unwrap().is_none());
    for cut in fixture()["cancelAfterTurns"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()) {
        let quoted=grant(MaskSelectionWork::begin_demands(&command,&document,&config).unwrap());let (mut work,receipt)=MaskSelectionWork::begin(&command,&document,&config,quoted).unwrap().unwrap();assert!(receipt.fits(quoted));
        for _ in 0..cut {let quoted=grant(work.demands(32768).unwrap());let step=work.advance(&document,&config,quoted).unwrap();assert!(step.progress().fits(quoted));assert!(work.take_emit().is_none());}
        close(&mut work);assert!(work.take_emit().is_none());assert!(document.assets.is_empty());assert!(matches!(&document.layers[0],RasterLayerNode::Pixel {mask:None,image_key:None,..}));
    }
    retire(document);
}

#[test]
fn mask_source_or_selection_revision_changes_refuse_unpublished_work() {
    for change_config in [false,true] {
        let (document,config,command)=source(3,2);let quoted=grant(MaskSelectionWork::begin_demands(&command,&document,&config).unwrap());let (mut work,_)=MaskSelectionWork::begin(&command,&document,&config,quoted).unwrap().unwrap();
        let mut changed_document=document.clone();let mut changed_config=config.clone();
        if change_config{changed_config.pixel_selection.as_mut().unwrap().spans=spans(vec![RasterSelectionSpan {start:0,length:1,coverage:128}]);}else if let RasterLayerNode::Pixel {locked,..}=&mut changed_document.layers[0]{*locked=true;}
        let quoted=grant(work.demands(32768).unwrap());assert!(work.advance(if change_config{&document}else{&changed_document},if change_config{&changed_config}else{&config},quoted).is_err());assert!(work.take_emit().is_none());close(&mut work);retire(changed_document);retire(document);
    }
}

#[test]
fn protected_pixels_refuse_before_preparation() {
    let (mut document,config,command)=source(3,2);if let RasterLayerNode::Pixel {locked,..}=&mut document.layers[0]{*locked=true;}
    refused_guard(&command,&document,&config);retire(document);
    let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for id in ["locked-pixel","inherited-pixel"] {let mut config=config.clone();config.pixel_selection.as_mut().unwrap().layer_id=id.into();refused_guard(&MaskFromSelection {layer_id:id.into(),expected_image_key:None},&document,&config);}
    retire(document);
}

fn assets(document:&mut RasterSnapshot,count:usize,shared:bool) {
    let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(3,2,vec![255;24]);
    for index in 0..count {let key=format!("retained-{index}");document.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(&key,&image)).unwrap();}
    if let RasterLayerNode::Pixel {mask,..}=&mut document.layers[0]{*mask=Some(RasterLayerMask {enabled:true,linked:true,invert:false,width:Some(3),height:Some(2),image_key:Some("retained-0".into()),transform:RasterTransform::default()});}
    if shared {let mut layer=semio_s_artifact_raster_raster_model::create_pixel_layer("Other",3,2);if let RasterLayerNode::Pixel {image_key,..}=&mut layer{*image_key=Some("retained-0".into());}document.layers.push(layer);}
}

#[test]
fn mask_replacement_preserves_referenced_children_and_reuses_full_asset_capacity() {
    for (count,shared) in [(64,false),(63,true)] {
        let (mut document,config,command)=source(3,2);assets(&mut document,count,shared);
        let before=document.clone();let emit=finish(&command,&document,&config);let mut inverses=Vec::new();
        for mutation in &emit.artifact_mutations {
            inverses.push(mutation.inverse(&document).unwrap());
            let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));assert!(document.assets.len()<=64);
        }
        assert_eq!(document.assets.len(),64);assert_eq!(document.assets.contains_key("retained-0"),shared);
        if shared {let other=semio_s_artifact_raster_raster_model::layer_node_id(&document.layers[1]);assert!(matches!(semio_s_artifact_raster_raster_model::find_layer(&document.layers,other),Some(RasterLayerNode::Pixel {image_key:Some(key),..}) if key=="retained-0"));}
        for inverse in inverses.into_iter().rev().flatten(){let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&inverse).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));inverse.retire_cold();}
        assert_eq!(document,before);for mutation in emit.artifact_mutations{mutation.retire_cold();}retire(document);retire(before);
    }
}

#[test]
fn full_asset_capacity_with_shared_prior_mask_refuses_without_publication() {
    let (mut document,config,command)=source(3,2);assets(&mut document,64,true);let before=document.clone();
    refused_work(&command,&document,&config);assert_eq!(document,before);retire(document);retire(before);
}

#[test]
fn full_asset_capacity_reuses_an_existing_mask_destination() {
    let (mut document,config,command)=source(3,2);let first=finish(&command,&document,&config);
    for mutation in first.artifact_mutations {let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));mutation.retire_cold();}
    if let RasterLayerNode::Pixel {mask:Some(mask),..}=&mut document.layers[0]{mask.enabled=false;}
    let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(3,2,vec![255;24]);
    for index in 1..64 {let key=format!("spare-{index}");document.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(&key,&image)).unwrap();}
    let emit=finish(&command,&document,&config);
    for mutation in emit.artifact_mutations {let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));mutation.retire_cold();}
    assert_eq!(document.assets.len(),64);assert!(matches!(&document.layers[0],RasterLayerNode::Pixel {mask:Some(mask),..} if mask.enabled));retire(document);
}

#[test]
fn an_existing_destination_must_own_the_canonical_mask_content() {
    let (mut document,config,command)=source(3,2);
    let expected=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(3,2,[255,255,255,0].repeat(6));
    let key=semio_s_artifact_raster_raster_model::raster_image_content_id("image",&expected);
    let wrong=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(3,2,vec![255;24]);
    document.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(&key,&wrong)).unwrap();
    let before=document.clone();refused_work(&command,&document,&config);assert_eq!(document,before);retire(document);retire(before);
}

fn nested(mut layer:RasterLayerNode,depth:usize,locked:Option<usize>)->RasterLayerNode {
    let suffix=semio_s_artifact_raster_raster_model::layer_node_id(&layer).to_owned();
    for index in (0..depth).rev(){layer=RasterLayerNode::Group {id:format!("group-{suffix}-{index}"),name:"Group".into(),visible:true,locked:locked==Some(index),opacity:1.0,blend_mode:"normal".into(),transform:RasterTransform::default(),mask:None,children:vec![layer]};}layer
}

fn wide_prefix(fixture:&serde_json::Value)->(RasterSnapshot,ConfigFixture,MaskFromSelection) {
    let (mut document,mut config,mut command)=source(3,2);let mut target=document.layers.pop().unwrap();
    let bytes=fixture["identifierBytes"].as_u64().unwrap() as usize;let prefix="x".repeat(bytes-8);command.layer_id=format!("{prefix}t0000000");config.pixel_selection.as_mut().unwrap().layer_id=command.layer_id.clone();
    if let RasterLayerNode::Pixel {id,..}=&mut target{*id=command.layer_id.clone();}
    for index in 0..fixture["prefixLayers"].as_u64().unwrap(){let mut layer=semio_s_artifact_raster_raster_model::create_pixel_layer("Prefix",3,2);if let RasterLayerNode::Pixel {id,..}=&mut layer{*id=format!("{prefix}d{index:07}");}document.layers.push(layer);}
    document.layers.push(target);(document,config,command)
}

#[test]
fn mask_target_lookup_yields_across_wide_equal_length_identifier_prefixes() {
    let fixture=fixture();let discovery=&fixture["discovery"];let (document,config,command)=wide_prefix(discovery);
    let quoted=grant(MaskSelectionWork::begin_demands(&command,&document,&config).unwrap());let (mut work,receipt)=MaskSelectionWork::begin(&command,&document,&config,quoted).unwrap().unwrap();assert!(receipt.fits(quoted));
    let page=discovery["copyPageBytes"].as_u64().unwrap() as usize;let turns=discovery["prefixLayers"].as_u64().unwrap() as usize*(discovery["identifierBytes"].as_u64().unwrap() as usize).div_ceil(page/2);
    for _ in 0..turns {let mut quoted=grant(work.demands(page).unwrap());quoted.maximum_copy_bytes=page;let empty=work.advance(&document,&config,RetainedCloneGrant::default()).unwrap();assert_eq!(empty.progress(),Default::default());let step=work.advance(&document,&config,quoted).unwrap();assert!(step.progress().fits(quoted));assert!(!matches!(step,RetainedCloneStep::Complete(_)));assert!(work.take_emit().is_none());}
    close(&mut work);let emit=finish(&command,&document,&config);assert!(!emit.artifact_mutations.is_empty());
    for mutation in emit.artifact_mutations {if let semio_s_artifact_raster_raster_mutations::RasterMutation::ChangeLayerMask(change)=&mutation{assert_eq!(change.layer_id,command.layer_id);}mutation.retire_cold();}retire(document);
}

#[test]
fn cancelling_mask_lookup_retires_only_private_work() {
    let fixture=fixture();let discovery=&fixture["discovery"];
    for cut in discovery["cancelAfterTurns"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()) {
        let (document,config,command)=wide_prefix(discovery);let before=document.clone();let quoted=grant(MaskSelectionWork::begin_demands(&command,&document,&config).unwrap());let (mut work,_)=MaskSelectionWork::begin(&command,&document,&config,quoted).unwrap().unwrap();
        for _ in 0..cut {let page=discovery["copyPageBytes"].as_u64().unwrap() as usize;let mut quoted=grant(work.demands(page).unwrap());quoted.maximum_copy_bytes=page;let step=work.advance(&document,&config,quoted).unwrap();assert!(step.progress().fits(quoted));assert!(work.take_emit().is_none());}
        close(&mut work);assert!(work.take_emit().is_none());assert_eq!(document,before);retire(document);retire(before);
    }
}

#[test]
fn mask_discovery_preserves_inherited_locks_at_every_depth() {
    let fixture=fixture();let discovery=&fixture["discovery"];let depth=discovery["maximumDepth"].as_u64().unwrap() as usize;
    for locked in discovery["inheritedLockDepths"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize) {
        let (mut document,config,command)=source(3,2);let layer=document.layers.pop().unwrap();document.layers.push(nested(layer,depth,Some(locked)));let before=document.clone();refused_work(&command,&document,&config);assert_eq!(document,before);retire(document);retire(before);
    }
    let (mut document,config,command)=source(3,2);let layer=document.layers.pop().unwrap();document.layers.push(nested(semio_s_artifact_raster_raster_model::create_pixel_layer("Other",3,2),1,Some(0)));document.layers.push(nested(layer,depth,None));
    let emit=finish(&command,&document,&config);assert!(!emit.artifact_mutations.is_empty());for mutation in emit.artifact_mutations{mutation.retire_cold();}retire(document);
}

#[test]
fn mask_discovery_refuses_excessive_depth_before_publication() {
    let fixture=fixture();let depth=fixture["discovery"]["refusedDepth"].as_u64().unwrap() as usize;
    let (mut document,config,command)=source(3,2);let layer=document.layers.pop().unwrap();document.layers.push(nested(layer,depth,None));let before=document.clone();refused_work(&command,&document,&config);assert_eq!(document,before);retire(document);retire(before);
}
