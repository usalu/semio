"""🧪 Portable heating architecture cases: hidden air-path guides and one heat-flow symbol Q̇."""

import json
import unittest
from pathlib import Path

CHAPTERS = Path(__file__).resolve().parent.parent
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


def source(chapter):
    """📖 Authored scene source of one heating chapter."""
    return (CHAPTERS / chapter / "🐍️.py").read_text(encoding="utf-8")


class HeatingArchitecture(unittest.TestCase):
    """🏛️ Air movement stays particles only; the heat flow is Q̇ in heating and ventilation."""

    def test_air_path_guides_stay_hidden(self):
        for case in CASES["air_guides_hidden"]:
            with self.subTest(chapter=case["chapter"], call=case["call"]):
                self.assertIn(case["call"], source(case["chapter"]))

    def test_heat_flow_is_written_q_dot(self):
        case = CASES["heat_flow_symbol"]
        for chapter in case["chapters"]:
            text = source(chapter)
            for token in case["forbidden"]:
                with self.subTest(chapter=chapter, token=token):
                    self.assertNotIn(token, text)
        for chapter, token in case["required"].items():
            with self.subTest(chapter=chapter, token=token):
                self.assertIn(token, source(chapter))

    def test_equilibrium_label_appears_after_readouts_fade(self):
        case = CASES["equilibrium_label_handoff"]
        conduction = next(CHAPTERS.glob("*conduction*/🐍️.py")).read_text(encoding="utf-8")
        transitions = json.loads(next(CHAPTERS.glob("*conduction*/🔀️transitions/🔣️.json")).read_text(encoding="utf-8"))
        item = next(t for t in transitions["transitions"] if t["id"] == case["transition_id"])
        self.assertEqual(item["recipe"], case["recipe"])
        self.assertIs(item["source"], None)
        self.assertLess(conduction.index(case["fade_out_marker"]), conduction.index(case["appear_marker"]))



    def test_micro_lattice_motion_stays_continuous(self):
        case = CASES["micro_lattice_motion"]
        visuals = (CHAPTERS.parents[1] / "🏗️authoring" / "🎨️visuals" / "🐍️.py").read_text(encoding="utf-8")
        self.assertIn(case["hold_for_wait"], visuals)
        conduction = next(CHAPTERS.glob("*conduction*/🐍️.py")).read_text(encoding="utf-8")
        self.assertIn(case["heat_attr"], conduction)
        self.assertIn(case["updater_dt"], conduction)
        for token in case["forbidden"]:
            self.assertNotIn(token, conduction)



    def test_r_formula_clears_insulation_slab(self):
        case = CASES["r_formula_clearance"]
        conduction = next(CHAPTERS.glob("*conduction*/🐍️.py")).read_text(encoding="utf-8")
        self.assertIn(case["marker"], conduction)


if __name__ == "__main__":
    unittest.main()
