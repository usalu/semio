"""🏠️ Building kinds: model, site, run period, zones, spaces, surfaces and surface boundaries."""

from __future__ import annotations

import copy
import math

from mutate_energy_model_1_mutation_vocabulary import applied, head_link, no_op, rejected, unchanged


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "rename-model-renames-the-model": "shared://🧬️mutations/🔋️model/🏷️rename/✅️renames-the-model",
    "rename-model-refuses-a-blank-name": "shared://🧬️mutations/🔋️model/🏷️rename/⛔️refuses",
    "change-model-version-bumps-the-version": "shared://🧬️mutations/🔋️model/🔢️change-version/✅️bumps",
    "change-model-version-refuses-a-blank-version": "shared://🧬️mutations/🔋️model/🔢️change-version/⛔️refuses",
    "change-site-latitude-sets": "shared://🧬️mutations/📍️site/🌍️change-latitude/✅️sets",
    "change-site-latitude-refuses": "shared://🧬️mutations/📍️site/🌍️change-latitude/⛔️refuses",
    "change-site-longitude-sets": "shared://🧬️mutations/📍️site/🌏️change-longitude/✅️sets",
    "change-site-longitude-refuses": "shared://🧬️mutations/📍️site/🌏️change-longitude/⛔️refuses",
    "change-site-elevation-sets": "shared://🧬️mutations/📍️site/⛰️change-elevation/✅️sets",
    "change-site-elevation-refuses": "shared://🧬️mutations/📍️site/⛰️change-elevation/⛔️refuses",
    "change-site-time-zone-sets": "shared://🧬️mutations/📍️site/🕰️change-time-zone/✅️sets",
    "change-site-time-zone-refuses": "shared://🧬️mutations/📍️site/🕰️change-time-zone/⛔️refuses",
    "change-site-north-axis-sets": "shared://🧬️mutations/📍️site/🔝️change-north-axis/✅️sets",
    "change-site-north-axis-same": "shared://🧬️mutations/📍️site/🔝️change-north-axis/🟰️same",
    "change-ground-building-sets": "shared://🧬️mutations/📍️site/🌡️change-ground-building/✅️sets",
    "change-ground-building-refuses": "shared://🧬️mutations/📍️site/🌡️change-ground-building/⛔️refuses",
    "change-ground-shallow-sets": "shared://🧬️mutations/📍️site/🌱️change-ground-shallow/✅️sets",
    "change-ground-shallow-refuses": "shared://🧬️mutations/📍️site/🌱️change-ground-shallow/⛔️refuses",
    "change-ground-deep-sets": "shared://🧬️mutations/📍️site/⛏️change-ground-deep/✅️sets",
    "change-ground-deep-refuses": "shared://🧬️mutations/📍️site/⛏️change-ground-deep/⛔️refuses",
    "change-run-start-month-sets": "shared://🧬️mutations/⏯️run/🛫️change-start-month/✅️sets",
    "change-run-start-month-refuses": "shared://🧬️mutations/⏯️run/🛫️change-start-month/⛔️refuses",
    "change-run-start-day-sets": "shared://🧬️mutations/⏯️run/▶️change-start-day/✅️sets",
    "change-run-start-day-refuses": "shared://🧬️mutations/⏯️run/▶️change-start-day/⛔️refuses",
    "change-run-end-month-sets": "shared://🧬️mutations/⏯️run/🛬️change-end-month/✅️sets",
    "change-run-end-month-refuses": "shared://🧬️mutations/⏯️run/🛬️change-end-month/⛔️refuses",
    "change-run-end-day-sets": "shared://🧬️mutations/⏯️run/⏹️change-end-day/✅️sets",
    "change-run-end-day-refuses": "shared://🧬️mutations/⏯️run/⏹️change-end-day/⛔️refuses",
    "change-run-year-sets": "shared://🧬️mutations/⏯️run/📅️change-year/✅️sets",
    "change-run-year-refuses": "shared://🧬️mutations/⏯️run/📅️change-year/⛔️refuses",
    "replace-airflow-network-attaches-a-network": "shared://🧬️mutations/🔋️model/🫧️replace-airflow-network/✅️attaches",
    "replace-airflow-network-refuses-unpaired-nodes": "shared://🧬️mutations/🔋️model/🫧️replace-airflow-network/⛔️refuses",
    "add-output-variable-adds-zone-air-temp": "shared://🧬️mutations/🔋️model/📊️add-output-variable/✅️adds",
    "add-output-variable-refuses-a-duplicate": "shared://🧬️mutations/🔋️model/📊️add-output-variable/⛔️refuses",
    "remove-output-variable-drops-zone-air-temp": "shared://🧬️mutations/🔋️model/📉️remove-output-variable/✅️drops",
    "remove-output-variable-refuses-an-absent-one": "shared://🧬️mutations/🔋️model/📉️remove-output-variable/⛔️refuses",
    "bind-weather-file-binds-hannover-epw": "shared://🧬️mutations/🔋️model/🌦️bind-weather-file/✅️binds",
    "bind-weather-file-refuses-a-bad-uri": "shared://🧬️mutations/🔋️model/🌦️bind-weather-file/⛔️refuses",
    "unbind-weather-file-unbinds-the-weather": "shared://🧬️mutations/🔋️model/🌤️unbind-weather-file/✅️unbinds",
    "unbind-weather-file-refuses-when-unbound": "shared://🧬️mutations/🔋️model/🌤️unbind-weather-file/⛔️refuses",
    "connect-referenced-model-connects-the-geometry": "shared://🧬️mutations/🔋️model/🪢️connect-referenced-model/✅️connects",
    "connect-referenced-model-refuses-a-bad-uri": "shared://🧬️mutations/🔋️model/🪢️connect-referenced-model/⛔️refuses",
    "disconnect-referenced-model-disconnects-the-geometry": "shared://🧬️mutations/🔋️model/✂️disconnect-referenced-model/✅️disconnects",
    "disconnect-referenced-model-refuses-when-absent": "shared://🧬️mutations/🔋️model/✂️disconnect-referenced-model/⛔️refuses",
    "rename-zone-renames-zone-one": "shared://🧬️mutations/🏡️zone/🏠️rename/✅️renames-zone-one",
    "rename-zone-refuses-a-missing-zone": "shared://🧬️mutations/🏡️zone/🏠️rename/⛔️refuses",
    "change-zone-volume-resizes-zone-one": "shared://🧬️mutations/🏡️zone/🧊️change-volume/✅️resizes",
    "change-zone-volume-refuses-zero-volume": "shared://🧬️mutations/🏡️zone/🧊️change-volume/⛔️refuses",
    "change-zone-multiplier-stacks-four-storeys": "shared://🧬️mutations/🏡️zone/✖️change-multiplier/✅️stacks",
    "change-zone-multiplier-refuses-zero-instances": "shared://🧬️mutations/🏡️zone/✖️change-multiplier/⛔️refuses",
    "change-zone-conditioned-frees-the-zone": "shared://🧬️mutations/🏡️zone/🌬️change-conditioned/✅️frees",
    "change-zone-conditioned-refuses-a-missing-zone": "shared://🧬️mutations/🏡️zone/🌬️change-conditioned/⛔️refuses",
    "change-zone-floor-area-participation-excludes-the-zone": "shared://🧬️mutations/🏡️zone/📐️change-floor-area-participation/✅️excludes",
    "change-zone-floor-area-participation-refuses-a-missing-zone": "shared://🧬️mutations/🏡️zone/📐️change-floor-area-participation/⛔️refuses",
    "create-zone-adds-a-second-zone": "shared://🧬️mutations/🏡️zone/🏘️create/✅️adds-a-second-zone",
    "create-zone-refuses-a-taken-id": "shared://🧬️mutations/🏡️zone/🏘️create/⛔️refuses-a-taken-id",
    "delete-zone-deletes-a-free-zone": "shared://🧬️mutations/🏡️zone/🏚️delete/✅️deletes-a-free-zone",
    "delete-zone-refuses-a-used-zone": "shared://🧬️mutations/🏡️zone/🏚️delete/⛔️refuses-a-used-zone",
    "create-space-adds-a-space": "shared://🧬️mutations/🛋️space/🪑️create/✅️adds-a-space",
    "create-space-refuses-a-missing-zone": "shared://🧬️mutations/🛋️space/🪑️create/⛔️refuses",
    "delete-space-deletes-a-free-space": "shared://🧬️mutations/🛋️space/🧹️delete/✅️deletes-a-free-space",
    "delete-space-refuses-a-missing-space": "shared://🧬️mutations/🛋️space/🧹️delete/⛔️refuses",
    "rename-space-renames-a-space": "shared://🧬️mutations/🛋️space/🔤️rename/✅️renames-a-space",
    "rename-space-refuses-a-blank-name": "shared://🧬️mutations/🛋️space/🔤️rename/⛔️refuses",
    "change-space-floor-area-resizes-a-space": "shared://🧬️mutations/🛋️space/🧮️change-floor-area/✅️resizes",
    "change-space-floor-area-refuses-negative-area": "shared://🧬️mutations/🛋️space/🧮️change-floor-area/⛔️refuses",
    "change-space-zone-moves-a-space": "shared://🧬️mutations/🛋️space/🚚️change-zone/✅️moves-a-space",
    "change-space-zone-refuses-a-missing-zone": "shared://🧬️mutations/🛋️space/🚚️change-zone/⛔️refuses",
    "create-surface-adds-a-south-wall": "shared://🧬️mutations/🧱️surface/🟫️create/✅️adds-a-south-wall",
    "create-surface-refuses-two-vertices": "shared://🧬️mutations/🧱️surface/🟫️create/⛔️refuses",
    "delete-surface-cascades-a-window": "shared://🧬️mutations/🧱️surface/🪚️delete/✅️cascades-a-window",
    "delete-surface-refuses-a-partner": "shared://🧬️mutations/🧱️surface/🪚️delete/⛔️refuses-a-partner",
    "rename-surface-renames-a-wall": "shared://🧬️mutations/🧱️surface/🏳️rename/✅️renames-a-wall",
    "rename-surface-refuses-a-taken-name": "shared://🧬️mutations/🧱️surface/🏳️rename/⛔️refuses",
    "change-surface-zone-moves-a-wall": "shared://🧬️mutations/🧱️surface/🗜️change-zone/✅️moves-a-wall",
    "change-surface-zone-refuses-a-missing-zone": "shared://🧬️mutations/🧱️surface/🗜️change-zone/⛔️refuses",
    "change-surface-class-turns-a-wall-to-roof": "shared://🧬️mutations/🧱️surface/🧩️change-class/✅️turns",
    "change-surface-class-refuses-a-missing-one": "shared://🧬️mutations/🧱️surface/🧩️change-class/⛔️refuses",
    "replace-surface-vertices-narrows-a-wall": "shared://🧬️mutations/🧱️surface/🔺️replace-vertices/✅️narrows",
    "replace-surface-vertices-refuses-a-line": "shared://🧬️mutations/🧱️surface/🔺️replace-vertices/⛔️refuses",
    "change-surface-construction-swaps-the-wall-stack": "shared://🧬️mutations/🧱️surface/🧰️change-construction/✅️swaps",
    "change-surface-construction-refuses-a-missing-one": "shared://🧬️mutations/🧱️surface/🧰️change-construction/⛔️refuses",
    "change-surface-boundary-condition-grounds-a-floor": "shared://🧬️mutations/🔲️boundary/🚧️change-boundary-condition/✅️grounds",
    "change-surface-boundary-condition-refuses-half-a-union": "shared://🧬️mutations/🔲️boundary/🚧️change-boundary-condition/⛔️refuses",
    "change-surface-sun-exposed-shades-a-wall": "shared://🧬️mutations/🔲️boundary/🌅️change-sun-exposed/✅️shades",
    "change-surface-sun-exposed-refuses-a-missing-one": "shared://🧬️mutations/🔲️boundary/🌅️change-sun-exposed/⛔️refuses",
    "change-surface-wind-exposed-shelters-a-wall": "shared://🧬️mutations/🔲️boundary/🍃️change-wind-exposed/✅️shelters",
    "change-surface-wind-exposed-refuses-a-missing-one": "shared://🧬️mutations/🔲️boundary/🍃️change-wind-exposed/⛔️refuses",
    "change-surface-multiplier-repeats-a-wall": "shared://🧬️mutations/🧱️surface/🔁️change-multiplier/✅️repeats",
    "change-surface-multiplier-refuses-zero": "shared://🧬️mutations/🧱️surface/🔁️change-multiplier/⛔️refuses",
}
# endregion 🔖️Fixtures


# region 🔖️Vocabulary
def rename_model(before, payload):
    """🏷️ `rename-model{newName}` — taxonomy.md's `rename` verb on the document identity field."""
    name = payload["newName"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [name])
    if before["model"]["name"] == name:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    after["model"]["name"] = name
    return after, applied()


def change_model_version(before, payload):
    """🔢️ `change-model-version{newVersion}` — one scalar field."""
    version = payload["newVersion"]
    if not version.strip():
        return unchanged(before), rejected("mutation.invariant", [version])
    if before["model"]["version"] == version:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    after["model"]["version"] = version
    return after, applied()


def _run_period_is_interval(period):
    """🗓️ Both bounds are real dates of the period's year and the start does not lie after the end."""
    year = period["year"]
    leap = (year % 4 == 0 and year % 100 != 0) or year % 400 == 0
    lengths = [31, 29 if leap else 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

    def exists(month, day):
        return 1 <= month <= 12 and 1 <= day <= lengths[month - 1]

    start, end = (period["start_month"], period["start_day"]), (period["end_month"], period["end_day"])
    return exists(*start) and exists(*end) and start <= end


def _site_leaf(field, wire, admissible):
    """🌍️ A `change-site-<field>` leaf: one scalar of the site facet, refused outside its range."""
    def apply(before, payload):
        value = payload[wire]
        if not admissible(value):
            return unchanged(before), rejected("mutation.invariant", [])
        if before["model"]["site"][field] == value:
            return unchanged(before), no_op()
        after = copy.deepcopy(before)
        after["model"]["site"][field] = value
        return after, applied()
    return apply


def _ground_leaf(series):
    """🌡️ A monthly ground-temperature leaf (`series` = None: the deep value): nothing below absolute zero."""
    def apply(before, payload):
        value, month = payload["newTemperatureC"], payload.get("month")
        if (series is not None and not 1 <= month <= 12) or not math.isfinite(value) or value < -273.15:
            return unchanged(before), rejected("mutation.invariant", [])
        ground = before["model"]["ground_temperature"]
        if (ground[series][month - 1] if series else ground["deep_c"]) == value:
            return unchanged(before), no_op()
        after = copy.deepcopy(before)
        if series:
            after["model"]["ground_temperature"][series][month - 1] = value
        else:
            after["model"]["ground_temperature"]["deep_c"] = value
        return after, applied()
    return apply


def _run_period_leaf(field, wire, low, high):
    """📅️ A run-period field leaf: refused outside its range, and refused (`mutation.target-mismatch`) when the edited
    period stops being a calendar interval with the current other bound."""
    def apply(before, payload):
        value = payload[wire]
        if not low <= value <= high:
            return unchanged(before), rejected("mutation.invariant", [])
        if before["model"]["run_period"][field] == value:
            return unchanged(before), no_op()
        after = copy.deepcopy(before)
        after["model"]["run_period"][field] = value
        if not _run_period_is_interval(after["model"]["run_period"]):
            return unchanged(before), rejected("mutation.target-mismatch", [])
        return after, applied()
    return apply


change_site_latitude = _site_leaf("latitude_deg", "newLatitudeDeg", lambda value: -90.0 <= value <= 90.0)


change_site_longitude = _site_leaf("longitude_deg", "newLongitudeDeg", lambda value: -180.0 <= value <= 180.0)


change_site_elevation = _site_leaf("elevation_m", "newElevationM", lambda value: -300.0 <= value < 8900.0)


change_site_time_zone = _site_leaf("time_zone_hours", "newTimeZoneHours", lambda value: -12.0 <= value <= 14.0)


change_site_north_axis = _site_leaf("north_axis_deg", "newNorthAxisDeg", math.isfinite)


change_ground_building = _ground_leaf("building_surface_c")


change_ground_shallow = _ground_leaf("shallow_c")


change_ground_deep = _ground_leaf(None)


change_run_start_month = _run_period_leaf("start_month", "newStartMonth", 1, 12)


change_run_start_day = _run_period_leaf("start_day", "newStartDay", 1, 31)


change_run_end_month = _run_period_leaf("end_month", "newEndMonth", 1, 12)


change_run_end_day = _run_period_leaf("end_day", "newEndDay", 1, 31)


change_run_year = _run_period_leaf("year", "newYear", 0, 65535)


#: ↩️ The base value each field leaf's undo writes back, as that leaf's own payload.
FIELD_LEAVES = {
    "change-site-latitude": lambda before, payload: {"newLatitudeDeg": before["model"]["site"]["latitude_deg"]},
    "change-site-longitude": lambda before, payload: {"newLongitudeDeg": before["model"]["site"]["longitude_deg"]},
    "change-site-elevation": lambda before, payload: {"newElevationM": before["model"]["site"]["elevation_m"]},
    "change-site-time-zone": lambda before, payload: {"newTimeZoneHours": before["model"]["site"]["time_zone_hours"]},
    "change-site-north-axis": lambda before, payload: {"newNorthAxisDeg": before["model"]["site"]["north_axis_deg"]},
    "change-ground-building": lambda before, payload: {"month": payload["month"], "newTemperatureC": before["model"]["ground_temperature"]["building_surface_c"][payload["month"] - 1]},
    "change-ground-shallow": lambda before, payload: {"month": payload["month"], "newTemperatureC": before["model"]["ground_temperature"]["shallow_c"][payload["month"] - 1]},
    "change-ground-deep": lambda before, payload: {"newTemperatureC": before["model"]["ground_temperature"]["deep_c"]},
    "change-run-start-month": lambda before, payload: {"newStartMonth": before["model"]["run_period"]["start_month"]},
    "change-run-start-day": lambda before, payload: {"newStartDay": before["model"]["run_period"]["start_day"]},
    "change-run-end-month": lambda before, payload: {"newEndMonth": before["model"]["run_period"]["end_month"]},
    "change-run-end-day": lambda before, payload: {"newEndDay": before["model"]["run_period"]["end_day"]},
    "change-run-year": lambda before, payload: {"newYear": before["model"]["run_period"]["year"]},
}


def replace_airflow_network(before, payload):
    """🫧️ `replace-airflow-network` — a whole-value swap of the document-root singleton; `present`
    false detaches it, so one kind covers attach and detach (taxonomy.md's `replace` verb)."""
    zone_ids = payload["zoneIds"]
    node_ids = payload["nodeIds"]
    link_ids = payload["linkIds"]
    if len(zone_ids) != len(node_ids):
        return unchanged(before), rejected("mutation.invariant", [], invariant="zone-node-pairs")
    if not payload["present"] and (zone_ids or link_ids):
        return unchanged(before), rejected("mutation.invariant", [])
    network = None
    if payload["present"]:
        network = {"zone_node_ids": [[zone, node] for zone, node in zip(zone_ids, node_ids)], "outdoor_node_id": payload["outdoorNodeId"], "link_ids": link_ids}
    if before["model"]["airflow_network"] == network:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    after["model"]["airflow_network"] = network
    return after, applied()


def add_output_variable(before, payload):
    """📊️ `add-output-variable` — set-like membership keyed by the natural `(name, key)` pair."""
    name, key = payload["name"], payload["key"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [key])
    if any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.duplicate-id", [name, key])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.target-missing", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"].insert(len(after["model"]["output_variables"]) if payload.get("index") is None else payload["index"], {"name": name, "key": key, "reporting_frequency": payload["reportingFrequency"]})
    return after, applied()


def remove_output_variable(before, payload):
    """📉️ `remove-output-variable` — the `add` verb's inverse partner."""
    name, key = payload["name"], payload["key"]
    if not any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.target-missing", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"] = [spec for spec in after["model"]["output_variables"] if not (spec["name"] == name and spec["key"] == key)]
    return after, applied()


def owned_target(target):
    """🪪️ Admits exact owned reference fields for semantic link mutations."""
    if not isinstance(target, dict) or set(target) != {"artifactId", "dialect"} or not isinstance(target["dialect"], dict) or set(target["dialect"]) != {"artifactKind", "standard", "subset"}:
        return None
    if any(not isinstance(v, str) or not v for v in [target["artifactId"], *target["dialect"].values()]):
        return None
    return copy.deepcopy(target)


def bind_weather_file(before, payload):
    """🌦️ `bind-weather-file{target}` — taxonomy.md's `bind` verb attaching a parameterization."""
    target = owned_target(payload["target"])
    if target is None:
        return unchanged(before), rejected("mutation.invariant", [payload["target"].get("artifactId", "") if isinstance(payload["target"], dict) else ""])
    link = head_link(target, "weather")
    if before.get("weatherLink") == link:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    after["weatherLink"] = link
    return after, applied()


def unbind_weather_file(before, payload):
    """🌤️ `unbind-weather-file` — `bind`'s inverse partner; refused when nothing is bound."""
    del payload
    if not before.get("weatherLink"):
        return unchanged(before), rejected("mutation.target-missing", [])
    after = copy.deepcopy(before)
    after["weatherLink"] = None
    return after, applied()


def connect_referenced_model(before, payload):
    """🪢️ `connect-referenced-model{target}` — taxonomy.md's `connect` verb on a relationship."""
    target = owned_target(payload["target"])
    if target is None:
        return unchanged(before), rejected("mutation.invariant", [payload["target"].get("artifactId", "") if isinstance(payload["target"], dict) else ""])
    link = head_link(target, "model")
    if before.get("referencedModel") == link:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    after["referencedModel"] = link
    return after, applied()


def disconnect_referenced_model(before, payload):
    """✂️ `disconnect-referenced-model` — `connect`'s inverse partner."""
    del payload
    if not before.get("referencedModel"):
        return unchanged(before), rejected("mutation.target-missing", [])
    after = copy.deepcopy(before)
    after["referencedModel"] = None
    return after, applied()


def _zone(before, entity_id):
    for zone in before["model"]["zones"]:
        if zone["id"] == entity_id:
            return zone
    return None


def _with_zone(before, entity_id, field, value):
    after = copy.deepcopy(before)
    for zone in after["model"]["zones"]:
        if zone["id"] == entity_id:
            zone[field] = value
    return after


def rename_zone(before, payload):
    """🏠️ `rename-zone{id,newName}` — id-keyed identity field; a duplicate name is refused because
    every report keys on it."""
    entity_id, name = payload["id"], payload["newName"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(other["id"] != entity_id and other["name"] == name for other in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if zone["name"] == name:
        return unchanged(before), no_op()
    return _with_zone(before, entity_id, "name", name), applied()


def change_zone_volume(before, payload):
    """📦️ `change-zone-volume{id,newVolumeM3}` — the zone air capacitance."""
    entity_id, volume = payload["id"], payload["newVolumeM3"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if volume != volume or volume in (float("inf"), float("-inf")) or volume <= 0.0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if zone["volume_m3"] == volume:
        return unchanged(before), no_op()
    return _with_zone(before, entity_id, "volume_m3", volume), applied()


def change_zone_multiplier(before, payload):
    """✖️ `change-zone-multiplier{id,newMultiplier}` — identical zone instances."""
    entity_id, multiplier = payload["id"], payload["newMultiplier"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if multiplier == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if zone["multiplier"] == multiplier:
        return unchanged(before), no_op()
    return _with_zone(before, entity_id, "multiplier", multiplier), applied()


def change_zone_conditioned(before, payload):
    """🌬️ `change-zone-conditioned{id,newConditioned}` — whether equipment serves the zone."""
    entity_id, conditioned = payload["id"], payload["newConditioned"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["conditioned"] == conditioned:
        return unchanged(before), no_op()
    return _with_zone(before, entity_id, "conditioned", conditioned), applied()


def change_zone_floor_area_participation(before, payload):
    """📐️ `change-zone-floor-area-participation{id,newPartOfTotalFloorArea}` — whether the zone's
    floor area counts toward the building total the normalized reports divide by."""
    entity_id, participates = payload["id"], payload["newPartOfTotalFloorArea"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["part_of_total_floor_area"] == participates:
        return unchanged(before), no_op()
    return _with_zone(before, entity_id, "part_of_total_floor_area", participates), applied()


def create_zone(before, payload):
    """🏘️ `create-zone` — Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["name"] == payload["name"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not (payload["volumeM3"] == payload["volumeM3"] and abs(payload["volumeM3"]) != float("inf") and payload["volumeM3"] > 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if payload["multiplier"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "volume_m3": payload["volumeM3"], "multiplier": payload["multiplier"], "conditioned": payload["conditioned"], "part_of_total_floor_area": payload["partOfTotalFloorArea"]}
    rows = after["model"]["zones"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    if payload.get("index") is not None:
        position = payload["index"]
    rows.insert(position, created)
    return after, applied()


def _invert_create_zone(before, payload):
    return [("delete-zone", {"id": payload["id"]})]


def delete_zone(before, payload):
    """🏚️ `delete-zone{id}` — Removes one thermal zone. It RESTRICTS rather than cascades: while any space, surface, gain, HVAC object, grouping or airflow node still names the zone it is refused with `mutation.target-referenced`, because a cascade here would have to delete surfaces, which cascade again to fenestrations and adjacency pairs."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["zones"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(row["zone_id"] == entity_id for name in ("spaces", "surfaces", "people", "lighting", "equipment", "thermostats", "humidistats", "ideal_loads", "zone_equipment", "infiltrations", "mechanical_ventilations", "sizing_objects", "daylight_zones", "room_air_models") for row in before["model"][name]) or any(entity_id in row["zone_ids"] for row in before["model"]["thermal_enclosures"]) or any(entity_id in row["terminal_zone_ids"] for row in before["model"]["air_loops"]) or (before["model"]["airflow_network"] is not None and any(pair[0] == entity_id for pair in before["model"]["airflow_network"]["zone_node_ids"])):
        return unchanged(before), rejected("mutation.target-referenced", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["zones"] = [row for row in after["model"]["zones"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_zone(before, payload):
    index, item = next((position, row) for position, row in enumerate(before["model"]["zones"]) if row["id"] == payload["id"])
    return [("create-zone", {"id": item["id"], "name": item["name"], "volumeM3": item["volume_m3"], "multiplier": item["multiplier"], "conditioned": item["conditioned"], "partOfTotalFloorArea": item["part_of_total_floor_area"], "index": index})]


def create_space(before, payload):
    """🪑️ `create-space` — Adds one space inside an existing zone. The owning zone must already exist, so the document never carries a space whose zone the reports cannot resolve."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["spaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (payload["floorAreaM2"] == payload["floorAreaM2"] and abs(payload["floorAreaM2"]) != float("inf") and payload["floorAreaM2"] >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if payload.get("index") is not None and payload["index"] > len(before["model"]["spaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "zone_id": payload["zoneId"], "floor_area_m2": payload["floorAreaM2"]}
    rows = after["model"]["spaces"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    if payload.get("index") is not None:
        position = payload["index"]
    rows.insert(position, created)
    return after, applied()


def _invert_create_space(before, payload):
    return [("delete-space", {"id": payload["id"]})]


def delete_space(before, payload):
    """🧹️ `delete-space{id}` — Removes one space. Refused while a space list still names it, so no grouping is left pointing at nothing."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(payload["id"] in row["space_ids"] for row in before["model"]["space_lists"]):
        return unchanged(before), rejected("mutation.target-referenced", [str(entity_id)])
    after = copy.deepcopy(before)
    after["model"]["spaces"] = [row for row in after["model"]["spaces"] if row["id"] != entity_id]
    return after, applied()


def _invert_delete_space(before, payload):
    index, item = next((position, row) for position, row in enumerate(before["model"]["spaces"]) if row["id"] == payload["id"])
    return [("create-space", {"id": item["id"], "name": item["name"], "zoneId": item["zone_id"], "floorAreaM2": item["floor_area_m2"], "index": index})]


def rename_space(before, payload):
    """🔤️ `rename-space{id,newName}` — Sets one space's identity field; a name another space already holds is refused."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["spaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["spaces"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_space(before, payload):
    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("rename-space", {"id": payload["id"], "newName": item["name"]})]


def change_space_floor_area(before, payload):
    """🧮️ `change-space-floor-area{id,newFloorAreaM2}` — Sets one space's floor area in square metres — the denominator every per-area gain in that space is expanded against."""
    entity_id, value = payload["id"], payload["newFloorAreaM2"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not (value == value and abs(value) != float("inf") and value >= 0.0):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["floor_area_m2"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["spaces"]:
        if row["id"] == entity_id:
            row["floor_area_m2"] = value
    return after, applied()


def _invert_change_space_floor_area(before, payload):
    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("change-space-floor-area", {"id": payload["id"], "newFloorAreaM2": item["floor_area_m2"]})]


def change_space_zone(before, payload):
    """🚚️ `change-space-zone{id,newZoneId}` — Reassigns one space to another existing zone. `Space::zone_id` is a foreign key between two flat collections, not a recursive parent field, so this is the `change` verb rather than a hierarchy move."""
    entity_id, value = payload["id"], payload["newZoneId"]
    item = next((row for row in before["model"]["spaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["spaces"]:
        if row["id"] == entity_id:
            row["zone_id"] = value
    return after, applied()


def _invert_change_space_zone(before, payload):
    item = next(row for row in before["model"]["spaces"] if row["id"] == payload["id"])
    return [("change-space-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def create_surface(before, payload):
    """🟫️ `create-surface` — Adds one planar polygon surface to an existing zone with an existing construction. The exterior boundary arrives as its two halves — a `boundary` discriminator and the `interzoneSurfaceId` only the `Interzone` arm carries — because `dsl::DslScalar` binds unit variants only."""
    entity_id = payload["id"]
    if any(row["id"] == entity_id for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if not payload["name"].strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if not any(row["id"] == payload["zoneId"] for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == payload["constructionId"] for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if len(payload["verticesM"]) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if (payload["interzoneSurfaceId"] is not None and (payload["interzoneSurfaceId"] == entity_id or not any(row["id"] == payload["interzoneSurfaceId"] for row in before["model"]["surfaces"]))):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if payload["multiplier"] == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if (payload["boundary"] == "Interzone") != (payload["interzoneSurfaceId"] is not None):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    boundary = {"Interzone": payload["interzoneSurfaceId"]} if payload["interzoneSurfaceId"] is not None else payload["boundary"]
    if payload.get("index") is not None and payload["index"] > len(before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.target-missing", [str(payload["id"])])
    after = copy.deepcopy(before)
    created = {"id": entity_id, "name": payload["name"], "zone_id": payload["zoneId"], "class": payload["class"], "vertices_m": payload["verticesM"], "construction_id": payload["constructionId"], "outside_boundary_condition": boundary, "sun_exposed": payload["sunExposed"], "wind_exposed": payload["windExposed"], "multiplier": payload["multiplier"]}
    rows = after["model"]["surfaces"]
    position = next((index for index, row in enumerate(rows) if row["id"] > entity_id), len(rows))
    if payload.get("index") is not None:
        position = payload["index"]
    rows.insert(position, created)
    return after, applied()


def _invert_create_surface(before, payload):
    return [("delete-surface", {"id": payload["id"]})]


def delete_surface(before, payload):
    """🪚️ `delete-surface{id}` — the one cascading delete in this vocabulary: fenestrations and
    adjacency pairs go with their host surface, an interzone partner blocks it."""
    entity_id = payload["id"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if any(row["outside_boundary_condition"] == {"Interzone": entity_id} for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.target-referenced", [str(entity_id)])
    windows = [row for row in before["model"]["fenestrations"] if row["surface_id"] == entity_id]
    pairs = [row for row in before["model"]["adjacency_pairs"] if entity_id in (row["surface_a_id"], row["surface_b_id"])]
    after = copy.deepcopy(before)
    after["model"]["surfaces"] = [row for row in after["model"]["surfaces"] if row["id"] != entity_id]
    after["model"]["fenestrations"] = [row for row in after["model"]["fenestrations"] if row["surface_id"] != entity_id]
    after["model"]["adjacency_pairs"] = [row for row in after["model"]["adjacency_pairs"] if entity_id not in (row["surface_a_id"], row["surface_b_id"])]
    if not windows and not pairs:
        return after, applied()
    return after, applied(("info", "mutation.cascade"))


def _invert_delete_surface(before, payload):
    entity_id = payload["id"]
    index, item = next((position, row) for position, row in enumerate(before["model"]["surfaces"]) if row["id"] == entity_id)
    boundary = item["outside_boundary_condition"]
    partner = boundary["Interzone"] if isinstance(boundary, dict) else None
    steps = [("create-surface", {"id": item["id"], "name": item["name"], "zoneId": item["zone_id"], "class": item["class"], "verticesM": item["vertices_m"], "constructionId": item["construction_id"], "boundary": "Interzone" if partner is not None else boundary, "interzoneSurfaceId": partner, "sunExposed": item["sun_exposed"], "windExposed": item["wind_exposed"], "multiplier": item["multiplier"], "index": index})]
    for window_index, window in enumerate(before["model"]["fenestrations"]):
        if window["surface_id"] == entity_id:
            steps.append(("create-fenestration", {"id": window["id"], "name": window["name"], "surfaceId": window["surface_id"], "uValueWM2k": window["u_value_w_m2k"], "shgc": window["shgc"], "vlt": window["vlt"], "areaM2": window["area_m2"], "heightM": window["height_m"], "sillHeightM": window["sill_height_m"], "frameConductanceWK": window["frame_conductance_w_k"], "dividerConductanceWK": window["divider_conductance_w_k"], "overhangDepthM": window["overhang_depth_m"], "overhangOffsetM": window["overhang_offset_m"], "finDepthM": window["fin_depth_m"], "finOffsetM": window["fin_offset_m"], "glazingConstructionId": window["glazing_construction_id"], "index": window_index}))
            if window["vertices_m"]:
                steps.append(("replace-fenestration-vertices", {"id": window["id"], "newVerticesM": window["vertices_m"]}))
    for pair_index, pair in enumerate(before["model"]["adjacency_pairs"]):
        if entity_id in (pair["surface_a_id"], pair["surface_b_id"]):
            steps.append(("connect-surfaces", {"surfaceAId": pair["surface_a_id"], "surfaceBId": pair["surface_b_id"], "index": pair_index}))
    return steps


def rename_surface(before, payload):
    """🏳️ `rename-surface{id,newName}` — Sets one surface's identity field; a name another surface already holds is refused because every surface-level report keys on it."""
    entity_id, value = payload["id"], payload["newName"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not value.strip():
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if any(row["id"] != entity_id and row["name"] == value for row in before["model"]["surfaces"]):
        return unchanged(before), rejected("mutation.duplicate-id", [str(entity_id)])
    if item["name"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["name"] = value
    return after, applied()


def _invert_rename_surface(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("rename-surface", {"id": payload["id"], "newName": item["name"]})]


def change_surface_zone(before, payload):
    """🗜️ `change-surface-zone{id,newZoneId}` — Reassigns one surface to another existing zone — the zone whose air heat balance the surface's inside face exchanges with."""
    entity_id, value = payload["id"], payload["newZoneId"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["zones"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["zone_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["zone_id"] = value
    return after, applied()


def _invert_change_surface_zone(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-zone", {"id": payload["id"], "newZoneId": item["zone_id"]})]


def change_surface_class(before, payload):
    """🧩️ `change-surface-class{id,newClass}` — Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone."""
    entity_id, value = payload["id"], payload["newClass"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["class"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["class"] = value
    return after, applied()


def _invert_change_surface_class(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-class", {"id": payload["id"], "newClass": item["class"]})]


def replace_surface_vertices(before, payload):
    """🔺️ `replace-surface-vertices{id,newVerticesM}` — Swaps the surface's whole polygon. Geometry is one value — moving a single corner would leave the other vertices describing a different plane — so the taxonomy's `replace` verb carries the full ring, minimum three vertices."""
    entity_id, value = payload["id"], payload["newVerticesM"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if len(value) < 3:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["vertices_m"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["vertices_m"] = value
    return after, applied()


def _invert_replace_surface_vertices(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("replace-surface-vertices", {"id": payload["id"], "newVerticesM": item["vertices_m"]})]


def change_surface_construction(before, payload):
    """🧰️ `change-surface-construction{id,newConstructionId}` — Points the surface at another existing layered construction — the layer stack the conduction transfer functions are derived from."""
    entity_id, value = payload["id"], payload["newConstructionId"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if not any(row["id"] == value for row in before["model"]["constructions"]):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["construction_id"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["construction_id"] = value
    return after, applied()


def _invert_change_surface_construction(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-construction", {"id": payload["id"], "newConstructionId": item["construction_id"]})]


def change_surface_boundary_condition(before, payload):
    """🚧️ `change-surface-boundary-condition{id,newBoundary,newInterzoneSurfaceId}` — the tagged
    union's two halves, admitted only when they agree."""
    entity_id, tag, partner = payload["id"], payload["newBoundary"], payload["newInterzoneSurfaceId"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if (tag == "Interzone") != (partner is not None):
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if partner is not None and (partner == entity_id or not any(row["id"] == partner for row in before["model"]["surfaces"])):
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    boundary = {"Interzone": partner} if partner is not None else tag
    if item["outside_boundary_condition"] == boundary:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["outside_boundary_condition"] = boundary
    return after, applied()


def _invert_change_surface_boundary_condition(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    boundary = item["outside_boundary_condition"]
    partner = boundary["Interzone"] if isinstance(boundary, dict) else None
    return [("change-surface-boundary-condition", {"id": payload["id"], "newBoundary": "Interzone" if partner is not None else boundary, "newInterzoneSurfaceId": partner})]


def change_surface_sun_exposed(before, payload):
    """🌅️ `change-surface-sun-exposed{id,newSunExposed}` — Sets whether the outside face receives beam and diffuse solar at all."""
    entity_id, value = payload["id"], payload["newSunExposed"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["sun_exposed"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["sun_exposed"] = value
    return after, applied()


def _invert_change_surface_sun_exposed(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-sun-exposed", {"id": payload["id"], "newSunExposed": item["sun_exposed"]})]


def change_surface_wind_exposed(before, payload):
    """🍃️ `change-surface-wind-exposed{id,newWindExposed}` — Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one."""
    entity_id, value = payload["id"], payload["newWindExposed"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if item["wind_exposed"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["wind_exposed"] = value
    return after, applied()


def _invert_change_surface_wind_exposed(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-wind-exposed", {"id": payload["id"], "newWindExposed": item["wind_exposed"]})]


def change_surface_multiplier(before, payload):
    """🔁️ `change-surface-multiplier{id,newMultiplier}` — Sets how many identical instances of the surface the heat balance is scaled by."""
    entity_id, value = payload["id"], payload["newMultiplier"]
    item = next((row for row in before["model"]["surfaces"] if row["id"] == entity_id), None)
    if item is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if value == 0:
        return unchanged(before), rejected("mutation.invariant", [str(entity_id)])
    if item["multiplier"] == value:
        return unchanged(before), no_op()
    after = copy.deepcopy(before)
    for row in after["model"]["surfaces"]:
        if row["id"] == entity_id:
            row["multiplier"] = value
    return after, applied()


def _invert_change_surface_multiplier(before, payload):
    item = next(row for row in before["model"]["surfaces"] if row["id"] == payload["id"])
    return [("change-surface-multiplier", {"id": payload["id"], "newMultiplier": item["multiplier"]})]


VOCABULARY = {
    "rename-model": rename_model,
    "change-model-version": change_model_version,
    "change-site-latitude": change_site_latitude,
    "change-site-longitude": change_site_longitude,
    "change-site-elevation": change_site_elevation,
    "change-site-time-zone": change_site_time_zone,
    "change-site-north-axis": change_site_north_axis,
    "change-ground-building": change_ground_building,
    "change-ground-shallow": change_ground_shallow,
    "change-ground-deep": change_ground_deep,
    "change-run-start-month": change_run_start_month,
    "change-run-start-day": change_run_start_day,
    "change-run-end-month": change_run_end_month,
    "change-run-end-day": change_run_end_day,
    "change-run-year": change_run_year,
    "replace-airflow-network": replace_airflow_network,
    "add-output-variable": add_output_variable,
    "remove-output-variable": remove_output_variable,
    "bind-weather-file": bind_weather_file,
    "unbind-weather-file": unbind_weather_file,
    "connect-referenced-model": connect_referenced_model,
    "disconnect-referenced-model": disconnect_referenced_model,
    "rename-zone": rename_zone,
    "change-zone-volume": change_zone_volume,
    "change-zone-multiplier": change_zone_multiplier,
    "change-zone-conditioned": change_zone_conditioned,
    "change-zone-floor-area-participation": change_zone_floor_area_participation,
    "create-zone": create_zone,
    "delete-zone": delete_zone,
    "create-space": create_space,
    "delete-space": delete_space,
    "rename-space": rename_space,
    "change-space-floor-area": change_space_floor_area,
    "change-space-zone": change_space_zone,
    "create-surface": create_surface,
    "delete-surface": delete_surface,
    "rename-surface": rename_surface,
    "change-surface-zone": change_surface_zone,
    "change-surface-class": change_surface_class,
    "replace-surface-vertices": replace_surface_vertices,
    "change-surface-construction": change_surface_construction,
    "change-surface-boundary-condition": change_surface_boundary_condition,
    "change-surface-sun-exposed": change_surface_sun_exposed,
    "change-surface-wind-exposed": change_surface_wind_exposed,
    "change-surface-multiplier": change_surface_multiplier,
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
    "create-zone": _invert_create_zone,
    "delete-zone": _invert_delete_zone,
    "create-space": _invert_create_space,
    "delete-space": _invert_delete_space,
    "rename-space": _invert_rename_space,
    "change-space-floor-area": _invert_change_space_floor_area,
    "change-space-zone": _invert_change_space_zone,
    "create-surface": _invert_create_surface,
    "delete-surface": _invert_delete_surface,
    "rename-surface": _invert_rename_surface,
    "change-surface-zone": _invert_change_surface_zone,
    "change-surface-class": _invert_change_surface_class,
    "replace-surface-vertices": _invert_replace_surface_vertices,
    "change-surface-construction": _invert_change_surface_construction,
    "change-surface-boundary-condition": _invert_change_surface_boundary_condition,
    "change-surface-sun-exposed": _invert_change_surface_sun_exposed,
    "change-surface-wind-exposed": _invert_change_surface_wind_exposed,
    "change-surface-multiplier": _invert_change_surface_multiplier,
}
# endregion 🔖️Vocabulary
