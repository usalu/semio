//! 🏗️ Coroutine backing storage is born only after independent original construction grants.
use super::*;

#[path = "🎟️original/🦀️.rs"]
mod original;
#[path = "🗂️index/🦀️.rs"]
mod index;
#[path = "📋️vector/🦀️.rs"]
mod vector;
#[path = "👁️read/🦀️.rs"]
mod read;
#[path = "🍂️remove/🦀️.rs"]
mod remove;
#[path = "🧾️bytes/🦀️.rs"]
mod bytes;
#[path = "🔏️hash/🦀️.rs"]
mod hash;

struct FoldBackingBirth<T: Default + RetireOwned> {
    control: HistoryFoldControl,
    count: usize,
    capacity: Option<usize>,
    copy: usize,
    allocate: fn(&mut T, usize) -> Result<(), ValueError>,
    owner: Option<HistoryFoldOwned<T>>,
    registered: bool,
}

impl<T: Default + RetireOwned> Unpin for FoldBackingBirth<T> {}

impl<T: Default + RetireOwned> FoldBackingBirth<T> {
    fn new(control: &HistoryFoldControl, count: usize, bytes: Option<usize>, allocate: fn(&mut T, usize) -> Result<(), ValueError>) -> Self {
        let capacity = bytes.and_then(|bytes| bytes.checked_add(owned_retirement_birth_bytes::<T>()));
        let copy = 2 * size_of::<T>();
        if let Some(capacity) = capacity {
            control.0.pending_construction_capacity.fetch_add(capacity, Ordering::Relaxed);
            control.0.pending_original_copy.fetch_add(copy, Ordering::Relaxed);
        }
        Self { control: control.clone(), count, capacity, copy, allocate, owner: None, registered: capacity.is_some() }
    }

    fn unregister(&mut self) {
        if self.registered {
            self.control.0.pending_construction_capacity.fetch_sub(self.capacity.unwrap(), Ordering::Relaxed);
            self.control.0.pending_original_copy.fetch_sub(self.copy, Ordering::Relaxed);
            self.registered = false;
        }
    }
}

impl<T: Default + RetireOwned> Future for FoldBackingBirth<T> {
    type Output = Result<HistoryFoldOwned<T>, semio_framework_replication_error::ProtocolError>;

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let Some(capacity) = self.capacity else { return Poll::Ready(Err(value_cause(ValueError::literal(ValueRefusalKind::OwnershipLimit, "history fold construction size overflow")))) };
        let grant = self.control.original_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < self.copy || grant.maximum_capacity_bytes < capacity { return Poll::Pending; }
        let (owner, mut progress) = match admit_typed_controlled_retirement(T::default(), grant) {
            Ok(admitted) => admitted,
            Err((error, _)) => return Poll::Ready(Err(value_cause(error))),
        };
        self.owner = Some(HistoryFoldOwned { owner: Some(owner), control: self.control.clone() });
        let allocate = self.allocate;
        let count = self.count;
        let result = allocate(self.owner.as_mut().unwrap(), count);
        progress.copied_bytes = self.copy;
        if result.is_ok() { progress.retained_capacity_bytes = capacity; }
        self.control.accept_original_progress(progress);
        self.unregister();
        match result { Ok(()) => Poll::Ready(Ok(self.owner.take().unwrap())), Err(error) => Poll::Ready(Err(value_cause(error))) }
    }
}

impl<T: Default + RetireOwned> Drop for FoldBackingBirth<T> {
    fn drop(&mut self) { self.unregister(); }
}

impl HistoryFoldControl {
    /// 📚️ Admits the actual vector backing and its cancellation frame before either allocation.
    pub fn vector<T: RetireOwned>(&self, count: usize) -> impl Future<Output = Result<HistoryFoldOwned<Vec<T>>, semio_framework_replication_error::ProtocolError>> {
        FoldBackingBirth::new(self, count, count.checked_mul(size_of::<T>()).filter(|bytes| *bytes <= isize::MAX as usize), |value: &mut Vec<T>, count| value.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "history fold vector allocation failed")))
    }

    /// 📝️ Admits UTF-8 storage before any source bytes are copied into it.
    pub fn text(&self, bytes: usize) -> impl Future<Output = Result<HistoryFoldOwned<String>, semio_framework_replication_error::ProtocolError>> {
        FoldBackingBirth::new(self, bytes, (bytes <= isize::MAX as usize).then_some(bytes), |value: &mut String, bytes| value.try_reserve_exact(bytes).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "history fold text allocation failed")))
    }
}

fn value_cause(error: ValueError) -> semio_framework_replication_error::ProtocolError { semio_framework_pack_error::PackRefusal::ValueRefusal(error).into() }

/// 🌳️ Hashes the exact length-prefixed trunk identity without a temporary material buffer.
pub async fn copy_history_trunk(document: &ArtifactId, control: &HistoryFoldControl) -> Result<String, semio_framework_replication_error::ProtocolError> {
    let mut hash = control.hash_start().await;
    for source in ["semio.history.trunk", document.0.as_str()] {
        let mut length = source.len() as u64;
        let mut prefix = [0; 10];
        let mut count = 0;
        loop {
            prefix[count] = (length as u8 & 127) | if length >= 128 { 128 } else { 0 };
            count += 1;
            length >>= 7;
            if length == 0 { break; }
        }
        control.hash_bytes(&mut hash,&prefix[..count]).await;
        control.hash_bytes(&mut hash,source.as_bytes()).await;
    }
    let digest = control.hash_finish(&hash).await;
    let mut result = [0;22];result[..6].copy_from_slice(b"trunk-");
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for (index,byte) in digest.as_bytes()[..8].iter().enumerate() { result[6+2*index]=HEX[(byte >> 4) as usize];result[7+2*index]=HEX[(byte & 15) as usize]; }
    super::copy_history_text(std::str::from_utf8(&result).expect("trunk identity is ASCII"),control).await
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
