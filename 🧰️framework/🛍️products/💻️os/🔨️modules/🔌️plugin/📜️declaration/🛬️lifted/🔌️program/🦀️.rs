use semio_framework::AppDefinition;
use semio_framework::kernel::ActorId;
use semio_framework_plugin_decl_surface::ArtifactCodecTableV1;

/// 🎭️ The definition travels WITH the bare fn pointer — same reason `SurfaceDeclaration.factory`
/// does (a monomorphized non-capturing `fn` item cannot close over `def`).
pub struct AppFactory<PA> {
    pub definition: AppDefinition,
    /// 🚫️async: E4 fn-pointer slot
    pub create: for<'a, 'b> fn(&AppDefinition, ActorId, semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> PA,
    pub document_schema: &'static str,
    pub codec: ArtifactCodecTableV1,
}
