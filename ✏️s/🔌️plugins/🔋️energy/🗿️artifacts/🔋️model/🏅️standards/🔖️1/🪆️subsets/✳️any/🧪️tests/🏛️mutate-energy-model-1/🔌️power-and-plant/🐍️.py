"""🔌️ Power and plant kinds: load centers, PV, batteries, hot water, solar thermal, refrigeration, water systems and faults."""

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


c1 = _load("mutate_energy_model_1_power_hot_water_solar", "🚿️hot-water-and-solar-thermal")
c2 = _load("mutate_energy_model_1_power_services_faults", "❄️refrigeration-water-and-faults")


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-electrical-load-center-applies": "shared://🧬️mutations/🔌️load-center/🏦️create/✅️applies",
    "create-electrical-load-center-refuses": "shared://🧬️mutations/🔌️load-center/🏦️create/⛔️refuses",
    "delete-electrical-load-center-applies": "shared://🧬️mutations/🔌️load-center/🔻️delete/✅️applies",
    "delete-electrical-load-center-refuses": "shared://🧬️mutations/🔌️load-center/🔻️delete/⛔️refuses",
    "rename-electrical-load-center-applies": "shared://🧬️mutations/🔌️load-center/🖊️rename/✅️applies",
    "rename-electrical-load-center-refuses": "shared://🧬️mutations/🔌️load-center/🖊️rename/⛔️refuses",
    "add-electrical-load-center-pv-applies": "shared://🧬️mutations/🔌️load-center/☄️add-pv/✅️applies",
    "add-electrical-load-center-pv-refuses": "shared://🧬️mutations/🔌️load-center/☄️add-pv/⛔️refuses",
    "remove-electrical-load-center-pv-applies": "shared://🧬️mutations/🔌️load-center/🌘️remove-pv/✅️applies",
    "remove-electrical-load-center-pv-refuses": "shared://🧬️mutations/🔌️load-center/🌘️remove-pv/⛔️refuses",
    "add-electrical-load-center-battery-applies": "shared://🧬️mutations/🔌️load-center/🔋️add-battery/✅️applies",
    "add-electrical-load-center-battery-refuses": "shared://🧬️mutations/🔌️load-center/🔋️add-battery/⛔️refuses",
    "remove-electrical-load-center-battery-applies": "shared://🧬️mutations/🔌️load-center/🪝️remove-battery/✅️applies",
    "remove-electrical-load-center-battery-refuses": "shared://🧬️mutations/🔌️load-center/🪝️remove-battery/⛔️refuses",
    "create-pv-system-applies": "shared://🧬️mutations/☀️pv-system/✨️create/✅️applies",
    "create-pv-system-refuses": "shared://🧬️mutations/☀️pv-system/✨️create/⛔️refuses",
    "delete-pv-system-applies": "shared://🧬️mutations/☀️pv-system/🌒️delete/✅️applies",
    "delete-pv-system-refuses": "shared://🧬️mutations/☀️pv-system/🌒️delete/⛔️refuses",
    "change-pv-system-dc-capacity-applies": "shared://🧬️mutations/☀️pv-system/⚛️change-dc-capacity/✅️applies",
    "change-pv-system-dc-capacity-refuses": "shared://🧬️mutations/☀️pv-system/⚛️change-dc-capacity/⛔️refuses",
    "change-pv-system-area-applies": "shared://🧬️mutations/☀️pv-system/🟨️change-area/✅️applies",
    "change-pv-system-area-refuses": "shared://🧬️mutations/☀️pv-system/🟨️change-area/⛔️refuses",
    "change-pv-system-tilt-applies": "shared://🧬️mutations/☀️pv-system/📈️change-tilt/✅️applies",
    "change-pv-system-tilt-refuses": "shared://🧬️mutations/☀️pv-system/📈️change-tilt/⛔️refuses",
    "change-pv-system-azimuth-applies": "shared://🧬️mutations/☀️pv-system/🧿️change-azimuth/✅️applies",
    "change-pv-system-azimuth-refuses": "shared://🧬️mutations/☀️pv-system/🧿️change-azimuth/⛔️refuses",
    "change-pv-system-module-efficiency-applies": "shared://🧬️mutations/☀️pv-system/🎖️change-module-efficiency/✅️applies",
    "change-pv-system-module-efficiency-refuses": "shared://🧬️mutations/☀️pv-system/🎖️change-module-efficiency/⛔️refuses",
    "change-pv-system-inverter-efficiency-applies": "shared://🧬️mutations/☀️pv-system/♌️change-inverter-efficiency/✅️applies",
    "change-pv-system-inverter-efficiency-refuses": "shared://🧬️mutations/☀️pv-system/♌️change-inverter-efficiency/⛔️refuses",
    "create-battery-applies": "shared://🧬️mutations/🪫️battery/🪙️create/✅️applies",
    "create-battery-refuses": "shared://🧬️mutations/🪫️battery/🪙️create/⛔️refuses",
    "delete-battery-applies": "shared://🧬️mutations/🪫️battery/♒️delete/✅️applies",
    "delete-battery-refuses": "shared://🧬️mutations/🪫️battery/♒️delete/⛔️refuses",
    "change-battery-capacity-applies": "shared://🧬️mutations/🪫️battery/🥫️change-capacity/✅️applies",
    "change-battery-capacity-refuses": "shared://🧬️mutations/🪫️battery/🥫️change-capacity/⛔️refuses",
    "change-battery-max-charge-applies": "shared://🧬️mutations/🪫️battery/⏫️change-max-charge/✅️applies",
    "change-battery-max-charge-refuses": "shared://🧬️mutations/🪫️battery/⏫️change-max-charge/⛔️refuses",
    "change-battery-max-discharge-applies": "shared://🧬️mutations/🪫️battery/⏬️change-max-discharge/✅️applies",
    "change-battery-max-discharge-refuses": "shared://🧬️mutations/🪫️battery/⏬️change-max-discharge/⛔️refuses",
    "change-battery-round-trip-efficiency-applies": "shared://🧬️mutations/🪫️battery/🥉️change-round-trip-efficiency/✅️applies",
    "change-battery-round-trip-efficiency-refuses": "shared://🧬️mutations/🪫️battery/🥉️change-round-trip-efficiency/⛔️refuses",
    **c1.VECTOR_ROOTS,
    **c2.VECTOR_ROOTS,
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_electrical_load_center(before, payload):
    """🏦️ `create-electrical-load-center{index,…}` — Adds one electrical load centre — the node that sums on-site generation and storage against the building's electrical demand. `pvIds` and `batteryIds` are checked against the document; `generatorIds` is carried verbatim and NOT checked, because `Model` has no generator collection for it to reference (vocabulary §5.4), which is also why this group ships no `add-electrical-load-center-generator`."""
    rows = before["model"]["electrical_load_centers"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    missing = next((candidate for candidate in payload["pvIds"] if not any(row["id"] == candidate for row in before["model"]["pv_systems"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    missing = next((candidate for candidate in payload["batteryIds"] if not any(row["id"] == candidate for row in before["model"]["battery_storage"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["electrical_load_centers"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "generator_ids": payload["generatorIds"], "pv_ids": payload["pvIds"], "battery_ids": payload["batteryIds"]})
    return after, applied()


def _invert_create_electrical_load_center(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-electrical-load-center", {"id": payload["id"]})]


def delete_electrical_load_center(before, payload):
    """🔻️ `delete-electrical-load-center{id}` — Removes one electrical load centre. Nothing in the document references a load centre — it is the referencing end of every generation edge — so this never has to refuse for use and never cascades."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["electrical_load_centers"] = [row for row in after["model"]["electrical_load_centers"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_electrical_load_center(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["electrical_load_centers"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-electrical-load-center", {"index": index, "id": item["id"], "name": item["name"], "generatorIds": item["generator_ids"], "pvIds": item["pv_ids"], "batteryIds": item["battery_ids"]})]


def rename_electrical_load_center(before, payload):
    """🖊️ `rename-electrical-load-center{id,newName}` — Sets one electrical load center's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["electrical_load_centers"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_electrical_load_center(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"])
    return [("rename-electrical-load-center", {"id": payload["id"], "newName": item["name"]})]


def add_electrical_load_center_pv(before, payload):
    """☄️ `add-electrical-load-center-pv{id,index,pvId}` — Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["pvId"] for row in before["model"]["pv_systems"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["pvId"])])
    if payload["index"] > len(item["pv_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["pvId"] in item["pv_ids"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["pv_ids"].insert(payload["index"], payload["pvId"])
    return after, applied()


def _invert_add_electrical_load_center_pv(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-electrical-load-center-pv", {"id": payload["id"], "pvId": payload["pvId"]})]


def remove_electrical_load_center_pv(before, payload):
    """🌘️ `remove-electrical-load-center-pv{id,pvId}` — Detaches one PV system from a load centre; the PV system itself survives, unattached."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["pvId"] not in item["pv_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["pvId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["pv_ids"] = [candidate for candidate in row["pv_ids"] if candidate != payload["pvId"]]
    return after, applied()


def _invert_remove_electrical_load_center_pv(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"])
    return [("add-electrical-load-center-pv", {"id": payload["id"], "index": item["pv_ids"].index(payload["pvId"]), "pvId": payload["pvId"]})]


def add_electrical_load_center_battery(before, payload):
    """🔋️ `add-electrical-load-center-battery{id,index,batteryId}` — Attaches one battery to a load centre, so the centre may charge and discharge it against the net bus balance."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["batteryId"] for row in before["model"]["battery_storage"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["batteryId"])])
    if payload["index"] > len(item["battery_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["batteryId"] in item["battery_ids"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["battery_ids"].insert(payload["index"], payload["batteryId"])
    return after, applied()


def _invert_add_electrical_load_center_battery(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-electrical-load-center-battery", {"id": payload["id"], "batteryId": payload["batteryId"]})]


def remove_electrical_load_center_battery(before, payload):
    """🪝️ `remove-electrical-load-center-battery{id,batteryId}` — Detaches one battery from a load centre; the battery itself survives, unattached."""
    item = next((row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["batteryId"] not in item["battery_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["batteryId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["electrical_load_centers"]:
        if row["id"] == payload["id"]:
            row["battery_ids"] = [candidate for candidate in row["battery_ids"] if candidate != payload["batteryId"]]
    return after, applied()


def _invert_remove_electrical_load_center_battery(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["electrical_load_centers"] if row["id"] == payload["id"])
    return [("add-electrical-load-center-battery", {"id": payload["id"], "index": item["battery_ids"].index(payload["batteryId"]), "batteryId": payload["batteryId"]})]


def create_pv_system(before, payload):
    """✨️ `create-pv-system{index,…}` — Adds one photovoltaic array. Capacity, aperture area and the two efficiencies fix the DC-to-AC chain; tilt and azimuth place the plane the incident-solar model integrates over."""
    rows = before["model"]["pv_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not math.isfinite(payload["temperatureCoefficient"]) or not math.isfinite(payload["faimanHeatLossConstantWM2k"]) or payload["faimanHeatLossConstantWM2k"] <= 0 or not math.isfinite(payload["faimanHeatLossWindWSM3k"]) or payload["faimanHeatLossWindWSM3k"] < 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["pv_systems"].insert(payload["index"], {"id": payload["id"], "dc_capacity_w": payload["dcCapacityW"], "area_m2": payload["areaM2"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"], "module_efficiency": payload["moduleEfficiency"], "inverter_efficiency": payload["inverterEfficiency"],"temperature_coefficient":payload["temperatureCoefficient"],"faiman_heat_loss_constant_w_m2k":payload["faimanHeatLossConstantWM2k"],"faiman_heat_loss_wind_w_s_m3k":payload["faimanHeatLossWindWSM3k"],"mounting":payload["mounting"]})
    return after, applied()


def _invert_create_pv_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-pv-system", {"id": payload["id"]})]


def delete_pv_system(before, payload):
    """🌒️ `delete-pv-system{id}` — Removes one photovoltaic array. Refused while a load centre still lists it, so the electrical bus never sums over an array the document no longer defines."""
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(payload["id"] in row["pv_ids"] for row in before["model"]["electrical_load_centers"]):
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["pv_systems"] = [row for row in after["model"]["pv_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_pv_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["pv_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-pv-system", {"index": index, "id": item["id"], "dcCapacityW": item["dc_capacity_w"], "areaM2": item["area_m2"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"], "moduleEfficiency": item["module_efficiency"], "inverterEfficiency": item["inverter_efficiency"],"temperatureCoefficient":item["temperature_coefficient"],"faimanHeatLossConstantWM2k":item["faiman_heat_loss_constant_w_m2k"],"faimanHeatLossWindWSM3k":item["faiman_heat_loss_wind_w_s_m3k"],"mounting":item["mounting"]})]


def change_pv_system_dc_capacity(before, payload):
    """⚛️ `change-pv-system-dc-capacity{id,newDcCapacityW}` — Sets DC capacity (W) on one pv system, addressed by id."""
    value = payload["newDcCapacityW"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["dc_capacity_w"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["dc_capacity_w"] = value
    return after, applied()


def _invert_change_pv_system_dc_capacity(before, payload):
    """↩️ The dc_capacity_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-dc-capacity", {"id": payload["id"], "newDcCapacityW": item["dc_capacity_w"]})]


def change_pv_system_area(before, payload):
    """🟨️ `change-pv-system-area{id,newAreaM2}` — Sets aperture area (m²) on one pv system, addressed by id."""
    value = payload["newAreaM2"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["area_m2"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["area_m2"] = value
    return after, applied()


def _invert_change_pv_system_area(before, payload):
    """↩️ The area_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-area", {"id": payload["id"], "newAreaM2": item["area_m2"]})]


def change_pv_system_tilt(before, payload):
    """📈️ `change-pv-system-tilt{id,newTiltDeg}` — Sets tilt on one pv system, addressed by id."""
    value = payload["newTiltDeg"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 90.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["tilt_deg"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["tilt_deg"] = value
    return after, applied()


def _invert_change_pv_system_tilt(before, payload):
    """↩️ The tilt_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-tilt", {"id": payload["id"], "newTiltDeg": item["tilt_deg"]})]


def change_pv_system_azimuth(before, payload):
    """🧿️ `change-pv-system-azimuth{id,newAzimuthDeg}` — Sets azimuth on one pv system, addressed by id."""
    value = payload["newAzimuthDeg"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or not 0.0 <= value <= 360.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["azimuth_deg"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["azimuth_deg"] = value
    return after, applied()


def _invert_change_pv_system_azimuth(before, payload):
    """↩️ The azimuth_deg the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-azimuth", {"id": payload["id"], "newAzimuthDeg": item["azimuth_deg"]})]


def change_pv_system_module_efficiency(before, payload):
    """🎖️ `change-pv-system-module-efficiency{id,newModuleEfficiency}` — Sets module efficiency on one pv system, addressed by id."""
    value = payload["newModuleEfficiency"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["module_efficiency"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["module_efficiency"] = value
    return after, applied()


def _invert_change_pv_system_module_efficiency(before, payload):
    """↩️ The module_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-module-efficiency", {"id": payload["id"], "newModuleEfficiency": item["module_efficiency"]})]


def change_pv_system_inverter_efficiency(before, payload):
    """♌️ `change-pv-system-inverter-efficiency{id,newInverterEfficiency}` — Sets inverter efficiency on one pv system, addressed by id."""
    value = payload["newInverterEfficiency"]
    item = next((row for row in before["model"]["pv_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["inverter_efficiency"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["pv_systems"]:
        if row["id"] == payload["id"]:
            row["inverter_efficiency"] = value
    return after, applied()


def _invert_change_pv_system_inverter_efficiency(before, payload):
    """↩️ The inverter_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["pv_systems"] if row["id"] == payload["id"])
    return [("change-pv-system-inverter-efficiency", {"id": payload["id"], "newInverterEfficiency": item["inverter_efficiency"]})]


def create_battery(before, payload):
    """🪙️ `create-battery{index,…}` — Adds one electrical storage unit. Capacity bounds the state of charge; the two power limits bound each timestep's charge and discharge, and the round-trip efficiency is what the stored energy is debited by."""
    rows = before["model"]["battery_storage"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["battery_storage"].insert(payload["index"], {"id": payload["id"], "capacity_kwh": payload["capacityKwh"], "max_charge_w": payload["maxChargeW"], "max_discharge_w": payload["maxDischargeW"], "round_trip_efficiency": payload["roundTripEfficiency"]})
    return after, applied()


def _invert_create_battery(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-battery", {"id": payload["id"]})]


def delete_battery(before, payload):
    """♒️ `delete-battery{id}` — Removes one electrical storage unit. Refused while a load centre still lists it."""
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if any(payload["id"] in row["battery_ids"] for row in before["model"]["electrical_load_centers"]):
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["battery_storage"] = [row for row in after["model"]["battery_storage"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_battery(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["battery_storage"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-battery", {"index": index, "id": item["id"], "capacityKwh": item["capacity_kwh"], "maxChargeW": item["max_charge_w"], "maxDischargeW": item["max_discharge_w"], "roundTripEfficiency": item["round_trip_efficiency"]})]


def change_battery_capacity(before, payload):
    """🥫️ `change-battery-capacity{id,newCapacityKwh}` — Sets storage capacity (kWh) on one battery, addressed by id."""
    value = payload["newCapacityKwh"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["capacity_kwh"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["capacity_kwh"] = value
    return after, applied()


def _invert_change_battery_capacity(before, payload):
    """↩️ The capacity_kwh the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-capacity", {"id": payload["id"], "newCapacityKwh": item["capacity_kwh"]})]


def change_battery_max_charge(before, payload):
    """⏫️ `change-battery-max-charge{id,newMaxChargeW}` — Sets maximum charge power (W) on one battery, addressed by id."""
    value = payload["newMaxChargeW"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["max_charge_w"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["max_charge_w"] = value
    return after, applied()


def _invert_change_battery_max_charge(before, payload):
    """↩️ The max_charge_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-max-charge", {"id": payload["id"], "newMaxChargeW": item["max_charge_w"]})]


def change_battery_max_discharge(before, payload):
    """⏬️ `change-battery-max-discharge{id,newMaxDischargeW}` — Sets maximum discharge power (W) on one battery, addressed by id."""
    value = payload["newMaxDischargeW"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["max_discharge_w"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["max_discharge_w"] = value
    return after, applied()


def _invert_change_battery_max_discharge(before, payload):
    """↩️ The max_discharge_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-max-discharge", {"id": payload["id"], "newMaxDischargeW": item["max_discharge_w"]})]


def change_battery_round_trip_efficiency(before, payload):
    """🥉️ `change-battery-round-trip-efficiency{id,newRoundTripEfficiency}` — Sets round-trip efficiency on one battery, addressed by id."""
    value = payload["newRoundTripEfficiency"]
    item = next((row for row in before["model"]["battery_storage"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["round_trip_efficiency"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["battery_storage"]:
        if row["id"] == payload["id"]:
            row["round_trip_efficiency"] = value
    return after, applied()


def _invert_change_battery_round_trip_efficiency(before, payload):
    """↩️ The round_trip_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["battery_storage"] if row["id"] == payload["id"])
    return [("change-battery-round-trip-efficiency", {"id": payload["id"], "newRoundTripEfficiency": item["round_trip_efficiency"]})]


VOCABULARY = {
    "create-electrical-load-center": create_electrical_load_center,
    "delete-electrical-load-center": delete_electrical_load_center,
    "rename-electrical-load-center": rename_electrical_load_center,
    "add-electrical-load-center-pv": add_electrical_load_center_pv,
    "remove-electrical-load-center-pv": remove_electrical_load_center_pv,
    "add-electrical-load-center-battery": add_electrical_load_center_battery,
    "remove-electrical-load-center-battery": remove_electrical_load_center_battery,
    "create-pv-system": create_pv_system,
    "delete-pv-system": delete_pv_system,
    "change-pv-system-dc-capacity": change_pv_system_dc_capacity,
    "change-pv-system-area": change_pv_system_area,
    "change-pv-system-tilt": change_pv_system_tilt,
    "change-pv-system-azimuth": change_pv_system_azimuth,
    "change-pv-system-module-efficiency": change_pv_system_module_efficiency,
    "change-pv-system-inverter-efficiency": change_pv_system_inverter_efficiency,
    "create-battery": create_battery,
    "delete-battery": delete_battery,
    "change-battery-capacity": change_battery_capacity,
    "change-battery-max-charge": change_battery_max_charge,
    "change-battery-max-discharge": change_battery_max_discharge,
    "change-battery-round-trip-efficiency": change_battery_round_trip_efficiency,
    **c1.VOCABULARY,
    **c2.VOCABULARY,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-electrical-load-center": _invert_create_electrical_load_center,
    "delete-electrical-load-center": _invert_delete_electrical_load_center,
    "rename-electrical-load-center": _invert_rename_electrical_load_center,
    "add-electrical-load-center-pv": _invert_add_electrical_load_center_pv,
    "remove-electrical-load-center-pv": _invert_remove_electrical_load_center_pv,
    "add-electrical-load-center-battery": _invert_add_electrical_load_center_battery,
    "remove-electrical-load-center-battery": _invert_remove_electrical_load_center_battery,
    "create-pv-system": _invert_create_pv_system,
    "delete-pv-system": _invert_delete_pv_system,
    "change-pv-system-dc-capacity": _invert_change_pv_system_dc_capacity,
    "change-pv-system-area": _invert_change_pv_system_area,
    "change-pv-system-tilt": _invert_change_pv_system_tilt,
    "change-pv-system-azimuth": _invert_change_pv_system_azimuth,
    "change-pv-system-module-efficiency": _invert_change_pv_system_module_efficiency,
    "change-pv-system-inverter-efficiency": _invert_change_pv_system_inverter_efficiency,
    "create-battery": _invert_create_battery,
    "delete-battery": _invert_delete_battery,
    "change-battery-capacity": _invert_change_battery_capacity,
    "change-battery-max-charge": _invert_change_battery_max_charge,
    "change-battery-max-discharge": _invert_change_battery_max_discharge,
    "change-battery-round-trip-efficiency": _invert_change_battery_round_trip_efficiency,
    **c1.EXTRA_INVERT,
    **c2.EXTRA_INVERT,
}
# endregion 🔖️Vocabulary
