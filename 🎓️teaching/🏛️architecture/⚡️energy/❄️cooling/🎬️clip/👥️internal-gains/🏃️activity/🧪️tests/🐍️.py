"""🧪 Filled activity silhouettes at a shared human scale and connected raster topology."""

from __future__ import annotations

import importlib
import json
from pathlib import Path
import unittest

import numpy as np

SCENE = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class ActivityTest(unittest.TestCase):
    def test_profiles_are_closed_opaque_and_keep_the_same_human_reference_size(self):
        from manim import Rectangle

        for case in CASES["poses"]:
            for scale in CASES["scales"]:
                with self.subTest(pose=case["kind"], scale=scale):
                    figure, heat = SCENE._activity_figure(case["kind"], s=scale)
                    body = figure[CASES["body_index"]]
                    contours = [part for part in body.get_family() if len(getattr(part, "points", []))]
                    self.assertEqual(len(contours), CASES["body_contours"])
                    self.assertTrue(all(part.is_closed() for part in contours))
                    self.assertTrue(all(part.get_fill_opacity() == CASES["fill_opacity"] for part in contours))
                    oracle = Rectangle(**{case["axis"]: CASES["reference_size"] * scale})
                    self.assertAlmostEqual(getattr(body, case["axis"]), getattr(oracle, case["axis"]))
                    self.assertAlmostEqual(body.get_bottom()[1], case["bottom"] * scale)
                    np.testing.assert_allclose(heat, np.array([*case["heat"], 0]) * scale)

    def test_rendered_bodies_are_connected_solid_and_emit_heat_from_inside_the_torso(self):
        from manim import Camera
        from scipy.ndimage import binary_fill_holes, label

        for case in CASES["poses"]:
            for scale in CASES["scales"]:
                with self.subTest(pose=case["kind"], scale=scale):
                    figure, heat = SCENE._activity_figure(case["kind"], color="#FFFFFF", s=scale)
                    body = figure[CASES["body_index"]]
                    span = max(body.width, body.height) * 1.5
                    center = body.get_center()
                    camera = Camera(pixel_width=384, pixel_height=384, frame_height=span, frame_width=span,
                                    frame_center=center, background_color="#000000")
                    camera.capture_mobjects([body])
                    mask = np.asarray(camera.get_image())[:, :, 0] > 100
                    _, components = label(mask, structure=np.ones((3, 3)))
                    self.assertEqual(components, CASES["connected_components"])
                    self.assertLessEqual(int(binary_fill_holes(mask).sum() - mask.sum()), CASES["maximum_hole_pixels"])
                    self.assertGreater(float(mask.mean()), CASES["minimum_coverage"])
                    column = int(round(((heat[0] - center[0]) / span + 0.5) * 384))
                    row = int(round((0.5 - (heat[1] - center[1]) / span) * 384))
                    self.assertTrue(mask[row, column])

    def test_seated_pose_and_metabolic_activity_values_are_preserved(self):
        from manim import ORIGIN

        actual, heat = SCENE._activity_figure("sit", s=1)
        expected = SCENE.seated_person_glyph(ORIGIN, color=SCENE.P_ORANGE, scale=2.2)
        np.testing.assert_allclose(actual.get_all_points(), expected["group"].get_all_points())
        np.testing.assert_allclose(heat, expected["chest"])
        import ast
        tree = ast.parse(Path(SCENE.__file__).read_text(encoding="utf-8"))
        comparison = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat2_HumanFactor")
        assignment = next(node for node in ast.walk(comparison) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "acts" for target in node.targets))
        self.assertEqual([row.elts[1].value for row in assignment.value.elts], CASES["metabolic_watts"])


if __name__ == "__main__":
    unittest.main()
