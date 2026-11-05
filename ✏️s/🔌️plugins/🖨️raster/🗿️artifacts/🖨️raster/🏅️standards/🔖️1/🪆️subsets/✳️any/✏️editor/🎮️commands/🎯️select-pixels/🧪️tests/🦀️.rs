use super::*;
use semio_framework_value::RetainedCloneGrant;
use semio_s_artifact_raster_raster_model::{RasterLayerNode,RasterTransform,RasterOwnedMap};
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;

fn spans(rows:Vec<RasterSelectionSpan>)->crate::selection::RasterSelectionSpans{let mut caller=OriginalWorkFixture::new();let result=caller.receive_native(rows,wallet());caller.finish();result}
fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T){let mut fixture=OriginalWorkFixture::new();fixture.retire_owned(value,OriginalWorkFixture::stock());fixture.finish();}

fn wallet()->RetainedCloneGrant{RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:128*1024*1024,maximum_release_bytes:128*1024*1024,maximum_depth:128}}
fn document(width:u32,height:u32)->RasterSnapshot{RasterSnapshot {schema:"raster.document".into(),id:"selection".into(),title:None,layers:vec![RasterLayerNode::Pixel {id:"p".into(),name:"Pixels".into(),visible:true,locked:false,opacity:1.0,blend_mode:"normal".into(),transform:RasterTransform::default(),mask:None,width:Some(width),height:Some(height),image_key:None}],assets:RasterOwnedMap::new()}}
fn config(width:u32,height:u32)->RasterConfig{RasterConfig {pixel_selection:Some(RasterPixelSelection {layer_id:"p".into(),target:"pixels".into(),width,height,spans:spans(vec![RasterSelectionSpan {start:0,length:width*height,coverage:128}])}),..Default::default()}}
fn request(method:&str,mode:&str,path:&str,width:u32,height:u32)->SelectPixels{SelectPixels {layer_id:"p".into(),target:"pixels".into(),expected_image_key:None,width,height,method:method.into(),mode:mode.into(),path:path.into()}}
fn close(work:&mut SelectPixelsWork){work.begin_closing();for _ in 0..262144{if work.empty(){return;}let quote=work.close_demands(32768).unwrap();assert!(quote.copy_bytes<=wallet().maximum_copy_bytes);let turn=work.close_one(wallet());let step=turn.result.unwrap();let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};assert!(progress.fits(wallet()));}panic!("selection owner did not close");}
fn coverage(selection:&RasterPixelSelection)->Vec<u8>{let mut pixels=vec![0;selection.width as usize*selection.height as usize];for span in selection.spans.iter(){pixels[span.start as usize..(span.start+span.length) as usize].fill(span.coverage as u8);}pixels}

#[test]
fn every_selection_method_matches_neutral_vectors_and_image_library_outputs(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("fixture.json")).unwrap();let width=fixture["width"].as_u64().unwrap() as u32;let height=fixture["height"].as_u64().unwrap() as u32;
    let base=document(width,height);let config=config(width,height);
    for case in fixture["cases"].as_array().unwrap(){
        let method=case["method"].as_str().unwrap();let mut work=SelectPixelsWork {request:Some(request(method,case["mode"].as_str().unwrap(),&case["path"].to_string(),width,height)),..Default::default()};
        assert!(work.take_result().is_err());let mut done=false;for _ in 0..1000{if work.advance(&base,&config,wallet()).unwrap().0{done=true;break;}assert!(work.take_result().is_err());}assert!(done,"{method}");
        let result=work.take_result().unwrap();if case["spans"].is_null(){assert!(result.is_none());}else{
            let result=result.unwrap();let expected:Vec<[u32;3]>=serde_json::from_value(case["spans"].clone()).unwrap();assert_eq!(result.spans.iter().map(|span|[span.start,span.length,span.coverage]).collect::<Vec<_>>(),expected,"{case}");
            let actual=coverage(&result);
            if method=="rectangle"&&case["mode"]=="replace"{let mut oracle=image::GrayImage::from_pixel(width,height,image::Luma([0]));image::imageops::overlay(&mut oracle,&image::GrayImage::from_pixel(4,4,image::Luma([255])),0,0);assert_eq!(actual,oracle.into_raw());}
            if matches!(method,"all"|"wand"){assert_eq!(actual,image::GrayImage::from_pixel(width,height,image::Luma([255])).into_raw());}
            if method=="invert"{let mut oracle=image::GrayImage::from_pixel(width,height,image::Luma([128]));image::imageops::invert(&mut oracle);assert_eq!(actual,oracle.into_raw());}
            retire(result);
        }assert!(work.take_result().is_err());close(&mut work);
    }
    retire(config);semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(base);
}

#[test]
fn cancellation_during_shape_wand_and_combination_keeps_prior_coverage(){
    let base=document(256,256);let config=config(256,256);let before=config.clone();
    for (method,phase,path) in [("rectangle",3,"[[0,0],[256,256]]"),("wand",3,"[[0,0]]"),("rectangle",5,"[[0,0],[256,256]]")]{
        let mut work=SelectPixelsWork {request:Some(request(method,"intersect",path,256,256)),..Default::default()};
        for _ in 0..2048{work.advance(&base,&config,wallet()).unwrap();if work.stage==phase{break;}}
        assert_eq!(work.stage,phase);work.advance(&base,&config,wallet()).unwrap();assert!(work.take_result().is_err());close(&mut work);assert_eq!(config,before);assert!(work.take_result().is_err());
    }retire(before);retire(config);semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(base);
}

#[test]
fn selection_refuses_changed_input_roots_invalid_paths_and_stale_image_keys(){
    let base=document(8,8);let other=document(8,8);let config=config(8,8);let other_config=config.clone();
    for change_config in [false,true]{let mut work=SelectPixelsWork {request:Some(request("rectangle","replace","[[0,0],[4,4]]",8,8)),..Default::default()};work.advance(&base,&config,wallet()).unwrap();assert!(if change_config{work.advance(&base,&other_config,wallet())}else{work.advance(&other,&config,wallet())}.is_err());close(&mut work);}
    for path in ["[[0,0],]","[[0,0],,[4,4]]","[[0,0] [4,4]]","[[0,0],[1e999,4]]"]{let mut work=SelectPixelsWork {request:Some(request("rectangle","replace",path,8,8)),..Default::default()};for _ in 0..100{work.advance(&base,&config,wallet()).unwrap();if work.stage==1{break;}}assert_eq!(work.stage,1);assert!(work.advance(&base,&config,wallet()).is_err());close(&mut work);}
    let mut payload=request("wand","replace","[[0,0]]",8,8);payload.expected_image_key=Some("stale".into());let mut work=SelectPixelsWork {request:Some(payload),..Default::default()};let mut refused=false;for _ in 0..100{if work.advance(&base,&config,wallet()).is_err(){refused=true;break;}}assert!(refused);close(&mut work);
    retire(config);retire(other_config);semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(base);semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(other);
}

#[test]
fn long_lasso_keeps_4096_points_and_rejects_a_4097th(){
    let base=document(8,8);let config=config(8,8);
    for count in [4096,4097]{let path=serde_json::to_string(&(0..count).map(|index|[f64::from(index%8),f64::from((index/8)%8)]).collect::<Vec<_>>()).unwrap();let mut work=SelectPixelsWork {request:Some(request("lasso","replace",&path,8,8)),..Default::default()};work.advance(&base,&config,wallet()).unwrap();let mut result=Ok(false);for _ in 0..300{result=work.path_step(1024,&mut||true);assert!(work.cpu_units<=1024);if !matches!(result,Ok(false)){break;}}assert_eq!(result.is_ok(),count==4096);if count==4096{assert_eq!(work.points.len(),4096);}close(&mut work);}
    retire(config);semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(base);
}

#[test]
fn whitespace_in_large_paths_yields_between_bounded_original_byte_pages(){
    let path=format!("{}[{}[0,0],{}[4,4]{}]{}"," ".repeat(131072)," ".repeat(65536)," ".repeat(65536)," ".repeat(65536)," ".repeat(65536));
    let mut work=SelectPixelsWork{request:Some(request("rectangle","replace",&path,8,8)),..Default::default()};
    let mut done=false;for _ in 0..1024{let before=work.cursor;done=work.path_step(1024,&mut||true).unwrap();assert!(work.cursor-before<=1024+128);assert!(work.cpu_units<=1024);if done{break;}}
    assert!(done);assert_eq!(work.points,[[0.0,0.0],[4.0,4.0]]);close(&mut work);
}

#[test]
fn escaped_original_path_and_metadata_fit_the_declared_raw_wire_budget(){
    let policy:serde_json::Value=serde_json::from_str(include_str!("execution.json")).unwrap();
    for whitespace in ['\n','\t']{
        let tail="[[0,0],[4,4]]";let path=whitespace.to_string().repeat(policy["maximumPathBytes"].as_u64().unwrap()as usize-tail.len())+tail;
        let value=serde_json::json!({"layerId":"\u{1}".repeat(256),"target":"pixels","expectedImageKey":"\u{1}".repeat(256),"width":8,"height":8,"method":"rectangle","mode":"replace","path":path,"windowId":"\u{1}".repeat(policy["maximumOwnedFieldBytes"].as_u64().unwrap()as usize)});
        let wire=serde_json::to_vec(&value).unwrap();assert!(wire.len()<=policy["maximumRawBytes"].as_u64().unwrap()as usize);assert_eq!(serde_json::from_slice::<serde_json::Value>(&wire).unwrap(),value);
        assert_eq!(serde_json::from_str::<Vec<[f64;2]>>(value["path"].as_str().unwrap()).unwrap(),[[0.0,0.0],[4.0,4.0]]);
    }
    for whitespace in ['\u{b}','\u{c}']{
        let path=format!("{whitespace}[[0,0],[4,4]]");let mut work=SelectPixelsWork{request:Some(request("rectangle","replace",&path,8,8)),..Default::default()};assert!(work.path_step(1024,&mut||true).is_err());close(&mut work);
    }
}

#[test]
fn original_selection_arguments_move_the_path_backing_and_refuse_extra_fields(){
    use semio_framework_value::{DslValue,FromValue,ToValue};
    let mut input=request("rectangle","replace","[[0,0],[4,4]]",8,8).to_value();
    let DslValue::Object(fields)=&mut input else{panic!("selection record")};let path=fields.iter_mut().find(|(key,_)|key=="path").unwrap();let mut text=" ".repeat(1_048_576-13);text.push_str("[[0,0],[4,4]]");path.1=DslValue::String(text);let pointer=path.1.as_str().unwrap().as_ptr();
    let decoded=SelectPixels::from_value(input).unwrap();assert_eq!(decoded.path.as_ptr(),pointer);assert_eq!(decoded.path.len(),1_048_576);
    let mut invalid=request("rectangle","replace","[]",8,8).to_value();let DslValue::Object(fields)=&mut invalid else{panic!("selection record")};fields.push(("spans".into(),DslValue::Array(Vec::new())));assert!(SelectPixels::validate_original_value(&invalid).is_err());
    let mut duplicate=request("rectangle","replace","[]",8,8).to_value();let DslValue::Object(fields)=&mut duplicate else{panic!("selection record")};fields.retain(|(key,_)|key!="expectedImageKey");fields.push(("path".into(),DslValue::String("[]".into())));assert!(SelectPixels::validate_original_value(&duplicate).is_err());
    let mut scoped=request("rectangle","replace","[[0,0],[4,4]]",8,8).to_value();let DslValue::Object(fields)=&mut scoped else{panic!("selection record")};fields.push(("windowId".into(),DslValue::String("composite-split-2".into())));let pointer=fields.iter().find(|(key,_)|key=="path").unwrap().1.as_str().unwrap().as_ptr();assert!(SelectPixels::validate_original_value(&scoped).is_err());
    let mut original=Some(scoped);let received=SelectPixels::from_original_action_value(&mut original).unwrap();assert_eq!(received.path.as_ptr(),pointer);let DslValue::Object(fields)=original.as_ref().unwrap()else{panic!("original owner")};assert!(fields.iter().filter(|(key,_)|key!="windowId").all(|(_,value)|matches!(value,DslValue::Null)));assert_eq!(fields.iter().find(|(key,_)|key=="windowId").unwrap().1.as_str(),Some("composite-split-2"));
}
