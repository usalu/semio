"""🧪 Portable exact cache cases checked against independent geometry."""
import importlib
import json
import unittest
from fractions import Fraction
from pathlib import Path
import jsonschema
from shapely.geometry import Polygon

cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
here = Path(__file__).parent
cases = json.loads((here / "🔣️.cases.json").read_text(encoding="utf-8"))
schema = json.loads((here.parent / "🔣️.schema.json").read_text(encoding="utf-8"))


class ExactCache(unittest.TestCase):
    """📐 Verify the portable codec contract and independent volume oracle."""

    def test_exact_values_and_geometry(self):
        model = cases["model"]
        height = Fraction(model["height"])
        value = {"points": tuple(tuple(map(Fraction, point)) for point in model["vertices"]), "height": height, "integers": tuple(int(x) for x in cases["exactIntegers"])}
        encoded = cache.encode(value)
        record = {"revision": cache.REVISION, "key": "0" * 64, "value": encoded, "digest": cache.digest(encoded)}
        jsonschema.validate(record, schema)
        self.assertEqual(cache.decode(encoded), value)
        oracle = Polygon(model["vertices"]).area * float(height)
        self.assertAlmostEqual(oracle, float(Fraction(cases["expectedVolume"])))
        for integer in cases["invalidIntegers"]:
            with self.assertRaises(ValueError):
                cache.decode(["integer", integer])
