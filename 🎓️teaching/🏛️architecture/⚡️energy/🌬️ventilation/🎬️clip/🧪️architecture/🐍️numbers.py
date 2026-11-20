"""🧪 The numbers of ventilation chapters 4 to 6 follow from first principles and agree with the authored scenes."""

import ast
import importlib
import json
import re
import sys
import unittest
from pathlib import Path

import numpy as np

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CHAPTERS = Path(__file__).resolve().parent.parent
CASES = json.loads(Path(__file__).with_name("🔣️.numbers.json").read_text(encoding="utf-8"))
RUNNING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📏️running-example.🐍️")


def natural():
    """🔬 Load the authored stack-effect functions and constants without the scene side effects."""
    tree = ast.parse((CHAPTERS / "🪟️natural" / "🐍️.py").read_text(encoding="utf-8"))
    functions, constants = {"_rho_air", "_stack_chain"}, {"G_ACC", "RHO_AIR", "CD_OPENING", "A_OPENING"}
    keep = [n for n in tree.body if (isinstance(n, ast.FunctionDef) and n.name in functions)
            or (isinstance(n, ast.Assign) and constants & {m.id for t in n.targets for m in ast.walk(t) if isinstance(m, ast.Name)})]
    tree.body = keep
    scope = {"np": np}
    exec(compile(ast.fix_missing_locations(tree), "natural", "exec"), scope)
    return scope


class VentilationNumbers(unittest.TestCase):
    """🔢 Tilted window, stack effect and latent hand-over."""

    def test_air_density_belongs_to_a_stated_temperature(self):
        spec = CASES["airDensity"]
        for case in spec["cases"]:
            ideal = spec["pressurePa"] / (spec["gasConstant"] * (273.15 + case["celsius"]))
            self.assertAlmostEqual(ideal, case["kgM3"], places=3)
        text = (CHAPTERS / "🌡️heat-flow" / "🐍️.py").read_text(encoding="utf-8")
        for stated in spec["stated"]:
            rho, temperature = re.escape(stated["rounded"]).replace(r"\.", "[.,]"), re.escape(f"{stated['celsius']}\\,°\\mathrm{{C}}")
            self.assertRegex(text, rf"(?s){rho}.{{0,160}}{temperature}|{temperature}.{{0,160}}{rho}")

    def test_tilted_window_share_after_five_minutes(self):
        case = CASES["tilt"]
        share = case["minutes"] / 60.0 * case["airChangePerHour"]
        self.assertAlmostEqual(share, case["share"], places=3)
        low, high = case["measuredRangePerHour"]
        self.assertTrue(low <= case["airChangePerHour"] <= high)

    def test_stack_chain_matches_first_principles(self):
        spec, api = CASES["stack"], natural()
        for case in spec["cases"]:
            with self.subTest(case=case["id"]):
                rho = lambda c: spec["pressurePa"] / (spec["gasConstant"] * (273.15 + c))
                drho = rho(case["outdoorC"]) - rho(case["indoorC"])
                dp = case["heightM"] * spec["gravity"] * drho
                v = np.sqrt(2.0 * dp / spec["airDensityForSpeed"])
                flow = spec["dischargeCoefficient"] * spec["openingAreaM2"] * v * 3600.0
                self.assertAlmostEqual(drho, case["densityDifference"], places=3)
                self.assertAlmostEqual(dp, case["pressurePa"], places=2)
                self.assertAlmostEqual(v, case["speedMS"], places=2)
                self.assertAlmostEqual(flow, case["flowM3H"], delta=1.0)
                chain = api["_stack_chain"](case["heightM"], case["outdoorC"], case["indoorC"])
                for key, expected in (("drho", drho), ("dp", dp), ("v", v), ("vdot_h", flow)):
                    self.assertAlmostEqual(chain[key], expected, places=6)

    def test_literal_narration_quotes_the_computed_numbers(self):
        def narration(folder, cls):
            tree = ast.parse((CHAPTERS / folder / "🐍️.py").read_text(encoding="utf-8"))
            node = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == cls)
            value = next(s.value for s in node.body if isinstance(s, ast.Assign) and s.targets[0].id == "NARRATION")
            return " ".join(en + " " + de for _, en, de in ast.literal_eval(value))

        api = natural()
        low, tall, winter = (api["_stack_chain"](2.5, 18.0, 26.0), api["_stack_chain"](4.4, 18.0, 26.0), api["_stack_chain"](4.4, 0.0, 20.0))
        text = narration("🪟️natural", "Beat4_Auftrieb")
        for quoted in (f"{low['rho_e']:.3f}", f"{low['rho_i']:.3f}", f"{low['drho']:.3f}", f"{low['dp']:.1f}", f"{low['v']:.2f}",
                       f"{low['vdot']:.3f}", f"{round(low['vdot_h'], -1):.0f}", f"{tall['dp']:.1f}", f"{winter['dp']:.1f}",
                       f"{winter['dp'] / tall['dp']:.1f}"):
            self.assertIn(quoted, text)
        self.assertIn(f"{round(RUNNING.Q_LAT_WINTER_W, -1):.0f}", narration("⚙️mechanical", "Beat2_Waermerueckgewinnung"))

    def test_latent_load_exists_in_both_seasons(self):
        spec = CASES["latent"]
        for season, running in (("winter", RUNNING.Q_LAT_WINTER_W), ("summer", RUNNING.Q_LAT_SUMMER_W)):
            with self.subTest(season=season):
                case = spec[season]
                power = spec["volumeFlowM3H"] / 3600.0 * spec["density"] * spec["vaporisationKJKg"] * case["humidityDifferenceGKg"]
                self.assertAlmostEqual(power, running, places=6)
                self.assertEqual(round(power, -1), case["powerW"])
                self.assertEqual(case["sensibleW"] + case["powerW"], case["sumW"])
                self.assertEqual(round(case["sensibleW"] * (1 - spec["recovery"]), -1) + case["powerW"], case["sumWithRecoveryW"])


if __name__ == "__main__":
    unittest.main()
