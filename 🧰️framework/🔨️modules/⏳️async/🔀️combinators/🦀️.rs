//! 🔀️ Future combinators: `select2`, `race2`, `join2` and the cooperative `yield_once`.

use std::future::Future;
use std::task::Poll;

//#region 🔀️Select2
/// 🌿️ Which branch of a [`select2`] race resolved first.
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

/// 🏎️ Races exactly two futures, returning whichever resolves first and dropping the other — the
/// repo-owned two-branch replacement for `tokio::select!` (INTERACTIVE-JOB-RUNTIME-REFACTOR Phase 9
/// packet P9a). Every real race in this repo (a result vs. a deadline, a sleep vs. an early-wake
/// notification) has exactly two arms, so no N-arm macro was built — build only what has a consumer.
pub async fn select2<A, B>(a: A, b: B) -> Either<A::Output, B::Output>
where
    A: Future,
    B: Future,
{
    let mut a = std::pin::pin!(a);
    let mut b = std::pin::pin!(b);
    std::future::poll_fn(move |cx| {
        if let Poll::Ready(value) = a.as_mut().poll(cx) {
            return Poll::Ready(Either::Left(value));
        }
        if let Poll::Ready(value) = b.as_mut().poll(cx) {
            return Poll::Ready(Either::Right(value));
        }
        Poll::Pending
    })
    .await
}

/// 🏁️ Races two futures with the same output type and returns the winner directly.
pub async fn race2<A, B, T>(left: A, right: B) -> T
where
    A: Future<Output = T>,
    B: Future<Output = T>,
{
    match select2(left, right).await {
        Either::Left(output) | Either::Right(output) => output,
    }
}
//#endregion 🔀️Select2

//#region 🤝️Cooperate
/// 🤝️ Yields once to the current executor, waking the task before returning `Pending` so the
/// next poll completes. This is the repository-owned cooperative handoff primitive for bounded
/// jobs and test doubles; it never sleeps, blocks, or creates an executor.
pub async fn yield_once() {
    let mut yielded = false;
    std::future::poll_fn(move |cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await;
}

/// 🪢️ Drives two futures concurrently on the current executor and returns both outputs after
/// both resolve. Poll order is stable (`left`, then `right`) on every turn.
pub async fn join2<A, B>(left: A, right: B) -> (A::Output, B::Output)
where
    A: Future,
    B: Future,
{
    let mut left = std::pin::pin!(left);
    let mut right = std::pin::pin!(right);
    let mut left_output = None;
    let mut right_output = None;
    std::future::poll_fn(move |cx| {
        if left_output.is_none() {
            if let Poll::Ready(output) = left.as_mut().poll(cx) {
                left_output = Some(output);
            }
        }
        if right_output.is_none() {
            if let Poll::Ready(output) = right.as_mut().poll(cx) {
                right_output = Some(output);
            }
        }
        match (left_output.take(), right_output.take()) {
            (Some(left), Some(right)) => Poll::Ready((left, right)),
            (left, right) => {
                left_output = left;
                right_output = right;
                Poll::Pending
            }
        }
    })
    .await
}
//#endregion 🤝️Cooperate
