//! 📸️ Canonical point buffers and owned parent SQLite capability baselines.
use super::RemodelingSnapshot;
fn laws()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
#[test]
fn sqlite_snapshot_remodeling_actual_factory_owns_parent_capability(){let codec=store::ArtifactCodec::bare::<RemodelingSnapshot,crate::RemodelingMutation>(crate::REMODELING_DOCUMENT_SCHEMA);assert!(codec.snapshot_sqlite.is_some(),"Remodeling parent has no authored relational capability");}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_remodeling_inline_clouds_are_native_tagged_values(){
    use semio_framework::io::io_mechanism::Serializer;
    use semio_framework::io_schema::IoPayload;
    use semio_framework_value::ToValue;
    use crate::standards::v1::subsets::any::io::{export::serializers::artifacts::json::v_rfc8259::any::RemodelingIntoJson,import::deserializers::artifacts::json::v_rfc8259::any::from_json_text};
    let fixture=laws();
    for (ordinal,buffer) in fixture["inlineBuffers"].as_array().unwrap().iter().enumerate(){
        let sparse:crate::SparseCloud=serde_json::from_value(serde_json::json!({"points":buffer,"colors":null})).expect("declared finite input samples are a typed tagged buffer");
        let mut snapshot=crate::default_remodeling_scene();snapshot.results.sparse=Some(sparse);
        let payload=RemodelingIntoJson::serialize(&snapshot, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.unwrap().value;let IoPayload::Text(text)=payload else{panic!("JSON text expected")};
        let independent:serde_json::Value=serde_json::from_str(&text).unwrap();assert_eq!(independent["results"]["sparse"]["points"],fixture["inlineBufferCanonical"][ordinal]);
        let restored=from_json_text(&text).unwrap();assert_eq!(restored.to_value(),snapshot.to_value());
    }
}
#[test]
fn sqlite_snapshot_remodeling_durable_chunks_are_native_literal_octets(){let expected=serde_json::json!({"kind":"sparse","mime":null,"width":0,"height":0,"chunks":laws()["byteChunks"]});let actual:crate::RemodelingDurableArtifact=serde_json::from_value(expected.clone()).expect("durable chunks carry literal octets");assert_eq!(serde_json::to_value(actual).unwrap(),expected);}
#[test]
fn sqlite_snapshot_remodeling_binary32_samples_agree_with_independent_words(){for raw in laws()["binary32Words"].as_array().unwrap(){let bits=u32::from_str_radix(raw.as_str().unwrap(),16).unwrap();let samples=[f32::from_bits(bits),f32::from_bits(bits)];let actual=crate::Float32Buffer::from_f32_slice(&samples).to_f32_vec();assert_eq!(actual.iter().map(|value|value.to_bits()).collect::<Vec<_>>(),vec![bits,bits]);let lexical=serde_json::from_str::<u32>(&bits.to_string()).unwrap();assert_eq!(lexical,bits);}}

#[test]
fn sqlite_snapshot_remodeling_native_text_binary32_words_match_source_neutral_spelling(){
    use store::ArtifactDsl;
    let fixture=laws();
    for row in fixture["nativeTextWordCases"].as_array().unwrap(){
        let bits=u32::from_str_radix(row["binary32Bits"].as_str().unwrap(),16).unwrap();let mut snapshot=crate::default_remodeling_scene();snapshot.params.ingest.min_sharpness=f32::from_bits(bits);
        let text=snapshot.print_dsl();assert!(text.contains(&format!("nan64_{}",row["widenedBinary64Bits"].as_str().unwrap())),"actual native Text must preserve its exact widened IEEE word");
        let restored=RemodelingSnapshot::parse_dsl(&text).unwrap();assert_eq!(restored.params.ingest.min_sharpness.to_bits(),bits);
    }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_remodeling_declared_json_preserves_every_owned_ieee_word(){
    use semio_framework::io::io_mechanism::Serializer;
    use semio_framework::io_schema::IoPayload;
    use crate::standards::v1::subsets::any::io::{export::serializers::artifacts::json::v_rfc8259::any::RemodelingIntoJson,import::deserializers::artifacts::json::v_rfc8259::any::from_json_text};
    let fixture=laws();
    for raw in fixture["binary64Words"].as_array().unwrap(){
        let raw=raw.as_str().unwrap();let bits=u64::from_str_radix(raw,16).unwrap();
        let mut expected=crate::default_remodeling_scene();expected.streams.push(crate::MediaStream{sync_offset_ms:f64::from_bits(bits),..Default::default()});
        let payload=RemodelingIntoJson::serialize(&expected, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.unwrap().value;
        let IoPayload::Text(text)=payload else{panic!("declared JSON serializer must return text")};
        let wire:serde_json::Value=serde_json::from_str(&text).unwrap();assert_eq!(wire["streams"][0]["syncOffsetMs"],serde_json::json!({"bits":raw}));
        let restored=from_json_text(&text).unwrap();assert_eq!(restored.streams[0].sync_offset_ms.to_bits(),bits);
    }
    for raw in fixture["binary32Words"].as_array().unwrap(){
        let raw=raw.as_str().unwrap();let bits=u32::from_str_radix(raw,16).unwrap();
        let mut expected=crate::default_remodeling_scene();expected.results.sparse=Some(crate::SparseCloud{points:crate::Float32Buffer::Inline{values:vec![f32::from_bits(bits)]},colors:None});
        let payload=RemodelingIntoJson::serialize(&expected, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.unwrap().value;
        let IoPayload::Text(text)=payload else{panic!("declared JSON serializer must return text")};
        let wire:serde_json::Value=serde_json::from_str(&text).unwrap();assert_eq!(wire["results"]["sparse"]["points"]["values"][0],serde_json::json!({"bits":raw}));
        let restored=from_json_text(&text).unwrap();let samples=restored.results.sparse.unwrap().points.to_f32_vec();assert_eq!(samples[0].to_bits(),bits);
    }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_remodeling_declared_json_covers_every_owned_scalar_path(){
    use semio_framework::io::io_mechanism::Serializer;
    use semio_framework::io_schema::IoPayload;
    use crate::standards::v1::subsets::any::io::{export::serializers::artifacts::json::v_rfc8259::any::RemodelingIntoJson,import::deserializers::artifacts::json::v_rfc8259::any::from_json_text};
    let fixture=laws();
    for (ordinal,raw) in fixture["binary32Words"].as_array().unwrap().iter().enumerate(){
        let bits32=u32::from_str_radix(raw.as_str().unwrap(),16).unwrap();let bits64=u64::from_str_radix(fixture["binary64Words"][ordinal%fixture["binary64Words"].as_array().unwrap().len()].as_str().unwrap(),16).unwrap();let f=f32::from_bits(bits32);let d=f64::from_bits(bits64);
        let mut snapshot=crate::default_remodeling_scene();
        snapshot.streams.push(crate::MediaStream{sync_offset_ms:d,fps_hint:d,frames:vec![crate::FrameRef{timestamp_ms:d,..Default::default()}],source:Some(crate::VideoSource{duration_ms:d,..Default::default()}),..Default::default()});
        snapshot.calibration.cameras.push(crate::CameraCalibration{fx:d,fy:d,cx:d,cy:d,skew:d,distortion:[f;5],rms_reprojection_px:Some(f),..Default::default()});
        snapshot.calibration.rig.push(crate::RigExtrinsic{rotation_wxyz:[f;4],translation_m:[f;3],..Default::default()});
        snapshot.gcps.push(crate::GroundControlPoint{world_position:[d;3],observations:vec![crate::GcpObservation{pixel:[f;2],..Default::default()}],..Default::default()});
        snapshot.params.ingest.min_sharpness=f;snapshot.params.feature.edge_threshold=f;snapshot.params.matching.ratio_test=f;snapshot.params.sfm.ransac_threshold_px=f;snapshot.params.sfm.huber_delta_px=f;snapshot.params.dense.confidence_threshold=f;snapshot.params.mesh.tsdf_voxel_size_mm=f;snapshot.params.mesh.tsdf_truncation_mm=f;snapshot.params.motion.min_track_quality=f;
        snapshot.params.geo.origin_lon=Some(d);snapshot.params.geo.origin_lat=Some(d);snapshot.params.geo.origin_alt=Some(d);snapshot.params.geo.gsd_m=f;snapshot.params.geo.dsm_cell_m=f;snapshot.params.geo.dtm_filter_radius_m=f;
        let report=crate::WatertightReportSnapshot{euler_characteristic:i64::MIN,genus:Some(i64::MAX),signed_volume:d,..Default::default()};
        snapshot.results.sparse=Some(crate::SparseCloud{points:crate::Float32Buffer::Inline{values:vec![f]},colors:None});
        snapshot.results.dense=Some(crate::DenseCloud{positions:crate::Float32Buffer::Content{content_id:"literal\0世界".into(),chunk_count:u64::MAX},confidence:Some(crate::Float32Buffer::Inline{values:vec![f]}),..Default::default()});
        snapshot.results.mesh.watertight=Some(report.clone());snapshot.results.trajectory=Some(crate::CameraTrajectory{poses:vec![crate::CameraPosePreview{rotation_wxyz:[f;4],translation:[f;3],..Default::default()}]});
        snapshot.results.tracks.push(crate::MotionTrackSummary{mean_speed_m_s:f,..Default::default()});
        snapshot.results.qc=Some(crate::QcReportSnapshot{reprojection_rms_px:d,gcp_checkpoint_rmse:Some(d),watertight:Some(report),mean_track_length:f,registered_frame_ratio:f,dense_coverage_ratio:f,..Default::default()});
        let first=RemodelingIntoJson::serialize(&snapshot, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.unwrap().value;let IoPayload::Text(text)=first else{panic!("JSON text expected")};let independent:serde_json::Value=serde_json::from_str(&text).unwrap();
        assert_eq!(independent["results"]["tracks"][0]["meanSpeedMS"],serde_json::json!({"bits":format!("{bits32:08x}")}));assert_eq!(independent["results"]["mesh"]["watertight"]["eulerCharacteristic"],i64::MIN.to_string());assert_eq!(independent["results"]["mesh"]["watertight"]["genus"],i64::MAX.to_string());assert_eq!(independent["results"]["dense"]["positions"]["chunkCount"],u64::MAX.to_string());
        let restored=from_json_text(&text).unwrap();let second=RemodelingIntoJson::serialize(&restored, &semio_framework::io::io_mechanism::ArchiveChildren::empty()).await.unwrap().value;let IoPayload::Text(second)=second else{panic!("JSON text expected")};assert_eq!(second,text,"every actual owned scalar path must retain its exact word");
        let mut malformed=independent.clone();malformed["params"]["ingest"]["minSharpness"]=serde_json::json!({"bits":format!("{bits32:08x}"),"other":0});assert!(from_json_text(&malformed.to_string()).is_err());
        let mut malformed=independent;malformed["results"]["dense"]["positions"]["chunkCount"]=serde_json::json!(u64::MAX);assert!(from_json_text(&malformed.to_string()).is_err());
    }
}
