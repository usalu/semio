"""🧪 Solar-gains house keeps outer casements; loss path arrows stay hidden."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
SOURCE = Path(__file__).resolve().parents[1] / "🐍️.py"


def _beat5(text: str) -> str:
    start = text.index("class Beat5_Verschattung")
    end = text.find("\nclass ", start + 1)
    return text[start:] if end < 0 else text[start:end]


class SolarHouseGuidesTest(unittest.TestCase):
    def test_loss_path_arrows_are_hidden(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn("flow_guides(loss_paths", text)
        self.assertIn(CASES["lossGuides"]["required"], text)
        self.assertNotIn("FadeIn(loss_guides)", text)

    def test_irradiance_wall_is_translucent(self):
        text = SOURCE.read_text(encoding="utf-8")
        self.assertIn(f"set_fill(opacity={CASES['irradianceWall']['fillOpacity']})", text)
        self.assertLess(CASES["irradianceWall"]["fillOpacity"], 1.0)

    def test_shade_wall_flush_with_floor_outer(self):
        text = SOURCE.read_text(encoding="utf-8")
        for needle in CASES["shadeFacade"]["required"]:
            self.assertIn(needle, text)
        self.assertNotIn("slab_left, room_end_x = lintel.get_right()", text)
        self.assertNotIn("inward=-1))", _beat5(text))


class ShadeWindowSectionTest(unittest.TestCase):
    def test_facade_and_slab_share_realistic_thickness(self):
        case = CASES["shadeFacade"]
        body = _beat5(SOURCE.read_text(encoding="utf-8"))
        self.assertIn(f"facade_t = {case['facadeThickness']}", body)
        self.assertIn(f"t = {case['slabThickness']}", body)
        self.assertLessEqual(
            case["facadeThickness"] / case["slabThickness"],
            case["maxFacadeToSlabRatio"],
        )
        for needle in case["required"]:
            self.assertIn(needle, body)
        for needle in CASES["shadeWindow"]["forbidden"]:
            self.assertNotIn(needle, body)

    def test_outer_leaf_sash_is_capped_for_thick_walls(self):
        visuals = __import__("importlib").import_module(
            "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️"
        )
        thick = visuals.window_glyph(0.0, -1.0, 1.0, wall=0.5, depth=1.2, inward=1)
        thin = visuals.window_glyph(0.0, -1.0, 1.0, wall=0.18, depth=1.2, inward=1)
        thick_span = abs(thick["sash"][0].get_right()[0] - thick["sash"][0].get_left()[0])
        thin_span = abs(thin["sash"][0].get_right()[0] - thin["sash"][0].get_left()[0])
        self.assertLessEqual(thick_span, CASES["shadeWindow"]["paneCap"] + 1e-9)
        self.assertAlmostEqual(thin_span, 0.18 * 0.85, places=6)




class HauptOverlapTest(unittest.TestCase):
    def test_token_ghosts_are_removed_and_mass_panel_is_banded(self):
        case = CASES["hauptOverlap"]
        text = SOURCE.read_text(encoding="utf-8")
        start = text.index(f"class {case['sceneClass']}")
        end = text.find("\nclass ", start + 1)
        if end < 0:
            end = text.find("\n#endregion", start + 1)
        body = text[start:] if end < 0 else text[start:end]
        for needle in case["required"]:
            self.assertIn(needle, body)
        for needle in case["forbidden"]:
            self.assertNotIn(needle, body)

    def test_wipe_zeros_math_text_leaves_like_manim_text(self):
        """Language-agnostic: `_wipe` must match Manim leaf opacity after an explicit zeroing."""
        import importlib

        case = CASES["hauptOverlap"]
        solar = importlib.import_module(
            "🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.☀️solar-gains.🐍️"
        )
        visuals = importlib.import_module(
            "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️"
        )
        from manim import Text, VGroup

        class _StubScene:
            def __init__(self):
                self.mobjects = []

            def remove(self, *mobs):
                for mob in mobs:
                    if mob in self.mobjects:
                        self.mobjects.remove(mob)

        glyphs = VGroup(*[
            visuals.math_text(src, font_size=36, color="#FFD166")
            for src in case["wipeSymbols"]
        ])
        scene = _StubScene()
        scene.mobjects.append(glyphs)
        solar._wipe(scene, glyphs)

        ref = Text("ref", font_size=36)
        for leaf in ref.family_members_with_points():
            leaf.set_fill(opacity=0)
            leaf.set_stroke(opacity=0)
            leaf.set_opacity(0)
        ref_opacity = min(leaf.get_fill_opacity() for leaf in ref.family_members_with_points())
        leaf_ops = [leaf.get_fill_opacity() for leaf in glyphs.family_members_with_points()]
        self.assertTrue(leaf_ops)
        self.assertTrue(all(op == case["wipeLeafOpacity"] for op in leaf_ops))
        self.assertEqual(ref_opacity, case["wipeLeafOpacity"])
        self.assertNotIn(glyphs, scene.mobjects)


if __name__ == "__main__":
    unittest.main()
