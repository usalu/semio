"""🚘 Check that the BIM plan car preserves the catalog metre scale at every heading."""
import ast
import importlib
import json
import sys
import unittest
from pathlib import Path

from shapely.geometry import Polygon, box

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
INFO = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.📊️information.🐍️")
ART = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")


class CarScale(unittest.TestCase):
    def test_plan_car_preserves_catalog_dimensions(self):
        """📏 Shapely bounds match the source cast at the scene's metre scale."""
        scene = INFO.Beat1_DataToInformation.__new__(INFO.Beat1_DataToInformation)
        scene._c = {"car_length": 4.7, "car_width": 1.8}
        for scale in CASES["scales"]:
            for heading in CASES["headings"]:
                with self.subTest(scale=scale, heading=heading):
                    car = scene._car(scale, heading)
                    source = ART.staffage(CASES["cast"], unit=scale)
                    expected = (source.width, source.height) if heading == 0 else (source.height, source.width)
                    actual = box(0, 0, car.width, car.height)
                    reference = box(0, 0, *expected)
                    self.assertLess(actual.symmetric_difference(reference).area, CASES["tolerance"])

    def test_question_card_keeps_the_period_car_inside_the_assumed_envelope(self):
        """📦 Shapely: the drawn saloon matches catalog metres and sits inside the larger assumed rectangle."""
        scene = INFO.Beat1_DataToInformation.__new__(INFO.Beat1_DataToInformation)
        unit = CASES["callout_unit"]
        card = scene.question_car(unit, CASES["assumed_length_m"], CASES["assumed_width_m"])
        period = box(-card["length_scene"] / 2, -card["width_scene"] / 2, card["length_scene"] / 2, card["width_scene"] / 2)
        drawn = box(-card["car"].width / 2, -card["car"].height / 2, card["car"].width / 2, card["car"].height / 2)
        envelope = Polygon([(x, y) for x, y, _ in card["envelope_corners"]])
        self.assertLess(drawn.symmetric_difference(period).area, CASES["tolerance"])
        self.assertTrue(envelope.buffer(CASES["tolerance"]).covers(period))
        self.assertGreater(envelope.difference(period).area, CASES["envelope_gap_m2"])
        self.assertAlmostEqual(card["length_m"] * unit, card["length_scene"])
        self.assertGreater(card["length_m"], card["width_m"])
        self.assertNotAlmostEqual(card["length_m"], CASES["assumed_length_m"], places=2)

    def test_clearance_rectangle_keeps_its_area_when_turned(self):
        """📦 Shapely: the assumed rectangle rotates without changing area or stretching the period car."""
        length, width = CASES["assumed_length_m"], CASES["assumed_width_m"]
        for heading in CASES["headings"]:
            with self.subTest(heading=heading):
                ring = Polygon(INFO.clearance_corners(length, width, 1.0, 2.0, heading))
                self.assertAlmostEqual(ring.area, length * width, places=6)
        period_length, period_width = INFO.period_plan_metres()
        self.assertGreater(length * width, period_length * period_width)

    def test_swept_path_uses_the_period_track(self):
        """🛞 The driven outline follows the catalog track, and the driver calls that half-width."""
        _, track = INFO.period_plan_metres()
        self.assertAlmostEqual(INFO.swept_half_metres(), track / 2)
        self.assertLess(INFO.swept_half_metres(), 1.2)
        self.assertNotAlmostEqual(INFO.swept_half_metres(), CASES["assumed_width_m"] / 2, places=3)
        tree = ast.parse(Path(INFO.__file__).read_text(encoding="utf-8"))
        driver = next(node for node in ast.walk(tree) if isinstance(node, ast.FunctionDef) and node.name == "_driver")
        halves = [node for node in ast.walk(driver) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "half" for target in node.targets)]
        self.assertEqual(len(halves), 1)
        call = halves[0].value
        self.assertIsInstance(call, ast.Call)
        self.assertEqual(call.func.id, "swept_half_metres")


if __name__ == "__main__":
    unittest.main()
