mod owned_media_error_tests {
    use super::*;

    #[test]
    fn intrinsic_media_wire_artifact_helpers_preserve_descriptor_full_value_and_refusals(){
        use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,Number,ValueRefusalKind};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🎒️pack/🌱️value/🧫️fixtures/🎞️intrinsic-media/🔣️.json")).unwrap();
        let mut values=vec![DslValue::Null,DslValue::Bool(true),DslValue::uint(u64::MAX),DslValue::int(i64::MIN)];
        values.extend(fixture["binary64Words"].as_array().unwrap().iter().map(|word|DslValue::float(f64::from_bits(u64::from_str_radix(word.as_str().unwrap(),16).unwrap()))));
        values.extend([DslValue::String("\0literal ä é 🗺️".into()),DslValue::Bytes(vec![0,255,128,127,10,0]),DslValue::Array(vec![]),DslValue::Object(vec![])]);
        let value=DslValue::Object(vec![("z".into(),DslValue::Array(values)),("a".into(),DslValue::String("first".into())),("z".into(),DslValue::String("duplicate".into())),("\0ä".into(),DslValue::Object(vec![("second".into(),DslValue::Bool(false)),("first".into(),DslValue::uint(0))]))]);
        let media_type=MediaType{class:MediaClass::Data,form:MediaForm::Value};let schema="schema\0ä";let port="port\0🗺️";
        let media=Media{media_type,payload:MediaPayload::Intrinsic{schema:schema.into(),value}};let expected=MediaFingerprint::of(&media);
        let MediaPayload::Intrinsic{value,..}=&media.payload else{panic!("intrinsic fixture")};let mut accept=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut accept);
        let mut artifact=MediaArtifact::encode_intrinsic(port,media_type,schema,value,&mut output).unwrap();assert_eq!(artifact.descriptor.port_id.as_deref(),Some(port));assert!(matches!(&artifact.descriptor.wire,MediaWireFormat::Intrinsic{schema:actual} if actual==schema));
        let wire=semio_framework_value::ToValue::to_value(&artifact.descriptor);let descriptor=<MediaArtifactDescriptor as semio_framework_value::FromValue>::from_value(wire).unwrap();assert_eq!(descriptor.wire,artifact.descriptor.wire);assert_eq!(descriptor.media_type,Some(media_type));assert_eq!(descriptor.port_id.as_deref(),Some(port));
        let mut decode_accept=|_|true;let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_accept);let decoded=artifact.decode_intrinsic(&mut input).unwrap();assert_eq!(MediaFingerprint::of(&decoded),expected);
        let MediaPayload::Intrinsic{value:DslValue::Object(members),..}=decoded.payload else{panic!("full intrinsic owner")};assert_eq!(members.iter().map(|member|member.0.as_str()).collect::<Vec<_>>(),vec!["z","a","z","\0ä"]);let DslValue::Array(items)=&members[0].1 else{panic!("full intrinsic variants")};assert!(matches!(items[2],DslValue::Number(Number::UInt(u64::MAX))));assert!(matches!(items[3],DslValue::Number(Number::Int(i64::MIN))));
        for(actual,word)in items[4..13].iter().zip(fixture["binary64Words"].as_array().unwrap()){let DslValue::Number(Number::Float(number))=actual else{panic!("raw float")};assert_eq!(format!("{:016x}",number.to_bits()),word.as_str().unwrap());}
        let mut deny=|_|false;let mut output=NativeEncodeControl::new(usize::MAX,&mut deny);let error=MediaArtifact::encode_intrinsic(port,media_type,schema,value,&mut output).unwrap_err();assert!(matches!(&error,MediaArtifactError::Value(error) if error.kind==ValueRefusalKind::Canceled));assert!(std::error::Error::source(&error).unwrap().downcast_ref::<semio_framework_value::ValueError>().is_some());
        artifact.descriptor.media_type=None;let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_accept);assert!(matches!(artifact.decode_intrinsic(&mut input),Err(MediaArtifactError::Value(error)) if error.kind==ValueRefusalKind::InvalidValue));
        artifact.descriptor.media_type=Some(media_type);artifact.descriptor.blob_hash=Some("blob".into());let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_accept);assert!(matches!(artifact.decode_intrinsic(&mut input),Err(MediaArtifactError::Value(error)) if error.kind==ValueRefusalKind::InvalidValue));
        artifact.descriptor.blob_hash=None;artifact.data.push(0);let mut input=NativeDecodeControl::new(usize::MAX,&mut decode_accept);assert!(matches!(artifact.decode_intrinsic(&mut input),Err(MediaArtifactError::Value(error)) if error.kind==ValueRefusalKind::InvalidValue));
    }

    #[semio_framework_async_macros::async_test]
    async fn owned_errors_preserve_their_messages() {
        assert_eq!(MediaArtifactError::Payload("payload".into()).to_string(), "payload");
        assert_eq!(MediaArtifactError::SchemaMismatch { expected: "a".into(), found: "b".into() }.to_string(), "document schema mismatch: expected a, found b");
        assert_eq!(MediaArtifactError::NoImporter("raw".into()).to_string(), "no binary importer registered for format \"raw\"");
    }
}
