"""📆️ Schedule and layer kinds: space lists, enclosures, schedules, time series, window layers and finishes."""

from __future__ import annotations

import copy

from mutate_energy_model_1_base import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "create-space-list-applies": "shared://🧬️mutations/📋️space-list/📋️create/✅️applies",
    "create-space-list-refuses": "shared://🧬️mutations/📋️space-list/📋️create/⛔️refuses",
    "delete-space-list-applies": "shared://🧬️mutations/📋️space-list/🗒️delete/✅️applies",
    "delete-space-list-refuses": "shared://🧬️mutations/📋️space-list/🗒️delete/⛔️refuses",
    "rename-space-list-applies": "shared://🧬️mutations/📋️space-list/🪶️rename/✅️applies",
    "rename-space-list-refuses": "shared://🧬️mutations/📋️space-list/🪶️rename/⛔️refuses",
    "add-space-list-member-applies": "shared://🧬️mutations/📋️space-list/➡️add-member/✅️applies",
    "add-space-list-member-refuses": "shared://🧬️mutations/📋️space-list/➡️add-member/⛔️refuses",
    "remove-space-list-member-applies": "shared://🧬️mutations/📋️space-list/🪤️remove-member/✅️applies",
    "remove-space-list-member-refuses": "shared://🧬️mutations/📋️space-list/🪤️remove-member/⛔️refuses",
    "create-thermal-enclosure-applies": "shared://🧬️mutations/🔒️enclosure/🏟️create/✅️applies",
    "create-thermal-enclosure-refuses": "shared://🧬️mutations/🔒️enclosure/🏟️create/⛔️refuses",
    "delete-thermal-enclosure-applies": "shared://🧬️mutations/🔒️enclosure/🏯️delete/✅️applies",
    "delete-thermal-enclosure-refuses": "shared://🧬️mutations/🔒️enclosure/🏯️delete/⛔️refuses",
    "rename-thermal-enclosure-applies": "shared://🧬️mutations/🔒️enclosure/🖋️rename/✅️applies",
    "rename-thermal-enclosure-refuses": "shared://🧬️mutations/🔒️enclosure/🖋️rename/⛔️refuses",
    "add-thermal-enclosure-zone-applies": "shared://🧬️mutations/🔒️enclosure/🔒️add-zone/✅️applies",
    "add-thermal-enclosure-zone-refuses": "shared://🧬️mutations/🔒️enclosure/🔒️add-zone/⛔️refuses",
    "remove-thermal-enclosure-zone-applies": "shared://🧬️mutations/🔒️enclosure/🔓️remove-zone/✅️applies",
    "remove-thermal-enclosure-zone-refuses": "shared://🧬️mutations/🔒️enclosure/🔓️remove-zone/⛔️refuses",
    "create-constant-schedule-applies": "shared://🧬️mutations/🕐️fixed-schedule/🕜️create/✅️applies",
    "create-constant-schedule-refuses": "shared://🧬️mutations/🕐️fixed-schedule/🕜️create/⛔️refuses",
    "delete-constant-schedule-applies": "shared://🧬️mutations/🕐️fixed-schedule/📍️delete/✅️applies",
    "delete-constant-schedule-refuses": "shared://🧬️mutations/🕐️fixed-schedule/📍️delete/⛔️refuses",
    "change-constant-schedule-value-applies": "shared://🧬️mutations/🕐️fixed-schedule/🕝️change-value/✅️applies",
    "change-constant-schedule-value-refuses": "shared://🧬️mutations/🕐️fixed-schedule/🕝️change-value/⛔️refuses",
    "create-daily-schedule-applies": "shared://🧬️mutations/🕒️day-schedule/🕞️create/✅️applies",
    "create-daily-schedule-refuses": "shared://🧬️mutations/🕒️day-schedule/🕞️create/⛔️refuses",
    "delete-daily-schedule-applies": "shared://🧬️mutations/🕒️day-schedule/🌓️delete/✅️applies",
    "delete-daily-schedule-refuses": "shared://🧬️mutations/🕒️day-schedule/🌓️delete/⛔️refuses",
    "replace-daily-schedule-hourly-values-applies": "shared://🧬️mutations/🕒️day-schedule/🕔️replace-hourly-values/✅️applies",
    "replace-daily-schedule-hourly-values-refuses": "shared://🧬️mutations/🕒️day-schedule/🕔️replace-hourly-values/⛔️refuses",
    "change-daily-schedule-interpolation-applies": "shared://🧬️mutations/🕒️day-schedule/🕕️change-interpolation/✅️applies",
    "change-daily-schedule-interpolation-refuses": "shared://🧬️mutations/🕒️day-schedule/🕕️change-interpolation/⛔️refuses",
    "change-daily-schedule-limits-applies": "shared://🧬️mutations/🕒️day-schedule/🕟️change-limits/✅️applies",
    "change-daily-schedule-limits-refuses": "shared://🧬️mutations/🕒️day-schedule/🕟️change-limits/⛔️refuses",
    "create-weekly-schedule-applies": "shared://🧬️mutations/🗓️week-schedule/🗓️create/✅️applies",
    "create-weekly-schedule-refuses": "shared://🧬️mutations/🗓️week-schedule/🗓️create/⛔️refuses",
    "delete-weekly-schedule-applies": "shared://🧬️mutations/🗓️week-schedule/🕖️delete/✅️applies",
    "delete-weekly-schedule-refuses": "shared://🧬️mutations/🗓️week-schedule/🕖️delete/⛔️refuses",
    "change-weekly-schedule-day-applies": "shared://🧬️mutations/🗓️week-schedule/🕗️change-day/✅️applies",
    "change-weekly-schedule-day-refuses": "shared://🧬️mutations/🗓️week-schedule/🕗️change-day/⛔️refuses",
    "create-annual-schedule-applies": "shared://🧬️mutations/📆️year-schedule/📚️create/✅️applies",
    "create-annual-schedule-refuses": "shared://🧬️mutations/📆️year-schedule/📚️create/⛔️refuses",
    "delete-annual-schedule-applies": "shared://🧬️mutations/📆️year-schedule/📕️delete/✅️applies",
    "delete-annual-schedule-refuses": "shared://🧬️mutations/📆️year-schedule/📕️delete/⛔️refuses",
    "insert-annual-schedule-rule-applies": "shared://🧬️mutations/📆️year-schedule/📗️insert-rule/✅️applies",
    "insert-annual-schedule-rule-refuses": "shared://🧬️mutations/📆️year-schedule/📗️insert-rule/⛔️refuses",
    "remove-annual-schedule-rule-applies": "shared://🧬️mutations/📆️year-schedule/📙️remove-rule/✅️applies",
    "remove-annual-schedule-rule-refuses": "shared://🧬️mutations/📆️year-schedule/📙️remove-rule/⛔️refuses",
    "reorder-annual-schedule-rules-applies": "shared://🧬️mutations/📆️year-schedule/🗂️reorder-rules/✅️applies",
    "reorder-annual-schedule-rules-refuses": "shared://🧬️mutations/📆️year-schedule/🗂️reorder-rules/⛔️refuses",
    "change-annual-schedule-default-daily-schedule-applies": "shared://🧬️mutations/📆️year-schedule/🎌️change-default-daily-schedule/✅️applies",
    "change-annual-schedule-default-daily-schedule-refuses": "shared://🧬️mutations/📆️year-schedule/🎌️change-default-daily-schedule/⛔️refuses",
    "change-annual-schedule-holiday-daily-schedule-applies": "shared://🧬️mutations/📆️year-schedule/🎄️change-holiday-daily-schedule/✅️applies",
    "change-annual-schedule-holiday-daily-schedule-refuses": "shared://🧬️mutations/📆️year-schedule/🎄️change-holiday-daily-schedule/⛔️refuses",
    "add-annual-schedule-holiday-applies": "shared://🧬️mutations/📆️year-schedule/🎉️add-holiday/✅️applies",
    "add-annual-schedule-holiday-refuses": "shared://🧬️mutations/📆️year-schedule/🎉️add-holiday/⛔️refuses",
    "remove-annual-schedule-holiday-applies": "shared://🧬️mutations/📆️year-schedule/🎊️remove-holiday/✅️applies",
    "remove-annual-schedule-holiday-refuses": "shared://🧬️mutations/📆️year-schedule/🎊️remove-holiday/⛔️refuses",
    "create-time-series-schedule-applies": "shared://🧬️mutations/📈️time-series/🪗️create/✅️applies",
    "create-time-series-schedule-refuses": "shared://🧬️mutations/📈️time-series/🪗️create/⛔️refuses",
    "delete-time-series-schedule-applies": "shared://🧬️mutations/📈️time-series/🎞️delete/✅️applies",
    "delete-time-series-schedule-refuses": "shared://🧬️mutations/📈️time-series/🎞️delete/⛔️refuses",
    "replace-time-series-schedule-values-applies": "shared://🧬️mutations/📈️time-series/🕘️replace-values/✅️applies",
    "replace-time-series-schedule-values-refuses": "shared://🧬️mutations/📈️time-series/🕘️replace-values/⛔️refuses",
    "change-time-series-schedule-timestep-applies": "shared://🧬️mutations/📈️time-series/🕙️change-timestep/✅️applies",
    "change-time-series-schedule-timestep-refuses": "shared://🧬️mutations/📈️time-series/🕙️change-timestep/⛔️refuses",
    "replace-fenestration-vertices-shapes-a-gable": "shared://🧬️mutations/🪟️fenestration/🔶️replace-vertices/✅️shapes",
    "replace-fenestration-vertices-refuses-a-line": "shared://🧬️mutations/🪟️fenestration/🔶️replace-vertices/⛔️refuses",
    "change-glazing-material-thickness-applies": "shared://🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/✅️applies",
    "change-glazing-material-thickness-refuses": "shared://🧬️mutations/🥂️window-layer/🔷️change-glazing-thickness/⛔️refuses",
    "change-glazing-material-conductivity-applies": "shared://🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/✅️applies",
    "change-glazing-material-conductivity-refuses": "shared://🧬️mutations/🥂️window-layer/🟠️change-glazing-conductivity/⛔️refuses",
    "change-glazing-material-solar-transmittance-applies": "shared://🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/✅️applies",
    "change-glazing-material-solar-transmittance-refuses": "shared://🧬️mutations/🥂️window-layer/🟡️change-glazing-solar-transmittance/⛔️refuses",
    "change-glazing-material-visible-transmittance-applies": "shared://🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/✅️applies",
    "change-glazing-material-visible-transmittance-refuses": "shared://🧬️mutations/🥂️window-layer/🥽️change-glazing-visible-transmittance/⛔️refuses",
    "change-glazing-material-infrared-emissivity-applies": "shared://🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/✅️applies",
    "change-glazing-material-infrared-emissivity-refuses": "shared://🧬️mutations/🥂️window-layer/🩻️change-glazing-infrared-emissivity/⛔️refuses",
    "rename-glazing-material-applies": "shared://🧬️mutations/🥂️window-layer/🟢️rename-glazing/✅️applies",
    "rename-glazing-material-refuses": "shared://🧬️mutations/🥂️window-layer/🟢️rename-glazing/⛔️refuses",
    "change-gas-material-thickness-applies": "shared://🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/✅️applies",
    "change-gas-material-thickness-refuses": "shared://🧬️mutations/🥂️window-layer/🟣️change-gas-thickness/⛔️refuses",
    "change-gas-material-gas-applies": "shared://🧬️mutations/🥂️window-layer/🟤️change-gas-type/✅️applies",
    "change-gas-material-gas-refuses": "shared://🧬️mutations/🥂️window-layer/🟤️change-gas-type/⛔️refuses",
    "rename-gas-material-applies": "shared://🧬️mutations/🥂️window-layer/🔘️rename-gas/✅️applies",
    "rename-gas-material-refuses": "shared://🧬️mutations/🥂️window-layer/🔘️rename-gas/⛔️refuses",
    "change-material-roughness-applies": "shared://🧬️mutations/🎨️finish/🗻️change-roughness/✅️applies",
    "change-material-roughness-refuses": "shared://🧬️mutations/🎨️finish/🗻️change-roughness/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def create_space_list(before, payload):
    """📋️ `create-space-list{index,…}` — Adds one named grouping of spaces — the handle a report or an assignment addresses many spaces by at once."""
    rows = before["model"]["space_lists"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    missing = next((candidate for candidate in payload["spaceIds"] if not any(row["id"] == candidate for row in before["model"]["spaces"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["space_lists"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "space_ids": payload["spaceIds"]})
    return after, applied()


def _invert_create_space_list(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-space-list", {"id": payload["id"]})]


def delete_space_list(before, payload):
    """🗒️ `delete-space-list{id}` — Removes one space grouping. The spaces themselves are untouched — a list owns membership, not the members."""
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["space_lists"] = [row for row in after["model"]["space_lists"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_space_list(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["space_lists"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-space-list", {"index": index, "id": item["id"], "name": item["name"], "spaceIds": item["space_ids"]})]


def rename_space_list(before, payload):
    """🪶️ `rename-space-list{id,newName}` — Sets one space list's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["space_lists"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["space_lists"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_space_list(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["space_lists"] if row["id"] == payload["id"])
    return [("rename-space-list", {"id": payload["id"], "newName": item["name"]})]


def add_space_list_member(before, payload):
    """➡️ `add-space-list-member{id,index,spaceId}` — Puts one space into a grouping."""
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["spaceId"] for row in before["model"]["spaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["spaceId"])])
    if payload["index"] > len(item["space_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["spaceId"] in item["space_ids"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["space_lists"]:
        if row["id"] == payload["id"]:
            row["space_ids"].insert(payload["index"], payload["spaceId"])
    return after, applied()


def _invert_add_space_list_member(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-space-list-member", {"id": payload["id"], "spaceId": payload["spaceId"]})]


def remove_space_list_member(before, payload):
    """🪤️ `remove-space-list-member{id,spaceId}` — Takes one space back out of a grouping; the space itself survives."""
    item = next((row for row in before["model"]["space_lists"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["spaceId"] not in item["space_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["spaceId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["space_lists"]:
        if row["id"] == payload["id"]:
            row["space_ids"] = [candidate for candidate in row["space_ids"] if candidate != payload["spaceId"]]
    return after, applied()


def _invert_remove_space_list_member(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["space_lists"] if row["id"] == payload["id"])
    return [("add-space-list-member", {"id": payload["id"], "index": item["space_ids"].index(payload["spaceId"]), "spaceId": payload["spaceId"]})]


def create_thermal_enclosure(before, payload):
    """🏟️ `create-thermal-enclosure{index,…}` — Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over."""
    rows = before["model"]["thermal_enclosures"]
    if any(row["id"] == payload["id"] for row in rows):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    missing = next((candidate for candidate in payload["zoneIds"] if not any(row["id"] == candidate for row in before["model"]["zones"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["thermal_enclosures"].insert(payload["index"], {"id": payload["id"], "name": payload["name"], "zone_ids": payload["zoneIds"]})
    return after, applied()


def _invert_create_thermal_enclosure(before, payload):
    """↩️ Creation is undone by deleting exactly the element it inserted."""
    return [("delete-thermal-enclosure", {"id": payload["id"]})]


def delete_thermal_enclosure(before, payload):
    """🏯️ `delete-thermal-enclosure{id}` — Removes one thermal enclosure. The zones themselves are untouched — an enclosure owns membership, not the members."""
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["thermal_enclosures"] = [row for row in after["model"]["thermal_enclosures"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_thermal_enclosure(before, payload):
    """↩️ Re-creates the removed element at the index it actually held."""
    rows = before["model"]["thermal_enclosures"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-thermal-enclosure", {"index": index, "id": item["id"], "name": item["name"], "zoneIds": item["zone_ids"]})]


def rename_thermal_enclosure(before, payload):
    """🖋️ `rename-thermal-enclosure{id,newName}` — Sets one thermal enclosure's identity field; a name a sibling already holds is refused."""
    value = payload["newName"]
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(row["id"] != payload["id"] and row["name"] == value for row in before["model"]["thermal_enclosures"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["thermal_enclosures"]:
        if row["id"] == payload["id"]:
            row["name"] = value
    return after, applied()


def _invert_rename_thermal_enclosure(before, payload):
    """↩️ The name the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"])
    return [("rename-thermal-enclosure", {"id": payload["id"], "newName": item["name"]})]


def add_thermal_enclosure_zone(before, payload):
    """🔒️ `add-thermal-enclosure-zone{id,index,zoneId}` — Puts one zone inside a thermal enclosure."""
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    if payload["index"] > len(item["zone_ids"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["zoneId"] in item["zone_ids"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["thermal_enclosures"]:
        if row["id"] == payload["id"]:
            row["zone_ids"].insert(payload["index"], payload["zoneId"])
    return after, applied()


def _invert_add_thermal_enclosure_zone(before, payload):
    """↩️ The added member is taken back out; the owner keeps every other member in place."""
    return [("remove-thermal-enclosure-zone", {"id": payload["id"], "zoneId": payload["zoneId"]})]


def remove_thermal_enclosure_zone(before, payload):
    """🔓️ `remove-thermal-enclosure-zone{id,zoneId}` — Takes one zone back out of a thermal enclosure; the zone itself survives."""
    item = next((row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["zoneId"] not in item["zone_ids"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["zoneId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["thermal_enclosures"]:
        if row["id"] == payload["id"]:
            row["zone_ids"] = [candidate for candidate in row["zone_ids"] if candidate != payload["zoneId"]]
    return after, applied()


def _invert_remove_thermal_enclosure_zone(before, payload):
    """↩️ The member goes back at the position it actually held in BASE, not at the end."""
    item = next(row for row in before["model"]["thermal_enclosures"] if row["id"] == payload["id"])
    return [("add-thermal-enclosure-zone", {"id": payload["id"], "index": item["zone_ids"].index(payload["zoneId"]), "zoneId": payload["zoneId"]})]


def create_constant_schedule(before, payload):
    """🕜️ `create-constant-schedule{index,…}` — Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes."""
    rows = before["model"]["schedules"]["constants"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["value"] != payload["value"] or payload["value"] in (float("inf"), float("-inf")):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["constants"].insert(payload["index"], {"id": payload["id"], "value": payload["value"]})
    return after, applied()


def _invert_create_constant_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-constant-schedule", {"id": payload["id"]})]


def delete_constant_schedule(before, payload):
    """📍️ `delete-constant-schedule{id}` — Removes one constant schedule. Refused while any gain, thermostat, system or other schedule still resolves its id — the document would otherwise carry a reference `Model::validate` reports as severe."""
    item = next((row for row in before["model"]["schedules"]["constants"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["constants"] = [row for row in after["model"]["schedules"]["constants"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_constant_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["constants"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-constant-schedule", {"index": index, "id": item["id"], "value": item["value"]})]


def change_constant_schedule_value(before, payload):
    """🕝️ `change-constant-schedule-value{id,newValue}` — Sets the one value a constant schedule returns at every timestep."""
    value = payload["newValue"]
    item = next((row for row in before["model"]["schedules"]["constants"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value != value or value in (float("inf"), float("-inf")):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["value"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["constants"]:
        if row["id"] == payload["id"]:
            row["value"] = value
    return after, applied()


def _invert_change_constant_schedule_value(before, payload):
    """↩️ The value the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["constants"] if row["id"] == payload["id"])
    return [("change-constant-schedule-value", {"id": payload["id"], "newValue": item["value"]})]


def create_daily_schedule(before, payload):
    """🕞️ `create-daily-schedule{index,…}` — Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied."""
    rows = before["model"]["schedules"]["daily"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if len(payload["hourlyValues"]) != 24:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in payload["hourlyValues"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if (payload["limitsMin"] is None) != (payload["limitsMax"] is None):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["limitsMin"] is not None and payload["limitsMin"] > payload["limitsMax"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])], invariant="limits-ordered")
    after = copy.deepcopy(before)
    after["model"]["schedules"]["daily"].insert(payload["index"], {"id": payload["id"], "hourly_values": list(payload["hourlyValues"]), "interpolation": payload["interpolation"], "limits": ({"min": payload["limitsMin"], "max": payload["limitsMax"]} if payload["limitsMin"] is not None else None)})
    return after, applied()


def _invert_create_daily_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-daily-schedule", {"id": payload["id"]})]


def delete_daily_schedule(before, payload):
    """🌓️ `delete-daily-schedule{id}` — Removes one daily profile. Refused while a weekly or annual schedule — or any consumer — still names it."""
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["daily"] = [row for row in after["model"]["schedules"]["daily"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_daily_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["daily"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-daily-schedule", {"index": index, "id": item["id"], "hourlyValues": item["hourly_values"], "interpolation": item["interpolation"], "limitsMin": (item["limits"]["min"] if item["limits"] is not None else None), "limitsMax": (item["limits"]["max"] if item["limits"] is not None else None)})]


def replace_daily_schedule_hourly_values(before, payload):
    """🕔️ `replace-daily-schedule-hourly-values{id,newHourlyValues}` — Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together."""
    value = payload["newHourlyValues"]
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if len(value) != 24:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in value):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["hourly_values"] == list(value):
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["daily"]:
        if row["id"] == payload["id"]:
            row["hourly_values"] = list(value)
    return after, applied()


def _invert_replace_daily_schedule_hourly_values(before, payload):
    """↩️ The hourly_values the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"])
    return [("replace-daily-schedule-hourly-values", {"id": payload["id"], "newHourlyValues": item["hourly_values"]})]


def change_daily_schedule_interpolation(before, payload):
    """🕕️ `change-daily-schedule-interpolation{id,newInterpolation}` — Sets whether a daily profile steps between its hourly values or ramps across them."""
    value = payload["newInterpolation"]
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if item["interpolation"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["daily"]:
        if row["id"] == payload["id"]:
            row["interpolation"] = value
    return after, applied()


def _invert_change_daily_schedule_interpolation(before, payload):
    """↩️ The interpolation the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"])
    return [("change-daily-schedule-interpolation", {"id": payload["id"], "newInterpolation": item["interpolation"]})]


def change_daily_schedule_limits(before, payload):
    """🚧️ `change-daily-schedule-limits{id,newLimitsMin,newLimitsMax}` — the optional clamp, set or cleared as one pair."""
    item = next((row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if (payload["newLimitsMin"] is None) != (payload["newLimitsMax"] is None):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["newLimitsMin"] is not None and payload["newLimitsMin"] > payload["newLimitsMax"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])], invariant="limits-ordered")
    limits = {"min": payload["newLimitsMin"], "max": payload["newLimitsMax"]} if payload["newLimitsMin"] is not None else None
    if item["limits"] == limits:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["daily"]:
        if row["id"] == payload["id"]:
            row["limits"] = limits
    return after, applied()


def _invert_change_daily_schedule_limits(before, payload):
    """↩️ The clamp the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["daily"] if row["id"] == payload["id"])
    old = item["limits"]
    return [("change-daily-schedule-limits", {"id": payload["id"], "newLimitsMin": (old["min"] if old is not None else None), "newLimitsMax": (old["max"] if old is not None else None)})]


def create_weekly_schedule(before, payload):
    """🗓️ `create-weekly-schedule{index,…}` — Defines one week as seven daily profiles, Sunday first (slot 0) through Saturday (slot 6) — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour."""
    rows = before["model"]["schedules"]["weekly"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if len(payload["dailyScheduleIds"]) != 7:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    missing = next((candidate for candidate in payload["dailyScheduleIds"] if not any(row["id"] == candidate for row in before["model"]["schedules"]["daily"])), None)
    if missing is not None:
        return unchanged(before), rejected("mutation.target-missing", [str(missing)])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["weekly"].insert(payload["index"], {"id": payload["id"], "daily_schedule_ids": list(payload["dailyScheduleIds"])})
    return after, applied()


def _invert_create_weekly_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-weekly-schedule", {"id": payload["id"]})]


def delete_weekly_schedule(before, payload):
    """🕖️ `delete-weekly-schedule{id}` — Removes one weekly schedule. Refused while any consumer still resolves its id."""
    item = next((row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["weekly"] = [row for row in after["model"]["schedules"]["weekly"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_weekly_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["weekly"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-weekly-schedule", {"index": index, "id": item["id"], "dailyScheduleIds": item["daily_schedule_ids"]})]


def change_weekly_schedule_day(before, payload):
    """🕗️ `change-weekly-schedule-day{id,dayIndex,newDailyScheduleId}` — one day of the week re-pointed."""
    item = next((row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["dayIndex"] > 6:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["newDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["newDailyScheduleId"])])
    if item["daily_schedule_ids"][payload["dayIndex"]] == payload["newDailyScheduleId"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["weekly"]:
        if row["id"] == payload["id"]:
            row["daily_schedule_ids"][payload["dayIndex"]] = payload["newDailyScheduleId"]
    return after, applied()


def _invert_change_weekly_schedule_day(before, payload):
    """↩️ The daily profile that day named in BASE."""
    item = next(row for row in before["model"]["schedules"]["weekly"] if row["id"] == payload["id"])
    return [("change-weekly-schedule-day", {"id": payload["id"], "dayIndex": payload["dayIndex"], "newDailyScheduleId": item["daily_schedule_ids"][payload["dayIndex"]]})]


def create_annual_schedule(before, payload):
    """📚️ `create-annual-schedule{index,…}` — Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at."""
    rows = before["model"]["schedules"]["annual"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == payload["defaultDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["defaultDailyScheduleId"])])
    if payload["holidayDailyScheduleId"] is not None and not any(row["id"] == payload["holidayDailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["holidayDailyScheduleId"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["annual"].insert(payload["index"], {"id": payload["id"], "rules": [], "default_daily_schedule_id": payload["defaultDailyScheduleId"], "holiday_daily_schedule_id": payload["holidayDailyScheduleId"], "holiday_dates": []})
    return after, applied()


def _invert_create_annual_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-annual-schedule", {"id": payload["id"]})]


def delete_annual_schedule(before, payload):
    """📕️ `delete-annual-schedule{id}` — the rule-based year, removed whole."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["annual"] = [row for row in after["model"]["schedules"]["annual"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_annual_schedule(before, payload):
    """↩️ Re-defines the year, then replays its rules in order and its holidays in order."""
    rows = before["model"]["schedules"]["annual"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    steps = [("create-annual-schedule", {"index": index, "id": item["id"], "defaultDailyScheduleId": item["default_daily_schedule_id"], "holidayDailyScheduleId": item["holiday_daily_schedule_id"]})]
    for position, rule in enumerate(item["rules"]):
        steps.append(("insert-annual-schedule-rule", {"id": item["id"], "index": position, "startMonth": rule["start_month"], "startDay": rule["start_day"], "endMonth": rule["end_month"], "endDay": rule["end_day"], "dailyScheduleId": rule["daily_schedule_id"]}))
    for position, holiday in enumerate(item["holiday_dates"]):
        steps.append(("add-annual-schedule-holiday", {"id": item["id"], "index": position, "year": holiday[0], "month": holiday[1], "day": holiday[2]}))
    return steps


def insert_annual_schedule_rule(before, payload):
    """📗️ `insert-annual-schedule-rule{id,index,…}` — one date rule at a FINAL-state position."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] > len(item["rules"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not (1 <= payload["startMonth"] <= 12 and 1 <= payload["endMonth"] <= 12 and 1 <= payload["startDay"] <= 31 and 1 <= payload["endDay"] <= 31):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if not any(row["id"] == payload["dailyScheduleId"] for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["dailyScheduleId"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["rules"].insert(payload["index"], {"start_month": payload["startMonth"], "start_day": payload["startDay"], "end_month": payload["endMonth"], "end_day": payload["endDay"], "daily_schedule_id": payload["dailyScheduleId"]})
    return after, applied()


def _invert_insert_annual_schedule_rule(before, payload):
    """↩️ The inserted rule is taken back out at the position it was placed."""
    return [("remove-annual-schedule-rule", {"id": payload["id"], "index": payload["index"]})]


def remove_annual_schedule_rule(before, payload):
    """📙️ `remove-annual-schedule-rule{id,index}` — one date rule at a BASE-state position."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["index"] >= len(item["rules"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            del row["rules"][payload["index"]]
    return after, applied()


def _invert_remove_annual_schedule_rule(before, payload):
    """↩️ The removed rule goes back at the index it held."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    rule = item["rules"][payload["index"]]
    return [("insert-annual-schedule-rule", {"id": payload["id"], "index": payload["index"], "startMonth": rule["start_month"], "startDay": rule["start_day"], "endMonth": rule["end_month"], "endDay": rule["end_day"], "dailyScheduleId": rule["daily_schedule_id"]})]


def reorder_annual_schedule_rules(before, payload):
    """🗂️ `reorder-annual-schedule-rules{id,from,to}` — rule precedence IS list order."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["from"] >= len(item["rules"]) or payload["to"] >= len(item["rules"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if payload["from"] == payload["to"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            rule = row["rules"].pop(payload["from"])
            row["rules"].insert(payload["to"], rule)
    return after, applied()


def _invert_reorder_annual_schedule_rules(before, payload):
    """↩️ The same move, back the other way."""
    return [("reorder-annual-schedule-rules", {"id": payload["id"], "from": payload["to"], "to": payload["from"]})]


def change_annual_schedule_default_daily_schedule(before, payload):
    """🎌️ `change-annual-schedule-default-daily-schedule{id,newDefaultDailyScheduleId}` — Re-points the profile a year falls back to on every day no rule matches."""
    value = payload["newDefaultDailyScheduleId"]
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not any(row["id"] == value for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["default_daily_schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["default_daily_schedule_id"] = value
    return after, applied()


def _invert_change_annual_schedule_default_daily_schedule(before, payload):
    """↩️ The default_daily_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    return [("change-annual-schedule-default-daily-schedule", {"id": payload["id"], "newDefaultDailyScheduleId": item["default_daily_schedule_id"]})]


def change_annual_schedule_holiday_daily_schedule(before, payload):
    """🎄️ `change-annual-schedule-holiday-daily-schedule{id,newHolidayDailyScheduleId}` — Re-points — or clears, with a null — the profile a year uses on the dates it holds as holidays."""
    value = payload["newHolidayDailyScheduleId"]
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value is not None and not any(row["id"] == value for row in before["model"]["schedules"]["daily"]):
        return unchanged(before), rejected("mutation.target-missing", [str(value)])
    if item["holiday_daily_schedule_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_daily_schedule_id"] = value
    return after, applied()


def _invert_change_annual_schedule_holiday_daily_schedule(before, payload):
    """↩️ The holiday_daily_schedule_id the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    return [("change-annual-schedule-holiday-daily-schedule", {"id": payload["id"], "newHolidayDailyScheduleId": item["holiday_daily_schedule_id"]})]


def add_annual_schedule_holiday(before, payload):
    """🎉️ `add-annual-schedule-holiday{id,index,year,month,day}` — one dated holiday."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not (1 <= payload["month"] <= 12 and 1 <= payload["day"] <= 31):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["index"] > len(item["holiday_dates"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    date = [payload["year"], payload["month"], payload["day"]]
    if date in item["holiday_dates"]:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_dates"].insert(payload["index"], date)
    return after, applied()


def _invert_add_annual_schedule_holiday(before, payload):
    """↩️ The added holiday is taken back out."""
    return [("remove-annual-schedule-holiday", {"id": payload["id"], "year": payload["year"], "month": payload["month"], "day": payload["day"]})]


def remove_annual_schedule_holiday(before, payload):
    """🎊️ `remove-annual-schedule-holiday{id,year,month,day}` — one dated holiday, taken back out."""
    item = next((row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    date = [payload["year"], payload["month"], payload["day"]]
    if date not in item["holiday_dates"]:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["annual"]:
        if row["id"] == payload["id"]:
            row["holiday_dates"] = [holiday for holiday in row["holiday_dates"] if holiday != date]
    return after, applied()


def _invert_remove_annual_schedule_holiday(before, payload):
    """↩️ The holiday goes back at the position it held in BASE."""
    item = next(row for row in before["model"]["schedules"]["annual"] if row["id"] == payload["id"])
    date = [payload["year"], payload["month"], payload["day"]]
    return [("add-annual-schedule-holiday", {"id": payload["id"], "index": item["holiday_dates"].index(date), "year": payload["year"], "month": payload["month"], "day": payload["day"]})]


def create_time_series_schedule(before, payload):
    """🪗️ `create-time-series-schedule{index,…}` — Defines one externally measured series, indexed by timestep rather than by calendar — the shape a metered profile or a co-simulation trace takes."""
    rows = before["model"]["schedules"]["time_series"]
    if any(row["id"] == payload["id"] for group in ("constants", "daily", "weekly", "annual", "time_series") for row in before["model"]["schedules"][group]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(payload["id"])])
    if payload["index"] > len(rows):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not payload["values"]:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in payload["values"]):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if payload["timestepSeconds"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["time_series"].insert(payload["index"], {"id": payload["id"], "values": list(payload["values"]), "timestep_seconds": payload["timestepSeconds"]})
    return after, applied()


def _invert_create_time_series_schedule(before, payload):
    """↩️ Creation is undone by deleting exactly the schedule it defined."""
    return [("delete-time-series-schedule", {"id": payload["id"]})]


def delete_time_series_schedule(before, payload):
    """🎞️ `delete-time-series-schedule{id}` — Removes one measured series. Refused while any consumer still resolves its id."""
    item = next((row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    consumers = (("people", ("schedule_id", "activity_schedule_id")), ("lighting", ("schedule_id",)), ("equipment", ("schedule_id",)), ("infiltrations", ("schedule_id",)), ("mechanical_ventilations", ("schedule_id",)), ("thermostats", ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id")), ("humidistats", ("humidifying_setpoint_schedule_id", "dehumidifying_setpoint_schedule_id")), ("setpoint_managers", ("schedule_id",)), ("shading_surfaces", ("transmittance_schedule_id",)), ("shw_systems", ("schedule_id",)), ("refrigeration_systems", ("defrost_schedule_id",)), ("water_systems", ("schedule_id",)), ("faults", ("start_schedule_id",)))
    in_use = any(row[name] == payload["id"] for collection, names in consumers for row in before["model"][collection] for name in names)
    in_use = in_use or any(payload["id"] in row["daily_schedule_ids"] for row in before["model"]["schedules"]["weekly"])
    in_use = in_use or any(row["default_daily_schedule_id"] == payload["id"] or row["holiday_daily_schedule_id"] == payload["id"] or any(rule["daily_schedule_id"] == payload["id"] for rule in row["rules"]) for row in before["model"]["schedules"]["annual"])
    if in_use:
        return unchanged(before), rejected("mutation.target-referenced", [str(payload["id"])])
    after = copy.deepcopy(before)
    after["model"]["schedules"]["time_series"] = [row for row in after["model"]["schedules"]["time_series"] if row["id"] != payload["id"]]
    return after, applied()


def _invert_delete_time_series_schedule(before, payload):
    """↩️ Re-defines the removed schedule at the index it actually held."""
    rows = before["model"]["schedules"]["time_series"]
    index = next(position for position, row in enumerate(rows) if row["id"] == payload["id"])
    item = rows[index]
    return [("create-time-series-schedule", {"index": index, "id": item["id"], "values": item["values"], "timestepSeconds": item["timestep_seconds"]})]


def replace_time_series_schedule_values(before, payload):
    """🕘️ `replace-time-series-schedule-values{id,newValues}` — Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together."""
    value = payload["newValues"]
    item = next((row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if not value:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if any(entry != entry or entry in (float("inf"), float("-inf")) for entry in value):
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["values"] == list(value):
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["time_series"]:
        if row["id"] == payload["id"]:
            row["values"] = list(value)
    return after, applied()


def _invert_replace_time_series_schedule_values(before, payload):
    """↩️ The values the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"])
    return [("replace-time-series-schedule-values", {"id": payload["id"], "newValues": item["values"]})]


def change_time_series_schedule_timestep(before, payload):
    """🕙️ `change-time-series-schedule-timestep{id,newTimestepSeconds}` — Sets how many seconds one sample of a measured series covers — what turns its index into a clock."""
    value = payload["newTimestepSeconds"]
    item = next((row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"]), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(payload["id"])])
    if item["timestep_seconds"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["schedules"]["time_series"]:
        if row["id"] == payload["id"]:
            row["timestep_seconds"] = value
    return after, applied()


def _invert_change_time_series_schedule_timestep(before, payload):
    """↩️ The timestep_seconds the forward step overwrote, read off BASE."""
    item = next(row for row in before["model"]["schedules"]["time_series"] if row["id"] == payload["id"])
    return [("change-time-series-schedule-timestep", {"id": payload["id"], "newTimestepSeconds": item["timestep_seconds"]})]


def replace_fenestration_vertices(before, payload):
    """🔶️ `replace-fenestration-vertices{id,newVerticesM}` — Swaps the aperture's own polygon. An empty ring hands the shape back to the derived area/height/sill rectangle; three or more vertices, planar and in the host surface's plane, ARE the aperture."""
    entity_id, value = payload["id"], payload["newVerticesM"]
    item = next((row for row in before["model"]["fenestrations"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if value and len(value) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(not _finite(axis) for corner in value for axis in corner):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    host = next((row for row in before["model"]["surfaces"] if row["id"] == item["surface_id"]), None)
    if host is not None and not _on_plane(value, host["vertices_m"]):
        return unchanged(before), rejected("mutation.target-mismatch", [str(entity_id)])
    if item["vertices_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["fenestrations"]:
        if row["id"] == entity_id:
            row["vertices_m"] = value
    return after, applied()


def _invert_replace_fenestration_vertices(before, payload):
    item = next(row for row in before["model"]["fenestrations"] if row["id"] == payload["id"])
    return [("replace-fenestration-vertices", {"id": payload["id"], "newVerticesM": item["vertices_m"]})]


def _finite(value):
    """🔢️ True when `value` is a real, finite number (the payload schema admits JSON numbers only)."""
    return isinstance(value, (int, float)) and value == value and value not in (float("inf"), float("-inf"))


def _on_plane(polygon, host, tolerance=1e-3):
    """📐️ True when every vertex of `polygon` sits within `tolerance` metres of `host`'s plane."""
    if len(host) < 3:
        return True
    normal = [0.0, 0.0, 0.0]
    for index, current in enumerate(host):
        following = host[(index + 1) % len(host)]
        normal[0] += (current[1] - following[1]) * (current[2] + following[2])
        normal[1] += (current[2] - following[2]) * (current[0] + following[0])
        normal[2] += (current[0] - following[0]) * (current[1] + following[1])
    length = sum(axis * axis for axis in normal) ** 0.5
    if length <= 1e-12:
        return True
    normal = [axis / length for axis in normal]
    anchor = host[0]
    return all(abs(sum((vertex[axis] - anchor[axis]) * normal[axis] for axis in range(3))) <= tolerance for vertex in polygon)


def _change_glazing_scalar(before, payload, key, field, fraction):
    """🪟️ Shared body of every one-scalar glazing-material change."""
    entity_id, value = payload["id"], payload[key]
    item = next((row for row in before["model"]["glazing_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    admissible = _finite(value) and (0.0 <= value <= 1.0 if fraction else value > 0.0)
    if not admissible:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item[field] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["glazing_materials"]:
        if row["id"] == entity_id:
            row[field] = value
    return after, applied()


def _invert_glazing_scalar(before, payload, kind, key, field):
    item = next(row for row in before["model"]["glazing_materials"] if row["id"] == payload["id"])
    return [(kind, {"id": payload["id"], key: item[field]})]


def change_glazing_material_thickness(before, payload):
    """🔷️ `change-glazing-material-thickness{id,newThicknessM}` — Sets pane thickness (m) on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newThicknessM", "thickness_m", False)


def _invert_change_glazing_material_thickness(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-thickness", "newThicknessM", "thickness_m")


def change_glazing_material_conductivity(before, payload):
    """🟠️ `change-glazing-material-conductivity{id,newConductivityWMK}` — Sets pane conductivity (W/m·K) on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newConductivityWMK", "conductivity_w_m_k", False)


def _invert_change_glazing_material_conductivity(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-conductivity", "newConductivityWMK", "conductivity_w_m_k")


def change_glazing_material_solar_transmittance(before, payload):
    """🟡️ `change-glazing-material-solar-transmittance{id,newSolarTransmittance}` — Sets normal-incidence solar transmittance on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newSolarTransmittance", "solar_transmittance", True)


def _invert_change_glazing_material_solar_transmittance(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-solar-transmittance", "newSolarTransmittance", "solar_transmittance")


def change_glazing_material_visible_transmittance(before, payload):
    """🥽️ `change-glazing-material-visible-transmittance{id,newVisibleTransmittance}` — Sets normal-incidence visible transmittance on one glazing material, addressed by id."""
    return _change_glazing_scalar(before, payload, "newVisibleTransmittance", "visible_transmittance", True)


def _invert_change_glazing_material_visible_transmittance(before, payload):
    return _invert_glazing_scalar(before, payload, "change-glazing-material-visible-transmittance", "newVisibleTransmittance", "visible_transmittance")


def change_glazing_material_infrared_emissivity(before, payload):
    """🩻️ `change-glazing-material-infrared-emissivity{id,newInfraredEmissivityFront,newInfraredEmissivityBack}` — Sets both long-wave emissivities of one glazing pane in one step."""
    entity_id = payload["id"]
    front, back = payload["newInfraredEmissivityFront"], payload["newInfraredEmissivityBack"]
    item = next((row for row in before["model"]["glazing_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(not (_finite(value) and 0.0 <= value <= 1.0) for value in (front, back)):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["infrared_emissivity_front"] == front and item["infrared_emissivity_back"] == back:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["glazing_materials"]:
        if row["id"] == entity_id:
            row["infrared_emissivity_front"] = front
            row["infrared_emissivity_back"] = back
    return after, applied()


def _invert_change_glazing_material_infrared_emissivity(before, payload):
    item = next(row for row in before["model"]["glazing_materials"] if row["id"] == payload["id"])
    return [("change-glazing-material-infrared-emissivity", {"id": payload["id"], "newInfraredEmissivityFront": item["infrared_emissivity_front"], "newInfraredEmissivityBack": item["infrared_emissivity_back"]})]


def rename_glazing_material(before, payload):
    """🟢️ `rename-glazing-material{id,newName}` — Renames one glazing material, addressed by id."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["glazing_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["glazing_materials"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_glazing_material(before, payload):
    item = next(row for row in before["model"]["glazing_materials"] if row["id"] == payload["id"])
    return [("rename-glazing-material", {"id": payload["id"], "newName": item["name"]})]


def change_gas_material_thickness(before, payload):
    """🟣️ `change-gas-material-thickness{id,newThicknessM}` — Sets gap width (m) on one gas material, addressed by id."""
    entity_id, value = payload["id"], payload["newThicknessM"]
    item = next((row for row in before["model"]["gas_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (_finite(value) and value > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["thickness_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["gas_materials"]:
        if row["id"] == entity_id:
            row["thickness_m"] = value
    return after, applied()


def _invert_change_gas_material_thickness(before, payload):
    item = next(row for row in before["model"]["gas_materials"] if row["id"] == payload["id"])
    return [("change-gas-material-thickness", {"id": payload["id"], "newThicknessM": item["thickness_m"]})]


def change_gas_material_gas(before, payload):
    """🟤️ `change-gas-material-gas{id,newGas}` — Sets which fill gas occupies one glazing gap, addressed by id."""
    entity_id, value = payload["id"], payload["newGas"]
    item = next((row for row in before["model"]["gas_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["gas"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["gas_materials"]:
        if row["id"] == entity_id:
            row["gas"] = value
    return after, applied()


def _invert_change_gas_material_gas(before, payload):
    item = next(row for row in before["model"]["gas_materials"] if row["id"] == payload["id"])
    return [("change-gas-material-gas", {"id": payload["id"], "newGas": item["gas"]})]


def rename_gas_material(before, payload):
    """🔘️ `rename-gas-material{id,newName}` — Renames one gas material, addressed by id."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["gas_materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["gas_materials"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_gas_material(before, payload):
    item = next(row for row in before["model"]["gas_materials"] if row["id"] == payload["id"])
    return [("rename-gas-material", {"id": payload["id"], "newName": item["name"]})]


def change_material_roughness(before, payload):
    """🗻️ `change-material-roughness{id,newRoughness}` — Sets the surface roughness class of one material, addressed by id."""
    entity_id, value = payload["id"], payload["newRoughness"]
    item = next((row for row in before["model"]["materials"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["roughness"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["materials"]:
        if row["id"] == entity_id:
            row["roughness"] = value
    return after, applied()


def _invert_change_material_roughness(before, payload):
    item = next(row for row in before["model"]["materials"] if row["id"] == payload["id"])
    return [("change-material-roughness", {"id": payload["id"], "newRoughness": item["roughness"]})]


VOCABULARY = {
    "create-space-list": create_space_list,
    "delete-space-list": delete_space_list,
    "rename-space-list": rename_space_list,
    "add-space-list-member": add_space_list_member,
    "remove-space-list-member": remove_space_list_member,
    "create-thermal-enclosure": create_thermal_enclosure,
    "delete-thermal-enclosure": delete_thermal_enclosure,
    "rename-thermal-enclosure": rename_thermal_enclosure,
    "add-thermal-enclosure-zone": add_thermal_enclosure_zone,
    "remove-thermal-enclosure-zone": remove_thermal_enclosure_zone,
    "create-constant-schedule": create_constant_schedule,
    "delete-constant-schedule": delete_constant_schedule,
    "change-constant-schedule-value": change_constant_schedule_value,
    "create-daily-schedule": create_daily_schedule,
    "delete-daily-schedule": delete_daily_schedule,
    "replace-daily-schedule-hourly-values": replace_daily_schedule_hourly_values,
    "change-daily-schedule-interpolation": change_daily_schedule_interpolation,
    "change-daily-schedule-limits": change_daily_schedule_limits,
    "create-weekly-schedule": create_weekly_schedule,
    "delete-weekly-schedule": delete_weekly_schedule,
    "change-weekly-schedule-day": change_weekly_schedule_day,
    "create-annual-schedule": create_annual_schedule,
    "delete-annual-schedule": delete_annual_schedule,
    "insert-annual-schedule-rule": insert_annual_schedule_rule,
    "remove-annual-schedule-rule": remove_annual_schedule_rule,
    "reorder-annual-schedule-rules": reorder_annual_schedule_rules,
    "change-annual-schedule-default-daily-schedule": change_annual_schedule_default_daily_schedule,
    "change-annual-schedule-holiday-daily-schedule": change_annual_schedule_holiday_daily_schedule,
    "add-annual-schedule-holiday": add_annual_schedule_holiday,
    "remove-annual-schedule-holiday": remove_annual_schedule_holiday,
    "create-time-series-schedule": create_time_series_schedule,
    "delete-time-series-schedule": delete_time_series_schedule,
    "replace-time-series-schedule-values": replace_time_series_schedule_values,
    "change-time-series-schedule-timestep": change_time_series_schedule_timestep,
    "replace-fenestration-vertices": replace_fenestration_vertices,
    "change-glazing-material-thickness": change_glazing_material_thickness,
    "change-glazing-material-conductivity": change_glazing_material_conductivity,
    "change-glazing-material-solar-transmittance": change_glazing_material_solar_transmittance,
    "change-glazing-material-visible-transmittance": change_glazing_material_visible_transmittance,
    "change-glazing-material-infrared-emissivity": change_glazing_material_infrared_emissivity,
    "rename-glazing-material": rename_glazing_material,
    "change-gas-material-thickness": change_gas_material_thickness,
    "change-gas-material-gas": change_gas_material_gas,
    "rename-gas-material": rename_gas_material,
    "change-material-roughness": change_material_roughness,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-space-list": _invert_create_space_list,
    "delete-space-list": _invert_delete_space_list,
    "rename-space-list": _invert_rename_space_list,
    "add-space-list-member": _invert_add_space_list_member,
    "remove-space-list-member": _invert_remove_space_list_member,
    "create-thermal-enclosure": _invert_create_thermal_enclosure,
    "delete-thermal-enclosure": _invert_delete_thermal_enclosure,
    "rename-thermal-enclosure": _invert_rename_thermal_enclosure,
    "add-thermal-enclosure-zone": _invert_add_thermal_enclosure_zone,
    "remove-thermal-enclosure-zone": _invert_remove_thermal_enclosure_zone,
    "create-constant-schedule": _invert_create_constant_schedule,
    "delete-constant-schedule": _invert_delete_constant_schedule,
    "change-constant-schedule-value": _invert_change_constant_schedule_value,
    "create-daily-schedule": _invert_create_daily_schedule,
    "delete-daily-schedule": _invert_delete_daily_schedule,
    "replace-daily-schedule-hourly-values": _invert_replace_daily_schedule_hourly_values,
    "change-daily-schedule-interpolation": _invert_change_daily_schedule_interpolation,
    "change-daily-schedule-limits": _invert_change_daily_schedule_limits,
    "create-weekly-schedule": _invert_create_weekly_schedule,
    "delete-weekly-schedule": _invert_delete_weekly_schedule,
    "change-weekly-schedule-day": _invert_change_weekly_schedule_day,
    "create-annual-schedule": _invert_create_annual_schedule,
    "delete-annual-schedule": _invert_delete_annual_schedule,
    "insert-annual-schedule-rule": _invert_insert_annual_schedule_rule,
    "remove-annual-schedule-rule": _invert_remove_annual_schedule_rule,
    "reorder-annual-schedule-rules": _invert_reorder_annual_schedule_rules,
    "change-annual-schedule-default-daily-schedule": _invert_change_annual_schedule_default_daily_schedule,
    "change-annual-schedule-holiday-daily-schedule": _invert_change_annual_schedule_holiday_daily_schedule,
    "add-annual-schedule-holiday": _invert_add_annual_schedule_holiday,
    "remove-annual-schedule-holiday": _invert_remove_annual_schedule_holiday,
    "create-time-series-schedule": _invert_create_time_series_schedule,
    "delete-time-series-schedule": _invert_delete_time_series_schedule,
    "replace-time-series-schedule-values": _invert_replace_time_series_schedule_values,
    "change-time-series-schedule-timestep": _invert_change_time_series_schedule_timestep,
    "replace-fenestration-vertices": _invert_replace_fenestration_vertices,
    "change-glazing-material-thickness": _invert_change_glazing_material_thickness,
    "change-glazing-material-conductivity": _invert_change_glazing_material_conductivity,
    "change-glazing-material-solar-transmittance": _invert_change_glazing_material_solar_transmittance,
    "change-glazing-material-visible-transmittance": _invert_change_glazing_material_visible_transmittance,
    "change-glazing-material-infrared-emissivity": _invert_change_glazing_material_infrared_emissivity,
    "rename-glazing-material": _invert_rename_glazing_material,
    "change-gas-material-thickness": _invert_change_gas_material_thickness,
    "change-gas-material-gas": _invert_change_gas_material_gas,
    "rename-gas-material": _invert_rename_gas_material,
    "change-material-roughness": _invert_change_material_roughness,
}
# endregion 🔖️Vocabulary
