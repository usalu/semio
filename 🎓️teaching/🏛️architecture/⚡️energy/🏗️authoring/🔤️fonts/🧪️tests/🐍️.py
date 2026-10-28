"""🧪 Verify bundled glyph fallback with actual Pango SVGs and independent font cmaps."""
from __future__ import annotations

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import json
import os
import struct
import tempfile
import unittest
from unittest.mock import patch
import xml.etree.ElementTree as ET

ROOT = _WORKSPACE_ROOT / "🎓️teaching"
CASES = json.loads((Path(__file__).with_name("🔣️.cases.json")).read_text(encoding="utf-8"))


def cmap_fixture(case):
    """🧱 Encode language-neutral cmap vectors as a minimal font table fixture."""
    if case["format"] == 12:
        groups = case["groups"]
        subtable = struct.pack(">HHIII", 12, 0, 16 + len(groups) * 12, 0, len(groups))
        subtable += b"".join(struct.pack(">III", *group) for group in groups)
        encoding = 10
    else:
        segments = [*case["segments"], {"codepoint": 65535, "delta": 1}]
        count, glyphs, distances = len(segments), [], []
        for index, segment in enumerate(segments):
            distances.append(2 * (count - index + len(glyphs)) if "glyph" in segment else 0)
            if "glyph" in segment:
                glyphs.append(segment["glyph"])
        values = [*(segment["codepoint"] for segment in segments), 0,
                  *(segment["codepoint"] for segment in segments),
                  *(segment["delta"] & 65535 for segment in segments), *distances, *glyphs]
        power = 1 << (count.bit_length() - 1)
        subtable = struct.pack(">HHHHHHH", 4, 14 + len(values) * 2, 0, count * 2, power * 2, count.bit_length() - 1, count * 2 - power * 2)
        subtable += struct.pack(">" + "H" * len(values), *values)
        encoding = 1
    return struct.pack(">HHHHI", 0, 1, 3, encoding, 12) + subtable


def glyph_outline(font, character, path, *, glyph_name=None):
    """✒️ Draw an exact checked-in glyph independently of the native text renderer."""
    from fontTools.pens.svgPathPen import SVGPathPen
    from manim import SVGMobject

    glyphs = font.getGlyphSet()
    pen = SVGPathPen(glyphs)
    glyphs[glyph_name or font.getBestCmap()[ord(character)]].draw(pen)
    path.write_text('<svg xmlns="http://www.w3.org/2000/svg"><g transform="scale(1,-1)"><path d="' + pen.getCommands() + '"/></g></svg>', encoding="utf-8")
    return SVGMobject(path, stroke_width=0, fill_opacity=1)


def outline_points(mobject):
    """🔍 Sample normalized cubic contours independently of winding and glyph ordering."""
    import numpy as np

    shape = mobject.copy().move_to([0, 0, 0])
    shape.scale(2 / shape.height)
    result = []
    time = np.linspace(0, 1, 257)
    weights = np.column_stack(((1 - time) ** 3, 3 * (1 - time) ** 2 * time, 3 * (1 - time) * time ** 2, time ** 3))
    for member in shape.get_family():
        for curve in member.get_cubic_bezier_tuples() if hasattr(member, "get_cubic_bezier_tuples") else ():
            result.append(weights @ curve[:, :2])
    return np.concatenate(result)


class GlyphFallbackTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        from manim import tempconfig
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        apply_body_font = _manim_module.apply_body_font

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        cls.directory = tempfile.TemporaryDirectory(dir=output)
        cls.settings = tempconfig({"media_dir": cls.directory.name, "text_dir": str(Path(cls.directory.name) / "t")})
        cls.settings.__enter__()
        apply_body_font()

    @classmethod
    def tearDownClass(cls):
        cls.settings.__exit__(None, None, None)
        cls.directory.cleanup()

    def reference(self, text, font, **kwargs):
        from manim import Text, WHITE
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        BODY_LINE_SPACING = _manim_module.BODY_LINE_SPACING

        result = Text.__new__(Text)
        Text._original__init__(result, text, font=font, font_size=60, color=WHITE, line_spacing=BODY_LINE_SPACING, disable_ligatures=True, warn_missing_font=False, **kwargs)
        result.scale(24 / 60)
        return result

    def source_coverage(self, font, character):
        from fontTools.ttLib import TTFont

        theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
        for path in theme.font_sources().get(font, ()):
            with TTFont(path) as source:
                if ord(character) in source.getBestCmap():
                    return True
        return False

    def test_body_font_gaps_use_renderable_symbol_faces(self):
        from manim import Text, WHITE

        actual = Text(CASES["missing_in_body"], font=CASES["body_font"], font_size=24, color=WHITE)
        for character in CASES["missing_in_body"]:
            with self.subTest(character=character):
                self.assertIn(character, actual.t2f)
                self.assertNotEqual(actual.t2f[character], CASES["body_font"])
                self.assertTrue(self.source_coverage(actual.t2f[character], character))

    def test_every_declared_symbol_matches_a_renderable_native_glyph(self):
        import numpy as np
        from manim import WHITE
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        BODY_FONT = _manim_module.BODY_FONT
        body_text = _manim_module.body_text

        text = " ".join(CASES["required_symbols"])
        actual = body_text(text, font_size=24, color=WHITE)
        expected = self.reference(text, BODY_FONT, t2f=actual.t2f)
        np.testing.assert_allclose(actual.get_all_points(), expected.get_all_points(), atol=1e-6)
        self.assertEqual(ET.parse(actual.file_name).getroot().tag, "{http://www.w3.org/2000/svg}svg")
        for character in CASES["required_symbols"]:
            with self.subTest(character=character):
                font = actual.t2f.get(character, BODY_FONT)
                self.assertTrue(self.source_coverage(font, character))

    def test_synthetic_cmaps_match_independent_fonttools(self):
        from fontTools.ttLib import TTFont, newTable

        fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        for case in CASES["cmaps"]:
            cmap = cmap_fixture(case)
            path = Path(self.directory.name) / (case["id"] + ".ttf")
            path.write_bytes(struct.pack(">IHHHH", 65536, 1, 16, 0, 0) + b"cmap" + struct.pack(">III", 0, 28, len(cmap)) + cmap)
            reference = TTFont()
            reference.setGlyphOrder([".notdef", "one", "two", "three", "four", "five"])
            table = newTable("cmap")
            table.decompile(cmap, reference)
            mapping = table.getBestCmap()
            for codepoint, glyph in case["expected"].items():
                with self.subTest(case=case["id"], codepoint=codepoint):
                    character = int(codepoint)
                    expected = reference.getGlyphID(mapping[character]) if character in mapping else 0
                    self.assertEqual(expected, glyph)
                    self.assertEqual(fonts._cmap_supports(path, character), bool(expected))

    def test_native_rendered_glyphs_match_exact_source_outlines(self):
        from fontTools.ttLib import TTFont
        from manim import Text, WHITE
        from scipy.spatial import cKDTree

        for case in CASES["rendered_glyphs"]:
            run = Text(case["text"], font=case["family"], font_size=24, color=WHITE) if case.get("run") else None
            with TTFont(_WORKSPACE_ROOT / case["source"]) as font:
                for index, character in enumerate(case["text"]):
                    if case.get("characters") and character not in case["characters"]:
                        continue
                    with self.subTest(family=case["family"], character=character):
                        actual = run.chars[index] if run is not None else Text(character, font=case["family"], font_size=24, color=WHITE)
                        expected = glyph_outline(font, character, Path(self.directory.name) / f"glyph-{ord(character)}.svg")
                        left, right = outline_points(actual), outline_points(expected)
                        distance = max(cKDTree(left).query(right)[0].max(), cKDTree(right).query(left)[0].max())
                        self.assertLess(distance, CASES["outline_distance"])

    def test_outline_oracle_rejects_source_notdef_glyph(self):
        from fontTools.ttLib import TTFont
        from scipy.spatial import cKDTree

        case = CASES["rendered_glyphs"][0]
        with TTFont(_WORKSPACE_ROOT / case["source"]) as font:
            expected = glyph_outline(font, "ä", Path(self.directory.name) / "source-a-umlaut.svg")
            missing = glyph_outline(font, "ä", Path(self.directory.name) / "source-notdef.svg", glyph_name=".notdef")
            left, right = outline_points(missing), outline_points(expected)
            distance = max(cKDTree(left).query(right)[0].max(), cKDTree(right).query(left)[0].max())
            self.assertGreater(distance, CASES["outline_distance"])

    def test_reset_text_defaults_restore_bundled_face_and_fallback(self):
        from manim import Text, WHITE

        fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        previous = Text.__init__
        try:
            Text.set_default()
            fonts.apply_body_font()
            actual = Text("A ϑ", font_size=24, color=WHITE)
            self.assertEqual(actual.font, CASES["body_font"])
            self.assertIn("ϑ", actual.t2f)
            self.assertTrue(self.source_coverage(actual.t2f["ϑ"], "ϑ"))
        finally:
            Text.__init__ = previous

    def test_supported_german_and_math_keep_svg_cache_and_geometry(self):
        import numpy as np
        from manim import WHITE
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
        BODY_FONT = _manim_module.BODY_FONT
        body_text = _manim_module.body_text

        for text in CASES["plain_samples"]:
            with self.subTest(text=text):
                actual = body_text(text, font_size=24, color=WHITE)
                expected = self.reference(text, BODY_FONT, t2f=actual.t2f)
                self.assertEqual(Path(actual.file_name).name, Path(expected.file_name).name)
                np.testing.assert_allclose(actual.get_all_points(), expected.get_all_points(), atol=1e-6)

    def test_affected_strings_change_svg_cache_identity(self):
        from manim import Text, WHITE

        for text in CASES["affected_samples"]:
            with self.subTest(text=text):
                actual = Text(text=text, font=CASES["body_font"], font_size=24, color=WHITE)
                previous = self.reference(text, CASES["body_font"])
                self.assertNotEqual(Path(actual.file_name).name, Path(previous.file_name).name)
                self.assertTrue(actual.t2f)

    def test_explicit_font_ranges_remain_authoritative(self):
        from manim import Text, WHITE

        for keyword in ("t2f", "text2font"):
            with self.subTest(keyword=keyword):
                actual = Text("A →", font=CASES["body_font"], font_size=24, color=WHITE, **{keyword: {"[0:1]": "Kelly Slab"}})
                self.assertEqual(actual.t2f["[0:1]"], "Kelly Slab")
                self.assertGreater(len(actual.t2f), 1)

    def test_math_wrapper_uses_the_same_fallback(self):
        from manim import Text
        _manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
        math_text = _manim_module.math_text

        rendered = math_text(r"\vartheta_{\mathrm{i}}=20\,\mathrm{°C}\to\vartheta_e", font_size=24)
        glyphs = [item for item in rendered.get_family() if isinstance(item, Text) and any(char in CASES["missing_in_body"] for char in item.original_text)]
        self.assertTrue(glyphs)
        for glyph in glyphs:
            for char in set(glyph.original_text) & set(CASES["missing_in_body"]):
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
        from fontTools.ttLib import TTFont

        theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
        for case in CASES["bundled_fonts"]:
            for subset in case["subsets"]:
                path = theme.ASSET_ROOT / "🔤️fonts" / case["directory"] / subset / "📖️regular/🔤️outline.ttf"
                self.assertIn(path, theme.font_files())
                with TTFont(path) as font:
                    self.assertEqual(font["name"].getDebugName(1), case["family"])
        cmaps = {}
        for path in theme.font_files():
            with TTFont(path) as font:
                family = font["name"].getDebugName(1)
                cmaps.setdefault(family, set()).update(font.getBestCmap())
        for family, cmap in cmaps.items():
            for char in CASES["required_symbols"] + CASES["coverage_samples"]:
                with self.subTest(family=family, character=char):
                    self.assertEqual(manim_fonts._font_supports(family, char), ord(char) in cmap)


if __name__ == "__main__":
    unittest.main()
