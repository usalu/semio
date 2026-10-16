"""🔌️ Power and plant kinds: load centers, PV, batteries, hot water, solar thermal, refrigeration, water systems and faults."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


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
    "create-refrigeration-system-applies": "shared://🧬️mutations/❄️refrigeration/❄️create/✅️applies",
    "create-refrigeration-system-refuses": "shared://🧬️mutations/❄️refrigeration/❄️create/⛔️refuses",
    "delete-refrigeration-system-applies": "shared://🧬️mutations/❄️refrigeration/🫠️delete/✅️applies",
    "delete-refrigeration-system-refuses": "shared://🧬️mutations/❄️refrigeration/🫠️delete/⛔️refuses",
    "change-refrigeration-system-case-count-applies": "shared://🧬️mutations/❄️refrigeration/🗄️change-case-count/✅️applies",
    "change-refrigeration-system-case-count-refuses": "shared://🧬️mutations/❄️refrigeration/🗄️change-case-count/⛔️refuses",
    "change-refrigeration-system-design-load-applies": "shared://🧬️mutations/❄️refrigeration/🏋️change-design-load/✅️applies",
    "change-refrigeration-system-design-load-refuses": "shared://🧬️mutations/❄️refrigeration/🏋️change-design-load/⛔️refuses",
    "change-refrigeration-system-defrost-schedule-applies": "shared://🧬️mutations/❄️refrigeration/🕑️change-defrost-schedule/✅️applies",
    "change-refrigeration-system-defrost-schedule-refuses": "shared://🧬️mutations/❄️refrigeration/🕑️change-defrost-schedule/⛔️refuses",
    "create-water-system-applies": "shared://🧬️mutations/🚰️water-system/🚽️create/✅️applies",
    "create-water-system-refuses": "shared://🧬️mutations/🚰️water-system/🚽️create/⛔️refuses",
    "delete-water-system-applies": "shared://🧬️mutations/🚰️water-system/🧼️delete/✅️applies",
    "delete-water-system-refuses": "shared://🧬️mutations/🚰️water-system/🧼️delete/⛔️refuses",
    "change-water-system-fixture-count-applies": "shared://🧬️mutations/🚰️water-system/🪣️change-fixture-count/✅️applies",
    "change-water-system-fixture-count-refuses": "shared://🧬️mutations/🚰️water-system/🪣️change-fixture-count/⛔️refuses",
    "change-water-system-peak-flow-applies": "shared://🧬️mutations/🚰️water-system/🚾️change-peak-flow/✅️applies",
    "change-water-system-peak-flow-refuses": "shared://🧬️mutations/🚰️water-system/🚾️change-peak-flow/⛔️refuses",
    "change-water-system-schedule-applies": "shared://🧬️mutations/🚰️water-system/🕒️change-schedule/✅️applies",
    "change-water-system-schedule-refuses": "shared://🧬️mutations/🚰️water-system/🕒️change-schedule/⛔️refuses",
    "create-fault-applies": "shared://🧬️mutations/⚠️fault/⚠️create/✅️applies",
    "create-fault-refuses": "shared://🧬️mutations/⚠️fault/⚠️create/⛔️refuses",
    "delete-fault-applies": "shared://🧬️mutations/⚠️fault/🩹️delete/✅️applies",
    "delete-fault-refuses": "shared://🧬️mutations/⚠️fault/🩹️delete/⛔️refuses",
    "change-fault-target-equipment-applies": "shared://🧬️mutations/⚠️fault/🎣️change-target-equipment/✅️applies",
    "change-fault-target-equipment-refuses": "shared://🧬️mutations/⚠️fault/🎣️change-target-equipment/⛔️refuses",
    "change-fault-type-applies": "shared://🧬️mutations/⚠️fault/🐛️change-type/✅️applies",
    "change-fault-type-refuses": "shared://🧬️mutations/⚠️fault/🐛️change-type/⛔️refuses",
    "change-fault-severity-applies": "shared://🧬️mutations/⚠️fault/🌶️change-severity/✅️applies",
    "change-fault-severity-refuses": "shared://🧬️mutations/⚠️fault/🌶️change-severity/⛔️refuses",
    "change-fault-start-schedule-applies": "shared://🧬️mutations/⚠️fault/🕓️change-start-schedule/✅️applies",
    "change-fault-start-schedule-refuses": "shared://🧬️mutations/⚠️fault/🕓️change-start-schedule/⛔️refuses",
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
    after = copy.deepcopy(before)
    after["model"]["pv_systems"].insert(payload["index"], {"id": payload["id"], "dc_capacity_w": payload["dcCapacityW"], "area_m2": payload["areaM2"], "tilt_deg": payload["tiltDeg"], "azimuth_deg": payload["azimuthDeg"], "module_efficiency": payload["moduleEfficiency"], "inverter_efficiency": payload["inverterEfficiency"]})
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
    return [("create-pv-system", {"index": index, "id": item["id"], "dcCapacityW": item["dc_capacity_w"], "areaM2": item["area_m2"], "tiltDeg": item["tilt_deg"], "azimuthDeg": item["azimuth_deg"], "moduleEfficiency": item["module_efficiency"], "inverterEfficiency": item["inverter_efficiency"]})]


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


def create_refrigeration_system(before, payload):
    """❄️ `create-refrigeration-system{index,…}` — Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it."""
    rows = before["model"]["refrigeration_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["defrostScheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["defrostScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["refrigeration_systems"].insert(payload["index"], {"id": payload["id"], "case_count": payload["caseCount"], "design_load_w": payload["designLoadW"], "defrost_schedule_id": payload["defrostScheduleId"]})
    return after, applied()


def _invert_create_refrigeration_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-refrigeration-system", {"id": payload["id"]})]


def delete_refrigeration_system(before, payload):
    """🫠️ `delete-refrigeration-system{id}` — Removes one refrigeration system. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["refrigeration_systems"] = [row for row in after["model"]["refrigeration_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_refrigeration_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["refrigeration_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-refrigeration-system", {"index": index, "id": item["id"], "caseCount": item["case_count"], "designLoadW": item["design_load_w"], "defrostScheduleId": item["defrost_schedule_id"]})]


def change_refrigeration_system_case_count(before, payload):
    """🗄️ `change-refrigeration-system-case-count{id,newCaseCount}` — Sets how many display cases one refrigeration system carries."""
    value = payload["newCaseCount"]
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["case_count"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["refrigeration_systems"]:
        if row["id"] == payload["id"]:
            row["case_count"] = value
    return after, applied()


def _invert_change_refrigeration_system_case_count(before, payload):
    """↩️ The case_count the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"])
    return [("change-refrigeration-system-case-count", {"id": payload["id"], "newCaseCount": item["case_count"]})]


def change_refrigeration_system_design_load(before, payload):
    """🏋️ `change-refrigeration-system-design-load{id,newDesignLoadW}` — Sets design load (W) on one refrigeration system, addressed by id."""
    value = payload["newDesignLoadW"]
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_load_w"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["refrigeration_systems"]:
        if row["id"] == payload["id"]:
            row["design_load_w"] = value
    return after, applied()


def _invert_change_refrigeration_system_design_load(before, payload):
    """↩️ The design_load_w the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"])
    return [("change-refrigeration-system-design-load", {"id": payload["id"], "newDesignLoadW": item["design_load_w"]})]


def change_refrigeration_system_defrost_schedule(before, payload):
    """🕑️ `change-refrigeration-system-defrost-schedule{id,newDefrostScheduleId}` — Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused."""
    value = payload["newDefrostScheduleId"]
    item = next((row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["defrost_schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["refrigeration_systems"]:
        if row["id"] == payload["id"]:
            row["defrost_schedule_id"] = value
    return after, applied()


def _invert_change_refrigeration_system_defrost_schedule(before, payload):
    """↩️ The defrost_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["refrigeration_systems"] if row["id"] == payload["id"])
    return [("change-refrigeration-system-defrost-schedule", {"id": payload["id"], "newDefrostScheduleId": item["defrost_schedule_id"]})]


def create_water_system(before, payload):
    """🚽️ `create-water-system{index,…}` — Adds one cold-water use system: the fixtures it serves, their combined peak flow, and the draw schedule the profile is read from."""
    rows = before["model"]["water_systems"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["water_systems"].insert(payload["index"], {"id": payload["id"], "fixture_count": payload["fixtureCount"], "peak_flow_l_s": payload["peakFlowLS"], "schedule_id": payload["scheduleId"]})
    return after, applied()


def _invert_create_water_system(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-water-system", {"id": payload["id"]})]


def delete_water_system(before, payload):
    """🧼️ `delete-water-system{id}` — Removes one cold-water use system. Nothing in the document references one, so this never refuses for use."""
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["water_systems"] = [row for row in after["model"]["water_systems"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_water_system(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["water_systems"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-water-system", {"index": index, "id": item["id"], "fixtureCount": item["fixture_count"], "peakFlowLS": item["peak_flow_l_s"], "scheduleId": item["schedule_id"]})]


def change_water_system_fixture_count(before, payload):
    """🪣️ `change-water-system-fixture-count{id,newFixtureCount}` — Sets how many fixtures one water system carries."""
    value = payload["newFixtureCount"]
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["fixture_count"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["water_systems"]:
        if row["id"] == payload["id"]:
            row["fixture_count"] = value
    return after, applied()


def _invert_change_water_system_fixture_count(before, payload):
    """↩️ The fixture_count the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["water_systems"] if row["id"] == payload["id"])
    return [("change-water-system-fixture-count", {"id": payload["id"], "newFixtureCount": item["fixture_count"]})]


def change_water_system_peak_flow(before, payload):
    """🚾️ `change-water-system-peak-flow{id,newPeakFlowLS}` — Sets peak flow (L/s) on one water system, addressed by id."""
    value = payload["newPeakFlowLS"]
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["peak_flow_l_s"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["water_systems"]:
        if row["id"] == payload["id"]:
            row["peak_flow_l_s"] = value
    return after, applied()


def _invert_change_water_system_peak_flow(before, payload):
    """↩️ The peak_flow_l_s the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["water_systems"] if row["id"] == payload["id"])
    return [("change-water-system-peak-flow", {"id": payload["id"], "newPeakFlowLS": item["peak_flow_l_s"]})]


def change_water_system_schedule(before, payload):
    """🕒️ `change-water-system-schedule{id,newScheduleId}` — Re-points one water system's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["water_systems"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["water_systems"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_water_system_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["water_systems"] if row["id"] == payload["id"])
    return [("change-water-system-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def create_fault(before, payload):
    """⚠️ `create-fault{index,…}` — Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it."""
    rows = before["model"]["faults"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["targetEquipmentId"] for row in before["model"]["ideal_loads"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["targetEquipmentId"])])
    if not any(row["id"] == payload["startScheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["startScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["faults"].insert(payload["index"], {"id": payload["id"], "target_equipment_id": payload["targetEquipmentId"], "fault_type": payload["faultType"], "severity": payload["severity"], "start_schedule_id": payload["startScheduleId"]})
    return after, applied()


def _invert_create_fault(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-fault", {"id": payload["id"]})]


def delete_fault(before, payload):
    """🩹️ `delete-fault{id}` — Removes one equipment fault, restoring the equipment it degraded to its rated behaviour. Nothing references a fault, so this never refuses for use."""
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["faults"] = [row for row in after["model"]["faults"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_fault(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["faults"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-fault", {"index": index, "id": item["id"], "targetEquipmentId": item["target_equipment_id"], "faultType": item["fault_type"], "severity": item["severity"], "startScheduleId": item["start_schedule_id"]})]


def change_fault_target_equipment(before, payload):
    """🎣️ `change-fault-target-equipment{id,newTargetEquipmentId}` — Re-points one fault at another ideal loads system; a target the document does not define is refused."""
    value = payload["newTargetEquipmentId"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["ideal_loads"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["target_equipment_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["target_equipment_id"] = value
    return after, applied()


def _invert_change_fault_target_equipment(before, payload):
    """↩️ The target_equipment_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-target-equipment", {"id": payload["id"], "newTargetEquipmentId": item["target_equipment_id"]})]


def change_fault_type(before, payload):
    """🐛️ `change-fault-type{id,newFaultType}` — Sets one fault's degradation mechanism."""
    value = payload["newFaultType"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if item["fault_type"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["fault_type"] = value
    return after, applied()


def _invert_change_fault_type(before, payload):
    """↩️ The fault_type the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-type", {"id": payload["id"], "newFaultType": item["fault_type"]})]


def change_fault_severity(before, payload):
    """🌶️ `change-fault-severity{id,newSeverity}` — Sets severity on one fault, addressed by id."""
    value = payload["newSeverity"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["severity"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["severity"] = value
    return after, applied()


def _invert_change_fault_severity(before, payload):
    """↩️ The severity the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-severity", {"id": payload["id"], "newSeverity": item["severity"]})]


def change_fault_start_schedule(before, payload):
    """🕓️ `change-fault-start-schedule{id,newStartScheduleId}` — Re-points one fault's schedule reference; a schedule the document does not define is refused."""
    value = payload["newStartScheduleId"]
    item = next((row for row in before["model"]["faults"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["start_schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["faults"]:
        if row["id"] == payload["id"]:
            row["start_schedule_id"] = value
    return after, applied()


def _invert_change_fault_start_schedule(before, payload):
    """↩️ The start_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["faults"] if row["id"] == payload["id"])
    return [("change-fault-start-schedule", {"id": payload["id"], "newStartScheduleId": item["start_schedule_id"]})]


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
    "create-refrigeration-system": create_refrigeration_system,
    "delete-refrigeration-system": delete_refrigeration_system,
    "change-refrigeration-system-case-count": change_refrigeration_system_case_count,
    "change-refrigeration-system-design-load": change_refrigeration_system_design_load,
    "change-refrigeration-system-defrost-schedule": change_refrigeration_system_defrost_schedule,
    "create-water-system": create_water_system,
    "delete-water-system": delete_water_system,
    "change-water-system-fixture-count": change_water_system_fixture_count,
    "change-water-system-peak-flow": change_water_system_peak_flow,
    "change-water-system-schedule": change_water_system_schedule,
    "create-fault": create_fault,
    "delete-fault": delete_fault,
    "change-fault-target-equipment": change_fault_target_equipment,
    "change-fault-type": change_fault_type,
    "change-fault-severity": change_fault_severity,
    "change-fault-start-schedule": change_fault_start_schedule,
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
    "create-refrigeration-system": _invert_create_refrigeration_system,
    "delete-refrigeration-system": _invert_delete_refrigeration_system,
    "change-refrigeration-system-case-count": _invert_change_refrigeration_system_case_count,
    "change-refrigeration-system-design-load": _invert_change_refrigeration_system_design_load,
    "change-refrigeration-system-defrost-schedule": _invert_change_refrigeration_system_defrost_schedule,
    "create-water-system": _invert_create_water_system,
    "delete-water-system": _invert_delete_water_system,
    "change-water-system-fixture-count": _invert_change_water_system_fixture_count,
    "change-water-system-peak-flow": _invert_change_water_system_peak_flow,
    "change-water-system-schedule": _invert_change_water_system_schedule,
    "create-fault": _invert_create_fault,
    "delete-fault": _invert_delete_fault,
    "change-fault-target-equipment": _invert_change_fault_target_equipment,
    "change-fault-type": _invert_change_fault_type,
    "change-fault-severity": _invert_change_fault_severity,
    "change-fault-start-schedule": _invert_change_fault_start_schedule,
}
# endregion 🔖️Vocabulary
