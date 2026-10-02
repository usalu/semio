"""🐍️ Independent oracle of the history-edit runtime corpus (`🧫️fixtures/🧫️history-edit-runtime/🔣️.json`).

Validates the corpus against its JSON Schema and every leaf (authored, ingested, and edited) against its puzzle 2d payload
schema with `jsonschema`. It then folds each named head with shapely's affine transforms, using the select-tool oracle's
fold (`🧪️select-tool-history/🐍️.py`, which was written from the leaf schemas and design §8, not from the Rust). Each head is
the board with the scenario's `log` folded, after the leaves its `edits` name are replaced or dropped. The fold must give
the head's nodes and must not contain its absent ids. It also checks that every head a step names exists and that every
`row` a step names is an authored edit.

Exits non-zero on the first disagreement. An optional argument names another corpus, for negative controls.
"""

import importlib.util
import json
import math
import pathlib
import sys

import jsonschema

EDITOR = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("select_tool_history_oracle", EDITOR / "🧪️tests" / "🧪️select-tool-history" / "🐍️.py")
SELECT_TOOL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SELECT_TOOL)
TOLERANCE = 1e-9


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def edited_log(scenario, head):
    edits = {edit["index"]: edit for edit in head["edits"]}
    for index, leaf in enumerate(scenario["log"]):
        edit = edits.get(index)
        if edit is None:
            yield leaf
        elif "leaf" in edit:
            yield edit["leaf"]


def leaves(scenario):
    yield from scenario["log"]
    for head in scenario["heads"].values():
        yield from (edit["leaf"] for edit in head["edits"] if "leaf" in edit)
    yield from (step["ingest"] for step in scenario["steps"] if "ingest" in step)


def main():
    corpus = load(pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else EDITOR / "🧫️fixtures" / "🧫️history-edit-runtime" / "🔣️.json")
    jsonschema.Draft7Validator(load(EDITOR / "🧬️schema" / "🔣️history-edit-runtime" / "🔣️.json")).validate(corpus)
    checked = 0
    for scenario in corpus["scenarios"]:
        name = scenario["id"]
        for leaf in leaves(scenario):
            SELECT_TOOL.leaf_validator(leaf["mutation"]).validate(leaf)
        for step in scenario["steps"]:
            expect = step.get("expect", {})
            if "head" in expect and expect["head"] not in scenario["heads"]:
                sys.exit(f"{name}: a step names the unknown head {expect['head']}")
            rows = [outcome["row"] for outcome in expect.get("outcomes", [])] + [expect[key]["row"] for key in ("nextProblem", "words") if key in expect]
            if isinstance(step.get("begin"), dict):
                rows.append(step["begin"]["row"])
            if any(row >= len(scenario["log"]) for row in rows):
                sys.exit(f"{name}: a step names a row beyond the {len(scenario['log'])} authored edits")
        for head_name, head in scenario["heads"].items():
            nodes = SELECT_TOOL.fold(corpus["board"], list(edited_log(scenario, head)))
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
    print(f"history-edit-runtime oracle: {len(corpus['scenarios'])} scenarios, {checked} head nodes agree")


if __name__ == "__main__":
    main()
