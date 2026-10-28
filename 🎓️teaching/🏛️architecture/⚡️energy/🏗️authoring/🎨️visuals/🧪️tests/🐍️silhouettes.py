"""🖌️ Portable architectural and brush silhouette acceptance against raster oracles."""

import importlib
import json
from pathlib import Path
import unittest
import xml.etree.ElementTree as ET

import numpy as np

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️silhouettes.json").read_text(encoding="utf-8"))


def raster(body):
    """📷 Render the production geometry on an independent Manim camera."""
    from manim import Camera

    span = max(body.width, body.height) * 1.3
    camera = Camera(pixel_width=512, pixel_height=512, frame_width=span, frame_height=span,
                    frame_center=body.get_center(), background_color="#000000")
    camera.capture_mobjects([body.copy().set_color("#FFFFFF")])
    return np.asarray(camera.get_image())[:, :, 0] > 100


def component_count(mask):
    """🧮 Count eight-connected foreground regions without a third-party algorithm."""
    remaining = set(map(tuple, np.argwhere(mask)))
    count = 0
    while remaining:
        count += 1
        pending = [remaining.pop()]
        while pending:
            y, x = pending.pop()
            for dy in (-1, 0, 1):
                for dx in (-1, 0, 1):
                    candidate = y + dy, x + dx
                    if candidate in remaining:
                        remaining.remove(candidate)
                        pending.append(candidate)
    return count


class SilhouetteTest(unittest.TestCase):
    def test_human_contours_use_flowing_anatomical_curves(self):
        for kind in ("standing", "seated"):
            with self.subTest(kind=kind):
                segments = V._ARCHITECTURE[kind]["segments"]
                self.assertGreaterEqual(sum(len(s) == 3 for s in segments) / len(segments),
                                        CASES["people"]["minimum_cubic_fraction"])

    def test_standing_entourage_has_a_natural_asymmetric_pose(self):
        mask = raster(V.person_glyph([0, 0, 0], color="#FFFFFF", scale=4))
        asymmetry = np.logical_xor(mask, mask[:, ::-1]).sum() / mask.sum()
        self.assertGreater(asymmetry, CASES["people"]["minimum_standing_asymmetry"])

    def test_people_are_solid_connected_and_preserve_architectural_height(self):
        from manim import Rectangle
        from scipy.ndimage import binary_fill_holes, label

        for scale in CASES["people"]["scales"]:
            for kind in ("standing", "seated"):
                with self.subTest(kind=kind, scale=scale):
                    figure = (V.person_glyph([0, 0, 0], scale=scale) if kind == "standing" else
                              V.seated_person_glyph([0, 0, 0], scale=scale)["figure"])
                    expected = V._ARCHITECTURE_DIMENSIONS[kind + "_height"] * scale
                    self.assertAlmostEqual(figure.height, Rectangle(height=expected).height)
                    mask = raster(figure)
                    independent = label(mask, structure=np.ones((3, 3)))[1]
                    self.assertEqual(component_count(mask), independent)
                    self.assertEqual(independent, CASES["people"]["connected_components"])
                    self.assertLessEqual(int(binary_fill_holes(mask).sum() - mask.sum()),
                                         CASES["people"]["maximum_hole_pixels"])

    def test_equipment_is_closed_opaque_silhouette_art(self):
        from manim import Rectangle
        from scipy.ndimage import label

        for kind in CASES["devices"]:
            with self.subTest(kind=kind):
                figure = V.object_silhouette(kind, color="#FFFFFF", width=0.65)
                self.assertAlmostEqual(figure.width, Rectangle(width=0.65).width)
                parts = figure.family_members_with_points()
                self.assertEqual(len(parts), 1)
                self.assertTrue(all(p.is_closed() and p.get_fill_opacity() == 1 and p.get_stroke_width() == 0 for p in parts))
                self.assertEqual(label(raster(figure), structure=np.ones((3, 3)))[1], 1)

    def test_sun_and_moon_are_single_filled_profiles_without_icon_rays_or_erasure(self):
        for kind in CASES["celestial"]:
            with self.subTest(kind=kind):
                figure = getattr(V, kind + "_glyph")([0, 0, 0], color="#FFFFFF")
                parts = figure.family_members_with_points()
                self.assertEqual(len(parts), 1)
                self.assertTrue(all(p.is_closed() and p.get_fill_opacity() == 1 and p.get_stroke_width() == 0 for p in parts))

    def test_activity_art_has_brush_contours_without_speed_lines(self):
        scene = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🐍️")
        for kind in CASES["activities"]:
            with self.subTest(kind=kind):
                profile = scene._ACTIVITY["poses"][kind]
                asset = ET.parse(scene._ACTIVITY_ROOT / profile["asset"]).getroot()
                path = next(item.attrib["d"] for item in asset.iter() if item.tag.endswith("path"))
                curves, lines = path.count("C"), path.count("L")
                self.assertGreaterEqual(curves / (curves + lines), CASES["activity_minimum_cubic_fraction"])
                if kind == "sprint":
                    self.assertEqual(len(scene._activity_figure(kind)[0][1]), 0)


    def test_annotation_frames_and_leaders_replace_thought_bubbles(self):
        from manim import Rectangle

        content = Rectangle(width=1.2, height=0.4)
        frame = V.cloud_glyph(content)
        trail = V.cloud_trail(frame, [0, -2, 0])
        self.assertEqual(len(frame.get_vertices()), 4)
        self.assertTrue(all(type(p).__name__ != "Circle" for p in trail.family_members_with_points()))
        physics = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
        self.assertEqual(type(physics._cloud(content)), type(frame))

    def test_natural_and_industrial_profiles_are_connected_filled_contours(self):
        from scipy.ndimage import label

        for kind in ("tree", "factory", "flame", "monitor", "microbe"):
            with self.subTest(kind=kind):
                figure = V.object_silhouette(kind, color="#FFFFFF")
                self.assertTrue(figure.is_closed())
                self.assertEqual(figure.get_fill_opacity(), 1)
                self.assertEqual(figure.get_stroke_width(), 0)
                self.assertEqual(label(raster(figure), structure=np.ones((3, 3)))[1], 1)


    def test_silhouette_backdrops_can_fade_without_changing_animation_families(self):
        from manim import FadeIn, Scene, tempconfig

        with tempconfig({"skip_animations": True, "write_to_movie": False, "save_last_frame": False,
                         "pixel_width": 320, "pixel_height": 180, "frame_rate": 6}):
            scene = Scene(skip_animations=True)
            room = V.room_section([0, 0, 0])
            lamp = V.lamp_glyph([0, 1, 0])
            scene.play(FadeIn(room["group"]), FadeIn(lamp["group"]), run_time=CASES["backdrop_fade"]["duration"])


if __name__ == "__main__":
    unittest.main()
