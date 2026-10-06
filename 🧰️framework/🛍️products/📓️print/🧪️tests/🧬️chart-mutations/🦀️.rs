//! 🧪️ Language-neutral chart replay vectors and real inference registry dispatch.
use crate::*;
use semio_framework_value::{DslValue,FromValue,ToValue};
use protocol::{Mutation,MutationDiff,DiffAlgebra,Inference};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧫️fixtures/🧬️chart-mutations/🔣️.json")).unwrap() }
fn snapshot() -> ChartSnapshot { ChartSnapshot::from_value(DslValue::from(fixture()["snapshot"].clone())).unwrap() }

#[test]
fn mutations_replay_inverse_and_reject_atomically() {
    let source = fixture();
    let base = snapshot();
    let mut current = base.clone();
    let mut combined = ChartDiff::default();
    for value in source["mutations"].as_array().unwrap() {
        let mutation = ChangeChartValue::from_value(DslValue::from(value.clone())).unwrap();
        let outcome = mutation.diff(&current);
        assert!(outcome.messages().is_empty());
        current = outcome.diff().apply(&current).unwrap();
        combined.absorb(outcome.diff().clone());
    }
    assert_eq!(current.chart["width"].as_u64(), Some(120));
    assert_eq!(current.chart["language"].as_str(), Some("de"));
    assert_eq!(combined.apply(&base).unwrap(), current);
    assert_eq!(combined.inverse(&base).apply(&current).unwrap(), base);
    for value in source["rejections"].as_array().unwrap() {
        let mutation = ChangeChartValue::from_value(DslValue::from(value.clone())).unwrap();
        let outcome = mutation.diff(&base);
        assert!(!outcome.messages().is_empty());
        assert!(outcome.diff().is_empty());
        assert_eq!(outcome.diff().apply(&base).unwrap(), base);
    }
    assert!(ChangeChartValue::DESCRIPTORS[0].validate().is_ok());
}

#[test]
fn array_removal_inverse_restores_shifted_elements() {
    let mut base = snapshot();
    if let DslValue::Object(entries) = &mut base.chart {
        let layers = entries.iter_mut().find(|(name,_)|name=="layers").unwrap();
        if let DslValue::Array(items) = &mut layers.1 { items.extend([items[0].clone(),items[0].clone()]); }
    }
    let mutation = ChangeChartValue { path: vec!["layers".into(), "0".into()], value: None };
    let diff = mutation.diff(&base).diff().clone();
    let next = diff.apply(&base).unwrap();
    assert_eq!(diff.inverse(&base).apply(&next).unwrap(), base);
    assert_eq!(mutation.inverse(&base).unwrap()[0].diff(&next).diff().apply(&next).unwrap(), base);
}

#[test]
fn native_registry_dispatch_is_deterministic_and_controlled() {
    use semio_framework_plugin::*;
    let base = snapshot();
    let payload = protocol::pack_rt::encode_wire_value(&base.to_value());
    let budgets = WireArtifactInferenceBudget { work_units: 100, allocation_bytes: 1_000_000, recursion_depth: 64 };
    let request = ArtifactInferenceExecutionRequest { policy: &[], budgets: &budgets, cancellation_id: "chart-test", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Bypass, canonical_payload: &payload, dependencies: &[] };
    let mut registry = ArtifactInferenceServiceRegistry::new();
    registry.register(chart_inference_service()).unwrap();
    register_chart_artifact().unwrap();
    register_chart_artifact().unwrap();
    let published=semio_framework_plugin::app::artifact_inference_service(CHART_ARTIFACT_KIND,"framework.print.chart.inference").unwrap().unwrap();
    assert_eq!(published.infer(&request).unwrap().canonical_payload,registry.infer(CHART_ARTIFACT_KIND,"framework.print.chart.inference",&request).unwrap().canonical_payload);
    let first = registry.infer(CHART_ARTIFACT_KIND, "framework.print.chart.inference", &request).unwrap();
    let second = registry.infer(CHART_ARTIFACT_KIND, "framework.print.chart.inference", &request).unwrap();
    assert_eq!(first.canonical_payload, second.canonical_payload);
    let decoded = ChartInference::from_value(protocol::pack_rt::decode_wire_value(&first.canonical_payload).unwrap()).unwrap();
    assert_eq!(decoded, ChartInference::infer(&base).unwrap());
    assert!(decoded.tikz.contains("\\SemioVizPlot[data=semio-print-layer-0,mark=point"));
    let mut progress = Vec::new();
    assert!(execute_chart_inference_controlled(&request, &mut |work| { progress.push(work); Ok(()) }).is_ok());
    assert!(progress.windows(2).all(|pair| pair[0] <= pair[1]));
    assert!(execute_chart_inference_controlled(&request, &mut |work| { if work >= 1 { Err(ArtifactInferenceExecutionError::new("test.cancelled", "cancelled")) } else { Ok(()) } }).is_err());
    let invalid_payload=protocol::pack_rt::encode_wire_value(&ChartSnapshot::default().to_value());
    let invalid_request=ArtifactInferenceExecutionRequest{canonical_payload:&invalid_payload,..request};
    let invalid=published.infer(&invalid_request).unwrap();
    let decoded=ChartInference::from_value(protocol::pack_rt::decode_wire_value(&invalid.canonical_payload).unwrap()).unwrap();
    assert!(!invalid.complete);assert!(!decoded.complete);assert!(decoded.tikz.is_empty());assert_eq!(decoded.diagnostics[0].path,"chart");
}

#[test]
fn missing_language_is_diagnostic_data() {
    let inferred = ChartInference::infer(&ChartSnapshot::default()).unwrap();
    assert!(inferred.tikz.is_empty());
    assert_eq!(inferred.diagnostics.len(), 1);
    assert!(!inferred.complete);
}

#[test]
fn shared_result_schema_accepts_valid_and_invalid_native_outputs(){
    let validator=semio_framework_schema_validator::OwnedJsonSchemaValidator::compile_with_documents(include_str!("../../🧬️schema/💡️inferences/🔣️.json"),&[]).unwrap();
    for base in [snapshot(),ChartSnapshot::default()]{let result=ChartInference::infer(&base).unwrap();validator.validate_json(&semio_framework_pack_json::to_json_string(&result)).unwrap();assert_eq!(ChartInference::from_value(result.to_value()).unwrap(),result);}
}

#[test]
fn css_paints_preserve_named_rgb_hsl_and_fractional_alpha(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️chart-mutations/🎨️paint.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){let paint=inferences::paint::parse(case["input"].as_str().unwrap()).unwrap();assert_eq!(paint.hex,case["hex"].as_str().unwrap());assert!((paint.alpha-case["alpha"].as_f64().unwrap()).abs()<1e-12);if paint.alpha<1.0{assert!(paint.declaration().contains("\\SemioVizPaintAlpha"));}}
    for input in fixture["invalid"].as_array().unwrap(){assert!(inferences::paint::parse(input.as_str().unwrap()).is_err());}
    let contract:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/📸️snapshot/📊️chart/🎨️color/🔣️.json")).unwrap();
    for (name,hex) in contract["x-semio-named-colors"].as_object().unwrap(){assert_eq!(inferences::paint::parse(name).unwrap().hex,hex.as_str().unwrap());}
    let mut authored=crate::tests::fixture()["snapshot"].clone();
    authored["chart"]["layers"][0]["encodings"]["fill"]=serde_json::json!({"value":"rgba(255,0,0,0.125)"});
    authored["chart"]["annotations"]=serde_json::json!([{"kind":"text","x":1,"y":2,"text":{"en":"green","de":"grün"},"options":{"fill":"rgba(255,0,0,0.125)","opacity":0.5}}]);
    let inferred=ChartInference::infer(&ChartSnapshot::from_value(DslValue::from(authored)).unwrap()).unwrap();
    assert!(inferred.complete,"{:?}",inferred.diagnostics);
    assert!(inferred.tikz.contains("\\SemioVizPaintAlpha{semio-print-color-FF0000-A0p125}{0.125}"));
    assert!(inferred.tikz.contains("text opacity=0.0625"));
}

#[test]
fn explicit_null_is_distinct_from_omitted_value_in_wire_mutations(){
    let null=ChangeChartValue{path:vec!["layers".into(),"0".into(),"encodings".into(),"fill".into(),"value".into()],value:Some(DslValue::Null)};
    let deleted=ChangeChartValue{path:null.path.clone(),value:None};
    assert_eq!(ChangeChartValue::from_value(null.to_value()).unwrap(),null);
    assert_eq!(ChangeChartValue::from_value(deleted.to_value()).unwrap(),deleted);
    assert_ne!(null.to_value(),deleted.to_value());
    assert_eq!(ChartInference::default(),ChartInference::infer(&ChartSnapshot::default()).unwrap());
}

#[test]
fn derived_names_do_not_overwrite_authored_tables_columns_or_text(){
    let source=fixture();
    let snapshot=ChartSnapshot::from_value(DslValue::from(source["collision"]["snapshot"].clone())).unwrap();
    let inference=ChartInference::infer(&snapshot).unwrap();
    assert!(inference.diagnostics.is_empty(),"{:?}",inference.diagnostics);
    let table=source["collision"]["preparedTable"].as_str().unwrap();
    let column=source["collision"]["constantColumn"].as_str().unwrap();
    assert!(inference.tikz.contains(&format!("\\SemioVizTable{{{table}}}")));
    assert!(inference.tikz.contains(&format!("fill={{column={column}}}")));
    assert!(inference.tikz.contains("text={column=semio-constant-fill}"));
    assert!(inference.tikz.contains("{\\#00ff00}"));
    assert!(inference.tikz.contains("{semio-print-color-FF00AA}"));
}

#[test]
fn every_catalogue_kind_is_reachable_through_snapshot_inference() {
    let kinds = inferences::catalog().unwrap()["kinds"].as_array().unwrap();
    assert!(kinds.len() > 1700);
    let chart = DslValue::object([("width".into(),DslValue::uint(80)),("height".into(),DslValue::uint(40)),("language".into(),DslValue::String("en".into())),("layers".into(),DslValue::Array(vec![])),("presets".into(),DslValue::Array(kinds.iter().map(|entry|DslValue::object([("kind".into(),entry["slug"].clone())])).collect()))]);
    let inferred = ChartInference::infer(&ChartSnapshot{chart}).unwrap();
    assert!(inferred.diagnostics.is_empty(),"{:?}",inferred.diagnostics);
    assert_eq!(inferred.tikz.matches("\\SemioVizChart{").count(), kinds.len());
}

#[test]
fn native_chart_source_can_be_compiled_by_the_print_toolchain(){
    let Some(path)=std::env::var_os("PRINT_NATIVE_CHART_TIKZ") else{return;};
    let authored=include_str!("../../🧫️fixtures/🧬️chart-mutations/📊️native-grammar.json");
    let snapshot=semio_framework_pack_json::from_json_str::<ChartSnapshot>(authored,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let inference=ChartInference::infer(&snapshot).unwrap();
    assert!(inference.diagnostics.is_empty(),"{:?}",inference.diagnostics);
    std::fs::write(path,inference.tikz).unwrap();
}

#[test]
fn typed_rows_preserve_null_strings_booleans_and_color_fallback_alpha(){
    let snapshot=semio_framework_pack_json::from_json_str::<ChartSnapshot>(include_str!("../../🧫️fixtures/🧬️chart-mutations/📊️native-grammar.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let inference=ChartInference::infer(&snapshot).unwrap();
    assert!(inference.diagnostics.is_empty(),"{:?}",inference.diagnostics);
    assert!(inference.tikz.contains("{\\SemioVizNull{}}"));
    assert!(inference.tikz.contains("{\\SemioVizBoolean{true}}"));
    assert!(inference.tikz.contains("{\\SemioVizBoolean{false}}"));
    assert!(inference.tikz.contains("\\SemioVizRow{native-typed}{{\\SemioVizString{}},{1}}"));
    assert!(inference.tikz.contains("unknown={semio-print-color-0080FF-A0p35}"));
    assert!(inference.tikz.contains("\\SemioVizPaintAlpha{semio-print-color-0080FF-A0p35}{0.35}"));
}
#[path = "../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;

#[test]
fn paged_chart_original_operation_keeps_exact_variants_policy_and_source_owner(){
    use protocol::os_spr::operation_bytes::{OwnedOperationBytes,OperationBytePreparation,OperationByteMeasurement,OperationByteCloseStep};
    use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,Number};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️chart-mutations/📦️operation-pages.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=ChangeChartValue{path:serde_json::from_value(fixture["path"].clone()).unwrap(),value:Some(DslValue::Array(vec![DslValue::String(payload),DslValue::uint(u64::MAX),DslValue::int(i64::MIN),DslValue::float(-0.0)]))};
    let expected=protocol::OpBinary::encode_op(&operation).unwrap();
    let header:Vec<u8>=serde_json::from_value(fixture["header"].clone()).unwrap();assert!(expected.starts_with(&header));
    let metadata=fixture["metadataBytes"].as_u64().unwrap()as usize;
    let allocation=fixture["allocationBytes"].as_u64().unwrap()as usize;
    let items=fixture["maximumCloseItems"].as_u64().unwrap()as usize;
    let bytes=fixture["maximumCloseBytes"].as_u64().unwrap()as usize;
    let close=|owner:&mut OwnedOperationBytes|{let mut released=0;for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){match owner.close_one(items,bytes).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=bytes);released+=released_bytes;}}}assert!(owner.terminal_is_empty());assert_eq!(owner.allocated_bytes(),0);released};
    let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);
    let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);
    encoding.scoped_maximum(metadata,|control|Ok::<_,semio_framework_value::ValueError>(operation.encode_op_into(&options,&mut measure,control))).unwrap().unwrap();
    assert_eq!(measure.exact_length().unwrap(),expected.len());assert!(encoding.owned_bytes()<=metadata);
    let mut preparation=OperationBytePreparation::try_new(expected.len(),allocation).unwrap();
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){preparation.fund_one(items,bytes,&mut encoding).unwrap();if preparation.is_funded(){break;}}
    assert!(preparation.is_funded());let paid=encoding.owned_bytes();let backing=preparation.allocated_bytes();
    encoding.scoped_maximum(paid+metadata,|control|Ok::<_,semio_framework_value::ValueError>(operation.encode_op_into(&options,&mut preparation,control))).unwrap().unwrap();
    assert_eq!(preparation.allocated_bytes(),backing);assert!(encoding.owned_bytes()-paid<=metadata);
    let mut source=preparation.take_ready().unwrap();assert!(source.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(allocation,&mut allow);
    let mut allow=|_|true;let mut canonical=NativeEncodeControl::new(metadata,&mut allow);
    let decoded=ChangeChartValue::decode_op_span(protocol::ByteSpan::from_source(&source),&Default::default(),&options,&mut decoding,&mut canonical).unwrap();
    assert_eq!(decoded,operation);
    let Some(DslValue::Array(values))=decoded.value else{panic!("original Chart array lost")};
    assert!(matches!(values[1],DslValue::Number(Number::UInt(u64::MAX))));assert!(matches!(values[2],DslValue::Number(Number::Int(i64::MIN))));
    let DslValue::Number(Number::Float(number))=values[3] else{panic!("exact Chart float variant lost")};assert_eq!(number.to_bits(),(-0.0f64).to_bits());
    assert_eq!(source.close_one(0,bytes).unwrap(),OperationByteCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!(source.len(),expected.len());assert!(close(&mut source)>=expected.len());
    let kind=|error:protocol::ProtocolError|match error{protocol::ProtocolError::Pack(protocol::PackError::Refusal(refusal))=>refusal.kind(),error=>panic!("expected genuine typed Pack refusal, got {error:?}")};
    let mut short=options.clone();short.limits.max_file_len-=1;
    let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();
    let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);
    assert_eq!(kind(operation.encode_op_into(&short,&mut prefix,&mut encoding).unwrap_err()),semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix);
    let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();
    let mut cancel=|work:semio_framework_value::native_encoding::NativeEncodeProgress|work.completed<1024;let mut encoding=NativeEncodeControl::new(allocation,&mut cancel);
    assert_eq!(kind(operation.encode_op_into(&options,&mut prefix,&mut encoding).unwrap_err()),semio_framework_value::ValueRefusalKind::Canceled);
    assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix);
    println!("[DEBUG] Original Chart8194 text source preserves [1,1], UInt/Int/-0float and whole-frame caller policy under512 metadata with exact refusal prefix and fixed4096 terminal page return");
}
