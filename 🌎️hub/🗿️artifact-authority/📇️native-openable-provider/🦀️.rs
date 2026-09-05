//! 🪢 Hub-private static native openable catalog provider.

use super::trusted_catalog::NativeCodecBinding;
use super::{AuthorityError, OperationContext};
use semio_s_plugin_stdio::registry::NativeCodecFactoryReceipt;
use std::collections::BTreeSet;

/// 🚪 Exact identity of the statically linked provider admitted by this boundary.
pub const NATIVE_OPENABLE_PROVIDER_SET_V1_ID: &str = "stdio+gis/native-codecs/v1";
/// 🧮 Complete fixed stdio and GIS factory closure admitted by V1.
pub const NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 28;
const NATIVE_STDIO_PROVIDER_RECEIPTS: usize = 26;

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
    /// 🧷️ Selects only compiled-in providers; VCS remains absent until its private receipt is verified.
    pub const fn linked() -> Self {
        Self {
            entries: &[
                NativeCodecProviderEntryV1 { plugin_id: "stdio", package_id: "semio:stdio", preview: preview_stdio_bindings },
                NativeCodecProviderEntryV1 { plugin_id: "gis", package_id: "semio:gis", preview: preview_gis_bindings },
            ],
        }
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
        if identity.plugin_id != "gis" || identity.package_id != "semio:gis" || identity.package_version != version
            || identity.pack_schema_hash == [0; 32] || !factories.insert(identity.factory_id) || !artifacts.insert((identity.artifact_kind, identity.schema)) {
            return Err(rejected());
        }
        let codec = receipt.into_codec().map_err(|_| rejected())?;
        if codec.schema != identity.schema || codec.extension != identity.extension || codec.pack_schema_hash != identity.pack_schema_hash {
            return Err(rejected());
        }
        context.checkpoint()?;
        bindings.push(NativeCodecBinding::new(identity.plugin_id, identity.package_id, identity.artifact_kind, codec));
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
        if factories.len() != NATIVE_STDIO_PROVIDER_RECEIPTS
            || descriptor_codecs.len() != NATIVE_STDIO_PROVIDER_RECEIPTS
            || artifact_schemas.len() != NATIVE_STDIO_PROVIDER_RECEIPTS
        {
            return Err(provider_error("provider receipt closure is incomplete"));
        }
        Ok(Self { bindings })
    }
}

fn provider_error(error: impl std::fmt::Display) -> AuthorityError {
    AuthorityError::Catalog(format!("{NATIVE_OPENABLE_PROVIDER_SET_V1_ID}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn receipts() -> Vec<NativeCodecFactoryReceipt> {
        semio_s_plugin_stdio::registry::native_codec_factory_receipts().expect("verified stdio receipts")
    }

    #[test]
    fn native_openable_provider_consumes_exact_complete_stdio_factory_closure() {
        let provider = NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), receipts()).expect("complete provider");
        assert_eq!(provider.into_bindings().len(), NATIVE_STDIO_PROVIDER_RECEIPTS);
    }

    #[test]
    fn native_openable_provider_rejects_missing_extra_and_duplicate_receipts_without_publication() {
        let mut missing = receipts();
        missing.pop();
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), missing).is_err());

        let mut extra = receipts();
        extra.push(extra[0].clone());
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), extra).is_err());

        let mut duplicate = receipts();
        duplicate[1].factory_id = duplicate[0].factory_id.clone();
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), duplicate).is_err());
    }

    #[test]
    fn native_openable_provider_rejects_identity_hash_schema_and_factory_substitution() {
        let mut wrong_version = receipts();
        wrong_version[0].package_version = "9.9.9";
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), wrong_version).is_err());

        let mut wrong_plugin = receipts();
        wrong_plugin[0].plugin_id = "foreign";
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), wrong_plugin).is_err());

        let mut zero_hash = receipts();
        zero_hash[0].pack_schema_hash = [0; 32];
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), zero_hash).is_err());

        let mut wrong_schema = receipts();
        wrong_schema[0].schema = "stdio.foreign".into();
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), wrong_schema).is_err());

        let mut substituted_factory = receipts();
        substituted_factory[0].factory = substituted_factory[1].factory;
        assert!(NativeOpenableCatalogProviderV1::from_receipts(env!("CARGO_PKG_VERSION"), substituted_factory).is_err());
    }
}
