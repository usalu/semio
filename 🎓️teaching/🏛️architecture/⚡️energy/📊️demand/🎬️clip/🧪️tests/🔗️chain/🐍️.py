"""🔗 Energy-chain contract of the certificate and practice chapters: shared numbers, GEG Anlage 10 classes, notation, narration values."""

from __future__ import annotations

import importlib
import json
import re
import sys
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import numpy as np

FIXTURE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
DEMAND = next(path for path in Path(__file__).resolve().parents if path.name == "🎬️clip")
PREFIX = "🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip"
certificate = importlib.import_module(f"{PREFIX}.🏷️certificate.🐍️")
practice = importlib.import_module(f"{PREFIX}.🧪️practice.🐍️")
demand_power = importlib.import_module(f"{PREFIX}.⚡️demand-power.🐍️")
primary = importlib.import_module(f"{PREFIX}.🏭️primary-energy.🐍️")


def narration(clip_module):
    scenes = [obj for name, obj in vars(clip_module).items() if name.startswith("Beat") and hasattr(obj, "NARRATION")]
    return {key: (en, de) for scene in scenes for key, en, de in scene.NARRATION}


class ChainContract(unittest.TestCase):
    """🧮 The chain numbers agree across chapters and with the language-agnostic fixture."""

    def test_chain_numbers_match_the_fixture(self):
        gas, wp = FIXTURE["gas"], FIXTURE["heat_pump"]
        self.assertEqual(practice.CHAIN_NUTZ, FIXTURE["useful"])
        self.assertEqual((practice.CHAIN_END, practice.CHAIN_PRIM, practice.CHAIN_CLASS), (gas["final"], gas["primary"], gas["class"]))
        self.assertEqual((practice.CHAIN_END_WP, practice.CHAIN_PRIM_WP, practice.CHAIN_CLASS_WP), (wp["final"], wp["primary"], wp["class"]))
        self.assertEqual((certificate.GAS_END, certificate.WP_END, certificate.Q_NUTZ), (gas["final"], wp["final"], FIXTURE["useful"]))
        self.assertAlmostEqual(practice.CHAIN_END / practice.CHAIN_NUTZ, gas["e_E"], places=2)
        self.assertAlmostEqual(practice.CHAIN_END_WP / practice.CHAIN_NUTZ, wp["e_E"], places=2)

    def test_annual_heating_demand_is_one_value_from_the_running_example(self):
        load = demand_power._annual_load()
        self.assertEqual(demand_power.Q_H_KWH_A, FIXTURE["annual_heating_demand"])
        self.assertAlmostEqual(float(np.sum(load)) * demand_power.PEAK_KW, FIXTURE["annual_heating_demand"], delta=1.0)
        self.assertEqual(round(float(demand_power._cumulative(load)[-1]), -2), FIXTURE["annual_heating_demand"])

    def test_primary_energy_expenditure_numbers_are_derived_and_narrated(self):
        self.assertAlmostEqual(primary.E_P_GAS, FIXTURE["gas"]["primary"] / FIXTURE["useful"])
        self.assertAlmostEqual(primary.E_P_WP, FIXTURE["heat_pump"]["primary"] / FIXTURE["useful"])
        en, de = narration(primary)["anlagenzahl"]
        self.assertIn("$1.54$ against $0.56$", en)
        self.assertIn("$1{,}54$ gegen $0{,}56$", de)

    def test_counted_units_use_the_singular_for_one(self):
        self.assertIn(r"1\;\text{month}", certificate._count_label(1.0))
        self.assertNotIn("months", certificate._count_label(1.4))
        self.assertIn(r"2\;\text{months}", certificate._count_label(2.0))

    def test_class_scale_equals_anlage_10_and_a_third_party_lookup(self):
        letters = [letter for letter, _ in FIXTURE["classes"]]
        bounds = np.array([bound for _, bound in FIXTURE["classes"] if bound is not None], dtype=float)
        self.assertEqual([(c[0], c[1]) for c in certificate.CLASSES], [tuple(c) for c in FIXTURE["classes"]])
        self.assertEqual([(c[0], c[1]) for c in practice.CHAIN_CLASSES], [tuple(c) for c in FIXTURE["classes"]])
        for value, expected in FIXTURE["boundaries"].items():
            reference = letters[int(np.searchsorted(bounds, float(value), side="left"))]
            self.assertEqual(reference, expected, value)
            self.assertEqual(certificate._class_letter(float(value)), expected, value)
            self.assertEqual(practice._class_of(float(value))[0], expected, value)

    def test_narration_states_the_chain_values_with_units(self):
        for clip, cases in FIXTURE["narration_values"].items():
            module = certificate if clip == "🏷️certificate" else practice
            lines = narration(module)
            for key, snippets in cases.items():
                keys = [k for k in lines if k == key or k.startswith(key)]
                self.assertTrue(keys, key)
                joined = " ".join(" ".join(lines[k]) for k in keys)
                for snippet in snippets:
                    self.assertIn(snippet, joined, (clip, key, snippet))

    def test_notation_rules_hold_in_both_chapters(self):
        for clip in FIXTURE["chapters"]:
            text = (DEMAND / clip / "🐍️.py").read_text(encoding="utf-8")
            for token in FIXTURE["forbidden_source"]:
                self.assertNotIn(token, text, (clip, token))

    def test_narration_math_is_balanced_and_symbols_are_not_spelled_out(self):
        for module in (certificate, practice):
            for key, (en, de) in narration(module).items():
                for line in (en, de):
                    self.assertEqual(line.count("$") % 2, 0, key)
                    outside = re.sub(r"\$[^$]*\$", "", line)
                    self.assertNotRegex(outside, r"[ΔθΦρηλ]", key)
                    self.assertNotRegex(outside, r"\[(?:W|kW|kWh)\]", key)


if __name__ == "__main__":
    unittest.main()
