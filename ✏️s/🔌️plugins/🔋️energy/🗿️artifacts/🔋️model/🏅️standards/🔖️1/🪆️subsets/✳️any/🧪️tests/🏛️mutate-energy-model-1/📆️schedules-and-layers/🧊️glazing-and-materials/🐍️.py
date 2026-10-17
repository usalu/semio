"""🧊️ Fenestration vertices, glazing and gas material kinds and surface roughness."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "replace-fenestration-vertices-shapes-a-gable": "shared://🧬️mutations/🪟️fenestration/🔶️replace-vertices/✅️shapes",
    "replace-fenestration-vertices-refuses-a-line": "shared://🧬️mutations/🪟️fenestration/🔶️replace-vertices/⛔️refuses",
    "change-glazing-material-thickness-applies": "shared://🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/✅️applies",
    "change-glazing-material-thickness-refuses": "shared://🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/⛔️refuses",
    "change-glazing-material-conductivity-applies": "shared://🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/✅️applies",
    "change-glazing-material-conductivity-refuses": "shared://🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/⛔️refuses",
    "change-glazing-material-solar-transmittance-applies": "shared://🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/✅️applies",
    "change-glazing-material-solar-transmittance-refuses": "shared://🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/⛔️refuses",
    "change-glazing-material-visible-transmittance-applies": "shared://🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/✅️applies",
    "change-glazing-material-visible-transmittance-refuses": "shared://🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/⛔️refuses",
    "change-glazing-material-infrared-emissivity-applies": "shared://🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/✅️applies",
    "change-glazing-material-infrared-emissivity-refuses": "shared://🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/⛔️refuses",
    "rename-glazing-material-applies": "shared://🧬️mutations/🥂️window-layer/🟢️rename-glazing/✅️applies",
    "rename-glazing-material-refuses": "shared://🧬️mutations/🥂️window-layer/🟢️rename-glazing/⛔️refuses",
    "change-gas-material-thickness-applies": "shared://🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/✅️applies",
    "change-gas-material-thickness-refuses": "shared://🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/⛔️refuses",
    "change-gas-material-gas-applies": "shared://🧬️mutations/🥂️window-layer/🟤️change-gas-type/✅️applies",
    "change-gas-material-gas-refuses": "shared://🧬️mutations/🥂️window-layer/🟤️change-gas-type/⛔️refuses",
    "rename-gas-material-applies": "shared://🧬️mutations/🥂️window-layer/🔘️rename-gas/✅️applies",
    "rename-gas-material-refuses": "shared://🧬️mutations/🥂️window-layer/🔘️rename-gas/⛔️refuses",
    "change-material-roughness-applies": "shared://🧬️mutations/🎨️finish/🗻️change-roughness/✅️applies",
    "change-material-roughness-refuses": "shared://🧬️mutations/🎨️finish/🗻️change-roughness/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def replace_fenestration_vertices(before, payload):
    """🔶️ `replace-fenestration-vertices{id,newVerticesM}` — Swaps the aperture's own polygon. An empty ring hands the shape back to the derived area/height/sill rectangle; three or more vertices, planar and in the host surface's plane, ARE the aperture."""
    entity_id, value = payload["id"], payload["newVerticesM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if value and len(value) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(not _finite(axis) for corner in value for axis in corner):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    host = next((row for row in before["model"]["surfaces"] if row["id"] == item["surface_id"]), None)
    if host is not None and not _on_plane(value, host["vertices_m"]):
        return unchanged(before), rejected("mutation.target-mismatch", [str(entity_id)])
    if item["vertices_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["vertices_m"] = value
    return after, applied()


def _invert_replace_fenestration_vertices(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("replace-fenestration-vertices", {"id": payload["id"], "newVerticesM": item["vertices_m"]})]


def _finite(value):
    """🔢️ True when `value` is a real, finite number (the payload schema admits JSON numbers only)."""
    return isinstance(value, (int, float)) and value == value and value not in (float("inf"), float("-inf"))


def _on_plane(polygon, host, tolerance=1e-3):
    """📐️ True when every vertex of `polygon` sits within `tolerance` metres of `host`'s plane."""
    if len(host) < 3:
        return True
    normal = [0.0, 0.0, 0.0]
    for index, current in enumerate(host):
        following = host[(index + 1) % len(host)]
        normal[0] += (current[1] - following[1]) * (current[2] + following[2])
        normal[1] += (current[2] - following[2]) * (current[0] + following[0])
        normal[2] += (current[0] - following[0]) * (current[1] + following[1])
    length = sum(axis * axis for axis in normal) ** 0.5
    if length <= 1e-12:
        return True
    normal = [axis / length for axis in normal]
    anchor = host[0]
    return all(abs(sum((vertex[axis] - anchor[axis]) * normal[axis] for axis in range(3))) <= tolerance for vertex in polygon)


def _change_glazing_scalar(before, payload, key, field, fraction):
    """🪟️ Shared body of every one-scalar glazing-material change."""
    entity_id, value = payload["id"], payload[key]
    item = next((row for row in before["model"]["glazing_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    admissible = _finite(value) and (0.0 <= value <= 1.0 if fraction else value > 0.0)
    if not admissible:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item[field] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["glazing_materials"]:
        if row["id"] == entity_id:
            row[field] = value
    return after, applied()


def _invert_glazing_scalar(before, payload, kind, key, field):
    item = next(row for row in before["model"]["glazing_materials"] if row["id"] == payload["id"])
    return [(kind, {"id": payload["id"], key: item[field]})]


def change_glazing_material_thickness(before, payload):
    """🔷️ `change-glazing-material-thickness{id,newThicknessM}` — Sets pane thickness (m) on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newThicknessM", "thickness_m", False)


def _invert_change_glazing_material_thickness(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-thickness", "newThicknessM", "thickness_m")


def change_glazing_material_conductivity(before, payload):
    """🟠️ `change-glazing-material-conductivity{id,newConductivityWMK}` — Sets pane conductivity (W/m·K) on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newConductivityWMK", "conductivity_w_m_k", False)


def _invert_change_glazing_material_conductivity(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-conductivity", "newConductivityWMK", "conductivity_w_m_k")


def change_glazing_material_solar_transmittance(before, payload):
    """🟡️ `change-glazing-material-solar-transmittance{id,newSolarTransmittance}` — Sets normal-incidence solar transmittance on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newSolarTransmittance", "solar_transmittance", True)


def _invert_change_glazing_material_solar_transmittance(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-solar-transmittance", "newSolarTransmittance", "solar_transmittance")


def change_glazing_material_visible_transmittance(before, payload):
    """🥽️ `change-glazing-material-visible-transmittance{id,newVisibleTransmittance}` — Sets normal-incidence visible transmittance on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newVisibleTransmittance", "visible_transmittance", True)


def _invert_change_glazing_material_visible_transmittance(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-visible-transmittance", "newVisibleTransmittance", "visible_transmittance")


def change_glazing_material_infrared_emissivity(before, payload):
    """🩻️ `change-glazing-material-infrared-emissivity{id,newInfraredEmissivityFront,newInfraredEmissivityBack}` — Sets both long-wave emissivities of one glazing pane in one step."""
    entity_id = payload["id"]
    front, back = payload["newInfraredEmissivityFront"], payload["newInfraredEmissivityBack"]
    item = next((row for row in before["model"]["glazing_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(not (_finite(value) and 0.0 <= value <= 1.0) for value in (front, back)):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["infrared_emissivity_front"] == front and item["infrared_emissivity_back"] == back:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["glazing_materials"]:
        if row["id"] == entity_id:
            row["infrared_emissivity_front"] = front
            row["infrared_emissivity_back"] = back
    return after, applied()


def _invert_change_glazing_material_infrared_emissivity(before, payload):
    item = next(row for row in before["model"]["glazing_materials"] if row["id"] == payload["id"])
    return [("change-glazing-material-infrared-emissivity", {"id": payload["id"], "newInfraredEmissivityFront": item["infrared_emissivity_front"], "newInfraredEmissivityBack": item["infrared_emissivity_back"]})]


def rename_glazing_material(before, payload):
    """🟢️ `rename-glazing-material{id,newName}` — Renames one glazing material, addressed by id."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["glazing_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["glazing_materials"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_glazing_material(before, payload):
    item = next(row for row in before["model"]["glazing_materials"] if row["id"] == payload["id"])
    return [("rename-glazing-material", {"id": payload["id"], "newName": item["name"]})]


def change_gas_material_thickness(before, payload):
    """🟣️ `change-gas-material-thickness{id,newThicknessM}` — Sets gap width (m) on one gas material, addressed by id."""
    entity_id, value = payload["id"], payload["newThicknessM"]
    item = next((row for row in before["model"]["gas_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (_finite(value) and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["thickness_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["gas_materials"]:
        if row["id"] == entity_id:
            row["thickness_m"] = value
    return after, applied()


def _invert_change_gas_material_thickness(before, payload):
    item = next(row for row in before["model"]["gas_materials"] if row["id"] == payload["id"])
    return [("change-gas-material-thickness", {"id": payload["id"], "newThicknessM": item["thickness_m"]})]


def change_gas_material_gas(before, payload):
    """🟤️ `change-gas-material-gas{id,newGas}` — Sets which fill gas occupies one glazing gap, addressed by id."""
    entity_id, value = payload["id"], payload["newGas"]
    item = next((row for row in before["model"]["gas_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["gas"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["gas_materials"]:
        if row["id"] == entity_id:
            row["gas"] = value
    return after, applied()


def _invert_change_gas_material_gas(before, payload):
    item = next(row for row in before["model"]["gas_materials"] if row["id"] == payload["id"])
    return [("change-gas-material-gas", {"id": payload["id"], "newGas": item["gas"]})]


def rename_gas_material(before, payload):
    """🔘️ `rename-gas-material{id,newName}` — Renames one gas material, addressed by id."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["gas_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["gas_materials"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_gas_material(before, payload):
    item = next(row for row in before["model"]["gas_materials"] if row["id"] == payload["id"])
    return [("rename-gas-material", {"id": payload["id"], "newName": item["name"]})]


def change_material_roughness(before, payload):
    """🗻️ `change-material-roughness{id,newRoughness}` — Sets the surface roughness class of one material, addressed by id."""
    entity_id, value = payload["id"], payload["newRoughness"]
    item = next((row for row in before["model"]["materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["roughness"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == entity_id:
            row["roughness"] = value
    return after, applied()


def _invert_change_material_roughness(before, payload):
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-roughness", {"id": payload["id"], "newRoughness": item["roughness"]})]


VOCABULARY = {
    "replace-fenestration-vertices": replace_fenestration_vertices,
    "change-glazing-material-thickness": change_glazing_material_thickness,
    "change-glazing-material-conductivity": change_glazing_material_conductivity,
    "change-glazing-material-solar-transmittance": change_glazing_material_solar_transmittance,
    "change-glazing-material-visible-transmittance": change_glazing_material_visible_transmittance,
    "change-glazing-material-infrared-emissivity": change_glazing_material_infrared_emissivity,
    "rename-glazing-material": rename_glazing_material,
    "change-gas-material-thickness": change_gas_material_thickness,
    "change-gas-material-gas": change_gas_material_gas,
    "rename-gas-material": rename_gas_material,
    "change-material-roughness": change_material_roughness,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "replace-fenestration-vertices": _invert_replace_fenestration_vertices,
    "change-glazing-material-thickness": _invert_change_glazing_material_thickness,
    "change-glazing-material-conductivity": _invert_change_glazing_material_conductivity,
    "change-glazing-material-solar-transmittance": _invert_change_glazing_material_solar_transmittance,
    "change-glazing-material-visible-transmittance": _invert_change_glazing_material_visible_transmittance,
    "change-glazing-material-infrared-emissivity": _invert_change_glazing_material_infrared_emissivity,
    "rename-glazing-material": _invert_rename_glazing_material,
    "change-gas-material-thickness": _invert_change_gas_material_thickness,
    "change-gas-material-gas": _invert_change_gas_material_gas,
    "rename-gas-material": _invert_rename_gas_material,
    "change-material-roughness": _invert_change_material_roughness,
}
# endregion 🔖️Vocabulary
