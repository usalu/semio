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

    def test_year_profile_reproduces_heating_days_and_degree_days(self):
        theta = E.year_outdoor_profile()
        heating = theta < E.THETA_HG_C
        self.assertEqual(theta.size, 365)
        self.assertEqual(int(heating.sum()), E.SEASON_DAYS)
        self.assertAlmostEqual(float((E.THETA_I_WINTER_C - theta)[heating].sum()), E.GT_K_D_PER_A, places=3)
        self.assertLess(float(theta.max()), E.THETA_I_WINTER_C + 5.0)

    def test_degree_days_convert_to_the_din_v_4108_6_gradtagszahlfaktor(self):
        f_gt_kkh_per_a = 0.024 * E.GT_K_D_PER_A
        self.assertAlmostEqual(f_gt_kkh_per_a, 84.0)
        self.assertAlmostEqual((E.H_T_W_K + E.H_V_W_K) * f_gt_kkh_per_a, E.Q_LOSS_KWH_PER_A)

    def test_summer_loads(self):
        self.assertEqual(round(E.Q_SENS_SUMMER_W, -1), 450.0)
        self.assertEqual(round(E.Q_LAT_SUMMER_W, -1), 810.0)

    def test_winter_latent_load_is_as_large_as_the_summer_one(self):
        self.assertAlmostEqual(E.D_X_WINTER_G_KG, 4.3, places=1)
        self.assertEqual(round(E.Q_LAT_WINTER_W, -1), 770.0)
        self.assertGreater(E.Q_LAT_WINTER_W, E.Q_SENS_SUMMER_W)

    def test_the_example_winter_day_and_the_design_day_are_two_named_cases(self):
        self.assertEqual((E.THETA_E_WINTER_C, E.D_THETA_WINTER_K), (0.0, 20.0))
        self.assertEqual((E.THETA_E_DESIGN_C, E.D_THETA_DESIGN_K), (-12.0, 32.0))
        self.assertEqual(round(E.Q_H_DESIGN_W), 6080)
        self.assertIn("Beispiel-Wintertag", E.winter_day_label("de"))
        self.assertIn("Example winter day", E.winter_day_label("en"))
        for language, day in (("de", "Auslegungstag"), ("en", "Design day")):
            label = E.design_day_label(language)
            self.assertIn(day, label)
            self.assertIn("{-}12", label)
            self.assertIn("DIN/TS 12831-1:2020-04", E.design_day_source(language))

    def test_the_glazing_values_every_video_shares(self):
        self.assertAlmostEqual(E.G_PERP, 0.60)
        self.assertAlmostEqual(E.G_PERP, E.G_PERP_SPLIT)
        self.assertAlmostEqual(E.G_SHARE_DIRECT + E.G_SHARE_SECONDARY_IN + E.G_SHARE_REFLECTED + E.G_SHARE_SECONDARY_OUT, 1.0)
        self.assertAlmostEqual(E.WINDOW_A_M2, 1.92)
        self.assertAlmostEqual(E.F_SOLAR_WINTER, 0.6 * 0.8 * 0.7 * 0.9)

    def test_every_envelope_part_is_named_in_both_languages(self):
        self.assertEqual(set(E.ENVELOPE_EN), {name for name, *_ in E.ENVELOPE})


if __name__ == "__main__":
    unittest.main()
