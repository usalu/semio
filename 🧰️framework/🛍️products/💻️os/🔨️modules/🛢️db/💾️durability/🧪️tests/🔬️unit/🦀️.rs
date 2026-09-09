use super::*;

//#region 🔖️Durability
#[semio_framework_async_macros::async_test]
async fn durability_class_orders_memory_below_os_below_fsync_below_quorum() {
    assert!(DurabilityClass::Memory < DurabilityClass::Os);
    assert!(DurabilityClass::Os < DurabilityClass::Fsync);
    assert!(DurabilityClass::Fsync < DurabilityClass::Quorum(1));
    assert!(DurabilityClass::Quorum(1) < DurabilityClass::Quorum(3));
    assert_eq!(DurabilityClass::default(), DurabilityClass::Memory);

    let mut classes = vec![DurabilityClass::Quorum(2), DurabilityClass::Memory, DurabilityClass::Fsync, DurabilityClass::Os];
    classes.sort();
    assert_eq!(classes, vec![DurabilityClass::Memory, DurabilityClass::Os, DurabilityClass::Fsync, DurabilityClass::Quorum(2)]);
}

#[semio_framework_async_macros::async_test]
async fn durability_class_batch_max_picks_strongest_requested() {
    let requested = [DurabilityClass::Os, DurabilityClass::Memory, DurabilityClass::Fsync];
    let strongest = requested.into_iter().max().unwrap();
    assert_eq!(strongest, DurabilityClass::Fsync);
}
//#endregion 🔖️Durability

//#region 🔖️Frontier
async fn sample_frontier(document: &str, head_seq: u64, commit_seq: u64, epoch: u64) -> Frontier {
    let mut chain_hash = [0u8; 32];
    chain_hash[0] = head_seq as u8;
    Frontier { document: document.into(), head_seq, commit_seq, chain_hash, epoch }
}

#[semio_framework_async_macros::async_test]
async fn frontier_genesis_is_all_zero() {
    let frontier = Frontier::genesis("doc-1".into());
    assert_eq!(frontier.head_seq, 0);
    assert_eq!(frontier.commit_seq, 0);
    assert_eq!(frontier.epoch, 0);
    assert_eq!(frontier.chain_hash, [0u8; 32]);
}

#[semio_framework_async_macros::async_test]
async fn frontier_chain_hash_typed_bridges_to_pack_core_content_hash() {
    let frontier = sample_frontier("doc-1", 5, 5, 0).await;
    let typed = frontier.chain_hash_typed().await;
    assert_eq!(typed.0, frontier.chain_hash);
}

#[semio_framework_async_macros::async_test]
async fn frontier_dominates_requires_same_document_and_all_fields_at_least() {
    let earlier = sample_frontier("doc-1", 3, 3, 0).await;
    let later = sample_frontier("doc-1", 5, 5, 0).await;
    assert!(later.dominates(&earlier).unwrap());
    assert!(!earlier.dominates(&later).unwrap());
    assert!(later.dominates(&later).unwrap());

    let other_document = sample_frontier("doc-2", 5, 5, 0).await;
    assert!(matches!(later.dominates(&other_document), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn frontier_delta_between_computes_gap_and_rejects_backwards_or_mismatched() {
    let from = sample_frontier("doc-1", 3, 3, 0).await;
    let to = sample_frontier("doc-1", 8, 8, 0).await;
    let delta = FrontierDelta::between(&from, &to).await.unwrap();
    assert_eq!(delta.from_head_seq, 3);
    assert_eq!(delta.to_head_seq, 8);
    assert_eq!(delta.commands, 5);
    assert!(!delta.is_empty().await);

    let same = FrontierDelta::between(&from, &from).await.unwrap();
    assert!(same.is_empty().await);

    assert!(FrontierDelta::between(&to, &from).await.is_err());

    let other_document = sample_frontier("doc-2", 8, 8, 0).await;
    assert!(FrontierDelta::between(&from, &other_document).await.is_err());
}

#[semio_framework_async_macros::async_test]
async fn resume_token_round_trips_through_encode_decode() {
    let frontier = sample_frontier("doc-1", 42, 41, 7).await;
    let token = ResumeToken::encode(&frontier).unwrap();
    let decoded = token.decode().unwrap();
    assert_eq!(decoded, frontier);
    assert!(token.as_str().starts_with("v1|doc-1|42|41|7|"));
}

#[semio_framework_async_macros::async_test]
async fn resume_token_encode_rejects_pipe_in_document_id() {
    let frontier = sample_frontier("doc|1", 1, 1, 0).await;
    assert!(ResumeToken::encode(&frontier).is_err());
}

#[semio_framework_async_macros::async_test]
async fn resume_token_decode_rejects_malformed_input_without_panicking() {
    assert!(matches!(ResumeToken("garbage".to_string()).decode(), Err(DbError::Corrupt(_))));
    assert!(matches!(ResumeToken("v2|doc|1|1|1|00".to_string()).decode(), Err(DbError::Corrupt(_))));
    assert!(matches!(ResumeToken("v1|doc|notanumber|1|1|00".to_string()).decode(), Err(DbError::Corrupt(_))));
    let short_hash = format!("v1|doc-1|1|1|1|{}", "ab".repeat(10));
    assert!(matches!(ResumeToken(short_hash).decode(), Err(DbError::Corrupt(_))));
}
//#endregion 🔖️Frontier

//#region 🔖️Fencing
#[semio_framework_async_macros::async_test]
async fn epoch_fence_check_accepts_matching_epoch_and_rejects_stale_or_ahead() {
    let current = EpochFence::INITIAL.next().next();
    assert!(current.check(current).is_ok());

    let stale = EpochFence::INITIAL;
    assert_eq!(stale.check(current), Err(DbError::Fenced { expected: current.epoch, actual: stale.epoch }));

    let ahead = current.next();
    assert_eq!(ahead.check(current), Err(DbError::Fenced { expected: current.epoch, actual: ahead.epoch }));
}

#[semio_framework_async_macros::async_test]
async fn epoch_fence_next_is_monotonic() {
    let mut fence = EpochFence::INITIAL;
    for expected in 1..=5u64 {
        fence = fence.next();
        assert_eq!(fence.epoch, expected);
    }
}
//#endregion 🔖️Fencing
