"""🪨️ Fabric and gain kinds: materials, finishes, constructions and the people, lighting and equipment gains."""

from __future__ import annotations

import copy
import importlib.util
import os
import sys

from mutate_energy_model_1_mutation_vocabulary import applied, no_op, rejected, unchanged


def _load(alias, folder):
    """🧩️ Loads one child module of this module from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


gains = _load("mutate_energy_model_1_gain_kinds", "👥️gain-kinds")


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
    **gains.VECTOR_ROOTS,
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
    **gains.VOCABULARY,
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
    **gains.EXTRA_INVERT,
}
# endregion 🔖️Vocabulary
