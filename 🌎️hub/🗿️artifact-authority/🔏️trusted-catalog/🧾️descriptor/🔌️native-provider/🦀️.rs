//! 🔌️ The native codec provider vocabulary: explicit bindings, the borrowed package identity, the requirement, the provider port and the binding validation.

use super::*;

/// 🔗️ One explicitly linked native executable for an exact bundle artifact identity.
#[derive(Clone)]
pub struct NativeCodecBinding {
    pub plugin_id: String,
    pub package_id: String,
    pub artifact_kind: String,
    pub factory_id: Option<String>,
    pub codec: ArtifactCodec,
}

impl NativeCodecBinding {
    /// 🪢️ Binds a native executable without deriving package identity from plugin identity.
    pub fn new(plugin_id: impl Into<String>, package_id: impl Into<String>, artifact_kind: impl Into<String>, factory_id: Option<String>, codec: ArtifactCodec) -> Self {
        Self { plugin_id: plugin_id.into(), package_id: package_id.into(), artifact_kind: artifact_kind.into(), factory_id, codec }
    }

    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    pub fn artifact_kind(&self) -> &str {
        &self.artifact_kind
    }

    pub fn codec(&self) -> &ArtifactCodec {
        &self.codec
    }

    /// 🪪️ Returns only factory identity retained from an actual verified native receipt.
    pub fn factory_id(&self)->Option<&str>{self.factory_id.as_deref()}
}

/// 🪪️ Borrowed immutable package identity passed from the trusted loader to one native provider.
#[derive(Clone, Copy)]
pub struct NativeCodecProviderPackageV1<'a> {
    pub plugin_id: &'a str,
    pub package_id: &'a str,
    pub version: &'a str,
}

/// 🧬️ One descriptor-committed codec requirement exposed without product-plugin types.
#[derive(Clone, Copy)]
pub struct NativeCodecProviderRequirementV1<'a> {
    pub package: NativeCodecProviderPackageV1<'a>,
    pub artifact_kind: &'a str,
    pub artifact_schema: &'a str,
    pub pack_schema_hash: &'a str,
}

/// 🔌️ Headless Hub provider port. Implementations may supply executable codecs only for the exact
/// verified package closure and cannot publish them outside the loader's atomic registration.
pub trait NativeCodecProviderSourceV1: Sync {
    fn preflight_selection(&self, _selected: &[NativeCodecProviderRequirementV1<'_>]) -> Result<(), AuthorityError> {
        Ok(())
    }

    fn preview(&self, package: NativeCodecProviderPackageV1<'_>, descriptor: &PackageDescriptor, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError>;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodecKey {
    pub plugin_id: String,
    pub package_id: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
}

impl CodecKey {
    pub fn from_parts(plugin_id: &str, package_id: &str, artifact_kind: &str, artifact_schema: &str) -> Self {
        Self { plugin_id: plugin_id.to_string(), package_id: package_id.to_string(), artifact_kind: artifact_kind.to_string(), artifact_schema: artifact_schema.to_string() }
    }
}

pub fn validate_native_bindings(bindings: &[NativeCodecBinding]) -> Result<BTreeMap<CodecKey, &NativeCodecBinding>, AuthorityError> {
    ensure_count(bindings.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
    let mut map = BTreeMap::new();
    for binding in bindings {
        if !valid_identity(&binding.plugin_id) || !valid_package_id(&binding.package_id) || !valid_identity(&binding.artifact_kind) || !valid_identity(&binding.codec.schema) {
            return Err(catalog("native codec binding identity is invalid"));
        }
        let key = CodecKey::from_parts(&binding.plugin_id, &binding.package_id, &binding.artifact_kind, &binding.codec.schema);
        if map.insert(key, binding).is_some() {
            return Err(catalog("native codec binding identity is duplicated"));
        }
    }
    Ok(map)
}
