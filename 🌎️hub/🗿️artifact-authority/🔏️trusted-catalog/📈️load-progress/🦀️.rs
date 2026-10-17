//! 📈️ The live load progress of one catalog, written by its load and its background verification and read for readiness and observability.

use super::*;

/// 📈️ The live [`TrustedCatalogLoadProgressV1`] of one catalog: its load and its background verification
/// report into it, and the hub reads it for `/readyz` while it starts and for its admin observability route.
#[derive(Default)]
pub struct TrustedCatalogLoadProgressCellV1 {
    pub progress: std::sync::Mutex<TrustedCatalogLoadProgressV1>,
}

impl TrustedCatalogLoadProgressCellV1 {
    /// 📸️ The progress now.
    pub fn snapshot(&self) -> TrustedCatalogLoadProgressV1 {
        self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    pub fn update(&self, change: impl FnOnce(&mut TrustedCatalogLoadProgressV1)) {
        let mut progress = self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        change(&mut progress);
        progress.packages_total = u64::try_from(progress.packages.len()).unwrap_or(u64::MAX);
        progress.packages_ready = u64::try_from(progress.packages.iter().filter(|package| package.phase == TrustedCatalogPackagePhaseV1::Ready).count()).unwrap_or(u64::MAX);
        progress.packages_refused = u64::try_from(progress.packages.iter().filter(|package| package.phase == TrustedCatalogPackagePhaseV1::Refused).count()).unwrap_or(u64::MAX);
        progress.component_bytes_total = progress.packages.iter().map(|package| package.component_bytes).sum();
        progress.rows_total = progress.packages.iter().map(|package| package.rows).sum();
        progress.rows_pinned = progress.packages.iter().map(|package| package.rows_pinned).sum();
        progress.rows_verified = progress.packages.iter().map(|package| package.rows_verified).sum();
    }

    pub fn select(&self, packages: Vec<TrustedCatalogPackageProgressV1>) {
        self.update(|progress| *progress = TrustedCatalogLoadProgressV1 { packages, ..TrustedCatalogLoadProgressV1::default() });
    }

    pub fn package(&self, position: usize, change: impl FnOnce(&mut TrustedCatalogPackageProgressV1)) {
        self.update(|progress| {
            if let Some(package) = progress.packages.get_mut(position) {
                change(package);
            }
        });
    }

    pub fn phase(&self, position: usize, phase: TrustedCatalogPackagePhaseV1) {
        self.package(position, |package| package.phase = phase);
    }

    pub(super) fn component_read(&self, position: usize) {
        self.update(|progress| {
            if let Some(bytes) = progress.packages.get(position).map(|package| package.component_bytes) {
                progress.component_bytes_read = progress.component_bytes_read.saturating_add(bytes);
            }
        });
    }
}
