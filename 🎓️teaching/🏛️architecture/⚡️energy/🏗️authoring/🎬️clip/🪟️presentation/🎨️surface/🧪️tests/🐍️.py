"""🧊 Verify chip glass against the styling token projection and an independent Pillow layer composite."""

import importlib
import json
import unittest
from pathlib import Path

PRESENTATION = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class ChipGlassTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.surface = importlib.import_module(PRESENTATION + ".🎨️surface.🐍️")
        cls.theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")

    def test_levels_come_from_the_generated_token_projection_and_equal_the_token_file(self):
        self.assertEqual(self.surface.LEVELS, self.theme.TOKENS["levels"])
        self.assertEqual(self.surface.LEVELS["chipTintPercent"], CASES["chipTintPercent"])

    def test_chip_glass_matches_the_independent_two_layer_composite(self):
        from PIL import Image

        for case in CASES["chips"]:
            with self.subTest(case=case["id"]):
                paint = self.surface.chip_paint(case["content"], case["parent"], appearance=case["appearance"])
                glass = self.surface.surface_paint(paint["glass_level"], case["parent"], appearance=case["appearance"])
                foreground = self.surface._token_color(self.theme.TOKENS["appearances"][case["appearance"]]["chrome"]["foreground"])
                layers = [(glass["rgb"], glass["alpha"]), (foreground, paint["veil"])]
                canvas = Image.new("RGBA", (1, 1), tuple(round(c) for c in glass["backdrop_rgb"]) + (255,))
                for rgb, alpha in layers:
                    canvas = Image.alpha_composite(canvas, Image.new("RGBA", (1, 1), tuple(round(c) for c in rgb) + (round(alpha * 255),)))
                for actual, expected in zip(paint["composite_rgb"], canvas.getpixel((0, 0))[:3], strict=True):
                    self.assertAlmostEqual(actual, expected, delta=CASES["tolerance"])
                for actual, expected in zip(paint["composite_rgb"], case["composite"], strict=True):
                    self.assertAlmostEqual(actual, expected, delta=CASES["tolerance"])
                self.assertAlmostEqual(paint["alpha"], case["alpha"], places=6)
                self.assertEqual(paint["glass_level"], case["glassLevel"])

    def test_a_chip_is_visibly_not_the_surface_it_lies_on(self):
        for case in CASES["chips"]:
            with self.subTest(case=case["id"]):
                paint = self.surface.chip_paint(case["content"], case["parent"], appearance=case["appearance"])
                own = self.surface.surface_paint(case["content"], case["parent"], appearance=case["appearance"])
                distance = max(abs(a - b) for a, b in zip(paint["composite_rgb"], own["composite_rgb"]))
                self.assertGreaterEqual(distance, CASES["minimumContrast"])

    def test_chip_paint_validates_against_its_schema(self):
        import jsonschema

        schema = json.loads(Path(self.surface.__file__).with_name("🧬️chip.schema.json").read_text(encoding="utf-8"))
        for case in CASES["chips"]:
            jsonschema.validate(self.surface.chip_paint(case["content"], case["parent"], appearance=case["appearance"]), schema)

    def test_the_topmost_level_keeps_its_glass(self):
        self.assertEqual(self.surface.chip_paint("menu")["glass_level"], "menu")
        with self.assertRaises(ValueError):
            self.surface.chip_paint("floor")


if __name__ == "__main__":
    unittest.main()
