"""🧪 Cooling system sizing captions render with the course font."""
import ast
import importlib
import json
import os
import unittest
from unittest import mock
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class SizingCaptions(unittest.TestCase):
    """🔤 The authored English clause fits and renders through Manim and Pango."""

    def test_substitution_caption(self):
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat3_IsolateAirflow")
        assignment = next(node for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
        clauses = ast.literal_eval(assignment.value)
        caption = next(row[1] for row in clauses if row[0] == "substitute")
        self.assertEqual(caption, CASES["caption"])
        with unittest.mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "en"}):
            self.assertGreater(VISUALS.caption_bar(caption).width, 0)

    def test_english_clauses_render(self):
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        for scene in (node for node in tree.body if isinstance(node, ast.ClassDef)):
            for assignment in (node for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets)):
                for key, english, _ in ast.literal_eval(assignment.value):
                    with self.subTest(scene=scene.name, clause=key):
                        self.assertGreater(VISUALS.caption_bar(english).width, 0)


class SizingChain(unittest.TestCase):
    """🔗 The numbers on screen follow one chain from the chart peak to the duct area."""

    def test_chain_matches_independent_arithmetic(self):
        chain = CASES["chain"]
        sizing = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.📐️system-sizing.🐍️")
        load = chain["areaM2"] * chain["glassShare"] * chain["blind"] * chain["gPerp"] * chain["irradianceWM2"]
        self.assertEqual(round(load, -1), chain["loadW"])
        self.assertEqual(sizing.W_SOLAR, chain["loadW"])
        self.assertAlmostEqual(sizing.Q_V_R_SOLAR, chain["loadW"] / 1000 / (1.2 * 1.0 * chain["supplyLiftK"]), places=6)
        self.assertEqual(round(sizing.Q_V_R_SOLAR, 3), chain["airflowM3S"])
        self.assertEqual(round(sizing.A_SOLAR, 4), chain["ductAreaM2"])

    def test_chart_peak_is_the_design_irradiance(self):
        solar = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.☀️solar-radiation.🐍️")
        peak = max(solar._facade_irradiance(name, hour) for name in ("Horiz", "O", "S", "W", "N") for hour in range(6, 19))
        self.assertEqual(round(peak), CASES["chain"]["irradianceWM2"])


if __name__ == "__main__":
    unittest.main()
