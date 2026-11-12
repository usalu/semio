"""🧪 Guard of the Excel clips: every clip plays its morphs through declared transitions, no Excel clip touches morph internals or a Manim shape transform, and every Excel transition document is valid, schema-checked by the third-party oracle and fully referenced by its clip."""

from __future__ import annotations

import ast
import importlib
import json
import re
import unittest
from pathlib import Path

jsonschema = importlib.import_module("jsonschema")
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")

ARCHITECTURE = Path(__file__).resolve().parents[4]
CLIPS = ARCHITECTURE / "📊️excel" / "🎬️clip"
BIM_INFORMATION = ARCHITECTURE / "🏗️bim" / "🎬️clip" / "📊️information"
ALLOWED = frozenset({"stage", "token"})
FORBIDDEN = frozenset({"Transform", "ReplacementTransform", "TransformFromCopy", "FadeTransform", "FadeTransformPieces", "TransformMatchingShapes", "TransformMatchingTex", "ClockwiseTransform",
                       "CounterclockwiseTransform", "TransformAnimations"})
INTERNALS = ("🎞️full", "🖼️preview", "⏱️timeline", "📍️placement", "🎭️stage", "📋️spec", "📊️instrument", "🪙️token")
ORACLE = jsonschema.Draft202012Validator(json.loads(spec.SCHEMA_PATH.read_text(encoding="utf-8")))
OWNED = {"🦴️anatomy": "excel.anatomy.", "⚡️energy": "excel.energy.", "📋️program": "excel.program."}


def modules():
    """📚 Every Excel clip module, series helpers and widgets included, tests excluded."""
    return [path for path in sorted(CLIPS.rglob("🐍️.py")) if "🧪️tests" not in path.parts and "__pycache__" not in path.parts]


def violations(tree):
    """🚫 Direct Manim transforms, morph facade members beyond the allowed ones and imports of morph internals in a module."""
    found = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Attribute) and isinstance(node.value, ast.Name) and node.value.id == "_morph" and node.attr not in ALLOWED:
            found.append((node.lineno, f"_morph.{node.attr}"))
        elif isinstance(node, (ast.Name, ast.Attribute)) and (node.id if isinstance(node, ast.Name) else node.attr) in FORBIDDEN:
            found.append((node.lineno, node.id if isinstance(node, ast.Name) else node.attr))
        elif isinstance(node, ast.ImportFrom):
            found += [(node.lineno, alias.name) for alias in node.names if alias.name in FORBIDDEN]
        elif isinstance(node, ast.Constant) and isinstance(node.value, str) and (node.value in FORBIDDEN or ("morph" in node.value and any(f".{name}." in node.value for name in INTERNALS))):
            found.append((node.lineno, node.value))
    return found


def documents():
    """📄 The transition documents of the three owned Excel clips and the BIM information clip as (clip module, document, id prefix)."""
    for folder, prefix in OWNED.items():
        yield CLIPS / folder / "🐍️.py", CLIPS / folder / "🔀️transitions" / "🔣️.json", prefix
    yield BIM_INFORMATION / "🐍️.py", BIM_INFORMATION / "🔀️transitions" / "🔣️.json", "bim.information."


class Guard(unittest.TestCase):
    def test_excel_modules_are_found(self):
        self.assertGreaterEqual(len(modules()), 7)

    def test_no_excel_clip_uses_morph_internals_or_a_manim_shape_transform(self):
        problems = [f"{path.relative_to(ARCHITECTURE)}:{line} uses {name}; declare the change in <clip>/🔀️transitions/🔣️.json and play stage.transition(id, source=..., target=...)"
                    for path in modules() for line, name in violations(ast.parse(path.read_text(encoding="utf-8")))]
        self.assertEqual(problems, [])

    def test_guard_detects_every_form(self):
        tree = ast.parse("_morph.morph(a, b)\nfrom manim import Transform\nx = mn.ReplacementTransform(a, b)\ny = getattr(m, 'TransformFromCopy')\n"
                         "importlib.import_module('🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️')\n_morph.stage(s, p)\n_morph.token('a')\n")
        self.assertEqual(sorted(name for _, name in violations(tree)), sorted(["_morph.morph", "Transform", "ReplacementTransform", "TransformFromCopy", "🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️"]))


class Documents(unittest.TestCase):
    def test_documents_are_valid_for_spec_and_oracle_and_round_trip(self):
        for _, path, prefix in documents():
            with self.subTest(path.relative_to(ARCHITECTURE).as_posix()):
                raw = json.loads(path.read_text(encoding="utf-8"))
                self.assertEqual(list(ORACLE.iter_errors(raw)), [])
                document = spec.load(path)
                self.assertEqual(spec.parse(spec.dump(document)), document)
                self.assertEqual(path.read_text(encoding="utf-8"), spec.dumps(raw))
                self.assertTrue(all(item.id.startswith(prefix) for item in document.transitions))
                self.assertEqual(len({item.id for item in document.transitions}), len(document.transitions))

    def test_every_declared_transition_is_played_and_every_played_one_is_declared(self):
        for clip, path, prefix in documents():
            text = clip.read_text(encoding="utf-8")
            literals = set(re.findall(r'"([a-z0-9][a-z0-9.\-]*)"', text))
            declared = {item.id for item in spec.load(path).transitions}
            for identifier in declared:
                with self.subTest(identifier):
                    self.assertTrue(identifier in literals or identifier.removeprefix(prefix) in literals, f"{clip.parent.name} never plays {identifier}")
            for call in re.findall(r'(?:transition|fly|move|collect|_fly|_swarm|_ghosted)\(\s*"([a-z0-9][a-z0-9.\-]*)"', text):
                with self.subTest(call):
                    self.assertIn(call if call.startswith(prefix) else prefix + call, declared)

    def test_teaching_critical_intermediates_stay_full_morphs_in_the_documents(self):
        wanted = {"excel.energy.ht.arrows-to-total": "gather", "excel.energy.ua.cells-to-arrows": "scatter", "excel.energy.mistake.b-to-item": "parts", "excel.energy.fix.b-to-cell": "parts",
                  "excel.program.countifs.seeds-to-capsules": "scatter", "excel.program.fix.ring-to-ok": "gather", "excel.program.typo.dot-to-ring": "parts"}
        recipes = {item.id: item.recipe for _, path, _ in documents() for item in spec.load(path).transitions}
        for identifier, recipe in wanted.items():
            with self.subTest(identifier):
                self.assertEqual(recipes[identifier], recipe)

    def test_durations_equal_the_authored_clip_timings(self):
        authored = {"excel.anatomy.workbook.tower-to-file": 1.6, "excel.anatomy.sheets.capsules-to-tabs": 2.0, "excel.energy.mistake.b-to-item": 2.0, "excel.energy.sheet.capsule-to-large": 1.6,
                    "excel.program.towers.capsules-to-sums": 0.7, "bim.information.context.raw-to-dimension": 5.0}
        durations = {item.id: item.timing.duration for _, path, _ in documents() for item in spec.load(path).transitions}
        for identifier, seconds in authored.items():
            with self.subTest(identifier):
                self.assertEqual(durations[identifier], seconds)


if __name__ == "__main__":
    unittest.main()
