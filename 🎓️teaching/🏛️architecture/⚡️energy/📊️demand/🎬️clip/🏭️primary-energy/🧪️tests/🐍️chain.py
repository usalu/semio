"""⛓️ The Nutzenergie → Endenergie → Primärenergie chain agrees between ♨️system-losses and 🏭️primary-energy and the shown text keeps the notation."""
import importlib.util
import json
import re
import unittest
from pathlib import Path

import numpy as np

HERE = Path(__file__).resolve().parent
CLIP = HERE.parent.parent
VECTORS = json.loads((HERE / "🔣️chain.json").read_text(encoding="utf-8"))


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


LOSSES = load("system_losses_chain", CLIP / "♨️system-losses" / "🐍️.py")
PRIMARY = load("primary_energy_chain", CLIP / "🏭️primary-energy" / "🐍️.py")


class Chain(unittest.TestCase):
    def test_final_energy_is_the_product_of_the_stage_figures(self):
        s = VECTORS["stages"]
        gas = float(np.prod([s["emission"], s["distribution"], s["storage"], s["gas"]])) * VECTORS["useful"]
        pump = float(np.prod([s["emission"], s["distribution"], s["storage"], s["heatPump"]])) * VECTORS["useful"]
        self.assertEqual(round(gas), VECTORS["final"]["gas"])
        self.assertEqual(round(pump), VECTORS["final"]["heatPump"])
        self.assertEqual(round(LOSSES.Q_E_GAS), VECTORS["final"]["gas"])
        self.assertEqual(round(LOSSES.Q_E_WP), VECTORS["final"]["heatPump"])
        self.assertEqual(PRIMARY.Q_END_GAS, VECTORS["final"]["gas"])
        self.assertEqual(PRIMARY.Q_END_WP, VECTORS["final"]["heatPump"])

    def test_primary_energy_is_final_energy_times_the_factor(self):
        self.assertEqual(PRIMARY.Q_PRIM_GAS, VECTORS["primary"]["gas"])
        self.assertEqual(PRIMARY.Q_PRIM_WP, VECTORS["primary"]["heatPump"])
        self.assertAlmostEqual(PRIMARY.F_P_GAS, VECTORS["factors"]["gas"])
        self.assertAlmostEqual(PRIMARY.F_P_STROM, VECTORS["factors"]["electricity"])
        self.assertAlmostEqual(PRIMARY.F_P_HOLZ, VECTORS["factors"]["wood"])
        self.assertAlmostEqual(PRIMARY.F_P_UMWELT, VECTORS["factors"]["ambient"])

    def test_plant_figure_of_the_standards_includes_the_factor(self):
        self.assertAlmostEqual(PRIMARY.Q_PRIM_GAS / VECTORS["useful"], VECTORS["plantFigure"]["gas"])
        self.assertAlmostEqual(PRIMARY.Q_PRIM_WP / VECTORS["useful"], VECTORS["plantFigure"]["heatPump"])

    def test_heat_pump_split_adds_up(self):
        split = VECTORS["heatPumpSplit"]
        self.assertEqual(PRIMARY.Q_AMBIENT_WP, split["ambient"])
        self.assertEqual(PRIMARY.Q_POWER_SHARE, split["electricity"])
        self.assertEqual(split["ambient"] + split["electricity"], VECTORS["useful"])
        self.assertEqual(PRIMARY.Q_END_WP - PRIMARY.Q_POWER_SHARE, split["losses"])

    def test_reference_limit(self):
        limit = VECTORS["limit"]
        self.assertEqual(PRIMARY.Q_REF, limit["reference"])
        self.assertAlmostEqual(PRIMARY.REF_SHARE, limit["share"])
        self.assertEqual(PRIMARY.Q_LIMIT, limit["value"])

    def test_narration_math_is_balanced_and_the_notation_is_clean(self):
        patterns = [re.compile(p) for p in VECTORS["forbiddenPatterns"]]
        for module in (LOSSES, PRIMARY):
            source = Path(module.__file__).read_text(encoding="utf-8")
            code = "\n".join(line for line in source.splitlines() if not line.lstrip().startswith("#"))
            for pattern in patterns:
                with self.subTest(module=module.__name__, pattern=pattern.pattern):
                    self.assertIsNone(pattern.search(code))
            for name, cls in vars(module).items():
                clauses = getattr(cls, "NARRATION", None)
                if not clauses:
                    continue
                for key, en, de in clauses:
                    with self.subTest(scene=name, key=key):
                        self.assertEqual(en.count("$") % 2, 0)
                        self.assertEqual(de.count("$") % 2, 0)
                        self.assertEqual(en.count("$"), de.count("$"))


if __name__ == "__main__":
    unittest.main()
