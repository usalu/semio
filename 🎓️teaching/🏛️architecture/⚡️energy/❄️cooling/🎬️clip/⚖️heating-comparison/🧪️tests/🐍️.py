"""🧪 Kühlung comparison house keeps outer-leaf casements; rays and AHU hit the outer sash."""

from __future__ import annotations

import importlib
import json
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy"


class CoolingComparisonWindowTest(unittest.TestCase):
    def test_house_windows_flush_outer_leaf_with_joinery(self):
        module = importlib.import_module(PACKAGE + ".❄️cooling.🎬️clip.⚖️heating-comparison.🐍️")
        house = module._house()
        outer = house["bottom_left"][0]
        for window in house["windows"]:
            self.assertAlmostEqual(window["outer"], outer, delta=CASES["house"]["outer_tolerance"])
            self.assertGreaterEqual(len(window["joinery"]), CASES["house"]["min_joinery"])
            self.assertGreaterEqual(len(window["sash"][1]), CASES["house"]["min_muntins"])
            xs = [v[0] for v in window["sash"][0].get_vertices()]
            self.assertAlmostEqual(min(xs), outer, delta=CASES["house"]["outer_tolerance"])

    def test_solar_and_ahu_aim_at_outer_leaf(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(CASES["solarRays"]["required"], text)
        self.assertNotIn(CASES["solarRays"]["forbid"], text)
        self.assertIn(CASES["ahu"]["required"], text)
        self.assertNotIn(CASES["ahu"]["forbid"], text)


if __name__ == "__main__":
    unittest.main()
