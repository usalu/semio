"""🧪 Thermal-mass room keeps the outer-leaf casement for night ventilation."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


class ThermalMassWindowTest(unittest.TestCase):
    def test_beat6_reuses_outer_leaf_casement(self):
        text = SOURCE.read_text(encoding="utf-8")
        beat = text.split("class Beat6_ThermischeMasse")[1].split("class ")[0]
        self.assertIn(CASES["thermalMassWindow"]["reuse"], beat)
        self.assertIn(CASES["thermalMassWindow"]["wall"], beat)
        for needle in CASES["thermalMassWindow"]["forbid"]:
            self.assertNotIn(needle, beat)


if __name__ == "__main__":
    unittest.main()
