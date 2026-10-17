//! 🧰️ A fresh scratch directory under the system temp dir, removed when dropped, shared with the tests of the crates above (`testkit`).

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SCRATCH: AtomicU64 = AtomicU64::new(0);

/// 🧹️ A fresh directory under the system temp dir, removed when dropped.
pub struct Scratch(pub PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// 🧪️ A fresh scratch directory named after `label`.
pub fn scratch(label: &str) -> Scratch {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("teaching-proctor-{label}-{}-{nanos}-{}", std::process::id(), NEXT_SCRATCH.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(&path).expect("scratch directory");
    Scratch(path)
}
