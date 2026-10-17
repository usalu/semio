"""🧾️ Gain kinds: the people, lighting and equipment gains."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-people-gain-applies": "shared://🧬️mutations/👥️people-gain/👤️create/✅️applies",
    "create-people-gain-refuses": "shared://🧬️mutations/👥️people-gain/👤️create/⛔️refuses",
    "delete-people-gain-applies": "shared://🧬️mutations/👥️people-gain/🚷️delete/✅️applies",
    "delete-people-gain-refuses": "shared://🧬️mutations/👥️people-gain/🚷️delete/⛔️refuses",
    "change-people-gain-zone-applies": "shared://🧬️mutations/👥️people-gain/🚶️change-zone/✅️applies",
    "change-people-gain-zone-refuses": "shared://🧬️mutations/👥️people-gain/🚶️change-zone/⛔️refuses",
    "change-people-gain-schedule-applies": "shared://🧬️mutations/👥️people-gain/⏰️change-schedule/✅️applies",
    "change-people-gain-schedule-refuses": "shared://🧬️mutations/👥️people-gain/⏰️change-schedule/⛔️refuses",
    "change-people-gain-activity-schedule-applies": "shared://🧬️mutations/👥️people-gain/🏃️change-activity-schedule/✅️applies",
    "change-people-gain-activity-schedule-refuses": "shared://🧬️mutations/👥️people-gain/🏃️change-activity-schedule/⛔️refuses",
    "change-people-gain-people-per-area-applies": "shared://🧬️mutations/👥️people-gain/👥️change-people-per-area/✅️applies",
    "change-people-gain-people-per-area-refuses": "shared://🧬️mutations/👥️people-gain/👥️change-people-per-area/⛔️refuses",
    "change-people-gain-sensible-fraction-applies": "shared://🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/✅️applies",
    "change-people-gain-sensible-fraction-refuses": "shared://🧬️mutations/👥️people-gain/🌞️change-sensible-fraction/⛔️refuses",
    "change-people-gain-latent-fraction-applies": "shared://🧬️mutations/👥️people-gain/💧️change-latent-fraction/✅️applies",
    "change-people-gain-latent-fraction-refuses": "shared://🧬️mutations/👥️people-gain/💧️change-latent-fraction/⛔️refuses",
    "change-people-gain-radiant-fraction-applies": "shared://🧬️mutations/👥️people-gain/📡️change-radiant-fraction/✅️applies",
    "change-people-gain-radiant-fraction-refuses": "shared://🧬️mutations/👥️people-gain/📡️change-radiant-fraction/⛔️refuses",
    "create-lighting-gain-applies": "shared://🧬️mutations/💡️lighting-gain/💡️create/✅️applies",
    "create-lighting-gain-refuses": "shared://🧬️mutations/💡️lighting-gain/💡️create/⛔️refuses",
    "delete-lighting-gain-applies": "shared://🧬️mutations/💡️lighting-gain/🕯️delete/✅️applies",
    "delete-lighting-gain-refuses": "shared://🧬️mutations/💡️lighting-gain/🕯️delete/⛔️refuses",
    "change-lighting-gain-zone-applies": "shared://🧬️mutations/💡️lighting-gain/🔦️change-zone/✅️applies",
    "change-lighting-gain-zone-refuses": "shared://🧬️mutations/💡️lighting-gain/🔦️change-zone/⛔️refuses",
    "change-lighting-gain-schedule-applies": "shared://🧬️mutations/💡️lighting-gain/⏱️change-schedule/✅️applies",
    "change-lighting-gain-schedule-refuses": "shared://🧬️mutations/💡️lighting-gain/⏱️change-schedule/⛔️refuses",
    "change-lighting-gain-watts-per-area-applies": "shared://🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/✅️applies",
    "change-lighting-gain-watts-per-area-refuses": "shared://🧬️mutations/💡️lighting-gain/🔌️change-watts-per-area/⛔️refuses",
    "change-lighting-gain-radiant-fraction-applies": "shared://🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/✅️applies",
    "change-lighting-gain-radiant-fraction-refuses": "shared://🧬️mutations/💡️lighting-gain/🌟️change-radiant-fraction/⛔️refuses",
    "change-lighting-gain-visible-fraction-applies": "shared://🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/✅️applies",
    "change-lighting-gain-visible-fraction-refuses": "shared://🧬️mutations/💡️lighting-gain/🔅️change-visible-fraction/⛔️refuses",
    "change-lighting-gain-return-air-fraction-applies": "shared://🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/✅️applies",
    "change-lighting-gain-return-air-fraction-refuses": "shared://🧬️mutations/💡️lighting-gain/🎐️change-return-air-fraction/⛔️refuses",
    "create-equipment-gain-applies": "shared://🧬️mutations/🖥️equipment-gain/🖥️create/✅️applies",
    "create-equipment-gain-refuses": "shared://🧬️mutations/🖥️equipment-gain/🖥️create/⛔️refuses",
    "delete-equipment-gain-applies": "shared://🧬️mutations/🖥️equipment-gain/🧯️delete/✅️applies",
    "delete-equipment-gain-refuses": "shared://🧬️mutations/🖥️equipment-gain/🧯️delete/⛔️refuses",
    "change-equipment-gain-zone-applies": "shared://🧬️mutations/🖥️equipment-gain/🖨️change-zone/✅️applies",
    "change-equipment-gain-zone-refuses": "shared://🧬️mutations/🖥️equipment-gain/🖨️change-zone/⛔️refuses",
    "change-equipment-gain-schedule-applies": "shared://🧬️mutations/🖥️equipment-gain/⌛️change-schedule/✅️applies",
    "change-equipment-gain-schedule-refuses": "shared://🧬️mutations/🖥️equipment-gain/⌛️change-schedule/⛔️refuses",
    "change-equipment-gain-watts-per-area-applies": "shared://🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/✅️applies",
    "change-equipment-gain-watts-per-area-refuses": "shared://🧬️mutations/🖥️equipment-gain/⚡️change-watts-per-area/⛔️refuses",
    "change-equipment-gain-radiant-fraction-applies": "shared://🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/✅️applies",
    "change-equipment-gain-radiant-fraction-refuses": "shared://🧬️mutations/🖥️equipment-gain/🌠️change-radiant-fraction/⛔️refuses",
    "change-equipment-gain-latent-fraction-applies": "shared://🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/✅️applies",
    "change-equipment-gain-latent-fraction-refuses": "shared://🧬️mutations/🖥️equipment-gain/💦️change-latent-fraction/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_people_gain(before, payload):
    """👤️ `create-people-gain{index,…}` — Adds one occupancy gain to a zone: the occupant density, the occupancy schedule, the metabolic activity schedule and the sensible/latent/radiant split the zone heat balance needs."""
    rows = before["model"]["people"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    if not any(row["id"] == payload["activityScheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["activityScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["people"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "activity_schedule_id": payload["activityScheduleId"], "people_per_area": payload["peoplePerArea"], "sensible_fraction": payload["sensibleFraction"], "latent_fraction": payload["latentFraction"], "radiant_fraction": payload["radiantFraction"]})
    return after, applied()


def _invert_create_people_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-people-gain", {"id": payload["id"]})]


def delete_people_gain(before, payload):
    """🚷️ `delete-people-gain{id}` — Removes one occupancy gain. Nothing references a gain, so this never has to refuse for use."""
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["people"] = [row for row in after["model"]["people"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_people_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["people"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-people-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "activityScheduleId": item["activity_schedule_id"], "peoplePerArea": item["people_per_area"], "sensibleFraction": item["sensible_fraction"], "latentFraction": item["latent_fraction"], "radiantFraction": item["radiant_fraction"]})]


def change_people_gain_zone(before, payload):
    """🚶️ `change-people-gain-zone{id,newZoneId}` — Re-points one people gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_people_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_people_gain_schedule(before, payload):
    """⏰️ `change-people-gain-schedule{id,newScheduleId}` — Re-points one people gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_people_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_people_gain_activity_schedule(before, payload):
    """🏃️ `change-people-gain-activity-schedule{id,newActivityScheduleId}` — Re-points one people gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newActivityScheduleId"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["activity_schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["activity_schedule_id"] = value
    return after, applied()


def _invert_change_people_gain_activity_schedule(before, payload):
    """↩️ The activity_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-activity-schedule", {"id": payload["id"], "newActivityScheduleId": item["activity_schedule_id"]})]


def change_people_gain_people_per_area(before, payload):
    """👥️ `change-people-gain-people-per-area{id,newPeoplePerArea}` — Sets occupant density (people/m²) on one people gain, addressed by id."""
    value = payload["newPeoplePerArea"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["people_per_area"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["people_per_area"] = value
    return after, applied()


def _invert_change_people_gain_people_per_area(before, payload):
    """↩️ The people_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-people-per-area", {"id": payload["id"], "newPeoplePerArea": item["people_per_area"]})]


def change_people_gain_sensible_fraction(before, payload):
    """🌞️ `change-people-gain-sensible-fraction{id,newSensibleFraction}` — Sets sensible fraction on one people gain, addressed by id."""
    value = payload["newSensibleFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["sensible_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["sensible_fraction"] = value
    return after, applied()


def _invert_change_people_gain_sensible_fraction(before, payload):
    """↩️ The sensible_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-sensible-fraction", {"id": payload["id"], "newSensibleFraction": item["sensible_fraction"]})]


def change_people_gain_latent_fraction(before, payload):
    """💧️ `change-people-gain-latent-fraction{id,newLatentFraction}` — Sets latent fraction on one people gain, addressed by id."""
    value = payload["newLatentFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["latent_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["latent_fraction"] = value
    return after, applied()


def _invert_change_people_gain_latent_fraction(before, payload):
    """↩️ The latent_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-latent-fraction", {"id": payload["id"], "newLatentFraction": item["latent_fraction"]})]


def change_people_gain_radiant_fraction(before, payload):
    """📡️ `change-people-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one people gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["people"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["people"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_people_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["people"] if row["id"] == payload["id"])
    return [("change-people-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def create_lighting_gain(before, payload):
    """💡️ `create-lighting-gain{index,…}` — Adds one lighting gain to a zone: the installed power density, its schedule and the radiant/visible/return-air split that decides how much of it reaches the zone air directly."""
    rows = before["model"]["lighting"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["lighting"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "watts_per_area": payload["wattsPerArea"], "radiant_fraction": payload["radiantFraction"], "visible_fraction": payload["visibleFraction"], "return_air_fraction": payload["returnAirFraction"]})
    return after, applied()


def _invert_create_lighting_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-lighting-gain", {"id": payload["id"]})]


def delete_lighting_gain(before, payload):
    """🕯️ `delete-lighting-gain{id}` — Removes one lighting gain."""
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["lighting"] = [row for row in after["model"]["lighting"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_lighting_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["lighting"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-lighting-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "wattsPerArea": item["watts_per_area"], "radiantFraction": item["radiant_fraction"], "visibleFraction": item["visible_fraction"], "returnAirFraction": item["return_air_fraction"]})]


def change_lighting_gain_zone(before, payload):
    """🔦️ `change-lighting-gain-zone{id,newZoneId}` — Re-points one lighting gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_lighting_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_lighting_gain_schedule(before, payload):
    """⏱️ `change-lighting-gain-schedule{id,newScheduleId}` — Re-points one lighting gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_lighting_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_lighting_gain_watts_per_area(before, payload):
    """🔌️ `change-lighting-gain-watts-per-area{id,newWattsPerArea}` — Sets installed power density (W/m²) on one lighting gain, addressed by id."""
    value = payload["newWattsPerArea"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["watts_per_area"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["watts_per_area"] = value
    return after, applied()


def _invert_change_lighting_gain_watts_per_area(before, payload):
    """↩️ The watts_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-watts-per-area", {"id": payload["id"], "newWattsPerArea": item["watts_per_area"]})]


def change_lighting_gain_radiant_fraction(before, payload):
    """🌟️ `change-lighting-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one lighting gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def change_lighting_gain_visible_fraction(before, payload):
    """🔅️ `change-lighting-gain-visible-fraction{id,newVisibleFraction}` — Sets visible fraction on one lighting gain, addressed by id."""
    value = payload["newVisibleFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["visible_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["visible_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_visible_fraction(before, payload):
    """↩️ The visible_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-visible-fraction", {"id": payload["id"], "newVisibleFraction": item["visible_fraction"]})]


def change_lighting_gain_return_air_fraction(before, payload):
    """🎐️ `change-lighting-gain-return-air-fraction{id,newReturnAirFraction}` — Sets return-air fraction on one lighting gain, addressed by id."""
    value = payload["newReturnAirFraction"]
    item = next((row for row in before["model"]["lighting"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["return_air_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["lighting"]:
        if row["id"] == payload["id"]:
            row["return_air_fraction"] = value
    return after, applied()


def _invert_change_lighting_gain_return_air_fraction(before, payload):
    """↩️ The return_air_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["lighting"] if row["id"] == payload["id"])
    return [("change-lighting-gain-return-air-fraction", {"id": payload["id"], "newReturnAirFraction": item["return_air_fraction"]})]


def create_equipment_gain(before, payload):
    """🖥️ `create-equipment-gain{index,…}` — Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative."""
    rows = before["model"]["equipment"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if not any(row["id"] == payload["scheduleId"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["scheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["equipment"].insert(payload["index"], {"id": payload["id"], "zone_id": payload["zoneId"], "schedule_id": payload["scheduleId"], "watts_per_area": payload["wattsPerArea"], "radiant_fraction": payload["radiantFraction"], "latent_fraction": payload["latentFraction"]})
    return after, applied()


def _invert_create_equipment_gain(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-equipment-gain", {"id": payload["id"]})]


def delete_equipment_gain(before, payload):
    """🧯️ `delete-equipment-gain{id}` — Removes one electric equipment gain."""
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["equipment"] = [row for row in after["model"]["equipment"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_equipment_gain(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["equipment"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-equipment-gain", {"index": index, "id": item["id"], "zoneId": item["zone_id"], "scheduleId": item["schedule_id"], "wattsPerArea": item["watts_per_area"], "radiantFraction": item["radiant_fraction"], "latentFraction": item["latent_fraction"]})]


def change_equipment_gain_zone(before, payload):
    """🖨️ `change-equipment-gain-zone{id,newZoneId}` — Re-points one equipment gain's zone reference; a zone the document does not define is refused."""
    value = payload["newZoneId"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["zone_id"] = value
    return after, applied()


def _invert_change_equipment_gain_zone(before, payload):
    """↩️ The zone_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_equipment_gain_schedule(before, payload):
    """⌛️ `change-equipment-gain-schedule{id,newScheduleId}` — Re-points one equipment gain's schedule reference; a schedule the document does not define is refused."""
    value = payload["newScheduleId"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["schedule_id"] = value
    return after, applied()


def _invert_change_equipment_gain_schedule(before, payload):
    """↩️ The schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-schedule", {"id": payload["id"], "newScheduleId": item["schedule_id"]})]


def change_equipment_gain_watts_per_area(before, payload):
    """⚡️ `change-equipment-gain-watts-per-area{id,newWattsPerArea}` — Sets equipment power density (W/m²) on one equipment gain, addressed by id."""
    value = payload["newWattsPerArea"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")) or value < 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["watts_per_area"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["watts_per_area"] = value
    return after, applied()


def _invert_change_equipment_gain_watts_per_area(before, payload):
    """↩️ The watts_per_area the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-watts-per-area", {"id": payload["id"], "newWattsPerArea": item["watts_per_area"]})]


def change_equipment_gain_radiant_fraction(before, payload):
    """🌠️ `change-equipment-gain-radiant-fraction{id,newRadiantFraction}` — Sets radiant fraction on one equipment gain, addressed by id."""
    value = payload["newRadiantFraction"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["radiant_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["radiant_fraction"] = value
    return after, applied()


def _invert_change_equipment_gain_radiant_fraction(before, payload):
    """↩️ The radiant_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-radiant-fraction", {"id": payload["id"], "newRadiantFraction": item["radiant_fraction"]})]


def change_equipment_gain_latent_fraction(before, payload):
    """💦️ `change-equipment-gain-latent-fraction{id,newLatentFraction}` — Sets latent fraction on one equipment gain, addressed by id."""
    value = payload["newLatentFraction"]
    item = next((row for row in before["model"]["equipment"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not 0.0 <= value <= 1.0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["latent_fraction"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["equipment"]:
        if row["id"] == payload["id"]:
            row["latent_fraction"] = value
    return after, applied()


def _invert_change_equipment_gain_latent_fraction(before, payload):
    """↩️ The latent_fraction the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["equipment"] if row["id"] == payload["id"])
    return [("change-equipment-gain-latent-fraction", {"id": payload["id"], "newLatentFraction": item["latent_fraction"]})]

VOCABULARY = {
    "create-people-gain": create_people_gain,
    "delete-people-gain": delete_people_gain,
    "change-people-gain-zone": change_people_gain_zone,
    "change-people-gain-schedule": change_people_gain_schedule,
    "change-people-gain-activity-schedule": change_people_gain_activity_schedule,
    "change-people-gain-people-per-area": change_people_gain_people_per_area,
    "change-people-gain-sensible-fraction": change_people_gain_sensible_fraction,
    "change-people-gain-latent-fraction": change_people_gain_latent_fraction,
    "change-people-gain-radiant-fraction": change_people_gain_radiant_fraction,
    "create-lighting-gain": create_lighting_gain,
    "delete-lighting-gain": delete_lighting_gain,
    "change-lighting-gain-zone": change_lighting_gain_zone,
    "change-lighting-gain-schedule": change_lighting_gain_schedule,
    "change-lighting-gain-watts-per-area": change_lighting_gain_watts_per_area,
    "change-lighting-gain-radiant-fraction": change_lighting_gain_radiant_fraction,
    "change-lighting-gain-visible-fraction": change_lighting_gain_visible_fraction,
    "change-lighting-gain-return-air-fraction": change_lighting_gain_return_air_fraction,
    "create-equipment-gain": create_equipment_gain,
    "delete-equipment-gain": delete_equipment_gain,
    "change-equipment-gain-zone": change_equipment_gain_zone,
    "change-equipment-gain-schedule": change_equipment_gain_schedule,
    "change-equipment-gain-watts-per-area": change_equipment_gain_watts_per_area,
    "change-equipment-gain-radiant-fraction": change_equipment_gain_radiant_fraction,
    "change-equipment-gain-latent-fraction": change_equipment_gain_latent_fraction,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-people-gain": _invert_create_people_gain,
    "delete-people-gain": _invert_delete_people_gain,
    "change-people-gain-zone": _invert_change_people_gain_zone,
    "change-people-gain-schedule": _invert_change_people_gain_schedule,
    "change-people-gain-activity-schedule": _invert_change_people_gain_activity_schedule,
    "change-people-gain-people-per-area": _invert_change_people_gain_people_per_area,
    "change-people-gain-sensible-fraction": _invert_change_people_gain_sensible_fraction,
    "change-people-gain-latent-fraction": _invert_change_people_gain_latent_fraction,
    "change-people-gain-radiant-fraction": _invert_change_people_gain_radiant_fraction,
    "create-lighting-gain": _invert_create_lighting_gain,
    "delete-lighting-gain": _invert_delete_lighting_gain,
    "change-lighting-gain-zone": _invert_change_lighting_gain_zone,
    "change-lighting-gain-schedule": _invert_change_lighting_gain_schedule,
    "change-lighting-gain-watts-per-area": _invert_change_lighting_gain_watts_per_area,
    "change-lighting-gain-radiant-fraction": _invert_change_lighting_gain_radiant_fraction,
    "change-lighting-gain-visible-fraction": _invert_change_lighting_gain_visible_fraction,
    "change-lighting-gain-return-air-fraction": _invert_change_lighting_gain_return_air_fraction,
    "create-equipment-gain": _invert_create_equipment_gain,
    "delete-equipment-gain": _invert_delete_equipment_gain,
    "change-equipment-gain-zone": _invert_change_equipment_gain_zone,
    "change-equipment-gain-schedule": _invert_change_equipment_gain_schedule,
    "change-equipment-gain-watts-per-area": _invert_change_equipment_gain_watts_per_area,
    "change-equipment-gain-radiant-fraction": _invert_change_equipment_gain_radiant_fraction,
    "change-equipment-gain-latent-fraction": _invert_change_equipment_gain_latent_fraction,
}
# endregion 🔖️Vocabulary
