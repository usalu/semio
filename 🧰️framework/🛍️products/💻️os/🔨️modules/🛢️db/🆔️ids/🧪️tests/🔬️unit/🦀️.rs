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
async fn pack_error_conversion_never_panics_and_maps_by_category() {
    let corrupt: DbError = pack::PackError::BadMagic.into();
    assert!(matches!(corrupt, DbError::Corrupt(_)));

    let limit: DbError = pack::PackError::LimitExceeded("too big").into();
    assert_eq!(limit, DbError::LimitExceeded("too big"));

    let io: DbError = pack::PackError::Io("disk full".to_string()).into();
    assert_eq!(io, DbError::Io("disk full".to_string()));

    let schema: DbError = pack::PackError::Schema("bad field".to_string()).into();
    assert_eq!(schema, DbError::InvalidArgument("bad field".to_string()));
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
async fn pack_refusal_kind_controls_db_category_and_owned_message() {
    use semio_framework_value::{ValueError,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    fn kind(value:&str)->ValueRefusalKind{match value{"InvalidValue"=>ValueRefusalKind::InvalidValue,"Canceled"=>ValueRefusalKind::Canceled,"OwnershipLimit"=>ValueRefusalKind::OwnershipLimit,"AllocationFailed"=>ValueRefusalKind::AllocationFailed,"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,"UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"InvariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown authored kind")}}
    for row in fixture["cases"].as_array().unwrap(){
        let mut retained=None;
        let error=match row["source"].as_str().unwrap(){
            "ValueRefusal"=>{let mut cause=ValueError::new(kind(row["kind"].as_str().unwrap()),row["message"].as_str().unwrap());for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}if cause.kind==ValueRefusalKind::InvalidValue{retained=Some(cause.message.as_ptr());}pack::PackError::ValueRefusal(cause)},
            "Io"=>pack::PackError::Io(row["message"].as_str().unwrap().to_owned()),
            "LimitExceeded"=>pack::PackError::LimitExceeded("too big"),
            "BadMagic"=>pack::PackError::BadMagic,
            "Truncated"=>pack::PackError::Truncated(row["offset"].as_u64().unwrap()),
            _=>panic!("unknown authored source")
        };
        let converted=DbError::from(error);
        let(category,message)=match &converted{DbError::InvalidArgument(message)=>("InvalidArgument",message.as_str()),DbError::Corrupt(message)=>("Corrupt",message.as_str()),DbError::Io(message)=>("Io",message.as_str()),DbError::LimitExceeded(message)=>("LimitExceeded",*message),_=>panic!("unexpected conversion category")};
        assert_eq!(serde_json::json!({"category":category,"output":message}),serde_json::json!({"category":row["category"],"output":row["output"]}),"{}",row["id"]);
        if let Some(pointer)=retained{let DbError::InvalidArgument(message)=converted else{panic!("typed invalid argument")};assert_eq!(message.as_ptr(),pointer);}
    }
    eprintln!("[DEBUG] DB Pack conversion twelve closed vectors use typed kind and retain invalid argument allocation");
}
