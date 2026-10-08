//! 📦️ Typed canonical collection package identity and admission.

/// 🧬️ Neutral package declaration admitted by this artifact.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
pub struct CollectionPackageDeclaration {
    pub definition_version: u8,
    pub id: String,
    pub artifact: String,
    pub directory: String,
    pub rust_package: String,
    pub nx_project: String,
    pub dependencies: Vec<String>,
}

/// 📦️ Validated package identity for the builtin collection artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionArtifactPackage {
    pub id: String,
    pub artifact: String,
    pub directory: String,
    pub rust_package: String,
    pub nx_project: String,
    pub dependencies: Vec<String>,
}

/// 🚫️ Refused canonical package identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionPackageError(pub String);
impl std::fmt::Display for CollectionPackageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.0) }
}
impl std::error::Error for CollectionPackageError {}

/// 🛬️ Admits a typed package declaration against its canonical identity.
pub fn admit_collection_package_declaration(declaration: CollectionPackageDeclaration) -> Result<CollectionArtifactPackage, CollectionPackageError> {
    if declaration.definition_version != 1 || declaration.id != "os.collection" || declaration.artifact != "collection" || declaration.directory != "🗂️collection" || declaration.rust_package != "semio-framework-artifact-space-collection" || declaration.nx_project != "@semio-tech/framework-space-collection-rs" || !declaration.dependencies.is_empty() { return Err(CollectionPackageError("builtin collection package identity does not match its canonical declaration".into())); }
    Ok(CollectionArtifactPackage { id: declaration.id, artifact: declaration.artifact, directory: declaration.directory, rust_package: declaration.rust_package, nx_project: declaration.nx_project, dependencies: declaration.dependencies })
}

/// 🏷️ Constructs this artifact's typed canonical identity.
pub fn package_descriptor() -> Result<CollectionArtifactPackage, CollectionPackageError> {
    admit_collection_package_declaration(CollectionPackageDeclaration { definition_version: 1, id: "os.collection".into(), artifact: "collection".into(), directory: "🗂️collection".into(), rust_package: "semio-framework-artifact-space-collection".into(), nx_project: "@semio-tech/framework-space-collection-rs".into(), dependencies: Vec::new() })
}
