"""🧪 Fensterlüftung Qv panel keeps the room outer-leaf casement."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


class QvWindowOuterTest(unittest.TestCase):
    def test_fensterlueftung_reuses_room_casement(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(CASES["qvWindow"]["reuse"], text)
        self.assertNotIn(CASES["qvWindow"]["forbid"], text)


if __name__ == "__main__":
    unittest.main()
