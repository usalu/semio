//! 🧪️ Acceptance test 5: a port whose methods declare `-> impl Future<Output = T> + Send` instead of
//! `async fn` — the shape a trait MUST take once its call site reaches it behind a generic parameter
//! (`S: Store`) and needs the future to be provably `Send`, because an `async fn` in a trait returns
//! an opaque future with NO auto-trait guarantee and every `Send`-bounded caller then rejects it.
//! Closing such a set used to be impossible: one generated `match` would put N distinct opaque future
//! types in one return position. The delegate is now emitted as `async fn .. -> T`, which Rust accepts
//! against an `impl Future` declaration, and this file proves all three halves of that claim — it
//! compiles, the future really is `Send` (asserted through the generic bound AND moved across a real
//! thread), and the right variant answers at runtime.
#![allow(async_fn_in_trait)] // R7 — never resolved by `+ Send` or by making a method sync.

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use std::future::Future;

//#region 🔖️Trait — Send-future methods, a `&mut self` one, a plain `async fn`, a sync one

#[dyn_enum]
pub trait Store: Send + Sync {
    fn read(&self, key: &str) -> impl Future<Output = Option<String>> + Send;
    fn write(&mut self, key: &str, value: String) -> impl Future<Output = usize> + Send;
    fn tag(&self) -> impl Future<Output = &str> + Send;
    fn probe<K: AsRef<str> + Sync>(&self, key: &K) -> impl Future<Output = bool> + Send;
    async fn count(&self) -> usize;
    fn label(&self) -> &'static str {
        "store"
    }
}

/// 📓️ A SECOND port with a method named exactly like [`Store::read`], implemented by the very same
/// types. One backend serving two roles is the ordinary case (a durable store that is both the
/// projection store and the blob store), and it is what makes an unqualified `inner.read(..)` in a
/// generated arm an outright `E0034 multiple applicable items in scope`.
#[dyn_enum]
pub trait Journal: Send + Sync {
    fn read(&self, key: &str) -> impl Future<Output = usize> + Send;
}

//#endregion

//#region 🔖️Two concrete impls, both ordinary `async fn`

#[derive(Default)]
pub struct TextStore {
    slot: Option<String>,
    writes: usize,
}

impl Store for TextStore {
    async fn read(&self, key: &str) -> Option<String> {
        if key == "k" {
            self.slot.clone()
        } else {
            None
        }
    }

    async fn write(&mut self, key: &str, value: String) -> usize {
        if key == "k" {
            self.slot = Some(value);
        }
        self.writes += 1;
        self.writes
    }

    async fn tag(&self) -> &str {
        "text"
    }

    async fn probe<K: AsRef<str> + Sync>(&self, key: &K) -> bool {
        Store::read(self, key.as_ref()).await.is_some()
    }

    async fn count(&self) -> usize {
        usize::from(self.slot.is_some())
    }

    fn label(&self) -> &'static str {
        "text"
    }
}

#[derive(Default)]
pub struct KvStore {
    map: std::collections::BTreeMap<String, String>,
}

impl Store for KvStore {
    async fn read(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }

    async fn write(&mut self, key: &str, value: String) -> usize {
        self.map.insert(key.to_string(), value);
        self.map.len()
    }

    async fn tag(&self) -> &str {
        "kv"
    }

    async fn probe<K: AsRef<str> + Sync>(&self, key: &K) -> bool {
        self.map.contains_key(key.as_ref())
    }

    async fn count(&self) -> usize {
        self.map.len()
    }
}

impl Journal for TextStore {
    async fn read(&self, key: &str) -> usize {
        key.len()
    }
}

impl Journal for KvStore {
    async fn read(&self, key: &str) -> usize {
        self.map.get(key).map_or(0, String::len)
    }
}

impl TextStore {
    /// 🥷️ An INHERENT method of the same name, which method-call syntax would prefer over either
    /// trait's — so a generated arm written `inner.read(..)` would silently delegate to something no
    /// closed set ever promised.
    pub fn read(&self, _key: &str) -> &'static str {
        "inherent"
    }
}

//#endregion

//#region 🔖️Closing enums — two variants, and the empty sibling

dyn_enum_close! {
    pub enum Stores: Store {
        Text(TextStore),
        Kv(KvStore),
    }
}

dyn_enum_close! {
    pub enum NoStores: Store {}
}

dyn_enum_close! {
    pub enum Journals: Journal {
        Text(TextStore),
        Kv(KvStore),
    }
}

//#endregion

/// 🧵️ The property the whole slice exists for: a caller that only knows `S: Store` must be able to
/// hold the returned future across an `.await` inside a `Send` future of its own. This is exactly
/// what an axum handler and a `tokio::spawn`ed socket task demand, and exactly what an `async fn`
/// trait method cannot promise.
async fn read_through_the_bound<S: Store>(store: &S, key: &str) -> Option<String> {
    let first = store.read(key).await;
    let tag = store.tag().await;
    first.map(|value| format!("{tag}:{value}"))
}

fn assert_send<T: Send>(_value: &T) {}

fn block_on<F: Future>(future: F) -> F::Output {
    // 🌀️ E5 executor bridge (at most one per crate, per R4/R5) — every future here is eagerly ready.
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
fn the_delegated_future_is_send_directly_and_through_a_generic_bound() {
    let store = Stores::Text(TextStore { slot: Some("hello".to_string()), writes: 0 });
    let direct = store.read("k");
    assert_send(&direct);
    assert_eq!(block_on(direct), Some("hello".to_string()));
    let through_bound = read_through_the_bound(&store, "k");
    assert_send(&through_bound);
    assert_eq!(block_on(through_bound), Some("text:hello".to_string()));
}

#[test]
fn the_delegated_future_really_crosses_a_thread() {
    let store = Stores::Kv(KvStore { map: [("a".to_string(), "1".to_string())].into_iter().collect() });
    let joined = std::thread::spawn(move || block_on(read_through_the_bound(&store, "a"))).join().expect("the worker thread must not panic");
    assert_eq!(joined, Some("kv:1".to_string()));
}

#[test]
fn every_arm_is_reached_including_the_mutable_one() {
    let mut text = Stores::Text(TextStore::default());
    assert_eq!(block_on(text.write("k", "one".to_string())), 1);
    assert_eq!(block_on(text.write("k", "two".to_string())), 2);
    assert_eq!(block_on(text.read("k")), Some("two".to_string()));
    assert_eq!(block_on(text.count()), 1);
    assert_eq!(text.label(), "text");

    let mut kv = Stores::Kv(KvStore::default());
    assert_eq!(block_on(kv.write("a", "1".to_string())), 1);
    assert_eq!(block_on(kv.write("b", "2".to_string())), 2);
    assert_eq!(block_on(kv.read("b")), Some("2".to_string()));
    assert_eq!(block_on(kv.count()), 2);
    assert_eq!(kv.label(), "store", "KvStore inherits the trait's default `label` body");
}

#[test]
fn a_generic_method_on_a_send_future_port_delegates_too() {
    let store = Stores::Kv(KvStore { map: [("a".to_string(), "1".to_string())].into_iter().collect() });
    let present = "a".to_string();
    let absent = "b".to_string();
    assert!(block_on(store.probe(&present)));
    assert!(!block_on(store.probe(&absent)));
    let probing = store.probe(&present);
    assert_send(&probing);
    assert!(block_on(probing));
}

#[test]
fn a_set_with_no_members_still_closes_the_send_future_port() {
    fn accepts_the_port<S: Store>() {}
    accepts_the_port::<NoStores>();
    assert_eq!(size_of::<NoStores>(), 0);
}

#[test]
fn a_name_shared_by_two_ports_and_an_inherent_method_still_delegates_to_the_closed_one() {
    let store = Stores::Text(TextStore { slot: Some("hello".to_string()), writes: 0 });
    let journal = Journals::Text(TextStore { slot: None, writes: 0 });
    assert_eq!(block_on(Store::read(&store, "k")), Some("hello".to_string()));
    assert_eq!(block_on(Journal::read(&journal, "key")), 3);
    assert_eq!(block_on(Journal::read(&Journals::Kv(KvStore { map: [("a".to_string(), "12".to_string())].into_iter().collect() }), "a")), 2);
}

#[test]
fn from_impls_are_generated_for_each_variant() {
    let text: Stores = TextStore::default().into();
    let kv: Stores = KvStore::default().into();
    assert!(matches!(text, Stores::Text(_)));
    assert!(matches!(kv, Stores::Kv(_)));
}
