//! 🖼️ Shared IFC2x3 surface projection and explicit representation diagnostics.
use super::{geometry,Ifc2x3Snapshot};
use semio_framework_plugin::{world3d_camera_json, world3d_selection_json_with_granularity, BuiltNode, MeshWindowKit, SurfaceKind, WindowKit};

pub fn render_with(document: &Ifc2x3Snapshot, limits: geometry::GeometryLimits, progress: &mut dyn FnMut(geometry::GeometryProgress) -> bool) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let scene = geometry::project_with(document, limits, progress);
    render_scene(scene,None)
}

fn render_scene(mut scene: geometry::GeometryScene,selection:Option<(&semio_framework_plugin::app::InteractionView<'_>,&str,&str)>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    if let Some(issue) = scene.payload_issue() { scene.issues.push(issue); scene.status = geometry::GeometryStatus::Limited; }
    if scene.status != geometry::GeometryStatus::Complete { scene.meshes.clear(); }
    let (position, target) = scene.camera();
    let mut world = semio_framework_plugin::world3d_scene(world3d_camera_json(position, target, 45.), scene.meshes_json(), scene.instances_json(), world3d_selection_json_with_granularity("pick",selection.map_or(&[][..],|(interaction,domain,_)|interaction.selection(domain).ids.as_slice()),None,selection.map(|(_,_,granularity)|granularity)), &semio_framework_plugin::WorldSunConfig::default());
    let issues: Vec<semio_framework_pack_json::Value> = scene.issues.iter().map(|issue| semio_framework_pack_json::json!({ "entity": issue.entity.to_string(), "reason": issue.reason.clone() })).collect();
    let status = match scene.status { geometry::GeometryStatus::Complete => "complete", geometry::GeometryStatus::Cancelled => "cancelled", geometry::GeometryStatus::Limited => "limited" };
    world.status_json = Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({
        "status": status, "work": scene.work, "issues": semio_framework_pack_json::array(issues),
        "label": { "en": "IFC Representation Geometry", "de": "IFC-Repräsentationsgeometrie" }
    })));
    semio_framework_plugin::scene_surface(MeshWindowKit::KIND_ID, SurfaceKind::World3d, &world)
}

pub fn render(document: &Ifc2x3Snapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    render_with(document, geometry::GeometryLimits::default(), &mut |_| true)
}

pub fn render_in_context(document: &Ifc2x3Snapshot, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    render_context(document,cx,None)
}

pub fn render_selected_in_context(document:&Ifc2x3Snapshot,cx:&mut semio_framework_job::StepContext<'_>,interaction:&semio_framework_plugin::app::InteractionView<'_>,domain:&str,granularity:&str)->semio_framework_plugin::UiAssemblyResult<BuiltNode>{render_context(document,cx,Some((interaction,domain,granularity)))}

fn render_context(document:&Ifc2x3Snapshot,cx:&mut semio_framework_job::StepContext<'_>,selection:Option<(&semio_framework_plugin::app::InteractionView<'_>,&str,&str)>)->semio_framework_plugin::UiAssemblyResult<BuiltNode>{
    let _ = cx.set_stage("ifc-geometry");
    let limits = geometry::GeometryLimits { work: geometry::GeometryLimits::default().work.min(cx.fuel_remaining() as usize), ..geometry::GeometryLimits::default() };
    let mut scene = geometry::project_controlled(document, limits, &mut |_| {
        if cx.is_cancelled() { return geometry::GeometryControl::Cancel; }
        if cx.should_yield() { return geometry::GeometryControl::Limit; }
        cx.consume_fuel(1);
        geometry::GeometryControl::Continue
    });
    if cx.is_cancelled() { scene.status = geometry::GeometryStatus::Cancelled; }
    else if cx.should_yield() { scene.status = geometry::GeometryStatus::Limited; }
    render_scene(scene,selection)
}
