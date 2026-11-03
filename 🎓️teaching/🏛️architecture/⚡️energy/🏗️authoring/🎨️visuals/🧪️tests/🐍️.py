"""🧪 Architectural section drawings and dimensions against Manim geometry."""

from __future__ import annotations

import importlib
import json
import unittest
from pathlib import Path

import numpy as np

VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class ArchitectureTest(unittest.TestCase):
    def test_house_material_has_real_wall_thickness_and_closed_fills(self):
        case = CASES["house"]
        house = VISUALS.house_section()
        self.assertAlmostEqual(house["wall_right"].width, case["wall"])
        ratio = house["wall_right"].width / case["storey"]
        self.assertTrue(case["wall_to_storey"][0] <= ratio <= case["wall_to_storey"][1])
        for group in (house["walls"], house["roof"]):
            for part in group.get_family():
                if len(getattr(part, "points", [])):
                    self.assertTrue(part.is_closed())
                    self.assertGreater(part.get_fill_opacity(), 0.5)

    def test_wall_tops_follow_the_roof_underside_across_full_thickness(self):
        case = CASES["house"]
        house = VISUALS.house_section()
        slope = case["roof_rise"] / (house["w_width"] / 2 + case["eave"])
        expected = house["y_c"] + slope * (case["eave"] + case["wall"]) - case["roof_depth"]
        self.assertAlmostEqual(house["roof_wall_top"], expected)
        self.assertGreater(house["roof_wall_top"], house["y_c"])
        for wall in (house["walls"][0][-1], house["wall_right"]):
            self.assertAlmostEqual(wall.get_top()[1], expected)

    def test_house_material_raster_has_no_roof_wall_air_gap(self):
        from manim import Camera, VGroup
        from scipy.ndimage import label

        case = CASES["house"]
        house = VISUALS.house_section()
        material = VGroup(house["floor"], house["walls"], house["roof"]).copy()
        camera = Camera(pixel_width=1200, pixel_height=1200, frame_height=4.8, frame_width=4.8,
                        frame_center=np.array([0, 0.5, 0]), background_color=VISUALS.P_DEEP_DARK)
        camera.capture_mobjects([material])
        mask = np.asarray(camera.get_image())[:, :, 0] > 100
        _, components = label(mask)
        self.assertEqual(components, case["material_components"])
        slope = case["roof_rise"] / (house["w_width"] / 2 + case["eave"])
        for side in (-1, 1):
            for inward in (0.035, 0.07, 0.105):
                x = side * (house["w_width"] / 2 - inward)
                roof_bottom = house["y_c"] + slope * (case["eave"] + inward) - case["roof_depth"]
                for y in np.linspace(house["y_c"] - 0.02, roof_bottom + 0.02, 15):
                    column = int(round((x / 4.8 + 0.5) * 1200))
                    row = int(round((0.5 - (y - 0.5) / 4.8) * 1200))
                    self.assertTrue(mask[row, column], f"Air gap at roof-wall joint {(x, y)}")

    def test_room_material_meets_at_corners_without_gaps(self):
        case = CASES["room"]
        room = VISUALS.room_section([0, 0, 0], w=case["width"], h=case["height"], wall=case["wall"], slab=case["slab"])
        self.assertAlmostEqual(room["floor"].get_top()[1], room["wall_r"].get_bottom()[1])
        self.assertAlmostEqual(room["ceiling"].get_bottom()[1], room["wall_r"].get_top()[1])
        self.assertAlmostEqual(room["wall_lo"].get_right()[0], room["x_l"])
        for part in room["shell"].get_family():
            if len(getattr(part, "points", [])):
                self.assertTrue(all(np.allclose(contour[0], contour[-1]) for contour in part.get_subpaths()))
                self.assertGreater(part.get_fill_opacity(), 0.5)

    def test_open_window_uses_oblique_projection_and_follows_section_transform(self):
        from manim import Transform

        case = CASES["window"]
        window = VISUALS.window_glyph(case["x"], case["sill"], case["lintel"], depth=case["depth"])
        window["group"].scale(1.4).shift(np.array([0.8, -0.2, 0]))
        animation = VISUALS.open_window(window)
        self.assertIsInstance(animation, Transform)
        target = animation.target_mobject[0]
        vertices = target.get_vertices()
        near = np.linalg.norm(vertices[3] - vertices[0])
        far = np.linalg.norm(vertices[2] - vertices[1])
        self.assertTrue(case["far_edge_height_ratio"][0] <= far / near <= case["far_edge_height_ratio"][1])
        self.assertGreater(target.width, window["sash"].width * 4)
        self.assertAlmostEqual(vertices[0][0], window["sash"][0].get_vertices()[0][0])


    def test_window_closing_and_reopening_use_the_same_hinge(self):
        case = CASES["window"]
        window = VISUALS.window_glyph(case["x"], case["sill"], case["lintel"], depth=case["depth"])
        closed = window["reference"].get_vertices().copy()
        for fraction in (1, 0.35, 0, 0.7):
            target = VISUALS.window_opening(window, fraction=fraction)
            np.testing.assert_allclose(target[0].get_vertices()[0], closed[0])
            window["sash"].become(target)
        np.testing.assert_allclose(VISUALS.window_opening(window, fraction=0)[0].get_vertices(), closed)
        tilted = VISUALS.window_opening(window, tilt=True)[0].get_vertices()
        np.testing.assert_allclose(tilted[:2], closed[:2])
        self.assertGreater(tilted[3][0], closed[3][0])
        self.assertLess(tilted[3][1], closed[3][1])

    def test_both_room_walls_have_apertures_when_requested(self):
        room = VISUALS.room_section([0, 0, 0], window_side="both")
        self.assertIsNotNone(room["right_glass"])
        self.assertEqual(len(room["wall_l"]), 2)
        self.assertEqual(len(room["wall_r"]), 2)
        for side in (room["wall_l"], room["wall_r"]):
            self.assertAlmostEqual(side[0].get_top()[1], room["win_lo"])
            self.assertAlmostEqual(side[1].get_bottom()[1], room["win_hi"])



if __name__ == "__main__":
    unittest.main()
