"""🪟️ Opening kinds: fenestration, glazing, shading surfaces, shading devices and surface connections."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-fenestration-adds-a-south-window": "shared://🧬️mutations/🪟️fenestration/🪟️create/✅️adds",
    "create-fenestration-refuses-a-missing-host": "shared://🧬️mutations/🪟️fenestration/🪟️create/⛔️refuses",
    "delete-fenestration-removes-a-window": "shared://🧬️mutations/🪟️fenestration/🚪️delete/✅️removes",
    "delete-fenestration-refuses-a-missing-one": "shared://🧬️mutations/🪟️fenestration/🚪️delete/⛔️refuses",
    "rename-fenestration-renames-a-window": "shared://🧬️mutations/🪟️fenestration/🏁️rename/✅️renames",
    "rename-fenestration-refuses-a-blank-name": "shared://🧬️mutations/🪟️fenestration/🏁️rename/⛔️refuses",
    "change-fenestration-surface-rehosts-a-window": "shared://🧬️mutations/🪟️fenestration/🧲️change-surface/✅️rehosts",
    "change-fenestration-surface-refuses-a-missing-host": "shared://🧬️mutations/🪟️fenestration/🧲️change-surface/⛔️refuses",
    "change-fenestration-u-value-swaps-the-glazing": "shared://🧬️mutations/🔍️glazing/🌐️change-u-value/✅️swaps",
    "change-fenestration-u-value-refuses-zero-u": "shared://🧬️mutations/🔍️glazing/🌐️change-u-value/⛔️refuses",
    "change-fenestration-shgc-dims-the-solar-gain": "shared://🧬️mutations/🔍️glazing/🌇️change-shgc/✅️dims",
    "change-fenestration-shgc-refuses-shgc-above-one": "shared://🧬️mutations/🔍️glazing/🌇️change-shgc/⛔️refuses",
    "change-fenestration-vlt-dims-the-daylight": "shared://🧬️mutations/🔍️glazing/🌈️change-vlt/✅️dims",
    "change-fenestration-vlt-refuses-negative-vlt": "shared://🧬️mutations/🔍️glazing/🌈️change-vlt/⛔️refuses",
    "change-fenestration-area-doubles-the-glazing": "shared://🧬️mutations/🪟️fenestration/🟥️change-area/✅️doubles",
    "change-fenestration-area-refuses-zero-area": "shared://🧬️mutations/🪟️fenestration/🟥️change-area/⛔️refuses",
    "change-fenestration-frame-conductance-adds-a-frame": "shared://🧬️mutations/🔍️glazing/🖼️change-frame-conductance/✅️adds-a",
    "change-fenestration-frame-conductance-refuses-negative-frame": "shared://🧬️mutations/🔍️glazing/🖼️change-frame-conductance/⛔️refuses",
    "change-fenestration-divider-conductance-adds-dividers": "shared://🧬️mutations/🔍️glazing/🧷️change-divider-conductance/✅️adds",
    "change-fenestration-divider-conductance-refuses-a-negative": "shared://🧬️mutations/🔍️glazing/🧷️change-divider-conductance/⛔️refuses",
    "create-shading-surface-adds-an-awning": "shared://🧬️mutations/🌳️shading/🌳️create/✅️adds",
    "create-shading-surface-refuses-a-line": "shared://🧬️mutations/🌳️shading/🌳️create/⛔️refuses",
    "delete-shading-surface-removes-an-awning": "shared://🧬️mutations/🌳️shading/🪵️delete/✅️removes",
    "delete-shading-surface-refuses-a-missing-one": "shared://🧬️mutations/🌳️shading/🪵️delete/⛔️refuses",
    "rename-shading-surface-renames-an-awning": "shared://🧬️mutations/🌳️shading/🏕️rename/✅️renames",
    "rename-shading-surface-refuses-a-blank-name": "shared://🧬️mutations/🌳️shading/🏕️rename/⛔️refuses",
    "replace-shading-surface-vertices-deepens-an-awning": "shared://🧬️mutations/🌳️shading/🗺️replace-vertices/✅️deepens",
    "replace-shading-surface-vertices-refuses-a-line": "shared://🧬️mutations/🌳️shading/🗺️replace-vertices/⛔️refuses",
    "change-shading-surface-transmittance-schedule-lets-light-in": "shared://🧬️mutations/🌳️shading/⛱️change-transmittance-schedule/✅️lets",
    "change-shading-surface-transmittance-schedule-refuses-a-ghost": "shared://🧬️mutations/🌳️shading/⛱️change-transmittance-schedule/⛔️refuses",
    "connect-surfaces-joins-two-walls": "shared://🧬️mutations/🔲️boundary/🤝️connect/✅️joins-two-walls",
    "connect-surfaces-refuses-a-self-pair": "shared://🧬️mutations/🔲️boundary/🤝️connect/⛔️refuses",
    "disconnect-surfaces-parts-two-walls": "shared://🧬️mutations/🔲️boundary/💔️disconnect/✅️parts",
    "disconnect-surfaces-refuses-a-missing-pair": "shared://🧬️mutations/🔲️boundary/💔️disconnect/⛔️refuses",
    "bind-fenestration-glazing-construction-glazes-a-window": "shared://🧬️mutations/🔍️glazing/🧊️bind-glazing-construction/✅️glazes",
    "bind-fenestration-glazing-construction-refuses-no-stack": "shared://🧬️mutations/🔍️glazing/🧊️bind-glazing-construction/⛔️refuses",
    "clear-fenestration-glazing-construction-ungazes-a-window": "shared://🧬️mutations/🔍️glazing/🫗️clear-glazing-construction/✅️ungazes",
    "clear-fenestration-glazing-construction-refuses-empty": "shared://🧬️mutations/🔍️glazing/🫗️clear-glazing-construction/⛔️refuses",
    "change-fenestration-height-raises-the-head": "shared://🧬️mutations/🪟️fenestration/⬆️change-height/✅️raises",
    "change-fenestration-height-refuses-zero-height": "shared://🧬️mutations/🪟️fenestration/⬆️change-height/⛔️refuses",
    "change-fenestration-sill-height-raises-the-sill": "shared://🧬️mutations/🪟️fenestration/⬇️change-sill-height/✅️raises",
    "change-fenestration-sill-height-refuses-negative-sill": "shared://🧬️mutations/🪟️fenestration/⬇️change-sill-height/⛔️refuses",
    "change-fenestration-overhang-depth-adds-case-610-shade": "shared://🧬️mutations/⛱️shading-device/🧢️change-overhang-depth/✅️adds",
    "change-fenestration-overhang-depth-refuses-negative-depth": "shared://🧬️mutations/⛱️shading-device/🧢️change-overhang-depth/⛔️refuses",
    "change-fenestration-overhang-offset-lifts-the-overhang": "shared://🧬️mutations/⛱️shading-device/🎩️change-overhang-offset/✅️lifts",
    "change-fenestration-overhang-offset-refuses-negative-offset": "shared://🧬️mutations/⛱️shading-device/🎩️change-overhang-offset/⛔️refuses",
    "change-fenestration-fin-depth-adds-case-630-fins": "shared://🧬️mutations/⛱️shading-device/🐬️change-fin-depth/✅️adds",
    "change-fenestration-fin-depth-refuses-negative-fin": "shared://🧬️mutations/⛱️shading-device/🐬️change-fin-depth/⛔️refuses",
    "change-fenestration-fin-offset-spreads-the-fins": "shared://🧬️mutations/⛱️shading-device/🐋️change-fin-offset/✅️spreads",
    "change-fenestration-fin-offset-refuses-negative-offset": "shared://🧬️mutations/⛱️shading-device/🐋️change-fin-offset/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_fenestration(before, payload):
    """🪟️ `create-fenestration` — Adds one window, skylight or door to an existing host surface, with the full optics, geometry and attached-shading payload the entity carries — including the optional `glazingConstructionId` that supersedes the three scalar optics fields when it is set."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["fenestrations"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not any(row["id"] == payload["surfaceId"] for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if (payload["glazingConstructionId"] is not None and not any(row["id"] == payload["glazingConstructionId"] for row in before["model"]["constructions"])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (payload["uValueWM2k"] == payload["uValueWM2k"] and abs(payload["uValueWM2k"]) != float("inf") and payload["uValueWM2k"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["shgc"] == payload["shgc"] and 0.0 <= payload["shgc"] <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["vlt"] == payload["vlt"] and 0.0 <= payload["vlt"] <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["areaM2"] == payload["areaM2"] and abs(payload["areaM2"]) != float("inf") and payload["areaM2"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["heightM"] == payload["heightM"] and abs(payload["heightM"]) != float("inf") and payload["heightM"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not (payload["sillHeightM"] == payload["sillHeightM"] and abs(payload["sillHeightM"]) != float("inf") and payload["sillHeightM"] >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["fenestrations"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "surface_id": payload["surfaceId"], "u_value_w_m2k": payload["uValueWM2k"], "shgc": payload["shgc"], "vlt": payload["vlt"], "area_m2": payload["areaM2"], "height_m": payload["heightM"], "sill_height_m": payload["sillHeightM"], "frame_conductance_w_k": payload["frameConductanceWK"], "divider_conductance_w_k": payload["dividerConductanceWK"], "overhang_depth_m": payload["overhangDepthM"], "overhang_offset_m": payload["overhangOffsetM"], "fin_depth_m": payload["finDepthM"], "fin_offset_m": payload["finOffsetM"], "glazing_construction_id": payload["glazingConstructionId"], "vertices_m": []}
    rows = after["model"]["fenestrations"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    if payload.get("index") is not None:
        position = payload["index"]
    rows.insert(position, created)
    return after, applied()


def _invert_create_fenestration(before, payload):
    return [("delete-fenestration", {"id": payload["id"]})]


def delete_fenestration(before, payload):
    """🚪️ `delete-fenestration{id}` — Removes one fenestration. Nothing in the document references a fenestration, so it neither cascades nor restricts."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["fenestrations"] = [row for row in after["model"]["fenestrations"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_fenestration(before, payload):
    index, item = next((position, row) for position, row in enumerate(before["model"]["fenestrations"]) if row["id"] == payload["id"])
    return [("create-fenestration", {"id": item["id"], "name": item["name"], "surfaceId": item["surface_id"], "uValueWM2k": item["u_value_w_m2k"], "shgc": item["shgc"], "vlt": item["vlt"], "areaM2": item["area_m2"], "heightM": item["height_m"], "sillHeightM": item["sill_height_m"], "frameConductanceWK": item["frame_conductance_w_k"], "dividerConductanceWK": item["divider_conductance_w_k"], "overhangDepthM": item["overhang_depth_m"], "overhangOffsetM": item["overhang_offset_m"], "finDepthM": item["fin_depth_m"], "finOffsetM": item["fin_offset_m"], "glazingConstructionId": item["glazing_construction_id"], "index": index})] + ([("replace-fenestration-vertices", {"id": item["id"], "newVerticesM": item["vertices_m"]})] if item["vertices_m"] else [])


def rename_fenestration(before, payload):
    """🏁️ `rename-fenestration{id,newName}` — Sets one fenestration's identity field; a name another fenestration already holds is refused."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["fenestrations"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_fenestration(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("rename-fenestration", {"id": payload["id"], "newName": item["name"]})]


def change_fenestration_surface(before, payload):
    """🧲️ `change-fenestration-surface{id,newSurfaceId}` — Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits."""
    entity_id, value = payload["id"], payload["newSurfaceId"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["surface_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["surface_id"] = value
    return after, applied()


def _invert_change_fenestration_surface(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-surface", {"id": payload["id"], "newSurfaceId": item["surface_id"]})]


def change_fenestration_u_value(before, payload):
    """🌐️ `change-fenestration-u-value{id,newUValueWM2k}` — Sets the glazing's air-to-air thermal transmittance in W/(m²·K) — the conductance the film-free window term multiplies by area and the outdoor-to-zone temperature difference."""
    entity_id, value = payload["id"], payload["newUValueWM2k"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["u_value_w_m2k"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["u_value_w_m2k"] = value
    return after, applied()


def _invert_change_fenestration_u_value(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-u-value", {"id": payload["id"], "newUValueWM2k": item["u_value_w_m2k"]})]


def change_fenestration_shgc(before, payload):
    """🌇️ `change-fenestration-shgc{id,newShgc}` — Sets the solar heat gain coefficient — the fraction of incident solar the glazing passes to the zone, directly and by re-radiation."""
    entity_id, value = payload["id"], payload["newShgc"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and 0.0 <= value <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["shgc"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["shgc"] = value
    return after, applied()


def _invert_change_fenestration_shgc(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-shgc", {"id": payload["id"], "newShgc": item["shgc"]})]


def change_fenestration_vlt(before, payload):
    """🌈️ `change-fenestration-vlt{id,newVlt}` — Sets the visible light transmittance — the daylight fraction the illuminance calculation reads, independent of the solar gain the SHGC governs."""
    entity_id, value = payload["id"], payload["newVlt"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and 0.0 <= value <= 1.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["vlt"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["vlt"] = value
    return after, applied()


def _invert_change_fenestration_vlt(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-vlt", {"id": payload["id"], "newVlt": item["vlt"]})]


def change_fenestration_area(before, payload):
    """🟥️ `change-fenestration-area{id,newAreaM2}` — Sets the glazed area in square metres — the area every window heat-balance and solar term is proportional to."""
    entity_id, value = payload["id"], payload["newAreaM2"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["area_m2"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["area_m2"] = value
    return after, applied()


def _invert_change_fenestration_area(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-area", {"id": payload["id"], "newAreaM2": item["area_m2"]})]


def change_fenestration_frame_conductance(before, payload):
    """🖼️ `change-fenestration-frame-conductance{id,newFrameConductanceWK}` — Sets the frame's whole-assembly thermal conductance in W/K, added to the glazing term rather than distributed over the area."""
    entity_id, value = payload["id"], payload["newFrameConductanceWK"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["frame_conductance_w_k"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["frame_conductance_w_k"] = value
    return after, applied()


def _invert_change_fenestration_frame_conductance(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-frame-conductance", {"id": payload["id"], "newFrameConductanceWK": item["frame_conductance_w_k"]})]


def change_fenestration_divider_conductance(before, payload):
    """🧷️ `change-fenestration-divider-conductance{id,newDividerConductanceWK}` — Sets the divider bars' whole-assembly thermal conductance in W/K, the frame term's sibling for the muntins inside the glazed area."""
    entity_id, value = payload["id"], payload["newDividerConductanceWK"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["divider_conductance_w_k"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["divider_conductance_w_k"] = value
    return after, applied()


def _invert_change_fenestration_divider_conductance(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-divider-conductance", {"id": payload["id"], "newDividerConductanceWK": item["divider_conductance_w_k"]})]


def create_shading_surface(before, payload):
    """🌳️ `create-shading-surface` — Adds one free-standing solar obstruction — the site-level sibling of a window's own attached overhang and fins, which stay on the fenestration."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["shading_surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if len(payload["verticesM"]) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if (payload["transmittanceScheduleId"] is not None and not any(row["id"] == payload["transmittanceScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["shading_surfaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "vertices_m": payload["verticesM"], "transmittance_schedule_id": payload["transmittanceScheduleId"]}
    rows = after["model"]["shading_surfaces"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    if payload.get("index") is not None:
        position = payload["index"]
    rows.insert(position, created)
    return after, applied()


def _invert_create_shading_surface(before, payload):
    return [("delete-shading-surface", {"id": payload["id"]})]


def delete_shading_surface(before, payload):
    """🪵️ `delete-shading-surface{id}` — Removes one free-standing solar obstruction. Nothing in the document references a shading surface, so it neither cascades nor restricts."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["shading_surfaces"] = [row for row in after["model"]["shading_surfaces"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_shading_surface(before, payload):
    index, item = next((position, row) for position, row in enumerate(before["model"]["shading_surfaces"]) if row["id"] == payload["id"])
    return [("create-shading-surface", {"id": item["id"], "name": item["name"], "verticesM": item["vertices_m"], "transmittanceScheduleId": item["transmittance_schedule_id"], "index": index})]


def rename_shading_surface(before, payload):
    """🏕️ `rename-shading-surface{id,newName}` — Sets one shading surface's identity field; a name another shading surface already holds is refused."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["shading_surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["shading_surfaces"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_shading_surface(before, payload):
    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("rename-shading-surface", {"id": payload["id"], "newName": item["name"]})]


def replace_shading_surface_vertices(before, payload):
    """🗺️ `replace-shading-surface-vertices{id,newVerticesM}` — Swaps the shading surface's whole polygon — geometry is one value, so the taxonomy's `replace` verb carries the full ring, minimum three vertices."""
    entity_id, value = payload["id"], payload["newVerticesM"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if len(value) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["vertices_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["shading_surfaces"]:
        if row["id"] == entity_id:
            row["vertices_m"] = value
    return after, applied()


def _invert_replace_shading_surface_vertices(before, payload):
    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("replace-shading-surface-vertices", {"id": payload["id"], "newVerticesM": item["vertices_m"]})]


def change_shading_surface_transmittance_schedule(before, payload):
    """⛱️ `change-shading-surface-transmittance-schedule{id,newTransmittanceScheduleId}` — Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction."""
    entity_id, value = payload["id"], payload["newTransmittanceScheduleId"]
    item = next((row for row in before["model"]["shading_surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if (value is not None and not any(row["id"] == value for family in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["transmittance_schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["shading_surfaces"]:
        if row["id"] == entity_id:
            row["transmittance_schedule_id"] = value
    return after, applied()


def _invert_change_shading_surface_transmittance_schedule(before, payload):
    item = next(row for row in before["model"]["shading_surfaces"] if row["id"] == payload["id"])
    return [("change-shading-surface-transmittance-schedule", {"id": payload["id"], "newTransmittanceScheduleId": item["transmittance_schedule_id"]})]


def connect_surfaces(before, payload):
    """🤝️ `connect-surfaces{surfaceAId,surfaceBId}` — taxonomy.md's `connect` verb on an edge
    collection with no id of its own, so the unordered pair is the address."""
    first, second = payload["surfaceAId"], payload["surfaceBId"]
    address = [str(first), str(second)]
    if first == second:
        return unchanged(before), rejected("mutation.invariant", address, invariant="distinct-surfaces")
    known = [row["id"] for row in before["model"]["surfaces"]]
    if first not in known or second not in known:
        return unchanged(before), rejected("mutation.target-missing", address)
    if any({row["surface_a_id"], row["surface_b_id"]} == {first, second} for row in before["model"]["adjacency_pairs"]):
        return unchanged(before), rejected("mutation.duplicate-id", address)
    if payload.get("index") is not None and payload["index"] > len(before["model"]["adjacency_pairs"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["surfaceAId"]), str(payload["surfaceBId"])])
    after = copy.deepcopy(before)
    rows = after["model"]["adjacency_pairs"]
    position = next((index for index, row in enumerate(rows) if (row["surface_a_id"], row["surface_b_id"]) > (first, second)), len(rows))
    if payload.get("index") is not None:
        position = payload["index"]
    rows.insert(position, {"surface_a_id": first, "surface_b_id": second})
    return after, applied()


def _invert_connect_surfaces(before, payload):
    return [("disconnect-surfaces", {"surfaceAId": payload["surfaceAId"], "surfaceBId": payload["surfaceBId"]})]


def disconnect_surfaces(before, payload):
    """💔️ `disconnect-surfaces{surfaceAId,surfaceBId}` — `connect`'s inverse partner."""
    first, second = payload["surfaceAId"], payload["surfaceBId"]
    if not any({row["surface_a_id"], row["surface_b_id"]} == {first, second} for row in before["model"]["adjacency_pairs"]):
        return unchanged(before), rejected("mutation.target-missing", [str(first), str(second)])
    after = copy.deepcopy(before)
    after["model"]["adjacency_pairs"] = [row for row in after["model"]["adjacency_pairs"] if {row["surface_a_id"], row["surface_b_id"]} != {first, second}]
    return after, applied()


def _invert_disconnect_surfaces(before, payload):
    index, item = next((position, row) for position, row in enumerate(before["model"]["adjacency_pairs"]) if {row["surface_a_id"], row["surface_b_id"]} == {payload["surfaceAId"], payload["surfaceBId"]})
    return [("connect-surfaces", {"surfaceAId": item["surface_a_id"], "surfaceBId": item["surface_b_id"], "index": index})]


def bind_fenestration_glazing_construction(before, payload):
    """🧊️ `bind-fenestration-glazing-construction{id,constructionId}` — taxonomy.md's `bind` verb
    filling the optional glazing slot with a real layer stack."""
    entity_id, construction_id = payload["id"], payload["constructionId"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == construction_id for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["glazing_construction_id"] == construction_id:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["glazing_construction_id"] = construction_id
    return after, applied()


def _invert_bind_fenestration_glazing_construction(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    previous = item["glazing_construction_id"]
    if previous is None:
        return [("clear-fenestration-glazing-construction", {"id": payload["id"]})]
    return [("bind-fenestration-glazing-construction", {"id": payload["id"], "constructionId": previous})]


def clear_fenestration_glazing_construction(before, payload):
    """🫗️ `clear-fenestration-glazing-construction{id}` — `bind`'s inverse partner."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None or item["glazing_construction_id"] is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["glazing_construction_id"] = None
    return after, applied()


def _invert_clear_fenestration_glazing_construction(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("bind-fenestration-glazing-construction", {"id": payload["id"], "constructionId": item["glazing_construction_id"]})]


def change_fenestration_height(before, payload):
    """⬆️ `change-fenestration-height{id,newHeightM}` — Sets the glazing's head-to-sill height in metres — the length the overhang and fin shadow geometry is projected against."""
    entity_id, value = payload["id"], payload["newHeightM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["height_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["height_m"] = value
    return after, applied()


def _invert_change_fenestration_height(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-height", {"id": payload["id"], "newHeightM": item["height_m"]})]


def change_fenestration_sill_height(before, payload):
    """⬇️ `change-fenestration-sill-height{id,newSillHeightM}` — Sets the height of the glazing's sill above the host surface's base in metres."""
    entity_id, value = payload["id"], payload["newSillHeightM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["sill_height_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["sill_height_m"] = value
    return after, applied()


def _invert_change_fenestration_sill_height(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-sill-height", {"id": payload["id"], "newSillHeightM": item["sill_height_m"]})]


def change_fenestration_overhang_depth(before, payload):
    """🧢️ `change-fenestration-overhang-depth{id,newOverhangDepthM}` — Sets how far the horizontal projection above the window head reaches out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 610/910 are exactly this field."""
    entity_id, value = payload["id"], payload["newOverhangDepthM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["overhang_depth_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["overhang_depth_m"] = value
    return after, applied()


def _invert_change_fenestration_overhang_depth(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-overhang-depth", {"id": payload["id"], "newOverhangDepthM": item["overhang_depth_m"]})]


def change_fenestration_overhang_offset(before, payload):
    """🎩️ `change-fenestration-overhang-offset{id,newOverhangOffsetM}` — Sets how far above the window head the horizontal projection sits, in metres."""
    entity_id, value = payload["id"], payload["newOverhangOffsetM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["overhang_offset_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["overhang_offset_m"] = value
    return after, applied()


def _invert_change_fenestration_overhang_offset(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-overhang-offset", {"id": payload["id"], "newOverhangOffsetM": item["overhang_offset_m"]})]


def change_fenestration_fin_depth(before, payload):
    """🐬️ `change-fenestration-fin-depth{id,newFinDepthM}` — Sets how far the two vertical projections beside the window jambs reach out of the glazing plane, in metres — ANSI/ASHRAE 140 §5.2 cases 630/930 are exactly this field."""
    entity_id, value = payload["id"], payload["newFinDepthM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["fin_depth_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["fin_depth_m"] = value
    return after, applied()


def _invert_change_fenestration_fin_depth(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-fin-depth", {"id": payload["id"], "newFinDepthM": item["fin_depth_m"]})]


def change_fenestration_fin_offset(before, payload):
    """🐋️ `change-fenestration-fin-offset{id,newFinOffsetM}` — Sets how far beside the window jambs the two vertical projections stand, in metres."""
    entity_id, value = payload["id"], payload["newFinOffsetM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["fin_offset_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["fin_offset_m"] = value
    return after, applied()


def _invert_change_fenestration_fin_offset(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("change-fenestration-fin-offset", {"id": payload["id"], "newFinOffsetM": item["fin_offset_m"]})]


VOCABULARY = {
    "create-fenestration": create_fenestration,
    "delete-fenestration": delete_fenestration,
    "rename-fenestration": rename_fenestration,
    "change-fenestration-surface": change_fenestration_surface,
    "change-fenestration-u-value": change_fenestration_u_value,
    "change-fenestration-shgc": change_fenestration_shgc,
    "change-fenestration-vlt": change_fenestration_vlt,
    "change-fenestration-area": change_fenestration_area,
    "change-fenestration-frame-conductance": change_fenestration_frame_conductance,
    "change-fenestration-divider-conductance": change_fenestration_divider_conductance,
    "create-shading-surface": create_shading_surface,
    "delete-shading-surface": delete_shading_surface,
    "rename-shading-surface": rename_shading_surface,
    "replace-shading-surface-vertices": replace_shading_surface_vertices,
    "change-shading-surface-transmittance-schedule": change_shading_surface_transmittance_schedule,
    "connect-surfaces": connect_surfaces,
    "disconnect-surfaces": disconnect_surfaces,
    "bind-fenestration-glazing-construction": bind_fenestration_glazing_construction,
    "clear-fenestration-glazing-construction": clear_fenestration_glazing_construction,
    "change-fenestration-height": change_fenestration_height,
    "change-fenestration-sill-height": change_fenestration_sill_height,
    "change-fenestration-overhang-depth": change_fenestration_overhang_depth,
    "change-fenestration-overhang-offset": change_fenestration_overhang_offset,
    "change-fenestration-fin-depth": change_fenestration_fin_depth,
    "change-fenestration-fin-offset": change_fenestration_fin_offset,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-fenestration": _invert_create_fenestration,
    "delete-fenestration": _invert_delete_fenestration,
    "rename-fenestration": _invert_rename_fenestration,
    "change-fenestration-surface": _invert_change_fenestration_surface,
    "change-fenestration-u-value": _invert_change_fenestration_u_value,
    "change-fenestration-shgc": _invert_change_fenestration_shgc,
    "change-fenestration-vlt": _invert_change_fenestration_vlt,
    "change-fenestration-area": _invert_change_fenestration_area,
    "change-fenestration-frame-conductance": _invert_change_fenestration_frame_conductance,
    "change-fenestration-divider-conductance": _invert_change_fenestration_divider_conductance,
    "create-shading-surface": _invert_create_shading_surface,
    "delete-shading-surface": _invert_delete_shading_surface,
    "rename-shading-surface": _invert_rename_shading_surface,
    "replace-shading-surface-vertices": _invert_replace_shading_surface_vertices,
    "change-shading-surface-transmittance-schedule": _invert_change_shading_surface_transmittance_schedule,
    "connect-surfaces": _invert_connect_surfaces,
    "disconnect-surfaces": _invert_disconnect_surfaces,
    "bind-fenestration-glazing-construction": _invert_bind_fenestration_glazing_construction,
    "clear-fenestration-glazing-construction": _invert_clear_fenestration_glazing_construction,
    "change-fenestration-height": _invert_change_fenestration_height,
    "change-fenestration-sill-height": _invert_change_fenestration_sill_height,
    "change-fenestration-overhang-depth": _invert_change_fenestration_overhang_depth,
    "change-fenestration-overhang-offset": _invert_change_fenestration_overhang_offset,
    "change-fenestration-fin-depth": _invert_change_fenestration_fin_depth,
    "change-fenestration-fin-offset": _invert_change_fenestration_fin_offset,
}
# endregion 🔖️Vocabulary
