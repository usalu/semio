"""🧪 Guard of the migrated clips: no direct Manim shape transforms in the energy (heating, cooling, physics, ventilation, demand), optimization and BIM-model clips, and every clip transition document is valid, schema-checked by the third-party oracle and fully referenced by its clip."""

from __future__ import annotations

import ast
import importlib
import json
import re
import unittest
from pathlib import Path

jsonschema = importlib.import_module("jsonschema")
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")

ARCHITECTURE = Path(__file__).resolve().parents[3]
ENERGY = ARCHITECTURE / "⚡️energy"
GUARDED = (
    ENERGY / "🔥️heating" / "🎬️clip",
    ENERGY / "❄️cooling" / "🎬️clip",
    ENERGY / "🧲️physics" / "🎬️clip",
    ENERGY / "🌬️ventilation" / "🎬️clip",
    ENERGY / "📊️demand" / "🎬️clip",
    ENERGY / "🏗️authoring" / "🎨️visuals",
    ARCHITECTURE / "🎯️optimization" / "🎬️clip",
    ARCHITECTURE / "🏗️bim" / "🎬️clip" / "🧩️models",
)
FORBIDDEN = frozenset({"Transform", "ReplacementTransform", "TransformFromCopy", "FadeTransform", "FadeTransformPieces", "TransformMatchingShapes", "TransformMatchingTex", "ClockwiseTransform", "CounterclockwiseTransform", "TransformAnimations"})
SCHEMA = json.loads(spec.SCHEMA_PATH.read_text(encoding="utf-8"))
ORACLE = jsonschema.Draft202012Validator(SCHEMA)


def sources():
    """📚 Every guarded module (tests excluded)."""
    for folder in GUARDED:
        for path in sorted(folder.rglob("🐍️.py")):
            if "🧪️tests" not in path.parts and "__pycache__" not in path.parts:
                yield path


def violations(tree):
    """🚫 Names of forbidden Manim transforms referenced in a module (imports, calls, attributes, alias strings)."""
    found = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Name) and node.id in FORBIDDEN:
            found.append((node.lineno, node.id))
        elif isinstance(node, ast.Attribute) and node.attr in FORBIDDEN:
            found.append((node.lineno, node.attr))
        elif isinstance(node, ast.ImportFrom):
            found += [(node.lineno, alias.name) for alias in node.names if alias.name in FORBIDDEN]
        elif isinstance(node, ast.Constant) and node.value in FORBIDDEN:
            found.append((node.lineno, node.value))
    return found


def documents():
    """📄 Every clip transition document below the guarded folders."""
    for folder in GUARDED:
        yield from sorted(folder.rglob("🔀️transitions/🔣️.json"))


class ClipGuardTests(unittest.TestCase):
    def test_guarded_modules_exist(self):
        self.assertGreater(len(list(sources())), 10)

    def test_clips_construct_no_direct_manim_transforms(self):
        problems = []
        for path in sources():
            for line, name in violations(ast.parse(path.read_text(encoding="utf-8"))):
                problems.append(f"{path.relative_to(ARCHITECTURE)}:{line} uses {name}; declare the change in <clip>/🔀️transitions/🔣️.json and play stage.transition(id, source=..., target=...)")
        self.assertEqual(problems, [])

    def test_guard_detects_every_form(self):
        tree = ast.parse("from manim import Transform\nx = mn.ReplacementTransform(a, b)\ny = FadeTransform(a, b)\nz = getattr(m, 'TransformFromCopy')\n")
        self.assertEqual({name for _, name in violations(tree)}, {"Transform", "ReplacementTransform", "FadeTransform", "TransformFromCopy"})


class ClipDocumentTests(unittest.TestCase):
    def test_documents_exist_for_every_migrated_clip(self):
        self.assertGreaterEqual(len(list(documents())), 17)

    def test_documents_are_valid_for_spec_and_oracle(self):
        for path in documents():
            with self.subTest(path.relative_to(ARCHITECTURE).as_posix()):
                raw = json.loads(path.read_text(encoding="utf-8"))
                self.assertEqual(list(ORACLE.iter_errors(raw)), [])
                document = spec.load(path)
                self.assertEqual(spec.parse(spec.dump(document)), document)
                self.assertEqual(len({t.id for t in document.transitions}), len(document.transitions))

    def test_every_declared_transition_is_played_by_its_clip(self):
        for path in documents():
            clip = path.parents[1] / "🐍️.py"
            text = clip.read_text(encoding="utf-8")
            literals = set(re.findall(r'"([a-z0-9][a-z0-9.\-]*)"', text))
            templates = [re.compile("".join(".+" if part.startswith("{") else re.escape(part) for part in re.split(r"(\{[^}]*\})", template))) for template in re.findall(r'f"([^"]*\{[^"]*)"', text)]
            dynamic = bool(re.search(r"transition\(\s*(?!f?\")[^\s)]", text))
            for item in spec.load(path).transitions:
                with self.subTest(item.id):
                    played = item.id in literals or any(item.id.startswith(prefix + ".") for prefix in literals) or any(t.fullmatch(item.id) for t in templates) or dynamic
                    self.assertTrue(played, f"{clip.parent.name} never plays {item.id}")

    def test_every_played_transition_is_declared(self):
        for path in documents():
            clip = path.parents[1] / "🐍️.py"
            declared = {t.id for t in spec.load(path).transitions}
            prefixes = {t.id.rsplit(".", 1)[0] for t in spec.load(path).transitions}
            for call in re.findall(r'transition\(\s*f?"([^"{]+)"', clip.read_text(encoding="utf-8")):
                with self.subTest(call):
                    self.assertTrue(call in declared or call in prefixes, f"{clip} plays undeclared {call}")


if __name__ == "__main__":
    unittest.main()
