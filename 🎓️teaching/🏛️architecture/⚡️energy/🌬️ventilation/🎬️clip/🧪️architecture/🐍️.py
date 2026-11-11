"""🧪 Portable ventilation architecture cases compared with Manim geometry."""

import ast
import importlib
import json
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

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
    names = {"_room", "_stand", "_remember", "_level", "_house_section", "_pf_house", "_to_scene", "_poly", "_shoelace", "_vent_room", "_open_sash", "_clip_above", "_clip_below", "_scale_marks"}
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
                target = CASES["occupant_visible_height_m"] * (h - t) / CASES["clear_storey_m"]
                figure = person.get_points_defining_boundary()
                self.assertAlmostEqual(figure[:, 1].max() - figure[:, 1].min(), target, delta=0.02 * target)
                self.assertGreaterEqual(person.height, 0.95 * target)
                self.assertLessEqual(person.height, 1.5 * target)
                self.assertAlmostEqual(person.get_bottom()[1], room[1].get_top()[1], delta=0.01 * target)

    def test_occupants_use_clear_room_metres(self):
        for case in CASES["rooms"]:
            api = helpers(case["chapter"])
            with self.subTest(chapter=case["chapter"]), patch.dict(api, {"staffage_motion": lambda cast, **kwargs: kwargs}):
                result = api["_stand"](np.array(case["center"]), case["height"], case["center"][0], t=case["thickness"])
                np.testing.assert_allclose(result["unit"], (case["height"] - case["thickness"]) / CASES["clear_storey_m"])

    def test_embedded_devices_use_the_architectural_metre(self):
        root = CHAPTERS.parents[1]
        scenes = [(CHAPTERS / "📦️volume" / "🐍️.py", "_use_glyph", "shared-mvhr-normal", {"_USE_UNIT": CASES["embedded_devices"]["mini_room_metre"]}, CASES["embedded_devices"]["mini_room_metre"]),
                  (root / "❄️cooling" / "🎬️clip" / "⚖️heating-comparison" / "🐍️.py", "_build_ahu", "ventilation-mechanical-mvhr", {"house": {"w_height": 2 * CASES["embedded_devices"]["house_storey"]}, "storey": CASES["embedded_devices"]["house_storey"], "room_metre": VISUALS.room_metre}, CASES["embedded_devices"]["house_storey"] / CASES["embedded_devices"]["clear_storey_m"])]
        scenes.append((root / "❄️cooling" / "🎬️clip" / "⚖️heating-comparison" / "🐍️.py", "_build_ahu", "physics-fundamentals-evaporator", {"metres": CASES["embedded_devices"]["house_storey"] / CASES["embedded_devices"]["clear_storey_m"]}, CASES["embedded_devices"]["house_storey"] / CASES["embedded_devices"]["clear_storey_m"]))
        for path, function, cast, scope, expected in scenes:
            tree = ast.parse(path.read_text(encoding="utf-8"))
            body = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == function)
            call = next(node for node in ast.walk(body) if isinstance(node, ast.Call) and node.args and isinstance(node.args[0], ast.Constant) and node.args[0].value == cast)
            unit = next(keyword.value for keyword in call.keywords if keyword.arg == "unit")
            np.testing.assert_allclose(eval(compile(ast.Expression(unit), str(path), "eval"), scope), expected)

    def test_scale_labels_stay_in_the_frame(self):
        api = helpers("📦️volume")
        api["math_label"] = lambda *args, **kwargs: Text("Passive house with ventilation system " * 3, font_size=36)
        case = CASES["scale_labels"]
        _, labels, _ = api["_scale_marks"]({"x": 0, "y_of": lambda value: value}, [(1, "long", WHITE)], x_text=case["x_text"])
        self.assertLessEqual(labels[0].get_right()[0], config.frame_width / 2 - case["safe_margin"] + 1e-6)
        source = (CHAPTERS / "📦️volume" / "🐍️.py").read_text(encoding="utf-8")
        self.assertIn(r'tr(r"\text{' + case["expected_en"] + r'} \approx 0.5"', source)

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
        self.assertAlmostEqual(rad.height, case["height_m"] * k, delta=0.03 * rad.height)
        self.assertAlmostEqual(rad.height, VISUALS.staffage_height("ventilation-natural-radiator-on-mirrored") * k, delta=0.03 * rad.height)
        self.assertAlmostEqual(rad.get_bottom()[1], case["floor_y"] + case["floor_clearance_m"] * k, delta=0.1 * case["floor_clearance_m"] * k)
        self.assertLess(rad.get_top()[1], room["sill"])
        self.assertLess(rad[0].get_left()[0], room["x_in"])
        self.assertGreater(rad[0].get_left()[0], room["x_in"] - 0.4 * k)


if __name__ == "__main__":
    unittest.main()
