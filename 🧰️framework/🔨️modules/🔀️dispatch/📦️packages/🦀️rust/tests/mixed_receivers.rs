//! 🧪️ Acceptance test 1: mixed async/sync methods, `&self`/`&mut self`, a default-bodied method, a
//! generic method, a `where` clause → 3 variants → delegation reaches the right variant at RUNTIME.
//! This is a genuine `tests/*.rs` integration crate — it depends on `semio-framework-dispatch-macros`
//! as an ordinary crate, so `#[dyn_enum]`/`dyn_enum_close!` run exactly as any downstream consumer
//! would use them (a proc-macro crate cannot invoke its OWN macros from inside itself — see
//! `📓️terra-dyn-enum-macro-report.md`).
#![allow(async_fn_in_trait)] // R7 — never resolved by `+ Send` or by making a method sync.

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use std::cell::Cell;

//#region 🔖️Trait — mixed receivers, default body, generic method, where clause

#[dyn_enum]
pub trait Store {
    async fn read(&self, key: &str) -> Option<String>;
    async fn write(&mut self, key: &str, value: String);
    fn label(&self) -> &'static str {
        "store"
    }
    fn echo<T>(&self, value: T) -> T
    where
        T: Clone,
    {
        value
    }
}

//#endregion

//#region 🔖️Two concrete impls

pub struct TextStore {
    slot: Option<String>,
    writes: Cell<u32>,
}

impl Store for TextStore {
    async fn read(&self, key: &str) -> Option<String> {
        if key == "k" {
            self.slot.clone()
        } else {
            None
        }
    }
    async fn write(&mut self, key: &str, value: String) {
        if key == "k" {
            self.slot = Some(value);
        }
        self.writes.set(self.writes.get() + 1);
    }
    fn label(&self) -> &'static str {
        "text"
    }
}

pub struct KvStore {
    map: std::collections::HashMap<String, String>,
}

impl Store for KvStore {
    async fn read(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
    async fn write(&mut self, key: &str, value: String) {
        self.map.insert(key.to_string(), value);
    }
    // `label` and `echo` inherit the trait's default bodies — dyn_enum must delegate to
    // WHATEVER the concrete impl resolves to, override or default, without knowing which.
}

//#endregion

//#region 🔖️Closing enum

dyn_enum_close! {
    pub enum Stores: Store {
        Text(TextStore),
        Kv(KvStore),
    }
}

//#endregion

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    // 🌀️ E5 executor bridge (at most one per crate, per R4/R5) — this test crate has no runtime
    // dependency, and every future here is eagerly ready (no real IO), so a single poll suffices.
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
}

#[test]
fn delegates_read_and_write_to_the_text_variant() {
    let mut store = Stores::Text(TextStore { slot: None, writes: Cell::new(0) });
    assert_eq!(block_on(store.read("k")), None);
    block_on(store.write("k", "hello".to_string()));
    assert_eq!(block_on(store.read("k")), Some("hello".to_string()));
    assert_eq!(store.label(), "text");
}

#[test]
fn delegates_to_the_kv_variant_and_uses_default_bodied_methods() {
    let mut store = Stores::Kv(KvStore { map: std::collections::HashMap::new() });
    block_on(store.write("a", "1".to_string()));
    assert_eq!(block_on(store.read("a")), Some("1".to_string()));
    assert_eq!(store.label(), "store", "KvStore inherits the trait's default `label` body");
    assert_eq!(store.echo(42u32), 42u32, "generic method with a where clause must still delegate");
}

#[test]
fn from_impls_are_generated_for_each_variant() {
    let text: Stores = TextStore { slot: None, writes: Cell::new(0) }.into();
    let kv: Stores = KvStore { map: std::collections::HashMap::new() }.into();
    assert!(matches!(text, Stores::Text(_)));
    assert!(matches!(kv, Stores::Kv(_)));
}
