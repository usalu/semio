"""🚿️ Service hot water and solar thermal system kinds."""

from __future__ import annotations

import copy

from mutate_energy_model_1_mutation_vocabulary import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-shw-system-applies": "shared://🧬️mutations/🚿️shw-system/🛀️create/✅️applies",
    "create-shw-system-refuses": "shared://🧬️mutations/🚿️shw-system/🛀️create/⛔️refuses",
    "delete-shw-system-applies": "shared://🧬️mutations/🚿️shw-system/🚱️delete/✅️applies",
    "delete-shw-system-refuses": "shared://🧬️mutations/🚿️shw-system/🚱️delete/⛔️refuses",
    "change-shw-system-heater-capacity-applies": "shared://🧬️mutations/🚿️shw-system/🍵️change-heater-capacity/✅️applies",
    "change-shw-system-heater-capacity-refuses": "shared://🧬️mutations/🚿️shw-system/🍵️change-heater-capacity/⛔️refuses",
    "change-shw-system-storage-volume-applies": "shared://🧬️mutations/🚿️shw-system/🛢️change-storage-volume/✅️applies",
    "change-shw-system-storage-volume-refuses": "shared://🧬️mutations/🚿️shw-system/🛢️change-storage-volume/⛔️refuses",
    "change-shw-system-setpoint-applies": "shared://🧬️mutations/🚿️shw-system/🏹️change-setpoint/✅️applies",
    "change-shw-system-setpoint-refuses": "shared://🧬️mutations/🚿️shw-system/🏹️change-setpoint/⛔️refuses",
    "change-shw-system-schedule-applies": "shared://🧬️mutations/🚿️shw-system/🕐️change-schedule/✅️applies",
    "change-shw-system-schedule-refuses": "shared://🧬️mutations/🚿️shw-system/🕐️change-schedule/⛔️refuses",
    "create-solar-thermal-system-applies": "shared://🧬️mutations/🔥️solar-thermal/🌄️create/✅️applies",
    "create-solar-thermal-system-refuses": "shared://🧬️mutations/🔥️solar-thermal/🌄️create/⛔️refuses",
    "delete-solar-thermal-system-applies": "shared://🧬️mutations/🔥️solar-thermal/🌆️delete/✅️applies",
    "delete-solar-thermal-system-refuses": "shared://🧬️mutations/🔥️solar-thermal/🌆️delete/⛔️refuses",
    "change-solar-thermal-system-collector-area-applies": "shared://🧬️mutations/🔥️solar-thermal/🟩️change-collector-area/✅️applies",
    "change-solar-thermal-system-collector-area-refuses": "shared://🧬️mutations/🔥️solar-thermal/🟩️change-collector-area/⛔️refuses",
    "change-solar-thermal-system-efficiency-applies": "shared://🧬️mutations/🔥️solar-thermal/🏅️change-efficiency/✅️applies",
    "change-solar-thermal-system-efficiency-refuses": "shared://🧬️mutations/🔥️solar-thermal/🏅️change-efficiency/⛔️refuses",
    "change-solar-thermal-system-storage-volume-applies": "shared://🧬️mutations/🔥️solar-thermal/🧃️change-storage-volume/✅️applies",
    "change-solar-thermal-system-storage-volume-refuses": "shared://🧬️mutations/🔥️solar-thermal/🧃️change-storage-volume/⛔️refuses",
    "change-solar-thermal-system-tilt-applies": "shared://🧬️mutations/🔥️solar-thermal/🔼️change-tilt/✅️applies",
    "change-solar-thermal-system-tilt-refuses": "shared://🧬️mutations/🔥️solar-thermal/🔼️change-tilt/⛔️refuses",
    "change-solar-thermal-system-azimuth-applies": "shared://🧬️mutations/🔥️solar-thermal/⛵️change-azimuth/✅️applies",
    "change-solar-thermal-system-azimuth-refuses": "shared://🧬️mutations/🔥️solar-thermal/⛵️change-azimuth/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_shw_system(before, payload):
    """🛀️ `create-shw-system{index,…}` — Adds one service-hot-water system: a storage tank, the heater that keeps it at setpoint, and the draw schedule the load profile is read from."""
    rows = before["model"]["shw_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["shw_systems"].insert(payload["index"], {"id": payload["id"], "heater_capacity_w": payload["heaterCapacityW"], "storage_volume_m3": payload["storageVolumeM3"], "setpoint_c": payload["setpointC"], "schedule_id": payload["scheduleId"]})
    return after, applied()


def _invert_create_shw_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-shw-system", {"id": payload["id"]})]


def delete_shw_system(before, payload):
    """🚱️ `delete-shw-system{id}` — Removes one service-hot-water system. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["shw_systems"] = [row for row in after["model"]["shw_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_shw_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["shw_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-shw-system", {"index": index, "id": item["id"], "heaterCapacityW": item["heater_capacity_w"], "storageVolumeM3": item["storage_volume_m3"], "setpointC": item["setpoint_c"], "scheduleId": item["schedule_id"]})]


def change_shw_system_heater_capacity(before, payload):
    """🍵️ `change-shw-system-heater-capacity{id,newHeaterCapacityW}` — Sets heater capacity (W) on one service hot water system, addressed by id."""
    value = payload["newHeaterCapacityW"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["heater_capacity_w"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["heater_capacity_w"] = value
    return after, applied()


def _invert_change_shw_system_heater_capacity(before, payload):
    """↩️ The heater_capacity_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-heater-capacity", {"id": payload["id"], "newHeaterCapacityW": item["heater_capacity_w"]})]


def change_shw_system_storage_volume(before, payload):
    """🛢️ `change-shw-system-storage-volume{id,newStorageVolumeM3}` — Sets storage volume (m³) on one service hot water system, addressed by id."""
    value = payload["newStorageVolumeM3"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["storage_volume_m3"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["storage_volume_m3"] = value
    return after, applied()


def _invert_change_shw_system_storage_volume(before, payload):
    """↩️ The storage_volume_m3 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-storage-volume", {"id": payload["id"], "newStorageVolumeM3": item["storage_volume_m3"]})]


def change_shw_system_setpoint(before, payload):
    """🏹️ `change-shw-system-setpoint{id,newSetpointC}` — Sets tank setpoint (°C) on one service hot water system, addressed by id."""
    value = payload["newSetpointC"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 100.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["setpoint_c"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["setpoint_c"] = value
    return after, applied()


def _invert_change_shw_system_setpoint(before, payload):
    """↩️ The setpoint_c the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-setpoint", {"id": payload["id"], "newSetpointC": item["setpoint_c"]})]


def change_shw_system_schedule(before, payload):
    """🕐️ `change-shw-system-schedule{id,newScheduleId}` — Re-points one service hot water system's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["shw_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["shw_systems"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_shw_system_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["shw_systems"] if row["id"] == payload["id"])
    return [("change-shw-system-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def create_solar_thermal_system(before, payload):
    """🌄️ `create-solar-thermal-system{index,…}` — Adds one solar thermal collector loop: aperture, conversion efficiency, the buffer it charges, and the plane the incident-solar model integrates over."""
    rows = before["model"]["solar_thermal_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["solar_thermal_systems"].insert(payload["index"], {"id": payload["id"], "collector_area_m2": payload["collectorAreaM2"], "efficiency": payload["efficiency"], "storage_volume_m3": payload["storageVolumeM3"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"]})
    return after, applied()


def _invert_create_solar_thermal_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-solar-thermal-system", {"id": payload["id"]})]


def delete_solar_thermal_system(before, payload):
    """🌆️ `delete-solar-thermal-system{id}` — Removes one solar thermal collector loop. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["solar_thermal_systems"] = [row for row in after["model"]["solar_thermal_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_solar_thermal_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["solar_thermal_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-solar-thermal-system", {"index": index, "id": item["id"], "collectorAreaM2": item["collector_area_m2"], "efficiency": item["efficiency"], "storageVolumeM3": item["storage_volume_m3"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"]})]


def change_solar_thermal_system_collector_area(before, payload):
    """🟩️ `change-solar-thermal-system-collector-area{id,newCollectorAreaM2}` — Sets collector area (m²) on one solar thermal system, addressed by id."""
    value = payload["newCollectorAreaM2"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["collector_area_m2"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["collector_area_m2"] = value
    return after, applied()


def _invert_change_solar_thermal_system_collector_area(before, payload):
    """↩️ The collector_area_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-collector-area", {"id": payload["id"], "newCollectorAreaM2": item["collector_area_m2"]})]


def change_solar_thermal_system_efficiency(before, payload):
    """🏅️ `change-solar-thermal-system-efficiency{id,newEfficiency}` — Sets collector efficiency on one solar thermal system, addressed by id."""
    value = payload["newEfficiency"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["efficiency"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["efficiency"] = value
    return after, applied()


def _invert_change_solar_thermal_system_efficiency(before, payload):
    """↩️ The efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-efficiency", {"id": payload["id"], "newEfficiency": item["efficiency"]})]


def change_solar_thermal_system_storage_volume(before, payload):
    """🧃️ `change-solar-thermal-system-storage-volume{id,newStorageVolumeM3}` — Sets buffer volume (m³) on one solar thermal system, addressed by id."""
    value = payload["newStorageVolumeM3"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["storage_volume_m3"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["storage_volume_m3"] = value
    return after, applied()


def _invert_change_solar_thermal_system_storage_volume(before, payload):
    """↩️ The storage_volume_m3 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-storage-volume", {"id": payload["id"], "newStorageVolumeM3": item["storage_volume_m3"]})]


def change_solar_thermal_system_tilt(before, payload):
    """🔼️ `change-solar-thermal-system-tilt{id,newTiltDeg}` — Sets tilt on one solar thermal system, addressed by id."""
    value = payload["newTiltDeg"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 90.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["tilt_deg"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["tilt_deg"] = value
    return after, applied()


def _invert_change_solar_thermal_system_tilt(before, payload):
    """↩️ The tilt_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-tilt", {"id": payload["id"], "newTiltDeg": item["tilt_deg"]})]


def change_solar_thermal_system_azimuth(before, payload):
    """⛵️ `change-solar-thermal-system-azimuth{id,newAzimuthDeg}` — Sets azimuth on one solar thermal system, addressed by id."""
    value = payload["newAzimuthDeg"]
    item = next((row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 360.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["azimuth_deg"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["solar_thermal_systems"]:
        if row["id"] == payload["id"]:
            row["azimuth_deg"] = value
    return after, applied()


def _invert_change_solar_thermal_system_azimuth(before, payload):
    """↩️ The azimuth_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["solar_thermal_systems"] if row["id"] == payload["id"])
    return [("change-solar-thermal-system-azimuth", {"id": payload["id"], "newAzimuthDeg": item["azimuth_deg"]})]


VOCABULARY = {
    "create-shw-system": create_shw_system,
    "delete-shw-system": delete_shw_system,
    "change-shw-system-heater-capacity": change_shw_system_heater_capacity,
    "change-shw-system-storage-volume": change_shw_system_storage_volume,
    "change-shw-system-setpoint": change_shw_system_setpoint,
    "change-shw-system-schedule": change_shw_system_schedule,
    "create-solar-thermal-system": create_solar_thermal_system,
    "delete-solar-thermal-system": delete_solar_thermal_system,
    "change-solar-thermal-system-collector-area": change_solar_thermal_system_collector_area,
    "change-solar-thermal-system-efficiency": change_solar_thermal_system_efficiency,
    "change-solar-thermal-system-storage-volume": change_solar_thermal_system_storage_volume,
    "change-solar-thermal-system-tilt": change_solar_thermal_system_tilt,
    "change-solar-thermal-system-azimuth": change_solar_thermal_system_azimuth,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-shw-system": _invert_create_shw_system,
    "delete-shw-system": _invert_delete_shw_system,
    "change-shw-system-heater-capacity": _invert_change_shw_system_heater_capacity,
    "change-shw-system-storage-volume": _invert_change_shw_system_storage_volume,
    "change-shw-system-setpoint": _invert_change_shw_system_setpoint,
    "change-shw-system-schedule": _invert_change_shw_system_schedule,
    "create-solar-thermal-system": _invert_create_solar_thermal_system,
    "delete-solar-thermal-system": _invert_delete_solar_thermal_system,
    "change-solar-thermal-system-collector-area": _invert_change_solar_thermal_system_collector_area,
    "change-solar-thermal-system-efficiency": _invert_change_solar_thermal_system_efficiency,
    "change-solar-thermal-system-storage-volume": _invert_change_solar_thermal_system_storage_volume,
    "change-solar-thermal-system-tilt": _invert_change_solar_thermal_system_tilt,
    "change-solar-thermal-system-azimuth": _invert_change_solar_thermal_system_azimuth,
}
# endregion 🔖️Vocabulary
