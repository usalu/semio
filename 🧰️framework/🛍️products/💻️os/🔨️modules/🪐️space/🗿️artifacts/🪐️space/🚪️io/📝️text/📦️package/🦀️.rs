//! 📥️ Explicit JSON ingress for the space package declaration.
use crate::{admit_space_package_declaration, SpaceArtifactPackage, SpacePackageDeclaration, SpacePackageError};
/// 🧬️ Bundled physical package declaration owned by JSON IO.
pub const SPACE_PACKAGE_DECLARATION_JSON: &str = include_str!("../../../🧬️schema/📜️artifact-definition.json");
/// 📥️ Decodes and admits a physical package declaration.
pub fn decode_space_package_json(source: &str) -> Result<SpaceArtifactPackage, SpacePackageError> {
 let declaration: SpacePackageDeclaration=semio_framework_pack_json::from_json_str(source,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| SpacePackageError(error.to_string()))?;
 admit_space_package_declaration(declaration)
}
/// 📦️ Reads and admits the bundled physical package declaration.
pub fn read_builtin_space_package() -> Result<SpaceArtifactPackage, SpacePackageError> { decode_space_package_json(SPACE_PACKAGE_DECLARATION_JSON) }
