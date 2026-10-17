//! 🖱️ Context menu: the localized row builder, the surface selection and the object, vortex, volume and reference menu items.

use super::*;

/// 🏷️ Admits dynamic puzzle labels into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d label admission failed"))
}

/// 🖱️ Bespoke row builder — every row here carries a localized (`Puzzle3dLabels`) label/icon that the
/// declared `ActionDefinition` (English-only) cannot resolve, so each row is emitted via `Menu::item`
/// rather than `Menu::action`. Grouping/ordering/the pre-destructive separator are still handled by
/// `Menu::group` + the `organize_context_menu` funnel in `context_menu`.
fn puzzle3d_context_menu_row(id: &str, label: impl Into<String>, icon: &str, action: &str, args: Option<Value>, destructive: bool) -> semio_framework_plugin::ContextMenuItemSpec {
    semio_framework_plugin::ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)),
        destructive: destructive.then_some(true),
        ..Default::default()
    }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-granularity ids
/// `ContextMenuRequest.surface.selection` carries — the CLIENT-supplied, always-available substitute
/// for `runtime.selection` at context-menu time (unlike `render`, `context_menu` never had a live
/// config selection to read even before this ticket; `ContextMenuSurfaceTarget.selection` is the
/// framework's own sanctioned channel for it).
#[derive(Default)]
struct Puzzle3dContextSelection {
    object_ids: Vec<String>,
    vortex_ids: Vec<String>,
    attraction_ids: Vec<String>,
    target_volume_ids: Vec<String>,
    reference_ids: Vec<String>,
    /// 🎯️ The pointer's hit alone is the subject, so no selection read may widen it again.
    hit_subject: bool,
}

impl Puzzle3dContextSelection {
    fn bucket(&mut self, domain: &str) -> Option<&mut Vec<String>> {
        match domain {
            "node" | PUZZLE3D_GRANULARITY_OBJECT => Some(&mut self.object_ids),
            PUZZLE3D_GRANULARITY_VORTEX => Some(&mut self.vortex_ids),
            PUZZLE3D_GRANULARITY_ATTRACTION => Some(&mut self.attraction_ids),
            PUZZLE3D_GRANULARITY_TARGET_VOLUME => Some(&mut self.target_volume_ids),
            PUZZLE3D_GRANULARITY_REFERENCE => Some(&mut self.reference_ids),
            _ => None,
        }
    }

    /// 🎯️ `surface.hits` is the entity the pointer is actually over, `surface.selection` the entities the
    /// document already holds selected. A hit OUTSIDE the selection is the menu's whole subject, so a
    /// right-click on an unselected object opens that object's menu instead of the empty menu that made the
    /// shell fallback take over the viewport (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 8), and a
    /// right-click on a vortex of the selected object opens the vortex's menu, not the object's. A hit
    /// inside the current selection changes nothing — the whole selection stays the menu's subject, so
    /// "Delete (3 objects)" never silently narrows to the one row under the cursor.
    fn from_surface(surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Self {
        let mut out = Self::default();
        let Some(surface) = surface else {
            return out;
        };
        for group in &surface.selection {
            let ids = group.ids.clone();
            if let Some(bucket) = out.bucket(group.domain.as_str()) {
                bucket.extend(ids);
            }
        }
        // 🎯️ Hosts list every target under the pointer, most specific first, and only that one is the subject.
        let mut hits = Self::default();
        if let Some(hit) = surface.hits.iter().find(|hit| hits.bucket(hit.domain.as_str()).is_some()) {
            hits.bucket(hit.domain.as_str()).into_iter().for_each(|bucket| bucket.push(hit.id.clone()));
        }
        let hit_selected = |selected: &[String], hit: &[String]| hit.iter().all(|id| selected.contains(id));
        let inside = hit_selected(&out.object_ids, &hits.object_ids)
            && hit_selected(&out.vortex_ids, &hits.vortex_ids)
            && hit_selected(&out.attraction_ids, &hits.attraction_ids)
            && hit_selected(&out.target_volume_ids, &hits.target_volume_ids)
            && hit_selected(&out.reference_ids, &hits.reference_ids);
        if surface.hits.is_empty() || inside {
            return out;
        }
        hits.hit_subject = true;
        hits
    }

    /// 🕹️ Fills in the granularities the CLIENT surface never sends. `World3dHost` only puts its
    /// painted object ids (domain `"object"`) and component ids (`"feature"`) into
    /// `ContextMenuSurfaceTarget.selection`, so a selected vortex / attraction / target volume /
    /// reference reaches a menu only through the framework-owned domain read. Per-granularity
    /// additive: whatever the surface DID supply keeps priority (a right-click on an unselected
    /// entity still targets what was clicked).
    fn fill_from_interaction(&mut self, interaction: &Puzzle3dInteractionSnapshot) {
        if self.hit_subject {
            return;
        }
        for (bucket, ids) in [
            (&mut self.object_ids, interaction.selected_object_ids()),
            (&mut self.vortex_ids, interaction.selected_vortex_ids()),
            (&mut self.attraction_ids, interaction.selected_attraction_ids()),
            (&mut self.target_volume_ids, interaction.selected_target_volume_ids()),
            (&mut self.reference_ids, interaction.selected_reference_ids()),
        ] {
            if bucket.is_empty() {
                bucket.extend(ids.iter().cloned());
            }
        }
    }
}

fn puzzle3d_context_menu_items(envelope: &Puzzle3dScene, selection: &Puzzle3dContextSelection, labels: &Puzzle3dLabels, view_state: &semio_framework_plugin::ViewModel, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::Menu;
    if !selection.object_ids.is_empty() {
        let all_hidden = envelope.scene_snapshot.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.hidden);
        let all_locked = envelope.scene_snapshot.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.locked);
        let count = selection.object_ids.len();
        let phrase = if count == 1 { format!("1 {}", labels.object.as_str()) } else { format!("{count} {}", labels.objects.as_str()) };
        return {
            Menu::of(registry, view_state)
                .item(puzzle3d_context_menu_row("duplicate", labels.duplicate, "copy", "duplicateSelection", None, false))
                .item(puzzle3d_context_menu_row("select-same-kind", labels.select_same_kind, "layers", "selectSameKindSelection", None, false))
                .item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false))
                .group("hand", |m| {
                    m.item(puzzle3d_context_menu_row("hide-show", if all_hidden { labels.show } else { labels.hide }, if all_hidden { "eye" } else { "eye-off" }, "setSelectionFlag", Some(json!({ "flag": "hidden", "value": !all_hidden })), false))
                        .item(puzzle3d_context_menu_row(
                            "lock-unlock",
                            if all_locked { labels.unlock } else { labels.lock },
                            if all_locked { "lock-open" } else { "lock" },
                            "setSelectionFlag",
                            Some(json!({ "flag": "locked", "value": !all_locked })),
                            false,
                        ))
                })
                .item(puzzle3d_context_menu_row("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
                .build()
        };
    }
    if !selection.vortex_ids.is_empty() {
        let mut menu = Menu::of(registry, view_state);
        if let [only] = selection.vortex_ids.as_slice() {
            menu = menu.item(puzzle3d_context_menu_row("suggest", labels.suggest_objects, "sparkles", "openVortexSuggestions", Some(json!({ "fullId": only.as_str() })), false));
        }
        return menu.item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false)).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true)).build();
    }
    if let Some(id) = selection.attraction_ids.first() {
        return Menu::of(registry, view_state).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteAttraction", Some(json!({ "id": id.as_str() })), true)).build();
    }
    if let Some(id) = selection.target_volume_ids.first() {
        let target_volume = envelope.scene_snapshot.target_volumes.iter().find(|volume| &volume.id == id);
        let hidden = target_volume.is_some_and(|volume| volume.hidden);
        let locked = target_volume.is_some_and(|volume| volume.locked);
        return {
            Menu::of(registry, view_state)
                .group("targets", |m| {
                    m.item(puzzle3d_context_menu_row(
                        "hide-show",
                        if hidden { labels.show } else { labels.hide },
                        if hidden { "eye" } else { "eye-off" },
                        "setTargetVolumeFlag",
                        Some(json!({ "id": id.as_str(), "flag": "hidden", "value": !hidden })),
                        false,
                    ))
                    .item(puzzle3d_context_menu_row(
                        "lock-unlock",
                        if locked { labels.unlock } else { labels.lock },
                        if locked { "lock-open" } else { "lock" },
                        "setTargetVolumeFlag",
                        Some(json!({ "id": id.as_str(), "flag": "locked", "value": !locked })),
                        false,
                    ))
                })
                .item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteTargetVolume", Some(json!({ "id": id.as_str() })), true))
                .build()
        };
    }
    if !selection.reference_ids.is_empty() {
        return {
            Menu::of(registry, view_state).item(puzzle3d_context_menu_row("zoom", labels.zoom_to_selection, "crosshair", "focusSelection", None, false)).item(puzzle3d_context_menu_row("delete", labels.delete, "trash", "deleteSelection", None, true)).build()
        };
    }
    Vec::new()
}
