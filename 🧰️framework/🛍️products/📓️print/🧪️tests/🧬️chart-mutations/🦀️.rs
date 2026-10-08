//! 🧪️ Language-neutral chart replay vectors and real inference registry dispatch.
use crate::io::text::inferences::ChartTextOutput;

#[test]
fn semantic_chart_inference_owns_values_and_io_owns_native_text(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚪️inference-native-ownership/🔣️.json")).unwrap();
    let source=ChartSnapshot::from_value(DslValue::from(&fixture["snapshot"])).unwrap();
    let inference=crate::ChartInference::infer(&source).unwrap();
    assert!(inference.complete);assert_eq!(inference.chart.as_ref(),Some(&source));
    let semantic=serde_json::Value::from(inference.to_value());
    let native=serde_json::Value::from(ChartTextOutput::render(&source).unwrap().to_value());
    let keys=|value:&serde_json::Value|value.as_object().unwrap().keys().cloned().collect::<Vec<_>>();
    assert_eq!(keys(&semantic),fixture["semanticKeys"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_string()).collect::<Vec<_>>());
    assert_eq!(keys(&native),fixture["nativeKeys"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_string()).collect::<Vec<_>>());
    assert!(native["tikz"].as_str().unwrap().contains("\\begin{VizFigure}"));
    println!("[DEBUG] typed chart inference/native text boundary oracle=serde_json");
}
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
        current = protocol::apply_diff(outcome.diff(), &current).unwrap();
        combined.absorb(outcome.diff().clone());
    }
    assert_eq!(current.chart["width"].as_u64(), Some(120));
    assert_eq!(current.chart["language"].as_str(), Some("de"));
    assert_eq!(protocol::apply_diff(&combined, &base).unwrap(), current);
    assert_eq!(protocol::apply_diff(&combined.inverse(&base), &current).unwrap(), base);
    for value in source["rejections"].as_array().unwrap() {
        let mutation = ChangeChartValue::from_value(DslValue::from(value.clone())).unwrap();
        let outcome = mutation.diff(&base);
        assert!(!outcome.messages().is_empty());
        assert!(outcome.diff().is_empty());
        assert_eq!(protocol::apply_diff(outcome.diff(), &base).unwrap(), base);
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
    let next = protocol::apply_diff(&diff, &base).unwrap();
    assert_eq!(protocol::apply_diff(&diff.inverse(&base), &next).unwrap(), base);
    let mut restored = next.clone();
    for step in mutation.inverse(&base).unwrap().iter().rev() { restored = protocol::apply_diff(step.diff(&restored).diff(), &restored).unwrap(); }
    assert_eq!(restored, base);
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
    let decoded = ChartTextOutput::from_value(protocol::pack_rt::decode_wire_value(&first.canonical_payload).unwrap()).unwrap();
    assert_eq!(decoded, ChartTextOutput::render(&base).unwrap());
    assert!(decoded.tikz.contains("\\SemioVizPlot[data=semio-print-layer-0,mark=point"));
    let mut progress = Vec::new();
    assert!(execute_chart_inference_controlled(&request, &mut |work| { progress.push(work); Ok(()) }).is_ok());
    assert!(progress.windows(2).all(|pair| pair[0] <= pair[1]));
    assert!(execute_chart_inference_controlled(&request, &mut |work| { if work >= 1 { Err(ArtifactInferenceExecutionError::new("test.cancelled", "cancelled")) } else { Ok(()) } }).is_err());
    let invalid_payload=protocol::pack_rt::encode_wire_value(&ChartSnapshot::default().to_value());
    let invalid_request=ArtifactInferenceExecutionRequest{canonical_payload:&invalid_payload,..request};
    let invalid=published.infer(&invalid_request).unwrap();
    let decoded=ChartTextOutput::from_value(protocol::pack_rt::decode_wire_value(&invalid.canonical_payload).unwrap()).unwrap();
    assert!(!invalid.complete);assert!(!decoded.complete);assert!(decoded.tikz.is_empty());assert_eq!(decoded.diagnostics[0].path,"chart");
}

#[test]
fn missing_language_is_diagnostic_data() {
    let inferred = ChartTextOutput::render(&ChartSnapshot::default()).unwrap();
    assert!(inferred.tikz.is_empty());
    assert_eq!(inferred.diagnostics.len(), 1);
    assert!(!inferred.complete);
}

#[test]
fn shared_result_schema_accepts_valid_and_invalid_native_outputs(){
    let validator=semio_framework_schema_validator::OwnedJsonSchemaValidator::compile_with_documents(include_str!("../../🚪️io/📝️text/💡️inferences/🔣️.json"),&[]).unwrap();
    for base in [snapshot(),ChartSnapshot::default()]{let result=ChartTextOutput::render(&base).unwrap();validator.validate_json(&semio_framework_pack_json::to_json_string(&result)).unwrap();assert_eq!(ChartTextOutput::from_value(result.to_value()).unwrap(),result);}
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
    let inferred=ChartTextOutput::render(&ChartSnapshot::from_value(DslValue::from(authored)).unwrap()).unwrap();
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
    assert_eq!(ChartTextOutput::default(),ChartTextOutput::render(&ChartSnapshot::default()).unwrap());
}

#[test]
fn derived_names_do_not_overwrite_authored_tables_columns_or_text(){
    let source=fixture();
    let snapshot=ChartSnapshot::from_value(DslValue::from(source["collision"]["snapshot"].clone())).unwrap();
    let inference=ChartTextOutput::render(&snapshot).unwrap();
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
    let inferred = ChartTextOutput::render(&ChartSnapshot{chart}).unwrap();
    assert!(inferred.diagnostics.is_empty(),"{:?}",inferred.diagnostics);
    assert_eq!(inferred.tikz.matches("\\SemioVizChart{").count(), kinds.len());
}

#[test]
fn native_chart_source_can_be_compiled_by_the_print_toolchain(){
    let Some(path)=std::env::var_os("PRINT_NATIVE_CHART_TIKZ") else{return;};
    let authored=include_str!("../../🧫️fixtures/🧬️chart-mutations/📊️native-grammar.json");
    let snapshot=semio_framework_pack_json::from_json_str::<ChartSnapshot>(authored,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let inference=ChartTextOutput::render(&snapshot).unwrap();
    assert!(inference.diagnostics.is_empty(),"{:?}",inference.diagnostics);
    std::fs::write(path,inference.tikz).unwrap();
}

/// 🪡️ Neutral authored option mutations emit actual canonical Rust grammar for native compilation.
#[test]
fn canonical_customization_emits_neutral_mutation_vectors(){
 let source=fixture();for(case,valid)in source["optionSyntax"]["cases"].as_array().unwrap().iter().map(|case|(case,true)).chain(source["optionSyntax"]["rejections"].as_array().unwrap().iter().map(|case|(case,false))){
  let before=ChartSnapshot::from_value(DslValue::from(case["snapshot"].clone())).unwrap();let mut current=before.clone();
  for value in case["mutations"].as_array().unwrap(){let mutation=ChangeChartValue::from_value(DslValue::from(value.clone())).unwrap();let outcome=mutation.diff(&current);assert!(outcome.messages().is_empty(),"{:?}",outcome.messages());let next=protocol::apply_diff(outcome.diff(),&current).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&current),&next).unwrap(),current);current=next;}
  let inference=ChartTextOutput::render(&current).unwrap();assert_eq!(inference.complete,valid,"{}: {:?}",case["id"],inference.diagnostics);if !valid{assert!(!inference.diagnostics.is_empty());continue;}assert!(inference.diagnostics.is_empty(),"{:?}",inference.diagnostics);
  if let Some(root)=std::env::var_os("PRINT_NATIVE_OPTION_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("{}.tex",case["id"].as_str().unwrap())),inference.tikz).unwrap();}
  assert_eq!(before,ChartSnapshot::from_value(DslValue::from(case["snapshot"].clone())).unwrap());
 }
 eprintln!("[DEBUG] Canonical Rust authored option mutations inferred; emitted source awaits actual native compiler adjudication");
}
#[test]
fn typed_rows_preserve_null_strings_booleans_and_color_fallback_alpha(){
    let snapshot=semio_framework_pack_json::from_json_str::<ChartSnapshot>(include_str!("../../🧫️fixtures/🧬️chart-mutations/📊️native-grammar.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let inference=ChartTextOutput::render(&snapshot).unwrap();
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
/// 🛃️ Neutral descriptor primitives and grammar alternatives survive actual mutation replay before native compilation.
#[test]
fn canonical_option_descriptor_admission(){
 let source=fixture();let mut failures=Vec::new();
 for case in source["optionSyntax"]["admission"].as_array().unwrap(){let before=ChartSnapshot::from_value(DslValue::from(case["snapshot"].clone())).unwrap();let mut current=before.clone();
  for value in case["mutations"].as_array().unwrap(){let mutation=ChangeChartValue::from_value(DslValue::from(value.clone())).unwrap();let outcome=mutation.diff(&current);assert!(outcome.messages().is_empty(),"{:?}",outcome.messages());let next=protocol::apply_diff(outcome.diff(),&current).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&current),&next).unwrap(),current);current=next;}
  let result=ChartTextOutput::render(&current).unwrap();let valid=case["valid"].as_bool().unwrap();if result.complete!=valid{failures.push(format!("{}: actual {}, expected {}, {:?}",case["id"],result.complete,valid,result.diagnostics));}
  if valid&&result.complete{if let Some(root)=std::env::var_os("PRINT_NATIVE_OPTION_TIKZ"){let root=std::path::Path::new(&root).join("admission");std::fs::create_dir_all(&root).unwrap();std::fs::write(root.join(format!("{}.tex",case["id"].as_str().unwrap())),result.tikz).unwrap();}}
  assert_eq!(before,ChartSnapshot::from_value(DslValue::from(case["snapshot"].clone())).unwrap());
 }
 assert!(failures.is_empty(),"{}",failures.join("\n"));eprintln!("[DEBUG] Rust declared option admission: 58 neutral en/de actual mutation/replay/inference controls PASS; valid emission requires actual native compiler");
}
/// 🗺️ Authored structured spatial carriers require valid replay and native geometry emission in both languages.
#[test]
fn canonical_spatial_data_mutation_vectors(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧬️native-chart-grammar/🔣️.json")).unwrap();let source=&fixture["nativeSpatialData"];let mut failures=Vec::new();
 for(appearance,language)in ["light","dark"].into_iter().flat_map(|appearance|["en","de"].into_iter().map(move|language|(appearance,language))){for(case,bad)in source["cases"].as_array().unwrap().iter().map(|case|(case,None)).chain(source["badCases"].as_array().unwrap().iter().map(|bad|(source["cases"].as_array().unwrap().iter().find(|case|case["id"]==bad["caseId"]).unwrap(),Some(bad)))){
  let id=format!("spatial-{}-{language}-{appearance}",bad.unwrap_or(case)["id"].as_str().unwrap());let mut authored=source["snapshot"].clone();authored["chart"]["language"]=serde_json::Value::String(language.into());authored["chart"]["theme"]=serde_json::json!({"appearance":appearance});let before=ChartSnapshot::from_value(DslValue::from(authored)).unwrap();let mut current=before.clone();let mut setup=true;
  let mutations=[serde_json::json!({"path":["tables"],"value":[case["table"].clone()]}),serde_json::json!({"path":["presets"],"value":[case["preset"].clone()]})];
  for value in mutations.iter().chain(if bad.is_none(){Some(&case["mutation"])}else{None}).chain(bad.map(|bad|&bad["mutation"])) {let mutation=ChangeChartValue::from_value(DslValue::from(value.clone())).unwrap();let outcome=mutation.diff(&current);if !outcome.messages().is_empty(){failures.push(format!("{id}: structured mutation refused {:?}",outcome.messages()));setup=false;break;}let next=protocol::apply_diff(outcome.diff(),&current).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&current),&next).unwrap(),current);current=next;}
  if !setup{continue;}let result=ChartTextOutput::render(&current).unwrap();let valid=bad.is_none();if result.complete!=valid{failures.push(format!("{id}: inferred {}, expected {valid}, {:?}",result.complete,result.diagnostics));}
  if valid&&result.complete{let mut seen=Vec::new();let controlled=crate::io::text::inferences::render_chart_controlled(&current,&mut |work|{seen.push(work);Ok(())}).unwrap();assert_eq!(controlled,result.tikz);assert!(seen.windows(2).all(|pair|pair[0]<=pair[1]));let boundary=current.chart.get("tables").unwrap().as_array().unwrap()[0].get("rows").unwrap().as_array().unwrap().len()as u64+1;let mut last=0;let stopped=crate::io::text::inferences::render_chart_controlled(&current,&mut |work|{last=work;if work>=boundary{Err("spatial-registry-stop".into())}else{Ok(())}});assert_eq!(stopped.unwrap_err(),"spatial-registry-stop");assert_eq!(last,boundary);}
  if valid&&result.complete{if let Some(root)=std::env::var_os("PRINT_NATIVE_SPATIAL_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("{id}.tex")),result.tikz).unwrap();}}
  assert_eq!(before.chart.get("tables"),None);
 }}
 assert!(failures.is_empty(),"{}",failures.join("\n"));eprintln!("[DEBUG] Native spatial Rust: {} valid and{} invalid carriers in en/de and light/dark passed canonical mutation/inverse/inference/progress/cancellation; emitted valid source awaits native D3/PDF adjudication",source["cases"].as_array().unwrap().len(),source["badCases"].as_array().unwrap().len());
}
/// 🐝️ Neutral planar frame and closed control mutations produce source-bound Rust grammar in both themes.
#[test]
fn canonical_planar_frame_mutation_vectors(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧬️native-chart-grammar/🔣️.json")).unwrap();let source=&fixture["nativeGeoPlanar"];let mut failures=Vec::new();
 for appearance in ["light","dark"]{for case in source["cases"].as_array().unwrap(){
  let mut authored=source["snapshot"].clone();authored["chart"]["language"]=case["language"].clone();authored["chart"]["theme"]=serde_json::json!({"appearance":appearance});let mut table=source["table"].clone();table["rows"]=serde_json::Value::Array(case["points"].as_array().unwrap().iter().map(|point|serde_json::json!({"x":point[0].clone(),"y":point[1].clone()})).collect());authored["chart"]["tables"]=serde_json::json!([table]);authored["chart"]["presets"]=serde_json::json!([{"kind":case["kind"].clone(),"options":{"points":source["table"]["name"].clone(),"radius":case["radius"].clone(),"bandwidth":case["bandwidth"].clone(),"gridWidth":16,"gridHeight":11,"gridOrigin":"0,0","gridCell":5,"hexScale":0.92,"heatOpacity":0.3,"thresholds":"","levels":case["levels"].clone(),"width":case["width"].clone(),"height":case["height"].clone(),"opacity":1,"lineWidth":".1mm"}}]);
  let before=ChartSnapshot::from_value(DslValue::from(authored)).unwrap();let mut current=before.clone();for key in ["gridWidth","gridHeight","gridOrigin","gridCell","hexScale","heatOpacity","thresholds","opacity","lineWidth"]{let value=if ["gridOrigin","thresholds"].contains(&key){serde_json::Value::String(case[key].as_array().unwrap().iter().map(|item|item.to_string()).collect::<Vec<_>>().join(","))}else{case[key].clone()};let options=current.chart.get("presets").unwrap().as_array().unwrap()[0].get("options").unwrap();if options.get(key)==Some(&DslValue::from(value.clone())){continue;}let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options",key],"value":value}))).unwrap();let outcome=mutation.diff(&current);assert!(outcome.messages().is_empty());let next=protocol::apply_diff(outcome.diff(),&current).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&current),&next).unwrap(),current);current=next;}
  let result=ChartTextOutput::render(&current).unwrap();if !result.complete{failures.push(format!("{}-{appearance}: {:?}",case["id"],result.diagnostics));continue;}let mut progress=Vec::new();let controlled=crate::io::text::inferences::render_chart_controlled(&current,&mut|work|{progress.push(work);Ok(())}).unwrap();assert_eq!(controlled,result.tikz);assert!(progress.windows(2).all(|pair|pair[0]<=pair[1]));assert_eq!(crate::io::text::inferences::render_chart_controlled(&current,&mut|work|if work>=1{Err("planar-stop".into())}else{Ok(())}).unwrap_err(),"planar-stop");
  if let Some(root)=std::env::var_os("PRINT_NATIVE_GEO_PLANAR_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("geo-planar-{}-{appearance}.tex",case["id"].as_str().unwrap())),result.tikz).unwrap();}
  assert_eq!(before.chart.get("presets").unwrap().as_array().unwrap()[0].get("options").unwrap().get("opacity").unwrap().as_u64(),Some(1));
 }}
 for case in source["admissionCases"].as_array().unwrap(){let mut authored=source["snapshot"].clone();authored["chart"]["language"]=case["language"].clone();authored["chart"]["tables"]=serde_json::json!([source["table"].clone()]);authored["chart"]["presets"]=serde_json::json!([{"kind":"hexbin-map","options":{"points":source["table"]["name"].clone(),"width":60,"height":30,"radius":5,"bandwidth":5,"lineWidth":".3mm","opacity":0.5}}]);let before=ChartSnapshot::from_value(DslValue::from(authored)).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(case["mutation"].clone())).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=protocol::apply_diff(outcome.diff(),&before).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before),&current).unwrap(),before);let result=ChartTextOutput::render(&current).unwrap();let valid=case["valid"].as_bool().unwrap();if result.complete!=valid{failures.push(format!("{} owned={} expected={valid}: {:?}",case["id"],result.complete,result.diagnostics));}if !valid&&!result.complete{assert!(result.tikz.is_empty());assert!(!result.diagnostics.is_empty());}}
 assert!(failures.is_empty(),"{}",failures.join("\n"));eprintln!("[DEBUG] Rust planar{} mutated theme/language sources plus{} neutral accepted/rejected controls and progress/cancellation passed; emitted source awaits actual native consumption",source["cases"].as_array().unwrap().len()*2,source["admissionCases"].as_array().unwrap().len());
}

/// 🧾️ Language-neutral scientific mode and resolved-record vectors pass canonical Rust mutation and inference admission.
#[test]
fn canonical_scientific_mode_record_admission_vectors(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧬️native-chart-grammar/🔣️.json")).unwrap();let source=&fixture["nativeScientificAdmission"];let mut failures=Vec::new();
 for language in ["en","de"]{for case in source["cases"].as_array().unwrap(){let family=source["families"].as_array().unwrap().iter().find(|family|family["name"]==case["family"]).unwrap();let mut authored=source["snapshot"].clone();authored["chart"]["language"]=language.into();let mut options=case["options"].clone();options["mode"]=family["modes"][if options["mode"]==family["modes"][0]{1}else{0}].clone();authored["chart"]["presets"]=serde_json::json!([{"kind":case["kind"].clone(),"options":options}]);let before=ChartSnapshot::from_value(DslValue::from(authored)).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options","mode"],"value":case["options"]["mode"].clone()}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=protocol::apply_diff(outcome.diff(),&before).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before),&current).unwrap(),before);let result=ChartTextOutput::render(&current).unwrap();let valid=case["valid"].as_bool().unwrap();if result.complete!=valid{failures.push(format!("{}-{language}: admitted={} expected={valid}: {:?}",case["id"],result.complete,result.diagnostics));}if valid&&result.complete{if let Some(root)=std::env::var_os("PRINT_NATIVE_SCIENTIFIC_ADMISSION_TIKZ"){let root=std::path::Path::new(&root);std::fs::create_dir_all(root).unwrap();std::fs::write(root.join(format!("{}-{language}.tex",case["id"].as_str().unwrap())),&result.tikz).unwrap();}}if !result.complete{assert!(result.tikz.is_empty());assert!(!result.diagnostics.is_empty());}}
 }assert!(failures.is_empty(),"{}",failures.join("\n"));eprintln!("[DEBUG] Scientific Rust admission{} neutral en/de mutation/inverse and accepted/rejected record cases passed",source["cases"].as_array().unwrap().len()*2);
}
/// 📏️ Declared scientific scale and shape controls match the same independent neutral admission corpus.
#[test]
fn canonical_scientific_scalar_admission_vectors(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧬️native-chart-grammar/🔣️.json")).unwrap();let cases=fixture["scientificScalarAdmission"]["vectors"]["cases"].as_array().unwrap();
 for language in ["en","de"]{for case in cases{let key=case["key"].as_str().unwrap();let initial=if key=="nodeShape"{serde_json::Value::String(if case["value"]=="circle"{"box"}else{"circle"}.into())}else{serde_json::json!(1)};let mut options=serde_json::json!({});options[key]=initial;let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":140,"height":120,"language":language,"layers":[],"presets":[{"kind":case["kind"].clone(),"options":options}]}}))).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options",key],"value":case["value"].clone()}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=protocol::apply_diff(outcome.diff(),&before).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before),&current).unwrap(),before);let result=ChartTextOutput::render(&current).unwrap();assert_eq!(result.complete,case["valid"].as_bool().unwrap(),"{}-{language}: {:?}",case["id"],result.diagnostics);if !result.complete{assert!(result.tikz.is_empty());assert!(!result.diagnostics.is_empty());}}
 }eprintln!("[DEBUG] Scientific Rust scalar admission{} neutral EN/de mutation/inverse scale and shape cases passed",cases.len()*2);
}

/// 🧠️ Neutral neural layer topology and endpoint contracts survive Rust canonical mutation and inverse replay.
#[test]
fn canonical_neural_architecture_incidence_vectors(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧬️native-chart-grammar/🔣️.json")).unwrap();let source=&fixture["nativeNeuralTopologyControls"];let mut count=0;
 for language in ["en","de"]{for entry in source["vectors"]["architectures"].as_array().unwrap(){let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":80,"height":50,"language":language,"layers":[],"presets":[{"kind":entry["kind"].clone(),"options":{"mode":"tensor"}}]}}))).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options","mode"],"value":entry["mode"].clone()}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=protocol::apply_diff(outcome.diff(),&before).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before),&current).unwrap(),before);let result=ChartTextOutput::render(&current).unwrap();assert!(result.complete,"{}-{language}: {:?}",entry["kind"],result.diagnostics);if let Some(root)=std::env::var_os("PRINT_NATIVE_NEURAL_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("{}-{language}.tex",entry["kind"].as_str().unwrap())),result.tikz).unwrap();}count+=1;}
  let layers=source["custom"]["layers"].as_array().unwrap().iter().enumerate().map(|(index,row)|serde_json::json!({"layer":index+1,"units":row[0].clone(),"kind":row[1].clone(),"label":row[2].clone()})).collect::<Vec<_>>();let valid=[source["custom"]["links"].clone(),source["custom"]["empty"].clone()];for(links,accepted)in valid.iter().map(|value|(value,true)).chain(source["invalid"].as_array().unwrap().iter().map(|value|(value,false))){let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":80,"height":50,"language":language,"layers":[],"tables":[{"name":"neural-custom","columns":["layer","units","kind","label"],"rows":layers}],"presets":[{"kind":"feed-forward-neural-network","options":{"data":"neural-custom","links":"","connections":"full","mode":"rnn","directed":true}}]}}))).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options","links"],"value":links}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=protocol::apply_diff(outcome.diff(),&before).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before),&current).unwrap(),before);let result=ChartTextOutput::render(&current).unwrap();assert_eq!(result.complete,accepted,"{language}/{links}: {:?}",result.diagnostics);if accepted{if let Some(root)=std::env::var_os("PRINT_NATIVE_NEURAL_TIKZ"){let id=if links.as_str()==Some(""){ "empty" }else{ "custom" };std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("{id}-{language}.tex")),result.tikz).unwrap();}}else{assert!(result.tikz.is_empty());assert!(!result.diagnostics.is_empty());}count+=1;}
  for connections in source["vectors"]["implicitConnections"].as_array().unwrap(){let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":80,"height":50,"language":language,"layers":[],"tables":[{"name":"neural-custom","columns":["layer","units","kind","label"],"rows":layers}],"presets":[{"kind":"feed-forward-neural-network","options":{"data":"neural-custom","connections":"residual","mode":"rnn","directed":true,"curvature":0}}]}}))).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options","connections"],"value":connections}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=protocol::apply_diff(outcome.diff(),&before).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before),&current).unwrap(),before);let result=ChartTextOutput::render(&current).unwrap();assert!(result.complete,"{language}/{connections}: {:?}",result.diagnostics);if let Some(root)=std::env::var_os("PRINT_NATIVE_NEURAL_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("implicit-{}-{language}.tex",connections.as_str().unwrap())),result.tikz).unwrap();}count+=1;}
  for entry in source["vectors"]["selfLoops"].as_array().unwrap(){let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":80,"height":50,"language":language,"theme":{"appearance":if language=="de"{"dark"}else{"light"}},"layers":[],"tables":[{"name":"neural-self","columns":["layer","units","kind","label"],"rows":layers}],"presets":[{"kind":"feed-forward-neural-network","options":{"data":"neural-self","mode":"rnn","render":entry["render"].clone(),"directed":entry["directed"].clone(),"padding":source["vectors"]["padding"].clone(),"unitSize":source["vectors"]["unitSize"].clone(),"labels":false,"curvature":0.35,"links":"2/1/2/1"}}]}}))).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options","links"],"value":"2/2/2/2"}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=protocol::apply_diff(outcome.diff(),&before).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&before),&current).unwrap(),before);let result=ChartTextOutput::render(&current).unwrap();assert!(result.complete,"{language}/{}: {:?}",entry["id"],result.diagnostics);if let Some(root)=std::env::var_os("PRINT_NATIVE_NEURAL_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("self-{}-{language}.tex",entry["id"].as_str().unwrap())),result.tikz).unwrap();}count+=1;}
 }assert_eq!(count,68);eprintln!("[DEBUG] Neural Rust{count} neutral en/de stock/custom/empty/implicit/self-loop/invalid incidence cases passed mutation, inverse and canonical inference");
}

/// 📐️ Neutral construction windows preserve canonical Rust mutation, inverse replay and actual native source ownership.
#[test]
fn canonical_scientific_construction_window_vectors(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧬️native-chart-grammar/🔣️.json")).unwrap();let control=&fixture["scientificConstructionWindows"]["vectors"];let mut count=0;
 for(appearance,language)in[("light","en"),("dark","de")]{for(index,entry)in control["cases"].as_array().unwrap().iter().enumerate(){for mode in control["variants"].as_array().unwrap(){let mode=mode.as_u64().unwrap();let mut options=entry["options"].clone();options["points"]="O/0/0,Q/1/0,R/0/1".into();options["width"]=control["familyFrame"][0].clone();options["height"]=control["familyFrame"][1].clone();for key in["title","xlabel","ylabel","domain","range"]{options[key]="".into();}if mode!=3{options["axes"]=false.into();options["grid"]=false.into();}
  let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":control["canvas"][0].clone(),"height":control["canvas"][1].clone(),"language":language,"theme":{"appearance":appearance},"layers":[],"presets":[{"kind":entry["kind"].clone(),"options":options}]}}))).unwrap();let mut current=before.clone();let mut changes=vec![("points",entry["options"]["points"].clone())];
  if mode!=0&&mode!=3{changes.extend([("axes",true.into()),("grid",true.into()),("ticks",control["tickDivisions"][if mode==1{0}else{1}].clone()),("title","Cvtitle".into()),("xlabel","Cvx".into()),("ylabel","Cvy".into())]);for(key,enabled)in[("domain",[2,4,6].contains(&mode)),("range",[2,5,6].contains(&mode))]{if enabled{let mut values=control[key].as_array().unwrap().clone();if mode==6{values.reverse();}changes.push((key,values.iter().map(|value|value.to_string()).collect::<Vec<_>>().join(",").into()));}}}
  for(key,value)in changes{let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options",key],"value":value}))).unwrap();let outcome=mutation.diff(&current);assert!(outcome.messages().is_empty());let next=protocol::apply_diff(outcome.diff(),&current).unwrap();assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&current),&next).unwrap(),current);current=next;}
  let result=ChartTextOutput::render(&current).unwrap();assert!(result.complete,"Cv{index}M{mode}/{appearance}: {:?}",result.diagnostics);if let Some(root)=std::env::var_os("PRINT_NATIVE_CONSTRUCTION_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("Cv{index}M{mode}-{appearance}.tex")),result.tikz).unwrap();}assert_eq!(before.chart.get("presets").unwrap().as_array().unwrap()[0].get("options").unwrap().get("points").unwrap().as_str(),Some("O/0/0,Q/1/0,R/0/1"));count+=1;
 }}}eprintln!("[DEBUG] Construction Rust{count} neutral en/de canonical mutation/inverse/inference sources emitted for native D3/PDF verification");
}

#[test]
fn chart_diff_text_and_binary_preserve_owned_edits() {
    use protocol::{DiffText,DiffBinary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔺️diff-wire/🔣️.json")).unwrap();
    for expected in fixture["cases"].as_array().unwrap() {
        let diff = ChartDiff::from_value(DslValue::from(expected.clone())).unwrap();
        let text = diff.print_diff();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(), *expected);
        assert_eq!(ChartDiff::parse_diff(&text).unwrap(), diff);
        assert_eq!(ChartDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(), diff);
    }
    assert!(ChartDiff::parse_diff("{\"edits\":[{\"path\":[],\"unknown\":1}]}").is_err());
    assert!(ChartDiff::decode_diff(&[255]).is_err());
    eprintln!("[DEBUG] chart diff physical owners preserve typed edits; independent serde_json oracle");
}
