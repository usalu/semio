"""📟 Compare bounded live readout templates against independent Manim and Cairo output."""
from __future__ import annotations

import importlib
import json
import os
import sys
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

import numpy as np
from manim import Camera, Dot, ManimColor, Text, always_redraw, tempconfig

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"


class ReadoutTemplateTest(unittest.TestCase):
    """🧬 Preserve live typesetting while owning bounded immutable geometry prototypes."""

    def setUp(self):
        """📦 Configure generated paths and isolate template state for each focused scenario."""
        self.visuals = importlib.import_module(PACKAGE + ".🎨️visuals.🐍️")
        self.fonts = importlib.import_module(PACKAGE + ".🔤️fonts.🐍️")
        self.assertTrue(callable(getattr(self.fonts, "text_context", None)))
        self.assertTrue(callable(getattr(self.visuals, "_math_readout_text", None)))
        self.visuals._math_readout_template.cache_clear()
        self.addCleanup(self.visuals._math_readout_template.cache_clear)
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        self.config = tempconfig({"media_dir": str(output / "media"), "text_dir": str(output / "text"),
                                  "pixel_width": 480, "pixel_height": 270})
        self.config.__enter__()
        self.addCleanup(self.config.__exit__, None, None, None)

    def equal_geometry(self, actual, expected):
        """📐 Compare every point, native style, baseline and edge against an uncached oracle."""
        self.assertIsNot(actual, expected)
        self.assertIsNot(actual.base, expected.base)
        self.assertEqual(len(actual.get_family()), len(expected.get_family()))
        for left, right in zip(actual.get_family(), expected.get_family(), strict=True):
            np.testing.assert_array_equal(left.points, right.points)
            self.assertFalse(np.shares_memory(left.points, right.points))
            for getter in ("get_fill_opacity", "get_stroke_opacity", "get_stroke_width"):
                np.testing.assert_array_equal(getattr(left, getter)(), getattr(right, getter)())
            for getter in ("get_fill_color", "get_stroke_color"):
                left_color, right_color = getattr(left, getter)(), getattr(right, getter)()
                if left_color is None or right_color is None:
                    self.assertIs(left_color, right_color)
                else:
                    np.testing.assert_array_equal(left_color.to_rgba(), right_color.to_rgba())
        np.testing.assert_array_equal(actual.base.get_center(), expected.base.get_center())
        for getter in ("get_left", "get_right", "get_top", "get_bottom"):
            np.testing.assert_array_equal(getattr(actual, getter)(), getattr(expected, getter)())

    def text(self, source="x", *, size=None, color="#FFFFFF"):
        """🏷️ Request a fresh unplaced cached copy through the production live-readout port."""
        return self.visuals._math_readout_text(source, size=size or CASES["fontSize"], color=color)

    def test_uncached_geometry_styles_anchors_and_cairo_pixels_match(self):
        """🖼️ Keep real fraction, index, alpha color and baseline output identical."""
        for source, color in zip(CASES["sources"], CASES["colors"][::2], strict=True):
            for edge in CASES["anchors"]:
                with self.subTest(source=source, color=color, edge=edge):
                    expected = self.visuals.math_label(source, CASES["positions"][1], size=CASES["fontSize"], color=color, edge=edge)
                    actual = self.visuals.place_math(self.text(source, color=color), CASES["positions"][1], edge)
                    self.equal_geometry(actual, expected)
                    cameras = [Camera(pixel_width=480, pixel_height=270) for _ in range(2)]
                    for camera, obj in zip(cameras, (expected, actual), strict=True):
                        camera.capture_mobjects([obj])
                    np.testing.assert_array_equal(np.asarray(cameras[0].get_image()), np.asarray(cameras[1].get_image()))

    def test_template_copies_isolate_geometry_style_updaters_and_baseline(self):
        """🔒 Prevent any returned object's mutations from reaching the stored template."""
        source = CASES["sources"][0]
        first, expected = self.text(source), self.visuals.math_text(source, font_size=CASES["fontSize"], color="#FFFFFF")
        mutation = CASES["mutations"]
        first.base.shift([0, 2, 0])
        first.shift(mutation["shift"]).set_color(mutation["color"]).set_opacity(mutation["opacity"])
        first.set_stroke(width=mutation["strokeWidth"], opacity=mutation["opacity"])
        first.add_updater(lambda obj: obj.shift([1, 0, 0]))
        first.inspection_marker = True
        actual = self.text(source)
        self.equal_geometry(actual, expected)
        self.assertFalse(any(member.updaters for member in actual.get_family()))
        self.assertNotIn("inspection_marker", vars(actual))

    def test_live_updates_reuse_text_and_preserve_identity_and_callable_position(self):
        """🖱️ Move unchanged formulas and update changed values without replacing the live object."""
        source, point = [CASES["sources"][0]], list(CASES["positions"][0])
        oracle_source, oracle_point = [source[0]], list(point)
        expected = always_redraw(lambda: self.visuals.math_label(oracle_source[0], oracle_point, size=CASES["fontSize"], edge="left"))
        with patch.object(self.visuals, "math_text", wraps=self.visuals.math_text) as build:
            readout = self.visuals.math_readout(lambda: source[0], lambda: point, size=CASES["fontSize"], edge="left")
            identity = id(readout)
            readout.update(0.25)
            point[:] = CASES["positions"][1]
            readout.update(0.25)
            self.assertEqual(build.call_count, 1)
            source[0] = CASES["sources"][1]
            readout.update(0.25)
            self.assertEqual(build.call_count, 2)
            self.assertEqual(id(readout), identity)
        expected.update(0.25)
        oracle_point[:] = CASES["positions"][1]
        expected.update(0.25)
        oracle_source[0] = CASES["sources"][1]
        expected.update(0.25)
        self.equal_geometry(readout, expected)

    def test_source_size_color_alpha_and_bounded_lru_eviction(self):
        """♻️ Check cache keys and deterministic eviction with cheap native geometry."""
        prototype = self.visuals._math_box([Dot()], [0, 0, 0])
        with patch.object(self.visuals, "math_text", side_effect=lambda *args, **kwargs: prototype.copy()) as build:
            self.text("x")
            self.text("x", color=ManimColor("#ffffff"))
            self.assertEqual(build.call_count, 1)
            self.text("x", size=CASES["fontSize"] + 1)
            self.text("x", color=CASES["colors"][1])
            self.text("x", color=CASES["colors"][2])
            self.text("y")
            self.assertEqual(build.call_count, 5)
            self.visuals._math_readout_template.cache_clear()
            build.reset_mock()
            for index in range(CASES["capacity"] + 1):
                self.text(str(index))
            self.assertEqual(self.visuals._math_readout_template.cache_info().currsize, CASES["capacity"])
            self.text(str(CASES["capacity"]))
            self.assertEqual(build.call_count, CASES["capacity"] + 1)
            self.text("0")
            self.assertEqual(build.call_count, CASES["capacity"] + 2)

    def test_font_theme_kerning_canvas_frame_renderer_and_text_defaults_change_keys(self):
        """🧭 Bind templates to cheap primitive typography and native rendering context values."""
        first = self.fonts.text_context()
        self.assertEqual(first, self.fonts.text_context())
        hash(first)
        with patch.object(self.visuals, "math_text", side_effect=lambda *args, **kwargs: SimpleNamespace(copy=lambda: object())) as build:
            self.text()
            count = build.call_count
            changes = [("BODY_FONT", self.fonts._theme.FONT_SERIF), ("_KERNING_SAFE_FONT_SIZE", CASES["kerning"][1]),
                       ("_TEXT_CANVAS_MIN_SIZE", tuple(CASES["minimumCanvas"][1]))]
            for name, value in changes:
                with patch.object(self.fonts, name, value):
                    self.text()
                    count += 1
                    self.assertEqual(build.call_count, count)
            with patch.object(self.fonts, "_font_digest", return_value=CASES["namespace"][1]):
                self.text()
                count += 1
                self.assertEqual(build.call_count, count)
            for value in ({"pixel_width": 640, "pixel_height": 360}, {"frame_width": CASES["frameWidth"][1]}, {"renderer": "opengl"}):
                with tempconfig(value):
                    self.text()
                    count += 1
                    self.assertEqual(build.call_count, count)
            with patch.object(Text, "__init__", lambda *args, **kwargs: None):
                self.text()
                count += 1
                self.assertEqual(build.call_count, count)


if __name__ == "__main__":
    unittest.main()
