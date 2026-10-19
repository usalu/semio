"""🔧️ Zone system kinds: zone equipment, daylight, sizing, room air, setpoints, air loops, plant loops and outdoor air."""

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


c1 = _load("mutate_energy_model_1_zone_systems_air_handling", "🌬️air-loops-and-setpoints")
c2 = _load("mutate_energy_model_1_zone_systems_plant_oa", "🏭️plant-loops-and-outdoor-air")


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-zone-equipment-adds-a-baseboard": "shared://🧬️mutations/🔧️zone-equipment/🛠️create/✅️adds",
    "create-zone-equipment-refuses-rank-zero": "shared://🧬️mutations/🔧️zone-equipment/🛠️create/⛔️refuses",
    "delete-zone-equipment-drops-the-baseboard": "shared://🧬️mutations/🔧️zone-equipment/🗑️delete/✅️drops",
    "delete-zone-equipment-refuses-an-absent-one": "shared://🧬️mutations/🔧️zone-equipment/🗑️delete/⛔️refuses",
    "change-zone-equipment-zone-moves-to-zone-two": "shared://🧬️mutations/🔧️zone-equipment/🏬️change-zone/✅️moves",
    "change-zone-equipment-zone-refuses-an-absent-zone": "shared://🧬️mutations/🔧️zone-equipment/🏬️change-zone/⛔️refuses",
    "change-zone-equipment-type-swaps-to-a-fan-coil": "shared://🧬️mutations/🔧️zone-equipment/🔧️change-type/✅️swaps",
    "change-zone-equipment-type-refuses-an-absent-row": "shared://🧬️mutations/🔧️zone-equipment/🔧️change-type/⛔️refuses",
    "change-zone-equipment-priority-demotes-it": "shared://🧬️mutations/🔧️zone-equipment/🎗️change-priority/✅️demotes-it",
    "change-zone-equipment-priority-refuses-rank-zero": "shared://🧬️mutations/🔧️zone-equipment/🎗️change-priority/⛔️refuses",
    "change-zone-equipment-heating-capacity-uprates-heating": "shared://🧬️mutations/🔧️zone-equipment/🧇️change-heating-capacity/✅️uprates",
    "change-zone-equipment-heating-capacity-refuses-a-negative": "shared://🧬️mutations/🔧️zone-equipment/🧇️change-heating-capacity/⛔️refuses",
    "change-zone-equipment-cooling-capacity-uprates-cooling": "shared://🧬️mutations/🔧️zone-equipment/🍧️change-cooling-capacity/✅️uprates",
    "change-zone-equipment-cooling-capacity-refuses-a-negative": "shared://🧬️mutations/🔧️zone-equipment/🍧️change-cooling-capacity/⛔️refuses",
    "create-daylight-zone-lights-zone-one": "shared://🧬️mutations/🌞️daylight-zone/🔭️create/✅️lights",
    "create-daylight-zone-refuses-a-bad-tau": "shared://🧬️mutations/🌞️daylight-zone/🔭️create/⛔️refuses",
    "delete-daylight-zone-darkens-the-zone": "shared://🧬️mutations/🌞️daylight-zone/🌗️delete/✅️darkens",
    "delete-daylight-zone-refuses-an-absent-one": "shared://🧬️mutations/🌞️daylight-zone/🌗️delete/⛔️refuses",
    "change-daylight-zone-zone-moves-to-zone-two": "shared://🧬️mutations/🌞️daylight-zone/🏫️change-zone/✅️moves",
    "change-daylight-zone-zone-refuses-an-absent-zone": "shared://🧬️mutations/🌞️daylight-zone/🏫️change-zone/⛔️refuses",
    "change-daylight-zone-illuminance-target-dims-the-target": "shared://🧬️mutations/🌞️daylight-zone/🪔️change-illuminance-target/✅️dims",
    "change-daylight-zone-illuminance-target-refuses-a-dark-target": "shared://🧬️mutations/🌞️daylight-zone/🪔️change-illuminance-target/⛔️refuses",
    "change-daylight-zone-glare-limit-tightens-glare": "shared://🧬️mutations/🌞️daylight-zone/🕶️change-glare-limit/✅️tightens",
    "change-daylight-zone-glare-limit-refuses-a-negative": "shared://🧬️mutations/🌞️daylight-zone/🕶️change-glare-limit/⛔️refuses",
    "change-daylight-zone-window-transmittance-darkens-the-glass": "shared://🧬️mutations/🌞️daylight-zone/🥃️change-window-transmittance/✅️darkens",
    "change-daylight-zone-window-transmittance-refuses-a-bad-tau": "shared://🧬️mutations/🌞️daylight-zone/🥃️change-window-transmittance/⛔️refuses",
    "create-sizing-object-sizes-zone-one": "shared://🧬️mutations/📏️sizing/📶️create/✅️sizes",
    "create-sizing-object-refuses-an-absent-zone": "shared://🧬️mutations/📏️sizing/📶️create/⛔️refuses",
    "delete-sizing-object-drops-the-sizing": "shared://🧬️mutations/📏️sizing/🪒️delete/✅️drops",
    "delete-sizing-object-refuses-an-absent-one": "shared://🧬️mutations/📏️sizing/🪒️delete/⛔️refuses",
    "change-sizing-object-zone-moves-to-zone-two": "shared://🧬️mutations/📏️sizing/🏨️change-zone/✅️moves",
    "change-sizing-object-zone-refuses-an-absent-zone": "shared://🧬️mutations/📏️sizing/🏨️change-zone/⛔️refuses",
    "change-sizing-object-sizing-type-sizes-for-cooling": "shared://🧬️mutations/📏️sizing/🧾️change-sizing-type/✅️sizes",
    "change-sizing-object-sizing-type-refuses-an-absent-row": "shared://🧬️mutations/📏️sizing/🧾️change-sizing-type/⛔️refuses",
    "change-sizing-object-design-day-type-reads-a-hot-day": "shared://🧬️mutations/📏️sizing/🌥️change-design-day-type/✅️reads",
    "change-sizing-object-design-day-type-refuses-an-absent-row": "shared://🧬️mutations/📏️sizing/🌥️change-design-day-type/⛔️refuses",
    "create-room-air-model-assignment-stratifies-zone-two": "shared://🧬️mutations/🛏️room-air/🛏️create-assignment/✅️stratifies",
    "create-room-air-model-assignment-refuses-a-second-one": "shared://🧬️mutations/🛏️room-air/🛏️create-assignment/⛔️refuses",
    "delete-room-air-model-assignment-falls-back": "shared://🧬️mutations/🛏️room-air/🧺️delete-assignment/✅️falls-back",
    "delete-room-air-model-assignment-refuses-an-absent-one": "shared://🧬️mutations/🛏️room-air/🧺️delete-assignment/⛔️refuses",
    "change-room-air-model-stratifies-the-air": "shared://🧬️mutations/🛏️room-air/🪭️change/✅️stratifies",
    "change-room-air-model-refuses-an-absent-row": "shared://🧬️mutations/🛏️room-air/🪭️change/⛔️refuses",
    **c1.VECTOR_ROOTS,
    **c2.VECTOR_ROOTS,
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_zone_equipment(before, payload):
    """🛠️ `create-zone-equipment` — one new zone equipment row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["zone_equipment"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["priority"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["heatingCapacityW"] != payload["heatingCapacityW"] or payload["heatingCapacityW"] in (float("inf"), float("-inf")) or payload["heatingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["coolingCapacityW"] != payload["coolingCapacityW"] or payload["coolingCapacityW"] in (float("inf"), float("-inf")) or payload["coolingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["zone_equipment"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["zone_equipment"].insert(len(after["model"]["zone_equipment"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "equipment_type": payload["equipmentType"], "priority": payload["priority"], "heating_capacity_w": payload["heatingCapacityW"], "cooling_capacity_w": payload["coolingCapacityW"]})
    return after, applied()


def _invert_create_zone_equipment(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-zone-equipment", {"id": payload["id"]})]


def delete_zone_equipment(before, payload):
    """🗑️ `delete-zone-equipment` — drops the addressed zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["zone_equipment"] = [entry for entry in after["model"]["zone_equipment"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_zone_equipment(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["zone_equipment"]) if entry["id"] == payload["id"])
    return [("create-zone-equipment", {"id": item["id"], "zoneId": item["zone_id"], "equipmentType": item["equipment_type"], "priority": item["priority"], "heatingCapacityW": item["heating_capacity_w"], "coolingCapacityW": item["cooling_capacity_w"], "index": index})]


def change_zone_equipment_zone(before, payload):
    """🏬️ `change-zone-equipment-zone` — the zone of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_zone_equipment_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_zone_equipment_type(before, payload):
    """🔧️ `change-zone-equipment-type` — the equipment type of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["equipment_type"] == payload["newEquipmentType"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["equipment_type"] = payload["newEquipmentType"]
    return after, applied()


def _invert_change_zone_equipment_type(before, payload):
    """↩️ Its own inverse, with the old equipment type read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-type", {"id": payload["id"], "newEquipmentType": item["equipment_type"]})]


def change_zone_equipment_priority(before, payload):
    """🎗️ `change-zone-equipment-priority` — the priority of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newPriority"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["priority"] == payload["newPriority"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["priority"] = payload["newPriority"]
    return after, applied()


def _invert_change_zone_equipment_priority(before, payload):
    """↩️ Its own inverse, with the old priority read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-priority", {"id": payload["id"], "newPriority": item["priority"]})]


def change_zone_equipment_heating_capacity(before, payload):
    """🧇️ `change-zone-equipment-heating-capacity` — the heating capacity of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newHeatingCapacityW"] != payload["newHeatingCapacityW"] or payload["newHeatingCapacityW"] in (float("inf"), float("-inf")) or payload["newHeatingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["heating_capacity_w"] == payload["newHeatingCapacityW"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["heating_capacity_w"] = payload["newHeatingCapacityW"]
    return after, applied()


def _invert_change_zone_equipment_heating_capacity(before, payload):
    """↩️ Its own inverse, with the old heating capacity read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-heating-capacity", {"id": payload["id"], "newHeatingCapacityW": item["heating_capacity_w"]})]


def change_zone_equipment_cooling_capacity(before, payload):
    """🍧️ `change-zone-equipment-cooling-capacity` — the cooling capacity of one zone equipment row."""
    item = next((entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newCoolingCapacityW"] != payload["newCoolingCapacityW"] or payload["newCoolingCapacityW"] in (float("inf"), float("-inf")) or payload["newCoolingCapacityW"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["cooling_capacity_w"] == payload["newCoolingCapacityW"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["zone_equipment"]:
        if entry["id"] == payload["id"]:
            entry["cooling_capacity_w"] = payload["newCoolingCapacityW"]
    return after, applied()


def _invert_change_zone_equipment_cooling_capacity(before, payload):
    """↩️ Its own inverse, with the old cooling capacity read off BASE."""
    item = next(entry for entry in before["model"]["zone_equipment"] if entry["id"] == payload["id"])
    return [("change-zone-equipment-cooling-capacity", {"id": payload["id"], "newCoolingCapacityW": item["cooling_capacity_w"]})]


def create_daylight_zone(before, payload):
    """🔭️ `create-daylight-zone` — one new daylight zone row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["daylight_zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["illuminanceTargetLux"] != payload["illuminanceTargetLux"] or payload["illuminanceTargetLux"] in (float("inf"), float("-inf")) or payload["illuminanceTargetLux"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["glareLimit"] != payload["glareLimit"] or payload["glareLimit"] in (float("inf"), float("-inf")) or payload["glareLimit"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["windowTransmittance"] != payload["windowTransmittance"] or payload["windowTransmittance"] in (float("inf"), float("-inf")) or not 0.0 <= payload["windowTransmittance"] <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["daylight_zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["daylight_zones"].insert(len(after["model"]["daylight_zones"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "illuminance_target_lux": payload["illuminanceTargetLux"], "glare_limit": payload["glareLimit"], "window_transmittance": payload["windowTransmittance"]})
    return after, applied()


def _invert_create_daylight_zone(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-daylight-zone", {"id": payload["id"]})]


def delete_daylight_zone(before, payload):
    """🌗️ `delete-daylight-zone` — drops the addressed daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["daylight_zones"] = [entry for entry in after["model"]["daylight_zones"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_daylight_zone(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["daylight_zones"]) if entry["id"] == payload["id"])
    return [("create-daylight-zone", {"id": item["id"], "zoneId": item["zone_id"], "illuminanceTargetLux": item["illuminance_target_lux"], "glareLimit": item["glare_limit"], "windowTransmittance": item["window_transmittance"], "index": index})]


def change_daylight_zone_zone(before, payload):
    """🏫️ `change-daylight-zone-zone` — the zone of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_daylight_zone_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_daylight_zone_illuminance_target(before, payload):
    """🪔️ `change-daylight-zone-illuminance-target` — the illuminance target of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newIlluminanceTargetLux"] != payload["newIlluminanceTargetLux"] or payload["newIlluminanceTargetLux"] in (float("inf"), float("-inf")) or payload["newIlluminanceTargetLux"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["illuminance_target_lux"] == payload["newIlluminanceTargetLux"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["illuminance_target_lux"] = payload["newIlluminanceTargetLux"]
    return after, applied()


def _invert_change_daylight_zone_illuminance_target(before, payload):
    """↩️ Its own inverse, with the old illuminance target read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-illuminance-target", {"id": payload["id"], "newIlluminanceTargetLux": item["illuminance_target_lux"]})]


def change_daylight_zone_glare_limit(before, payload):
    """🕶️ `change-daylight-zone-glare-limit` — the glare limit of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newGlareLimit"] != payload["newGlareLimit"] or payload["newGlareLimit"] in (float("inf"), float("-inf")) or payload["newGlareLimit"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["glare_limit"] == payload["newGlareLimit"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["glare_limit"] = payload["newGlareLimit"]
    return after, applied()


def _invert_change_daylight_zone_glare_limit(before, payload):
    """↩️ Its own inverse, with the old glare limit read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-glare-limit", {"id": payload["id"], "newGlareLimit": item["glare_limit"]})]


def change_daylight_zone_window_transmittance(before, payload):
    """🥃️ `change-daylight-zone-window-transmittance` — the window transmittance of one daylight zone row."""
    item = next((entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newWindowTransmittance"] != payload["newWindowTransmittance"] or payload["newWindowTransmittance"] in (float("inf"), float("-inf")) or not 0.0 <= payload["newWindowTransmittance"] <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["window_transmittance"] == payload["newWindowTransmittance"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["daylight_zones"]:
        if entry["id"] == payload["id"]:
            entry["window_transmittance"] = payload["newWindowTransmittance"]
    return after, applied()


def _invert_change_daylight_zone_window_transmittance(before, payload):
    """↩️ Its own inverse, with the old window transmittance read off BASE."""
    item = next(entry for entry in before["model"]["daylight_zones"] if entry["id"] == payload["id"])
    return [("change-daylight-zone-window-transmittance", {"id": payload["id"], "newWindowTransmittance": item["window_transmittance"]})]


def create_sizing_object(before, payload):
    """📶️ `create-sizing-object` — one new sizing object row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["sizing_objects"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["sizing_objects"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["sizing_objects"].insert(len(after["model"]["sizing_objects"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "sizing_type": payload["sizingType"], "design_day_type": payload["designDayType"]})
    return after, applied()


def _invert_create_sizing_object(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-sizing-object", {"id": payload["id"]})]


def delete_sizing_object(before, payload):
    """🪒️ `delete-sizing-object` — drops the addressed sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["sizing_objects"] = [entry for entry in after["model"]["sizing_objects"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_sizing_object(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["sizing_objects"]) if entry["id"] == payload["id"])
    return [("create-sizing-object", {"id": item["id"], "zoneId": item["zone_id"], "sizingType": item["sizing_type"], "designDayType": item["design_day_type"], "index": index})]


def change_sizing_object_zone(before, payload):
    """🏨️ `change-sizing-object-zone` — the zone of one sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["sizing_objects"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_sizing_object_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"])
    return [("change-sizing-object-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_sizing_object_sizing_type(before, payload):
    """🧾️ `change-sizing-object-sizing-type` — the sizing type of one sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["sizing_type"] == payload["newSizingType"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["sizing_objects"]:
        if entry["id"] == payload["id"]:
            entry["sizing_type"] = payload["newSizingType"]
    return after, applied()


def _invert_change_sizing_object_sizing_type(before, payload):
    """↩️ Its own inverse, with the old sizing type read off BASE."""
    item = next(entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"])
    return [("change-sizing-object-sizing-type", {"id": payload["id"], "newSizingType": item["sizing_type"]})]


def change_sizing_object_design_day_type(before, payload):
    """🌥️ `change-sizing-object-design-day-type` — the design day type of one sizing object row."""
    item = next((entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["design_day_type"] == payload["newDesignDayType"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["sizing_objects"]:
        if entry["id"] == payload["id"]:
            entry["design_day_type"] = payload["newDesignDayType"]
    return after, applied()


def _invert_change_sizing_object_design_day_type(before, payload):
    """↩️ Its own inverse, with the old design day type read off BASE."""
    item = next(entry for entry in before["model"]["sizing_objects"] if entry["id"] == payload["id"])
    return [("change-sizing-object-design-day-type", {"id": payload["id"], "newDesignDayType": item["design_day_type"]})]


def create_room_air_model_assignment(before, payload):
    """🛏️ `create-room-air-model-assignment` — one new room air model assignment row under a caller-minted id."""
    if any(entry["zone_id"] == payload["zoneId"] for entry in before["model"]["room_air_models"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["zoneId"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["room_air_models"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    after["model"]["room_air_models"].insert(len(after["model"]["room_air_models"]) if payload.get("index") is None else payload["index"], {"zone_id": payload["zoneId"], "model": payload["model"]})
    return after, applied()


def _invert_create_room_air_model_assignment(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-room-air-model-assignment", {"zoneId": payload["zoneId"]})]


def delete_room_air_model_assignment(before, payload):
    """🧺️ `delete-room-air-model-assignment` — drops the addressed room air model assignment row."""
    item = next((entry for entry in before["model"]["room_air_models"] if entry["zone_id"] == payload["zoneId"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])

    after = copy.deepcopy(before)
    after["model"]["room_air_models"] = [entry for entry in after["model"]["room_air_models"] if entry["zone_id"] != payload["zoneId"]]
    return after, applied()


def _invert_delete_room_air_model_assignment(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["room_air_models"]) if entry["zone_id"] == payload["zoneId"])
    return [("create-room-air-model-assignment", {"zoneId": item["zone_id"], "model": item["model"], "index": index})]


def change_room_air_model(before, payload):
    """🪭️ `change-room-air-model` — the room air model of one room air model assignment row."""
    item = next((entry for entry in before["model"]["room_air_models"] if entry["zone_id"] == payload["zoneId"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])

    if item["model"] == payload["newModel"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["room_air_models"]:
        if entry["zone_id"] == payload["zoneId"]:
            entry["model"] = payload["newModel"]
    return after, applied()


def _invert_change_room_air_model(before, payload):
    """↩️ Its own inverse, with the old room air model read off BASE."""
    item = next(entry for entry in before["model"]["room_air_models"] if entry["zone_id"] == payload["zoneId"])
    return [("change-room-air-model", {"zoneId": payload["zoneId"], "newModel": item["model"]})]


VOCABULARY = {
    "create-zone-equipment": create_zone_equipment,
    "delete-zone-equipment": delete_zone_equipment,
    "change-zone-equipment-zone": change_zone_equipment_zone,
    "change-zone-equipment-type": change_zone_equipment_type,
    "change-zone-equipment-priority": change_zone_equipment_priority,
    "change-zone-equipment-heating-capacity": change_zone_equipment_heating_capacity,
    "change-zone-equipment-cooling-capacity": change_zone_equipment_cooling_capacity,
    "create-daylight-zone": create_daylight_zone,
    "delete-daylight-zone": delete_daylight_zone,
    "change-daylight-zone-zone": change_daylight_zone_zone,
    "change-daylight-zone-illuminance-target": change_daylight_zone_illuminance_target,
    "change-daylight-zone-glare-limit": change_daylight_zone_glare_limit,
    "change-daylight-zone-window-transmittance": change_daylight_zone_window_transmittance,
    "create-sizing-object": create_sizing_object,
    "delete-sizing-object": delete_sizing_object,
    "change-sizing-object-zone": change_sizing_object_zone,
    "change-sizing-object-sizing-type": change_sizing_object_sizing_type,
    "change-sizing-object-design-day-type": change_sizing_object_design_day_type,
    "create-room-air-model-assignment": create_room_air_model_assignment,
    "delete-room-air-model-assignment": delete_room_air_model_assignment,
    "change-room-air-model": change_room_air_model,
    **c1.VOCABULARY,
    **c2.VOCABULARY,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-zone-equipment": _invert_create_zone_equipment,
    "delete-zone-equipment": _invert_delete_zone_equipment,
    "change-zone-equipment-zone": _invert_change_zone_equipment_zone,
    "change-zone-equipment-type": _invert_change_zone_equipment_type,
    "change-zone-equipment-priority": _invert_change_zone_equipment_priority,
    "change-zone-equipment-heating-capacity": _invert_change_zone_equipment_heating_capacity,
    "change-zone-equipment-cooling-capacity": _invert_change_zone_equipment_cooling_capacity,
    "create-daylight-zone": _invert_create_daylight_zone,
    "delete-daylight-zone": _invert_delete_daylight_zone,
    "change-daylight-zone-zone": _invert_change_daylight_zone_zone,
    "change-daylight-zone-illuminance-target": _invert_change_daylight_zone_illuminance_target,
    "change-daylight-zone-glare-limit": _invert_change_daylight_zone_glare_limit,
    "change-daylight-zone-window-transmittance": _invert_change_daylight_zone_window_transmittance,
    "create-sizing-object": _invert_create_sizing_object,
    "delete-sizing-object": _invert_delete_sizing_object,
    "change-sizing-object-zone": _invert_change_sizing_object_zone,
    "change-sizing-object-sizing-type": _invert_change_sizing_object_sizing_type,
    "change-sizing-object-design-day-type": _invert_change_sizing_object_design_day_type,
    "create-room-air-model-assignment": _invert_create_room_air_model_assignment,
    "delete-room-air-model-assignment": _invert_delete_room_air_model_assignment,
    "change-room-air-model": _invert_change_room_air_model,
    **c1.EXTRA_INVERT,
    **c2.EXTRA_INVERT,
}
# endregion 🔖️Vocabulary
