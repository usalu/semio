use super::*;

//#region 🔖️Ids
#[semio_framework_async_macros::async_test]
async fn document_id_and_actor_id_convert_and_display() {
    let document: ArtifactId = "doc-1".into();
    assert_eq!(document.to_string(), "doc-1");
    assert_eq!(document, ArtifactId::from("doc-1".to_string()));

    let actor: ActorId = "actor-1".into();
    assert_eq!(actor.to_string(), "actor-1");
}

#[semio_framework_async_macros::async_test]
async fn generation_id_next_is_strictly_monotonic() {
    let g0 = GenerationId::INITIAL;
    let g1 = g0.next();
    let g2 = g1.next();
    assert!(g0 < g1);
    assert!(g1 < g2);
    assert_eq!(g0, GenerationId(0));
    assert_eq!(g2, GenerationId(2));
}
//#endregion 🔖️Ids

//#region 🔖️Errors
#[semio_framework_async_macros::async_test]
async fn pack_error_conversion_preserves_complete_typed_semantic_cause() {
    let refused = pack::PackRefusal::ChecksumMismatch { segment: "manifest", offset: 42 };
    let converted = DbError::from(refused.clone());
    assert_eq!(converted, DbError::Pack(pack::PackError::Refusal(refused)));
    assert!(std::error::Error::source(&converted).unwrap().downcast_ref::<pack::PackError>().is_some());
}
//#endregion 🔖️Errors

//#region 🔖️Limits
#[semio_framework_async_macros::async_test]
async fn check_len_rejects_over_limit_before_any_allocation_would_happen() {
    assert!(check_len(10, 100, "test").is_ok());
    assert_eq!(check_len(101, 100, "test"), Err(DbError::LimitExceeded("test")));
}
//#endregion 🔖️Limits

#[semio_framework_async_macros::async_test]
async fn pack_refusal_preserves_kind_fields_and_original_owned_message() {
    use semio_framework_value::{ValueError,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    fn kind(value:&str)->ValueRefusalKind{match value{"InvalidValue"=>ValueRefusalKind::InvalidValue,"Canceled"=>ValueRefusalKind::Canceled,"OwnershipLimit"=>ValueRefusalKind::OwnershipLimit,"AllocationFailed"=>ValueRefusalKind::AllocationFailed,"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,"UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"InvariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown authored kind")}}
    for row in fixture["cases"].as_array().unwrap(){
        let mut retained=None;
        let error=match row["source"].as_str().unwrap(){
            "ValueRefusal"=>{let mut cause=ValueError::new(kind(row["kind"].as_str().unwrap()),row["message"].as_str().unwrap());for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}retained=Some(cause.message.as_ptr());pack::PackRefusal::ValueRefusal(cause)},
            "LimitExceeded"=>pack::PackRefusal::LimitExceeded { kind: kind(row["kind"].as_str().unwrap()), limit: "too big" },
            "BadMagic"=>pack::PackRefusal::BadMagic,
            "Truncated"=>pack::PackRefusal::Truncated(row["offset"].as_u64().unwrap()),
            _=>panic!("unknown authored source")
        };
        let converted=DbError::from(error);
        let DbError::Pack(pack::PackError::Refusal(cause)) = converted else { panic!("typed semantic cause") };
        assert_eq!(cause.kind(), kind(row["kind"].as_str().unwrap()));
        match cause {
            pack::PackRefusal::ValueRefusal(cause) => { let expected = row["path"].as_array().unwrap().iter().rev().map(|part| part.as_str().unwrap()).chain(std::iter::once(row["message"].as_str().unwrap())).collect::<Vec<_>>().join("."); assert_eq!(serde_json::json!({"message":cause.message}), serde_json::json!({"message":expected})); assert_eq!(cause.message.as_ptr(), retained.unwrap()); },
            pack::PackRefusal::Truncated(offset) => assert_eq!(serde_json::json!({"offset":offset}), serde_json::json!({"offset":row["offset"]})),
            pack::PackRefusal::LimitExceeded { limit, .. } => assert_eq!(limit, row["limit"].as_str().unwrap()),
            pack::PackRefusal::BadMagic => assert_eq!(row["source"], "BadMagic"),
            _ => panic!("unexpected authored refusal"),
        }
    }
}

struct DbTransportPolicy;
impl pack::PackTransportPolicy for DbTransportPolicy {
    fn checkpoint(&self, _: pack::PackTransportProgress) -> Result<(), semio_framework_value::ValueError> { Ok(()) }
    fn progress(&self, _: pack::PackTransportProgress) {}
}

#[semio_framework_async_macros::async_test]
async fn pack_transport_conversion_preserves_source_identity_and_retry_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    for row in fixture["transport"].as_array().unwrap() {
        let retry = match row["retry"].as_str().unwrap() { "Never" => pack::PackRetryDisposition::Never, "Transient" => pack::PackRetryDisposition::Transient, _ => panic!("authored retry disposition") };
        let context = pack::PackTransportContext::try_new(pack::TransportAdmission::new(4096, 0), DbTransportPolicy).ok().unwrap();
        let original = context.reserve::<std::io::Error>(pack::PackTransportCategory::NativeIo, retry).unwrap().publish(std::io::Error::from(std::io::ErrorKind::NotFound));
        let pointer = original.source_error().downcast_ref::<std::io::Error>().unwrap() as *const std::io::Error;
        let nested = DbError::Protocol(protocol::ProtocolError::Pack(pack::PackError::TransportFailure(original.clone())));
        assert_eq!(nested.is_transient(), row["transient"].as_bool().unwrap());
        let converted = DbError::from(pack::PackError::TransportFailure(original));
        assert_eq!(converted.is_transient(), row["transient"].as_bool().unwrap());
        let DbError::Pack(pack::PackError::TransportFailure(cause)) = converted else { panic!("original transport cause") };
        assert_eq!(cause.retry(), retry);
        assert_eq!(cause.source_error().downcast_ref::<std::io::Error>().unwrap() as *const std::io::Error, pointer);
    }
}

#[semio_framework_async_macros::async_test]
async fn spr_writer_conversion_returns_original_database_sink_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    let message = fixture["sink"]["message"].as_str().unwrap().to_owned();
    let pointer = message.as_ptr();
    let converted = DbError::from(protocol::SprWriteError::Sink(DbError::Io(message)));
    let DbError::Io(message) = converted else { panic!("original database sink cause") };
    assert_eq!(message.as_ptr(), pointer);
    assert_eq!(serde_json::json!({"message":message}), serde_json::json!({"message":fixture["sink"]["message"]}));
    let error = protocol::ProtocolError::ChainMismatch { commit_seq: 42 };
    assert_eq!(DbError::from(protocol::SprWriteError::Protocol(error.clone())), DbError::Protocol(error));
}
