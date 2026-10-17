"""💯️ Ideal-loads system kinds: creation, zone binding, supply temperatures, capacities and outdoor air rates."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-ideal-loads-system-serves-zone-one": "shared://🧬️mutations/💯️ideal-loads/🫁️create/✅️serves",
    "create-ideal-loads-system-refuses-an-absent-zone": "shared://🧬️mutations/💯️ideal-loads/🫁️create/⛔️refuses",
    "delete-ideal-loads-system-drops-the-system": "shared://🧬️mutations/💯️ideal-loads/🫥️delete/✅️drops",
    "delete-ideal-loads-system-refuses-an-absent-one": "shared://🧬️mutations/💯️ideal-loads/🫥️delete/⛔️refuses",
    "change-ideal-loads-system-zone-moves-to-zone-two": "shared://🧬️mutations/💯️ideal-loads/🏢️change-zone/✅️moves",
    "change-ideal-loads-system-zone-refuses-an-absent-zone": "shared://🧬️mutations/💯️ideal-loads/🏢️change-zone/⛔️refuses",
    "change-ideal-loads-system-max-heating-supply-air-temp-cools-the-supply": "shared://🧬️mutations/💯️ideal-loads/🔴️change-max-heating-supply-air-temp/✅️cools",
    "change-ideal-loads-system-max-heating-supply-air-temp-refuses-a-hot-supply": "shared://🧬️mutations/💯️ideal-loads/🔴️change-max-heating-supply-air-temp/⛔️refuses",
    "change-ideal-loads-system-min-cooling-supply-air-temp-lowers-the-supply": "shared://🧬️mutations/💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/✅️lowers",
    "change-ideal-loads-system-min-cooling-supply-air-temp-refuses-a-cold-supply": "shared://🧬️mutations/💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/⛔️refuses",
    "change-ideal-loads-system-max-heating-capacity-caps-the-heating": "shared://🧬️mutations/💯️ideal-loads/⛽️change-max-heating-capacity/✅️caps",
    "change-ideal-loads-system-max-heating-capacity-refuses-a-stray-value": "shared://🧬️mutations/💯️ideal-loads/⛽️change-max-heating-capacity/⛔️refuses",
    "change-ideal-loads-system-max-cooling-capacity-caps-the-cooling": "shared://🧬️mutations/💯️ideal-loads/🟧️change-max-cooling-capacity/✅️caps",
    "change-ideal-loads-system-max-cooling-capacity-refuses-a-stray-value": "shared://🧬️mutations/💯️ideal-loads/🟧️change-max-cooling-capacity/⛔️refuses",
    "change-ideal-loads-system-outdoor-air-per-person-ventilates-per-head": "shared://🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/✅️ventilates",
    "change-ideal-loads-system-outdoor-air-per-person-refuses-a-negative": "shared://🧬️mutations/🧍️change-ideal-loads-system-outdoor-air-per-person/⛔️refuses",
    "change-ideal-loads-system-outdoor-air-per-area-ventilates-per-area": "shared://🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/✅️ventilates",
    "change-ideal-loads-system-outdoor-air-per-area-refuses-a-negative": "shared://🧬️mutations/🔳️change-ideal-loads-system-outdoor-air-per-area/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_ideal_loads_system(before, payload):
    """🫁️ `create-ideal-loads-system` — one new ideal loads system row under a caller-minted id."""
    if any(entry["id"] == payload["id"] for entry in before["model"]["ideal_loads"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if not any(entry["id"] == payload["zoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["maxHeatingSupplyAirTempC"] != payload["maxHeatingSupplyAirTempC"] or payload["maxHeatingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["maxHeatingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["minCoolingSupplyAirTempC"] != payload["minCoolingSupplyAirTempC"] or payload["minCoolingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["minCoolingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["outdoorAirPerPersonM3S"] != payload["outdoorAirPerPersonM3S"] or payload["outdoorAirPerPersonM3S"] in (float("inf"), float("-inf")) or payload["outdoorAirPerPersonM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["outdoorAirPerAreaM3SM2"] != payload["outdoorAirPerAreaM3SM2"] or payload["outdoorAirPerAreaM3SM2"] in (float("inf"), float("-inf")) or payload["outdoorAirPerAreaM3SM2"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not payload["maxHeatingCapacityPresent"] and payload["maxHeatingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not payload["maxCoolingCapacityPresent"] and payload["maxCoolingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["maxHeatingCapacityPresent"] and (payload["maxHeatingCapacityW"] != payload["maxHeatingCapacityW"] or payload["maxHeatingCapacityW"] in (float("inf"), float("-inf")) or payload["maxHeatingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["maxCoolingCapacityPresent"] and (payload["maxCoolingCapacityW"] != payload["maxCoolingCapacityW"] or payload["maxCoolingCapacityW"] in (float("inf"), float("-inf")) or payload["maxCoolingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["ideal_loads"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["ideal_loads"].insert(len(after["model"]["ideal_loads"]) if payload.get("index") is None else payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "max_heating_supply_air_temp_c": payload["maxHeatingSupplyAirTempC"], "min_cooling_supply_air_temp_c": payload["minCoolingSupplyAirTempC"], "max_heating_capacity_w": payload["maxHeatingCapacityW"] if payload["maxHeatingCapacityPresent"] else None, "max_cooling_capacity_w": payload["maxCoolingCapacityW"] if payload["maxCoolingCapacityPresent"] else None, "outdoor_air_per_person_m3_s": payload["outdoorAirPerPersonM3S"], "outdoor_air_per_area_m3_s_m2": payload["outdoorAirPerAreaM3SM2"]})
    return after, applied()


def _invert_create_ideal_loads_system(before, payload):
    """↩️ The undo of a create is the delete of exactly the row it added."""
    return [("delete-ideal-loads-system", {"id": payload["id"]})]


def delete_ideal_loads_system(before, payload):
    """🫥️ `delete-ideal-loads-system` — drops the addressed ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])

    after = copy.deepcopy(before)
    after["model"]["ideal_loads"] = [entry for entry in after["model"]["ideal_loads"] if entry["id"] != payload["id"]]
    return after, applied()


def _invert_delete_ideal_loads_system(before, payload):
    """↩️ The undo of a delete re-creates the row exactly as BASE held it."""
    index, item = next((position, entry) for position, entry in enumerate(before["model"]["ideal_loads"]) if entry["id"] == payload["id"])
    return [("create-ideal-loads-system", {"id": item["id"], "zoneId": item["zone_id"], "maxHeatingSupplyAirTempC": item["max_heating_supply_air_temp_c"], "minCoolingSupplyAirTempC": item["min_cooling_supply_air_temp_c"], "maxHeatingCapacityPresent": item["max_heating_capacity_w"] is not None, "maxHeatingCapacityW": item["max_heating_capacity_w"] if item["max_heating_capacity_w"] is not None else 0.0, "maxCoolingCapacityPresent": item["max_cooling_capacity_w"] is not None, "maxCoolingCapacityW": item["max_cooling_capacity_w"] if item["max_cooling_capacity_w"] is not None else 0.0, "outdoorAirPerPersonM3S": item["outdoor_air_per_person_m3_s"], "outdoorAirPerAreaM3SM2": item["outdoor_air_per_area_m3_s_m2"], "index": index})]


def change_ideal_loads_system_zone(before, payload):
    """🏢️ `change-ideal-loads-system-zone` — the zone of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(entry["id"] == payload["newZoneId"] for entry in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newZoneId"])])
    if item["zone_id"] == payload["newZoneId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["zone_id"] = payload["newZoneId"]
    return after, applied()


def _invert_change_ideal_loads_system_zone(before, payload):
    """↩️ Its own inverse, with the old zone read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_ideal_loads_system_max_heating_supply_air_temp(before, payload):
    """🔴️ `change-ideal-loads-system-max-heating-supply-air-temp` — the maximum heating supply air temperature of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newMaxHeatingSupplyAirTempC"] != payload["newMaxHeatingSupplyAirTempC"] or payload["newMaxHeatingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newMaxHeatingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["max_heating_supply_air_temp_c"] == payload["newMaxHeatingSupplyAirTempC"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["max_heating_supply_air_temp_c"] = payload["newMaxHeatingSupplyAirTempC"]
    return after, applied()


def _invert_change_ideal_loads_system_max_heating_supply_air_temp(before, payload):
    """↩️ Its own inverse, with the old maximum heating supply air temperature read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-max-heating-supply-air-temp", {"id": payload["id"], "newMaxHeatingSupplyAirTempC": item["max_heating_supply_air_temp_c"]})]


def change_ideal_loads_system_min_cooling_supply_air_temp(before, payload):
    """🔵️ `change-ideal-loads-system-min-cooling-supply-air-temp` — the minimum cooling supply air temperature of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newMinCoolingSupplyAirTempC"] != payload["newMinCoolingSupplyAirTempC"] or payload["newMinCoolingSupplyAirTempC"] in (float("inf"), float("-inf")) or not -100.0 <= payload["newMinCoolingSupplyAirTempC"] <= 200.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["min_cooling_supply_air_temp_c"] == payload["newMinCoolingSupplyAirTempC"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["min_cooling_supply_air_temp_c"] = payload["newMinCoolingSupplyAirTempC"]
    return after, applied()


def _invert_change_ideal_loads_system_min_cooling_supply_air_temp(before, payload):
    """↩️ Its own inverse, with the old minimum cooling supply air temperature read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-min-cooling-supply-air-temp", {"id": payload["id"], "newMinCoolingSupplyAirTempC": item["min_cooling_supply_air_temp_c"]})]


def change_ideal_loads_system_max_heating_capacity(before, payload):
    """⛽️ `change-ideal-loads-system-max-heating-capacity` — the optional maximum heating capacity of one ideal loads system row, carried as a present
    flag beside its value because `Option<T>` has no `dsl::DslField`."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newCapacityPresent"] and payload["newMaxHeatingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newCapacityPresent"] and (payload["newMaxHeatingCapacityW"] != payload["newMaxHeatingCapacityW"] or payload["newMaxHeatingCapacityW"] in (float("inf"), float("-inf")) or payload["newMaxHeatingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    value = payload["newMaxHeatingCapacityW"] if payload["newCapacityPresent"] else None
    if item["max_heating_capacity_w"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["max_heating_capacity_w"] = value
    return after, applied()


def _invert_change_ideal_loads_system_max_heating_capacity(before, payload):
    """↩️ Its own inverse, with the old maximum heating capacity read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    old = item["max_heating_capacity_w"]
    return [("change-ideal-loads-system-max-heating-capacity", {"id": payload["id"], "newCapacityPresent": old is not None, "newMaxHeatingCapacityW": old if old is not None else 0.0})]


def change_ideal_loads_system_max_cooling_capacity(before, payload):
    """🟧️ `change-ideal-loads-system-max-cooling-capacity` — the optional maximum cooling capacity of one ideal loads system row, carried as a present
    flag beside its value because `Option<T>` has no `dsl::DslField`."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["newCapacityPresent"] and payload["newMaxCoolingCapacityW"] != 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newCapacityPresent"] and (payload["newMaxCoolingCapacityW"] != payload["newMaxCoolingCapacityW"] or payload["newMaxCoolingCapacityW"] in (float("inf"), float("-inf")) or payload["newMaxCoolingCapacityW"] <= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    value = payload["newMaxCoolingCapacityW"] if payload["newCapacityPresent"] else None
    if item["max_cooling_capacity_w"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["max_cooling_capacity_w"] = value
    return after, applied()


def _invert_change_ideal_loads_system_max_cooling_capacity(before, payload):
    """↩️ Its own inverse, with the old maximum cooling capacity read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    old = item["max_cooling_capacity_w"]
    return [("change-ideal-loads-system-max-cooling-capacity", {"id": payload["id"], "newCapacityPresent": old is not None, "newMaxCoolingCapacityW": old if old is not None else 0.0})]


def change_ideal_loads_system_outdoor_air_per_person(before, payload):
    """🧍️ `change-ideal-loads-system-outdoor-air-per-person` — the outdoor air rate per person of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newOutdoorAirPerPersonM3S"] != payload["newOutdoorAirPerPersonM3S"] or payload["newOutdoorAirPerPersonM3S"] in (float("inf"), float("-inf")) or payload["newOutdoorAirPerPersonM3S"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["outdoor_air_per_person_m3_s"] == payload["newOutdoorAirPerPersonM3S"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["outdoor_air_per_person_m3_s"] = payload["newOutdoorAirPerPersonM3S"]
    return after, applied()


def _invert_change_ideal_loads_system_outdoor_air_per_person(before, payload):
    """↩️ Its own inverse, with the old outdoor air rate per person read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-outdoor-air-per-person", {"id": payload["id"], "newOutdoorAirPerPersonM3S": item["outdoor_air_per_person_m3_s"]})]


def change_ideal_loads_system_outdoor_air_per_area(before, payload):
    """🔳️ `change-ideal-loads-system-outdoor-air-per-area` — the outdoor air rate per floor area of one ideal loads system row."""
    item = next((entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["newOutdoorAirPerAreaM3SM2"] != payload["newOutdoorAirPerAreaM3SM2"] or payload["newOutdoorAirPerAreaM3SM2"] in (float("inf"), float("-inf")) or payload["newOutdoorAirPerAreaM3SM2"] < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["outdoor_air_per_area_m3_s_m2"] == payload["newOutdoorAirPerAreaM3SM2"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for entry in after["model"]["ideal_loads"]:
        if entry["id"] == payload["id"]:
            entry["outdoor_air_per_area_m3_s_m2"] = payload["newOutdoorAirPerAreaM3SM2"]
    return after, applied()


def _invert_change_ideal_loads_system_outdoor_air_per_area(before, payload):
    """↩️ Its own inverse, with the old outdoor air rate per floor area read off BASE."""
    item = next(entry for entry in before["model"]["ideal_loads"] if entry["id"] == payload["id"])
    return [("change-ideal-loads-system-outdoor-air-per-area", {"id": payload["id"], "newOutdoorAirPerAreaM3SM2": item["outdoor_air_per_area_m3_s_m2"]})]


VOCABULARY = {
    "create-ideal-loads-system": create_ideal_loads_system,
    "delete-ideal-loads-system": delete_ideal_loads_system,
    "change-ideal-loads-system-zone": change_ideal_loads_system_zone,
    "change-ideal-loads-system-max-heating-supply-air-temp": change_ideal_loads_system_max_heating_supply_air_temp,
    "change-ideal-loads-system-min-cooling-supply-air-temp": change_ideal_loads_system_min_cooling_supply_air_temp,
    "change-ideal-loads-system-max-heating-capacity": change_ideal_loads_system_max_heating_capacity,
    "change-ideal-loads-system-max-cooling-capacity": change_ideal_loads_system_max_cooling_capacity,
    "change-ideal-loads-system-outdoor-air-per-person": change_ideal_loads_system_outdoor_air_per_person,
    "change-ideal-loads-system-outdoor-air-per-area": change_ideal_loads_system_outdoor_air_per_area,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-ideal-loads-system": _invert_create_ideal_loads_system,
    "delete-ideal-loads-system": _invert_delete_ideal_loads_system,
    "change-ideal-loads-system-zone": _invert_change_ideal_loads_system_zone,
    "change-ideal-loads-system-max-heating-supply-air-temp": _invert_change_ideal_loads_system_max_heating_supply_air_temp,
    "change-ideal-loads-system-min-cooling-supply-air-temp": _invert_change_ideal_loads_system_min_cooling_supply_air_temp,
    "change-ideal-loads-system-max-heating-capacity": _invert_change_ideal_loads_system_max_heating_capacity,
    "change-ideal-loads-system-max-cooling-capacity": _invert_change_ideal_loads_system_max_cooling_capacity,
    "change-ideal-loads-system-outdoor-air-per-person": _invert_change_ideal_loads_system_outdoor_air_per_person,
    "change-ideal-loads-system-outdoor-air-per-area": _invert_change_ideal_loads_system_outdoor_air_per_area,
}
# endregion 🔖️Vocabulary
