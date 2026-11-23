"""🧪 Internal-gains beat layout contracts."""

from __future__ import annotations

import json
import os
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


def _scene_body(text: str, scene: str) -> str:
    start = text.index(f"class {scene}")
    end = text.find("\nclass ", start + 1)
    return text[start:] if end < 0 else text[start:end]

class FormulaEdgeBuffTest(unittest.TestCase):
    def test_formula_edge_buff_clears_caption(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(f"FORMULA_EDGE_BUFF = {CASES['formulaEdgeBuff']['value']}", text)
        visuals = __import__("importlib").import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        self.assertGreaterEqual(CASES["formulaEdgeBuff"]["value"], visuals.FORMULA_PANEL_EDGE_BUFF)
        self.assertGreaterEqual(CASES["formulaEdgeBuff"]["value"], CASES["formulaEdgeBuff"]["min"])


class Beat5DinCaptionTest(unittest.TestCase):
    def test_din_caption_is_single_line(self):
        case = CASES["beat5DinCaption"]
        body = _scene_body(SOURCE.read_text(encoding="utf-8"), case["sceneClass"])
        self.assertIn(case["requiredDe"], body)
        self.assertIn(case["requiredEn"], body)
        self.assertNotIn(case["forbiddenDe"], body)
        os.environ.setdefault("SEMIO_LANGUAGE", "de")
        visuals = __import__("importlib").import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        narration = __import__("importlib").import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🐍️")
        fonts = __import__("importlib").import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        for clause in (case["requiredDe"], case["requiredEn"]):
            _label, n_lines = visuals._caption_label(
                clause, narration.plain_text(clause), fonts.CAPTION_FONT_SIZE, visuals.P_WHITE
            )
            self.assertEqual(n_lines, case["maxLines"], clause)


if __name__ == "__main__":
    unittest.main()
