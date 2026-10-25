//! 🌉️ Browser-only `Promise`/`JsValue` bridge: owned microtask spawn and Rust-future-to-Promise adapters (excluded from `wasm32-wasip2`).

use js_sys::Promise;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::JsValue;

/// 🌐️ Schedules one browser-local future through the owned microtask executor.
pub fn spawn_local(future: impl Future<Output = ()> + 'static) {
    js_sys::futures::spawn_local(future);
}

/// 🤝 Converts an owned Rust future into the browser Promise ABI.
pub fn future_to_promise<F>(future: F) -> Promise
where
    F: Future<Output = Result<JsValue, JsValue>> + 'static,
{
    js_sys::futures::future_to_promise(future)
}

/// 🔄️ Adapts one browser Promise into the framework-owned Rust future type.
pub struct JsFuture {
    inner: js_sys::futures::JsFuture,
}

impl From<Promise> for JsFuture {
    fn from(promise: Promise) -> Self {
        Self { inner: promise.into() }
    }
}

impl Future for JsFuture {
    type Output = Result<JsValue, JsValue>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(context)
    }
}
