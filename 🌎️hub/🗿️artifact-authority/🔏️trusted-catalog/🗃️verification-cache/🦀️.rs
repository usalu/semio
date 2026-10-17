//! 🗃️ The content-addressed memory of guest codec verifications and the verification concurrency bound.

use super::*;

/// 🪪️ What a remembered verification was computed by: the owned engine's semantic identity and the
/// fuel ceiling every guest codec call runs under.
pub fn guest_codec_engine_identity() -> String {
    format!("{}:fuel-{}", semio_framework_plugin_interpreter_runtime::owned_engine_identity(), GUEST_CODEC_BUDGET.fuel)
}

/// 🗃️ The hub's content-addressed memory of guest codec verifications (`GuestCodecVerificationV1`),
/// one file per verification under `<hub data>/trusted-catalog/guest-codec-verifications/`, named by
/// the SHA-256 of its canonical key. A record is used only when its component SHA-256 — taken from the
/// bytes this very load re-read and re-hashed — its artifact schema and its engine all match. The
/// engine is the owned interpreter's semantic identity plus the codec budget, never the executable's
/// path, inode or signature, so a copied, re-signed or container build of the same engine boots warm
/// on a catalog another build verified or published, and a changed engine verifies afresh. The records
/// live inside `trusted-catalog/` so they travel with a copied or restored catalog. Reading or writing a
/// record never decides a verification: an unreadable record is a miss, and a record that cannot be
/// written is recomputed on the next boot.
#[derive(Clone)]
pub struct GuestCodecVerificationCacheV1 {
    pub root: Option<std::path::PathBuf>,
    engine: String,
}

impl GuestCodecVerificationCacheV1 {
    /// 📂️ The cache inside a hub's trusted catalog, keyed to the owned engine that interprets its guests.
    pub fn beside(data_path: &Path) -> Self {
        Self { root: Some(data_path.join("trusted-catalog").join("guest-codec-verifications")), engine: guest_codec_engine_identity() }
    }

    /// 🚫️ No memory at all: every verification interprets its component.
    pub fn disabled() -> Self {
        Self { root: None, engine: String::new() }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn at(root: std::path::PathBuf, engine: &str) -> Self {
        Self { root: Some(root), engine: engine.into() }
    }

    pub fn record(&self, component_sha256: &[u8; 32], artifact_schema: &str, pack_schema_hash: &[u8; 32]) -> GuestCodecVerificationV1 {
        GuestCodecVerificationV1 { schema: GUEST_CODEC_VERIFICATION_SCHEMA.into(), component_sha256: hex_lower(component_sha256), artifact_schema: artifact_schema.into(), engine: self.engine.clone(), pack_schema_hash: hex_lower(pack_schema_hash) }
    }

    pub fn path(&self, component_sha256: &[u8; 32], artifact_schema: &str) -> Option<std::path::PathBuf> {
        let root = self.root.as_ref()?;
        let key = serde_json::to_vec(&(GUEST_CODEC_VERIFICATION_SCHEMA, hex_lower(component_sha256), artifact_schema, &self.engine)).ok()?;
        let mut hash = Sha256::new();
        hash.update(&key);
        Some(root.join(format!("{}.json", hex_lower(&hash.finalize()))))
    }

    /// 🔎️ The pack-schema hash remembered for exactly this component, schema and engine.
    pub async fn recall(&self, component_sha256: &[u8; 32], artifact_schema: &str) -> Option<[u8; 32]> {
        let bytes = tokio::fs::read(self.path(component_sha256, artifact_schema)?).await.ok()?;
        if bytes.len() > 4096 {
            return None;
        }
        let record: GuestCodecVerificationV1 = serde_json::from_slice(&bytes).ok()?;
        let hash = decode_digest(&record.pack_schema_hash, "remembered pack schema hash").ok()?;
        (record == self.record(component_sha256, artifact_schema, &hash) && hash != [0; 32]).then_some(hash)
    }

    /// 💾️ Remembers one verification: written to a sibling temporary file, then renamed into place.
    pub async fn remember(&self, component_sha256: &[u8; 32], artifact_schema: &str, pack_schema_hash: &[u8; 32]) -> bool {
        let Some(path) = self.path(component_sha256, artifact_schema) else { return false };
        let Ok(bytes) = serde_json::to_vec(&self.record(component_sha256, artifact_schema, pack_schema_hash)) else { return false };
        let staged = path.with_extension(format!("{}.tmp", std::process::id()));
        let written = async {
            tokio::fs::create_dir_all(path.parent().ok_or_else(|| std::io::Error::other("cache path has no parent"))?).await?;
            tokio::fs::write(&staged, &bytes).await?;
            tokio::fs::rename(&staged, &path).await
        }
        .await;
        if written.is_err() {
            let _ = tokio::fs::remove_file(&staged).await;
        }
        written.is_ok()
    }
}

/// 🧵️ How many codec rows of one package are interpreted at once: the declared residency bound, never
/// more than half the cores this hub may use, never fewer than one.
pub fn guest_verification_concurrency() -> usize {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    TRUSTED_CATALOG_GUEST_RESIDENCY.concurrent_verifications.min(cores / 2).max(1)
}
