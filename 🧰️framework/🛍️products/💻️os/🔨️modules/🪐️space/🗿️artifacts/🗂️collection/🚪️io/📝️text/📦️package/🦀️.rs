//! 📥️ Explicit JSON ingress for the collection package declaration.
use crate::{admit_collection_package_declaration, CollectionArtifactPackage, CollectionPackageDeclaration, CollectionPackageError};
/// 🧬️ Bundled physical package declaration owned by JSON IO.
pub const COLLECTION_PACKAGE_DECLARATION_JSON: &str = include_str!("../../../🧬️schema/📜️artifact-definition.json");
/// 📥️ Decodes and admits a physical package declaration.
pub fn decode_collection_package_json(source: &str) -> Result<CollectionArtifactPackage, CollectionPackageError> {
 let declaration: CollectionPackageDeclaration=semio_framework_pack_json::from_json_str(source,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| CollectionPackageError(error.to_string()))?;
 admit_collection_package_declaration(declaration)
}
/// 📦️ Reads and admits the bundled physical package declaration.
pub fn read_builtin_collection_package() -> Result<CollectionArtifactPackage, CollectionPackageError> { decode_collection_package_json(COLLECTION_PACKAGE_DECLARATION_JSON) }
