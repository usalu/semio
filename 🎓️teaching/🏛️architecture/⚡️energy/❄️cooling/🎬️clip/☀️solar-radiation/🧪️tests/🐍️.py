"""🧪 Kühlung solar scenes aim rays and the frame factor box at the outer sash."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


class CoolingSolarWindowTest(unittest.TestCase):
    def test_house_and_room_rays_use_outer_leaf(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(CASES["houseRays"]["required"], text)
        self.assertNotIn(CASES["houseRays"]["forbid"], text)
        self.assertIn(CASES["roomRays"]["required"], text)
        self.assertNotIn(CASES["roomRays"]["forbid"], text)

    def test_frame_factor_hugs_outer_sash(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(CASES["frame"]["required"], text)
        self.assertNotIn(CASES["frame"]["forbid"], text)


if __name__ == "__main__":
    unittest.main()
