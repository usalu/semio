"""🔣 Diagram icons, annotation frames and watt anchors against independent raster oracles."""

import importlib
import json
import os
from pathlib import Path
import unittest
from unittest import mock

import numpy as np

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️icons.json").read_text(encoding="utf-8"))


def raster(body):
    """📷 Render the production geometry on an independent Manim camera."""
    from manim import Camera

    span = max(body.width, body.height) * 1.3
    camera = Camera(pixel_width=512, pixel_height=512, frame_width=span, frame_height=span,
                    frame_center=body.get_center(), background_color="#000000")
    camera.capture_mobjects([body.copy().set_color("#FFFFFF")])
    return np.asarray(camera.get_image())[:, :, 0] > 100


class IconTest(unittest.TestCase):
    def test_diagram_icons_are_connected_filled_contours(self):
        from scipy.ndimage import label

        for kind in CASES["icons"]:
            with self.subTest(kind=kind):
                figure = V.icon_glyph(kind, color="#FFFFFF")
                self.assertTrue(figure.is_closed())
                self.assertEqual(figure.get_fill_opacity(), 1)
                self.assertEqual(figure.get_stroke_width(), 0)
                self.assertEqual(label(raster(figure), structure=np.ones((3, 3)))[1], 1)

    def test_icon_width_follows_the_request(self):
        from manim import Rectangle

        for kind in CASES["icons"]:
            with self.subTest(kind=kind):
                self.assertAlmostEqual(V.icon_glyph(kind, width=0.65).width, Rectangle(width=0.65).width)

    def test_sun_and_moon_are_single_filled_profiles(self):
        for kind in CASES["celestial"]:
            with self.subTest(kind=kind):
                parts = getattr(V, kind + "_glyph")([0, 0, 0], color="#FFFFFF").family_members_with_points()
                self.assertEqual(len(parts), 1)
                self.assertTrue(all(p.is_closed() and p.get_fill_opacity() == 1 and p.get_stroke_width() == 0 for p in parts))

    def test_annotation_frames_and_leaders_replace_thought_bubbles(self):
        from manim import Rectangle

        content = Rectangle(width=1.2, height=0.4)
        frame = V.cloud_glyph(content)
        trail = V.cloud_trail(frame, [0, -2, 0])
        self.assertEqual(len(frame.get_vertices()), 4)
        self.assertTrue(all(type(p).__name__ != "Circle" for p in trail.family_members_with_points()))
        physics = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
        self.assertEqual(type(physics._cloud(content)), type(frame))

    def test_watt_anchor_draws_handcrafted_staffage_and_icons(self):
        for language in CASES["watt_languages"]:
            for kind in (*V._WATT_CASTS, *V._WATT_ICONS):
                with self.subTest(kind=kind, language=language), mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}):
                    self.assertGreater(len(V.watt_anchor(100, compare=kind).family_members_with_points()), 6)

    def test_watt_people_and_irradiance_have_their_own_badges(self):
        for kind in ("person", "seated"):
            self.assertIn(kind, V._WATT_CASTS)
            self.assertNotEqual(V._WATT_CASTS[kind], V._WATT_CASTS["laptop"])
        self.assertIn("sun", V._WATT_ICONS)
        self.assertNotIn("sun", V._WATT_CASTS)

    def test_pendant_lamp_hangs_from_top_and_switches_on_by_crossfade(self):
        from unittest.mock import patch

        from manim import Rectangle, VGroup

        height_m = 0.5

        def fake(cast, *, unit, **_):
            return VGroup(Rectangle(width=0.2 * unit, height=height_m * unit)).set_color("#FFFFFF" if cast.endswith("on") else "#888888")

        with patch.object(V, "staffage", fake), patch.object(V, "staffage_height", lambda cast: height_m):
            lamp = V.pendant_lamp([1.0, 2.0, 0.0], 0.8, on_cast="x-on", off_cast="x-off")
            self.assertAlmostEqual(lamp["group"].get_top()[1], 2.0)
            self.assertAlmostEqual(lamp["off"].height, 0.8)
            self.assertAlmostEqual(lamp["on"].height, 0.8)
            self.assertEqual(lamp["on"].get_fill_opacity(), 0)
            self.assertGreater(lamp["off"].get_stroke_opacity(), 0)
            self.assertAlmostEqual(lamp["on"].get_center()[0], lamp["off"].get_center()[0])
            self.assertTrue(lamp["off"].get_bottom()[1] < lamp["bulb"].get_center()[1] < lamp["group"].get_top()[1])
            lamp["on"].set_opacity(1)
            lamp["off"].set_opacity(0)
            self.assertEqual(lamp["on"].get_stroke_opacity(), 1)
            self.assertEqual(lamp["off"].get_stroke_opacity(), 0)

    def test_backdrops_can_fade_without_changing_animation_families(self):
        from manim import FadeIn, Scene, tempconfig

        with tempconfig({"skip_animations": True, "write_to_movie": False, "save_last_frame": False,
                         "pixel_width": 320, "pixel_height": 180, "frame_rate": 6}):
            scene = Scene(skip_animations=True)
            room = V.room_section([0, 0, 0])
            icon = V.icon_glyph("flame")
            scene.play(FadeIn(room["group"]), FadeIn(icon), run_time=CASES["backdrop_fade"]["duration"])


if __name__ == "__main__":
    unittest.main()
