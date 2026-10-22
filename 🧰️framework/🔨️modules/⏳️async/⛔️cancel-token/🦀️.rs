//! ⛔️ `CancelToken`: the `Arc`-shared tri-state cancellation handle with parent links, the `Cancelled` future and `CancelState`.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use super::CancelWaiters;

//#region 🛑️CancelToken
/// 🛑️ Tri-state a [`CancelToken`] can be in. `Park` is the suspend state — in-flight operations
/// finish, new work is held — distinct from the terminal `Cancelled`. Ordered by severity
/// (`Live < Park < Cancelled`) so a parent's state can be folded into a child's with a plain `max`.
/// No `Serialize`/`Deserialize`: an in-process atomic state, never carried over any wire — repo-wide
/// grep confirms zero serialization call sites, so the derive was dead weight kept off the
/// `wasm32-wasip2` guest link path (ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-
/// AND-ARTIFACTS`, `serde-off-guest-path.md`). `schema_metadata::TYPES` above owns the hand-written
/// TypeScript projection independently of this derive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CancelState {
    Live,
    Park,
    Cancelled,
}

impl CancelState {
    async fn from_u8(tag: u8) -> CancelState {
        match tag {
            0 => CancelState::Live,
            1 => CancelState::Park,
            _ => CancelState::Cancelled,
        }
    }

    #[doc(hidden)] pub async fn to_u8(self) -> u8 {
        match self {
            CancelState::Live => 0,
            CancelState::Park => 1,
            CancelState::Cancelled => 2,
        }
    }
}


#[doc(hidden)] pub struct CancelNode {
    pub local: AtomicU8,
    pub parent: Option<CancelToken>,
    pub waiters:CancelWaiters,
}

static NEXT_CANCEL_WAITER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// 🛑️ Cooperative cancellation handle: an `Arc`-shared tri-state ([`CancelState`]) plus an optional
/// parent link. [`CancelToken::child`] derives a descendant whose effective [`CancelToken::state`]
/// is the max-severity fold of its own local state and every ancestor's — cancelling a parent scope
/// therefore transitively cancels every descendant without walking a child registry, and a child's
/// own `park`/`cancel` never affects its parent.
#[derive(Clone)]
pub struct CancelToken(#[doc(hidden)] pub Arc<CancelNode>);

impl CancelToken {
    /// 📏️ Quotes one original cancellation Arc and its inline empty waiter owner.
    pub fn root_admission_demands()->semio_framework_value::RetirementDemand{
        semio_framework_value::RetirementDemand{capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<CancelNode>(),depth:1,..Default::default()}
    }

    /// 🎟️ Creates one live root only after its original independent authority admits the actual birth.
    pub fn admit_root(grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Self,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{
        let bytes=semio_framework_value::shared_retirement_allocation_bytes::<CancelNode>();
        if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes||grant.maximum_depth==0{return Ok(None);}
        let original=Self(Arc::new(CancelNode{local:AtomicU8::new(0),parent:None,waiters:CancelWaiters::new()}));
        Ok(Some((original,semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})))
    }

    /// 👶️ Creates one admitted child while retaining the exact borrowed original parent.
    pub fn admit_child(&self,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Self,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{
        let bytes=semio_framework_value::shared_retirement_allocation_bytes::<CancelNode>();
        if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes||grant.maximum_depth==0{return Ok(None);}
        let original=Self(Arc::new(CancelNode{local:AtomicU8::new(0),parent:Some(self.clone()),waiters:CancelWaiters::new()}));
        Ok(Some((original,semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})))
    }

    /// 🌱️ A fresh root token with no parent, starting `Live`.
    pub async fn root() -> CancelToken {
        CancelToken::root_now()
    }

    /// 🌱️ Creates a live root from synchronous scheduler/bootstrap code.
    pub fn root_now() -> CancelToken {
        CancelToken(Arc::new(CancelNode { local: AtomicU8::new(0), parent: None, waiters:CancelWaiters::new() }))
    }

    /// 👶️ A descendant token: its effective state is never less severe than `self`'s.
    pub async fn child(&self) -> CancelToken {
        self.child_now()
    }

    /// 👶️ Creates a cancellable descendant from a synchronous finite turn.
    pub fn child_now(&self) -> CancelToken {
        CancelToken(Arc::new(CancelNode { local: AtomicU8::new(0), parent: Some(self.clone()), waiters:CancelWaiters::new() }))
    }

    /// ⏸️ Enter the suspend state — a no-op once `Cancelled` (terminal, never downgraded).
    pub async fn park(&self) {
        let _ = self.0.local.compare_exchange(CancelState::Live.to_u8().await, CancelState::Park.to_u8().await, Ordering::SeqCst, Ordering::SeqCst);
    }

    /// ▶️ Leave the suspend state back to `Live` — a no-op once `Cancelled`.
    pub async fn unpark(&self) {
        let _ = self.0.local.compare_exchange(CancelState::Park.to_u8().await, CancelState::Live.to_u8().await, Ordering::SeqCst, Ordering::SeqCst);
    }

    /// 🛑️ Terminal: always wins over `Live`/`Park`, and over anything a later `park`/`unpark` on
    /// this same token would attempt.
    pub async fn cancel(&self) {
        self.cancel_now();
    }

    /// 🛑️ Cancels from a synchronous scheduler turn or `Drop` boundary without needing an executor.
    pub fn cancel_now(&self) {
        self.0.local.store(2, Ordering::SeqCst);
        let waiters = std::mem::take(&mut *self.0.waiters.lock());
        for (_, waker) in waiters {
            waker.wake();
        }
    }

    /// 🚩️ Marks only the same original node; waiter ownership remains intact for separately funded notification and closure.
    pub fn signal_cancelled_flag_step(&self,grant:semio_framework_value::RetainedCloneGrant)->Option<semio_framework_value::RetainedCloneProgress>{
        if self.0.local.load(Ordering::SeqCst)>=2{return Some(Default::default())}
        if grant.maximum_items==0||grant.maximum_depth==0{return None}
        self.0.local.store(2,Ordering::SeqCst);
        Some(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🔔️ Resolves once this token or any ancestor is cancelled. The waiter is registered on every
    /// node of the chain, so an ancestor's [`CancelToken::cancel_now`] wakes it directly — no timer,
    /// no poll interval. Dropping the future removes its registrations.
    pub fn cancelled(&self) -> Cancelled {
        Cancelled { token: self.clone(), id: 0 }
    }

    /// 🔍️ Max-severity fold of this token's local state and every ancestor's. Walks the parent
    /// chain iteratively rather than recursing through `parent.state().await` — a recursive
    /// `async fn` call needs `Box::pin` indirection (its state machine would otherwise be
    /// infinitely sized), and an iterative walk needs neither the allocation nor the indirection.
    pub async fn state(&self) -> CancelState {
        let mut effective = CancelState::Live;
        let mut node = Some(self);
        while let Some(current) = node {
            let local = CancelState::from_u8(current.0.local.load(Ordering::SeqCst)).await;
            effective = effective.max(local);
            node = current.0.parent.as_ref();
        }
        effective
    }

    pub async fn is_cancelled(&self) -> bool {
        self.is_cancelled_now()
    }

    /// 🪪️ True when both handles are aliases of the same original cancellation node.
    pub fn is_same_node(&self, other: &CancelToken) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    /// 🔍️ Reads cancellation from a finite synchronous scheduler turn.
    pub fn is_cancelled_now(&self) -> bool {
        let mut node = Some(self);
        while let Some(current) = node {
            if current.0.local.load(Ordering::SeqCst) >= 2 {
                return true;
            }
            node = current.0.parent.as_ref();
        }
        false
    }

    pub async fn is_parked(&self) -> bool {
        self.state().await == CancelState::Park
    }

    pub async fn is_live(&self) -> bool {
        self.state().await == CancelState::Live
    }
}

/// 🔔️ Future returned by [`CancelToken::cancelled`].
pub struct Cancelled {
    pub(super) token: CancelToken,
    pub(super) id: u64,
}

impl Cancelled {
    fn for_each_node(&self, mut visit: impl FnMut(&CancelNode)) {
        let mut node = Some(&self.token);
        while let Some(current) = node {
            visit(&current.0);
            node = current.0.parent.as_ref();
        }
    }
}

impl Future for Cancelled {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.token.is_cancelled_now() {
            return Poll::Ready(());
        }
        if self.id == 0 {
            self.id = NEXT_CANCEL_WAITER.fetch_add(1, Ordering::Relaxed);
        }
        let id = self.id;
        self.for_each_node(|node| {
            let mut waiters = node.waiters.lock();
            match waiters.iter_mut().find(|(waiter, _)| *waiter == id) {
                Some((_, waker)) => waker.clone_from(cx.waker()),
                None => waiters.push((id, cx.waker().clone())),
            }
        });
        if self.token.is_cancelled_now() { Poll::Ready(()) } else { Poll::Pending }
    }
}

impl Drop for Cancelled {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        let id = self.id;
        self.for_each_node(|node| node.waiters.lock().retain(|(waiter, _)| *waiter != id));
    }
}

impl std::fmt::Debug for CancelToken {
    /// 🚫️async: E1 external-trait impl — `std::fmt::Debug`'s signature is fixed by std, so this
    /// can never `.await`. Folds the same max-severity parent chain as [`CancelToken::state`], but
    /// inline and iteratively over the raw atomics: a `Debug` impl must never need an executor to
    /// run, and duplicating the fold as a second named (necessarily non-`async`) function would just
    /// relocate the same problem onto a fn that then needs its own R2 exception tag.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut effective = CancelState::Live;
        let mut node = Some(self);
        while let Some(current) = node {
            let local = match current.0.local.load(Ordering::SeqCst) {
                0 => CancelState::Live,
                1 => CancelState::Park,
                _ => CancelState::Cancelled,
            };
            effective = effective.max(local);
            node = current.0.parent.as_ref();
        }
        write!(f, "CancelToken({effective:?})")
    }
}
//#endregion 🛑️CancelToken
