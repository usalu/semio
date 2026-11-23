"""🧪 Kühlung summer strategy blinds and purge tracks sit on the outer leaf."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


class CoolingSummerWindowTest(unittest.TestCase):
    def test_blinds_and_through_use_outer_leaf(self):
        text = SOURCE.read_text(encoding="utf-8")
        for needle in CASES["required"]:
            self.assertIn(needle, text)
        for needle in CASES["forbid"]:
            self.assertNotIn(needle, text)


if __name__ == "__main__":
    unittest.main()
