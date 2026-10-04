//! 🔂️ Completes one immediate future poll and rejects suspension.

pub fn resolve_ready<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll};
    let waker = std::task::Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut pinned = std::pin::pin!(fut);
    match pinned.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("resolve_ready: future was not ready on its first poll"),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
