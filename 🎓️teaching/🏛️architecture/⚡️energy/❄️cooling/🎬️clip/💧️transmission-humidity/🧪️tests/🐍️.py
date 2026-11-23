"""🧪 Kühlung transmission tracks cross the outer leaf sash."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


class CoolingTransmissionWindowTest(unittest.TestCase):
    def test_through_uses_outer_leaf(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(CASES["required"], text)
        self.assertNotIn(CASES["forbid"], text)


if __name__ == "__main__":
    unittest.main()
