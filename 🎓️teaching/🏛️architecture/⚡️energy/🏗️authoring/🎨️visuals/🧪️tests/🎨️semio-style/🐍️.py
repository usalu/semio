"""🪟 Verify semio clip chrome against portable cases and native path unions."""

from __future__ import annotations

import importlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"


def polygon_area(vertices):
    """📐 Measure a closed rectilinear outline by its directed edge products."""
    return abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(vertices, vertices[1:] + vertices[:1]))) / 2


class ClipStyleTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        from manim import tempconfig

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        cls.directory = tempfile.TemporaryDirectory(dir=output)
        cls.settings = tempconfig({"media_dir": cls.directory.name, "text_dir": str(Path(cls.directory.name) / "t")})
        cls.settings.__enter__()
        cls.visuals = importlib.import_module(MODULE + ".🎨️visuals.🐍️")
        cls.fonts = importlib.import_module(MODULE + ".🔤️fonts.🐍️")
        cls.theme = importlib.import_module(MODULE + ".🎨️theme.🐍️")
        cls.fonts.apply_body_font()

    @classmethod
    def tearDownClass(cls):
        cls.settings.__exit__(None, None, None)
        cls.directory.cleanup()

    def test_portable_frame_outline_and_chip_regions(self):
        for case in CASES["frames"]:
            with self.subTest(case=case["id"]):
                size = (case["input"]["width"], case["input"]["height"])
                actual = self.visuals.window_frame_outline(*size, inset=case["input"].get("inset", 1))
                self.assertEqual([list(point) for point in actual], case["outline"])
                chips = {key: value for key, value in case["input"].items() if key not in ("width", "height", "inset")}
                self.assertEqual(self.visuals.window_frame_chip_regions(*size, **chips), case["chips"])

    def test_frame_matches_independent_skia_union_with_its_chips(self):
        import pathops

        def rectangle(left, top, right, bottom):
            path = pathops.Path()
            path.moveTo(left, top)
            path.lineTo(right, top)
            path.lineTo(right, bottom)
            path.lineTo(left, bottom)
            path.close()
            return path

        for case in CASES["frames"]:
            with self.subTest(case=case["id"]):
                outline = self.visuals.window_frame_outline(case["input"]["width"], case["input"]["height"], inset=case["input"].get("inset", 1))
                frame = rectangle(outline[0][0], outline[0][1], outline[2][0], outline[2][1])
                union = frame
                for chip in case["chips"]:
                    union = pathops.op(union, rectangle(chip["x"], chip["y"], chip["x"] + chip["width"], chip["y"] + chip["height"]), pathops.PathOp.UNION)
                self.assertAlmostEqual(polygon_area(outline), abs(frame.area))
                self.assertAlmostEqual(abs(pathops.op(union, frame, pathops.PathOp.XOR).area), 0)

    def test_fonts_palette_and_spacing_follow_canonical_semio_tokens(self):
        tokens = json.loads((ROOT / "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json").read_text(encoding="utf-8"))
        self.assertEqual(self.fonts.BODY_FONT, tokens["fontStacks"]["sans"].split(",")[0])
        self.assertEqual(self.theme.FONT_SANS, "Anta")
        self.assertEqual(self.theme.FONT_SERIF, "Kelly Slab")
        self.assertEqual(self.theme.FONT_MONO, "Share Tech Mono")
        self.assertEqual(self.theme.UI_SPACING, float(tokens["spacing"]["compact"].removesuffix("rem")) * 0.5)
        self.assertEqual(self.theme.CHROME_BORDER, tokens["strokes"]["chromeBorderDefault"])
        self.assertEqual(self.theme.CHROME_FILL.lower(), tokens["colors"]["dark-8-9"])
        for name, token in CASES["type_scale"].items():
            with self.subTest(type_role=name):
                self.assertEqual(getattr(self.fonts, name), tokens["metrics"]["typography"][token] * CASES["type_multiplier"])
        for name, token in CASES["palette"].items():
            with self.subTest(role=name):
                self.assertEqual(getattr(self.visuals, name).lower(), tokens["colors"][token])

    def accent_input(self, value):
        """🧱 Materialize portable primitive arguments at the Manim test boundary."""
        from manim import Rectangle

        if isinstance(value, dict):
            if value.get("shape") == "rectangle":
                return Rectangle(width=value["width"], height=value["height"])
            return {key: self.accent_input(part) for key, part in value.items()}
        if isinstance(value, list):
            return [self.accent_input(part) for part in value]
        return value

    def assert_accent(self, result, case, expected):
        """🎯 Compare native object RGB against an independent Pillow color parser."""
        from PIL import ImageColor

        for key in case["path"]:
            result = result[key]
        color = getattr(result, "get_" + case["channel"] + "_color")()
        actual = tuple(round(float(part) * 255) for part in color.to_rgb())
        self.assertEqual(actual, ImageColor.getrgb(expected))

    def test_portable_primary_accents_and_explicit_secondary_overrides(self):
        accents = CASES["accents"]
        for case in accents["defaults"]:
            helper = getattr(self.visuals, case["helper"])
            for role, kwargs in (("primary", {}), ("secondary", {"color": accents["secondary"]["hex"]})):
                with self.subTest(helper=case["helper"], path=case["path"], role=role):
                    result = helper(*self.accent_input(case["args"]), **kwargs)
                    self.assert_accent(result, case, accents[role]["hex"])

    def test_instructional_heading_chrome_uses_primary(self):
        for case in CASES["accents"]["typography"]:
            with self.subTest(helper=case["helper"], role=case["role"]):
                result = getattr(self.fonts, case["helper"])(*self.accent_input(case["args"]))
                self.assert_accent(result, case, self.theme.color(case["role"]))

    def test_compact_instructional_heading_aligns_to_left_window_edge(self):
        """📑 Verify a real standalone instructional heading at the first content row."""
        from manim import config

        case = CASES["tab_geometry"]
        section = self.fonts.section_heading("Heat · Wärme")
        self.assertAlmostEqual(section.get_left()[0], -config.frame_width / 2 + case["sectionLeftSpacing"] * self.theme.UI_SPACING)
        self.assertAlmostEqual(section.get_top()[1], config.frame_height / 2 - case["sectionTopSpacing"] * self.theme.UI_SPACING)
        self.assertLess(section.width, config.frame_width / 2)
        self.assertAlmostEqual(section[1].font_size, self.fonts.LABEL_FONT_SIZE)

    def test_render_cache_fingerprints_the_complete_semio_style_closure(self):
        from hashlib import sha256

        runtime = importlib.import_module(MODULE + ".🎬️clip.⚙️runtime.🐍️")
        segment = runtime.load_manifest()[0]["chapters"][0]["segments"][0]
        state = runtime.segment_state(segment, {"language": "en", "profile": "preview", "resolution": None, "fps": None})
        visuals = Path(self.visuals.__file__).parent
        for path in (*self.theme.source_files(), visuals / "🧍️staffage/🐍️.py", visuals / "🧍️staffage/🔣️.json", visuals / "🔣️icons/🔣️.json"):
            with self.subTest(source=str(path.relative_to(ROOT))):
                self.assertEqual(state["sources"][path.resolve().as_posix()], sha256(path.read_bytes()).hexdigest())

    def test_render_key_names_the_cast_keys_of_every_cast_a_clip_draws_and_follows_the_exporter_and_catalog(self):
        from unittest import mock

        runtime = importlib.import_module(MODULE + ".🎬️clip.⚙️runtime.🐍️")
        staffage = importlib.import_module(MODULE + ".🎨️visuals.🧍️staffage.🐍️")
        settings = {"language": "en", "profile": "preview", "resolution": None, "fps": None}
        segments = [segment for video in runtime.load_manifest() for chapter in video["chapters"] for segment in chapter["segments"]]
        drawing = {}
        for segment in segments:
            state = runtime.segment_state(segment, settings)
            if state["casts"]:
                drawing[segment["scene"]] = (segment, state)
        self.assertGreaterEqual(len(drawing), 20)
        segment, state = next(iter(drawing.values()))
        self.assertTrue(all(key == staffage.cast_key(staffage._entry(cast)) for cast, key in state["casts"].items()))
        with mock.patch.object(staffage, "_source_digest", mock.Mock(return_value="exporter-changed")):
            changed = runtime.segment_state(segment, settings)
        self.assertEqual(sorted(changed["casts"]), sorted(state["casts"]))
        self.assertTrue(all(changed["casts"][cast] != key for cast, key in state["casts"].items()))
        self.assertNotEqual(changed, state)
        self.assertFalse(runtime.reusable_clip(Path(tempfile.gettempdir()) / "absent.mp4", state))

    def test_window_frame_is_one_rectangle_with_a_glass_chip_inside_it(self):
        import numpy as np
        from manim import Polygon

        case = CASES["chrome"]
        frame = self.visuals.window_frame(**case["frame"])
        self.assertIsInstance(frame, Polygon)
        np.testing.assert_allclose(frame.get_vertices()[:, :2], case["frame_vertices"], atol=1e-8)
        chip = frame.submobjects[0]
        np.testing.assert_allclose(chip.get_vertices()[:, :2], case["chip_vertices"], atol=1e-8)
        self.assertEqual(chip.get_stroke_width(), 0)
        self.assertLess(chip.get_fill_opacity(), 1)
        self.assertNotEqual(chip.get_fill_color().to_hex().lower(), self.theme.CHROME_FILL.lower())
        np.testing.assert_allclose([frame.get_left()[0], frame.get_right()[0], frame.get_top()[1], frame.get_bottom()[1]], [-2, 2, 1, -1], atol=1e-8)
        self.assertEqual(frame.get_stroke_width(), self.theme.CHROME_BORDER)
        self.assertEqual(frame.get_fill_color().to_hex().lower(), self.theme.CHROME_FILL.lower())

    def test_chip_and_stat_card_keep_readable_content_inside_square_chrome(self):
        from manim import Rectangle

        case = CASES["chrome"]
        content = Rectangle(width=1, height=0.4)
        frame, actual = self.visuals.window_chip(content)
        self.assertIs(actual, content)
        self.assertEqual(len(frame.get_vertices()), 4)
        self.assertGreater(frame.get_fill_opacity(), 0)
        self.assertGreater(frame.width, actual.width)
        self.assertGreater(frame.height, actual.height)
        frame, label = self.visuals.chip(case["chip_text"])
        self.assertEqual(len(frame.get_vertices()), 4)
        self.assertGreater(frame.width, label.width)
        self.assertGreater(frame.height, label.height)
        frame, body = self.visuals.stat_card(**case["card"])
        self.assertEqual(len(frame.get_vertices()), 4)
        self.assertGreaterEqual(len(frame.submobjects), 1)
        self.assertLess(body[1].get_top()[1], body[0].get_bottom()[1])
        self.assertGreaterEqual(body.get_left()[0], frame.get_left()[0])
        self.assertLessEqual(body.get_right()[0], frame.get_right()[0])


if __name__ == "__main__":
    unittest.main()
