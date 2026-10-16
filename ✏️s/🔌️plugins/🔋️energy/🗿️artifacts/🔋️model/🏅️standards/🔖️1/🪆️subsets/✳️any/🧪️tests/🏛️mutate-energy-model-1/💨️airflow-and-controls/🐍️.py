"""💨️ Airflow and control kinds: infiltration, leakage, ventilation, thermostats, humidistats and ideal loads."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-infiltration-applies": "shared://🧬️mutations/💨️infiltration/💨️create/✅️applies",
    "create-infiltration-refuses": "shared://🧬️mutations/💨️infiltration/💨️create/⛔️refuses",
    "delete-infiltration-applies": "shared://🧬️mutations/💨️infiltration/🧽️delete/✅️applies",
    "delete-infiltration-refuses": "shared://🧬️mutations/💨️infiltration/🧽️delete/⛔️refuses",
    "change-infiltration-zone-applies": "shared://🧬️mutations/💨️infiltration/🌀️change-zone/✅️applies",
    "change-infiltration-zone-refuses": "shared://🧬️mutations/💨️infiltration/🌀️change-zone/⛔️refuses",
    "change-infiltration-schedule-applies": "shared://🧬️mutations/💨️infiltration/⏳️change-schedule/✅️applies",
    "change-infiltration-schedule-refuses": "shared://🧬️mutations/💨️infiltration/⏳️change-schedule/⛔️refuses",
    "change-infiltration-flow-per-exterior-area-applies": "shared://🧬️mutations/💨️infiltration/🌫️change-flow-per-exterior-area/✅️applies",
    "change-infiltration-flow-per-exterior-area-refuses": "shared://🧬️mutations/💨️infiltration/🌫️change-flow-per-exterior-area/⛔️refuses",
    "change-infiltration-constant-term-coefficient-applies": "shared://🧬️mutations/🕳️leakage/🅰️change-constant-term-coefficient/✅️applies",
    "change-infiltration-constant-term-coefficient-refuses": "shared://🧬️mutations/🕳️leakage/🅰️change-constant-term-coefficient/⛔️refuses",
    "change-infiltration-temperature-term-coefficient-applies": "shared://🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/✅️applies",
    "change-infiltration-temperature-term-coefficient-refuses": "shared://🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/⛔️refuses",
    "change-infiltration-velocity-term-coefficient-applies": "shared://🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/✅️applies",
    "change-infiltration-velocity-term-coefficient-refuses": "shared://🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/⛔️refuses",
    "change-infiltration-velocity-squared-term-coefficient-applies": "shared://🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/✅️applies",
    "change-infiltration-velocity-squared-term-coefficient-refuses": "shared://🧬️mutations/🆑️change-infiltration-velocity-squared-term-coefficient/⛔️refuses",
    "create-mechanical-ventilation-applies": "shared://🧬️mutations/🌪️ventilation/🌪️create/✅️applies",
    "create-mechanical-ventilation-refuses": "shared://🧬️mutations/🌪️ventilation/🌪️create/⛔️refuses",
    "delete-mechanical-ventilation-applies": "shared://🧬️mutations/🌪️ventilation/🚫️delete/✅️applies",
    "delete-mechanical-ventilation-refuses": "shared://🧬️mutations/🌪️ventilation/🚫️delete/⛔️refuses",
    "change-mechanical-ventilation-zone-applies": "shared://🧬️mutations/🌪️ventilation/🧭️change-zone/✅️applies",
    "change-mechanical-ventilation-zone-refuses": "shared://🧬️mutations/🌪️ventilation/🧭️change-zone/⛔️refuses",
    "change-mechanical-ventilation-schedule-applies": "shared://🧬️mutations/🌪️ventilation/📆️change-schedule/✅️applies",
    "change-mechanical-ventilation-schedule-refuses": "shared://🧬️mutations/🌪️ventilation/📆️change-schedule/⛔️refuses",
    "change-mechanical-ventilation-design-flow-applies": "shared://🧬️mutations/🌪️ventilation/🚿️change-design-flow/✅️applies",
    "change-mechanical-ventilation-design-flow-refuses": "shared://🧬️mutations/🌪️ventilation/🚿️change-design-flow/⛔️refuses",
    "change-mechanical-ventilation-fan-total-efficiency-applies": "shared://🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/✅️applies",
    "change-mechanical-ventilation-fan-total-efficiency-refuses": "shared://🧬️mutations/💠️change-mechanical-ventilation-fan-total-efficiency/⛔️refuses",
    "change-mechanical-ventilation-fan-delta-pressure-applies": "shared://🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/✅️applies",
    "change-mechanical-ventilation-fan-delta-pressure-refuses": "shared://🧬️mutations/🎈️change-mechanical-ventilation-fan-delta-pressure/⛔️refuses",
    "change-infiltration-method-applies": "shared://🧬️mutations/💨️infiltration/🔬️change-method/✅️applies",
    "change-infiltration-method-refuses": "shared://🧬️mutations/💨️infiltration/🔬️change-method/⛔️refuses",
    "change-infiltration-design-flow-ach-applies": "shared://🧬️mutations/💨️infiltration/🔄️change-design-flow-ach/✅️applies",
    "change-infiltration-design-flow-ach-refuses": "shared://🧬️mutations/💨️infiltration/🔄️change-design-flow-ach/⛔️refuses",
    "change-infiltration-effective-leakage-area-applies": "shared://🧬️mutations/🕳️leakage/🕳️change-effective-leakage-area/✅️applies",
    "change-infiltration-effective-leakage-area-refuses": "shared://🧬️mutations/🕳️leakage/🕳️change-effective-leakage-area/⛔️refuses",
    "change-infiltration-discharge-coefficient-applies": "shared://🧬️mutations/🕳️leakage/🚰️change-discharge-coefficient/✅️applies",
    "change-infiltration-discharge-coefficient-refuses": "shared://🧬️mutations/🕳️leakage/🚰️change-discharge-coefficient/⛔️refuses",
    "change-infiltration-stack-height-applies": "shared://🧬️mutations/🕳️leakage/🏭️change-stack-height/✅️applies",
    "change-infiltration-stack-height-refuses": "shared://🧬️mutations/🕳️leakage/🏭️change-stack-height/⛔️refuses",
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
def create_infiltration(before, payload):
    """💨️ `create-infiltration{index,…}` — Adds one zone infiltration specification: the method plus every parameter each method needs, so the kernel maps the entity onto one `air_exchange::InfiltrationSpec` without pinning a method or smuggling the flow through a coefficient."""
    rows = before["model"]["infiltrations"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["infiltrations"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "method": payload["method"], "design_flow_ach": payload["designFlowAch"], "flow_per_exterior_area_m3_s_m2": payload["flowPerExteriorAreaM3SM2"], "effective_leakage_area_m2": payload["effectiveLeakageAreaM2"], "discharge_coefficient": payload["dischargeCoefficient"], "stack_height_m": payload["stackHeightM"], "constant_term_coefficient": payload["constantTermCoefficient"], "temperature_term_coefficient": payload["temperatureTermCoefficient"], "velocity_term_coefficient": payload["velocityTermCoefficient"], "velocity_squared_term_coefficient": payload["velocitySquaredTermCoefficient"]})
    return after, applied()


def _invert_create_infiltration(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-infiltration", {"id": payload["id"]})]


def delete_infiltration(before, payload):
    """🧽️ `delete-infiltration{id}` — Removes one zone infiltration specification, leaving the zone airtight."""
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["infiltrations"] = [row for row in after["model"]["infiltrations"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_infiltration(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["infiltrations"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-infiltration", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "method": item["method"], "designFlowAch": item["design_flow_ach"], "flowPerExteriorAreaM3SM2": item["flow_per_exterior_area_m3_s_m2"], "effectiveLeakageAreaM2": item["effective_leakage_area_m2"], "dischargeCoefficient": item["discharge_coefficient"], "stackHeightM": item["stack_height_m"], "constantTermCoefficient": item["constant_term_coefficient"], "temperatureTermCoefficient": item["temperature_term_coefficient"], "velocityTermCoefficient": item["velocity_term_coefficient"], "velocitySquaredTermCoefficient": item["velocity_squared_term_coefficient"]})]


def change_infiltration_zone(before, payload):
    """🌀️ `change-infiltration-zone{id,newZoneId}` — Re-points one infiltration's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_infiltration_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_infiltration_schedule(before, payload):
    """⏳️ `change-infiltration-schedule{id,newScheduleId}` — Re-points one infiltration's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_infiltration_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_infiltration_flow_per_exterior_area(before, payload):
    """🌫️ `change-infiltration-flow-per-exterior-area{id,newFlowPerExteriorAreaM3SM2}` — Sets flow per exterior area (m³/s·m²) on one infiltration, addressed by id."""
    value = payload["newFlowPerExteriorAreaM3SM2"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["flow_per_exterior_area_m3_s_m2"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["flow_per_exterior_area_m3_s_m2"] = value
    return after, applied()


def _invert_change_infiltration_flow_per_exterior_area(before, payload):
    """↩️ The flow_per_exterior_area_m3_s_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-flow-per-exterior-area", {"id": payload["id"], "newFlowPerExteriorAreaM3SM2": item["flow_per_exterior_area_m3_s_m2"]})]


def change_infiltration_constant_term_coefficient(before, payload):
    """🅰️ `change-infiltration-constant-term-coefficient{id,newConstantTermCoefficient}` — Sets EnergyPlus's `ZoneInfiltration:DesignFlowRate` constant term coefficient A. The four coefficients are non-negative by that object's own convention (BLAST 0.606/0.03636/0.1177/0, DOE-2 0/0/0.224/0)."""
    value = payload["newConstantTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["constant_term_coefficient"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["constant_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_constant_term_coefficient(before, payload):
    """↩️ The constant_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-constant-term-coefficient", {"id": payload["id"], "newConstantTermCoefficient": item["constant_term_coefficient"]})]


def change_infiltration_temperature_term_coefficient(before, payload):
    """🅱️ `change-infiltration-temperature-term-coefficient{id,newTemperatureTermCoefficient}` — Sets the temperature term coefficient B on one infiltration, addressed by id."""
    value = payload["newTemperatureTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["temperature_term_coefficient"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["temperature_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_temperature_term_coefficient(before, payload):
    """↩️ The temperature_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-temperature-term-coefficient", {"id": payload["id"], "newTemperatureTermCoefficient": item["temperature_term_coefficient"]})]


def change_infiltration_velocity_term_coefficient(before, payload):
    """🆎️ `change-infiltration-velocity-term-coefficient{id,newVelocityTermCoefficient}` — Sets the wind velocity term coefficient C on one infiltration, addressed by id."""
    value = payload["newVelocityTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["velocity_term_coefficient"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["velocity_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_velocity_term_coefficient(before, payload):
    """↩️ The velocity_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-velocity-term-coefficient", {"id": payload["id"], "newVelocityTermCoefficient": item["velocity_term_coefficient"]})]


def change_infiltration_velocity_squared_term_coefficient(before, payload):
    """🆑️ `change-infiltration-velocity-squared-term-coefficient{id,newVelocitySquaredTermCoefficient}` — Sets the squared wind velocity term coefficient D on one infiltration, addressed by id."""
    value = payload["newVelocitySquaredTermCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["velocity_squared_term_coefficient"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["velocity_squared_term_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_velocity_squared_term_coefficient(before, payload):
    """↩️ The velocity_squared_term_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-velocity-squared-term-coefficient", {"id": payload["id"], "newVelocitySquaredTermCoefficient": item["velocity_squared_term_coefficient"]})]


def create_mechanical_ventilation(before, payload):
    """🌪️ `create-mechanical-ventilation{index,…}` — Adds one mechanical ventilation specification to a zone: the design supply flow, its schedule and the fan work the supply air arrives with."""
    rows = before["model"]["mechanical_ventilations"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["mechanical_ventilations"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "design_flow_m3_s": payload["designFlowM3S"], "fan_total_efficiency": payload["fanTotalEfficiency"], "fan_delta_pressure_pa": payload["fanDeltaPressurePa"]})
    return after, applied()


def _invert_create_mechanical_ventilation(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-mechanical-ventilation", {"id": payload["id"]})]


def delete_mechanical_ventilation(before, payload):
    """🚫️ `delete-mechanical-ventilation{id}` — Removes one mechanical ventilation specification."""
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["mechanical_ventilations"] = [row for row in after["model"]["mechanical_ventilations"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_mechanical_ventilation(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["mechanical_ventilations"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-mechanical-ventilation", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "designFlowM3S": item["design_flow_m3_s"], "fanTotalEfficiency": item["fan_total_efficiency"], "fanDeltaPressurePa": item["fan_delta_pressure_pa"]})]


def change_mechanical_ventilation_zone(before, payload):
    """🧭️ `change-mechanical-ventilation-zone{id,newZoneId}` — Re-points one mechanical ventilation's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_mechanical_ventilation_schedule(before, payload):
    """📆️ `change-mechanical-ventilation-schedule{id,newScheduleId}` — Re-points one mechanical ventilation's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_mechanical_ventilation_design_flow(before, payload):
    """🚿️ `change-mechanical-ventilation-design-flow{id,newDesignFlowM3S}` — Sets design supply flow (m³/s) on one mechanical ventilation, addressed by id."""
    value = payload["newDesignFlowM3S"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_flow_m3_s"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["design_flow_m3_s"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_design_flow(before, payload):
    """↩️ The design_flow_m3_s the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-design-flow", {"id": payload["id"], "newDesignFlowM3S": item["design_flow_m3_s"]})]


def change_mechanical_ventilation_fan_total_efficiency(before, payload):
    """💠️ `change-mechanical-ventilation-fan-total-efficiency{id,newFanTotalEfficiency}` — Sets fan total efficiency on one mechanical ventilation, addressed by id."""
    value = payload["newFanTotalEfficiency"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 < value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["fan_total_efficiency"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["fan_total_efficiency"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_fan_total_efficiency(before, payload):
    """↩️ The fan_total_efficiency the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-fan-total-efficiency", {"id": payload["id"], "newFanTotalEfficiency": item["fan_total_efficiency"]})]


def change_mechanical_ventilation_fan_delta_pressure(before, payload):
    """🎈️ `change-mechanical-ventilation-fan-delta-pressure{id,newFanDeltaPressurePa}` — Sets fan pressure rise (Pa) on one mechanical ventilation, addressed by id."""
    value = payload["newFanDeltaPressurePa"]
    item = next((row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["fan_delta_pressure_pa"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["mechanical_ventilations"]:
        if row["id"] == payload["id"]:
            row["fan_delta_pressure_pa"] = value
    return after, applied()


def _invert_change_mechanical_ventilation_fan_delta_pressure(before, payload):
    """↩️ The fan_delta_pressure_pa the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["mechanical_ventilations"] if row["id"] == payload["id"])
    return [("change-mechanical-ventilation-fan-delta-pressure", {"id": payload["id"], "newFanDeltaPressurePa": item["fan_delta_pressure_pa"]})]


def change_infiltration_method(before, payload):
    """🔬️ `change-infiltration-method{id,newMethod}` — Selects which of `air_exchange::InfiltrationMethod`'s four flow calculations the kernel runs for one infiltration object. The parameters every method needs are already carried side by side, so switching the method never has to move data."""
    value = payload["newMethod"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if item["method"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["method"] = value
    return after, applied()


def _invert_change_infiltration_method(before, payload):
    """↩️ The method the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-method", {"id": payload["id"], "newMethod": item["method"]})]


def change_infiltration_design_flow_ach(before, payload):
    """🔄️ `change-infiltration-design-flow-ach{id,newDesignFlowAch}` — Sets design flow (air changes per hour) on one infiltration, addressed by id."""
    value = payload["newDesignFlowAch"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["design_flow_ach"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["design_flow_ach"] = value
    return after, applied()


def _invert_change_infiltration_design_flow_ach(before, payload):
    """↩️ The design_flow_ach the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-design-flow-ach", {"id": payload["id"], "newDesignFlowAch": item["design_flow_ach"]})]


def change_infiltration_effective_leakage_area(before, payload):
    """🕳️ `change-infiltration-effective-leakage-area{id,newEffectiveLeakageAreaM2}` — Sets effective leakage area (m²) on one infiltration, addressed by id."""
    value = payload["newEffectiveLeakageAreaM2"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["effective_leakage_area_m2"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["effective_leakage_area_m2"] = value
    return after, applied()


def _invert_change_infiltration_effective_leakage_area(before, payload):
    """↩️ The effective_leakage_area_m2 the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-effective-leakage-area", {"id": payload["id"], "newEffectiveLeakageAreaM2": item["effective_leakage_area_m2"]})]


def change_infiltration_discharge_coefficient(before, payload):
    """🚰️ `change-infiltration-discharge-coefficient{id,newDischargeCoefficient}` — Sets the orifice discharge coefficient on one infiltration, addressed by id."""
    value = payload["newDischargeCoefficient"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["discharge_coefficient"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["discharge_coefficient"] = value
    return after, applied()


def _invert_change_infiltration_discharge_coefficient(before, payload):
    """↩️ The discharge_coefficient the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-discharge-coefficient", {"id": payload["id"], "newDischargeCoefficient": item["discharge_coefficient"]})]


def change_infiltration_stack_height(before, payload):
    """🏭️ `change-infiltration-stack-height{id,newStackHeightM}` — Sets stack height (m) on one infiltration, addressed by id."""
    value = payload["newStackHeightM"]
    item = next((row for row in before["model"]["infiltrations"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["stack_height_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["infiltrations"]:
        if row["id"] == payload["id"]:
            row["stack_height_m"] = value
    return after, applied()


def _invert_change_infiltration_stack_height(before, payload):
    """↩️ The stack_height_m the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["infiltrations"] if row["id"] == payload["id"])
    return [("change-infiltration-stack-height", {"id": payload["id"], "newStackHeightM": item["stack_height_m"]})]


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
    "create-infiltration": create_infiltration,
    "delete-infiltration": delete_infiltration,
    "change-infiltration-zone": change_infiltration_zone,
    "change-infiltration-schedule": change_infiltration_schedule,
    "change-infiltration-flow-per-exterior-area": change_infiltration_flow_per_exterior_area,
    "change-infiltration-constant-term-coefficient": change_infiltration_constant_term_coefficient,
    "change-infiltration-temperature-term-coefficient": change_infiltration_temperature_term_coefficient,
    "change-infiltration-velocity-term-coefficient": change_infiltration_velocity_term_coefficient,
    "change-infiltration-velocity-squared-term-coefficient": change_infiltration_velocity_squared_term_coefficient,
    "create-mechanical-ventilation": create_mechanical_ventilation,
    "delete-mechanical-ventilation": delete_mechanical_ventilation,
    "change-mechanical-ventilation-zone": change_mechanical_ventilation_zone,
    "change-mechanical-ventilation-schedule": change_mechanical_ventilation_schedule,
    "change-mechanical-ventilation-design-flow": change_mechanical_ventilation_design_flow,
    "change-mechanical-ventilation-fan-total-efficiency": change_mechanical_ventilation_fan_total_efficiency,
    "change-mechanical-ventilation-fan-delta-pressure": change_mechanical_ventilation_fan_delta_pressure,
    "change-infiltration-method": change_infiltration_method,
    "change-infiltration-design-flow-ach": change_infiltration_design_flow_ach,
    "change-infiltration-effective-leakage-area": change_infiltration_effective_leakage_area,
    "change-infiltration-discharge-coefficient": change_infiltration_discharge_coefficient,
    "change-infiltration-stack-height": change_infiltration_stack_height,
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
    "create-infiltration": _invert_create_infiltration,
    "delete-infiltration": _invert_delete_infiltration,
    "change-infiltration-zone": _invert_change_infiltration_zone,
    "change-infiltration-schedule": _invert_change_infiltration_schedule,
    "change-infiltration-flow-per-exterior-area": _invert_change_infiltration_flow_per_exterior_area,
    "change-infiltration-constant-term-coefficient": _invert_change_infiltration_constant_term_coefficient,
    "change-infiltration-temperature-term-coefficient": _invert_change_infiltration_temperature_term_coefficient,
    "change-infiltration-velocity-term-coefficient": _invert_change_infiltration_velocity_term_coefficient,
    "change-infiltration-velocity-squared-term-coefficient": _invert_change_infiltration_velocity_squared_term_coefficient,
    "create-mechanical-ventilation": _invert_create_mechanical_ventilation,
    "delete-mechanical-ventilation": _invert_delete_mechanical_ventilation,
    "change-mechanical-ventilation-zone": _invert_change_mechanical_ventilation_zone,
    "change-mechanical-ventilation-schedule": _invert_change_mechanical_ventilation_schedule,
    "change-mechanical-ventilation-design-flow": _invert_change_mechanical_ventilation_design_flow,
    "change-mechanical-ventilation-fan-total-efficiency": _invert_change_mechanical_ventilation_fan_total_efficiency,
    "change-mechanical-ventilation-fan-delta-pressure": _invert_change_mechanical_ventilation_fan_delta_pressure,
    "change-infiltration-method": _invert_change_infiltration_method,
    "change-infiltration-design-flow-ach": _invert_change_infiltration_design_flow_ach,
    "change-infiltration-effective-leakage-area": _invert_change_infiltration_effective_leakage_area,
    "change-infiltration-discharge-coefficient": _invert_change_infiltration_discharge_coefficient,
    "change-infiltration-stack-height": _invert_change_infiltration_stack_height,
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
