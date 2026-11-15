"""🧮 Inline TeX: segment parsing, deterministic Unicode text, loud failures; cross-checked against pylatexenc when present."""

from __future__ import annotations

import importlib
import json
import unittest
from pathlib import Path

import jsonschema

NARRATION = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🐍️")
TEX = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧮️tex.🐍️")
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
TEX_DIR = Path(TEX.__file__).parent


class SegmentTest(unittest.TestCase):
    def test_segments(self):
        for case in CASES["segments"]:
            with self.subTest(text=case["text"]):
                self.assertEqual([list(item) for item in TEX.split_segments(case["text"])], case["segments"])

    def test_malformed_dollars_fail(self):
        for text in CASES["segmentErrors"]:
            with self.subTest(text=text), self.assertRaises(TEX.TexError):
                TEX.split_segments(text)

    def test_symbol_table_matches_its_schema(self):
        data = json.loads((TEX_DIR / "🔣️.json").read_text(encoding="utf-8"))
        jsonschema.validate(data, json.loads((TEX_DIR / "🧬️schema.json").read_text(encoding="utf-8")))


class UnicodeTest(unittest.TestCase):
    def test_deterministic_plain_text(self):
        for case in CASES["unicode"]:
            with self.subTest(tex=case["tex"]):
                self.assertEqual(TEX.math_to_unicode(case["tex"]), case["expected"])
                self.assertEqual(TEX.math_to_unicode(case["tex"]), TEX.math_to_unicode(case["tex"]))

    def test_clause_plain_text_never_keeps_raw_tex(self):
        for case in CASES["plain"]:
            with self.subTest(text=case["text"]):
                result = TEX.plain_text(case["text"])
                self.assertEqual(result, case["expected"])
                self.assertNotIn("$", result)
                self.assertNotIn("\\", result)

    def test_unsupported_tex_fails(self):
        for src in CASES["invalid"]:
            with self.subTest(src=src), self.assertRaises(TEX.TexError):
                TEX.parse_tex(src)

    def test_matches_pylatexenc(self):
        try:
            converter = importlib.import_module("pylatexenc.latex2text").LatexNodes2Text(math_mode="text")
        except ImportError:
            self.skipTest("pylatexenc is a test-only dependency of the test group")
        for case in CASES["thirdParty"]:
            with self.subTest(tex=case["tex"]):
                reference = converter.latex_to_text("$" + case["tex"] + "$")
                self.assertEqual(reference.strip("$"), case["expected"])
                self.assertEqual(TEX.math_to_unicode(case["tex"]), case["expected"])


class CaptionTypesetterTest(unittest.TestCase):
    def test_every_case_is_accepted_by_the_caption_typesetter(self):
        sources = [case["tex"] for key in ("unicode", "thirdParty") for case in CASES[key]]
        for src in sources:
            with self.subTest(tex=src):
                self.assertTrue(VISUALS._math_parse_items(src))

    def test_every_symbol_is_typesettable(self):
        for name, entry in TEX.SYMBOLS.items():
            with self.subTest(symbol=name):
                self.assertEqual(VISUALS._MATH_SYMBOLS[name], entry["char"])
                self.assertTrue(VISUALS._math_parse_items("\\" + name))


if __name__ == "__main__":
    unittest.main()
