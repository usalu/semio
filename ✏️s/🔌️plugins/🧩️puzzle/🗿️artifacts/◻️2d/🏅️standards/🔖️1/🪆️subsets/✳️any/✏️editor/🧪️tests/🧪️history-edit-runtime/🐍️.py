"""🐍️ Independent oracle of the history-edit runtime corpus (`🧫️fixtures/🧫️history-edit-runtime/🔣️.json`).

Validates actual history records and every leaf (authored, ingested, and edited) against its puzzle 2d payload
schema with `jsonschema`. It then folds each named head with shapely, using the select-tool oracle's fold of the selection
leaves (`🧪️select-tool-history/🐍️.py`, which was written from the leaf schemas and design §8, not from the Rust). Each head
is the scenario's board with its `log` folded, after the leaves its `edits` name are replaced or dropped. The fold must give
the head's nodes, must not contain its absent ids, must hold exactly its `edges` and must report exactly its `outcomes`.

A `connectHandles` leaf adds its edge. One that states a `tolerance` is a recorded proximity (design §22.13): where its two
handles are farther apart than the tolerance on the document it folds onto, or one is on no node, the fold reports
`mutation.precondition-drifted` for both handles. A handle sits on its node's rim: a circle's east-zero angle on the radius,
a rectangle's north-zero angle on its outline.

It also checks that every head a step names exists, that every `row` a step names is an authored edit, and that a review
step's drift outcomes name the handles its previewed head folds to.

Exits non-zero on the first disagreement. An optional argument names another corpus, for negative controls.
"""

import importlib.util
import json
import math
import pathlib
import sys

import jsonschema
from referencing import Registry, Resource
from shapely import affinity
from shapely.geometry import LineString, Point, box

EDITOR = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("select_tool_history_oracle", EDITOR / "🧪️tests" / "🧪️select-tool-history" / "🐍️.py")
SELECT_TOOL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SELECT_TOOL)
TOLERANCE = 1e-9
CONNECT = "connectHandles"
DRIFTED = "mutation.precondition-drifted"


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def validator(kind, cache={}):
    if kind != CONNECT:
        return SELECT_TOOL.leaf_validator(kind)
    if kind not in cache:
        cache[kind] = jsonschema.Draft7Validator(load(SELECT_TOOL.MUTATIONS / "🪢️connect-handles" / "🧬️schema" / "🔣️.json"))
    return cache[kind]


def edited_log(scenario, head):
    edits = {edit["index"]: edit for edit in head["edits"]}
    for index, leaf in enumerate(scenario["log"]):
        edit = edits.get(index)
        if edit is None:
            yield index, leaf
        elif "leaf" in edit:
            yield index, edit["leaf"]


def leaves(scenario):
    yield from scenario["log"]
    for head in scenario["heads"].values():
        yield from (edit["leaf"] for edit in head["edits"] if "leaf" in edit)
    yield from (step["ingest"] for step in scenario["steps"] if "ingest" in step)


def rim(node, handle):
    centre = Point(node["x"], node["y"])
    if node.get("shape") != "rectangle":
        return affinity.rotate(Point(centre.x + node.get("radius", 24.0), centre.y), handle["angle"], origin=centre, use_radians=True)
    half_width, half_height = node.get("width", 48.0) / 2.0, node.get("height", 48.0) / 2.0
    reach = 2.0 * math.hypot(half_width, half_height)
    ray = LineString([centre, (centre.x - math.sin(handle["angle"]) * reach, centre.y - math.cos(handle["angle"]) * reach)])
    return ray.intersection(box(centre.x - half_width, centre.y - half_height, centre.x + half_width, centre.y + half_height).exterior)


def fold(board, log):
    edges, outcomes, selection = [edge["id"] for edge in board["edges"]], [], []
    for index, leaf in log:
        if leaf["mutation"] != CONNECT:
            selection.append(leaf)
            continue
        if leaf["id"] in edges:
            continue
        handles = {handle["id"]: (node, handle) for node in SELECT_TOOL.fold(board, selection).values() for handle in node.get("handles", [])}
        ends = [handles.get(leaf[end]) for end in ("source", "target")]
        tolerance = leaf.get("tolerance")
        if tolerance is not None and (None in ends or rim(*ends[0]).distance(rim(*ends[1])) > tolerance):
            outcomes.append({"index": index, "code": DRIFTED, "target": [leaf["source"], leaf["target"]]})
        edges.append(leaf["id"])
    return SELECT_TOOL.fold(board, selection), edges, outcomes


def main():
    corpus = load(pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else EDITOR / "🧫️fixtures" / "🧫️history-edit-runtime" / "🔣️.json")
    domain = load(EDITOR / "🧮️history" / "🧬️schema" / "🔣️.json")
    documents = [load(SELECT_TOOL.MUTATIONS / "🔣️.json"), *[load(path) for path in SELECT_TOOL.MUTATIONS.glob("*/🧬️schema/🔣️.json")]]
    registry = Registry().with_resources((document["$id"], Resource.from_contents(document)) for document in documents)
    for scenario in corpus["scenarios"]:
        for head in scenario["heads"].values():
            for edit in head["edits"]:
                jsonschema.validate(edit, {**domain, "$ref": "#/definitions/HistoryEdit"}, registry=registry)
            for outcome in head.get("outcomes", []):
                jsonschema.validate(outcome, {**domain, "$ref": "#/definitions/FoldedOutcome"})
        for step in scenario["steps"]:
            expected = step.get("expect", {})
            for outcome in expected.get("outcomes", []):
                jsonschema.validate(outcome, {**domain, "$ref": "#/definitions/RuntimeOutcome"})
            for label in expected.get("historyEdits", []):
                jsonschema.validate(label, {**domain, "$ref": "#/definitions/LocalizedLabel"})
    checked, drifted = 0, 0
    for scenario in corpus["scenarios"]:
        name = scenario["id"]
        for leaf in leaves(scenario):
            validator(leaf["mutation"]).validate(leaf)
        folded = {head_name: fold(scenario.get("board", corpus["board"]), list(edited_log(scenario, head))) for head_name, head in scenario["heads"].items()}
        for step in scenario["steps"]:
            expect = step.get("expect", {})
            for key in ("head", "preview"):
                if key in expect and expect[key] not in scenario["heads"]:
                    sys.exit(f"{name}: a step names the unknown head {expect[key]}")
            rows = [outcome["row"] for outcome in expect.get("outcomes", [])] + [expect[key]["row"] for key in ("nextProblem", "words") if key in expect]
            if isinstance(step.get("begin"), dict):
                rows.append(step["begin"]["row"])
            if any(row >= len(scenario["log"]) for row in rows):
                sys.exit(f"{name}: a step names a row beyond the {len(scenario['log'])} authored edits")
            reviewed = [outcome["target"] for outcome in expect.get("outcomes", []) if outcome["code"] == DRIFTED]
            if "preview" in expect and reviewed != [outcome["target"] for outcome in folded[expect["preview"]][2]]:
                sys.exit(f"{name}: a review reports the drift of {reviewed}, its previewed head {expect['preview']} folds to {folded[expect['preview']][2]}")
        for head_name, head in scenario["heads"].items():
            nodes, edges, outcomes = folded[head_name]
            for node_id, expected in head["nodes"].items():
                node = nodes.get(node_id)
                if node is None:
                    sys.exit(f"{name}/{head_name}: {node_id} is missing from the folded head")
                if not (math.isclose(node["x"], expected["x"], abs_tol=TOLERANCE) and math.isclose(node["y"], expected["y"], abs_tol=TOLERANCE)):
                    sys.exit(f"{name}/{head_name}: {node_id} folds to ({node['x']}, {node['y']}), the corpus says ({expected['x']}, {expected['y']})")
                if node.get("locked", False) != expected.get("locked", False):
                    sys.exit(f"{name}/{head_name}: {node_id} lock folds to {node.get('locked', False)}")
                checked += 1
            if any(node_id in nodes for node_id in head.get("absent", [])):
                sys.exit(f"{name}/{head_name}: an absent node is in the folded head")
            if "edges" in head and sorted(edges) != sorted(head["edges"]):
                sys.exit(f"{name}/{head_name}: the fold holds the edges {sorted(edges)}, the corpus says {sorted(head['edges'])}")
            if "outcomes" in head:
                if any(outcome["code"] != DRIFTED for outcome in head["outcomes"]):
                    sys.exit(f"{name}/{head_name}: this oracle folds only {DRIFTED} outcomes")
                if outcomes != head["outcomes"]:
                    sys.exit(f"{name}/{head_name}: the fold reports {outcomes}, the corpus says {head['outcomes']}")
                drifted += len(outcomes)
    print(f"history-edit-runtime oracle: {len(corpus['scenarios'])} scenarios, {checked} head nodes agree, {drifted} drift outcomes agree")


if __name__ == "__main__":
    main()
