//! 📦️ Typed canonical space package identity and admission.

/// 🧬️ Neutral package declaration admitted by this artifact.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
pub struct SpacePackageDeclaration {
    pub definition_version: u8,
    pub id: String,
    pub artifact: String,
    pub directory: String,
    pub rust_package: String,
    pub nx_project: String,
    pub dependencies: Vec<String>,
}

/// 📦️ Validated package identity for the builtin space artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceArtifactPackage {
    pub id: String,
    pub artifact: String,
    pub directory: String,
    pub rust_package: String,
    pub nx_project: String,
    pub dependencies: Vec<String>,
}

/// 🚫️ Refused canonical package identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpacePackageError(pub String);
impl std::fmt::Display for SpacePackageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.0) }
}
impl std::error::Error for SpacePackageError {}

/// 🛬️ Admits a typed package declaration against its canonical identity.
pub fn admit_space_package_declaration(declaration: SpacePackageDeclaration) -> Result<SpaceArtifactPackage, SpacePackageError> {
    if declaration.definition_version != 1 || declaration.id != "os.space" || declaration.artifact != "space" || declaration.directory != "🪐️space" || declaration.rust_package != "semio-framework-artifact-space-space" || declaration.nx_project != "@semio-tech/framework-space-space-rs" || !declaration.dependencies.is_empty() { return Err(SpacePackageError("builtin space package identity does not match its canonical declaration".into())); }
    Ok(SpaceArtifactPackage { id: declaration.id, artifact: declaration.artifact, directory: declaration.directory, rust_package: declaration.rust_package, nx_project: declaration.nx_project, dependencies: declaration.dependencies })
}

/// 🏷️ Constructs this artifact's typed canonical identity.
pub fn package_descriptor() -> Result<SpaceArtifactPackage, SpacePackageError> {
    admit_space_package_declaration(SpacePackageDeclaration { definition_version: 1, id: "os.space".into(), artifact: "space".into(), directory: "🪐️space".into(), rust_package: "semio-framework-artifact-space-space".into(), nx_project: "@semio-tech/framework-space-space-rs".into(), dependencies: Vec::new() })
}
