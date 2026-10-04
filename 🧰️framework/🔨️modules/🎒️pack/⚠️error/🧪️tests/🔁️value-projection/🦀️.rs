#[test]
fn canonical_value_projection_moves_semantic_causes_and_returns_original_transports(){
    use crate::{PackError,PackRefusal,PackRetryDisposition,PackTransportCategory,TransportAdmission,TransportCaptureRefusal,reserve_transport};
    use semio_framework_diagnostic::{TextError,TextSpan};
    use semio_framework_value::{ValueError,ValueRefusalKind};
    use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️value-projection/🔣️.json")).unwrap();
    let kinds=[ValueRefusalKind::InvalidValue,ValueRefusalKind::Canceled,ValueRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed,ValueRefusalKind::WorkLimit,ValueRefusalKind::DepthLimit,ValueRefusalKind::UnsupportedOwner,ValueRefusalKind::InvariantViolated];
    let message=fixture["ownedMessage"].as_str().unwrap();
    for(kind,token)in kinds.into_iter().zip(fixture["semanticKinds"].as_array().unwrap()){
        assert_eq!(kind.as_str(),token.as_str().unwrap());
        let owned=[PackRefusal::ValueRefusal(ValueError::new(kind,message)),PackRefusal::TextRefusal(TextError::from_value_error(ValueError::new(kind,message),TextSpan::at(3,7))),PackRefusal::Io{error:ValueError::new(kind,message),retry:PackRetryDisposition::Never},PackRefusal::Io{error:ValueError::new(kind,message),retry:PackRetryDisposition::Transient}];
        for(error,variant)in owned.into_iter().zip(fixture["ownedVariants"].as_array().unwrap()){
            let pointer=match &error{PackRefusal::ValueRefusal(error)|PackRefusal::Io{error,..}=>error.message.as_ptr(),PackRefusal::TextRefusal(error)=>error.message.as_ptr(),_=>panic!("declared owned cause")};
            let wrapped=PackError::from(error.clone());let PackError::Refusal(ref inner)=wrapped else{panic!("statically semantic source")};let outer_pointer=match inner{PackRefusal::ValueRefusal(error)|PackRefusal::Io{error,..}=>error.message.as_ptr(),PackRefusal::TextRefusal(error)=>error.message.as_ptr(),_=>panic!("declared wrapped cause")};
            let(result,bytes)=crate::test_allocation::observe(||error.into_value_error());let value:ValueError=result;
            assert_eq!(bytes,usize::try_from(fixture["contract"]["nativeOwnedProjectionBytes"].as_u64().unwrap()).unwrap());assert_eq!(value.message.as_ptr(),pointer);
            let output=serde_json::json!({"variant":variant,"kind":value.kind.as_str(),"message":value.message});let oracle=serde_json::json!({"variant":variant,"kind":token,"message":fixture["ownedMessage"]});assert_eq!(output,oracle);
            let(result,bytes)=crate::test_allocation::observe(||wrapped.into_value_error());let value=result.expect("declared wrapped semantic projection");assert_eq!(value.message.as_ptr(),outer_pointer);assert_eq!(bytes,0);assert_eq!(serde_json::json!({"variant":variant,"kind":value.kind.as_str(),"message":value.message}),oracle);
        }
        let value=PackRefusal::LimitExceeded{kind,limit:"same misleading canceled prose"}.into_value_error();assert_eq!(serde_json::json!({"kind":value.kind.as_str(),"message":value.message}),serde_json::json!({"kind":token,"message":fixture["structuralMessage"]}));
        let refusal=TransportCaptureRefusal{kind,reason:"same misleading canceled prose",allocated_bytes:usize::try_from(fixture["admission"]["retainedBytes"].as_u64().unwrap()).unwrap(),physical_bytes:usize::try_from(fixture["admission"]["physicalBytes"].as_u64().unwrap()).unwrap()};let error=PackRefusal::TransportAdmission{category:PackTransportCategory::NativeIo,refusal};assert_eq!(error.kind(),kind);assert_eq!(std::error::Error::source(&error).unwrap().downcast_ref::<TransportCaptureRefusal>().unwrap(),&refusal);let value=error.into_value_error();assert_eq!(serde_json::json!({"kind":value.kind.as_str(),"message":value.message}),serde_json::json!({"kind":token,"message":fixture["admission"]["message"]}));
    }
    #[derive(Debug)]
    struct Source(Arc<AtomicUsize>);
    impl std::fmt::Display for Source{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.write_str("same misleading canceled prose")}}
    impl std::error::Error for Source{}
    impl Drop for Source{fn drop(&mut self){self.0.fetch_add(1,Ordering::SeqCst);}}
    for row in fixture["transportCases"].as_array().unwrap(){
        let category=match row["category"].as_str().unwrap(){"nativeIo"=>PackTransportCategory::NativeIo,"httpRequest"=>PackTransportCategory::HttpRequest,"httpBody"=>PackTransportCategory::HttpBody,_=>panic!("declared transport category")};let retry=match row["retry"].as_str().unwrap(){"never"=>PackRetryDisposition::Never,"transient"=>PackRetryDisposition::Transient,_=>panic!("declared retry")};
        let drops=Arc::new(AtomicUsize::new(0));let mut admission=TransportAdmission::new(4096,0);let reservation=reserve_transport::<Source,_>(&mut admission,category,retry,|_,_|true).unwrap();let allocated=reservation.allocated_bytes();let source=reservation.publish(Source(drops.clone()));let pointer=source.source_error()as*const _ as*const();
        let(result,bytes)=crate::test_allocation::observe(||PackError::TransportFailure(source).into_value_error());let original=result.expect_err("transport has no semantic refusal authority");assert_eq!(bytes,usize::try_from(fixture["contract"]["nativeTransportProjectionBytes"].as_u64().unwrap()).unwrap());assert_eq!(original.refusal_kind(),None);assert_eq!(std::error::Error::source(&original).unwrap()as*const _ as*const(),pointer);
        let PackError::TransportFailure(source)=original else{panic!("original transport variant")};assert_eq!(source.category(),category);assert_eq!(source.retry(),retry);assert_eq!(source.allocated_bytes(),allocated);assert!(source.source_error().downcast_ref::<Source>().is_some());assert_eq!(source.to_string(),message);assert_eq!(drops.load(Ordering::SeqCst),0);drop(source);assert_eq!(drops.load(Ordering::SeqCst),1);assert_eq!(admission.committed_bytes(),allocated);
    }
    eprintln!("[DEBUG] canonical Value projection40 semantic,8 admission and6 transport vectors preserve static semantic authority or original transport source; owned/refused native projections allocate zero bytes");
}
