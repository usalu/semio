//! 🧊️ The mesh window kit.
use semio_framework::{ActionDefinition, ActionKind, WindowKindDefinition};
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_ui_locale::LocalizedLabel;
use crate::window_kits::WindowKit;
use crate::window_surface::UiAssemblyResult;
use crate::window_surface::scene_surface;
use crate::window_surface::window_kind_definition;



/// 🧊️ Reuses the existing `world3d_host::World3dScene` wire shape (camera/meshes/instances/
/// selection JSON blobs) rather than a bespoke mesh payload — see `#region world3d_host`.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshView {
    pub camera_json: String,
    pub meshes_json: String,
    pub instances_json: String,
    pub selection_json: String,
}

pub struct MeshWindowKit;

impl WindowKit for MeshWindowKit {
    type ViewModel = MeshView;
    const KIND_ID: &'static str = "framework.window.mesh";

    fn window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Mesh", "Netz", SurfaceKind::World3d, "box", Vec::new())
    }

    fn editable_window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Mesh", "Netz", SurfaceKind::World3d, "box", vec![ActionDefinition::bounded_catalog("set-vertex", LocalizedLabel::native("Set Vertex", "Vertex setzen"), ActionKind::Mutation)])
    }

    fn render(view: &MeshView) -> UiAssemblyResult<BuiltNode> {
        let sun_config = crate::world3d_host::WorldSunConfig::default();
        let scene = crate::world3d_host::world3d_scene(view.camera_json.clone(), view.meshes_json.clone(), view.instances_json.clone(), view.selection_json.clone(), &sun_config);
        scene_surface(Self::KIND_ID, SurfaceKind::World3d, &scene)
    }
}
