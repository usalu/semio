"""🔧️ Zone system kinds: zone equipment, daylight, sizing, room air, setpoints, air loops, plant loops and outdoor air."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


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
    "create-setpoint-manager-adds-a-scheduled-spm": "shared://🧬️mutations/🎚️setpoint/📌️create/✅️adds",
    "create-setpoint-manager-refuses-a-bad-kind": "shared://🧬️mutations/🎚️setpoint/📌️create/⛔️refuses",
    "delete-setpoint-manager-drops-the-spm": "shared://🧬️mutations/🎚️setpoint/🍄️delete/✅️drops",
    "delete-setpoint-manager-refuses-an-absent-one": "shared://🧬️mutations/🎚️setpoint/🍄️delete/⛔️refuses",
    "rename-setpoint-manager-renames-the-spm": "shared://🧬️mutations/🎚️setpoint/🖇️rename/✅️renames",
    "rename-setpoint-manager-refuses-a-blank-name": "shared://🧬️mutations/🎚️setpoint/🖇️rename/⛔️refuses",
    "replace-setpoint-manager-kind-resets-on-outdoor-air": "shared://🧬️mutations/🎚️setpoint/🔃️replace-kind/✅️resets",
    "replace-setpoint-manager-kind-refuses-stray-limits": "shared://🧬️mutations/🎚️setpoint/🔃️replace-kind/⛔️refuses",
    "change-setpoint-manager-schedule-repoints-the-spm": "shared://🧬️mutations/🎚️setpoint/🎼️change-schedule/✅️repoints",
    "change-setpoint-manager-schedule-refuses-an-absent-one": "shared://🧬️mutations/🎚️setpoint/🎼️change-schedule/⛔️refuses",
    "create-air-loop-adds-a-main-loop": "shared://🧬️mutations/🌬️air-loop/🛞️create/✅️adds-a-main-loop",
    "create-air-loop-refuses-a-jumbled-list": "shared://🧬️mutations/🌬️air-loop/🛞️create/⛔️refuses",
    "delete-air-loop-drops-the-loop": "shared://🧬️mutations/🌬️air-loop/🥀️delete/✅️drops-the-loop",
    "delete-air-loop-refuses-a-served-loop": "shared://🧬️mutations/🌬️air-loop/🥀️delete/⛔️refuses",
    "rename-air-loop-renames-the-loop": "shared://🧬️mutations/🌬️air-loop/📇️rename/✅️renames-the-loop",
    "rename-air-loop-refuses-a-blank-name": "shared://🧬️mutations/🌬️air-loop/📇️rename/⛔️refuses",
    "change-air-loop-supply-node-repoints-supply": "shared://🧬️mutations/🌬️air-loop/↗️change-supply-node/✅️repoints",
    "change-air-loop-supply-node-refuses-node-zero": "shared://🧬️mutations/🌬️air-loop/↗️change-supply-node/⛔️refuses",
    "change-air-loop-return-node-repoints-return": "shared://🧬️mutations/🌬️air-loop/↘️change-return-node/✅️repoints",
    "change-air-loop-return-node-refuses-node-zero": "shared://🧬️mutations/🌬️air-loop/↘️change-return-node/⛔️refuses",
    "change-air-loop-design-supply-air-flow-uprates-the-flow": "shared://🧬️mutations/🌬️air-loop/🍥️change-design-supply-air-flow/✅️uprates",
    "change-air-loop-design-supply-air-flow-refuses-no-flow": "shared://🧬️mutations/🌬️air-loop/🍥️change-design-supply-air-flow/⛔️refuses",
    "add-air-loop-terminal-zone-serves-zone-two": "shared://🧬️mutations/🌬️air-loop/🪺️add-terminal-zone/✅️serves",
    "add-air-loop-terminal-zone-refuses-an-absent-zone": "shared://🧬️mutations/🌬️air-loop/🪺️add-terminal-zone/⛔️refuses",
    "remove-air-loop-terminal-zone-stops-serving-one": "shared://🧬️mutations/🌬️air-loop/🪹️remove-terminal-zone/✅️stops",
    "remove-air-loop-terminal-zone-refuses-an-unserved": "shared://🧬️mutations/🌬️air-loop/🪹️remove-terminal-zone/⛔️refuses",
    "create-plant-loop-adds-a-hot-loop": "shared://🧬️mutations/🏭️plant-loop/⚗️create/✅️adds-a-hot-loop",
    "create-plant-loop-refuses-a-jumbled-list": "shared://🧬️mutations/🏭️plant-loop/⚗️create/⛔️refuses",
    "delete-plant-loop-drops-the-loop": "shared://🧬️mutations/🏭️plant-loop/💣️delete/✅️drops-the-loop",
    "delete-plant-loop-refuses-an-absent-one": "shared://🧬️mutations/🏭️plant-loop/💣️delete/⛔️refuses",
    "rename-plant-loop-renames-the-loop": "shared://🧬️mutations/🏭️plant-loop/📛️rename/✅️renames-the-loop",
    "rename-plant-loop-refuses-a-blank-name": "shared://🧬️mutations/🏭️plant-loop/📛️rename/⛔️refuses",
    "change-plant-loop-type-turns-it-chilled": "shared://🧬️mutations/🏭️plant-loop/♻️change-type/✅️turns",
    "change-plant-loop-type-refuses-an-absent-row": "shared://🧬️mutations/🏭️plant-loop/♻️change-type/⛔️refuses",
    "change-plant-loop-supply-temperature-cools-the-supply": "shared://🧬️mutations/🏭️plant-loop/☕️change-supply-temperature/✅️cools",
    "change-plant-loop-supply-temperature-refuses-a-hot-supply": "shared://🧬️mutations/🏭️plant-loop/☕️change-supply-temperature/⛔️refuses",
    "change-plant-loop-return-temperature-cools-the-return": "shared://🧬️mutations/🏭️plant-loop/🫖️change-return-temperature/✅️cools",
    "change-plant-loop-return-temperature-refuses-a-cold-return": "shared://🧬️mutations/🏭️plant-loop/🫖️change-return-temperature/⛔️refuses",
    "change-plant-loop-design-flow-uprates-the-flow": "shared://🧬️mutations/🏭️plant-loop/🚤️change-design-flow/✅️uprates",
    "change-plant-loop-design-flow-refuses-no-flow": "shared://🧬️mutations/🏭️plant-loop/🚤️change-design-flow/⛔️refuses",
    "add-plant-loop-equipment-names-equipment": "shared://🧬️mutations/🏭️plant-loop/🔩️add-equipment/✅️names",
    "add-plant-loop-equipment-refuses-the-unset-id": "shared://🧬️mutations/🏭️plant-loop/🔩️add-equipment/⛔️refuses",
    "remove-plant-loop-equipment-drops-equipment": "shared://🧬️mutations/🏭️plant-loop/⚙️remove-equipment/✅️drops",
    "remove-plant-loop-equipment-refuses-an-unlisted": "shared://🧬️mutations/🏭️plant-loop/⚙️remove-equipment/⛔️refuses",
    "create-outdoor-air-system-ventilates-the-loop": "shared://🧬️mutations/🍃️outdoor-air/🌲️create/✅️ventilates",
    "create-outdoor-air-system-refuses-an-absent-loop": "shared://🧬️mutations/🍃️outdoor-air/🌲️create/⛔️refuses",
    "delete-outdoor-air-system-drops-the-system": "shared://🧬️mutations/🍃️outdoor-air/🍂️delete/✅️drops",
    "delete-outdoor-air-system-refuses-an-absent-one": "shared://🧬️mutations/🍃️outdoor-air/🍂️delete/⛔️refuses",
    "change-outdoor-air-system-air-loop-moves-to-the-spare": "shared://🧬️mutations/🍃️outdoor-air/⛓️change-air-loop/✅️moves",
    "change-outdoor-air-system-air-loop-refuses-an-absent-loop": "shared://🧬️mutations/🍃️outdoor-air/⛓️change-air-loop/⛔️refuses",
    "change-outdoor-air-system-min-oa-flow-raises-the-minimum": "shared://🧬️mutations/🍃️outdoor-air/🦋️change-min-oa-flow/✅️raises",
    "change-outdoor-air-system-min-oa-flow-refuses-a-negative": "shared://🧬️mutations/🍃️outdoor-air/🦋️change-min-oa-flow/⛔️refuses",
    "change-outdoor-air-system-economizer-enabled-frees-the-cooling": "shared://🧬️mutations/🍃️outdoor-air/💰️change-economizer-enabled/✅️frees",
    "change-outdoor-air-system-economizer-enabled-refuses-an-absent-row": "shared://🧬️mutations/🍃️outdoor-air/💰️change-economizer-enabled/⛔️refuses",
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


def create_setpoint_manager(before, payload):
    """📌️ `create-setpoint-manager` — one new setpoint manager row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["setpoint_managers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["name"] == payload["name"] for entry in before["model"]["setpoint_managers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["kind"] not in ("Scheduled", "OutdoorAirReset", "WarmestZone", "ColdestZone"):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["kind"] != "OutdoorAirReset" and not (payload["lowOutdoorC"] == 0.0 and payload["highOutdoorC"] == 0.0 and payload["lowSetpointC"] == 0.0 and payload["highSetpointC"] == 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["kind"] == "OutdoorAirReset" and payload["highOutdoorC"] <= payload["lowOutdoorC"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])], invariant="outdoor-reset-range")
    if not payload["schedulePresent"] and payload["scheduleId"] != 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["schedulePresent"] and (not any(entry["id"] == payload["scheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["setpoint_managers"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["setpoint_managers"].insert(len(after["model"]["setpoint_managers"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "name": payload["name"], "kind": ({"OutdoorAirReset": {"low_outdoor_c": payload["lowOutdoorC"], "high_outdoor_c": payload["highOutdoorC"], "low_setpoint_c": payload["lowSetpointC"], "high_setpoint_c": payload["highSetpointC"]}} if payload["kind"] == "OutdoorAirReset" else payload["kind"]), "schedule_id": (payload["scheduleId"] if payload["schedulePresent"] else None)})
    return after, applied()


def _invert_create_setpoint_manager(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-setpoint-manager", {"id": payload["id"]})]


def delete_setpoint_manager(before, payload):
    """🍄️ `delete-setpoint-manager` — drops the addressed setpoint manager row."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["setpoint_managers"] = [entry for entry in after["model"]["setpoint_managers"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_setpoint_manager(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["setpoint_managers"]) if entry["id"] == payload["id"])
    return [("create-setpoint-manager", {"id": item["id"], "name": item["name"], "kind": ("OutdoorAirReset" if isinstance(item["kind"], dict) else item["kind"]), "lowOutdoorC": (item["kind"]["OutdoorAirReset"]["low_outdoor_c"] if isinstance(item["kind"], dict) else 0.0), "highOutdoorC": (item["kind"]["OutdoorAirReset"]["high_outdoor_c"] if isinstance(item["kind"], dict) else 0.0), "lowSetpointC": (item["kind"]["OutdoorAirReset"]["low_setpoint_c"] if isinstance(item["kind"], dict) else 0.0), "highSetpointC": (item["kind"]["OutdoorAirReset"]["high_setpoint_c"] if isinstance(item["kind"], dict) else 0.0), "schedulePresent": item["schedule_id"] is not None, "scheduleId": (item["schedule_id"] if item["schedule_id"] is not None else 0), "index": index})]


def rename_setpoint_manager(before, payload):
    """🖇️ `rename-setpoint-manager` — the name of one setpoint manager row."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newName"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["id"] != payload["id"] and entry["name"] == payload["newName"] for entry in before["model"]["setpoint_managers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == payload["newName"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["name"] = payload["newName"]
    return after, applied()


def _invert_rename_setpoint_manager(before, payload):
    """↩️ Its own inverse, with the old name read off BASE."""
    item = next(entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"])
    return [("rename-setpoint-manager", {"id": payload["id"], "newName": item["name"]})]


def replace_setpoint_manager_kind(before, payload):
    """🔀️ `replace-setpoint-manager-kind` — the whole tagged control law of one manager."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newKind"] not in ("Scheduled", "OutdoorAirReset", "WarmestZone", "ColdestZone"):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newKind"] != "OutdoorAirReset" and not (payload["newLowOutdoorC"] == 0.0 and payload["newHighOutdoorC"] == 0.0 and payload["newLowSetpointC"] == 0.0 and payload["newHighSetpointC"] == 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newKind"] == "OutdoorAirReset" and payload["newHighOutdoorC"] <= payload["newLowOutdoorC"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])], invariant="outdoor-reset-range")
    kind = ({"OutdoorAirReset": {"low_outdoor_c": payload["newLowOutdoorC"], "high_outdoor_c": payload["newHighOutdoorC"], "low_setpoint_c": payload["newLowSetpointC"], "high_setpoint_c": payload["newHighSetpointC"]}} if payload["newKind"] == "OutdoorAirReset" else payload["newKind"])
    if item["kind"] == kind:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["kind"] = kind
    return after, applied()


def _invert_replace_setpoint_manager_kind(before, payload):
    """↩️ Its own inverse, with the old control law flattened back off BASE."""
    item = next(entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"])
    old = item["kind"]
    reset = old["OutdoorAirReset"] if isinstance(old, dict) else None
    return [("replace-setpoint-manager-kind", {"id": payload["id"], "newKind": "OutdoorAirReset" if reset is not None else old, "newLowOutdoorC": reset["low_outdoor_c"] if reset is not None else 0.0, "newHighOutdoorC": reset["high_outdoor_c"] if reset is not None else 0.0, "newLowSetpointC": reset["low_setpoint_c"] if reset is not None else 0.0, "newHighSetpointC": reset["high_setpoint_c"] if reset is not None else 0.0})]


def change_setpoint_manager_schedule(before, payload):
    """🎼️ `change-setpoint-manager-schedule` — the optional schedule reference of one manager."""
    item = next((entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newSchedulePresent"] and payload["newScheduleId"] != 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newSchedulePresent"] and (not any(entry["id"] == payload["newScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family])):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newScheduleId"])])
    value = payload["newScheduleId"] if payload["newSchedulePresent"] else None
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["setpoint_managers"]:
        if entry["id"] == payload["id"]:
            entry["schedule_id"] = value
    return after, applied()


def _invert_change_setpoint_manager_schedule(before, payload):
    """↩️ Its own inverse, with the old reference read off BASE."""
    item = next(entry for entry in before["model"]["setpoint_managers"] if entry["id"] == payload["id"])
    old = item["schedule_id"]
    return [("change-setpoint-manager-schedule", {"id": payload["id"], "newSchedulePresent": old is not None, "newScheduleId": old if old is not None else 0})]


def create_air_loop(before, payload):
    """🛞️ `create-air-loop` — one new air loop row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["name"] == payload["name"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["supplyNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["returnNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["designSupplyAirFlowM3S"] != payload["designSupplyAirFlowM3S"] or payload["designSupplyAirFlowM3S"] in (float("inf"), float("-inf")) or payload["designSupplyAirFlowM3S"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(payload["terminalZoneIds"][index] >= payload["terminalZoneIds"][index + 1] for index in range(len(payload["terminalZoneIds"]) - 1)):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])], invariant="terminal-zone-ids-ascending")
    if any(not any(zone["id"] == value for zone in before["model"]["zones"]) for value in payload["terminalZoneIds"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["air_loops"].insert(len(after["model"]["air_loops"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "name": payload["name"], "supply_node_id": payload["supplyNodeId"], "return_node_id": payload["returnNodeId"], "design_supply_air_flow_m3_s": payload["designSupplyAirFlowM3S"], "terminal_zone_ids": payload["terminalZoneIds"]})
    return after, applied()


def _invert_create_air_loop(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-air-loop", {"id": payload["id"]})]


def delete_air_loop(before, payload):
    """🥀️ `delete-air-loop` — drops the addressed air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(system["air_loop_id"] == payload["id"] for system in before["model"]["outdoor_air_systems"]):
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["air_loops"] = [entry for entry in after["model"]["air_loops"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_air_loop(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["air_loops"]) if entry["id"] == payload["id"])
    return [("create-air-loop", {"id": item["id"], "name": item["name"], "supplyNodeId": item["supply_node_id"], "returnNodeId": item["return_node_id"], "designSupplyAirFlowM3S": item["design_supply_air_flow_m3_s"], "terminalZoneIds": item["terminal_zone_ids"], "index": index})]


def rename_air_loop(before, payload):
    """📇️ `rename-air-loop` — the name of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newName"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["id"] != payload["id"] and entry["name"] == payload["newName"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == payload["newName"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["name"] = payload["newName"]
    return after, applied()


def _invert_rename_air_loop(before, payload):
    """↩️ Its own inverse, with the old name read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("rename-air-loop", {"id": payload["id"], "newName": item["name"]})]


def change_air_loop_supply_node(before, payload):
    """↗️ `change-air-loop-supply-node` — the supply node of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newSupplyNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["supply_node_id"] == payload["newSupplyNodeId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["supply_node_id"] = payload["newSupplyNodeId"]
    return after, applied()


def _invert_change_air_loop_supply_node(before, payload):
    """↩️ Its own inverse, with the old supply node read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("change-air-loop-supply-node", {"id": payload["id"], "newSupplyNodeId": item["supply_node_id"]})]


def change_air_loop_return_node(before, payload):
    """↘️ `change-air-loop-return-node` — the return node of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newReturnNodeId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["return_node_id"] == payload["newReturnNodeId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["return_node_id"] = payload["newReturnNodeId"]
    return after, applied()


def _invert_change_air_loop_return_node(before, payload):
    """↩️ Its own inverse, with the old return node read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("change-air-loop-return-node", {"id": payload["id"], "newReturnNodeId": item["return_node_id"]})]


def change_air_loop_design_supply_air_flow(before, payload):
    """🍥️ `change-air-loop-design-supply-air-flow` — the design supply air flow of one air loop row."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newDesignSupplyAirFlowM3S"] != payload["newDesignSupplyAirFlowM3S"] or payload["newDesignSupplyAirFlowM3S"] in (float("inf"), float("-inf")) or payload["newDesignSupplyAirFlowM3S"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_supply_air_flow_m3_s"] == payload["newDesignSupplyAirFlowM3S"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["design_supply_air_flow_m3_s"] = payload["newDesignSupplyAirFlowM3S"]
    return after, applied()


def _invert_change_air_loop_design_supply_air_flow(before, payload):
    """↩️ Its own inverse, with the old design supply air flow read off BASE."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("change-air-loop-design-supply-air-flow", {"id": payload["id"], "newDesignSupplyAirFlowM3S": item["design_supply_air_flow_m3_s"]})]


def add_air_loop_terminal_zone(before, payload):
    """🪺️ `add-air-loop-terminal-zone` — one terminal zone id into the ascending membership list."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["zoneId"] in item["terminal_zone_ids"]:
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["zoneId"])])
    if payload.get("index") is not None and payload["index"] > len(item["terminal_zone_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            members = entry["terminal_zone_ids"]
            position = next((index for index, value in enumerate(members) if value > payload["zoneId"]), len(members))
            if payload.get("index") is not None:
                position = payload["index"]
            members.insert(position, payload["zoneId"])
    return after, applied()


def _invert_add_air_loop_terminal_zone(before, payload):
    """↩️ The undo of an add is the removal of exactly that member."""
    return [("remove-air-loop-terminal-zone", {"id": payload["id"], "zoneId": payload["zoneId"]})]


def remove_air_loop_terminal_zone(before, payload):
    """🪹️ `remove-air-loop-terminal-zone` — one terminal zone id out of the membership list."""
    item = next((entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["zoneId"] not in item["terminal_zone_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["air_loops"]:
        if entry["id"] == payload["id"]:
            entry["terminal_zone_ids"] = [value for value in entry["terminal_zone_ids"] if value != payload["zoneId"]]
    return after, applied()


def _invert_remove_air_loop_terminal_zone(before, payload):
    """↩️ The undo of a removal puts the member back at its ascending position."""
    item = next(entry for entry in before["model"]["air_loops"] if entry["id"] == payload["id"])
    return [("add-air-loop-terminal-zone", {"id": payload["id"], "zoneId": payload["zoneId"], "index": item["terminal_zone_ids"].index(payload["zoneId"])})]


def create_plant_loop(before, payload):
    """⚗️ `create-plant-loop` — one new plant loop row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["plant_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["name"] == payload["name"] for entry in before["model"]["plant_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["supplyTemperatureC"] != payload["supplyTemperatureC"] or payload["supplyTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["supplyTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["returnTemperatureC"] != payload["returnTemperatureC"] or payload["returnTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["returnTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["designFlowKgS"] != payload["designFlowKgS"] or payload["designFlowKgS"] in (float("inf"), float("-inf")) or payload["designFlowKgS"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(payload["equipmentIds"][index] >= payload["equipmentIds"][index + 1] for index in range(len(payload["equipmentIds"]) - 1)):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])], invariant="equipment-ids-ascending")
    if any(value == 0 for value in payload["equipmentIds"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["plant_loops"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["plant_loops"].insert(len(after["model"]["plant_loops"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "name": payload["name"], "loop_type": payload["loopType"], "supply_temperature_c": payload["supplyTemperatureC"], "return_temperature_c": payload["returnTemperatureC"], "design_flow_kg_s": payload["designFlowKgS"], "equipment_ids": payload["equipmentIds"]})
    return after, applied()


def _invert_create_plant_loop(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-plant-loop", {"id": payload["id"]})]


def delete_plant_loop(before, payload):
    """💣️ `delete-plant-loop` — drops the addressed plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["plant_loops"] = [entry for entry in after["model"]["plant_loops"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_plant_loop(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["plant_loops"]) if entry["id"] == payload["id"])
    return [("create-plant-loop", {"id": item["id"], "name": item["name"], "loopType": item["loop_type"], "supplyTemperatureC": item["supply_temperature_c"], "returnTemperatureC": item["return_temperature_c"], "designFlowKgS": item["design_flow_kg_s"], "equipmentIds": item["equipment_ids"], "index": index})]


def rename_plant_loop(before, payload):
    """📛️ `rename-plant-loop` — the name of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newName"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry["id"] != payload["id"] and entry["name"] == payload["newName"] for entry in before["model"]["plant_loops"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == payload["newName"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["name"] = payload["newName"]
    return after, applied()


def _invert_rename_plant_loop(before, payload):
    """↩️ Its own inverse, with the old name read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("rename-plant-loop", {"id": payload["id"], "newName": item["name"]})]


def change_plant_loop_type(before, payload):
    """♻️ `change-plant-loop-type` — the loop type of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["loop_type"] == payload["newLoopType"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["loop_type"] = payload["newLoopType"]
    return after, applied()


def _invert_change_plant_loop_type(before, payload):
    """↩️ Its own inverse, with the old loop type read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-type", {"id": payload["id"], "newLoopType": item["loop_type"]})]


def change_plant_loop_supply_temperature(before, payload):
    """☕️ `change-plant-loop-supply-temperature` — the supply temperature of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newSupplyTemperatureC"] != payload["newSupplyTemperatureC"] or payload["newSupplyTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newSupplyTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["supply_temperature_c"] == payload["newSupplyTemperatureC"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["supply_temperature_c"] = payload["newSupplyTemperatureC"]
    return after, applied()


def _invert_change_plant_loop_supply_temperature(before, payload):
    """↩️ Its own inverse, with the old supply temperature read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-supply-temperature", {"id": payload["id"], "newSupplyTemperatureC": item["supply_temperature_c"]})]


def change_plant_loop_return_temperature(before, payload):
    """🧫️ `change-plant-loop-return-temperature` — the return temperature of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newReturnTemperatureC"] != payload["newReturnTemperatureC"] or payload["newReturnTemperatureC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newReturnTemperatureC"] <= 300.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["return_temperature_c"] == payload["newReturnTemperatureC"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["return_temperature_c"] = payload["newReturnTemperatureC"]
    return after, applied()


def _invert_change_plant_loop_return_temperature(before, payload):
    """↩️ Its own inverse, with the old return temperature read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-return-temperature", {"id": payload["id"], "newReturnTemperatureC": item["return_temperature_c"]})]


def change_plant_loop_design_flow(before, payload):
    """🚤️ `change-plant-loop-design-flow` — the design mass flow of one plant loop row."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newDesignFlowKgS"] != payload["newDesignFlowKgS"] or payload["newDesignFlowKgS"] in (float("inf"), float("-inf")) or payload["newDesignFlowKgS"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_flow_kg_s"] == payload["newDesignFlowKgS"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["design_flow_kg_s"] = payload["newDesignFlowKgS"]
    return after, applied()


def _invert_change_plant_loop_design_flow(before, payload):
    """↩️ Its own inverse, with the old design mass flow read off BASE."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("change-plant-loop-design-flow", {"id": payload["id"], "newDesignFlowKgS": item["design_flow_kg_s"]})]


def add_plant_loop_equipment(before, payload):
    """🔩️ `add-plant-loop-equipment` — one plant equipment id into the ascending membership list."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["equipmentId"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["equipmentId"])])
    if payload["equipmentId"] in item["equipment_ids"]:
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["equipmentId"])])
    if payload.get("index") is not None and payload["index"] > len(item["equipment_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            members = entry["equipment_ids"]
            position = next((index for index, value in enumerate(members) if value > payload["equipmentId"]), len(members))
            if payload.get("index") is not None:
                position = payload["index"]
            members.insert(position, payload["equipmentId"])
    return after, applied()


def _invert_add_plant_loop_equipment(before, payload):
    """↩️ The undo of an add is the removal of exactly that member."""
    return [("remove-plant-loop-equipment", {"id": payload["id"], "equipmentId": payload["equipmentId"]})]


def remove_plant_loop_equipment(before, payload):
    """⚙️ `remove-plant-loop-equipment` — one plant equipment id out of the membership list."""
    item = next((entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["equipmentId"] not in item["equipment_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["equipmentId"])])
    after = copy.deepcopy(before)
    for entry in after["model"]["plant_loops"]:
        if entry["id"] == payload["id"]:
            entry["equipment_ids"] = [value for value in entry["equipment_ids"] if value != payload["equipmentId"]]
    return after, applied()


def _invert_remove_plant_loop_equipment(before, payload):
    """↩️ The undo of a removal puts the member back at its ascending position."""
    item = next(entry for entry in before["model"]["plant_loops"] if entry["id"] == payload["id"])
    return [("add-plant-loop-equipment", {"id": payload["id"], "equipmentId": payload["equipmentId"], "index": item["equipment_ids"].index(payload["equipmentId"])})]


def create_outdoor_air_system(before, payload):
    """🌲️ `create-outdoor-air-system` — one new outdoor air system row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["outdoor_air_systems"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["airLoopId"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["airLoopId"])])
    if payload["minOaFlowM3S"] != payload["minOaFlowM3S"] or payload["minOaFlowM3S"] in (float("inf"), float("-inf")) or payload["minOaFlowM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["outdoor_air_systems"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["outdoor_air_systems"].insert(len(after["model"]["outdoor_air_systems"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "air_loop_id": payload["airLoopId"], "min_oa_flow_m3_s": payload["minOaFlowM3S"], "economizer_enabled": payload["economizerEnabled"]})
    return after, applied()


def _invert_create_outdoor_air_system(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-outdoor-air-system", {"id": payload["id"]})]


def delete_outdoor_air_system(before, payload):
    """🍂️ `delete-outdoor-air-system` — drops the addressed outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["outdoor_air_systems"] = [entry for entry in after["model"]["outdoor_air_systems"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_outdoor_air_system(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["outdoor_air_systems"]) if entry["id"] == payload["id"])
    return [("create-outdoor-air-system", {"id": item["id"], "airLoopId": item["air_loop_id"], "minOaFlowM3S": item["min_oa_flow_m3_s"], "economizerEnabled": item["economizer_enabled"], "index": index})]


def change_outdoor_air_system_air_loop(before, payload):
    """⛓️ `change-outdoor-air-system-air-loop` — the air loop of one outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newAirLoopId"] for entry in before["model"]["air_loops"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newAirLoopId"])])
    if item["air_loop_id"] == payload["newAirLoopId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["outdoor_air_systems"]:
        if entry["id"] == payload["id"]:
            entry["air_loop_id"] = payload["newAirLoopId"]
    return after, applied()


def _invert_change_outdoor_air_system_air_loop(before, payload):
    """↩️ Its own inverse, with the old air loop read off BASE."""
    item = next(entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"])
    return [("change-outdoor-air-system-air-loop", {"id": payload["id"], "newAirLoopId": item["air_loop_id"]})]


def change_outdoor_air_system_min_oa_flow(before, payload):
    """🦋️ `change-outdoor-air-system-min-oa-flow` — the minimum outdoor air flow of one outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newMinOaFlowM3S"] != payload["newMinOaFlowM3S"] or payload["newMinOaFlowM3S"] in (float("inf"), float("-inf")) or payload["newMinOaFlowM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["min_oa_flow_m3_s"] == payload["newMinOaFlowM3S"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["outdoor_air_systems"]:
        if entry["id"] == payload["id"]:
            entry["min_oa_flow_m3_s"] = payload["newMinOaFlowM3S"]
    return after, applied()


def _invert_change_outdoor_air_system_min_oa_flow(before, payload):
    """↩️ Its own inverse, with the old minimum outdoor air flow read off BASE."""
    item = next(entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"])
    return [("change-outdoor-air-system-min-oa-flow", {"id": payload["id"], "newMinOaFlowM3S": item["min_oa_flow_m3_s"]})]


def change_outdoor_air_system_economizer_enabled(before, payload):
    """💰️ `change-outdoor-air-system-economizer-enabled` — the economizer setting of one outdoor air system row."""
    item = next((entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    if item["economizer_enabled"] == payload["newEconomizerEnabled"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["outdoor_air_systems"]:
        if entry["id"] == payload["id"]:
            entry["economizer_enabled"] = payload["newEconomizerEnabled"]
    return after, applied()


def _invert_change_outdoor_air_system_economizer_enabled(before, payload):
    """↩️ Its own inverse, with the old economizer setting read off BASE."""
    item = next(entry for entry in before["model"]["outdoor_air_systems"] if entry["id"] == payload["id"])
    return [("change-outdoor-air-system-economizer-enabled", {"id": payload["id"], "newEconomizerEnabled": item["economizer_enabled"]})]


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
    "create-setpoint-manager": create_setpoint_manager,
    "delete-setpoint-manager": delete_setpoint_manager,
    "rename-setpoint-manager": rename_setpoint_manager,
    "replace-setpoint-manager-kind": replace_setpoint_manager_kind,
    "change-setpoint-manager-schedule": change_setpoint_manager_schedule,
    "create-air-loop": create_air_loop,
    "delete-air-loop": delete_air_loop,
    "rename-air-loop": rename_air_loop,
    "change-air-loop-supply-node": change_air_loop_supply_node,
    "change-air-loop-return-node": change_air_loop_return_node,
    "change-air-loop-design-supply-air-flow": change_air_loop_design_supply_air_flow,
    "add-air-loop-terminal-zone": add_air_loop_terminal_zone,
    "remove-air-loop-terminal-zone": remove_air_loop_terminal_zone,
    "create-plant-loop": create_plant_loop,
    "delete-plant-loop": delete_plant_loop,
    "rename-plant-loop": rename_plant_loop,
    "change-plant-loop-type": change_plant_loop_type,
    "change-plant-loop-supply-temperature": change_plant_loop_supply_temperature,
    "change-plant-loop-return-temperature": change_plant_loop_return_temperature,
    "change-plant-loop-design-flow": change_plant_loop_design_flow,
    "add-plant-loop-equipment": add_plant_loop_equipment,
    "remove-plant-loop-equipment": remove_plant_loop_equipment,
    "create-outdoor-air-system": create_outdoor_air_system,
    "delete-outdoor-air-system": delete_outdoor_air_system,
    "change-outdoor-air-system-air-loop": change_outdoor_air_system_air_loop,
    "change-outdoor-air-system-min-oa-flow": change_outdoor_air_system_min_oa_flow,
    "change-outdoor-air-system-economizer-enabled": change_outdoor_air_system_economizer_enabled,
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
    "create-setpoint-manager": _invert_create_setpoint_manager,
    "delete-setpoint-manager": _invert_delete_setpoint_manager,
    "rename-setpoint-manager": _invert_rename_setpoint_manager,
    "replace-setpoint-manager-kind": _invert_replace_setpoint_manager_kind,
    "change-setpoint-manager-schedule": _invert_change_setpoint_manager_schedule,
    "create-air-loop": _invert_create_air_loop,
    "delete-air-loop": _invert_delete_air_loop,
    "rename-air-loop": _invert_rename_air_loop,
    "change-air-loop-supply-node": _invert_change_air_loop_supply_node,
    "change-air-loop-return-node": _invert_change_air_loop_return_node,
    "change-air-loop-design-supply-air-flow": _invert_change_air_loop_design_supply_air_flow,
    "add-air-loop-terminal-zone": _invert_add_air_loop_terminal_zone,
    "remove-air-loop-terminal-zone": _invert_remove_air_loop_terminal_zone,
    "create-plant-loop": _invert_create_plant_loop,
    "delete-plant-loop": _invert_delete_plant_loop,
    "rename-plant-loop": _invert_rename_plant_loop,
    "change-plant-loop-type": _invert_change_plant_loop_type,
    "change-plant-loop-supply-temperature": _invert_change_plant_loop_supply_temperature,
    "change-plant-loop-return-temperature": _invert_change_plant_loop_return_temperature,
    "change-plant-loop-design-flow": _invert_change_plant_loop_design_flow,
    "add-plant-loop-equipment": _invert_add_plant_loop_equipment,
    "remove-plant-loop-equipment": _invert_remove_plant_loop_equipment,
    "create-outdoor-air-system": _invert_create_outdoor_air_system,
    "delete-outdoor-air-system": _invert_delete_outdoor_air_system,
    "change-outdoor-air-system-air-loop": _invert_change_outdoor_air_system_air_loop,
    "change-outdoor-air-system-min-oa-flow": _invert_change_outdoor_air_system_min_oa_flow,
    "change-outdoor-air-system-economizer-enabled": _invert_change_outdoor_air_system_economizer_enabled,
}
# endregion 🔖️Vocabulary
