"""🧪 The heating calculation quotes the running example: the narrated annual numbers are the shared derived ones."""

from __future__ import annotations

import importlib
import unittest

E = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
CALCULATION = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️")


def kwh_per_year(value: float) -> str:
    return rf"${value:,.0f}".replace(",", r"\,") + r"\,\mathrm{kWh/a}$"


def narrated(language: int) -> str:
    return " ".join(clause[language] for scene in vars(CALCULATION).values() if hasattr(scene, "NARRATION") for clause in scene.NARRATION)


class HeatingCalculationNumbers(unittest.TestCase):
    def test_the_scene_reads_the_shared_demand_and_utilisation(self):
        self.assertEqual(CALCULATION._Q_H, E.Q_H_KWH_PER_A)
        self.assertEqual(CALCULATION._Q_USE, E.Q_USE_KWH_PER_A)
        self.assertEqual(CALCULATION._ETA, E.ETA_UTILISATION)
        self.assertEqual(CALCULATION._Q_LOSS_SHOWN, E.Q_LOSS_SHOWN_KWH_PER_A)

    def test_the_narration_quotes_the_shared_values_in_both_languages(self):
        for language in (1, 2):
            text = narrated(language)
            for value in (E.Q_LOSS_SHOWN_KWH_PER_A, E.Q_USE_KWH_PER_A, E.Q_H_KWH_PER_A):
                with self.subTest(language=language, value=value):
                    self.assertIn(kwh_per_year(value), text)


if __name__ == "__main__":
    unittest.main()
