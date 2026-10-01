"""🐍️ Independent oracle of the select-tool history corpus (`🧫️fixtures/🧫️select-tool-history/🔣️.json`).

Validates the corpus against its JSON Schema and every leaf against its puzzle 2d payload schema with `jsonschema`, then
re-folds each scenario's edited log (`log` with its `drafts`, withdrawn leaves dropped) over the board with shapely's
affine transforms and requires the scenario's `head`. Written from the leaf schemas and design §8, not from the Rust:
locked and missing targets are skipped, a rotation turns positions about the recorded pivot, a scaling spreads them.
Exits non-zero on the first disagreement.
"""

import json
import math
import pathlib
import sys

import jsonschema
from shapely import affinity
from shapely.geometry import Point

EDITOR = pathlib.Path(__file__).resolve().parents[2]
MUTATIONS = EDITOR.parent / "🧬️schema" / "🧬️mutations"
LEAF_DIRS = {
    "dragSelection": "✋️drag-selection",
    "rotateSelection": "🔄️rotate-selection",
    "scaleSelection": "🔍️scale-selection",
    "createNode": "🌱create-node",
    "changeNodeLocked": "🔒change-node-locked",
}
TOLERANCE = 1e-9


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def leaf_validator(kind, cache={}):
    if kind not in cache:
        cache[kind] = jsonschema.Draft7Validator(load(MUTATIONS / LEAF_DIRS[kind] / "🧬️schema" / "🔣️.json"))
    return cache[kind]


def leaves(scenario):
    yield from scenario["log"]
    yield from (draft["leaf"] for draft in scenario["drafts"] if "leaf" in draft)
    for step in scenario["steps"]:
        if "ingest" in step:
            yield step["ingest"]
        if "row" in step.get("expect", {}):
            yield step["expect"]["row"]["leaf"]


def edited_log(scenario):
    drafts = {draft["index"]: draft for draft in scenario["drafts"]}
    for index, leaf in enumerate(scenario["log"]):
        draft = drafts.get(index)
        if draft is None:
            yield leaf
        elif "leaf" in draft:
            yield draft["leaf"]


def movable(nodes, targets):
    return [nodes[target] for target in targets if target in nodes and not nodes[target].get("locked", False)]


def fold(board, log):
    nodes = {node["id"]: dict(node) for node in board["nodes"]}
    for leaf in log:
        kind = leaf["mutation"]
        if kind == "createNode":
            nodes[leaf["node"]["id"]] = dict(leaf["node"])
        elif kind == "changeNodeLocked":
            if leaf["id"] in nodes:
                nodes[leaf["id"]]["locked"] = bool(leaf["newLocked"])
        else:
            for node in movable(nodes, leaf["targets"]):
                point = Point(node["x"], node["y"])
                if kind == "dragSelection":
                    point = affinity.translate(point, xoff=leaf["dx"], yoff=leaf["dy"])
                elif kind == "rotateSelection":
                    point = affinity.rotate(point, leaf["angle"], origin=(leaf["pivotX"], leaf["pivotY"]), use_radians=True)
                elif kind == "scaleSelection":
                    point = affinity.scale(point, xfact=leaf["factor"], yfact=leaf["factor"], origin=(leaf["pivotX"], leaf["pivotY"]))
                node["x"], node["y"] = point.x, point.y
    return nodes


def main():
    corpus = load(EDITOR / "🧫️fixtures" / "🧫️select-tool-history" / "🔣️.json")
    jsonschema.Draft7Validator(load(EDITOR / "🧬️schema" / "🔣️select-tool-history" / "🔣️.json")).validate(corpus)
    checked = 0
    for scenario in corpus["scenarios"]:
        for leaf in leaves(scenario):
            leaf_validator(leaf["mutation"]).validate(leaf)
        nodes = fold(corpus["board"], list(edited_log(scenario)))
        for node_id, expected in scenario["head"].items():
            node = nodes.get(node_id)
            if node is None:
                sys.exit(f"{scenario['id']}: {node_id} is missing from the folded head")
            if not (math.isclose(node["x"], expected["x"], abs_tol=TOLERANCE) and math.isclose(node["y"], expected["y"], abs_tol=TOLERANCE)):
                sys.exit(f"{scenario['id']}: {node_id} folds to ({node['x']}, {node['y']}), the corpus says ({expected['x']}, {expected['y']})")
            if node.get("locked", False) != expected.get("locked", False):
                sys.exit(f"{scenario['id']}: {node_id} lock folds to {node.get('locked', False)}")
            checked += 1
        absent = {node_id for step in scenario["steps"] for node_id in step.get("expect", {}).get("absent", [])}
        if any(node_id in scenario["head"] for node_id in absent):
            sys.exit(f"{scenario['id']}: a node the review shows absent is in the head")
    print(f"select-tool-history oracle: {len(corpus['scenarios'])} scenarios, {checked} head nodes agree")


if __name__ == "__main__":
    main()
