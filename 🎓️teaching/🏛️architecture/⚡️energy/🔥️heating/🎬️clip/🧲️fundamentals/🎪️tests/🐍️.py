"""🧪 Beat3 Konvektion keeps the air-loop path geometry but hides its cyan stroke."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


def _beat3_body(text: str) -> str:
    start = text.index(f"class {CASES['beat3LoopGuide']['sceneClass']}")
    end = text.index("class ", start + 1)
    return text[start:end]


class Beat3LoopGuideTest(unittest.TestCase):
    def test_air_loop_stroke_is_hidden(self):
        body = _beat3_body(SOURCE.read_text(encoding="utf-8"))
        self.assertIn("loop_guide = loop.copy().set_stroke", body)
        self.assertIn(CASES["beat3LoopGuide"]["requiredStroke"], body)
        self.assertNotIn(
            f"loop_guide = loop.copy().set_stroke(color=PASTEL_CYAN, width=2.0, {CASES['beat3LoopGuide']['forbiddenStroke']})",
            body,
        )
        self.assertIn(CASES["beat3LoopGuide"]["keepsLoopFlow"], body)

    def test_manim_zero_stroke_opacity_matches_case(self):
        from manim import VMobject

        guide = VMobject().set_stroke(opacity=float(CASES["beat3LoopGuide"]["requiredStroke"].split("=")[1]))
        self.assertEqual(guide.get_stroke_opacity(), 0.0)


def _scene_body(text: str, scene: str) -> str:
    start = text.index(f"class {scene}")
    end = text.index("\nclass ", start + 1)
    return text[start:end]


class Beat5LoopGuideTest(unittest.TestCase):
    def test_summary_air_loop_stroke_is_hidden(self):
        body = _scene_body(SOURCE.read_text(encoding="utf-8"), CASES["beat5LoopGuide"]["sceneClass"])
        self.assertIn("loop_guide = loop.copy().set_stroke", body)
        self.assertIn(CASES["beat5LoopGuide"]["requiredStroke"], body)
        self.assertNotIn(
            f"loop_guide = loop.copy().set_stroke(color=PASTEL_CYAN, width=2.0, {CASES['beat5LoopGuide']['forbiddenStroke']})",
            body,
        )


if __name__ == "__main__":
    unittest.main()
