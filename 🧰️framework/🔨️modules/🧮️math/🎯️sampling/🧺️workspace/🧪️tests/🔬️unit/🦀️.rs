use super::*;

// #region 🔖️WorkspaceTests
#[semio_framework_async_macros::async_test]
async fn workspace_reset_for_step_initializes_live_to_full_vocab_and_argmax() {
    let mut ws = LogitsWorkspace::new(5).await;
    let logits = [1.0f32, 3.0, 2.0, 3.0, 0.0];
    ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).await.expect("finite logits never error");
    assert_eq!(ws.live().await, &[0, 1, 2, 3, 4]);
    assert_eq!(ws.saved_argmax().await, TokenId::new(1));
}

#[semio_framework_async_macros::async_test]
async fn workspace_sync_live_with_mask_removes_masked_entries() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).await.unwrap();
    ws.mask_mut().await.set(TokenId::new(2), false).await;
    ws.sync_live_with_mask().await;
    assert_eq!(ws.live().await, &[0, 1, 3]);
}

#[semio_framework_async_macros::async_test]
async fn workspace_sort_live_by_prob_desc_orders_by_probability_then_token_id() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 3.0, 3.0, 0.5], SanitizePolicy::NegInfNan).await.unwrap();
    ws.sort_live_by_prob_desc().await;
    assert_eq!(ws.live().await, &[1, 2, 0, 3]);
    assert!(ws.probs().await[0] >= ws.probs().await[1]);
    assert!(ws.probs().await[1] >= ws.probs().await[2]);
}

#[semio_framework_async_macros::async_test]
async fn workspace_truncate_live_to_respects_min_keep() {
    let mut ws = LogitsWorkspace::new(5).await;
    ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).await.unwrap();
    ws.sort_live_by_prob_desc().await;
    ws.truncate_live_to(1, 3).await;
    assert_eq!(ws.live().await.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn workspace_collapse_live_to_argmax_leaves_single_best_entry() {
    let mut ws = LogitsWorkspace::new(4).await;
    ws.reset_for_step(&[1.0, 5.0, 2.0, 5.0], SanitizePolicy::NegInfNan).await.unwrap();
    ws.collapse_live_to_argmax().await;
    assert_eq!(ws.live().await, &[1]);
}

#[semio_framework_async_macros::async_test]
async fn workspace_pool_reuses_released_workspace() {
    let mut pool = WorkspacePool::new().await;
    let ws = pool.acquire(16).await;
    assert_eq!(ws.vocab_size().await, 16);
    pool.release(ws).await;
    let reused = pool.acquire(16).await;
    assert_eq!(reused.vocab_size().await, 16);
}

// #endregion 🔖️WorkspaceTests
