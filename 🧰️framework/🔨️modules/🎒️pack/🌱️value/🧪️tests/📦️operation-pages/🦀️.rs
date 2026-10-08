use super::*;

fn finish(owner: &mut protocol::io::binary::operation_bytes::OwnedOperationBytes) {
    for _ in 0..17000 {
        if owner.close_one(1, 4096).unwrap() == protocol::io::binary::operation_bytes::OperationByteCloseStep::Complete { break; }
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(owner.allocated_bytes(), 0);
}

#[test]
fn record_operation_pages_emit_exact_canonical_words_and_keep_canceled_prefix_owned() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "bytes", Shape::Bytes64), FieldSpec::new(2, "numbers", Shape::Value)]);
    let mut record = RecordValue::default();
    let payload: Vec<u8> = (0..fixture["payloadBytes"].as_u64().unwrap()).map(|index| (index % 251) as u8).collect();
    record.fields.insert(1, FieldValue::Bytes64(payload.clone()));
    record.fields.insert(2, FieldValue::Value(DslValue::Array(vec![DslValue::uint(u64::MAX), DslValue::int(i64::MIN), DslValue::float(-0.0)])));
    record.fields.insert(99, FieldValue::UInt(7));
    for preserve_unknown in [true, false] {
        let mut options = EncodeOptions::default();
        options.preserve_unknown = preserve_unknown;
        let expected = encode_record_body(&spec, &record, &options).unwrap();
        let literal = |key: &str| fixture[key].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect::<Vec<_>>();
        let mut independent = literal(if preserve_unknown { "preservedPrefix" } else { "knownPrefix" });
        independent.extend_from_slice(&payload);
        independent.extend(literal("numberSuffix"));
        if preserve_unknown { independent.extend(literal("unknownSuffix")); }
        assert_eq!(expected, independent);
        let mut output = protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(16384, 65536).unwrap();
        let mut allow = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(65536, &mut allow);
        let length = encode_record_body_into(&spec, &record, &options, &mut output, &mut control).unwrap();
        assert_eq!(length, expected.len());
        assert!(output.iter().eq(expected.iter().copied()));
        assert_eq!(serde_json::to_value(&output).unwrap(), serde_json::to_value(&expected).unwrap());
        let (decoded, report) = decode_record_body(&expected, &spec, &DecodeOptions::default()).unwrap();
        let mut expected_record = record.clone();
        if !preserve_unknown { expected_record.fields.remove(&99); }
        assert_eq!(decoded, expected_record);
        assert_eq!(report.unknown_field_ids, if preserve_unknown { vec![99] } else { vec![] });
        let FieldValue::Value(DslValue::Array(numbers)) = decoded.fields.get(&2).unwrap() else { panic!("exact number array lost") };
        assert!(matches!(numbers[0], DslValue::Number(Number::UInt(u64::MAX))));
        assert!(matches!(numbers[1], DslValue::Number(Number::Int(i64::MIN))));
        let DslValue::Number(Number::Float(word)) = numbers[2] else { panic!("exact float variant lost") };
        assert_eq!(word.to_bits(), 0x8000000000000000);
        finish(&mut output);

        let mut output = protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(16384, 65536).unwrap();
        let mut allow = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(65536, &mut allow);
        protocol::io::binary::operation_bytes::OperationByteOutput::write_bytes(&mut output, &[1, 7], &mut control).unwrap();
        assert_eq!(encode_record_body_into(&spec, &record, &options, &mut output, &mut control).unwrap(), expected.len());
        assert!(output.iter().eq([1, 7].into_iter().chain(expected.iter().copied())));
        finish(&mut output);

        let mut output = protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(16384, 65536).unwrap();
        let mut cancel = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| !(progress.total == payload.len() && progress.completed >= 256);
        let mut control = semio_framework_value::NativeEncodeControl::new(65536, &mut cancel);
        let error = encode_record_body_into(&spec, &record, &options, &mut output, &mut control).unwrap_err();
        let PackRefusal::ValueRefusal(refusal) = error else { panic!("cancellation category lost") };
        assert_eq!(refusal.kind.as_str(), fixture["canceledKind"].as_str().unwrap());
        assert!(output.len() >= 256 && output.len() < expected.len());
        assert!(output.iter().eq(expected[..output.len()].iter().copied()));
        assert_eq!(record.fields.get(&1), Some(&FieldValue::Bytes64(payload.clone())));
        finish(&mut output);
    }
    println!("[DEBUG] Core streamed canonical Record bytes directly into source-owned pages; cancellation retains the exact emitted prefix until actual bounded terminal retirement");
}

#[test]
fn record_operation_pages_decode_cross_page_bytes_and_utf8_without_flattening_the_source() {
    use protocol::{ByteSpan, io::binary::operation_bytes::{OwnedOperationBytes, OperationByteOutput}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["sourceRange"];
    let payload:Vec<u8>=(0..case["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect();
    let text=case["utf8Word"].as_str().unwrap().repeat(case["utf8Repetitions"].as_u64().unwrap()as usize);
    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"bytes",Shape::Bytes64),FieldSpec::new(2,"text",Shape::Text)]);
    let mut record=RecordValue::default();
    record.fields.insert(1,FieldValue::Bytes64(payload.clone()));
    record.fields.insert(2,FieldValue::Text(text.clone()));
    let mut source=OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    encode_record_body_into(&spec,&record,&Default::default(),&mut source,&mut encoding).unwrap();
    let retained=source.allocated_bytes();
    let mut allow=|_|true;
    let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    let decoded=decode_record_body_span_exact_controlled(ByteSpan::from_source(&source),&spec,&Default::default(),&mut decoding).unwrap();
    assert_eq!(decoded,record);
    let FieldValue::Bytes64(bytes)=decoded.fields.get(&1).unwrap() else{panic!("complete octet owner lost")};
    let FieldValue::Text(text)=decoded.fields.get(&2).unwrap() else{panic!("complete utf8 owner lost")};
    assert_eq!(serde_json::to_value((bytes,text)).unwrap(),serde_json::json!([payload,case["utf8Word"].as_str().unwrap().repeat(case["utf8Repetitions"].as_u64().unwrap()as usize)]));
    assert_eq!(source.allocated_bytes(),retained);
    finish(&mut source);

    let bad:Vec<u8>=case["malformedUtf8Record"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    let mut source=OwnedOperationBytes::try_new(32,65536).unwrap();
    let mut allow=|_|true;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    source.write_bytes(&bad,&mut encoding).unwrap();
    let mut allow=|_|true;
    let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    let error=decode_record_body_span_exact_controlled(ByteSpan::from_source(&source),&spec,&Default::default(),&mut decoding).unwrap_err();
    assert_eq!(error.kind().as_str(),case["malformedKind"].as_str().unwrap());
    assert!(source.iter().eq(bad));
    finish(&mut source);
    println!("[DEBUG] Core source-range decoding preserves all cross-page octets and UTF8 words; malformed input retains the same source until bounded terminal retirement");
}

#[test]
fn intrinsic_operation_pages_emit_and_decode_same_complete_owner_under_actual_policy() {
    use protocol::{ByteSpan, io::binary::operation_bytes::OwnedOperationBytes};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["intrinsicSource"];
    let payload:Vec<u8>=(0..fixture["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect();
    let value=DslValue::Bytes(payload.clone());
    let mut expected:Vec<u8>=case["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(&payload);
    let mut output=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(encode_value_record_body_into(1,&value,&options,&mut output,&mut encoding).unwrap(),expected.len());
    assert!(output.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&output).unwrap(),serde_json::to_value(&expected).unwrap());
    let retained=output.allocated_bytes();
    let mut options=DecodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    assert_eq!(decode_value_record_body_span_exact_controlled(ByteSpan::from_source(&output),1,&options,&mut decoding).unwrap(),value);
    assert_eq!(output.allocated_bytes(),retained);
    options.limits.max_file_len-=1;
    assert_eq!(decode_value_record_body_span_exact_controlled(ByteSpan::from_source(&output),1,&options,&mut decoding).unwrap_err().kind(),ValueRefusalKind::WorkLimit);
    assert!(output.iter().eq(expected.iter().copied()));finish(&mut output);
    let mut output=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64-1;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(encode_value_record_body_into(1,&value,&options,&mut output,&mut encoding).unwrap_err().kind(),ValueRefusalKind::WorkLimit);
    assert_eq!(output.len(),0);finish(&mut output);
    assert_eq!(serde_json::to_value(&payload).unwrap(),serde_json::to_value(&expected[case["prefix"].as_array().unwrap().len()..]).unwrap());
    println!("[DEBUG] Intrinsic Core producer emits exact literal-framed complete8194 owner into paid pages; source decoding keeps the same backing and exact/one-byte-short policy before fixed4096 terminal release");
}

#[test]
fn record_operation_pages_real_core_sink_uses_step_funded_backing_and_same_control(){
    use protocol::io::binary::operation_bytes::{OperationBytePreparation,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["prepaidRecord"];
    let payload:Vec<u8>=(0..fixture["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect();
    let mut expected:Vec<u8>=case["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(&payload);
    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"bytes",Shape::Bytes64)]);
    let mut record=RecordValue::default();record.fields.insert(1,FieldValue::Bytes64(payload));
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut prepared=OperationBytePreparation::try_new(expected.len(),65536).unwrap();
    let mut allow=|_|true;let control=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    let mut receipt=control.pause().unwrap();
    for _ in 0..17000{
        let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::resume(receipt,&mut allow).unwrap();
        prepared.fund_one(1,4096,&mut control).unwrap();receipt=control.pause().unwrap();
        if prepared.is_funded(){break;}
    }
    assert!(prepared.is_funded());assert_eq!(prepared.accepted_prefix().unwrap().len(),0);
    let paid=prepared.allocated_bytes();
    let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::resume(receipt,&mut allow).unwrap();
    assert_eq!(control.owned_bytes(),paid);
    assert_eq!(encode_record_body_into(&spec,&record,&options,&mut prepared,&mut control).unwrap(),expected.len());
    assert_eq!(prepared.allocated_bytes(),paid);
    assert_eq!(control.owned_bytes(),paid+case["fieldIndexAdmissionBytes"].as_u64().unwrap()as usize);
    let mut source=prepared.take_ready().unwrap();assert!(prepared.terminal_is_empty());
    assert!(source.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    assert_eq!(decode_record_body_span_exact_controlled(protocol::ByteSpan::from_source(&source),&spec,&Default::default(),&mut control).unwrap(),record);
    for _ in 0..17000{if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
    assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
    println!("[DEBUG] Real Core Record producer uses paid8194 payload backing from separate4096 funding hops with same cumulative Native control; only literal4-byte field indexes are newly admitted and same source decodes exactly before4096 terminal release");
}

#[test]
fn record_operation_pages_borrowed_source_excludes_large_payload_mirrors_and_keeps_literal_wire(){
    use protocol::io::binary::operation_bytes::{OperationBytePreparation,OperationByteMeasurement};
    use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V};
    struct Source{bytes:Vec<u8>,text:String}
    impl FieldProjectionSource for Source{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{
            match path{[]=>Ok(V::Record(&[1,2])),[0]=>Ok(V::Bytes(&self.bytes)),[1]=>Ok(V::Text(&self.text)),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}
        }
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["borrowedSource"];
    let source=Source{bytes:(0..fixture["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect(),text:case["textUnit"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize)};
    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"bytes",Shape::Bytes64),FieldSpec::new(2,"text",Shape::Text)]);
    let mut record=RecordValue::default();record.fields.insert(1,FieldValue::Bytes64(source.bytes.clone()));record.fields.insert(2,FieldValue::Text(source.text.clone()));
    let literal=|key:&str|case[key].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();
    let mut expected=literal("prefix");expected.extend_from_slice(&source.bytes);expected.extend(literal("textHeader"));expected.extend_from_slice(source.text.as_bytes());
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    assert_eq!(encode_record_body(&spec,&record,&options).unwrap(),expected);
    let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);
    let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    assert_eq!(encode_projected_record_body_into(&spec,&source,&options,&mut measure,&mut control).unwrap(),expected.len());
    assert!(control.owned_bytes()<case["maximumMetadataBytes"].as_u64().unwrap()as usize);
    let mut prepared=OperationBytePreparation::try_new(measure.exact_length().unwrap(),65536).unwrap();
    for _ in 0..34000{prepared.fund_one(1,4096,&mut control).unwrap();if prepared.is_funded(){break;}}
    assert!(prepared.is_funded());let before=control.owned_bytes();let backing=prepared.allocated_bytes();
    assert_eq!(control.scoped_maximum(before+case["maximumMetadataBytes"].as_u64().unwrap()as usize,|control|encode_projected_record_body_into(&spec,&source,&options,&mut prepared,control)).unwrap(),expected.len());
    assert!(control.owned_bytes()-before<case["maximumMetadataBytes"].as_u64().unwrap()as usize);assert_eq!(prepared.allocated_bytes(),backing);
    let mut wire=prepared.take_ready().unwrap();assert!(wire.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&wire).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    assert_eq!(decode_record_body_span_exact_controlled(protocol::ByteSpan::from_source(&wire),&spec,&Default::default(),&mut decoding).unwrap(),record);
    finish(&mut wire);
    let mut wire=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
    let mut canceled=|event:semio_framework_value::native_encoding::NativeEncodeProgress|!(event.total==source.bytes.len()&&event.completed>=256);
    let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut canceled);
    let refusal=encode_projected_record_body_into(&spec,&source,&options,&mut wire,&mut control).unwrap_err();
    assert!(matches!(refusal,PackRefusal::ValueRefusal(ValueError{kind:ValueRefusalKind::Canceled,..})));
    assert!(wire.len()>0&&wire.len()<expected.len());assert!(wire.iter().eq(expected[..wire.len()].iter().copied()));finish(&mut wire);
    assert_eq!(source.bytes.len(),fixture["payloadBytes"].as_u64().unwrap()as usize);assert_eq!(source.text.len(),source.bytes.len());
    println!("[DEBUG] Core borrowed source emits literal bytes plus8194 text without an owned payload mirror; same cumulative control funds fixed4096 pages and retires real wire, canceled prefix retained");
}

#[test]
fn record_operation_pages_borrowed_source_joins_actual_nested_derived_tables_numbers_and_wire(){
    use semio_framework_dsl_record_derive::DslRecord;
    use semio_framework_dsl_record::Wire;
    #[derive(DslRecord)]
    struct Row{active:Option<bool>,name:String,count:u64}
    #[derive(DslRecord)]
    struct Owner{signed:Vec<i64>,unsigned:Vec<u64>,wide:Vec<u64>,floats:Vec<f64>,tuple:[f64;3],missing:Option<String>,nested:Box<Row>,#[dsl(table)]rows:Vec<Row>,value:DslValue,wire:Wire}
    let owner=Owner{signed:vec![i64::MIN,-1,0,i64::MAX],unsigned:vec![0,127,128,i64::MAX as u64],wide:vec![u64::MAX],floats:vec![-0.0,1.5],tuple:[1.0,2.0,3.0],missing:None,nested:Box::new(Row{active:Some(false),name:"same".into(),count:17}),rows:vec![Row{active:Some(true),name:"same".into(),count:1},Row{active:None,name:"other".into(),count:u64::MAX}],value:DslValue::Object(vec![("z".into(),DslValue::Array(vec![DslValue::uint(u64::MAX),DslValue::int(i64::MIN),DslValue::float(-0.0)])),("a".into(),DslValue::String("same".into()))]),wire:Wire(WireValue{from:WireNode{id:"from".into(),kind:Some("node".into()),port:None},edge:Some((true,WireNode{id:"to".into(),kind:None,port:Some("port".into())})),edge_label:WireEdgeLabel{id:Some("edge".into()),kind:Some("kind".into())},properties:DslValue::Object(vec![("weight".into(),DslValue::float(-0.0))])})};
    let spec=Owner::__dsl_spec();let record=owner.__dsl_to_record();let options=EncodeOptions::default();
    let expected=encode_record_body(&spec,&record,&options).unwrap();
    let mut source=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(encode_projected_record_body_into(&spec,&owner,&options,&mut source,&mut encoding).unwrap(),expected.len());
    assert!(source.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    assert_eq!(decode_record_body_span_exact_controlled(protocol::ByteSpan::from_source(&source),&spec,&Default::default(),&mut decoding).unwrap(),record);
    finish(&mut source);
    let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    let mut narrow=options.clone();narrow.limits.max_depth=0;
    let refusal=encode_projected_record_body_into(&spec,&owner,&narrow,&mut output,&mut encoding).unwrap_err();
    assert!(matches!(refusal,PackRefusal::ValueRefusal(ValueError{kind:ValueRefusalKind::DepthLimit,..})));assert_eq!(output.len(),0);finish(&mut output);
    #[derive(DslRecord)]
    struct Only{nested:Row}
    let only=Only{nested:Row{active:Some(true),name:"same".into(),count:1}};let spec=Only::__dsl_spec();let mut narrow=options.clone();narrow.limits.max_depth=2;
    assert!(matches!(encode_record_body(&spec,&only.__dsl_to_record(),&narrow),Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,..})));
    let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    let result=encode_projected_record_body_into(&spec,&only,&narrow,&mut output,&mut encoding);let rejected=result.is_err();finish(&mut output);
    assert!(rejected,"nested canonical Record value depth3 must refuse at2 even when source path has only2 ordinals");
    println!("[DEBUG] Actual derived original Record source preserves packed signed/unsigned/Float words, nested field IDs, sparse table bitmap, forced symbols, exact intrinsic variants and wire presence through paged canonical emission and same-source decode");
}

struct ImmutableSymbolSource{texts:Vec<String>}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for ImmutableSymbolSource{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{
  if path.len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"closed symbol source path"))}
  self.texts.get(path[0]).map(|text|semio_framework_dsl_record::native_encoding::FieldProjectionView::Text(text)).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"closed symbol source ordinal"))
 }
}
#[test]
fn borrowed_operation_symbols_preserve_first_source_and_all_paid_pages(){
 use super::{ProjectedSymbolScratch,SourceTextLocator,SourceTextKind};
 use semio_framework_value::NativeEncodeControl;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️borrowed-symbols/🔣️.json")).unwrap();
 let rows=fixture["symbols"].as_array().unwrap();let source=ImmutableSymbolSource{texts:rows.iter().map(|row|row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize)).collect()};
 let mut scratch=ProjectedSymbolScratch::empty();let mut accepted=|_|true;let mut control=NativeEncodeControl::new(fixture["positiveMaximumAllocationBytes"].as_u64().unwrap()as usize,&mut accepted);
 let(result,requested,released)=crate::test_allocation::observe_backing(||{
  for row in rows{scratch.note(&source,SourceTextLocator::new(&[row["source"].as_u64().unwrap()as usize],SourceTextKind::Text)?,row["forced"].as_bool().unwrap(),&mut control)?;}
  scratch.finish(&source,&mut control)
 });result.unwrap();assert_eq!(released,0);assert_eq!(requested,scratch.allocated_bytes());assert_eq!(control.owned_bytes(),requested);
 let expected=fixture["expectedSourceOrder"].as_array().unwrap();assert_eq!(scratch.len().unwrap(),expected.len());
 for(index,ordinal)in expected.iter().enumerate(){let original=&source.texts[ordinal.as_u64().unwrap()as usize];let text=scratch.locator(index).unwrap().resolve(&source).unwrap();assert_eq!(text,original);assert_eq!(text.as_ptr(),original.as_ptr());assert_eq!(scratch.index(&source,text,&mut control).unwrap(),Some(index as u64));}
 assert_eq!(scratch.index(&source,&source.texts[6],&mut control).unwrap(),None);
 let backing=scratch.allocated_bytes();for(items,bytes)in[(0,4096),(1,0)]{let(step,requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(items,bytes).unwrap());assert_eq!(step,(false,0,0));assert_eq!((requested,released),(0,0));assert_eq!(scratch.len().unwrap(),expected.len());assert_eq!(scratch.allocated_bytes(),backing);}
 let mut retired=0;for _ in 0..1000{let((progress,items,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,bytes);assert!(items<=1&&released<=4096);assert!(scratch.len().is_err());retired+=bytes;if !progress{break;}}
 assert_eq!(retired,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut scratch=ProjectedSymbolScratch::empty();let mut accepted=|_|true;let mut initial=NativeEncodeControl::new(4096,&mut accepted);
 let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let mut refusal=None;for row in rows{if let Err(error)=scratch.note(&source,SourceTextLocator::new(&[row["source"].as_u64().unwrap()as usize],SourceTextKind::Text).unwrap(),row["forced"].as_bool().unwrap(),&mut initial){refusal=Some(error);break;}}let error=refusal.expect("fixed4096 original symbol admission must refuse with paid backing retained");let result=(error.kind,match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() });drop(error);result});
 assert_eq!(kind,ValueRefusalKind::OwnershipLimit);assert_eq!(requested-diagnostic,scratch.allocated_bytes());assert_eq!(released,diagnostic);assert_eq!(initial.owned_bytes(),scratch.allocated_bytes());
 let backing=scratch.allocated_bytes();let mut retired=0;for _ in 0..1000{let((progress,items,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,bytes);assert!(items<=1&&released<=4096);retired+=bytes;if !progress{break;}}assert_eq!(retired,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut scratch=ProjectedSymbolScratch::empty();let mut fired=false;let mut cancel=|_|{if crate::test_allocation::observed_requested_bytes().is_some_and(|bytes|bytes>0){fired=true;false}else{true}};let mut canceled=NativeEncodeControl::new(65536,&mut cancel);
 let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=scratch.note(&source,SourceTextLocator::new(&[0],SourceTextKind::Text).unwrap(),false,&mut canceled).expect_err("actual post-backing cancellation");let result=(error.kind,match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() });drop(error);result});assert_eq!(kind,ValueRefusalKind::Canceled);assert_eq!(requested-diagnostic,scratch.allocated_bytes());assert_eq!(released,diagnostic);assert!(canceled.owned_bytes()>=scratch.allocated_bytes());drop(canceled);assert!(fired);
 let backing=scratch.allocated_bytes();let mut retired=0;for _ in 0..1000{let((progress,items,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,bytes);assert!(items<=1&&released<=4096);retired+=bytes;if !progress{break;}}assert_eq!(retired,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut scratch=ProjectedSymbolScratch::empty();let mut accepted=|_|true;let mut control=NativeEncodeControl::new(65536,&mut accepted);for row in rows{scratch.note(&source,SourceTextLocator::new(&[row["source"].as_u64().unwrap()as usize],SourceTextKind::Text).unwrap(),row["forced"].as_bool().unwrap(),&mut control).unwrap();}scratch.finish(&source,&mut control).unwrap();let backing=scratch.allocated_bytes();
 use semio_framework_value::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};let mut parent=ParentAllocationReturn::<64>::try_new(4096,65536).unwrap();
 for(items,bytes)in[(0,4096),(1,0)]{let(step,requested,released)=crate::test_allocation::observe_backing(||scratch.return_one(&mut parent,items,bytes).unwrap());assert_eq!(step,(false,0));assert_eq!((requested,released),(0,0));assert_eq!(scratch.len().unwrap(),expected.len());assert_eq!(scratch.allocated_bytes(),backing);assert!(parent.terminal_is_empty());}
 for _ in 0..1000{let((progress,returned),requested,released)=crate::test_allocation::observe_backing(||scratch.return_one(&mut parent,1,1).unwrap());assert_eq!((requested,released,returned),(0,0,0));assert_eq!(scratch.allocated_bytes(),backing);assert!(parent.terminal_is_empty());if !progress{break;}}
 let mut returned=0;for _ in 0..1000{let before=scratch.allocated_bytes();let((progress,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.return_one(&mut parent,1,4096).unwrap());assert_eq!((requested,released),(0,0));assert!(bytes<=4096);assert_eq!(before-scratch.allocated_bytes(),bytes);returned+=bytes;assert_eq!(parent.retained_bytes(),returned);assert!(scratch.len().is_err());if !progress{break;}}assert_eq!(returned,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut retired=0;for _ in 0..1000{let(step,requested,released)=crate::test_allocation::observe_backing(||parent.close_step(1,4096));assert_eq!(requested,0);match step{AllocationReturnStep::Complete=>{assert_eq!(released,0);break;},AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released<=4096);assert_eq!(released,released_bytes);retired+=released;}}}assert!(parent.terminal_is_empty());assert_eq!(retired,backing);
 for(row,text)in rows.iter().zip(&source.texts){assert_eq!(text.len(),row["text"].as_str().unwrap().len()*row["repeat"].as_u64().unwrap()as usize);}
 eprintln!("[DEBUG] actual borrowed symbols occurrences10 selected7 first-source pointers unchanged; fixed4096 refusal retains paid scratch; one-item4096 physical retirement exact");
}

#[test]
fn borrowed_static_projected_pack_keeps_source_scratch_and_exact_wire(){
 use super::BorrowedProjectedPackOperation;
 let authority:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️source-bound-pack/🔣️.json")).unwrap();assert_eq!(authority["originalSourceBytes"],8194);assert_eq!(authority["initialMaximumAllocationBytes"],4096);assert_eq!(authority["positiveMaximumAllocationBytes"],65536);assert_eq!(authority["maximumItems"],1);assert_eq!(authority["maximumBytes"],4096);assert_eq!(authority["maximumSteps"],32768);assert_eq!(authority["terminalAllocatedBytes"],0);
 use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as S,BorrowedShape as B};
 use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V};
 use protocol::io::binary::operation_bytes::{OperationByteMeasurement,OperationBytePreparation};
 struct Source{bytes:Vec<u8>,text:String}
 impl FieldProjectionSource for Source{fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[1,2])),[0]=>Ok(V::Bytes(&self.bytes)),[1]=>Ok(V::Text(&self.text)),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}}
 static FIELDS:[F;2]=[F::new(1,"bytes",B::Bytes64),F::new(2,"text",B::Text)];
 let spec=S{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS};
 let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();let bytes=f["payloadBytes"].as_u64().unwrap()as usize;assert_eq!(bytes,8194);
 let source=Source{bytes:(0..bytes).map(|index|(index%251)as u8).collect(),text:f["borrowedSource"]["textUnit"].as_str().unwrap().repeat(bytes)};let pointers=(source.bytes.as_ptr(),source.text.as_ptr());
 let literal=|key:&str|f["borrowedSource"][key].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();let mut expected=literal("prefix");expected.extend_from_slice(&source.bytes);expected.extend(literal("textHeader"));expected.extend_from_slice(source.text.as_bytes());
 let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
 let mut capsule=BorrowedProjectedPackOperation::from_source(&source,spec);let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut accepted);let mut measured=OperationByteMeasurement::new(options.limits.max_file_len);
 let(result,requested,released)=crate::test_allocation::observe_backing(||capsule.write_body(&options,&mut measured,&mut control));assert_eq!(result.unwrap(),expected.len());assert_eq!((requested,released),(capsule.allocated_bytes(),0));assert_eq!(control.owned_bytes(),capsule.allocated_bytes());let scratch=capsule.allocated_bytes();assert!(scratch>0&&scratch<65536);
 let mut prepared=OperationBytePreparation::try_new(measured.exact_length().unwrap(),65536).unwrap();for _ in 0..32768{prepared.fund_one(1,4096,&mut control).unwrap();if prepared.is_funded(){break}}assert!(prepared.is_funded());let backing=prepared.allocated_bytes();assert_eq!(control.owned_bytes(),backing+scratch);
 let(result,requested,released)=crate::test_allocation::observe_backing(||capsule.write_body(&options,&mut prepared,&mut control));assert_eq!(result.unwrap(),expected.len());assert_eq!((requested,released),(0,0));assert_eq!(capsule.allocated_bytes(),scratch);assert_eq!((source.bytes.as_ptr(),source.text.as_ptr()),pointers);
 let mut wire=prepared.take_ready().unwrap();assert!(wire.iter().eq(expected.iter().copied()));finish(&mut wire);
 let(zero,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(0,4096).unwrap());assert_eq!((zero,requested,released),((false,0,0),0,0));assert_eq!(capsule.allocated_bytes(),scratch);
 let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,step.2);assert!(step.1<=1&&released<=4096);disposed+=released;if !step.0{break}}assert_eq!(disposed,scratch);assert_eq!(capsule.allocated_bytes(),0);
 let mut denied=OperationByteMeasurement::new(options.limits.max_file_len);assert_eq!(capsule.write_body(&options,&mut denied,&mut control).unwrap_err().kind(),ValueRefusalKind::InvariantViolated);assert_eq!(denied.exact_length().unwrap(),0);assert_eq!(capsule.allocated_bytes(),0);
 let mut capsule=BorrowedProjectedPackOperation::from_source(&source,spec);let mut accepted=|_|true;let mut narrow=semio_framework_value::NativeEncodeControl::new(4096,&mut accepted);let mut measured=OperationByteMeasurement::new(options.limits.max_file_len);
 let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=capsule.write_body(&options,&mut measured,&mut narrow).unwrap_err().into_value_error();let result=(error.kind,match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() });drop(error);result});assert_eq!(kind,ValueRefusalKind::OwnershipLimit);assert_eq!(requested-diagnostic,capsule.allocated_bytes());assert_eq!(released,diagnostic);assert!(narrow.owned_bytes()>=capsule.allocated_bytes());assert_eq!(measured.exact_length().unwrap(),0);let scratch_before=capsule.allocated_bytes();assert_eq!(capsule.write_body(&options,&mut measured,&mut narrow).unwrap_err().kind(),ValueRefusalKind::InvariantViolated);assert_eq!(capsule.allocated_bytes(),scratch_before);assert_eq!(measured.exact_length().unwrap(),0);
 let scratch=capsule.allocated_bytes();let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,step.2);assert!(step.1<=1&&released<=4096);disposed+=released;if !step.0{break}}assert_eq!(disposed,scratch);assert_eq!(capsule.allocated_bytes(),0);
 let mut capsule=BorrowedProjectedPackOperation::from_source(&source,spec);let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(expected.len(),65536).unwrap();let mut canceled=|event:semio_framework_value::native_encoding::NativeEncodeProgress|!(event.total==bytes&&event.completed>=256);let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut canceled);
 let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=capsule.write_body(&options,&mut output,&mut control).unwrap_err().into_value_error();let result=(error.kind,match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() });drop(error);result});assert_eq!(kind,ValueRefusalKind::Canceled);assert_eq!(requested-diagnostic,capsule.allocated_bytes()+output.allocated_bytes());assert_eq!(released,diagnostic);assert!(output.len()>0&&output.len()<expected.len());assert!(output.iter().eq(expected[..output.len()].iter().copied()));assert_eq!((source.bytes.as_ptr(),source.text.as_ptr()),pointers);finish(&mut output);
 use semio_framework_value::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};let mut parent=ParentAllocationReturn::<64>::try_new(4096,65536).unwrap();let scratch=capsule.allocated_bytes();assert!(scratch>0);
 for(items,bytes)in[(0,4096),(1,0)]{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.return_one(&mut parent,items,bytes).unwrap());assert_eq!((step,requested,released),((false,0),0,0));assert_eq!(capsule.allocated_bytes(),scratch);assert!(parent.terminal_is_empty());}
 for _ in 0..32768{let((progress,returned),requested,released)=crate::test_allocation::observe_backing(||capsule.return_one(&mut parent,1,1).unwrap());assert_eq!((requested,released,returned),(0,0,0));assert_eq!(capsule.allocated_bytes(),scratch);assert!(parent.terminal_is_empty());if !progress{break}}
 let mut returned=0;for _ in 0..32768{let before=capsule.allocated_bytes();let((progress,bytes),requested,released)=crate::test_allocation::observe_backing(||capsule.return_one(&mut parent,1,4096).unwrap());assert_eq!((requested,released),(0,0));assert!(bytes<=4096);assert_eq!(before-capsule.allocated_bytes(),bytes);returned+=bytes;assert_eq!(parent.retained_bytes(),returned);if !progress{break}}assert_eq!(returned,scratch);assert_eq!(capsule.allocated_bytes(),0);
 let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||parent.close_step(1,4096));assert_eq!(requested,0);match step{AllocationReturnStep::Complete=>{assert_eq!(released,0);break},AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released<=4096);assert_eq!(released,released_bytes);disposed+=released;}}}assert!(parent.terminal_is_empty());assert_eq!(disposed,scratch);
 eprintln!("[DEBUG] actual static borrowed Pack source8194 exact literal wire, original pointers retained, same cumulative65536 pays scratch+output, reused emission requests0, fixed4096 admission refuses with retained scratch, every1/4096 scratch retirement independently observed");
}

#[test]
fn borrowed_static_projected_pack_nested_authored_shapes_match_original_codec(){
 use super::BorrowedProjectedPackOperation;
 use semio_framework_dsl_record::{BorrowedDslRecord,Wire};
 use semio_framework_dsl_record_derive::DslRecord;
 use std::collections::BTreeMap;
 #[derive(DslRecord)]struct Row{active:Option<bool>,name:String,count:u64}
 #[derive(DslRecord)]struct Owner{signed:Vec<i64>,unsigned:Vec<u64>,wide:Vec<u64>,floats:Vec<f64>,tuple:[f64;3],missing:Option<String>,nested:Box<Row>,#[dsl(table)]rows:Vec<Row>,map:BTreeMap<String,DslValue>,value:DslValue,wire:Wire}
 let owner=Owner{signed:vec![i64::MIN,-1,0,i64::MAX],unsigned:vec![0,127,128,i64::MAX as u64],wide:vec![u64::MAX],floats:vec![-0.0,1.5],tuple:[1.0,2.0,3.0],missing:None,nested:Box::new(Row{active:Some(false),name:"same".into(),count:17}),rows:vec![Row{active:Some(true),name:"same".into(),count:1},Row{active:None,name:"other".into(),count:u64::MAX}],map:BTreeMap::from([("z".into(),DslValue::String("same".into())),("a".into(),DslValue::uint(u64::MAX))]),value:DslValue::Object(vec![("z".into(),DslValue::Array(vec![DslValue::uint(u64::MAX),DslValue::int(i64::MIN),DslValue::float(-0.0)])),("a".into(),DslValue::String("same".into()))]),wire:Wire(WireValue{from:WireNode{id:"from".into(),kind:Some("node".into()),port:None},edge:Some((true,WireNode{id:"to".into(),kind:None,port:Some("port".into())})),edge_label:WireEdgeLabel{id:Some("edge".into()),kind:Some("kind".into())},properties:DslValue::Object(vec![("weight".into(),DslValue::float(-0.0))])})};
 let spec=Owner::__dsl_spec();let record=owner.__dsl_to_record();let options=EncodeOptions::default();let expected=encode_record_body(&spec,&record,&options).unwrap();let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(32768,65536).unwrap();
 let mut capsule=BorrowedProjectedPackOperation::from_source(&owner,Owner::RECORD);let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut accepted);
 let(result,requested,released)=crate::test_allocation::observe_backing(||capsule.write_body(&options,&mut output,&mut control));assert_eq!(result.unwrap(),expected.len());assert_eq!(requested,capsule.allocated_bytes()+output.allocated_bytes());assert_eq!(released,0);assert_eq!(requested,control.owned_bytes());assert!(output.iter().eq(expected.iter().copied()));
 let mut accepted=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut accepted);assert_eq!(decode_record_body_span_exact_controlled(protocol::ByteSpan::from_source(&output),&spec,&Default::default(),&mut decoding).unwrap(),record);finish(&mut output);
 let scratch=capsule.allocated_bytes();let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,step.2);assert!(step.1<=1&&released<=4096);disposed+=released;if !step.0{break}}assert_eq!(disposed,scratch);assert_eq!(capsule.allocated_bytes(),0);
 eprintln!("[DEBUG] actual static borrowed nested Record/table sparseBool/forcedText/map keys/intrinsic object/numeric words/Wire exactly match original codec with caller retained paid sort ranges and bounded observed retirement");
}

#[test]
fn borrowed_static_projected_pack_statement_ref_and_unsorted_source_match_original(){
 use super::BorrowedProjectedPackOperation;
 use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as S,BorrowedShape as B};
 use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V};
 struct Source{reference:String,text:String}
 impl FieldProjectionSource for Source{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[9,3,1])),[0]=>Ok(V::Statements(1)),[0,0]=>Ok(V::Record(&[10])),[0,0,0]=>Ok(V::Text(&self.text)),[1]=>Ok(V::List(1)),[1,0]=>Ok(V::Record(&[4])),[1,0,0]=>Ok(V::Text(&self.reference)),[2]=>Ok(V::Map(2)),[2,0]=>Ok(V::UInt(17)),[2,1]=>Ok(V::UInt(23)),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
  fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match(path,index){([0],0)=>Ok("say"),([2],0)=>Ok("z"),([2],1)=>Ok("a"),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
 }
 static ROW:[F;1]=[F::new(4,"reference",B::Ref("node"))];static STATEMENT:[F;1]=[F::new(10,"text",B::Text)];
 fn row()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&ROW}}fn statement()->S{S{keyword:Some("say"),layout:RecordLayout::Inline,fields:&STATEMENT}}fn uint()->B{B::UInt}
 static VARIANTS:[(&str,fn()->S);1]=[("say",statement)];static FIELDS:[F;3]=[F::new(9,"statements",B::Statements(&VARIANTS)),F::new(3,"rows",B::Table(row)),F::new(1,"map",B::Map(uint))];
 fn row_owned()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(4,"reference",Shape::Ref("node".into()))])}
 fn statement_owned()->RecordSpec{RecordSpec::new(Some("say".into()),RecordLayout::Inline,vec![FieldSpec::new(10,"text",Shape::Text)])}
 let producer=|ordinary:fn()->RecordSpec|RecordSpecProducer{ordinary,encoding:|_|Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"ordinary oracle only")),decoding:|_|Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"ordinary oracle only"))};
 let source=Source{reference:"x".repeat(129),text:"say".into()};let pointer=source.reference.as_ptr();let ordinary=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(9,"statements",Shape::Statements(vec![("say".into(),producer(statement_owned))])),FieldSpec::new(3,"rows",Shape::Table(producer(row_owned))),FieldSpec::new(1,"map",Shape::Map(Box::new(Shape::UInt)))]);
 let mut row=RecordValue::default();row.fields.insert(4,FieldValue::Text(source.reference.clone()));let mut statement=RecordValue::default();statement.fields.insert(10,FieldValue::Text(source.text.clone()));let mut record=RecordValue::default();record.fields.insert(9,FieldValue::Statements(vec![("say".into(),statement)]));record.fields.insert(3,FieldValue::List(vec![FieldValue::Record(row)]));record.fields.insert(1,FieldValue::Map(vec![("z".into(),FieldValue::UInt(17)),("a".into(),FieldValue::UInt(23))]));
 let options=EncodeOptions::default();let expected=encode_record_body(&ordinary,&record,&options).unwrap();let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(16384,65536).unwrap();let mut capsule=BorrowedProjectedPackOperation::from_source(&source,S{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS});let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut accepted);
 let(result,requested,released)=crate::test_allocation::observe_backing(||capsule.write_body(&options,&mut output,&mut control));assert_eq!(result.unwrap(),expected.len());assert_eq!(requested,capsule.allocated_bytes()+output.allocated_bytes());assert_eq!(requested,control.owned_bytes());assert_eq!(released,0);assert!(output.iter().eq(expected.iter().copied()));assert_eq!(source.reference.as_ptr(),pointer);finish(&mut output);
 let scratch=capsule.allocated_bytes();let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,step.2);assert!(step.1<=1&&released<=4096);disposed+=released;if !step.0{break}}assert_eq!(disposed,scratch);assert_eq!(capsule.allocated_bytes(),0);
 eprintln!("[DEBUG] actual static borrowed statement forced keyword, single129-byte forcedRef column and unsorted original field/map ordinals exactly match original Pack; all paid canonical sort ranges remain in caller until observed1/4096 retirement");
}

#[test]
fn borrowed_static_projected_pack_actual_derived_variant_source_matches_original(){
 use super::BorrowedProjectedPackOperation;
 use semio_framework_dsl_record::{DslVariants,BorrowedDslVariants};
 use semio_framework_dsl_record_derive::{DslRecord,DslEnum};
 #[derive(DslRecord)]#[dsl(keyword="payload")]struct Data{value:String}
 #[derive(DslEnum)]enum Mutation{Text{value:String},Empty,Data(Data)}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️source-bound-pack/🔣️.json")).unwrap();assert_eq!(fixture["inlineMaximumAllocationBytes"],512);assert_eq!(fixture["inlineLocatorCapacity"],1);assert_eq!(fixture["inlineIndexCapacity"],1);assert_eq!(fixture["maximumInlineStorageBytes"],1024);
 let original=Mutation::Text{value:"ü".repeat(4097)};let delegated=Mutation::Data(Data{value:"same".into()});let empty=Mutation::Empty;
 for operation in[&original,&delegated,&empty]{
  let(keyword,record)=operation.to_named_record();let variants=Mutation::variants();let ordinal=variants.iter().position(|(tag,_)|tag==&keyword).unwrap();let ordinary=(variants[ordinal].1.ordinary)();let(expected_keyword,expected_ordinal,borrowed)=operation.projected_borrowed_variant_identity();assert_eq!(expected_keyword,keyword);assert_eq!(expected_ordinal,ordinal);if matches!(operation,Mutation::Data(_)){assert_eq!(borrowed.keyword,Some("payload"));}
  let options=EncodeOptions::default();let expected=encode_record_body(&ordinary,&record,&options).unwrap();let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(16384,65536).unwrap();let(mut capsule,requested,released)=crate::test_allocation::observe_backing(||BorrowedProjectedPackOperation::from_variant(operation));assert_eq!((requested,released,capsule.allocated_bytes()),(0,0,0));assert_eq!(capsule.variant_identity().unwrap(),(expected_keyword,expected_ordinal));let inline=capsule.inline_storage_bytes();assert!(inline>=std::mem::size_of::<super::SourceTextLocator>()&&inline<=1024);let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut accepted);
  let mut measurement=protocol::io::binary::operation_bytes::OperationByteMeasurement::new(options.limits.max_file_len);let(result,requested,released)=crate::test_allocation::observe_backing(||control.scoped_maximum(512,|control|capsule.write_body(&options,&mut measurement,control)));assert_eq!(result.unwrap(),expected.len());assert_eq!((requested,released,capsule.allocated_bytes(),control.owned_bytes()),(0,0,0,0));assert_eq!(capsule.inline_storage_bytes(),inline);
  let(result,requested,released)=crate::test_allocation::observe_backing(||capsule.write_body(&options,&mut output,&mut control));assert_eq!(result.unwrap(),expected.len());assert_eq!(requested,capsule.allocated_bytes()+output.allocated_bytes());assert_eq!(requested,control.owned_bytes());assert_eq!(released,0);assert!(output.iter().eq(expected.iter().copied()));finish(&mut output);
  let scratch=capsule.allocated_bytes();let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,step.2);assert!(step.1<=1&&released<=4096);disposed+=released;if !step.0{break}}assert_eq!(disposed,scratch);assert_eq!(capsule.allocated_bytes(),0);assert_eq!(capsule.variant_identity().unwrap_err().kind(),semio_framework_value::ValueRefusalKind::InvariantViolated);eprintln!("[DEBUG] actual variant ordinal{expected_ordinal} caller-embedded scratch bytes{inline}; constructor and512 measurement requests0/releases0; independently measured1/4096 settlement");
 }
 let Mutation::Text{value}=&original else{unreachable!()};assert_eq!(value.len(),8194);
 eprintln!("[DEBUG] actual derived named/Unit/delegated variant borrowed static metadata and original source match canonical ordinary bodies with all scratch retained and independently retired1/4096");
}

#[test]
fn borrowed_static_projected_pack_owned_variant_retains_source_until_scratch_settles(){
 use super::BorrowedProjectedPackOperation;use semio_framework_dsl_record::{DslVariants,BorrowedDslVariants};use semio_framework_dsl_record_derive::{DslRecord,DslEnum};
 #[derive(DslRecord)]struct Data{value:String,label:String}
 #[derive(DslEnum)]enum Mutation{Data(Data)}
 for mode in ["complete","initial4096"]{
  let source=Mutation::Data(Data{value:"ü".repeat(4097),label:"same".into()});let Mutation::Data(value)=&source;let pointer=value.value.as_ptr();assert_eq!(value.value.len(),8194);let(keyword,record)=source.to_named_record();let variants=Mutation::variants();let expected=encode_record_body(&(variants[0].1.ordinary)(),&record,&Default::default()).unwrap();let identity=source.projected_borrowed_variant_identity();assert_eq!(identity.0,keyword);
  let mut capsule=BorrowedProjectedPackOperation::from_owned_variant(source);assert_eq!(capsule.variant_identity().unwrap(),(identity.0,identity.1));capsule=match capsule.take_settled_source(){Err(retained)=>retained,Ok(_)=>panic!("fresh source escaped before scratch settlement")};
  let mut output=protocol::io::binary::operation_bytes::OperationByteMeasurement::new(65536);let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(if mode=="initial4096"{4096}else{65536},&mut allow);
  let((result,diagnostic),requested,released)=crate::test_allocation::observe_backing(||match capsule.write_body(&Default::default(),&mut output,&mut control){Ok(length)=>(Ok(length),0),Err(error)=>{let error=error.into_value_error();let result=(Err(error.kind),match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() });drop(error);result}});assert_eq!(requested-diagnostic,capsule.allocated_bytes());assert_eq!(released,diagnostic);assert!(capsule.allocated_bytes()>0);if mode=="complete"{assert_eq!(result.unwrap(),expected.len());assert_eq!(output.exact_length().unwrap(),expected.len())}else{assert_eq!(result.unwrap_err(),semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(output.exact_length().unwrap(),0)}
  capsule=match capsule.take_settled_source(){Err(retained)=>retained,Ok(_)=>panic!("source escaped with paid canonical backing")};let Mutation::Data(value)=capsule.retained_source();assert_eq!(value.value.as_ptr(),pointer);assert_eq!(value.value.len(),8194);
  let retained=capsule.allocated_bytes();let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,step.2);assert!(step.1<=1&&released<=4096);disposed+=released;if !step.0{break}}assert_eq!(disposed,retained);assert_eq!(capsule.allocated_bytes(),0);let source=match capsule.take_settled_source(){Ok(source)=>source,Err(_)=>panic!("settled canonical scratch withheld original source")};let Mutation::Data(value)=&source;assert_eq!(value.value.as_ptr(),pointer);assert_eq!(value.value.len(),8194);
 }
 eprintln!("[DEBUG] actual owned canonical variant keeps original8194 source pointer through success/refusal, refuses source transfer until scratch settles, measures every1/4096 retirement, then transfers genuine typed source without semantic retirement credit");
}
