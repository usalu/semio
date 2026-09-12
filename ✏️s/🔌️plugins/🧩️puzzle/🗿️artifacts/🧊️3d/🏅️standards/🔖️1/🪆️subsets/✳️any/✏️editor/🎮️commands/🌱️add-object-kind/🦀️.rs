//! 🧊️ `add-object-kind` command.

use crate::editor::puzzle3d::next_object_id;
use crate::editor::puzzle3d::puzzle3d_vortices_from_kind_template;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::Puzzle3dObject;
use crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT;
use dsl::os_pack::json::Value;

/// 🌱️ Materializes one catalogued kind and RE-SELECTS it through `Emit.interaction_writes`, exactly as
/// `duplicate_selection` and `add_brush_object` re-select what they create: a gesture that adds an object
/// the user cannot then inspect, transform or delete without hunting for it is half a gesture, and the
/// browser battery reads it as `catalogue-add-selects-new-object added=1 selected=[]`.
pub fn add_object_kind(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let object_kind = args.and_then(|value| value.get("objectKind")).and_then(|value| value.as_str()).unwrap_or("Object");
    let id = next_object_id();
    let catalog_entry = ctx.scene.fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")?.as_array()?.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(object_kind)).cloned());
    let mesh_url = catalog_entry.as_ref().and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string));
    let vortices = catalog_entry.as_ref().map(puzzle3d_vortices_from_kind_template).unwrap_or_default();
    let origin = args
        .and_then(|value| value.get("origin"))
        .and_then(|value| value.as_array())
        .map_or([0.0, 0.0, 0.0], |values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)]);
    ctx.scene.fixture.objects.push(Puzzle3dObject {
        id: id.clone(),
        label: Some(object_kind.into()),
        object_kind: Some(object_kind.into()),
        origin,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url,
        vortices,
        hidden: false,
        locked: false,
        reveal_index: None,
    });
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, [id]);
}
