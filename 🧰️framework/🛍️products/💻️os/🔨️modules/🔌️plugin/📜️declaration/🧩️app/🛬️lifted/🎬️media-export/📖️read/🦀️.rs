//! 📖️ Original media capture binds the actual Store issuer and exact parent payload before child qualification.
use store::os_store::snapshot_read::SnapshotRead;
use semio_framework_value::RetirementDemand;
/// 📏️ Covers fixed issuer authority publication, lease-ring metadata and all actual snapshot/read aliases without allocation.
pub fn media_original_read_capture_demand<T>() -> RetirementDemand{RetirementDemand{copy_bytes:16*std::mem::size_of::<SnapshotRead<T>>()+128,depth:2,..Default::default()}}
/// 🔐️ Same-value snapshots cannot replace the captured payload or its current original issuer.
pub fn media_original_read_matches<T>(read:&SnapshotRead<T>,snapshot:&std::sync::Arc<T>,generation:u64,revision:[u8;32])->bool{std::ptr::eq(read.get(),snapshot.as_ref())&&read.commit_authority_matches(generation,revision)}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;
