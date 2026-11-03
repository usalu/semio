"""🧪 The running example derives the numbers the clips and the narration quote."""

import importlib
import unittest

E = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")


class RunningExample(unittest.TestCase):
    def test_quoted_powers(self):
        self.assertEqual(E.PHI_T_W, 2300.0)
        self.assertEqual(E.PHI_V_W, 1500.0)
        self.assertEqual(E.PHI_V_WRG_W, 300.0)
        self.assertEqual(E.PHI_LOSS_W, 3800.0)
        self.assertEqual(round(E.Q_LOSS_KWH_PER_A, -2), 16000.0)

    def test_volumetric_heat_capacity_matches_air_properties(self):
        self.assertAlmostEqual(E.C_LUFT_WH_M3_K, E.RHO_AIR_KG_M3 * E.CP_AIR_J_KG_K / 3600.0, delta=0.01)

    def test_season_mean_outdoor_temperature(self):
        self.assertAlmostEqual(E.THETA_E_SEASON_MEAN_C, 3.49, places=2)

    def test_summer_loads(self):
        self.assertEqual(round(E.Q_SENS_SUMMER_W, -1), 450.0)
        self.assertEqual(round(E.Q_LAT_SUMMER_W, -1), 810.0)

    def test_every_envelope_part_is_named_in_both_languages(self):
        self.assertEqual(set(E.ENVELOPE_EN), {name for name, *_ in E.ENVELOPE})


if __name__ == "__main__":
    unittest.main()
