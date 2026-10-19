"""⏱️ Schedule kinds: constant, daily, weekly, annual and time-series schedules."""

from __future__ import annotations

import copy

from mutate_energy_model_1_mutation_vocabulary import applied, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
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
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
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


VOCABULARY = {
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
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
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
}
# endregion 🔖️Vocabulary
