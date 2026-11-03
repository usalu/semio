"""🧍 Verify the Excel program plan places one handcrafted occupant per user on the free floor, validated with Shapely against the drawn Manim bounds."""
import importlib
import json
import sys
import unittest
from pathlib import Path

import numpy as np
from shapely.geometry import box

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIP = ROOT / "🎓️teaching/🏛️architecture/📊️excel/🎬️clip"
CASES = json.loads((CLIP / "🧪️tests/🧍️occupants/🔣️.cases.json").read_text(encoding="utf-8"))
PROGRAM = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📋️program.🐍️")
NAKAGIN = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
STAFFAGE = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")


def footprint(mob):
    """📦 Axis-aligned drawn bounds of a Manim object as a Shapely box."""
    (x0, y0, _), (x1, y1, _) = mob.get_corner(np.array([-1, -1, 0])), mob.get_corner(np.array([1, 1, 0]))
    return box(x0, y0, x1, y1)


def plan_of(key):
    """🗺️ The drawn plan of one capsule type with its occupants, without constructing a scene."""
    scene = PROGRAM.Beat3_Program.__new__(PROGRAM.Beat3_Program)
    return scene.plan({t.key: t for t in NAKAGIN.capsule_types()}[key], np.array(CASES["at"], dtype=float), CASES["scale"], label=False, people=True)


class OccupantPlan(unittest.TestCase):
    def test_every_type_shows_its_users(self):
        """🔢 One occupant per user constant of the capsule type."""
        for key, count in CASES["occupants"].items():
            with self.subTest(type=key):
                self.assertEqual(len(plan_of(key).people), count)
                self.assertEqual(count, {t.key: t.occupants for t in NAKAGIN.capsule_types()}[key])

    def test_occupants_stay_on_the_free_floor_and_apart(self):
        """🧱 Drawn occupants lie inside the net floor, miss every fitting rectangle and never overlap each other."""
        for key in CASES["occupants"]:
            with self.subTest(type=key):
                plan = plan_of(key)
                floor = footprint(plan.floor)
                fittings = [footprint(rect) for fits in plan.fits.values() for rect, _, _ in fits]
                people = [footprint(person) for person in plan.people]
                for index, person in enumerate(people):
                    self.assertTrue(floor.buffer(CASES["tolerance"]).contains(person), "outside the net floor")
                    for fitting in fittings:
                        self.assertLess(person.intersection(fitting).area, CASES["tolerance"], "on a fitting")
                    for other in people[index + 1:]:
                        self.assertLess(person.intersection(other).area, CASES["tolerance"], "overlapping occupants")

    def test_occupant_is_a_true_scale_handcrafted_plan_figure(self):
        """📏 The rotated staffage plan figure measures a standing adult from above in metres."""
        person = plan_of("A").people[0]
        metres = np.array([person.width, person.height]) / CASES["scale"]
        self.assertTrue(CASES["person"]["width_m"][0] <= metres[0] <= CASES["person"]["width_m"][1], metres)
        self.assertTrue(CASES["person"]["depth_m"][0] <= metres[1] <= CASES["person"]["depth_m"][1], metres)
        figure = STAFFAGE.staffage(CASES["cast"], unit=1.0)
        self.assertAlmostEqual(metres[0], STAFFAGE.staffage_height(CASES["cast"]), delta=0.005)
        self.assertAlmostEqual(metres[1], figure.width, delta=0.005)

    def test_spots_fail_loudly_when_the_floor_is_full(self):
        """🚫 A room without free floor raises instead of overlapping."""
        with self.assertRaises(ValueError):
            PROGRAM.occupant_spots((1.0, 1.0), [(0.0, 0.0, 1.0, 1.0)], (0.5, 0.5), 1)


if __name__ == "__main__":
    unittest.main(verbosity=2)
