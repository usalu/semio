//! 🎞️ Neutral full-value wire laws preserve actual ordered occurrences and literal words.

#[test]
fn intrinsic_media_wire_neutral_materialization_uses_observed_physical_allowances(){
    use semio_framework_value::{NativeDecodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️wire-materialization/🔣️.json")).unwrap();
    let octets=|text:&str|text.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect::<Vec<_>>();
    for row in fixture["cases"].as_array().unwrap(){
        let bytes=octets(row["rawHex"].as_str().unwrap());let limits=&row["limits"];let mut options=DecodeOptions::default();
        options.limits.max_file_len=limits["maxFileLen"].as_u64().unwrap();options.limits.max_segment_len=limits["maxSegmentLen"].as_u64().unwrap();options.limits.max_symbols=u32::try_from(limits["maxSymbols"].as_u64().unwrap()).unwrap();options.limits.max_depth=u16::try_from(limits["maxDepth"].as_u64().unwrap()).unwrap();options.limits.max_items=limits["maxItems"].as_u64().unwrap();options.limits.max_total_alloc=u64::MAX;
        let mut allow=|_|true;let mut broad=NativeDecodeControl::new(usize::MAX,&mut allow);
        let ordinary=decode_value_record_body_exact(&bytes,1,&options.limits);let controlled=decode_value_record_body_exact_controlled(&bytes,1,&options,&mut broad);
        match row["grammar"]["outcome"].as_str().unwrap(){
            "accepted"=>{assert_eq!(serde_json::to_value(ordinary.unwrap()).unwrap(),row["grammar"]["value"]);assert_eq!(serde_json::to_value(controlled.unwrap()).unwrap(),row["grammar"]["value"]);},
            "limit"=>{assert!(matches!(ordinary,Err(PackRefusal::LimitExceeded { .. })),"{}",row["id"]);assert!(matches!(controlled,Err(PackRefusal::LimitExceeded { .. })),"{}",row["id"]);},
            "malformed"=>{assert!(matches!(ordinary,Err(PackRefusal::Malformed{..})),"{}: {ordinary:?}",row["id"]);assert!(matches!(controlled,Err(PackRefusal::Malformed{..})),"{}: {controlled:?}",row["id"]);},
            "truncated"=>{let expected=row["grammar"]["offset"].as_u64().unwrap();assert_eq!(expected,bytes.len()as u64);assert!(matches!(ordinary,Err(PackRefusal::Truncated(offset))if offset==expected),"{}: {ordinary:?}",row["id"]);assert!(matches!(controlled,Err(PackRefusal::Truncated(offset))if offset==expected),"{}: {controlled:?}",row["id"]);},
            _=>panic!("closed grammar outcome"),
        }
        let mode=row["physicalAllowance"]["mode"].as_str().unwrap();if mode=="unbounded"{continue;}
        let probe=row["allowanceProbeHex"].as_str().map(octets);let specimen=probe.as_deref().unwrap_or(&bytes);let mut probe_options=options.clone();if probe.is_some(){probe_options.limits.max_file_len=specimen.len()as u64;}
        let mut native=NativeDecodeControl::new(usize::MAX,&mut allow);
        let (decoded,requested)=crate::test_allocation::observe(||decode_value_record_body_exact_controlled(specimen,1,&probe_options,&mut native));
        let decoded=decoded.expect("successful physical allowance specimen");assert_eq!(native.owned_bytes(),requested,"{}",row["id"]);
        let expected=if probe.is_some(){serde_json::Value::Null}else{row["grammar"]["value"].clone()};assert_eq!(serde_json::to_value(&decoded).unwrap(),expected);
        let (ordinary,ordinary_requested)=crate::test_allocation::observe(||decode_value_record_body_exact(specimen,1,&probe_options.limits));assert_eq!(serde_json::to_value(ordinary.unwrap()).unwrap(),expected);assert_eq!(ordinary_requested,requested,"same complete ordinary slot backing");
        let maximum=match mode{"zero"=>{assert_eq!(requested,0);0},"exact"=>requested,"oneByteShort"=>requested.checked_sub(1).expect("short allowance requires positive backing"),_=>panic!("closed physical allowance")};
        options.limits.max_total_alloc=maximum as u64;let ordinary=decode_value_record_body_exact(&bytes,1,&options.limits);let mut native=NativeDecodeControl::new(maximum,&mut allow);let controlled=decode_value_record_body_exact_controlled(&bytes,1,&options,&mut native);
        match row["expect"]["outcome"].as_str().unwrap(){
            "accepted"=>{assert_eq!(serde_json::to_value(ordinary.unwrap()).unwrap(),row["expect"]["value"]);assert_eq!(serde_json::to_value(controlled.unwrap()).unwrap(),row["expect"]["value"]);assert_eq!(native.owned_bytes(),requested);},
            "limit"=>{assert!(matches!(ordinary,Err(PackRefusal::LimitExceeded { .. })),"{}",row["id"]);assert!(matches!(controlled,Err(PackRefusal::ValueRefusal(error))if error.kind==ValueRefusalKind::OwnershipLimit),"{}",row["id"]);},
            _=>panic!("closed physical outcome"),
        }
    }
}

#[test]
fn intrinsic_media_wire_direct_helpers_settle_complete_allocator_requests(){
    use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎞️intrinsic-media/🔣️.json")).unwrap();
    let depths:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎞️intrinsic-media/🌲️depth/🔣️.json")).unwrap();
    let word=u64::from_str_radix(depths["leafWord"].as_str().unwrap(),16).unwrap();
    let mut cases=vec![literal(&fixture["value"])];
    for row in depths["cases"].as_array().unwrap().iter().filter(|row|row["accepted"]==true){
        let mut value=DslValue::float(f64::from_bits(word));for _ in 0..row["edges"].as_u64().unwrap(){value=DslValue::Array(vec![value]);}cases.push(value);
    }
    let mut options=EncodeOptions::default();options.limits.max_depth=256;
    let decoding=DecodeOptions{limits:options.limits.clone(),..Default::default()};
    for value in &cases{
        let expected=neutral(value);let mut allow=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut allow);
        let (encoded,requested)=crate::test_allocation::observe(||encode_value_record_body_controlled(1,value,&options,&mut output));
        let bytes=encoded.expect("actual borrowed intrinsic producer");assert_eq!(output.owned_bytes(),requested,"complete producer backing requests");
        let mut exact=NativeEncodeControl::new(requested,&mut allow);
        let (encoded,replayed)=crate::test_allocation::observe(||encode_value_record_body_controlled(1,value,&options,&mut exact));
        assert_eq!(encoded.unwrap(),bytes);assert_eq!(replayed,requested);assert_eq!(exact.owned_bytes(),requested);
        if requested>0{
            let mut short=NativeEncodeControl::new(requested-1,&mut allow);assert!(matches!(encode_value_record_body_controlled(1,value,&options,&mut short),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::OwnershipLimit));
            let mut cumulative=NativeEncodeControl::new(requested*2-1,&mut allow);assert_eq!(encode_value_record_body_controlled(1,value,&options,&mut cumulative).unwrap(),bytes);assert!(matches!(encode_value_record_body_controlled(1,value,&options,&mut cumulative),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::OwnershipLimit));
        }
        let mut accept=|_|true;let mut input=NativeDecodeControl::new(usize::MAX,&mut accept);
        let (decoded,requested)=crate::test_allocation::observe(||decode_value_record_body_exact_controlled(&bytes,1,&decoding,&mut input));
        assert_eq!(neutral(&decoded.expect("actual complete intrinsic reconstruction")),expected);assert_eq!(input.owned_bytes(),requested,"complete decoder backing requests");
        let mut exact=NativeDecodeControl::new(requested,&mut accept);
        let (decoded,replayed)=crate::test_allocation::observe(||decode_value_record_body_exact_controlled(&bytes,1,&decoding,&mut exact));
        assert_eq!(neutral(&decoded.unwrap()),expected);assert_eq!(replayed,requested);assert_eq!(exact.owned_bytes(),requested);
        if requested>0{
            let mut short=NativeDecodeControl::new(requested-1,&mut accept);assert!(matches!(decode_value_record_body_exact_controlled(&bytes,1,&decoding,&mut short),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::OwnershipLimit));
            let mut cumulative=NativeDecodeControl::new(requested*2-1,&mut accept);assert_eq!(neutral(&decode_value_record_body_exact_controlled(&bytes,1,&decoding,&mut cumulative).unwrap()),expected);assert!(matches!(decode_value_record_body_exact_controlled(&bytes,1,&decoding,&mut cumulative),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::OwnershipLimit));
        }
    }
}

use super::*;

fn literal(value:&serde_json::Value)->DslValue{
    let body=&value["value"];
    match value["kind"].as_str().unwrap(){
        "null"=>DslValue::Null,"bool"=>DslValue::Bool(body.as_bool().unwrap()),
        "uint"=>DslValue::uint(body.as_str().unwrap().parse().unwrap()),"int"=>DslValue::int(body.as_str().unwrap().parse().unwrap()),
        "float"=>DslValue::float(f64::from_bits(u64::from_str_radix(body.as_str().unwrap(),16).unwrap())),"string"=>DslValue::String(body.as_str().unwrap().into()),
        "bytes"=>DslValue::Bytes(body.as_str().unwrap().as_bytes().chunks_exact(2).map(|word|u8::from_str_radix(std::str::from_utf8(word).unwrap(),16).unwrap()).collect()),
        "array"=>DslValue::Array(body.as_array().unwrap().iter().map(literal).collect()),
        "object"=>DslValue::Object(body.as_array().unwrap().iter().map(|entry|(entry["key"].as_str().unwrap().into(),literal(&entry["value"]))).collect()),_=>panic!("closed neutral intrinsic kind"),
    }
}
fn neutral(value:&DslValue)->serde_json::Value{
    match value{
        DslValue::Null=>serde_json::json!({"kind":"null"}),DslValue::Bool(value)=>serde_json::json!({"kind":"bool","value":value}),
        DslValue::Number(Number::UInt(value))=>serde_json::json!({"kind":"uint","value":value.to_string()}),DslValue::Number(Number::Int(value))=>serde_json::json!({"kind":"int","value":value.to_string()}),
        DslValue::Number(Number::Float(value))=>serde_json::json!({"kind":"float","value":format!("{:016x}",value.to_bits())}),DslValue::String(value)=>serde_json::json!({"kind":"string","value":value}),
        DslValue::Bytes(value)=>serde_json::json!({"kind":"bytes","value":value.iter().map(|byte|format!("{byte:02x}")).collect::<String>()}),
        DslValue::Array(value)=>serde_json::json!({"kind":"array","value":value.iter().map(neutral).collect::<Vec<_>>()}),
        DslValue::Object(value)=>serde_json::json!({"kind":"object","value":value.iter().map(|(key,value)|serde_json::json!({"key":key,"value":neutral(value)})).collect::<Vec<_>>()}),
    }
}

#[test]
fn intrinsic_media_wire_preserves_actual_order_duplicate_keys_and_full_literal_words(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎞️intrinsic-media/🔣️.json")).unwrap();
    let value=literal(&fixture["value"]);let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);
    let mut record=RecordValue::default();record.fields.insert(1,FieldValue::Value(value));
    let bytes=encode_record_body(&spec,&record,&EncodeOptions::default()).unwrap();
    let decoded=decode_value_record_body_exact(&bytes,1,&PackLimits::default()).unwrap();
    assert_eq!(neutral(&decoded),fixture["value"],"actual owner occurrences and complete scalar words");
    let mut allow=|_|true;let mut control=semio_framework_value::native_encoding::NativeEncodeControl::new(usize::MAX,&mut allow);
    let controlled=encode_record_body_controlled(&spec,&record,&EncodeOptions::default(),&mut control).unwrap();
    assert_eq!(controlled,bytes,"ordinary and controlled grammar");
    let decoded=decode_value_record_body_exact(&controlled,1,&PackLimits::default()).unwrap();assert_eq!(neutral(&decoded),fixture["value"]);
    let independent=serde_json::to_string(&neutral(&decoded)).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&independent).unwrap(),fixture["value"]);
}

#[test]
fn intrinsic_media_wire_direct_helpers_share_semantic_depth_and_typed_control(){
    use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
    let value=DslValue::Array(vec![DslValue::Null]);let mut options=EncodeOptions::default();options.limits.max_depth=1;
    let mut accept=|_|true;let mut decode_accept=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut accept);
    let bytes=encode_value_record_body_controlled(1,&value,&options,&mut output).expect("one intrinsic array edge is semantic depth one");
    let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_accept);let decoding=DecodeOptions{limits:options.limits.clone(),..Default::default()};
    assert_eq!(neutral(&decode_value_record_body_exact_controlled(&bytes,1,&decoding,&mut input).unwrap()),neutral(&value));
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎞️intrinsic-media/🔣️.json")).unwrap();let value=literal(&fixture["value"]);
    let mut output=NativeEncodeControl::new(usize::MAX,&mut accept);let bytes=encode_value_record_body_controlled(1,&value,&EncodeOptions::default(),&mut output).unwrap();let needed=output.owned_bytes();assert!(needed>=bytes.len());
    let mut exact=NativeEncodeControl::new(needed,&mut accept);assert_eq!(encode_value_record_body_controlled(1,&value,&EncodeOptions::default(),&mut exact).unwrap(),bytes);
    let mut short=NativeEncodeControl::new(needed-1,&mut accept);assert!(matches!(encode_value_record_body_controlled(1,&value,&EncodeOptions::default(),&mut short),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::OwnershipLimit));
    let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_accept);let decoded=decode_value_record_body_exact_controlled(&bytes,1,&DecodeOptions::default(),&mut input).unwrap();let needed=input.owned_bytes();assert_eq!(neutral(&decoded),fixture["value"]);
    let mut exact=NativeDecodeControl::new(needed,&mut decode_accept);assert_eq!(neutral(&decode_value_record_body_exact_controlled(&bytes,1,&DecodeOptions::default(),&mut exact).unwrap()),fixture["value"]);
    let mut short=NativeDecodeControl::new(needed-1,&mut decode_accept);assert!(matches!(decode_value_record_body_exact_controlled(&bytes,1,&DecodeOptions::default(),&mut short),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::OwnershipLimit));
    let mut cumulative=NativeDecodeControl::new(needed*2-1,&mut decode_accept);decode_value_record_body_exact_controlled(&bytes,1,&DecodeOptions::default(),&mut cumulative).unwrap();assert!(matches!(decode_value_record_body_exact_controlled(&bytes,1,&DecodeOptions::default(),&mut cumulative),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::OwnershipLimit));
    for malformed in [vec![0,0],vec![0,2],vec![0,1,2,0x11,0x12],vec![0,1,1,0x11,0x12,0]]{let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_accept);assert!(decode_value_record_body_exact_controlled(&malformed,1,&DecodeOptions::default(),&mut input).is_err(),"strict terminal grammar");}
    let mut refuse=|_|false;let mut output=NativeEncodeControl::new(usize::MAX,&mut refuse);assert!(matches!(encode_value_record_body_controlled(1,&value,&EncodeOptions::default(),&mut output),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::Canceled));
    let mut decode_refuse=|_|false;let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_refuse);assert!(matches!(decode_value_record_body_exact_controlled(&bytes,1,&DecodeOptions::default(),&mut input),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::Canceled));
    let text=DslValue::String("ä".repeat(70000));let mut output=NativeEncodeControl::new(usize::MAX,&mut accept);let bytes=encode_value_record_body_controlled(1,&text,&EncodeOptions::default(),&mut output).unwrap();
    let mut stop=|progress:semio_framework_value::native_decoding::NativeDecodeProgress|!(progress.total>=65536&&progress.completed>=65536);let mut input=NativeDecodeControl::new(usize::MAX,&mut stop);assert!(matches!(decode_value_record_body_exact_controlled(&bytes,1,&DecodeOptions::default(),&mut input),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::Canceled));
}

#[test]
fn intrinsic_media_wire_direct_helpers_honor_the_complete_declared_depth_frontier(){
    use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎞️intrinsic-media/🌲️depth/🔣️.json")).unwrap();
    let word=u64::from_str_radix(fixture["leafWord"].as_str().unwrap(),16).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let edges=u16::try_from(row["edges"].as_u64().unwrap()).unwrap();let maximum=u16::try_from(row["maxDepth"].as_u64().unwrap()).unwrap();let accepted=row["accepted"].as_bool().unwrap();
        let mut value=DslValue::float(f64::from_bits(word));for _ in 0..edges{value=DslValue::Array(vec![value]);}
        let expected=neutral(&value);let mut ordinary=EncodeOptions::default();ordinary.limits.max_depth=edges+2;
        let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);let mut record=RecordValue::default();record.fields.insert(1,FieldValue::Value(value));
        let bytes=encode_record_body(&spec,&record,&ordinary).unwrap();let FieldValue::Value(value)=record.fields.get(&1).unwrap() else{panic!("declared Value field")};
        let mut options=EncodeOptions::default();options.limits.max_depth=maximum;let decoding=DecodeOptions{limits:options.limits.clone(),..Default::default()};
        let mut allow=|_|true;let mut decode_allow=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut allow);let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_allow);
        let decoded=decode_value_record_body_exact_controlled(&bytes,1,&decoding,&mut input);
        if accepted{assert_eq!(neutral(&decoded.expect("declared decoder depth")),expected);assert_eq!(encode_value_record_body_controlled(1,value,&options,&mut output).expect("declared encoder depth"),bytes,"semantic edges={edges}, maximum={maximum}");}
        else{assert!(decoded.is_err(),"semantic decoder frontier");assert!(matches!(encode_value_record_body_controlled(1,value,&options,&mut output),Err(PackRefusal::ValueRefusal(error)) if error.kind==ValueRefusalKind::DepthLimit),"semantic encoder frontier");}
    }
}
