"""🧪 Static layout contracts of the Nakagin clips: building drawings are fitted never hard-scaled, and every numbered legend marker has exactly one filled row."""

from __future__ import annotations

import ast
import importlib
import json
import os
import sys
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
HERE = Path(__file__).resolve().parent
ROOT = next(parent for parent in HERE.parents if (parent / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
ARCHITECTURE = ROOT / "🎓️teaching/🏛️architecture"
CASES = json.loads((HERE / "🔣️.cases.json").read_text(encoding="utf-8"))


def literal_number(node) -> bool:
    """🔢 Whether an AST node is a plain numeric literal such as ``0.1`` or ``-0.1``."""
    if isinstance(node, ast.UnaryOp):
        node = node.operand
    return isinstance(node, ast.Constant) and isinstance(node.value, (int, float))


def hard_scaled_calls(path: Path) -> list:
    """🪤 Line numbers of building drawing calls that pass a literal ``scale``."""
    tree = ast.parse(path.read_text(encoding="utf-8"))
    return [node.lineno for node in ast.walk(tree) if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr in CASES["drawing_calls"]
            and any(keyword.arg == "scale" and literal_number(keyword.value) for keyword in node.keywords)]


class TestFittedDrawings(unittest.TestCase):
    def test_no_building_drawing_has_a_hard_coded_scale(self):
        for series, topics in CASES["fitted"].items():
            for topic in topics:
                path = ARCHITECTURE / series / topic / "🐍️.py"
                with self.subTest(f"{series}/{topic}"):
                    self.assertTrue(path.is_file())
                    self.assertEqual(hard_scaled_calls(path), [])


class TestLegends(unittest.TestCase):
    def test_every_marker_maps_to_exactly_one_row(self):
        scene = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️").Beat1_Anatomy
        for name, legend in CASES["legends"].items():
            terms, order = getattr(scene, legend["terms"]), getattr(scene, legend["order"])
            with self.subTest(name):
                self.assertEqual(sorted(order), list(range(1, len(terms) + 1)))
                self.assertTrue(all(english and german for english, german in terms))


if __name__ == "__main__":
    unittest.main()
