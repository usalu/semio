"""🏭️ Plant loop and outdoor air system kinds."""

from __future__ import annotations

import copy

from mutate_energy_model_1_mutation_vocabulary import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
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
