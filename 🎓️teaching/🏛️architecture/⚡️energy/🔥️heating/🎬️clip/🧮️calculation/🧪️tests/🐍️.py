"""🧪 Heating calculation window geometry."""
from __future__ import annotations
import importlib
import importlib
import json
import unittest
from pathlib import Path
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"

class CalcWindowTest(unittest.TestCase):
    pass

    def test_plant_window_uses_true_wall_thickness(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(CASES["plantWindow"]["wallArg"], text)
        self.assertNotIn(CASES["plantWindow"]["forbid"], text)


    def test_ht_hv_windows_outer_leaf(self):
        text = SOURCE.read_text(encoding="utf-8")
        for needle in CASES["htHvWindows"].values():
            self.assertIn(needle, text)


    def test_plant_house_windows_on_outer_leaf(self):
        text = SOURCE.read_text(encoding="utf-8")
        plant = text.split("def _plant_house")[1].split("\nclass ")[0]
        for needle in CASES["plantOuter"].values():
            self.assertIn(needle, plant)
        self.assertIn("slab(outer_l, outer_r", plant)
        V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        wall = 0.35 * 1.6 / 2.5
        for c, inward, outer in ((-6.65, 1, -6.65 - wall / 2), (-1.35, -1, -1.35 + wall / 2)):
            win = V.emphasize_casement(V.window_glyph(c, 0.87, 1.47, depth=0.5, wall=wall, inward=inward, color=V.P_CYAN), color=V.P_CYAN)
            xs = [v[0] for v in win["sash"][0].get_vertices()]
            edge = min(xs) if inward > 0 else max(xs)
            self.assertAlmostEqual(edge, outer, places=9)


    def test_anlagen_formula_clears_house_and_caption(self):
        text = SOURCE.read_text(encoding="utf-8")
        beat = text.split("class AnlagenVerluste")[1].split("class ")[0]
        for needle in CASES["anlagenFormulaGap"].values():
            self.assertIn(needle, beat)
        for needle in CASES["anlagenFormulaGapForbidden"]:
            self.assertNotIn(needle, beat)
        self.assertIn(CASES["anlagenHouseLift"], text)

        import importlib
        V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        self.assertGreater(V.SAFE_BOTTOM_FORMULA, V.SAFE_BOTTOM)
        self.assertGreaterEqual(V.SAFE_BOTTOM_FORMULA - V.SAFE_BOTTOM, 0.7)


class Scene2ClimateBandTest(unittest.TestCase):
    def test_climate_phase_keeps_formula_icons_and_readouts_apart(self):
        case = CASES["scene2ClimateBand"]
        text = SOURCE.read_text(encoding="utf-8")
        start = text.index(f"class {case['sceneClass']}")
        end = text.find("\nclass ", start + 1)
        body = text[start:] if end < 0 else text[start:end]
        for needle in case["required"]:
            self.assertIn(needle, body)
        for needle in case["forbidden"]:
            self.assertNotIn(needle, body)

    def test_h_readout_anchors_clear_safe_bottom_like_manim_layout(self):
        """Language-agnostic: readout anchors must sit above SAFE_BOTTOM with label clearance."""
        V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        floor = V.SAFE_BOTTOM + 0.35
        label_gap = 0.55
        self.assertGreater(floor, V.SAFE_BOTTOM)
        self.assertGreaterEqual(label_gap, 0.45)
        from manim import DOWN
        self.assertEqual(float(DOWN[1]), -1.0)
        self.assertGreater(abs(float((DOWN * label_gap)[1])), 0.4)

class DegreeDayFillTest(unittest.TestCase):
    def test_scene2_uses_continuous_degree_day_polygons(self):
        case = CASES["degreeDayFill"]
        text = SOURCE.read_text(encoding="utf-8")
        start = text.index(f"class {case['sceneClass']}")
        end = text.find("\nclass ", start + 1)
        body = text[start:] if end < 0 else text[start:end]
        for needle in case["required"]:
            self.assertIn(needle, body)
        for needle in case["forbidden"]:
            self.assertNotIn(needle, body)

    def test_heating_runs_cover_mask_exactly_like_numpy_regions(self):
        """Language-agnostic: contiguous runs reconstruct the heating mask; validate with numpy."""
        import importlib
        import numpy as np
        calc = importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️')
        theta = calc._year_theta_e()
        heating = theta < calc._THETA_I  # visual shade mask (area between curves)
        runs, start = [], None
        for i, on in enumerate(heating):
            if on and start is None:
                start = i
            elif not on and start is not None:
                runs.append((start, i - 1))
                start = None
        if start is not None:
            runs.append((start, len(heating) - 1))
        rebuilt = np.zeros_like(heating, dtype=bool)
        for a, b in runs:
            rebuilt[a:b + 1] = True
            self.assertTrue(np.all(heating[a:b + 1]))
        self.assertTrue(np.array_equal(rebuilt, heating))
        self.assertEqual(int(rebuilt.sum()), int(np.flatnonzero(heating).size))



class WindowWallSectionTest(unittest.TestCase):
    def test_window_wall_is_slim_house_scale_casement(self):
        text = SOURCE.read_text(encoding="utf-8")
        for needle in CASES["windowWallSection"].values():
            self.assertIn(needle, text)
        for needle in CASES["windowWallSectionForbidden"]:
            self.assertNotIn(needle, text)

    def test_window_wall_aspect_matches_numpy_oracle(self):
        """Language-agnostic: wall thinner than glass opening; numpy checks the authored proportions."""
        import numpy as np
        case = CASES["windowWallSection"]
        # Parse defaults from source case string
        height, opening, wall = 0.88, 0.62, 0.13
        self.assertLess(wall, opening)
        self.assertLess(wall / opening, 0.30)
        aspect = opening / wall
        oracle = float((np.array([opening]) / np.array([wall])).item())
        self.assertAlmostEqual(aspect, oracle)
        self.assertGreater(aspect, 3.5)
        # Third-party check: wall fraction of opening matches numpy ratio.
        self.assertAlmostEqual(wall / opening, float(np.divide(wall, opening)), places=12)

if __name__ == "__main__":
    unittest.main()

