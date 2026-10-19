"""❄️ Refrigeration system, water system and equipment fault kinds."""

from __future__ import annotations

import copy

from mutate_energy_model_1_mutation_vocabulary import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
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
