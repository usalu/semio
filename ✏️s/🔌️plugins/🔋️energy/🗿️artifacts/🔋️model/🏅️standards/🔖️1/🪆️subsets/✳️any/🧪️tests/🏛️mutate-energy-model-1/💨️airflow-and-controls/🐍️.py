"""💨️ Airflow and control kinds: infiltration, leakage, ventilation, thermostats, humidistats and ideal loads."""

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


c1 = _load("mutate_energy_model_1_airflow_thermostats", "🌡️thermostats-and-humidistats")
c2 = _load("mutate_energy_model_1_airflow_ideal_loads", "💯️ideal-loads")


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
    "change-infiltration-temperature-term-coefficient-applies": "shared://🧬️mutations/🕳️leakage/🅱️change-temperature-term-coefficient/✅️applies",
    "change-infiltration-temperature-term-coefficient-refuses": "shared://🧬️mutations/🕳️leakage/🅱️change-temperature-term-coefficient/⛔️refuses",
    "change-infiltration-velocity-term-coefficient-applies": "shared://🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/✅️applies",
    "change-infiltration-velocity-term-coefficient-refuses": "shared://🧬️mutations/🕳️leakage/🆎️change-velocity-term-coefficient/⛔️refuses",
    "change-infiltration-velocity-squared-term-coefficient-applies": "shared://🧬️mutations/🕳️leakage/🆑️change-velocity-squared-term-coefficient/✅️applies",
    "change-infiltration-velocity-squared-term-coefficient-refuses": "shared://🧬️mutations/🕳️leakage/🆑️change-velocity-squared-term-coefficient/⛔️refuses",
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
    "change-mechanical-ventilation-fan-total-efficiency-applies": "shared://🧬️mutations/🌪️ventilation/💠️change-fan-total-efficiency/✅️applies",
    "change-mechanical-ventilation-fan-total-efficiency-refuses": "shared://🧬️mutations/🌪️ventilation/💠️change-fan-total-efficiency/⛔️refuses",
    "change-mechanical-ventilation-fan-delta-pressure-applies": "shared://🧬️mutations/🌪️ventilation/🎈️change-fan-delta-pressure/✅️applies",
    "change-mechanical-ventilation-fan-delta-pressure-refuses": "shared://🧬️mutations/🌪️ventilation/🎈️change-fan-delta-pressure/⛔️refuses",
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
    **c1.VECTOR_ROOTS,
    **c2.VECTOR_ROOTS,
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
    **c1.VOCABULARY,
    **c2.VOCABULARY,
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
    **c1.EXTRA_INVERT,
    **c2.EXTRA_INVERT,
}
# endregion 🔖️Vocabulary
