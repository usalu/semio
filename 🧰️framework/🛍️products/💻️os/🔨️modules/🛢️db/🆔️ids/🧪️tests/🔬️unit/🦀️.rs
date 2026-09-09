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
