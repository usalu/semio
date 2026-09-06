"""🐍️ `s.energy.model`'s second, independent implementation of its own mutation vocabulary.

No third-party library reads or writes `.dsl.semio` — the recorded survey named and DECLINED
EnergyPlus and OpenStudio, and the `energyplus` weather reader already registered under
`✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset reads a different format for a different purpose, so it is
deliberately not reused here. The reference is therefore a second IMPLEMENTATION, written from this
subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json`, each kind's own
`../../🧬️schema/🧬️mutations/<dir>/🧬️.schema.json`, and
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/`'s `📓️taxonomy.md`
verb table and `📓️derivation-rules.md` shape rules. It imports nothing from the Rust it judges and
transliterates none of it.

The honest boundary this file used to carry is gone with `replace-model`: a whole-document swap is
not a mutation at all (rule 6), so there is no longer a kind whose only vector is a no-op. Every kind
below is exercised by one vector that moves the document and one that is refused.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome
# endregion 🔖️Imports


# region 🔖️Fixtures
#: 📂 `scenario id -> asset:// root`, mirroring the feature's `Examples` tables.
VECTOR_ROOTS = {
    "rename-model-renames-the-model": "asset://🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/✅️renames-the-model",
    "rename-model-refuses-a-blank-name": "asset://🧬️schema/🧬️mutations/🏷️rename-model/🧪️tests/⛔️refuses-a-blank-name",
    "change-model-version-bumps-the-version": "asset://🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/✅️bumps-the-version",
    "change-model-version-refuses-a-blank-version": "asset://🧬️schema/🧬️mutations/🔢️change-model-version/🧪️tests/⛔️refuses-a-blank-version",
    "update-site-relocates-to-denver": "asset://🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/✅️relocates-to-denver",
    "update-site-refuses-a-bad-latitude": "asset://🧬️schema/🧬️mutations/🌍️update-site/🧪️tests/⛔️refuses-a-bad-latitude",
    "update-ground-temperature-sets-denver-ground": "asset://🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/✅️sets-denver-ground",
    "update-ground-temperature-refuses-a-short-year": "asset://🧬️schema/🧬️mutations/🌡️update-ground-temperature/🧪️tests/⛔️refuses-a-short-year",
    "update-run-period-shortens-to-january": "asset://🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/✅️shortens-to-january",
    "update-run-period-refuses-month-13": "asset://🧬️schema/🧬️mutations/📅️update-run-period/🧪️tests/⛔️refuses-month-13",
    "replace-airflow-network-attaches-a-network": "asset://🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/✅️attaches-a-network",
    "replace-airflow-network-refuses-unpaired-nodes": "asset://🧬️schema/🧬️mutations/🫧️replace-airflow-network/🧪️tests/⛔️refuses-unpaired-nodes",
    "add-output-variable-adds-zone-air-temp": "asset://🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/✅️adds-zone-air-temp",
    "add-output-variable-refuses-a-duplicate": "asset://🧬️schema/🧬️mutations/📊️add-output-variable/🧪️tests/⛔️refuses-a-duplicate",
    "remove-output-variable-drops-zone-air-temp": "asset://🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/✅️drops-zone-air-temp",
    "remove-output-variable-refuses-an-absent-one": "asset://🧬️schema/🧬️mutations/📉️remove-output-variable/🧪️tests/⛔️refuses-an-absent-one",
    "bind-weather-file-binds-hannover-epw": "asset://🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/✅️binds-hannover-epw",
    "bind-weather-file-refuses-a-bad-uri": "asset://🧬️schema/🧬️mutations/🌦️bind-weather-file/🧪️tests/⛔️refuses-a-bad-uri",
    "unbind-weather-file-unbinds-the-weather": "asset://🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/✅️unbinds-the-weather",
    "unbind-weather-file-refuses-when-unbound": "asset://🧬️schema/🧬️mutations/🌤️unbind-weather-file/🧪️tests/⛔️refuses-when-unbound",
    "connect-referenced-model-connects-the-geometry": "asset://🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/✅️connects-the-geometry",
    "connect-referenced-model-refuses-a-bad-uri": "asset://🧬️schema/🧬️mutations/🪢️connect-referenced-model/🧪️tests/⛔️refuses-a-bad-uri",
    "disconnect-referenced-model-disconnects-the-geometry": "asset://🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/✅️disconnects-the-geometry",
    "disconnect-referenced-model-refuses-when-absent": "asset://🧬️schema/🧬️mutations/✂️disconnect-referenced-model/🧪️tests/⛔️refuses-when-absent",
    "rename-zone-renames-zone-one": "asset://🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/✅️renames-zone-one",
    "rename-zone-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/🏠️rename-zone/🧪️tests/⛔️refuses-a-missing-zone",
    "change-zone-volume-resizes-zone-one": "asset://🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/✅️resizes-zone-one",
    "change-zone-volume-refuses-zero-volume": "asset://🧬️schema/🧬️mutations/📦️change-zone-volume/🧪️tests/⛔️refuses-zero-volume",
    "change-zone-multiplier-stacks-four-storeys": "asset://🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/✅️stacks-four-storeys",
    "change-zone-multiplier-refuses-zero-instances": "asset://🧬️schema/🧬️mutations/✖️change-zone-multiplier/🧪️tests/⛔️refuses-zero-instances",
    "change-zone-conditioned-frees-the-zone": "asset://🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/✅️frees-the-zone",
    "change-zone-conditioned-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/🌬️change-zone-conditioned/🧪️tests/⛔️refuses-a-missing-zone",
    "change-zone-floor-area-participation-excludes-the-zone": "asset://🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/✅️excludes-the-zone",
    "change-zone-floor-area-participation-refuses-a-missing-zone": "asset://🧬️schema/🧬️mutations/📐️change-zone-floor-area-participation/🧪️tests/⛔️refuses-a-missing-zone",
}


def _read_json(ctx: Context, uri: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(uri))


def _vector(ctx: Context, scenario: str):
    root = VECTOR_ROOTS[scenario]
    return (
        _read_json(ctx, f"{root}/📸️snapshot/⬅️before/🔣️.json"),
        _read_json(ctx, f"{root}/🦠️mutation/🔣️.json"),
        _read_json(ctx, f"{root}/📸️snapshot/➡️after/🔣️.json"),
        _read_json(ctx, f"{root}/🎯️outcome/🔣️.json"),
    )
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 Splits the committed mutation document into its kind tag and its argument object. The wire
    tag is the lowerCamel spelling of the Rust variant (`renameModel`), not the kebab catalog id."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


def wire_tag(kind: str) -> str:
    """🔤 The catalog id (kebab) a wire tag maps to, read off the committed manifest's
    `productionDispatch`, never transliterated from a Rust variant name."""
    head, *rest = kind.split("-")
    return head + "".join(part.capitalize() for part in rest)
# endregion 🔖️Wire


# region 🔖️Outcomes
def applied(*messages):
    """🎯️ An applied outcome and its ordered diagnostics."""
    return {"status": "applied", "messages": [{"level": level, "code": code} for level, code in messages]}


def rejected(code, path):
    """⛔️ A refusal: one fault code and the offending address."""
    return {"status": "rejected", "code": code, "path": list(path)}


def unchanged(before):
    """🪞 A refused or no-op step leaves the document exactly where it was."""
    return copy.deepcopy(before)
# endregion 🔖️Outcomes


# region 🔖️Links
def parse_uri(uri):
    """🔗️ `<artifactId>!<artifactKind>@<standard>/<subset>` — the flattened `ArtifactRef` form the
    snapshot schema's link slots carry. Anything else is not a reference."""
    if not isinstance(uri, str) or "!" not in uri:
        return None
    artifact_id, rest = uri.split("!", 1)
    if "@" not in rest or "/" not in rest:
        return None
    artifact_kind, rest = rest.split("@", 1)
    standard, subset = rest.split("/", 1)
    if not artifact_id or not artifact_kind or not standard or not subset or "/" in subset:
        return None
    return {"artifactId": artifact_id, "dialect": {"artifactKind": artifact_kind, "standard": standard, "subset": subset}}


def head_link(target, role):
    """🔗️ A head-pinned link filling one named slot."""
    return {"target": target, "pin": {"kind": "head"}, "role": role}
# endregion 🔖️Links


# region 🔖️Vocabulary
def rename_model(before, payload):
    """🏷️ `rename-model{newName}` — taxonomy.md's `rename` verb on the document identity field."""
    name = payload["newName"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [name])
    if before["model"]["name"] == name:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["name"] = name
    return after, applied()


def change_model_version(before, payload):
    """🔢️ `change-model-version{newVersion}` — one scalar field."""
    version = payload["newVersion"]
    if not version.strip():
        return unchanged(before), rejected("mutation.invariant", [version])
    if before["model"]["version"] == version:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["version"] = version
    return after, applied()


def update_site(before, payload):
    """🌍️ `update-site` — one inseparable five-field facet, all fields required every time."""
    site = {
        "latitude_deg": payload["latitudeDeg"],
        "longitude_deg": payload["longitudeDeg"],
        "elevation_m": payload["elevationM"],
        "time_zone_hours": payload["timeZoneHours"],
        "north_axis_deg": payload["northAxisDeg"],
    }
    if not -90.0 <= site["latitude_deg"] <= 90.0 or not -180.0 <= site["longitude_deg"] <= 180.0 or not -12.0 <= site["time_zone_hours"] <= 14.0:
        return unchanged(before), rejected("mutation.invariant", [])
    if before["model"]["site"] == site:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["site"] = site
    return after, applied()


def update_ground_temperature(before, payload):
    """🌡️ `update-ground-temperature` — twelve monthly values per series, read together."""
    building = payload["buildingSurfaceC"]
    shallow = payload["shallowC"]
    if len(building) != 12 or len(shallow) != 12:
        return unchanged(before), rejected("mutation.invalid-payload", [])
    ground = {"building_surface_c": building, "shallow_c": shallow, "deep_c": payload["deepC"]}
    if before["model"]["ground_temperature"] == ground:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["ground_temperature"] = ground
    return after, applied()


def update_run_period(before, payload):
    """📅️ `update-run-period` — start and end are one calendar interval."""
    run_period = {
        "start_month": payload["startMonth"],
        "start_day": payload["startDay"],
        "end_month": payload["endMonth"],
        "end_day": payload["endDay"],
        "year": payload["year"],
    }
    months_ok = 1 <= run_period["start_month"] <= 12 and 1 <= run_period["end_month"] <= 12
    days_ok = 1 <= run_period["start_day"] <= 31 and 1 <= run_period["end_day"] <= 31
    if not months_ok or not days_ok:
        return unchanged(before), rejected("mutation.invariant", [])
    if before["model"]["run_period"] == run_period:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["run_period"] = run_period
    return after, applied()


def replace_airflow_network(before, payload):
    """🫧️ `replace-airflow-network` — a whole-value swap of the document-root singleton; `present`
    false detaches it, so one kind covers attach and detach (taxonomy.md's `replace` verb)."""
    zone_ids = payload["zoneIds"]
    node_ids = payload["nodeIds"]
    link_ids = payload["linkIds"]
    if len(zone_ids) != len(node_ids):
        return unchanged(before), rejected("mutation.invalid-payload", [])
    if not payload["present"] and (zone_ids or link_ids):
        return unchanged(before), rejected("mutation.invalid-payload", [])
    network = None
    if payload["present"]:
        network = {"zone_node_ids": [[zone, node] for zone, node in zip(zone_ids, node_ids)], "outdoor_node_id": payload["outdoorNodeId"], "link_ids": link_ids}
    if before["model"]["airflow_network"] == network:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    after = copy.deepcopy(before)
    after["model"]["airflow_network"] = network
    return after, applied()


def add_output_variable(before, payload):
    """📊️ `add-output-variable` — set-like membership keyed by the natural `(name, key)` pair."""
    name, key = payload["name"], payload["key"]
    if not name.strip():
        return unchanged(before), rejected("mutation.invariant", [key])
    if any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.duplicate", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"].append({"name": name, "key": key, "reporting_frequency": payload["reportingFrequency"]})
    return after, applied()


def remove_output_variable(before, payload):
    """📉️ `remove-output-variable` — the `add` verb's inverse partner."""
    name, key = payload["name"], payload["key"]
    if not any(spec["name"] == name and spec["key"] == key for spec in before["model"]["output_variables"]):
        return unchanged(before), rejected("mutation.target-missing", [name, key])
    after = copy.deepcopy(before)
    after["model"]["output_variables"] = [spec for spec in after["model"]["output_variables"] if not (spec["name"] == name and spec["key"] == key)]
    return after, applied()


def bind_weather_file(before, payload):
    """🌦️ `bind-weather-file{targetUri}` — taxonomy.md's `bind` verb attaching a parameterization."""
    target = parse_uri(payload["targetUri"])
    if target is None:
        return unchanged(before), rejected("mutation.invalid-payload", [payload["targetUri"]])
    link = head_link(target, "weather")
    if before.get("weatherLink") == link:
        return unchanged(before), applied(("warning", "mutation.no-op"))
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
    """🪢️ `connect-referenced-model{targetUri}` — taxonomy.md's `connect` verb on a relationship."""
    target = parse_uri(payload["targetUri"])
    if target is None:
        return unchanged(before), rejected("mutation.invalid-payload", [payload["targetUri"]])
    link = head_link(target, "model")
    if before.get("referencedModel") == link:
        return unchanged(before), applied(("warning", "mutation.no-op"))
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
        return unchanged(before), rejected("mutation.duplicate", [str(entity_id)])
    if zone["name"] == name:
        return unchanged(before), applied(("warning", "mutation.no-op"))
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
        return unchanged(before), applied(("warning", "mutation.no-op"))
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
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "multiplier", multiplier), applied()


def change_zone_conditioned(before, payload):
    """🌬️ `change-zone-conditioned{id,newConditioned}` — whether equipment serves the zone."""
    entity_id, conditioned = payload["id"], payload["newConditioned"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["conditioned"] == conditioned:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "conditioned", conditioned), applied()


def change_zone_floor_area_participation(before, payload):
    """📐️ `change-zone-floor-area-participation{id,newPartOfTotalFloorArea}` — whether the zone's
    floor area counts toward the building total the normalized reports divide by."""
    entity_id, participates = payload["id"], payload["newPartOfTotalFloorArea"]
    zone = _zone(before, entity_id)
    if zone is None:
        return unchanged(before), rejected("mutation.target-missing", [str(entity_id)])
    if zone["part_of_total_floor_area"] == participates:
        return unchanged(before), applied(("warning", "mutation.no-op"))
    return _with_zone(before, entity_id, "part_of_total_floor_area", participates), applied()


#: 🗺️ Catalog id -> this file's own implementation of that kind.
VOCABULARY = {
    "rename-model": rename_model,
    "change-model-version": change_model_version,
    "update-site": update_site,
    "update-ground-temperature": update_ground_temperature,
    "update-run-period": update_run_period,
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
}

#: ↩️ Catalog id -> the undo steps that kind owes, for every kind whose spec row states its own.
EXTRA_INVERT = {
}
# endregion 🔖️Vocabulary


# region 🔖️Inverse
def invert(kind, before, payload):
    """↩️ The undo steps a kind owes, always read off BASE — never by inverting a delta. A refused or
    no-op forward step owes nothing (taxonomy.md's addressing convention)."""
    after, outcome = VOCABULARY[kind](before, payload)
    if outcome["status"] == "rejected" or after == before:
        return []
    if kind in EXTRA_INVERT:
        return EXTRA_INVERT[kind](before, payload)
    if kind == "rename-model":
        return [("rename-model", {"newName": before["model"]["name"]})]
    if kind == "change-model-version":
        return [("change-model-version", {"newVersion": before["model"]["version"]})]
    if kind == "update-site":
        site = before["model"]["site"]
        return [("update-site", {"latitudeDeg": site["latitude_deg"], "longitudeDeg": site["longitude_deg"], "elevationM": site["elevation_m"], "timeZoneHours": site["time_zone_hours"], "northAxisDeg": site["north_axis_deg"]})]
    if kind == "update-ground-temperature":
        ground = before["model"]["ground_temperature"]
        return [("update-ground-temperature", {"buildingSurfaceC": ground["building_surface_c"], "shallowC": ground["shallow_c"], "deepC": ground["deep_c"]})]
    if kind == "update-run-period":
        run_period = before["model"]["run_period"]
        return [("update-run-period", {"startMonth": run_period["start_month"], "startDay": run_period["start_day"], "endMonth": run_period["end_month"], "endDay": run_period["end_day"], "year": run_period["year"]})]
    if kind == "replace-airflow-network":
        network = before["model"]["airflow_network"]
        if network is None:
            return [("replace-airflow-network", {"present": False, "zoneIds": [], "nodeIds": [], "outdoorNodeId": 0, "linkIds": []})]
        return [("replace-airflow-network", {"present": True, "zoneIds": [pair[0] for pair in network["zone_node_ids"]], "nodeIds": [pair[1] for pair in network["zone_node_ids"]], "outdoorNodeId": network["outdoor_node_id"], "linkIds": network["link_ids"]})]
    if kind == "add-output-variable":
        return [("remove-output-variable", {"name": payload["name"], "key": payload["key"]})]
    if kind == "remove-output-variable":
        spec = next(spec for spec in before["model"]["output_variables"] if spec["name"] == payload["name"] and spec["key"] == payload["key"])
        return [("add-output-variable", {"name": spec["name"], "key": spec["key"], "reportingFrequency": spec["reporting_frequency"]})]
    if kind in ("bind-weather-file", "unbind-weather-file"):
        existing = before.get("weatherLink")
        return [("bind-weather-file", {"targetUri": _uri_of(existing)})] if existing else [("unbind-weather-file", {})]
    if kind in ("connect-referenced-model", "disconnect-referenced-model"):
        existing = before.get("referencedModel")
        return [("connect-referenced-model", {"targetUri": _uri_of(existing)})] if existing else [("disconnect-referenced-model", {})]
    zone = _zone(before, payload["id"])
    if kind == "rename-zone":
        return [("rename-zone", {"id": payload["id"], "newName": zone["name"]})]
    if kind == "change-zone-volume":
        return [("change-zone-volume", {"id": payload["id"], "newVolumeM3": zone["volume_m3"]})]
    if kind == "change-zone-multiplier":
        return [("change-zone-multiplier", {"id": payload["id"], "newMultiplier": zone["multiplier"]})]
    if kind == "change-zone-conditioned":
        return [("change-zone-conditioned", {"id": payload["id"], "newConditioned": zone["conditioned"]})]
    if kind == "change-zone-floor-area-participation":
        return [("change-zone-floor-area-participation", {"id": payload["id"], "newPartOfTotalFloorArea": zone["part_of_total_floor_area"]})]
    raise AssertionError(f"no inverse is written for {kind!r}")


def _uri_of(link):
    """🔗️ Flattens a link's `ArtifactRef` back to the URI form `bind`/`connect` payloads carry."""
    target = link["target"]
    dialect = target["dialect"]
    return f"{target['artifactId']}!{dialect['artifactKind']}@{dialect['standard']}/{dialect['subset']}"
# endregion 🔖️Inverse


# region 🔖️Oracle
def _kind_of(scenario, wire):
    tag, payload = unwrap(wire)
    for kind in VOCABULARY:
        if wire_tag(kind) == tag:
            return kind, payload
    raise AssertionError(f"unexpected wire tag {tag!r} for scenario {scenario!r}")


def _mutate_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, expected_after, expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, outcome = VOCABULARY[kind](before, payload)
        assert after == expected_after, f"mutate-{scenario}: {after} != committed after-snapshot {expected_after}"
        assert outcome == expected_outcome, f"mutate-{scenario}: {outcome} != committed outcome {expected_outcome}"
        return Outcome(projection=after, raw=json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler


def _inverse_for(scenario):
    def handler(ctx: Context) -> Outcome:
        before, wire, _expected_after, _expected_outcome = _vector(ctx, scenario)
        kind, payload = _kind_of(scenario, wire)
        after, _outcome = VOCABULARY[kind](before, payload)
        restored = after
        for step_kind, step_payload in invert(kind, before, payload):
            restored, step_outcome = VOCABULARY[step_kind](restored, step_payload)
            assert step_outcome["status"] != "rejected", f"inverse-{scenario}: the undo step {step_kind} was itself refused"
        assert restored == before, f"inverse-{scenario}: {restored} != committed before-snapshot {before}"
        return Outcome(projection=restored, raw=json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only: registering these handlers as subjects too would make the
    reference its own subject and manufacture a guaranteed-green self-comparison."""
    built = Adapter("python")
    for scenario in VECTOR_ROOTS:
        built = built.oracle(f"mutate-{scenario}", _mutate_for(scenario)).oracle(f"inverse-{scenario}", _inverse_for(scenario))
    return built
# endregion 🔖️Registration
