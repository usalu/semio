"""🧮 Pin the month numbers of the energy balance to an independent numpy derivation of the running example."""
from __future__ import annotations
import importlib
import json
import os
from pathlib import Path
import sys
import unittest

sys.dont_write_bytecode = True
ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
FIXTURE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
os.environ["SEMIO_TEACHING_LANGUAGE"] = FIXTURE["language"]
import numpy as np
_RECAP = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🔁️recap.🐍️")
_REGULATIONS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🐍️")

AREAS = np.array([209.0, 89.0, 30.0, 80.0])
U_VALUES = np.array([0.24, 0.20, 1.10, 0.30])
GROUND_K = 10.0


def reference(d_theta: float, irradiation: float):
    """📐 Losses and gains in kWh for one month, derived without the authored module."""
    deltas = np.array([d_theta, d_theta, d_theta, GROUND_K])
    surfaces = float(np.sum(AREAS * U_VALUES * deltas))
    bridges = 0.05 * float(AREAS.sum()) * d_theta
    ventilation = 440.0 * 0.5 * 0.34 * d_theta
    q_l = (surfaces + bridges + ventilation) * FIXTURE["hoursPerMonth"] / 1000.0
    f = FIXTURE["solarFactors"]
    q_g = 30.0 * f["gPerp"] * f["fS"] * f["fC"] * f["fF"] * f["fW"] * irradiation + 5.0 * 60.0 * FIXTURE["hoursPerMonth"] / 1000.0
    return q_l, q_g


def eta_heat(q_l: float, q_g: float) -> float:
    gamma = q_g / q_l
    return (1.0 - gamma ** 2) / (1.0 - gamma ** 3)


def eta_cool(q_l: float, q_g: float) -> float:
    gamma = q_g / q_l
    return (1.0 - gamma ** -2) / (1.0 - gamma ** -3)


class BalanceTests(unittest.TestCase):
    """⚖️ The authored constants equal the fixture and the numpy oracle."""

    def test_heating_months(self):
        """🔥 Q_h = Q_l − η·Q_g for the transition month and January."""
        pairs = {"transition month": (_RECAP.Q_L, _RECAP.Q_G, _RECAP.ETA_H), "January": (_RECAP.Q_L_JAN, _RECAP.Q_G_JAN, _RECAP.ETA_JAN)}
        for case in FIXTURE["cases"]:
            with self.subTest(case=case["name"]):
                q_l, q_g = reference(case["dThetaK"], case["irradiationKWhM2"])
                eta = eta_heat(q_l, q_g)
                self.assertAlmostEqual(q_l, case["qlKWh"], delta=1.0)
                self.assertAlmostEqual(q_g, case["qgKWh"], delta=1.0)
                self.assertAlmostEqual(eta, case["eta"], delta=0.001)
                self.assertAlmostEqual(q_l - eta * q_g, case["qhKWh"], delta=1.0)
                for authored, expected in zip(pairs[case["name"]], (q_l, q_g, eta)):
                    self.assertAlmostEqual(authored, expected, places=6)

    def test_solar_factors_are_the_running_example_values_and_the_narration_quotes_the_result(self):
        """☀️ g, F_S, F_C, F_F, F_W equal the shared running-example glazing; the quoted Q_S equals the computed one."""
        f = FIXTURE["solarFactors"]
        example = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")
        self.assertEqual((_RECAP.G_VALUE, _RECAP.F_S, _RECAP.F_C, _RECAP.F_F, _RECAP.F_W), (example.G_PERP, example.F_S_WINTER, example.F_C_NONE, example.F_F, example.F_W))
        self.assertEqual((f["gPerp"], f["fS"], f["fC"], f["fF"], f["fW"]), (example.G_PERP, example.F_S_WINTER, example.F_C_NONE, example.F_F, example.F_W))
        quoted = next(clause for clause in _RECAP.Beat3_Gewinnseite.NARRATION if clause[0] == "solar_block")
        for text in quoted[1:]:
            self.assertIn(f"Q_S = {round(_RECAP.Q_SOLAR)}", text)

    def test_cooling_month(self):
        """❄️ Q_c = Q_g − η_c·Q_l for July."""
        case = FIXTURE["cooling"]
        q_l, q_g = reference(case["dThetaK"], case["irradiationKWhM2"])
        eta = eta_cool(q_l, q_g)
        self.assertAlmostEqual(eta, case["etaC"], delta=0.001)
        self.assertAlmostEqual(q_g - eta * q_l, case["qcKWh"], delta=1.5)
        self.assertAlmostEqual(_RECAP.ETA_C, eta, places=6)

    def test_degree_days_reproduce_the_running_example(self):
        """🌡️ G_{t,20/15} of the schematic climate equals the running example's 3500 Kd/a."""
        case = FIXTURE["degreeDays"]
        months = np.array(_REGULATIONS.THETA_E)
        days = np.linspace(0.0, 12.0, 4801)
        theta = np.array([_REGULATIONS._catmull(_REGULATIONS.THETA_E, m) for m in days])
        daily = np.where(theta < case["heatingLimitC"], case["indoorC"] - theta, 0.0)
        oracle = float(np.trapezoid(daily, days) * 365.0 / 12.0)
        self.assertAlmostEqual(oracle, case["expectedKdPerA"], delta=case["toleranceKdPerA"])
        self.assertAlmostEqual(_REGULATIONS._degree_days(12.0), oracle, delta=15.0)
        self.assertEqual(len(months), 12)


if __name__ == "__main__":
    unittest.main()
