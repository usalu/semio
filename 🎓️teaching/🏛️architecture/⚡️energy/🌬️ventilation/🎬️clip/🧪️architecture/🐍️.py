"""🧪 Portable ventilation architecture cases compared with Manim geometry."""

import ast
import importlib
import json
import sys
import unittest
from pathlib import Path

import numpy as np
from manim import *

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CHAPTERS = Path(__file__).resolve().parent.parent
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


def helpers(chapter):
    """🔬 Load authored geometry without voice timing or scene side effects."""
    tree = ast.parse((CHAPTERS / chapter / "🐍️.py").read_text(encoding="utf-8"))
    names = {"_room", "_stand", "_person", "_house_section", "_pf_house", "_to_scene", "_poly", "_shoelace", "_vent_room", "_open_sash", "_clip_above", "_clip_below"}
    tree.body = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name in names]
    scope = dict(globals())
    scope.update(vars(VISUALS))
    scope["BODY_FONT_SIZE"] = 24
    scope["math_label"] = lambda text, size, color: Rectangle(width=0.2, height=0.1, color=color)
    exec(compile(ast.fix_missing_locations(tree), str(CHAPTERS / chapter / "🐍️.py"), "exec"), scope)
    return scope


class VentilationArchitecture(unittest.TestCase):
    """🏛️ Geometry agrees with language-neutral dimensions and the drawing engine."""

    def test_rooms_cut_windows_and_anchor_occupants(self):
        for case in CASES["rooms"]:
            with self.subTest(chapter=case["chapter"]):
                api = helpers(case["chapter"])
                center = np.array(case["center"])
                t, w, h = case["thickness"], case["width"], case["height"]
                room = api["_room"](center, w, h, t=t, window_side="both", window_y=case["window_y"], window_h=case["window_height"])
                self.assertAlmostEqual(room.width, w + t)
                self.assertAlmostEqual(room.height, h + t)
                self.assertAlmostEqual(room[1].get_top()[1], center[1] - h / 2 + t / 2)
                leaves = [part for part in room.get_family() if part.has_points() and not part.submobjects]
                for part in leaves:
                    for key, expected in CASES["material"].items():
                        self.assertEqual(getattr(part, "get_" + key)(), expected)
                    for contour in part.get_subpaths():
                        np.testing.assert_allclose(contour[0], contour[-1])
                        for side in (-1, 1):
                            x = center[0] + side * w / 2
                            intersects_x = contour[:, 0].min() < x < contour[:, 0].max()
                            intersects_y = contour[:, 1].min() < case["window_y"] < contour[:, 1].max()
                            self.assertFalse(intersects_x and intersects_y)
                raster = CASES["raster"]
                camera = Camera(pixel_width=raster["pixel_width"], pixel_height=raster["pixel_height"],
                                frame_width=raster["frame_width"], frame_height=raster["frame_height"],
                                frame_center=center, background_color=raster["background"])
                camera.capture_mobjects([room])
                pixels = np.asarray(camera.get_image())[:, :, :3]
                for side in (-1, 1):
                    column = int(round((side * w / 2 / raster["frame_width"] + 0.5) * raster["pixel_width"]))
                    for fraction in raster["window_fractions"]:
                        y = case["window_y"] + fraction * case["window_height"]
                        row = int(round((0.5 - (y - center[1]) / raster["frame_height"]) * raster["pixel_height"]))
                        self.assertTrue(np.all(pixels[row - 1:row + 2, column - 1:column + 2] == 0))
                person = api["_stand"](center, h, center[0], t=t)
                self.assertAlmostEqual(person.height, case["occupant_ratio"] * (h - t))
                self.assertAlmostEqual(person.get_bottom()[1], room[1].get_top()[1])

    def test_small_symbolic_rooms_without_windows(self):
        for chapter in ("🪟️natural", "❓️need", "⚙️mechanical", "📦️volume"):
            room = helpers(chapter)["_room"](ORIGIN, 1.0, 0.62, t=0.07)
            self.assertAlmostEqual(room.width, 1.07)
            self.assertAlmostEqual(room.height, 0.69)

    def test_small_stack_inlet_stays_above_floor(self):
        room = helpers("🪟️natural")["_room"]([-2.9, -0.45, 0], 4.2, 1.7,
                                            window_side="left", window_y=-0.85, window_h=0.60)
        self.assertLess(room[1].get_top()[1], -0.85 - 0.30)

    def test_volume_house_uses_closed_material_and_real_apertures(self):
        api = helpers("📦️volume")
        case = CASES["house"]
        model = api["_house_section"]()
        drawn = api["_pf_house"](model, case["origin"], case["scale"])
        self.assertIn("material", drawn)
        for part in drawn["material"]:
            self.assertEqual(part.get_fill_opacity(), 1)
            for contour in part.get_subpaths():
                np.testing.assert_allclose(contour[0], contour[-1])
        self.assertEqual(len(drawn["windows"]), 4)
        for win in drawn["windows"]:
            self.assertAlmostEqual(win["wall"], model["wall"] * case["scale"])

    def test_closeup_radiator_scales_and_clears_floor(self):
        case = CASES["radiator"]
        api = helpers(case["chapter"])
        room = api["_vent_room"](case["origin_x"], case["floor_y"], case["units_per_m"], "test", VISUALS.P_CYAN)
        rad = room["rad"]
        k = case["units_per_m"]
        self.assertAlmostEqual(rad.width, case["width_m"] * k)
        self.assertAlmostEqual(rad.height, case["height_m"] * k)
        self.assertAlmostEqual(rad.get_bottom()[1], case["floor_y"] + case["floor_clearance_m"] * k)


if __name__ == "__main__":
    unittest.main()
