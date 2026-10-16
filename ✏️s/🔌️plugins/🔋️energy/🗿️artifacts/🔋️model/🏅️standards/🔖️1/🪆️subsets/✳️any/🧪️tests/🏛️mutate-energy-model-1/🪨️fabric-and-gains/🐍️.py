"""🪨️ Fabric and gain kinds: materials, finishes, constructions and the people, lighting and equipment gains."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-material-applies": "shared://🧬️mutations/🪨️material/🧱️create/✅️applies",
    "create-material-refuses": "shared://🧬️mutations/🪨️material/🧱️create/⛔️refuses",
    "delete-material-applies": "shared://🧬️mutations/🪨️material/🪨️delete/✅️applies",
    "delete-material-refuses": "shared://🧬️mutations/🪨️material/🪨️delete/⛔️refuses",
    "rename-material-applies": "shared://🧬️mutations/🪨️material/🪧️rename/✅️applies",
    "rename-material-refuses": "shared://🧬️mutations/🪨️material/🪧️rename/⛔️refuses",
    "change-material-thickness-applies": "shared://🧬️mutations/🪨️material/📏️change-thickness/✅️applies",
    "change-material-thickness-refuses": "shared://🧬️mutations/🪨️material/📏️change-thickness/⛔️refuses",
    "change-material-conductivity-applies": "shared://🧬️mutations/🪨️material/🔥️change-conductivity/✅️applies",
    "change-material-conductivity-refuses": "shared://🧬️mutations/🪨️material/🔥️change-conductivity/⛔️refuses",
    "change-material-density-applies": "shared://🧬️mutations/🪨️material/⚖️change-density/✅️applies",
    "change-material-density-refuses": "shared://🧬️mutations/🪨️material/⚖️change-density/⛔️refuses",
    "change-material-specific-heat-applies": "shared://🧬️mutations/🪨️material/♨️change-specific-heat/✅️applies",
    "change-material-specific-heat-refuses": "shared://🧬️mutations/🪨️material/♨️change-specific-heat/⛔️refuses",
    "change-material-thermal-absorptance-applies": "shared://🧬️mutations/🎨️finish/🔆️change-thermal-absorptance/✅️applies",
    "change-material-thermal-absorptance-refuses": "shared://🧬️mutations/🎨️finish/🔆️change-thermal-absorptance/⛔️refuses",
    "change-material-solar-absorptance-applies": "shared://🧬️mutations/🎨️finish/☀️change-solar-absorptance/✅️applies",
    "change-material-solar-absorptance-refuses": "shared://🧬️mutations/🎨️finish/☀️change-solar-absorptance/⛔️refuses",
    "change-material-visible-absorptance-applies": "shared://🧬️mutations/🎨️finish/👁️change-visible-absorptance/✅️applies",
    "change-material-visible-absorptance-refuses": "shared://🧬️mutations/🎨️finish/👁️change-visible-absorptance/⛔️refuses",
    "create-construction-applies": "shared://🧬️mutations/🏗️construction/🏗️create/✅️applies",
    "create-construction-refuses": "shared://🧬️mutations/🏗️construction/🏗️create/⛔️refuses",
    "delete-construction-applies": "shared://🧬️mutations/🏗️construction/🧨️delete/✅️applies",
    "delete-construction-refuses": "shared://🧬️mutations/🏗️construction/🧨️delete/⛔️refuses",
    "rename-construction-applies": "shared://🧬️mutations/🏗️construction/🪪️rename/✅️applies",
    "rename-construction-refuses": "shared://🧬️mutations/🏗️construction/🪪️rename/⛔️refuses",
    "add-construction-layer-applies": "shared://🧬️mutations/🏗️construction/➕️add-layer/✅️applies",
    "add-construction-layer-refuses": "shared://🧬️mutations/🏗️construction/➕️add-layer/⛔️refuses",
    "remove-construction-layer-applies": "shared://🧬️mutations/🏗️construction/➖️remove-layer/✅️applies",
    "remove-construction-layer-refuses": "shared://🧬️mutations/🏗️construction/➖️remove-layer/⛔️refuses",
    "reorder-construction-layers-applies": "shared://🧬️mutations/🏗️construction/🔀️reorder-layers/✅️applies",
    "reorder-construction-layers-refuses": "shared://🧬️mutations/🏗️construction/🔀️reorder-layers/⛔️refuses",
    "create-people-gain-applies": "shared://🧬️mutations/👥️people-gain/👤️create/✅️applies",
    "create-people-gain-refuses": "shared://🧬️mutations/👥️people-gain/👤️create/⛔️refuses",
    "delete-people-gain-applies": "shared://🧬️mutations/👥️people-gain/🚷️delete/✅️applies",
    "delete-people-gain-refuses": "shared://🧬️mutations/👥️people-gain/🚷️delete/⛔️refuses",
    "change-people-gain-zone-applies": "shared://🧬️mutations/👥️people-gain/🚶️change-zone/✅️applies",
    "change-people-gain-zone-refuses": "shared://🧬️mutations/👥️people-gain/🚶️change-zone/⛔️refuses",
    "change-people-gain-schedule-applies": "shared://🧬️mutations/👥️people-gain/⏰️change-schedule/✅️applies",
    "change-people-gain-schedule-refuses": "shared://🧬️mutations/👥️people-gain/⏰️change-schedule/⛔️refuses",
    "change-people-gain-activity-schedule-applies": "shared://🧬️mutations/👥️people-gain/🏃️change-activity-schedule/✅️applies",
    "change-people-gain-activity-schedule-refuses": "shared://🧬️mutations/👥️people-gain/🏃️change-activity-schedule/⛔️refuses",
    "change-people-gain-people-per-area-applies": "shared://🧬️mutations/👥️people-gain/👥️change-people-per-area/✅️applies",
    "change-people-gain-people-per-area-refuses": "shared://🧬️mutations/👥️people-gain/👥️change-people-per-area/⛔️refuses",
    "change-people-gain-sensible-fraction-applies": "shared://🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/✅️applies",
    "change-people-gain-sensible-fraction-refuses": "shared://🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/⛔️refuses",
    "change-people-gain-latent-fraction-applies": "shared://🧬️mutations/👥️people-gain/💧️change-latent-fraction/✅️applies",
    "change-people-gain-latent-fraction-refuses": "shared://🧬️mutations/👥️people-gain/💧️change-latent-fraction/⛔️refuses",
    "change-people-gain-radiant-fraction-applies": "shared://🧬️mutations/👥️people-gain/📡️change-radiant-fraction/✅️applies",
    "change-people-gain-radiant-fraction-refuses": "shared://🧬️mutations/👥️people-gain/📡️change-radiant-fraction/⛔️refuses",
    "create-lighting-gain-applies": "shared://🧬️mutations/💡️lighting-gain/💡️create/✅️applies",
    "create-lighting-gain-refuses": "shared://🧬️mutations/💡️lighting-gain/💡️create/⛔️refuses",
    "delete-lighting-gain-applies": "shared://🧬️mutations/💡️lighting-gain/🕯️delete/✅️applies",
    "delete-lighting-gain-refuses": "shared://🧬️mutations/💡️lighting-gain/🕯️delete/⛔️refuses",
    "change-lighting-gain-zone-applies": "shared://🧬️mutations/💡️lighting-gain/🔦️change-zone/✅️applies",
    "change-lighting-gain-zone-refuses": "shared://🧬️mutations/💡️lighting-gain/🔦️change-zone/⛔️refuses",
    "change-lighting-gain-schedule-applies": "shared://🧬️mutations/💡️lighting-gain/⏱️change-schedule/✅️applies",
    "change-lighting-gain-schedule-refuses": "shared://🧬️mutations/💡️lighting-gain/⏱️change-schedule/⛔️refuses",
    "change-lighting-gain-watts-per-area-applies": "shared://🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/✅️applies",
    "change-lighting-gain-watts-per-area-refuses": "shared://🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/⛔️refuses",
    "change-lighting-gain-radiant-fraction-applies": "shared://🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/✅️applies",
    "change-lighting-gain-radiant-fraction-refuses": "shared://🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/⛔️refuses",
    "change-lighting-gain-visible-fraction-applies": "shared://🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/✅️applies",
    "change-lighting-gain-visible-fraction-refuses": "shared://🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/⛔️refuses",
    "change-lighting-gain-return-air-fraction-applies": "shared://🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/✅️applies",
    "change-lighting-gain-return-air-fraction-refuses": "shared://🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/⛔️refuses",
    "create-equipment-gain-applies": "shared://🧬️mutations/🖥️equipment-gain/🖥️create/✅️applies",
    "create-equipment-gain-refuses": "shared://🧬️mutations/🖥️equipment-gain/🖥️create/⛔️refuses",
    "delete-equipment-gain-applies": "shared://🧬️mutations/🖥️equipment-gain/🧯️delete/✅️applies",
    "delete-equipment-gain-refuses": "shared://🧬️mutations/🖥️equipment-gain/🧯️delete/⛔️refuses",
    "change-equipment-gain-zone-applies": "shared://🧬️mutations/🖥️equipment-gain/🖨️change-zone/✅️applies",
    "change-equipment-gain-zone-refuses": "shared://🧬️mutations/🖥️equipment-gain/🖨️change-zone/⛔️refuses",
    "change-equipment-gain-schedule-applies": "shared://🧬️mutations/🖥️equipment-gain/⌛️change-schedule/✅️applies",
    "change-equipment-gain-schedule-refuses": "shared://🧬️mutations/🖥️equipment-gain/⌛️change-schedule/⛔️refuses",
    "change-equipment-gain-watts-per-area-applies": "shared://🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/✅️applies",
    "change-equipment-gain-watts-per-area-refuses": "shared://🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/⛔️refuses",
    "change-equipment-gain-radiant-fraction-applies": "shared://🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/✅️applies",
    "change-equipment-gain-radiant-fraction-refuses": "shared://🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/⛔️refuses",
    "change-equipment-gain-latent-fraction-applies": "shared://🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/✅️applies",
    "change-equipment-gain-latent-fraction-refuses": "shared://🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_material(before, payload):
    """🧱️ `create-material{index,…}` — Adds one opaque material layer definition at a stated position in the model's material list. Thickness, conductivity, density and specific heat are the four the conduction transfer functions integrate; the three absorptances close the surface radiation balance."""
    rows = before["model"]["materials"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["materials"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "roughness": payload["roughness"], "thickness_m": payload["thicknessM"], "conductivity_w_m_k": payload["conductivityWMK"], "density_kg_m3": payload["densityKgM3"], "specific_heat_j_kg_k": payload["specificHeatJKgK"], "thermal_absorptance": payload["thermalAbsorptance"], "solar_absorptance": payload["solarAbsorptance"], "visible_absorptance": payload["visibleAbsorptance"]})
    return after, applied()


def _invert_create_material(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-material", {"id": payload["id"]})]


def delete_material(before, payload):
    """🪨️ `delete-material{id}` — Removes one material definition. Refused while any construction still names it as a layer — the alternative would be to cascade into constructions, which cascade into surfaces."""
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(payload["id"] in row["layer_material_ids"] for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["materials"] = [row for row in after["model"]["materials"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_material(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["materials"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-material", {"index": index, "id": item["id"], "name": item["name"], "roughness": item["roughness"], "thicknessM": item["thickness_m"], "conductivityWMK": item["conductivity_w_m_k"], "densityKgM3": item["density_kg_m3"], "specificHeatJKgK": item["specific_heat_j_kg_k"], "thermalAbsorptance": item["thermal_absorptance"], "solarAbsorptance": item["solar_absorptance"], "visibleAbsorptance": item["visible_absorptance"]})]


def rename_material(before, payload):
    """🪧️ `rename-material{id,newName}` — Sets one material's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["materials"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_material(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("rename-material", {"id": payload["id"], "newName": item["name"]})]


def change_material_thickness(before, payload):
    """📏️ `change-material-thickness{id,newThicknessM}` — Sets thickness (m) on one material, addressed by id."""
    value = payload["newThicknessM"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["thickness_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["thickness_m"] = value
    return after, applied()


def _invert_change_material_thickness(before, payload):
    """↩️ The thickness_m the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-thickness", {"id": payload["id"], "newThicknessM": item["thickness_m"]})]


def change_material_conductivity(before, payload):
    """🔥️ `change-material-conductivity{id,newConductivityWMK}` — Sets conductivity (W/m·K) on one material, addressed by id."""
    value = payload["newConductivityWMK"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["conductivity_w_m_k"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["conductivity_w_m_k"] = value
    return after, applied()


def _invert_change_material_conductivity(before, payload):
    """↩️ The conductivity_w_m_k the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-conductivity", {"id": payload["id"], "newConductivityWMK": item["conductivity_w_m_k"]})]


def change_material_density(before, payload):
    """⚖️ `change-material-density{id,newDensityKgM3}` — Sets density (kg/m³) on one material, addressed by id."""
    value = payload["newDensityKgM3"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["density_kg_m3"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["density_kg_m3"] = value
    return after, applied()


def _invert_change_material_density(before, payload):
    """↩️ The density_kg_m3 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-density", {"id": payload["id"], "newDensityKgM3": item["density_kg_m3"]})]


def change_material_specific_heat(before, payload):
    """♨️ `change-material-specific-heat{id,newSpecificHeatJKgK}` — Sets specific heat (J/kg·K) on one material, addressed by id."""
    value = payload["newSpecificHeatJKgK"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["specific_heat_j_kg_k"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["specific_heat_j_kg_k"] = value
    return after, applied()


def _invert_change_material_specific_heat(before, payload):
    """↩️ The specific_heat_j_kg_k the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-specific-heat", {"id": payload["id"], "newSpecificHeatJKgK": item["specific_heat_j_kg_k"]})]


def change_material_thermal_absorptance(before, payload):
    """🔆️ `change-material-thermal-absorptance{id,newThermalAbsorptance}` — Sets thermal absorptance on one material, addressed by id."""
    value = payload["newThermalAbsorptance"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["thermal_absorptance"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["thermal_absorptance"] = value
    return after, applied()


def _invert_change_material_thermal_absorptance(before, payload):
    """↩️ The thermal_absorptance the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-thermal-absorptance", {"id": payload["id"], "newThermalAbsorptance": item["thermal_absorptance"]})]


def change_material_solar_absorptance(before, payload):
    """☀️ `change-material-solar-absorptance{id,newSolarAbsorptance}` — Sets solar absorptance on one material, addressed by id."""
    value = payload["newSolarAbsorptance"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["solar_absorptance"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["solar_absorptance"] = value
    return after, applied()


def _invert_change_material_solar_absorptance(before, payload):
    """↩️ The solar_absorptance the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-solar-absorptance", {"id": payload["id"], "newSolarAbsorptance": item["solar_absorptance"]})]


def change_material_visible_absorptance(before, payload):
    """👁️ `change-material-visible-absorptance{id,newVisibleAbsorptance}` — Sets visible absorptance on one material, addressed by id."""
    value = payload["newVisibleAbsorptance"]
    item = next((row for row in before["model"]["materials"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["visible_absorptance"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == payload["id"]:
            row["visible_absorptance"] = value
    return after, applied()


def _invert_change_material_visible_absorptance(before, payload):
    """↩️ The visible_absorptance the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-visible-absorptance", {"id": payload["id"], "newVisibleAbsorptance": item["visible_absorptance"]})]


def create_construction(before, payload):
    """🏗️ `create-construction{index,…}` — Adds one layered construction, outside-to-inside layer order. Every layer must already name a material the document defines, so a construction is never born dangling."""
    rows = before["model"]["constructions"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    missing = next((identifier for identifier in payload["layerMaterialIds"] if not any(row["id"] == identifier for row in before["model"]["materials"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["constructions"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "layer_material_ids": payload["layerMaterialIds"]})
    return after, applied()


def _invert_create_construction(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-construction", {"id": payload["id"]})]


def delete_construction(before, payload):
    """🧨️ `delete-construction{id}` — Removes one construction. Refused while any surface still names it, because a surface without a construction has no heat path at all."""
    item = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(row["construction_id"] == payload["id"] for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["constructions"] = [row for row in after["model"]["constructions"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_construction(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["constructions"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-construction", {"index": index, "id": item["id"], "name": item["name"], "layerMaterialIds": item["layer_material_ids"]})]


def rename_construction(before, payload):
    """🪪️ `rename-construction{id,newName}` — Sets one construction's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_construction(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("rename-construction", {"id": payload["id"], "newName": item["name"]})]


def add_construction_layer(before, payload):
    """➕️ `add-construction-layer{id,index,materialId}` — one layer inserted at a stated position,
    outside-to-inside; layer order is physically load-bearing."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["materialId"] for row in before["model"]["materials"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["materialId"])])
    if payload["index"] > len(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["layer_material_ids"].insert(payload["index"], payload["materialId"])
    return after, applied()


def _invert_add_construction_layer(before, payload):
    """↩️ Undone by removing the layer at exactly the index it was inserted at."""
    return [("remove-construction-layer", {"id": payload["id"], "index": payload["index"]})]


def remove_construction_layer(before, payload):
    """➖️ `remove-construction-layer{id,index}` — addressed by position, because one material may
    legitimately appear in a construction more than once."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] >= len(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            del row["layer_material_ids"][payload["index"]]
    return after, applied()


def _invert_remove_construction_layer(before, payload):
    """↩️ Re-inserts exactly the material that stood at that index, read off BASE."""
    construction = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("add-construction-layer", {"id": payload["id"], "index": payload["index"], "materialId": construction["layer_material_ids"][payload["index"]]})]


def reorder_construction_layers(before, payload):
    """🔀️ `reorder-construction-layers{id,newLayerMaterialIds}` — a permutation of the layers the
    construction already holds; exchanging a layer is a different kind's job."""
    construction = next((row for row in before["model"]["constructions"] if row["id"] == payload["id"]), None)
    if construction is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    wanted = payload["newLayerMaterialIds"]
    if sorted(wanted) != sorted(construction["layer_material_ids"]):
        return unchanged(before), rejected("mutation.target-mismatch", [str(payload["id"])])
    if construction["layer_material_ids"] == wanted:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["constructions"]:
        if row["id"] == payload["id"]:
            row["layer_material_ids"] = list(wanted)
    return after, applied()


def _invert_reorder_construction_layers(before, payload):
    """↩️ The order BASE held, restated."""
    construction = next(row for row in before["model"]["constructions"] if row["id"] == payload["id"])
    return [("reorder-construction-layers", {"id": payload["id"], "newLayerMaterialIds": list(construction["layer_material_ids"])})]


def create_people_gain(before, payload):
    """👤️ `create-people-gain{index,…}` — Adds one occupancy gain to a zone: the occupant density, the occupancy schedule, the metabolic activity schedule and the sensible/latent/radiant split the zone heat balance needs."""
    rows = before["model"]["people"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    if not any(row["id"] == payload["activityScheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["activityScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["people"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "activity_schedule_id": payload["activityScheduleId"], "people_per_area": payload["peoplePerArea"], "sensible_fraction": payload["sensibleFraction"], "latent_fraction": payload["latentFraction"], "radiant_fraction": payload["radiantFraction"]})
    return after, applied()


def _invert_create_people_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-people-gain", {"id": payload["id"]})]


def delete_people_gain(before, payload):
    """🚷️ `delete-people-gain{id}` — Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use."""
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["people"] = [row for row in after["model"]["people"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_people_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["people"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-people-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "activityScheduleId": item["activity_schedule_id"], "peoplePerArea": item["people_per_area"], "sensibleFraction": item["sensible_fraction"], "latentFraction": item["latent_fraction"], "radiantFraction": item["radiant_fraction"]})]


def change_people_gain_zone(before, payload):
    """🚶️ `change-people-gain-zone{id,newZoneId}` — Re-points one people gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_people_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_people_gain_schedule(before, payload):
    """⏰️ `change-people-gain-schedule{id,newScheduleId}` — Re-points one people gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_people_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_people_gain_activity_schedule(before, payload):
    """🏃️ `change-people-gain-activity-schedule{id,newActivityScheduleId}` — Re-points one people gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newActivityScheduleId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["activity_schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["activity_schedule_id"] = value
    return after, applied()


def _invert_change_people_gain_activity_schedule(before, payload):
    """↩️ The activity_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-activity-schedule", {"id": payload["id"], "newActivityScheduleId": item["activity_schedule_id"]})]


def change_people_gain_people_per_area(before, payload):
    """👥️ `change-people-gain-people-per-area{id,newPeoplePerArea}` — Sets occupant density (people/m²) on one people gain, addressed by id."""
    value = payload["newPeoplePerArea"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["people_per_area"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["people_per_area"] = value
    return after, applied()


def _invert_change_people_gain_people_per_area(before, payload):
    """↩️ The people_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-people-per-area", {"id": payload["id"], "newPeoplePerArea": item["people_per_area"]})]


def change_people_gain_sensible_fraction(before, payload):
    """🌞️ `change-people-gain-sensible-fraction{id,newSensibleFraction}` — Sets sensible fraction on one people gain, addressed by id."""
    value = payload["newSensibleFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["sensible_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["sensible_fraction"] = value
    return after, applied()


def _invert_change_people_gain_sensible_fraction(before, payload):
    """↩️ The sensible_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-sensible-fraction", {"id": payload["id"], "newSensibleFraction": item["sensible_fraction"]})]


def change_people_gain_latent_fraction(before, payload):
    """💧️ `change-people-gain-latent-fraction{id,newLatentFraction}` — Sets latent fraction on one people gain, addressed by id."""
    value = payload["newLatentFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["latent_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["latent_fraction"] = value
    return after, applied()


def _invert_change_people_gain_latent_fraction(before, payload):
    """↩️ The latent_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-latent-fraction", {"id": payload["id"], "newLatentFraction": item["latent_fraction"]})]


def change_people_gain_radiant_fraction(before, payload):
    """📡️ `change-people-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one people gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_people_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def create_lighting_gain(before, payload):
    """💡️ `create-lighting-gain{index,…}` — Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly."""
    rows = before["model"]["lighting"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["lighting"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "watts_per_area": payload["wattsPerArea"], "radiant_fraction": payload["radiantFraction"], "visible_fraction": payload["visibleFraction"], "return_air_fraction": payload["returnAirFraction"]})
    return after, applied()


def _invert_create_lighting_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-lighting-gain", {"id": payload["id"]})]


def delete_lighting_gain(before, payload):
    """🕯️ `delete-lighting-gain{id}` — Removes one lighting gain."""
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["lighting"] = [row for row in after["model"]["lighting"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_lighting_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["lighting"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-lighting-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "wattsPerArea": item["watts_per_area"], "radiantFraction": item["radiant_fraction"], "visibleFraction": item["visible_fraction"], "returnAirFraction": item["return_air_fraction"]})]


def change_lighting_gain_zone(before, payload):
    """🔦️ `change-lighting-gain-zone{id,newZoneId}` — Re-points one lighting gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_lighting_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_lighting_gain_schedule(before, payload):
    """⏱️ `change-lighting-gain-schedule{id,newScheduleId}` — Re-points one lighting gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_lighting_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_lighting_gain_watts_per_area(before, payload):
    """🔌️ `change-lighting-gain-watts-per-area{id,newWattsPerArea}` — Sets installed power density (W/m²) on one lighting gain, addressed by id."""
    value = payload["newWattsPerArea"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["watts_per_area"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["watts_per_area"] = value
    return after, applied()


def _invert_change_lighting_gain_watts_per_area(before, payload):
    """↩️ The watts_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-watts-per-area", {"id": payload["id"], "newWattsPerArea": item["watts_per_area"]})]


def change_lighting_gain_radiant_fraction(before, payload):
    """🌟️ `change-lighting-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one lighting gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def change_lighting_gain_visible_fraction(before, payload):
    """🔅️ `change-lighting-gain-visible-fraction{id,newVisibleFraction}` — Sets visible fraction on one lighting gain, addressed by id."""
    value = payload["newVisibleFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["visible_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["visible_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_visible_fraction(before, payload):
    """↩️ The visible_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-visible-fraction", {"id": payload["id"], "newVisibleFraction": item["visible_fraction"]})]


def change_lighting_gain_return_air_fraction(before, payload):
    """🎐️ `change-lighting-gain-return-air-fraction{id,newReturnAirFraction}` — Sets return-air fraction on one lighting gain, addressed by id."""
    value = payload["newReturnAirFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["return_air_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["return_air_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_return_air_fraction(before, payload):
    """↩️ The return_air_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-return-air-fraction", {"id": payload["id"], "newReturnAirFraction": item["return_air_fraction"]})]


def create_equipment_gain(before, payload):
    """🖥️ `create-equipment-gain{index,…}` — Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative."""
    rows = before["model"]["equipment"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["equipment"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "watts_per_area": payload["wattsPerArea"], "radiant_fraction": payload["radiantFraction"], "latent_fraction": payload["latentFraction"]})
    return after, applied()


def _invert_create_equipment_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-equipment-gain", {"id": payload["id"]})]


def delete_equipment_gain(before, payload):
    """🧯️ `delete-equipment-gain{id}` — Removes one electric equipment gain."""
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["equipment"] = [row for row in after["model"]["equipment"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_equipment_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["equipment"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-equipment-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "wattsPerArea": item["watts_per_area"], "radiantFraction": item["radiant_fraction"], "latentFraction": item["latent_fraction"]})]


def change_equipment_gain_zone(before, payload):
    """🖨️ `change-equipment-gain-zone{id,newZoneId}` — Re-points one equipment gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_equipment_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_equipment_gain_schedule(before, payload):
    """⌛️ `change-equipment-gain-schedule{id,newScheduleId}` — Re-points one equipment gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_equipment_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_equipment_gain_watts_per_area(before, payload):
    """⚡️ `change-equipment-gain-watts-per-area{id,newWattsPerArea}` — Sets equipment power density (W/m²) on one equipment gain, addressed by id."""
    value = payload["newWattsPerArea"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["watts_per_area"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["watts_per_area"] = value
    return after, applied()


def _invert_change_equipment_gain_watts_per_area(before, payload):
    """↩️ The watts_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-watts-per-area", {"id": payload["id"], "newWattsPerArea": item["watts_per_area"]})]


def change_equipment_gain_radiant_fraction(before, payload):
    """🌠️ `change-equipment-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one equipment gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_equipment_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def change_equipment_gain_latent_fraction(before, payload):
    """💦️ `change-equipment-gain-latent-fraction{id,newLatentFraction}` — Sets latent fraction on one equipment gain, addressed by id."""
    value = payload["newLatentFraction"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["latent_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["latent_fraction"] = value
    return after, applied()


def _invert_change_equipment_gain_latent_fraction(before, payload):
    """↩️ The latent_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-latent-fraction", {"id": payload["id"], "newLatentFraction": item["latent_fraction"]})]


VOCABULARY = {
    "create-material": create_material,
    "delete-material": delete_material,
    "rename-material": rename_material,
    "change-material-thickness": change_material_thickness,
    "change-material-conductivity": change_material_conductivity,
    "change-material-density": change_material_density,
    "change-material-specific-heat": change_material_specific_heat,
    "change-material-thermal-absorptance": change_material_thermal_absorptance,
    "change-material-solar-absorptance": change_material_solar_absorptance,
    "change-material-visible-absorptance": change_material_visible_absorptance,
    "create-construction": create_construction,
    "delete-construction": delete_construction,
    "rename-construction": rename_construction,
    "add-construction-layer": add_construction_layer,
    "remove-construction-layer": remove_construction_layer,
    "reorder-construction-layers": reorder_construction_layers,
    "create-people-gain": create_people_gain,
    "delete-people-gain": delete_people_gain,
    "change-people-gain-zone": change_people_gain_zone,
    "change-people-gain-schedule": change_people_gain_schedule,
    "change-people-gain-activity-schedule": change_people_gain_activity_schedule,
    "change-people-gain-people-per-area": change_people_gain_people_per_area,
    "change-people-gain-sensible-fraction": change_people_gain_sensible_fraction,
    "change-people-gain-latent-fraction": change_people_gain_latent_fraction,
    "change-people-gain-radiant-fraction": change_people_gain_radiant_fraction,
    "create-lighting-gain": create_lighting_gain,
    "delete-lighting-gain": delete_lighting_gain,
    "change-lighting-gain-zone": change_lighting_gain_zone,
    "change-lighting-gain-schedule": change_lighting_gain_schedule,
    "change-lighting-gain-watts-per-area": change_lighting_gain_watts_per_area,
    "change-lighting-gain-radiant-fraction": change_lighting_gain_radiant_fraction,
    "change-lighting-gain-visible-fraction": change_lighting_gain_visible_fraction,
    "change-lighting-gain-return-air-fraction": change_lighting_gain_return_air_fraction,
    "create-equipment-gain": create_equipment_gain,
    "delete-equipment-gain": delete_equipment_gain,
    "change-equipment-gain-zone": change_equipment_gain_zone,
    "change-equipment-gain-schedule": change_equipment_gain_schedule,
    "change-equipment-gain-watts-per-area": change_equipment_gain_watts_per_area,
    "change-equipment-gain-radiant-fraction": change_equipment_gain_radiant_fraction,
    "change-equipment-gain-latent-fraction": change_equipment_gain_latent_fraction,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-material": _invert_create_material,
    "delete-material": _invert_delete_material,
    "rename-material": _invert_rename_material,
    "change-material-thickness": _invert_change_material_thickness,
    "change-material-conductivity": _invert_change_material_conductivity,
    "change-material-density": _invert_change_material_density,
    "change-material-specific-heat": _invert_change_material_specific_heat,
    "change-material-thermal-absorptance": _invert_change_material_thermal_absorptance,
    "change-material-solar-absorptance": _invert_change_material_solar_absorptance,
    "change-material-visible-absorptance": _invert_change_material_visible_absorptance,
    "create-construction": _invert_create_construction,
    "delete-construction": _invert_delete_construction,
    "rename-construction": _invert_rename_construction,
    "add-construction-layer": _invert_add_construction_layer,
    "remove-construction-layer": _invert_remove_construction_layer,
    "reorder-construction-layers": _invert_reorder_construction_layers,
    "create-people-gain": _invert_create_people_gain,
    "delete-people-gain": _invert_delete_people_gain,
    "change-people-gain-zone": _invert_change_people_gain_zone,
    "change-people-gain-schedule": _invert_change_people_gain_schedule,
    "change-people-gain-activity-schedule": _invert_change_people_gain_activity_schedule,
    "change-people-gain-people-per-area": _invert_change_people_gain_people_per_area,
    "change-people-gain-sensible-fraction": _invert_change_people_gain_sensible_fraction,
    "change-people-gain-latent-fraction": _invert_change_people_gain_latent_fraction,
    "change-people-gain-radiant-fraction": _invert_change_people_gain_radiant_fraction,
    "create-lighting-gain": _invert_create_lighting_gain,
    "delete-lighting-gain": _invert_delete_lighting_gain,
    "change-lighting-gain-zone": _invert_change_lighting_gain_zone,
    "change-lighting-gain-schedule": _invert_change_lighting_gain_schedule,
    "change-lighting-gain-watts-per-area": _invert_change_lighting_gain_watts_per_area,
    "change-lighting-gain-radiant-fraction": _invert_change_lighting_gain_radiant_fraction,
    "change-lighting-gain-visible-fraction": _invert_change_lighting_gain_visible_fraction,
    "change-lighting-gain-return-air-fraction": _invert_change_lighting_gain_return_air_fraction,
    "create-equipment-gain": _invert_create_equipment_gain,
    "delete-equipment-gain": _invert_delete_equipment_gain,
    "change-equipment-gain-zone": _invert_change_equipment_gain_zone,
    "change-equipment-gain-schedule": _invert_change_equipment_gain_schedule,
    "change-equipment-gain-watts-per-area": _invert_change_equipment_gain_watts_per_area,
    "change-equipment-gain-radiant-fraction": _invert_change_equipment_gain_radiant_fraction,
    "change-equipment-gain-latent-fraction": _invert_change_equipment_gain_latent_fraction,
}
# endregion 🔖️Vocabulary
