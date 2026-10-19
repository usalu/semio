"""📆️ Schedule and layer kinds: space lists, enclosures, schedules, time series, window layers and finishes."""

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


c1 = _load("mutate_energy_model_1_schedules_schedules", "⏱️schedules")
c2 = _load("mutate_energy_model_1_schedules_glazing", "🧊️glazing-and-materials")


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
    **c1.VECTOR_ROOTS,
    **c2.VECTOR_ROOTS,
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
    **c1.VOCABULARY,
    **c2.VOCABULARY,
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
    **c1.EXTRA_INVERT,
    **c2.EXTRA_INVERT,
}
# endregion 🔖️Vocabulary
