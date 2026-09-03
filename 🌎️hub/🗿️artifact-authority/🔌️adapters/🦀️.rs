//! 🔌️ Production adapters from the hub-owned artifact-authority ports to the live plugin host
//! and registered artifact codecs.

use super::{AcceptedArtifactOperation, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactIdentity};
use directory::os_directory::hex_lower;
use directory::os_store::{document_codec, ArtifactCodec};
use semio_framework_plugin_host::{PackageRef, PluginGraph};
use std::fmt::Write;

/// 🧯️ Immutable ceiling for package handles admitted into one live catalog snapshot.
pub const TRUSTED_CATALOG_MAX_PACKAGES: usize = 4_096;
/// 🧯️ Immutable ceiling for executable artifact identities in one live catalog snapshot.
pub const TRUSTED_CATALOG_MAX_CODECS: usize = 16_384;
/// 🧯️ Immutable ceiling for diagnostic text retained across an authority boundary.
pub const AUTHORITY_MAX_DIAGNOSTIC_BYTES: usize = 4_096;
/// 🧯️ Immutable ceiling for codec mirror/log output accepted during validation/application.
pub const AUTHORITY_MAX_CODEC_TEXT_BYTES: usize = 64 * 1024 * 1024;

struct BoundedDiagnostic {
    value: String,
}

impl Write for BoundedDiagnostic {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        let remaining = AUTHORITY_MAX_DIAGNOSTIC_BYTES.saturating_sub(self.value.len());
        if remaining == 0 {
            return Ok(());
        }
        let mut end = remaining.min(value.len());
        while !value.is_char_boundary(end) {
            end -= 1;
        }
        self.value.push_str(&value[..end]);
        Ok(())
    }
}

pub(crate) fn bounded_message(error: impl std::fmt::Display) -> String {
    let mut diagnostic = BoundedDiagnostic { value: String::with_capacity(AUTHORITY_MAX_DIAGNOSTIC_BYTES) };
    let _ = write!(&mut diagnostic, "{error}");
    diagnostic.value
}

fn codec_text_bytes(dsl: &str, ops: &str) -> Result<usize, AuthorityError> {
    dsl.len().checked_add(ops.len()).filter(|length| *length <= AUTHORITY_MAX_CODEC_TEXT_BYTES).ok_or(AuthorityError::ResourceLimit("codec text byte"))
}

/// 🧬️ One exact host-manifest identity bound to its registered executable codec.
pub struct PluginHostArtifactCodec {
    identity: TrustedArtifactIdentity,
    codec: ArtifactCodec,
}

impl TrustedArtifactCodec for PluginHostArtifactCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let mirror = (self.codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        codec_text_bytes(&mirror.dsl, &mirror.ops)?;
        context.checkpoint()
    }

    async fn apply_operation(&self, pair: ArtifactPair, operation: &AcceptedArtifactOperation, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let encoded = directory::os_spr::encode_ops_vec(std::slice::from_ref(&operation.encoded));
        let (pack, spr, ops) = (self.codec.apply_ops_binary)(&pair.pack, &pair.spr, &encoded).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

/// 🗂️ Immutable snapshot of exact package-host identities and registered executable codecs.
pub struct PluginHostTrustedArtifactCatalog {
    codecs: Box<[PluginHostArtifactCodec]>,
}

/// 🔗️ Trusted live association between a plugin manifest key and its independently identified package.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LivePluginPackageBinding {
    plugin_id: String,
    package_id: String,
    package_hash: [u8; 32],
}

impl LivePluginPackageBinding {
    /// 🪢️ Captures the host-owned package identity without retaining a plugin-host type.
    pub fn from_host(plugin_id: impl Into<String>, package: &PackageRef) -> Self {
        Self { plugin_id: plugin_id.into(), package_id: package.package.0.clone(), package_hash: package.hash.0 }
    }
}

impl PluginHostTrustedArtifactCatalog {
    /// 🏭️ Resolves live plugin manifests and codec registrations into one exact trusted snapshot.
    pub async fn load(graph: &PluginGraph, bindings: &[LivePluginPackageBinding], context: &OperationContext<'_>) -> Result<Self, AuthorityError> {
        if bindings.is_empty() || bindings.len() > TRUSTED_CATALOG_MAX_PACKAGES {
            return Err(AuthorityError::ResourceLimit("trusted package count"));
        }
        let total_units = u64::try_from(bindings.len()).map_err(|_| AuthorityError::ResourceLimit("trusted package count"))?;
        let mut codecs = Vec::new();
        for (index, binding) in bindings.iter().enumerate() {
            context.checkpoint()?;
            if bindings[..index].iter().any(|existing| existing.plugin_id == binding.plugin_id || existing.package_id == binding.package_id) {
                return Err(AuthorityError::Catalog("trusted plugin/package binding is duplicated".to_string()));
            }
            if binding.plugin_id.is_empty() || binding.package_id.is_empty() || binding.package_hash == [0; 32] {
                return Err(AuthorityError::Catalog("trusted package identity is empty or zero".to_string()));
            }
            let manifest = graph.manifest(&binding.plugin_id).await.map_err(|error| AuthorityError::Catalog(bounded_message(error)))?.ok_or_else(|| AuthorityError::Catalog("trusted plugin has no loaded manifest".to_string()))?;
            context.checkpoint()?;
            if manifest.plugin_id != binding.plugin_id || manifest.version.is_empty() {
                return Err(AuthorityError::Catalog("loaded manifest does not exactly own its plugin identity".to_string()));
            }
            let next_len = codecs.len().checked_add(manifest.artifact_kinds.len()).ok_or(AuthorityError::ResourceLimit("trusted codec count"))?;
            if next_len > TRUSTED_CATALOG_MAX_CODECS {
                return Err(AuthorityError::ResourceLimit("trusted codec count"));
            }
            for kind in manifest.artifact_kinds {
                context.checkpoint()?;
                if kind.id.is_empty() || kind.schema.is_empty() {
                    return Err(AuthorityError::Catalog("loaded artifact kind identity is empty".to_string()));
                }
                let codec = document_codec(&kind.schema).await.map_err(|error| AuthorityError::Catalog(bounded_message(error)))?.ok_or_else(|| AuthorityError::Catalog("artifact schema has no registered executable codec".to_string()))?;
                context.checkpoint()?;
                if codec.schema != kind.schema || codec.pack_schema_hash == [0; 32] {
                    return Err(AuthorityError::Catalog("registered codec schema identity is empty or mismatched".to_string()));
                }
                let identity = TrustedArtifactIdentity {
                    plugin_id: manifest.plugin_id.clone(),
                    package_id: binding.package_id.clone(),
                    version: manifest.version.clone(),
                    package_hash: hex_lower(&binding.package_hash),
                    artifact_kind: kind.id,
                    artifact_schema: kind.schema,
                    pack_schema_hash: hex_lower(&codec.pack_schema_hash),
                };
                if codecs.iter().any(|entry: &PluginHostArtifactCodec| entry.identity == identity) {
                    return Err(AuthorityError::Catalog("duplicate exact trusted artifact identity".to_string()));
                }
                codecs.push(PluginHostArtifactCodec { identity, codec });
            }
            let completed_units = u64::try_from(index + 1).map_err(|_| AuthorityError::ResourceLimit("trusted package count"))?;
            context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogLoading, completed_units, total_units })?;
        }
        Ok(Self { codecs: codecs.into_boxed_slice() })
    }
}

impl TrustedArtifactCatalog for PluginHostTrustedArtifactCatalog {
    type Codec = PluginHostArtifactCodec;

    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.codecs.iter().find(|codec| &codec.identity == required).ok_or_else(|| AuthorityError::Catalog("descriptor identity is absent from the trusted live catalog snapshot".to_string()))
    }
}
