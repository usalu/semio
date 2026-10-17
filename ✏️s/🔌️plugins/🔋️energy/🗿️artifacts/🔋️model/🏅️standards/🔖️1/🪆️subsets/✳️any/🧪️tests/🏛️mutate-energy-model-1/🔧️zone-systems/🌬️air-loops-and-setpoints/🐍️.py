"""🌬️ Setpoint manager and air loop kinds."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
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
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
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


VOCABULARY = {
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
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
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
}
# endregion 🔖️Vocabulary
