//! 🧊️ `add-object-kind` command.

use crate::editor::puzzle3d::next_object_id;
use crate::editor::puzzle3d::puzzle3d_vortices_from_kind_template;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::Puzzle3dObject;
use crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT;
use dsl::os_pack::json::Value;

/// 🥽️ One catalog row's mesh identity — `representations[].url` FIRST, `meshUrl` only as the legacy
/// fallback. Catalogued kinds authored as compose `Representation` rows (both shipped examples, and
/// every row `puzzle3d_normalize_object_kind_row` rewrites on import) carry no `meshUrl` at all, so a
/// `meshUrl`-only read placed the object with no mesh identity of its own and the fill/brush lanes
/// rejected its candidates `mesh-unavailable`. Same precedence as `Puzzle3dKindMeshIndex::of` and the
/// catalogue panel's drag payload.
fn catalog_entry_mesh_url(entry: &dsl::DslValue) -> Option<String> {
    entry
        .get("representations")
        .and_then(dsl::DslValue::as_array)
        .into_iter()
        .flatten()
        .filter_map(|representation| representation.get("url").and_then(dsl::DslValue::as_str))
        .map(str::trim)
        .find(|url| !url.is_empty())
        .or_else(|| entry.get("meshUrl").and_then(dsl::DslValue::as_str).map(str::trim).filter(|url| !url.is_empty()))
        .map(str::to_string)
}

/// 🌱️ Materializes one catalogued kind and RE-SELECTS it through `Emit.interaction_writes`, exactly as
/// `duplicate_selection` and `add_brush_object` re-select what they create: a gesture that adds an object
/// the user cannot then inspect, transform or delete without hunting for it is half a gesture, and the
/// browser battery reads it as `catalogue-add-selects-new-object added=1 selected=[]`.
pub fn add_object_kind(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let object_kind = args.and_then(|value| value.get("objectKind")).and_then(|value| value.as_str()).unwrap_or("Object");
    let id = next_object_id();
    let catalog_entry = ctx.scene.fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")?.as_array()?.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(object_kind)).cloned());
    let mesh_url = catalog_entry.as_ref().and_then(catalog_entry_mesh_url);
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
    });
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, [id]);
}
