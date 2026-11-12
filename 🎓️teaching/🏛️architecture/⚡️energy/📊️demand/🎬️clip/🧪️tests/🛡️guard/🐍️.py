"""🧪 Guard of the demand clips: every shape transform is a declared transition, documents are valid and canonical, scenes play what they declare, authored timing equals the document, and the teaching-critical chains stay continuous."""

from __future__ import annotations

import ast
import importlib
import json
import sys
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
authoring = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.✏️authoring.🐍️")
FIXTURE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
DEMAND = next(path for path in Path(__file__).resolve().parents if path.name == "🎬️clip")


def script(clip):
    return DEMAND / clip / "🐍️.py"


def document(clip):
    return DEMAND / clip / "🔀️transitions" / "🔣️.json"


def tree(clip):
    return ast.parse(script(clip).read_text(encoding="utf-8"))


def called_name(node):
    func = node.func
    return func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None


class DemandTransitionGuard(unittest.TestCase):
    """🛡 Static guard over every demand clip."""

    def test_fixture_covers_every_demand_clip(self):
        folders = {path.parent.name for path in DEMAND.glob("*/🐍️.py")}
        self.assertEqual(set(FIXTURE["clips"]), folders)

    def test_no_direct_shape_transform_remains(self):
        found = [f"{clip}:{node.lineno} {called_name(node)}" for clip in FIXTURE["clips"] for node in ast.walk(tree(clip)) if isinstance(node, ast.Call) and called_name(node) in FIXTURE["forbidden"]]
        self.assertEqual(found, [], "declare these as transitions in the clip's 🔀️transitions/🔣️.json and play them with stage.transition")

    def test_become_is_only_analytical_or_one_shot(self):
        found = set()
        for clip in FIXTURE["clips"]:
            def visit(node, enclosing):
                for child in ast.iter_child_nodes(node):
                    name = child.name if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef)) else enclosing
                    if isinstance(child, ast.Call) and called_name(child) == "become":
                        found.add((clip, enclosing))
                    visit(child, name)

            visit(tree(clip), None)
        self.assertEqual(found, {(item["clip"], item["function"]) for item in FIXTURE["become"]})

    def test_documents_are_valid_canonical_and_complete(self):
        for clip, case in FIXTURE["clips"].items():
            with self.subTest(clip):
                path = document(clip)
                raw = json.loads(path.read_text(encoding="utf-8"))
                self.assertEqual(spec.validate(raw), [])
                self.assertEqual(path.read_text(encoding="utf-8"), spec.dumps(raw))
                self.assertGreaterEqual(len(raw["transitions"]), case["minimum"])

    def test_every_scene_uses_a_stage_and_plays_declared_transitions(self):
        for clip in FIXTURE["clips"]:
            with self.subTest(clip):
                declared = {t["id"] for t in json.loads(document(clip).read_text(encoding="utf-8"))["transitions"]}
                source = tree(clip)
                scenes = [node for node in source.body if isinstance(node, ast.ClassDef) and node.name.startswith("Beat")]
                played = set()
                for scene in scenes:
                    plays = authoring.scene_plays(script(clip), scene.name)
                    self.assertEqual(authoring.unresolved(plays, declared), [], scene.name)
                    played |= {i for i in declared if authoring.plays_id(plays, i)}
                    if plays.called or plays.patterns or plays.constants & declared:
                        calls = [n for n in ast.walk(scene) if isinstance(n, ast.Call) and called_name(n) == "stage" and isinstance(n.func, ast.Attribute)]
                        self.assertTrue(calls, f"{scene.name} plays transitions without creating a stage")
                self.assertEqual(sorted(declared - played), [], "declared but never played")

    def test_authored_run_time_equals_the_document(self):
        checked = 0
        for clip in FIXTURE["clips"]:
            with self.subTest(clip):
                durations = {t["id"]: t["timing"] for t in json.loads(document(clip).read_text(encoding="utf-8"))["transitions"]}
                for node in ast.walk(tree(clip)):
                    if isinstance(node, ast.Call) and called_name(node) == "play":
                        run = next((k.value.value for k in node.keywords if k.arg == "run_time" and isinstance(k.value, ast.Constant)), None)
                        scope = node.args
                    elif isinstance(node, ast.Tuple) and len(node.elts) == 2 and isinstance(node.elts[0], ast.Constant) and isinstance(node.elts[0].value, (int, float)) and isinstance(node.elts[1], (ast.List, ast.Lambda)):
                        run, scope = node.elts[0].value, [node.elts[1]]
                    else:
                        continue
                    if run is None:
                        continue
                    for arg in scope:
                        for inner in ast.walk(arg):
                            if isinstance(inner, ast.Call) and called_name(inner) == "transition" and inner.args and isinstance(inner.args[0], ast.Constant):
                                timing = durations[inner.args[0].value]
                                if timing["lag"]:
                                    lanes = (run - timing["duration"]) / timing["lag"]
                                    self.assertAlmostEqual(lanes, round(lanes), places=6, msg=inner.args[0].value)
                                    self.assertGreaterEqual(round(lanes), 1)
                                else:
                                    self.assertAlmostEqual(timing["duration"], run, places=9, msg=inner.args[0].value)
                                checked += 1
        self.assertGreater(checked, 60)

    def test_teaching_chains_are_continuous(self):
        for clip, case in FIXTURE["clips"].items():
            by_id = {t["id"]: t for t in json.loads(document(clip).read_text(encoding="utf-8"))["transitions"]}
            for chain in case["chains"]:
                with self.subTest(clip=clip, chain=chain[0]):
                    for earlier, later in zip(chain, chain[1:]):
                        self.assertEqual(by_id[earlier]["target"]["entity"], by_id[later]["source"]["entity"])
                        self.assertFalse(by_id[earlier]["copy"] or by_id[later]["copy"])

    def test_copies_declare_the_source_that_stays(self):
        for clip in FIXTURE["clips"]:
            for item in json.loads(document(clip).read_text(encoding="utf-8"))["transitions"]:
                if item["copy"]:
                    self.assertEqual(item["recipe"], "replace")
                    self.assertIsNotNone(item["source"])


if __name__ == "__main__":
    unittest.main()
