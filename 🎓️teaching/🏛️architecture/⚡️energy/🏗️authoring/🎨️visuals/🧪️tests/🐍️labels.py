"""🌐 On-screen label language: explicit en or de, never a default; numbers and units follow the language."""

from __future__ import annotations

import importlib
import json
import os
import unittest
from pathlib import Path
from unittest import mock

import numpy as np

VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️labels.json").read_text(encoding="utf-8"))


def selected(language):
    return mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language})


class LabelLanguageTest(unittest.TestCase):
    def test_tr_picks_the_explicit_language(self):
        for case in CASES["tr"]:
            with self.subTest(language=case["language"]), selected(case["language"]):
                self.assertEqual(VISUALS.tr(case["en"], case["de"]), case["expected"])

    def test_no_default_language_is_ever_assumed(self):
        unset = {key: value for key, value in os.environ.items() if key != "SEMIO_TEACHING_LANGUAGE"}
        with mock.patch.dict(os.environ, unset, clear=True), self.assertRaises(ValueError):
            VISUALS.tr("Light", "Licht")
        for language in CASES["invalid_languages"]:
            with self.subTest(language=language), selected(language), self.assertRaises(ValueError):
                VISUALS.language()

    def test_numbers_follow_the_language_and_match_numpy_digits(self):
        for case in CASES["numbers"]:
            for language in ("en", "de"):
                with self.subTest(value=case["value"], language=language), selected(language):
                    text = VISUALS.label_number(case["value"], case["digits"])
                    self.assertEqual(text, case[language])
                    plain = text.replace("\\,", "").replace("{,}", ".")
                    reference = np.format_float_positional(float(case["value"]), precision=case["digits"], unique=False, trim="-")
                    self.assertEqual(plain, reference)

    def test_unit_text_uses_one_narrow_space(self):
        case = CASES["unit"]
        self.assertEqual(VISUALS.unit_text(case["value"], case["unit"]), case["expected"])

    def test_watt_device_names_are_authored_in_both_languages(self):
        for key, (names, watts) in VISUALS._WATT_DEVICES.items():
            with self.subTest(device=key):
                self.assertEqual(len(names), 2)
                self.assertTrue(all(name.strip() for name in names))
                self.assertGreater(watts, 0)
        for key in CASES["devices_differ_between_languages"]:
            with self.subTest(differs=key):
                en, de = VISUALS._WATT_DEVICES[key][0]
                self.assertNotEqual(en, de)


if __name__ == "__main__":
    unittest.main()
