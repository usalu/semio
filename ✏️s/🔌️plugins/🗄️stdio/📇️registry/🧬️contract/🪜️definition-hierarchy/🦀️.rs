//! 🪜️ Stdio-owned hierarchy constructors and capability grammar validation.

use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactDefinitionError, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization, ArtifactMime};

/// 🪜️ Canonical Stdio identity grammar, authored by the specific contract owner.
pub trait StdioArtifactIdentity: Sized {
    fn stdio_artifact(artifact: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn standard(&self, revision: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn profile(&self, profile: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn source_dialect(&self, dialect: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn representation(&self, representation: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn codec(&self, codec: impl AsRef<str>, version: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn inference(&self, semantic_slug: impl AsRef<str>, version: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn mutation(&self, semantic_command: impl AsRef<str>, version: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
}

impl StdioArtifactIdentity for ArtifactIdentity {
    /// 🧭️ Creates the exact canonical root identity for one stdio artifact.
    fn stdio_artifact(artifact: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        let artifact = artifact.as_ref();
        stdio_validate_segment(artifact, "artifact")?;
        Self::parse(format!("s.stdio.{artifact}"))
    }

    /// 🏅️ Creates a canonical standard identity below this stdio artifact root.
    fn standard(&self, revision: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        stdio_artifact_child(self, "standard", revision.as_ref())
    }

    /// 🪆️ Creates a canonical profile identity below a stdio standard.
    fn profile(&self, profile: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        stdio_standard_child(self, "profile", profile.as_ref())
    }

    /// 🚪️ Creates a canonical source-dialect identity below a stdio standard.
    fn source_dialect(&self, dialect: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        stdio_standard_child(self, "dialect", dialect.as_ref())
    }

    /// 🎭️ Creates a canonical representation identity below a stdio standard.
    fn representation(&self, representation: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        stdio_standard_child(self, "representation", representation.as_ref())
    }

    /// 🗜️ Creates a canonical codec identity below a stdio standard.
    fn codec(&self, codec: impl AsRef<str>, version: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        let codec = codec.as_ref();
        let version = version.as_ref();
        stdio_standard_child(self, "codec", codec)?.child(stdio_version(version)?)
    }

    /// 💡️ Creates a canonical inference identity below this stdio artifact root.
    fn inference(&self, semantic_slug: impl AsRef<str>, version: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        let semantic_slug = semantic_slug.as_ref();
        let version = version.as_ref();
        stdio_artifact_child(self, "inference", semantic_slug)?.child(stdio_version(version)?)
    }

    /// 🧬️ Creates a canonical mutation identity below this stdio artifact root.
    fn mutation(&self, semantic_command: impl AsRef<str>, version: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        let semantic_command = semantic_command.as_ref();
        let version = version.as_ref();
        stdio_artifact_child(self, "mutation", semantic_command)?.child(stdio_version(version)?)
    }
}

fn stdio_artifact_child(identity: &ArtifactIdentity, namespace: &str, value: &str) -> Result<ArtifactIdentity, ArtifactDefinitionError> {
    if !is_stdio_artifact(identity) {
        return Err(ArtifactDefinitionError::new("artifact-definition.identity-hierarchy", format!("{} is not a stdio artifact identity", identity)));
    }
    stdio_validate_segment(value, namespace)?;
    identity.child(namespace)?.child(value)
}

fn stdio_standard_child(identity: &ArtifactIdentity, namespace: &str, value: &str) -> Result<ArtifactIdentity, ArtifactDefinitionError> {
    if !is_stdio_standard(identity) {
        return Err(ArtifactDefinitionError::new("artifact-definition.identity-hierarchy", format!("{} is not a stdio standard identity", identity)));
    }
    stdio_validate_segment(value, namespace)?;
    identity.child(namespace)?.child(value)
}

fn is_stdio_artifact(identity: &ArtifactIdentity) -> bool {
    identity.segments().len() == 3 && identity.segments()[0] == "s" && identity.segments()[1] == "stdio"
}

fn is_stdio_standard(identity: &ArtifactIdentity) -> bool {
    identity.segments().len() == 5 && identity.segments()[0] == "s" && identity.segments()[1] == "stdio" && identity.segments()[3] == "standard"
}

fn stdio_validate_segment(value: &str, label: &str) -> Result<(), ArtifactDefinitionError> {
    ArtifactIdentity::parse(value.to_owned()).and_then(|identity| if identity.segments().len() == 1 { Ok(()) } else { Err(ArtifactDefinitionError::new("artifact-definition.identity", format!("{label} segment {value:?} is not canonical"))) })
}
fn stdio_version(value: &str) -> Result<&str, ArtifactDefinitionError> {
    if value.len() < 2 || !value.starts_with('v') || !value[1..].bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ArtifactDefinitionError::new("artifact-definition.version", format!("version {value:?} must use the canonical vN form")));
    }
    Ok(value)
}

/// 🪜️ Stdio-specific definition construction and validation before runtime publication.
pub trait StdioArtifactDefinition: Sized {
    fn stdio(artifact: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError>;
    fn standard(self, revision: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn profile(self, revision: impl AsRef<str>, profile: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn source_dialect(self, revision: impl AsRef<str>, dialect: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn representation(self, revision: impl AsRef<str>, representation: impl AsRef<str>, mime: Option<ArtifactMime>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn codec(self, revision: impl AsRef<str>, codec: impl AsRef<str>, version: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn mutation(self, command: impl AsRef<str>, version: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn inference(self, semantic_slug: impl AsRef<str>, version: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn resource(self, name: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn localization(self, locale: ArtifactLocale, text: impl Into<String>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn conformance_suite(self, name: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError>;
    fn validate_stdio(&self) -> Result<(), ArtifactDefinitionError>;
}

impl StdioArtifactDefinition for ArtifactDefinition {
    /// 🧾️ Starts the one authoritative definition for a canonical stdio artifact.
    fn stdio(artifact: impl AsRef<str>) -> Result<Self, ArtifactDefinitionError> {
        Ok(Self::new(ArtifactIdentity::stdio_artifact(artifact)?))
    }

    /// 🏅️ Declares a standard leaf in this artifact's plural definition.
    fn standard(self, revision: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().standard(revision)?;
        stdio_capability(self, identity, ArtifactCapabilityKind::standard(), descriptor)
    }

    /// 🪆️ Declares a profile leaf in this artifact's plural definition.
    fn profile(self, revision: impl AsRef<str>, profile: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().standard(revision)?.profile(profile)?;
        stdio_capability(self, identity, ArtifactCapabilityKind::profile(), descriptor)
    }

    /// 🚪️ Declares a source-dialect leaf in this artifact's plural definition.
    fn source_dialect(self, revision: impl AsRef<str>, dialect: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().standard(revision)?.source_dialect(dialect)?;
        stdio_capability(self, identity, ArtifactCapabilityKind::source_dialect(), descriptor)
    }

    /// 🎭️ Declares a representation leaf, optionally claiming its MIME identity.
    fn representation(self, revision: impl AsRef<str>, representation: impl AsRef<str>, mime: Option<ArtifactMime>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().standard(revision)?.representation(representation)?;
        let capability = match mime {
            Some(mime) => {
                let claim = ArtifactIdentityClaim::new(ArtifactIdentityNamespace::mime(), mime.as_str())?;
                ArtifactCapability::new(identity, ArtifactCapabilityKind::representation()).claim(claim)?
            }
            None => ArtifactCapability::new(identity, ArtifactCapabilityKind::representation()),
        }
        .descriptor(descriptor)?;
        self.capability(capability)
    }

    /// 🗜️ Declares a codec leaf in this artifact's plural definition.
    fn codec(self, revision: impl AsRef<str>, codec: impl AsRef<str>, version: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().standard(revision)?.codec(codec, version)?;
        stdio_capability(self, identity, ArtifactCapabilityKind::codec(), descriptor)
    }

    /// 🧬️ Declares a semantic mutation leaf in this artifact's plural definition.
    fn mutation(self, command: impl AsRef<str>, version: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().mutation(command, version)?;
        stdio_capability(self, identity, ArtifactCapabilityKind::mutation(), descriptor)
    }

    /// 💡️ Declares an atomic inference leaf in this artifact's plural definition.
    fn inference(self, semantic_slug: impl AsRef<str>, version: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().inference(semantic_slug, version)?;
        stdio_capability(self, identity, ArtifactCapabilityKind::inference(), descriptor)
    }

    /// 📦️ Declares an open resource-policy leaf in this artifact's plural definition.
    fn resource(self, name: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().child("resource")?.child(name.as_ref())?;
        stdio_capability(self, identity, ArtifactCapabilityKind::resource(), descriptor)
    }

    /// 🗺️ Declares an explicit locale leaf in this artifact's plural definition.
    fn localization(self, locale: ArtifactLocale, text: impl Into<String>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let capability = ArtifactCapability::new(self.identity().child("localization")?.child(locale.as_str())?, ArtifactCapabilityKind::localization()).localization(ArtifactLocalization::new(locale, text)?)?.descriptor(descriptor)?;
        self.capability(capability)
    }

    /// ✅️ Declares a conformance-suite leaf in this artifact's plural definition.
    fn conformance_suite(self, name: impl AsRef<str>, descriptor: impl Into<Vec<u8>>) -> Result<Self, ArtifactDefinitionError> {
        let identity = self.identity().child("conformance-suite")?.child(name.as_ref())?;
        stdio_capability(self, identity, ArtifactCapabilityKind::conformance_suite(), descriptor)
    }

    fn validate_stdio(&self) -> Result<(), ArtifactDefinitionError> {
        if !is_stdio_artifact(self.identity()) {
            return Err(ArtifactDefinitionError::new("artifact-definition.identity-hierarchy", "Stdio definition requires its canonical artifact root"));
        }
        self.validate()?;
        for capability in self.capabilities() {
            validate_stdio_capability(self.identity(), capability)?;
        }
        Ok(())
    }
}

fn validate_stdio_capability(identity: &ArtifactIdentity, capability: &ArtifactCapability) -> Result<(), ArtifactDefinitionError> {
    if !is_stdio_artifact(identity) {
        return Ok(());
    }
    let segments = capability.identity().segments();
    let category = capability.kind().as_str();
    let valid = match category {
        "standard" => segments.len() == 5 && segments[3] == "standard",
        "profile" => segments.len() == 7 && segments[3] == "standard" && segments[5] == "profile",
        "source-dialect" => segments.len() == 7 && segments[3] == "standard" && segments[5] == "dialect",
        "representation" => segments.len() == 7 && segments[3] == "standard" && segments[5] == "representation",
        "codec" => segments.len() == 8 && segments[3] == "standard" && segments[5] == "codec" && stdio_version(&segments[7]).is_ok(),
        "mutation" => segments.len() == 6 && segments[3] == "mutation" && stdio_version(&segments[5]).is_ok(),
        "inference" => segments.len() == 6 && segments[3] == "inference" && stdio_version(&segments[5]).is_ok(),
        "schema" => segments.len() >= 5 && segments[3] == "schema",
        "extension" => segments.len() >= 5 && segments[3] == "extension",
        "grammar" => segments.len() >= 5 && segments[3] == "grammar",
        "composer" => segments.len() >= 5 && segments[3] == "composer",
        "subset-validator" => segments.len() >= 5 && segments[3] == "subset-validator",
        "resource" => segments.len() >= 5 && segments[3] == "resource",
        "localization" => segments.len() == 5 && segments[3] == "localization",
        "conformance-suite" => segments.len() >= 5 && segments[3] == "conformance-suite",
        _ => true,
    };
    if valid {
        Ok(())
    } else {
        Err(ArtifactDefinitionError::new("artifact-definition.category-identity", format!("{} does not use the canonical {} identity grammar", capability.identity(), category)))
    }
}

fn stdio_capability(definition: ArtifactDefinition, identity: ArtifactIdentity, kind: ArtifactCapabilityKind, descriptor: impl Into<Vec<u8>>) -> Result<ArtifactDefinition, ArtifactDefinitionError> {
    definition.capability(ArtifactCapability::new(identity, kind).descriptor(descriptor)?)
}

#[cfg(test)]
#[path = "../🧪️tests/🪜️definition-hierarchy/🦀️.rs"]
mod tests;
