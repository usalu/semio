//! 🪢 Hub-private static native openable catalog provider.

use super::trusted_catalog::{NativeCodecBinding, NativeCodecProviderPackageV1, NativeCodecProviderSourceV1};
use super::{AuthorityError, OperationContext};
use semio_s_plugin_stdio::registry::NativeCodecFactoryReceipt;
use std::collections::BTreeSet;

/// 🚪 Exact identity of the statically linked provider admitted by this boundary.
pub const NATIVE_OPENABLE_PROVIDER_SET_V1_ID: &str = "stdio+gis+vcs/native-codecs/v1";
/// 🧮 Complete fixed stdio, GIS and VCS factory closure admitted by V1.
pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 29;
const NATIVE_STDIO_PROVIDER_RECEIPTS: usize = 26;
const NATIVE_VCS_PROVIDER_RECEIPTS: usize = 1;

struct NativeCodecProviderEntryV1 {
    plugin_id: &'static str,
    package_id: &'static str,
    preview: fn(&str, &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError>,
}

/// 🗝️ Fixed native provider inventory; construction does not invoke factories or publish bindings.
pub struct NativeCodecProviderSetV1 {
    entries: &'static [NativeCodecProviderEntryV1],
}

impl NativeCodecProviderSetV1 {
    /// 🧷️ Selects only compiled-in providers; a package outside this table has no native codec authority.
    pub const fn linked() -> Self {
        Self { entries: &[NativeCodecProviderEntryV1 { plugin_id: "stdio", package_id: "semio:stdio", preview: preview_stdio_bindings }, NativeCodecProviderEntryV1 { plugin_id: "gis", package_id: "semio:gis", preview: preview_gis_bindings }, NativeCodecProviderEntryV1 { plugin_id: "vcs", package_id: "semio:vcs", preview: preview_vcs_bindings }] }
    }

    pub(crate) fn preview(&self, plugin_id: &str, package_id: &str, version: &str, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
        context.checkpoint()?;
        let mut matches = self.entries.iter().filter(|entry| entry.plugin_id == plugin_id && entry.package_id == package_id);
        let selected = matches.next().ok_or_else(|| provider_error("selected package has no compiled native provider"))?;
        if matches.next().is_some() {
            return Err(provider_error("compiled native provider identity is duplicated"));
        }
        let bindings = (selected.preview)(version, context)?;
        context.checkpoint()?;
        Ok(bindings)
    }
}

impl NativeCodecProviderSourceV1 for NativeCodecProviderSetV1 {
    fn preview(&self, package: NativeCodecProviderPackageV1<'_>, descriptor: &semio_framework::PackageDescriptor, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
        context.checkpoint()?;
        if descriptor.package_id != package.package_id || descriptor.manifest.plugin_id != package.plugin_id || descriptor.manifest.version != package.version {
            return Err(provider_error("decoded descriptor has a different native package identity"));
        }
        if package.plugin_id == "stdio" {
            semio_s_plugin_stdio::registry::validate_native_codec_artifact_kinds(&descriptor.manifest.artifact_kinds).map_err(provider_error)?;
            semio_s_plugin_stdio::registry::validate_native_artifact_catalog_contributions(&descriptor.manifest.topic_contributions).map_err(provider_error)?;
        } else if matches!(package.plugin_id, "gis" | "vcs") {
            semio_s_plugin_stdio::registry::validate_native_artifact_catalog_dependency(&descriptor.manifest.dependencies).map_err(provider_error)?;
            semio_s_plugin_stdio::registry::validate_native_artifact_catalog_contributions(&descriptor.manifest.topic_contributions).map_err(provider_error)?;
        }
        self.preview(package.plugin_id, package.package_id, package.version, context)
    }
}

fn preview_stdio_bindings(version: &str, _context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
    Ok(NativeOpenableCatalogProviderV1::linked(version)?.into_bindings())
}

fn preview_gis_bindings(version: &str, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
    let rejected = || AuthorityError::Catalog("gis/native-codecs/v1: exact private receipt rejected".into());
    context.checkpoint()?;
    let receipts = semio_s_plugin_gis::native_codecs::native_codec_factory_receipts().map_err(|_| rejected())?;
    let mut bindings = Vec::with_capacity(receipts.len());
    let mut factories = BTreeSet::new();
    let mut artifacts = BTreeSet::new();
    for receipt in receipts {
        context.checkpoint()?;
        let identity = receipt.identity();
        if identity.plugin_id != "gis"
            || identity.package_id != "semio:gis"
            || identity.package_version != version
            || identity.pack_schema_hash == [0; 32]
            || !factories.insert(identity.factory_id)
            || !artifacts.insert((identity.artifact_kind, identity.schema))
        {
            return Err(rejected());
        }
        let (codec, genesis) = receipt.into_codec_and_genesis().map_err(|_| rejected())?;
        if codec.schema != identity.schema || codec.extension != identity.extension || codec.pack_schema_hash != identity.pack_schema_hash {
            return Err(rejected());
        }
        context.checkpoint()?;
        bindings.push(NativeCodecBinding::with_genesis(identity.plugin_id, identity.package_id, identity.artifact_kind, codec, genesis));
    }
    Ok(bindings)
}

fn preview_vcs_bindings(version: &str, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
    let rejected = || AuthorityError::Catalog("vcs/native-codecs/v1: exact private receipt rejected".into());
    context.checkpoint()?;
    let receipts = semio_s_plugin_vcs::native_codecs::native_codec_factory_receipts().map_err(|_| rejected())?;
    if receipts.len() != NATIVE_VCS_PROVIDER_RECEIPTS {
        return Err(rejected());
    }
    let mut bindings = Vec::with_capacity(receipts.len());
    let mut factories = BTreeSet::new();
    let mut artifacts = BTreeSet::new();
    for receipt in receipts {
        context.checkpoint()?;
        let identity = receipt.identity();
        if identity.plugin_id != "vcs" || identity.package_id != "semio:vcs" || identity.package_version != version
            || identity.factory_id != "vcs.vcs.v1" || identity.artifact_kind != "s.vcs.vcs" || identity.schema != "vcs.vcs" || identity.extension != "vcs" || identity.capability != "s.vcs.vcs.codec.document"
            || identity.pack_schema_hash == [0; 32] || !factories.insert(identity.factory_id) || !artifacts.insert((identity.artifact_kind, identity.schema)) {
            return Err(rejected());
        }
        let (codec, genesis) = receipt.into_codec_and_genesis().map_err(|_| rejected())?;
        if codec.schema != identity.schema || codec.extension != identity.extension || codec.pack_schema_hash != identity.pack_schema_hash {
            return Err(rejected());
        }
        context.checkpoint()?;
        bindings.push(NativeCodecBinding::with_genesis(identity.plugin_id, identity.package_id, identity.artifact_kind, codec, genesis));
    }
    if factories.len() != NATIVE_VCS_PROVIDER_RECEIPTS || artifacts.len() != NATIVE_VCS_PROVIDER_RECEIPTS {
        return Err(rejected());
    }
    Ok(bindings)
}

/// 🔐 Complete in-memory provider result that cannot publish codecs by itself.
pub struct NativeOpenableCatalogProviderV1 {
    bindings: Vec<NativeCodecBinding>,
}

impl NativeOpenableCatalogProviderV1 {
    /// 🧬 Revalidates and consumes the complete static receipt closure atomically.
    pub fn linked(version: &str) -> Result<Self, AuthorityError> {
        Self::from_receipts(version, semio_s_plugin_stdio::registry::native_codec_factory_receipts().map_err(provider_error)?)
    }

    /// 🪢 Releases the complete vector only to the trusted catalog loader.
    pub fn into_bindings(self) -> Vec<NativeCodecBinding> {
        self.bindings
    }

    fn from_receipts(version: &str, receipts: Vec<NativeCodecFactoryReceipt>) -> Result<Self, AuthorityError> {
        if receipts.len() != NATIVE_STDIO_PROVIDER_RECEIPTS {
            return Err(provider_error(format!("provider closure has {} receipts", receipts.len())));
        }
        let mut factories = BTreeSet::new();
        let mut descriptor_codecs = BTreeSet::new();
        let mut artifact_schemas = BTreeSet::new();
        let mut bindings = Vec::with_capacity(receipts.len());
        for receipt in receipts {
            if receipt.plugin_id != "stdio"
                || receipt.package_id != "semio:stdio"
                || receipt.package_version != version
                || receipt.pack_schema_hash == [0; 32]
                || !factories.insert(receipt.factory_id.clone())
                || !descriptor_codecs.insert(receipt.descriptor_codec_id.clone())
                || !artifact_schemas.insert((receipt.artifact_kind.clone(), receipt.schema.clone()))
            {
                return Err(provider_error(format!("receipt {} is not a unique exact stdio authority", receipt.factory_id)));
            }
            let codec = receipt.instantiate().map_err(provider_error)?;
            if codec.schema != receipt.schema || codec.extension != receipt.extension || codec.pack_schema_hash != receipt.pack_schema_hash {
                return Err(provider_error(format!("factory {} result differs from its receipt", receipt.factory_id)));
            }
            bindings.push(NativeCodecBinding::new(receipt.plugin_id, receipt.package_id, receipt.artifact_kind, codec));
        }
        if factories.len() != NATIVE_STDIO_PROVIDER_RECEIPTS || descriptor_codecs.len() != NATIVE_STDIO_PROVIDER_RECEIPTS || artifact_schemas.len() != NATIVE_STDIO_PROVIDER_RECEIPTS {
            return Err(provider_error("provider receipt closure is incomplete"));
        }
        Ok(Self { bindings })
    }
}

fn provider_error(error: impl std::fmt::Display) -> AuthorityError {
    AuthorityError::Catalog(format!("{NATIVE_OPENABLE_PROVIDER_SET_V1_ID}: {error}"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
