"""🧪 Internal gains clip draws only staffage casts, and the metabolic story keeps its watt values."""

from __future__ import annotations

import ast
import importlib
import json
from pathlib import Path
import re
import unittest

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


def manifest():
    """📜 Load the shared cast manifest of the course visuals."""
    module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
    return module._manifest()["casts"]


class InternalGainsStaffageTest(unittest.TestCase):
    def test_no_ad_hoc_figures_remain(self):
        text = SOURCE.read_text(encoding="utf-8")
        for pattern in CASES["forbidden"]:
            with self.subTest(pattern=pattern):
                self.assertIsNone(re.search(pattern, text))
        self.assertFalse((SOURCE.parent / "🏃️activity").exists())

    def test_every_cast_in_use_is_listed_and_heat_figures_are_thermal(self):
        text = SOURCE.read_text(encoding="utf-8")
        used = set(re.findall(r'_figure\(\s*"([a-z-]+)"', text)) | set(re.findall(r'_lamp\([^)]*"(lamp-dimmed)"\)', text)) | {"lamp"} | set(re.findall(r"\"([a-z-]+)\"[,}]", re.search(r"_ACTIVITY_CAST = {.*}", text).group(0)))
        casts = manifest()
        shared = ast.literal_eval(re.search(r"_SHARED = ({.*})", text).group(1))
        key = lambda name: shared.get(name, CASES["cast_prefix"] + name)
        for name in used:
            with self.subTest(cast=name):
                entry = casts[key(name)]
                self.assertEqual(entry["theme"], "dark")
                if name in CASES["thermal"]:
                    self.assertEqual(entry["view"], "thermal")
        for kind, asset in CASES["activity_assets"].items():
            with self.subTest(activity=kind):
                self.assertIn(asset, {casts[key(name)]["asset"] for name in used})

    def test_metabolic_activity_values_are_preserved(self):
        tree = ast.parse(SOURCE.read_text(encoding="utf-8"))
        comparison = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat2_HumanFactor")
        assignment = next(node for node in ast.walk(comparison) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "acts" for target in node.targets))
        self.assertEqual([row.elts[1].value for row in assignment.value.elts], CASES["metabolic_watts"])


if __name__ == "__main__":
    unittest.main()
