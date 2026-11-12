"""🧪 The references clip declares every morph as data: the document validates, the clip uses exactly its ids, no hand-built morph remains and the teaching-critical transitions keep the former timing (JSON table)."""

from __future__ import annotations

import ast
import importlib
import json
import unittest
from pathlib import Path

clip = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📍️references.🐍️")
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
full = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️")

CASES = json.loads((Path(__file__).parent / "🧫️cases" / "🔣️.json").read_text(encoding="utf-8"))
PREFIX = "excel.references."
FORBIDDEN = {"silhouette", "scatter", "gather", "split", "morph", "retext", "unfold", "flight"}


def used_names(tree):
    """🔎 Transition names the clip passes to `_to(...)`, `_bar_retext(...)` and the f4 tuple."""
    names = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute):
            if node.func.attr == "_to" and node.args:
                first = node.args[0]
                if isinstance(first, ast.Constant):
                    names.add(first.value)
                elif isinstance(first, ast.Subscript) and isinstance(first.value, ast.Tuple):
                    names.update(e.value for e in first.value.elts)
            if node.func.attr == "_bar_retext" and len(node.args) >= 4 and isinstance(node.args[3], ast.Constant):
                names.add(node.args[3].value)
    return names


class ReferencesTransitionTests(unittest.TestCase):
    def setUp(self):
        self.document = spec.load(clip.TRANSITIONS)
        self.items = {t.id.removeprefix(PREFIX): t for t in self.document.transitions}
        self.tree = ast.parse(Path(clip.__file__).read_text(encoding="utf-8"))

    def test_document_validates_against_the_schema_and_registered_correspondences(self):
        self.assertEqual(spec.validate(self.document), [])
        names = {t.correspondence.name for t in self.document.transitions if t.correspondence.kind == "recipe"}
        self.assertLessEqual(names, set(full.CORRESPONDENCES))

    def test_clip_uses_exactly_the_declared_transitions(self):
        self.assertEqual(used_names(self.tree), set(self.items))
        self.assertEqual(set(CASES["transitions"]), set(self.items))

    def test_no_hand_built_morph_remains_in_the_clip(self):
        used = {n.attr for n in ast.walk(self.tree) if isinstance(n, ast.Attribute) and isinstance(n.value, ast.Name) and n.value.id == "_morph"}
        self.assertEqual(used & FORBIDDEN, set())

    def test_timing_and_final_parameters_equal_the_former_call_arguments(self):
        for name, expected in CASES["transitions"].items():
            with self.subTest(name):
                t = self.items[name]
                self.assertEqual((t.relation, t.recipe, t.copy), (expected["relation"], expected["recipe"], expected["copy"]))
                self.assertEqual((t.timing.duration, t.timing.lag, t.timing.limit), (expected["duration"], expected["lag"], expected["limit"]))
                self.assertEqual((t.final.path_arc, t.final.dip, t.final.ghost), (expected["pathArc"], expected["dip"], expected["ghost"]))
                self.assertEqual(t.correspondence.name or t.correspondence.kind, expected["correspondence"])

    def test_teaching_critical_moves_are_declared_with_their_semantics(self):
        self.assertTrue(set(CASES["critical"]) <= set(self.items))
        for name in ("shift-retext", "repair-retext", "guard-retext", "names-retext"):
            self.assertEqual(self.items[name].recipe, "retext")
        self.assertEqual(self.items["threed-sum"].relation, "many-to-one")
        self.assertEqual(self.items["deleted-sum"].recipe, "parts")
        self.assertEqual(self.items["absolute-split"].relation, "one-to-many")
        for name in ("hook-lift", "back-lift"):
            self.assertEqual(self.items[name].final.ghost, 0.25)
        for name in ("pointer-faces", "lights-faces", "mixed2-labels"):
            self.assertEqual(self.items[name].correspondence.name, "body")
        for name in ("slide-off-first", "slide-off-second"):
            self.assertTrue(self.items[name].copy)


if __name__ == "__main__":
    unittest.main()
