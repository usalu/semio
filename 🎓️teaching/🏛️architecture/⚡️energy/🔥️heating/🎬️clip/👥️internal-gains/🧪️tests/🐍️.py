"""🧪 Heating internal-gains lamp stays at room scale."""

from __future__ import annotations

import ast
import importlib
import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class LightingLampScaleTest(unittest.TestCase):
    def test_beleuchtung_lamp_uses_room_metre(self):
        case = CASES["lampUnit"]
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(f"B4_ROOM_H = {case['roomHeight']}", text)
        self.assertIn(f"B4_LAMP_UNIT = {case['expression']}", text)
        self.assertNotIn(f"B4_LAMP_UNIT = {case['forbiddenUnit']}", text)
        unit = VISUALS.room_metre(case["roomHeight"])
        self.assertLessEqual(unit / VISUALS.room_metre(case["roomHeight"]), case["maxUnitOverRoomMetre"])


if __name__ == "__main__":
    unittest.main()
