//! 🪪️ Artifact identity, capabilities, localization and definitions.
use crate::sdk_fault::ArtifactDefinitionError;
use crate::sdk_fault::PluginAssemblyError;
use std::collections::BTreeMap;
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactKindId;

/// 🪪️ Admits one canonical artifact root owned by this exact plugin identity.
pub(crate) fn preflight_artifact_identity(plugin_id: &str, kind: &str) -> Result<(), PluginAssemblyError> {
    let canonical = ArtifactKindId::parse(kind).map_err(|_| PluginAssemblyError::new("plugin-assembly.artifact-kind", "artifact identity must use canonical <domain>.<plugin>.<artifact> grammar"))?;
    if canonical.plugin() != plugin_id {
        return Err(PluginAssemblyError::new("plugin-assembly.artifact-owner", "artifact identity is not owned by the declaring plugin"));
    }
    Ok(())
}

/// 🪪️ Canonical, dot-delimited identity with explicit hierarchy.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactIdentity {
    pub(crate) value: String,
    segments: Vec<String>,
}

impl ArtifactIdentity {
    /// 🏗️ Parses a canonical identity whose segments contain only lower-case ASCII letters,
    /// digits, `_`, or `-`.
    pub fn parse(value: impl Into<String>) -> Result<Self, ArtifactDefinitionError> {
        let value = value.into();
        if value.is_empty() || value.starts_with('.') || value.ends_with('.') {
            return Err(ArtifactDefinitionError::new("artifact-definition.identity", format!("identity {value:?} must contain non-empty dot-delimited segments")));
        }
        let segments: Vec<String> = value.split('.').map(str::to_owned).collect();
        if segments.iter().any(|segment| segment.is_empty() || !segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-'))) {
            return Err(ArtifactDefinitionError::new("artifact-definition.identity", format!("identity {value:?} has a non-canonical segment")));
        }
        Ok(Self { value, segments })
    }

    /// 🧭️ Returns the canonical, stable serialized identity.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// 🌿️ Returns the validated path segments in hierarchical order.
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// 🌿️ Returns this identity extended by one validated segment.
    pub fn child(&self, segment: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        let segment = segment.as_ref();
        Self::validate_segment(segment, "identity")?;
        Self::parse(format!("{}.{}", self.value, segment))
    }

    /// 🌳️ Whether `self` is the same identity as, or a descendant of, `ancestor`.
    pub fn is_within(&self, ancestor: &Self) -> bool {
        self.segments.len() >= ancestor.segments.len() && self.segments[..ancestor.segments.len()] == ancestor.segments[..]
    }

    fn validate_segment(value: &str, label: &str) -> Result<(), ArtifactDefinitionError> {
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-')) {
            return Err(ArtifactDefinitionError::new("artifact-definition.identity", format!("{label} segment {value:?} is not canonical")));
        }
        Ok(())
    }
}

impl std::fmt::Display for ArtifactIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.value)
    }
}

/// 🏷️ Open-ended capability category. New categories are data, not central-dispatch edits.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactCapabilityKind(String);

impl ArtifactCapabilityKind {
    /// 🏗️ Creates a canonical category identity.
    pub fn parse(value: impl Into<String>) -> Result<Self, ArtifactDefinitionError> {
        let identity = ArtifactIdentity::parse(value)?;
        Ok(Self(identity.value))
    }

    /// 🧬️ Schema capability category.
    pub fn schema() -> Self {
        Self("schema".into())
    }

    /// 🏅️ Standard capability category.
    pub fn standard() -> Self {
        Self("standard".into())
    }

    /// 🪆️ Profile capability category.
    pub fn profile() -> Self {
        Self("profile".into())
    }

    /// 🚪️ Source-dialect capability category.
    pub fn source_dialect() -> Self {
        Self("source-dialect".into())
    }

    /// 🎭️ Representation capability category.
    pub fn representation() -> Self {
        Self("representation".into())
    }

    /// 🗜️ Codec capability category.
    pub fn codec() -> Self {
        Self("codec".into())
    }

    /// 🧬️ Mutation capability category.
    pub fn mutation() -> Self {
        Self("mutation".into())
    }

    /// 🧠️ Inference capability category.
    pub fn inference() -> Self {
        Self("inference".into())
    }

    /// 📦️ Resource capability category.
    pub fn resource() -> Self {
        Self("resource".into())
    }

    /// 🗺️ Localization capability category.
    pub fn localization() -> Self {
        Self("localization".into())
    }

    /// ✅️ Conformance-suite capability category.
    pub fn conformance_suite() -> Self {
        Self("conformance-suite".into())
    }

    /// 🧩️ Extension capability category.
    pub fn extension() -> Self {
        Self("extension".into())
    }

    /// 📖️ Grammar capability category.
    pub fn grammar() -> Self {
        Self("grammar".into())
    }

    /// 🎹️ Composer capability category.
    pub fn composer() -> Self {
        Self("composer".into())
    }

    /// 🧾️ Subset-validator capability category.
    pub fn subset_validator() -> Self {
        Self("subset-validator".into())
    }

    /// 🧭️ Returns the category's canonical identity.
    /// 🚫️async: E1 pure accessor consumed by sync iterator-adapter closures
    /// (`capabilities_of`/`standards`/etc.) — see R9.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🏷️ Namespace for an externally visible identity claimed by a capability.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactIdentityNamespace(String);

impl ArtifactIdentityNamespace {
    /// 🏗️ Creates a canonical namespace identity.
    pub fn parse(value: impl Into<String>) -> Result<Self, ArtifactDefinitionError> {
        Ok(Self(ArtifactIdentity::parse(value)?.value))
    }

    /// 🗿️ Artifact identity namespace.
    pub fn artifact() -> Self {
        Self("artifact".into())
    }

    /// 🧬️ Schema identity namespace.
    pub fn schema() -> Self {
        Self("schema".into())
    }

    /// 🚪️ Dialect identity namespace.
    pub fn dialect() -> Self {
        Self("dialect".into())
    }

    /// 🧾️ Exact dialect validation authority, distinct from dialect composition authority.
    pub fn validated_dialect() -> Self {
        Self("validated-dialect".into())
    }

    /// 🗜️ Codec identity namespace.
    pub fn codec() -> Self {
        Self("codec".into())
    }

    /// 📮️ MIME identity namespace.
    pub fn mime() -> Self {
        Self("mime".into())
    }

    /// 🗂️ File-extension identity namespace.
    pub fn extension() -> Self {
        Self("extension".into())
    }

    /// 🧩️ Extension implementation identity namespace.
    pub fn extension_implementation() -> Self {
        Self("extension-implementation".into())
    }

    /// 📖️ Grammar identity namespace.
    pub fn grammar() -> Self {
        Self("grammar".into())
    }

    /// 📑️ Shared schema document identity namespace: one named export of the OS-wide schema export registry.
    pub fn schema_export() -> Self {
        Self("schema-export".into())
    }

    /// 🧭️ Returns the namespace's canonical identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 📌️ One unique, externally addressable identity owned by a capability.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactIdentityClaim {
    namespace: ArtifactIdentityNamespace,
    pub(crate) value: String,
}

impl ArtifactIdentityClaim {
    /// 🗜️ Binds an exact extension to one codec schema using unambiguous UTF-8 length framing.
    pub fn codec_extension(schema: &str, extension: &str) -> Result<Self, ArtifactDefinitionError> {
        if schema.trim().is_empty() || extension.trim().is_empty() {
            return Err(ArtifactDefinitionError::new("artifact-definition.claim", "codec extension authority requires a schema and extension"));
        }
        Self::new(ArtifactIdentityNamespace::parse("codec-extension")?, format!("{}:{schema}:{extension}", schema.len()))
    }

    /// 🧷️ Claims the named export `export` of `scope` in the OS-wide schema export registry as `<scope>#<export>` — its own
    /// namespace, so a shared schema document never reads as (nor collides with) an artifact schema id such as a subset's.
    pub fn schema_export(scope: &str, export: &str) -> Result<Self, ArtifactDefinitionError> {
        if scope.trim().is_empty() || export.trim().is_empty() || scope.contains('#') || export.contains('#') {
            return Err(ArtifactDefinitionError::new("artifact-definition.claim", "schema export authority requires a scope and an export id without `#`"));
        }
        Self::new(ArtifactIdentityNamespace::schema_export(), format!("{scope}#{export}"))
    }

    /// 🏗️ Creates a non-empty claim; caller-selectable namespaces keep this contract open.
    pub fn new(namespace: ArtifactIdentityNamespace, value: impl Into<String>) -> Result<Self, ArtifactDefinitionError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ArtifactDefinitionError::new("artifact-definition.claim", "identity claim values must be non-empty"));
        }
        Ok(Self { namespace, value })
    }

    /// 🏷️ Returns the claim namespace.
    pub fn namespace(&self) -> &ArtifactIdentityNamespace {
        &self.namespace
    }

    /// 🧭️ Returns the exact identity value; MIME and extension values are deliberately not rewritten.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// 🌐️ Explicit locale identity. There is intentionally no default locale.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactLocale(String);

impl ArtifactLocale {
    /// 🏗️ Parses a lower-case BCP-47-like locale tag.
    pub fn parse(value: impl Into<String>) -> Result<Self, ArtifactDefinitionError> {
        let value = value.into();
        if value.is_empty() || value.starts_with('-') || value.ends_with('-') || value.contains("--") || !value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') {
            return Err(ArtifactDefinitionError::new("artifact-definition.locale", format!("locale {value:?} must be an explicit lower-case BCP-47-like tag")));
        }
        Ok(Self(value))
    }

    /// 🧭️ Returns the explicit locale tag.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🗺️ Localized descriptor text for exactly one explicit locale.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactLocalization {
    locale: ArtifactLocale,
    text: String,
}

impl ArtifactLocalization {
    /// 🏗️ Creates localized text. Locale is mandatory and text must be non-empty.
    pub fn new(locale: ArtifactLocale, text: impl Into<String>) -> Result<Self, ArtifactDefinitionError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(ArtifactDefinitionError::new("artifact-definition.localization", "localized text must be non-empty"));
        }
        Ok(Self { locale, text })
    }

    /// 🌐️ Returns the explicitly declared locale.
    pub fn locale(&self) -> &ArtifactLocale {
        &self.locale
    }

    /// 📝 Returns localized text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// 🗂️ Borrowed capability identity, kind, descriptor, claims and optional localization.
pub type ArtifactCapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);

/// 🧩️ One extension-owned capability and its conflict-checked external identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCapability {
    pub(crate) identity: ArtifactIdentity,
    kind: ArtifactCapabilityKind,
    descriptor: Vec<u8>,
    executable_identity: Option<ArtifactExecutableIdentity>,
    claims: Vec<ArtifactIdentityClaim>,
    localizations: Vec<ArtifactLocalization>,
}

/// ⚙️ Stable process-local identity of a non-capturing capability executable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactExecutableIdentity(usize);

impl ArtifactExecutableIdentity {
    /// 🧷️ Captures executable identity without exposing a runtime dependency type.
    pub fn from_function(function: fn()) -> Self {
        Self(function as usize)
    }

    /// 🧬️ Captures an actual typed function pointer erased to its process-local code address.
    pub fn from_function_pointer(function: *const ()) -> Self {
        Self(function as usize)
    }
}

impl ArtifactCapability {
    /// 🏗️ Constructs a capability at a full hierarchical identity.
    pub fn new(identity: ArtifactIdentity, kind: ArtifactCapabilityKind) -> Self {
        Self { identity, kind, descriptor: Vec::new(), executable_identity: None, claims: Vec::new(), localizations: Vec::new() }
    }

    /// 🧾️ Attaches the canonical byte descriptor whose exact bytes participate in idempotence.
    pub fn descriptor(mut self, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let descriptor = descriptor.into();
        if descriptor.is_empty() {
            return Err(ArtifactDefinitionError::new("artifact-definition.descriptor", format!("capability {} has an empty descriptor", self.identity)));
        }
        self.descriptor = descriptor;
        Ok(self)
    }

    /// ⚙️ Attaches the non-capturing executable identity whose exact value participates in idempotence.
    pub fn executable(mut self, identity: ArtifactExecutableIdentity) -> Self {
        self.executable_identity = Some(identity);
        self
    }

    /// 📌️ Adds an exclusive external identity claim.
    pub fn claim(mut self, claim: ArtifactIdentityClaim) -> Result<Self, ArtifactDefinitionError> {
        if self.claims.contains(&claim) {
            return Err(ArtifactDefinitionError::new("artifact-definition.duplicate-claim", format!("capability {} repeats {}:{}", self.identity, claim.namespace.as_str(), claim.value)));
        }
        self.claims.push(claim);
        self.claims.sort();
        Ok(self)
    }

    /// 🗺️ Adds localized text. An explicit locale can occur at most once per capability.
    pub fn localization(mut self, localization: ArtifactLocalization) -> Result<Self, ArtifactDefinitionError> {
        if self.localizations.iter().any(|existing| existing.locale == localization.locale) {
            return Err(ArtifactDefinitionError::new("artifact-definition.duplicate-locale", format!("capability {} repeats locale {}", self.identity, localization.locale.as_str())));
        }
        self.localizations.push(localization);
        self.localizations.sort_by(|left, right| left.locale.cmp(&right.locale));
        Ok(self)
    }

    /// 🪪️ Returns the hierarchical capability identity.
    /// 🚫️async: E1 pure accessor consumed by sync iterator-adapter closures (`.filter(|c| ...)`
    /// in `capabilities_of`/`standards`/etc. below, whose `impl Iterator` return type admits no
    /// suspension point) — see R9.
    pub fn identity(&self) -> &ArtifactIdentity {
        &self.identity
    }

    /// 🏷️ Returns the open-ended capability category.
    /// 🚫️async: E1 pure accessor — see `identity`'s own comment above.
    pub fn kind(&self) -> &ArtifactCapabilityKind {
        &self.kind
    }

    /// 🧾️ Returns the exact canonical descriptor bytes.
    pub fn descriptor_bytes(&self) -> &[u8] {
        &self.descriptor
    }

    /// ⚙️ Returns the optional executable identity.
    pub fn executable_identity(&self) -> Option<ArtifactExecutableIdentity> {
        self.executable_identity
    }

    /// 📌️ Returns deterministically ordered external identity claims.
    pub fn claims(&self) -> &[ArtifactIdentityClaim] {
        &self.claims
    }

    /// 🗺️ Returns deterministically ordered localized descriptor text.
    pub fn localizations(&self) -> &[ArtifactLocalization] {
        &self.localizations
    }
}

/// 🗿️ Schema-owned, plural declarative contract for one artifact family or standard profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactDefinition {
    pub(crate) identity: ArtifactIdentity,
    capabilities: BTreeMap<ArtifactIdentity, ArtifactCapability>,
}

impl ArtifactDefinition {
    /// 🏗️ Starts an empty definition and claims its artifact identity.
    pub fn new(identity: ArtifactIdentity) -> Self {
        Self { identity, capabilities: BTreeMap::new() }
    }

    /// 🪪️ Returns the definition's hierarchical artifact identity.
    pub fn identity(&self) -> &ArtifactIdentity {
        &self.identity
    }

    /// 🧩️ Adds one capability. Its identity must live below this definition's identity.
    pub fn capability(mut self, capability: ArtifactCapability) -> Result<Self, ArtifactDefinitionError> {
        if !capability.identity.is_within(&self.identity) || capability.identity == self.identity {
            return Err(ArtifactDefinitionError::new("artifact-definition.capability-owner", format!("capability {} is not a descendant of artifact {}", capability.identity, self.identity)));
        }
        if self.capabilities.insert(capability.identity.clone(), capability).is_some() {
            return Err(ArtifactDefinitionError::new("artifact-definition.duplicate-capability", "capability identity is already declared"));
        }
        Ok(self)
    }

    /// 📚️ Returns capabilities in stable hierarchical order.
    /// 🚫️async: E1 pure accessor — `impl Iterator` return type admits no suspension point,
    /// and every sibling below chains a sync `.filter()` off it — see R9.
    pub fn capabilities(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities.values()
    }

    /// 🔐️ Requires one exact immutable capability before runtime registration.
    pub fn require_declared_capability(&self, kind: &ArtifactCapabilityKind, id: &ArtifactIdentity, claims: &[ArtifactIdentityClaim], descriptor: Option<&[u8]>) -> Result<&ArtifactCapability, ArtifactDefinitionError> {
        let capability = self.capabilities.get(id).ok_or_else(|| ArtifactDefinitionError::new("artifact-definition.runtime-capability", format!("runtime registration names undeclared capability {id}")))?;
        if capability.kind != *kind {
            return Err(ArtifactDefinitionError::new("artifact-definition.runtime-category", format!("runtime registration for {id} requires {}, declared {}", kind.as_str(), capability.kind.as_str())));
        }
        let mut claims = claims.to_vec();
        claims.sort();
        if capability.claims != claims {
            return Err(ArtifactDefinitionError::new("artifact-definition.runtime-claims", format!("runtime registration claims do not exactly match {id}")));
        }
        if let Some(descriptor) = descriptor {
            if capability.descriptor_bytes() != descriptor {
                return Err(ArtifactDefinitionError::new("artifact-definition.runtime-descriptor", format!("runtime descriptor does not exactly match {id}")));
            }
        }
        Ok(capability)
    }

    /// 🧩️ Returns capabilities belonging to one open-ended category in stable identity order.
    /// 🚫️async: E1 pure filter — `impl Iterator` return type admits no suspension point — see R9.
    pub fn capabilities_of<'a>(&'a self, kind: &'a ArtifactCapabilityKind) -> impl Iterator<Item = &'a ArtifactCapability> {
        self.capabilities().filter(move |capability| capability.kind() == kind)
    }

    /// 🏅️ Returns every standard leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn standards(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "standard")
    }

    /// 🪆️ Returns every profile leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn profiles(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "profile")
    }

    /// 🚪️ Returns every source-dialect leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn source_dialects(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "source-dialect")
    }

    /// 🎭️ Returns every representation leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn representations(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "representation")
    }

    /// 🗜️ Returns every codec leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn codecs(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "codec")
    }

    /// 🧬️ Returns every mutation leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn mutations(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "mutation")
    }

    /// 💡️ Returns every inference leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn inferences(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "inference")
    }

    /// 📦️ Returns every resource-policy leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn resources(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "resource")
    }

    /// 🗺️ Returns every localization leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn localizations(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "localization")
    }

    /// ✅️ Returns every conformance-suite leaf in stable identity order.
    /// 🚫️async: E1 pure filter — see `capabilities_of`'s own comment above.
    pub fn conformance_suites(&self) -> impl Iterator<Item = &ArtifactCapability> {
        self.capabilities().filter(|capability| capability.kind().as_str() == "conformance-suite")
    }

    /// 🛡️ Validates identity ownership and all claims before registration mutates a registry.
    pub fn validate(&self) -> Result<(), ArtifactDefinitionError> {
        let mut claims = BTreeMap::<ArtifactIdentityClaim, ArtifactIdentity>::new();
        for capability in self.capabilities() {
            if !capability.identity.is_within(&self.identity) || capability.identity == self.identity {
                return Err(ArtifactDefinitionError::new("artifact-definition.capability-owner", format!("capability {} is not a descendant of artifact {}", capability.identity, self.identity)));
            }
            if capability.descriptor.is_empty() {
                return Err(ArtifactDefinitionError::new("artifact-definition.descriptor", format!("capability {} has no immutable descriptor", capability.identity)));
            }
            for claim in capability.claims() {
                if let Some(previous) = claims.insert(claim.clone(), capability.identity.clone()) {
                    return Err(ArtifactDefinitionError::new("artifact-definition.duplicate-claim", format!("{}:{} is claimed by both {} and {}", claim.namespace.as_str(), claim.value, previous, capability.identity)));
                }
            }
        }
        Ok(())
    }

    fn identical_to(&self, other: &Self) -> bool {
        self.identity == other.identity && self.capabilities == other.capabilities
    }
}

/// 📮️ A validated optional representation MIME identity.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactMime(String);

impl ArtifactMime {
    /// 🏗️ Parses a lower-case ASCII `type/subtype` MIME identity.
    pub fn parse(value: impl Into<String>) -> Result<Self, ArtifactDefinitionError> {
        let value = value.into();
        let mut segments = value.split('/');
        let Some(kind) = segments.next() else {
            return Err(ArtifactDefinitionError::new("artifact-definition.mime", "MIME must contain a type and subtype"));
        };
        let Some(subtype) = segments.next() else {
            return Err(ArtifactDefinitionError::new("artifact-definition.mime", "MIME must contain a type and subtype"));
        };
        if segments.next().is_some() || kind.is_empty() || subtype.is_empty() || !value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-' | b'/'))
        {
            return Err(ArtifactDefinitionError::new("artifact-definition.mime", format!("MIME {value:?} is not lower-case ASCII type/subtype")));
        }
        Ok(Self(value))
    }

    /// 🧭️ Returns the exact MIME identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 📚️ Deterministic, conflict-rejecting registry for independently assembled definitions.
pub struct ArtifactDefinitionRegistry {
    definitions: BTreeMap<ArtifactIdentity, ArtifactDefinition>,
    claims: BTreeMap<ArtifactIdentityClaim, ArtifactIdentity>,
}

impl Default for ArtifactDefinitionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactDefinitionRegistry {
    /// 🏗️ Creates an empty artifact-definition registry.
    pub fn new() -> Self {
        Self { definitions: BTreeMap::new(), claims: BTreeMap::new() }
    }

    /// 🛡️ Registers exactly one new definition or rejects it without partial mutation.
    pub fn register(&mut self, definition: ArtifactDefinition) -> Result<(), ArtifactDefinitionError> {
        definition.validate()?;
        let definition_identity = definition.identity();
        if let Some(existing) = self.definitions.get(definition_identity) {
            if existing.identical_to(&definition) {
                return Ok(());
            }
            return Err(ArtifactDefinitionError::new("artifact-definition.conflicting-artifact", format!("artifact {} has a different descriptor or executable identity", definition_identity)));
        }
        let artifact_claim = ArtifactIdentityClaim::new(ArtifactIdentityNamespace::artifact(), definition_identity.as_str())?;
        if let Some(previous) = self.claims.get(&artifact_claim) {
            return Err(ArtifactDefinitionError::new("artifact-definition.duplicate-artifact", format!("artifact {} is already registered by {}", definition_identity, previous)));
        }
        let mut additions = vec![(artifact_claim, definition_identity.clone())];
        for capability in definition.capabilities() {
            for claim in capability.claims() {
                if let Some(previous) = self.claims.get(claim) {
                    return Err(ArtifactDefinitionError::new("artifact-definition.conflicting-claim", format!("{}:{} is already registered by {}", claim.namespace.as_str(), claim.value(), previous)));
                }
                if let Some((_, previous)) = additions.iter().find(|(candidate, _)| candidate == claim) {
                    return Err(ArtifactDefinitionError::new("artifact-definition.duplicate-claim", format!("{}:{} is declared by both {} and {}", claim.namespace.as_str(), claim.value(), previous, capability.identity())));
                }
                additions.push((claim.clone(), capability.identity().clone()));
            }
        }
        for (claim, owner) in additions {
            self.claims.insert(claim, owner);
        }
        self.definitions.insert(definition_identity.clone(), definition);
        Ok(())
    }

    /// 🔎️ Resolves a definition by its artifact identity.
    pub fn get(&self, identity: &ArtifactIdentity) -> Option<&ArtifactDefinition> {
        self.definitions.get(identity)
    }

    /// 🔢 Returns the number of registered artifact definitions.
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// 📭️ Whether no definitions are registered.
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// 🚶️ Walks definitions in deterministic hierarchical identity order.
    pub fn definitions(&self) -> impl Iterator<Item = &ArtifactDefinition> {
        self.definitions.values()
    }
}

/// 🔐️ Locates the one immutable typed capability whose complete claim set authorizes a runtime facet.
pub(crate) fn require_declared_capability_or_record(definition: &ArtifactDefinition, definition_error: &mut Option<ArtifactDefinitionError>, kind: &ArtifactCapabilityKind, claims: Result<Vec<ArtifactIdentityClaim>, ArtifactDefinitionError>) {
    if definition_error.is_some() {
        return;
    }
    let result = (|| -> Result<(), ArtifactDefinitionError> {
        let mut claims = claims?;
        claims.sort();
        let mut found = None;
        for capability in definition.capabilities_of(kind) {
            if capability.claims() == claims {
                found = Some(capability);
                break;
            }
        }
        let capability = match found {
            Some(capability) => capability,
            None => return Err(ArtifactDefinitionError::new("artifact-definition.runtime-capability", format!("no declared {} capability owns the runtime claims", kind.as_str()))),
        };
        definition.require_declared_capability(kind, capability.identity(), &claims, None).map(|_| ())
    })();
    if let Err(error) = result {
        *definition_error = Some(error);
    }
}
