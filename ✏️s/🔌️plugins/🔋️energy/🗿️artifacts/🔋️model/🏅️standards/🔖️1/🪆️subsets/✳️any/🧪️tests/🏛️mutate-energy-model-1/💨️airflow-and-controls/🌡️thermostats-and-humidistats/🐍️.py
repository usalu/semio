"""🌡️ Thermostat and humidistat kinds: creation, zone binding, setpoint schedules and throttle ranges."""

from __future__ import annotations

import copy

from mutate_energy_model_1_mutation_vocabulary import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-thermostat-controls-zone-one": "shared://🧬️mutations/🌡️thermostat/🩺️create/✅️controls",
    "create-thermostat-refuses-an-absent-zone": "shared://🧬️mutations/🌡️thermostat/🩺️create/⛔️refuses",
    "delete-thermostat-frees-zone-one": "shared://🧬️mutations/🌡️thermostat/🛑️delete/✅️frees-zone-one",
    "delete-thermostat-refuses-an-absent-one": "shared://🧬️mutations/🌡️thermostat/🛑️delete/⛔️refuses",
    "change-thermostat-zone-moves-to-zone-two": "shared://🧬️mutations/🌡️thermostat/🛖️change-zone/✅️moves",
    "change-thermostat-zone-refuses-an-absent-zone": "shared://🧬️mutations/🌡️thermostat/🛖️change-zone/⛔️refuses",
    "change-thermostat-heating-setpoint-schedule-repoints-heating": "shared://🧬️mutations/🌡️thermostat/🥵️change-heating-setpoint-schedule/✅️repoints",
    "change-thermostat-heating-setpoint-schedule-refuses-an-absent-one": "shared://🧬️mutations/🌡️thermostat/🥵️change-heating-setpoint-schedule/⛔️refuses",
    "change-thermostat-cooling-setpoint-schedule-repoints-cooling": "shared://🧬️mutations/🌡️thermostat/🐧️change-cooling-setpoint-schedule/✅️repoints",
    "change-thermostat-cooling-setpoint-schedule-refuses-an-absent-one": "shared://🧬️mutations/🌡️thermostat/🐧️change-cooling-setpoint-schedule/⛔️refuses",
    "change-thermostat-heating-throttle-range-widens-heating-band": "shared://🧬️mutations/🌡️thermostat/🎚️change-heating-throttle-range/✅️widens",
    "change-thermostat-heating-throttle-range-refuses-a-zero-band": "shared://🧬️mutations/🌡️thermostat/🎚️change-heating-throttle-range/⛔️refuses",
    "change-thermostat-cooling-throttle-range-widens-cooling-band": "shared://🧬️mutations/🌡️thermostat/🎛️change-cooling-throttle-range/✅️widens",
    "change-thermostat-cooling-throttle-range-refuses-a-negative-band": "shared://🧬️mutations/🌡️thermostat/🎛️change-cooling-throttle-range/⛔️refuses",
    "create-humidistat-controls-zone-one": "shared://🧬️mutations/💧️humidistat/🌂️create/✅️controls",
    "create-humidistat-refuses-an-absent-one": "shared://🧬️mutations/💧️humidistat/🌂️create/⛔️refuses",
    "delete-humidistat-drops-the-control": "shared://🧬️mutations/💧️humidistat/🏜️delete/✅️drops",
    "delete-humidistat-refuses-an-absent-one": "shared://🧬️mutations/💧️humidistat/🏜️delete/⛔️refuses",
    "change-humidistat-zone-moves-to-zone-two": "shared://🧬️mutations/💧️humidistat/🏙️change-zone/✅️moves",
    "change-humidistat-zone-refuses-an-absent-zone": "shared://🧬️mutations/💧️humidistat/🏙️change-zone/⛔️refuses",
    "change-humidistat-humidifying-setpoint-schedule-repoints-humidifying": "shared://🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/✅️repoints",
    "change-humidistat-humidifying-setpoint-schedule-refuses-an-absent-one": "shared://🧬️mutations/☔️change-humidistat-humidifying-setpoint-schedule/⛔️refuses",
    "change-humidistat-dehumidifying-setpoint-schedule-repoints-drying": "shared://🧬️mutations/💧️humidistat/🏝️change-dehumidifying-setpoint-schedule/✅️repoints",
    "change-humidistat-dehumidifying-setpoint-schedule-refuses-an-absent-one": "shared://🧬️mutations/💧️humidistat/🏝️change-dehumidifying-setpoint-schedule/⛔️refuses",
    "change-humidistat-humidifying-throttle-range-widens-the-band": "shared://🧬️mutations/💧️humidistat/🌧️change-humidifying-throttle-range/✅️widens",
    "change-humidistat-humidifying-throttle-range-refuses-a-zero-band": "shared://🧬️mutations/💧️humidistat/🌧️change-humidifying-throttle-range/⛔️refuses",
    "change-humidistat-dehumidifying-throttle-range-widens-the-band": "shared://🧬️mutations/💧️humidistat/🧻️change-dehumidifying-throttle-range/✅️widens",
    "change-humidistat-dehumidifying-throttle-range-refuses-a-negative-band": "shared://🧬️mutations/💧️humidistat/🧻️change-dehumidifying-throttle-range/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_thermostat(before, payload):
    """🩺️ `create-thermostat` — one new thermostat row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["thermostats"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(entry["id"] == payload["heatingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["heatingSetpointScheduleId"])])
    if not any(entry["id"] == payload["coolingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["coolingSetpointScheduleId"])])
    if payload["heatingThrottleRangeK"] != payload["heatingThrottleRangeK"] or payload["heatingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["heatingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["coolingThrottleRangeK"] != payload["coolingThrottleRangeK"] or payload["coolingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["coolingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["thermostats"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["thermostats"].insert(len(after["model"]["thermostats"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "heating_setpoint_schedule_id": payload["heatingSetpointScheduleId"], "cooling_setpoint_schedule_id": payload["coolingSetpointScheduleId"], "heating_throttle_range_k": payload["heatingThrottleRangeK"], "cooling_throttle_range_k": payload["coolingThrottleRangeK"]})
    return after, applied()


def _invert_create_thermostat(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-thermostat", {"id": payload["id"]})]


def delete_thermostat(before, payload):
    """🛑️ `delete-thermostat` — drops the addressed thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["thermostats"] = [entry for entry in after["model"]["thermostats"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_thermostat(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["thermostats"]) if entry["id"] == payload["id"])
    return [("create-thermostat", {"id": item["id"], "zoneId": item["zone_id"], "heatingSetpointScheduleId": item["heating_setpoint_schedule_id"], "coolingSetpointScheduleId": item["cooling_setpoint_schedule_id"], "heatingThrottleRangeK": item["heating_throttle_range_k"], "coolingThrottleRangeK": item["cooling_throttle_range_k"], "index": index})]


def change_thermostat_zone(before, payload):
    """🛖️ `change-thermostat-zone` — the zone of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_thermostat_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_thermostat_heating_setpoint_schedule(before, payload):
    """🥵️ `change-thermostat-heating-setpoint-schedule` — the heating setpoint schedule of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newHeatingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newHeatingSetpointScheduleId"])])
    if item["heating_setpoint_schedule_id"] == payload["newHeatingSetpointScheduleId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["heating_setpoint_schedule_id"] = payload["newHeatingSetpointScheduleId"]
    return after, applied()


def _invert_change_thermostat_heating_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old heating setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-heating-setpoint-schedule", {"id": payload["id"], "newHeatingSetpointScheduleId": item["heating_setpoint_schedule_id"]})]


def change_thermostat_cooling_setpoint_schedule(before, payload):
    """🐧️ `change-thermostat-cooling-setpoint-schedule` — the cooling setpoint schedule of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newCoolingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newCoolingSetpointScheduleId"])])
    if item["cooling_setpoint_schedule_id"] == payload["newCoolingSetpointScheduleId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["cooling_setpoint_schedule_id"] = payload["newCoolingSetpointScheduleId"]
    return after, applied()


def _invert_change_thermostat_cooling_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old cooling setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-cooling-setpoint-schedule", {"id": payload["id"], "newCoolingSetpointScheduleId": item["cooling_setpoint_schedule_id"]})]


def change_thermostat_heating_throttle_range(before, payload):
    """🎚️ `change-thermostat-heating-throttle-range` — the heating throttle range of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newHeatingThrottleRangeK"] != payload["newHeatingThrottleRangeK"] or payload["newHeatingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["newHeatingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["heating_throttle_range_k"] == payload["newHeatingThrottleRangeK"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["heating_throttle_range_k"] = payload["newHeatingThrottleRangeK"]
    return after, applied()


def _invert_change_thermostat_heating_throttle_range(before, payload):
    """↩️ Its own inverse, with the old heating throttle range read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-heating-throttle-range", {"id": payload["id"], "newHeatingThrottleRangeK": item["heating_throttle_range_k"]})]


def change_thermostat_cooling_throttle_range(before, payload):
    """🎛️ `change-thermostat-cooling-throttle-range` — the cooling throttle range of one thermostat row."""
    item = next((entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newCoolingThrottleRangeK"] != payload["newCoolingThrottleRangeK"] or payload["newCoolingThrottleRangeK"] in (float("inf"), float("-inf")) or payload["newCoolingThrottleRangeK"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["cooling_throttle_range_k"] == payload["newCoolingThrottleRangeK"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["thermostats"]:
        if entry["id"] == payload["id"]:
            entry["cooling_throttle_range_k"] = payload["newCoolingThrottleRangeK"]
    return after, applied()


def _invert_change_thermostat_cooling_throttle_range(before, payload):
    """↩️ Its own inverse, with the old cooling throttle range read off BASE."""
    item = next(entry for entry in before["model"]["thermostats"] if entry["id"] == payload["id"])
    return [("change-thermostat-cooling-throttle-range", {"id": payload["id"], "newCoolingThrottleRangeK": item["cooling_throttle_range_k"]})]


def create_humidistat(before, payload):
    """🌂️ `create-humidistat` — one new humidistat row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["humidistats"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(entry["id"] == payload["humidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["humidifyingSetpointScheduleId"])])
    if not any(entry["id"] == payload["dehumidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["dehumidifyingSetpointScheduleId"])])
    if payload["humidifyingThrottleRange"] != payload["humidifyingThrottleRange"] or payload["humidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["humidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["dehumidifyingThrottleRange"] != payload["dehumidifyingThrottleRange"] or payload["dehumidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["dehumidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["humidistats"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["humidistats"].insert(len(after["model"]["humidistats"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "humidifying_setpoint_schedule_id": payload["humidifyingSetpointScheduleId"], "dehumidifying_setpoint_schedule_id": payload["dehumidifyingSetpointScheduleId"], "humidifying_throttle_range": payload["humidifyingThrottleRange"], "dehumidifying_throttle_range": payload["dehumidifyingThrottleRange"]})
    return after, applied()


def _invert_create_humidistat(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-humidistat", {"id": payload["id"]})]


def delete_humidistat(before, payload):
    """🏜️ `delete-humidistat` — drops the addressed humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["humidistats"] = [entry for entry in after["model"]["humidistats"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_humidistat(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["humidistats"]) if entry["id"] == payload["id"])
    return [("create-humidistat", {"id": item["id"], "zoneId": item["zone_id"], "humidifyingSetpointScheduleId": item["humidifying_setpoint_schedule_id"], "dehumidifyingSetpointScheduleId": item["dehumidifying_setpoint_schedule_id"], "humidifyingThrottleRange": item["humidifying_throttle_range"], "dehumidifyingThrottleRange": item["dehumidifying_throttle_range"], "index": index})]


def change_humidistat_zone(before, payload):
    """🏙️ `change-humidistat-zone` — the zone of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_humidistat_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_humidistat_humidifying_setpoint_schedule(before, payload):
    """☔️ `change-humidistat-humidifying-setpoint-schedule` — the humidifying setpoint schedule of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newHumidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newHumidifyingSetpointScheduleId"])])
    if item["humidifying_setpoint_schedule_id"] == payload["newHumidifyingSetpointScheduleId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["humidifying_setpoint_schedule_id"] = payload["newHumidifyingSetpointScheduleId"]
    return after, applied()


def _invert_change_humidistat_humidifying_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old humidifying setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-humidifying-setpoint-schedule", {"id": payload["id"], "newHumidifyingSetpointScheduleId": item["humidifying_setpoint_schedule_id"]})]


def change_humidistat_dehumidifying_setpoint_schedule(before, payload):
    """🏝️ `change-humidistat-dehumidifying-setpoint-schedule` — the dehumidifying setpoint schedule of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newDehumidifyingSetpointScheduleId"] for family in ("constants", "daily", "weekly", "annual", "time_series",) for entry in before["model"]["schedules"][family]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newDehumidifyingSetpointScheduleId"])])
    if item["dehumidifying_setpoint_schedule_id"] == payload["newDehumidifyingSetpointScheduleId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["dehumidifying_setpoint_schedule_id"] = payload["newDehumidifyingSetpointScheduleId"]
    return after, applied()


def _invert_change_humidistat_dehumidifying_setpoint_schedule(before, payload):
    """↩️ Its own inverse, with the old dehumidifying setpoint schedule read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-dehumidifying-setpoint-schedule", {"id": payload["id"], "newDehumidifyingSetpointScheduleId": item["dehumidifying_setpoint_schedule_id"]})]


def change_humidistat_humidifying_throttle_range(before, payload):
    """🌧️ `change-humidistat-humidifying-throttle-range` — the humidifying throttle range of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newHumidifyingThrottleRange"] != payload["newHumidifyingThrottleRange"] or payload["newHumidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["newHumidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["humidifying_throttle_range"] == payload["newHumidifyingThrottleRange"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["humidifying_throttle_range"] = payload["newHumidifyingThrottleRange"]
    return after, applied()


def _invert_change_humidistat_humidifying_throttle_range(before, payload):
    """↩️ Its own inverse, with the old humidifying throttle range read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-humidifying-throttle-range", {"id": payload["id"], "newHumidifyingThrottleRange": item["humidifying_throttle_range"]})]


def change_humidistat_dehumidifying_throttle_range(before, payload):
    """🧻️ `change-humidistat-dehumidifying-throttle-range` — the dehumidifying throttle range of one humidistat row."""
    item = next((entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newDehumidifyingThrottleRange"] != payload["newDehumidifyingThrottleRange"] or payload["newDehumidifyingThrottleRange"] in (float("inf"), float("-inf")) or payload["newDehumidifyingThrottleRange"] <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["dehumidifying_throttle_range"] == payload["newDehumidifyingThrottleRange"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["humidistats"]:
        if entry["id"] == payload["id"]:
            entry["dehumidifying_throttle_range"] = payload["newDehumidifyingThrottleRange"]
    return after, applied()


def _invert_change_humidistat_dehumidifying_throttle_range(before, payload):
    """↩️ Its own inverse, with the old dehumidifying throttle range read off BASE."""
    item = next(entry for entry in before["model"]["humidistats"] if entry["id"] == payload["id"])
    return [("change-humidistat-dehumidifying-throttle-range", {"id": payload["id"], "newDehumidifyingThrottleRange": item["dehumidifying_throttle_range"]})]


VOCABULARY = {
    "create-thermostat": create_thermostat,
    "delete-thermostat": delete_thermostat,
    "change-thermostat-zone": change_thermostat_zone,
    "change-thermostat-heating-setpoint-schedule": change_thermostat_heating_setpoint_schedule,
    "change-thermostat-cooling-setpoint-schedule": change_thermostat_cooling_setpoint_schedule,
    "change-thermostat-heating-throttle-range": change_thermostat_heating_throttle_range,
    "change-thermostat-cooling-throttle-range": change_thermostat_cooling_throttle_range,
    "create-humidistat": create_humidistat,
    "delete-humidistat": delete_humidistat,
    "change-humidistat-zone": change_humidistat_zone,
    "change-humidistat-humidifying-setpoint-schedule": change_humidistat_humidifying_setpoint_schedule,
    "change-humidistat-dehumidifying-setpoint-schedule": change_humidistat_dehumidifying_setpoint_schedule,
    "change-humidistat-humidifying-throttle-range": change_humidistat_humidifying_throttle_range,
    "change-humidistat-dehumidifying-throttle-range": change_humidistat_dehumidifying_throttle_range,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-thermostat": _invert_create_thermostat,
    "delete-thermostat": _invert_delete_thermostat,
    "change-thermostat-zone": _invert_change_thermostat_zone,
    "change-thermostat-heating-setpoint-schedule": _invert_change_thermostat_heating_setpoint_schedule,
    "change-thermostat-cooling-setpoint-schedule": _invert_change_thermostat_cooling_setpoint_schedule,
    "change-thermostat-heating-throttle-range": _invert_change_thermostat_heating_throttle_range,
    "change-thermostat-cooling-throttle-range": _invert_change_thermostat_cooling_throttle_range,
    "create-humidistat": _invert_create_humidistat,
    "delete-humidistat": _invert_delete_humidistat,
    "change-humidistat-zone": _invert_change_humidistat_zone,
    "change-humidistat-humidifying-setpoint-schedule": _invert_change_humidistat_humidifying_setpoint_schedule,
    "change-humidistat-dehumidifying-setpoint-schedule": _invert_change_humidistat_dehumidifying_setpoint_schedule,
    "change-humidistat-humidifying-throttle-range": _invert_change_humidistat_humidifying_throttle_range,
    "change-humidistat-dehumidifying-throttle-range": _invert_change_humidistat_dehumidifying_throttle_range,
}
# endregion 🔖️Vocabulary
