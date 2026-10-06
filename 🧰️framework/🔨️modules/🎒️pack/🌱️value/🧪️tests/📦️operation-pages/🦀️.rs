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
