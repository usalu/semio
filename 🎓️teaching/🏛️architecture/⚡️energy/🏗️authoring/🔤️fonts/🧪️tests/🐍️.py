"""🧪 Verify symbol-only fallback with native Cairo, actual Pango SVGs, and font cmaps."""
from __future__ import annotations

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import importlib.util
import json
import os
import tempfile
import unittest
from unittest.mock import patch
import xml.etree.ElementTree as ET

ROOT = _WORKSPACE_ROOT / "🎓️teaching"
CASES = json.loads((Path(__file__).with_name("🔣️.cases.json")).read_text(encoding="utf-8"))


class GlyphFallbackTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        import manimpango
        from manim import tempconfig
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        apply_body_font = _manim_module.apply_body_font

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        cls.directory = tempfile.TemporaryDirectory(dir=output)
        cls.settings = tempconfig({"media_dir": cls.directory.name, "text_dir": str(Path(cls.directory.name) / "t")})
        cls.settings.__enter__()
        cls.available = set(manimpango.list_fonts())
        apply_body_font()

    @classmethod
    def tearDownClass(cls):
        cls.settings.__exit__(None, None, None)
        cls.directory.cleanup()

    def reference(self, text, font):
        from manim import Text, WHITE
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        BODY_LINE_SPACING = _manim_module.BODY_LINE_SPACING

        result = Text.__new__(Text)
        Text._original__init__(result, text, font=font, font_size=60, color=WHITE, line_spacing=BODY_LINE_SPACING, disable_ligatures=True)
        result.scale(24 / 60)
        return result

    def native_coverage(self, font, character):
        import cairo

        scaled = cairo.ScaledFont(cairo.ToyFontFace(font), cairo.Matrix(xx=60, yy=60), cairo.Matrix(), cairo.FontOptions())
        glyphs = scaled.text_to_glyphs(0, 0, character, False)
        return bool(glyphs) and all(glyph[0] != 0 for glyph in glyphs)

    def test_known_georgia_gaps_use_existing_symbol_font(self):
        from manim import Text, WHITE

        if CASES["body_font"] not in self.available:
            self.skipTest("Georgia is not installed on this host")
        for character in CASES["missing_in_georgia"]:
            with self.subTest(character=character):
                actual = Text(character, font=CASES["body_font"], font_size=24, color=WHITE)
                self.assertIn(character, actual.t2f)
                self.assertNotEqual(actual.t2f[character], CASES["body_font"])
                self.assertIn(actual.t2f[character], self.available)
                self.assertTrue(self.native_coverage(actual.t2f[character], character))

    def test_every_declared_symbol_matches_a_renderable_native_glyph(self):
        import numpy as np
        from manim import WHITE
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        BODY_FONT = _manim_module.BODY_FONT
        body_text = _manim_module.body_text

        for character in CASES["required_symbols"]:
            with self.subTest(character=character):
                actual = body_text(character, font_size=24, color=WHITE)
                font = actual.t2f.get(character, BODY_FONT)
                self.assertTrue(self.native_coverage(font, character))
                expected = self.reference(character, font)
                np.testing.assert_allclose(actual.get_all_points(), expected.get_all_points(), atol=1e-6)
                self.assertEqual(ET.parse(actual.file_name).getroot().tag, "{http://www.w3.org/2000/svg}svg")

    def test_supported_german_and_math_keep_svg_cache_and_geometry(self):
        import numpy as np
        from manim import WHITE
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        BODY_FONT = _manim_module.BODY_FONT
        body_text = _manim_module.body_text

        for text in CASES["plain_samples"]:
            with self.subTest(text=text):
                actual = body_text(text, font_size=24, color=WHITE)
                expected = self.reference(text, BODY_FONT)
                self.assertEqual(actual.t2f, {})
                self.assertEqual(Path(actual.file_name).name, Path(expected.file_name).name)
                np.testing.assert_allclose(actual.get_all_points(), expected.get_all_points(), atol=1e-6)

    def test_affected_strings_change_svg_cache_identity(self):
        from manim import Text, WHITE

        if CASES["body_font"] not in self.available:
            self.skipTest("Georgia is not installed on this host")
        for text in CASES["affected_samples"]:
            with self.subTest(text=text):
                actual = Text(text=text, font=CASES["body_font"], font_size=24, color=WHITE)
                previous = self.reference(text, CASES["body_font"])
                self.assertNotEqual(Path(actual.file_name).name, Path(previous.file_name).name)
                self.assertTrue(actual.t2f)

    def test_explicit_font_ranges_remain_authoritative(self):
        from manim import Text, WHITE

        if not {"Georgia", "DejaVu Serif"}.issubset(self.available):
            self.skipTest("Explicit range test fonts are unavailable")
        for keyword in ("t2f", "text2font"):
            with self.subTest(keyword=keyword):
                actual = Text("ϑ →", font="Georgia", font_size=24, color=WHITE, **{keyword: {"[0:1]": "DejaVu Serif"}})
                self.assertEqual(actual.t2f["[0:1]"], "DejaVu Serif")
                self.assertGreater(len(actual.t2f), 1)

    def test_math_wrapper_uses_the_same_fallback(self):
        from manim import Text
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        math_text = _manim_module.math_text

        if "Georgia" not in self.available:
            self.skipTest("Georgia-specific math baseline is unavailable")
        rendered = math_text(r"\vartheta_{\mathrm{i}}=20\,\mathrm{°C}\to\vartheta_e", font_size=24)
        glyphs = [item for item in rendered.get_family() if isinstance(item, Text) and any(char in CASES["missing_in_georgia"] for char in item.original_text)]
        self.assertTrue(glyphs)
        for glyph in glyphs:
            for char in set(glyph.original_text) & set(CASES["missing_in_georgia"]):
                self.assertIn(char, glyph.t2f)

    def test_text_canvas_preserves_vectors_camera_and_cache_profiles(self):
        from hashlib import sha256
        import numpy as np
        from manim import Scene, WHITE, config, tempconfig

        fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        case = CASES["text_canvas"]
        directory = Path(self.directory.name) / "canvas"
        for number, sample in enumerate(case["samples"]):
            baseline = None
            for width, height in case["unchanged_profiles"]:
                media = directory / str(number) / f"{width}x{height}"
                with tempconfig({"pixel_width": width, "pixel_height": height, "text_dir": str(media)}):
                    actual = fonts.body_text(sample["text"], font_size=sample["font_size"], color=WHITE)
                self.assertRegex(Path(actual.file_name).stem, r"^[0-9a-f]{16}$")
                if baseline is None:
                    baseline = actual
                else:
                    self.assertEqual(Path(actual.file_name).name, Path(baseline.file_name).name)
                    np.testing.assert_array_equal(actual.get_all_points(), baseline.get_all_points())
            media = directory / str(number) / "low"
            media.mkdir(parents=True, exist_ok=True)
            stale = media / Path(baseline.file_name).name
            stale.write_text('<svg xmlns="http://www.w3.org/2000/svg"><path d="M0 0 L1 0 L1 1 Z"/></svg>', encoding="utf-8")
            with tempconfig({"pixel_width": case["output"][0], "pixel_height": case["output"][1], "text_dir": str(media)}):
                scene = Scene()
                before = (config.pixel_width, config.pixel_height, config.frame_width, config.frame_height)
                actual = fonts.body_text(sample["text"], font_size=sample["font_size"], color=WHITE)
                repeated = fonts.body_text(sample["text"], font_size=sample["font_size"], color=WHITE)
                self.assertEqual((config.pixel_width, config.pixel_height, config.frame_width, config.frame_height), before)
                self.assertEqual((scene.camera.pixel_width, scene.camera.pixel_height), tuple(case["output"]))
            self.assertEqual(sum(not char.isspace() for char in sample["text"]), sample["glyphs"])
            self.assertEqual(len(actual.chars), len(sample["text"]))
            expected_key = sha256((Path(baseline.file_name).stem + "|" + case["cache_domain"] + f'{case["minimum_width"]}x{case["minimum_height"]}').encode()).hexdigest()[:case["cache_hex_length"]]
            self.assertEqual(Path(actual.file_name).stem, expected_key)
            self.assertNotEqual(Path(actual.file_name), stale)
            self.assertEqual(actual.file_name, repeated.file_name)
            self.assertEqual((int(ET.parse(actual.file_name).getroot().get("width")), int(ET.parse(actual.file_name).getroot().get("height"))), (case["minimum_width"], case["minimum_height"]))
            np.testing.assert_array_equal(actual.get_all_points(), baseline.get_all_points())

    def test_text_canvas_restores_output_config_on_framework_error(self):
        from manim import Text, WHITE, config, tempconfig

        fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        case = CASES["text_canvas"]
        with tempconfig({"pixel_width": case["output"][0], "pixel_height": case["output"][1], "text_dir": str(Path(self.directory.name) / "canvas-error")}):
            before = (config.pixel_width, config.pixel_height, config.frame_width, config.frame_height)
            with patch.object(Text, "_gen_chars", side_effect=ValueError("canvas test")), self.assertRaisesRegex(ValueError, "canvas test"):
                fonts.body_text(case["samples"][0]["text"], font_size=22, color=WHITE)
            self.assertEqual((config.pixel_width, config.pixel_height, config.frame_width, config.frame_height), before)

    def test_framework_coverage_matches_independent_fonttools_cmap(self):
        manim_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        test_path = os.environ.get("SEMIO_FONTTOOLS_TEST_PATH")
        if importlib.util.find_spec("fontTools") is None and test_path:
            sys.path.append(test_path)
        if importlib.util.find_spec("fontTools") is None or os.name != "nt":
            self.skipTest("Native Windows FontTools comparison is optional")
        from fontTools.ttLib import TTFont

        with TTFont(Path(os.environ.get("WINDIR", "C:/Windows")) / "Fonts/georgia.ttf") as font:
            cmap = font.getBestCmap()
            for char in CASES["required_symbols"]:
                with self.subTest(character=char):
                    self.assertEqual(manim_fonts._font_supports("Georgia", char), ord(char) in cmap)


if __name__ == "__main__":
    unittest.main()
