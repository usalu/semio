"""🧪 Conduction beat layout contracts."""

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


class Beat4WattCaptionTest(unittest.TestCase):
    def test_u_caption_is_single_line(self):
        case = CASES["beat4WattCaption"]
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


class BridgeMethodLayoutTest(unittest.TestCase):
    def test_bridge_method_notes_clear_formula_band(self):
        case = CASES["bridgeMethodLayout"]
        body = _scene_body(SOURCE.read_text(encoding="utf-8"), case["sceneClass"])
        self.assertIn(f"base_y, k_h = {case['baseY']}, {case['kH']}", body)
        note_y = case["baseY"] - 0.66
        gap = abs(note_y - (-1.66))
        self.assertGreaterEqual(gap, case["minNoteFormulaGap"])


class BridgePsiDiffPanelTest(unittest.TestCase):
    def test_psi_labels_clear_before_diff_panel(self):
        case = CASES["bridgePsiDiffPanel"]
        body = _scene_body(SOURCE.read_text(encoding="utf-8"), case["sceneClass"])
        for needle in case["required"]:
            self.assertIn(needle, body)
        for needle in case["forbidden"]:
            self.assertNotIn(needle, body)
        # Manim panel edge buff lifts the formula above the default band.
        visuals = __import__("importlib").import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        self.assertGreater(1.95, visuals.FORMULA_PANEL_EDGE_BUFF)


if __name__ == "__main__":
    unittest.main()
