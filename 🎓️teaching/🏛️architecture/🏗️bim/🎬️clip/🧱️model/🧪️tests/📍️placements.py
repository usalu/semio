"""📍 Storey-relative solids preserve physical placements as measured datums change."""

import ast
from copy import deepcopy
import json
import math
from pathlib import Path
import unittest

import numpy as np
from shapely.geometry import Polygon


class LocalPlacementContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        root = Path(__file__).parents[1]
        tree = ast.parse((root / "🐍️.py").read_text(encoding="utf-8"))
        names = {"_solid_geometry", "_stair_geometry", "_slab_finish_parts", "_flights"}
        nodes = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name in names]
        cls.api = {"math": math, "_by_class": lambda model, cls: [e for e in model["elements"] if e["class"] == cls], "deepcopy": deepcopy, "_z": lambda model, key: model[key], "_round": lambda value: round(value, 6), "_expand": lambda model, outline: deepcopy(outline)}
        exec(compile(ast.Module(body=nodes, type_ignores=[]), str(root / "🐍️.py"), "exec"), cls.api)
        cls.cases = json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))

    def test_generic_local_parts_resolve_world_datums(self):
        case = self.cases["generic"]
        source = {"tag": "finish", "guid": "f", "class": "IfcBuildingElementProxy", "storey": "first", "name": {}, "metadata":{"domains":case["domains"]}, "geometry": {"kind": "parquet", "base_level": "first", "parts": case["parts"]}}
        result = self.api["_solid_geometry"]({"first": case["level"]}, source)
        part = result["parts"][0]
        np.testing.assert_allclose([part["z0"], part["z1"]], case["world_bounds"], atol=1e-9)
        self.assertAlmostEqual(Polygon(part["outline"]).area * (part["z1"] - part["z0"]), case["volume"])
        self.assertEqual(source["geometry"]["parts"], case["parts"])

    def test_explicit_stair_steps_and_landings_have_distinct_origins(self):
        case = self.cases["stair"]
        source = {"tag": "stair", "guid": "s", "class": "IfcStair", "storey": "basement", "name": {}, "metadata":{"domains":case["domains"]}, "geometry": {"kind": "cellar_stair", "from_level": "from", "to_level": "to", "step_count": 1, "centre": [0, 0], "radius": 1, "inner_radius": .1, "step_parts": [case["step"]], "landing_parts": [case["landing"]], "structural_parts": []}}
        result = self.api["_stair_geometry"]({"from": case["from"], "to": case["to"]}, source)
        for key, expected in (("steps", "world_step"), ("landings", "world_landing")):
            part = result[key][0]
            np.testing.assert_allclose([part["z0"], part["z1"]], case[expected], atol=1e-9)
            self.assertAlmostEqual(Polygon(part["outline"]).area, 1)

    def test_level_roof_finish_accounts_for_regional_depth(self):
        case = self.cases["roof"]
        slab = deepcopy(case)
        parts = self.api["_slab_finish_parts"](slab)
        np.testing.assert_allclose([p["z1"] for p in parts], [case["baseline_top"]] * 2)
        self.assertAlmostEqual(sum(Polygon(p["outline"]).area * (p["z1"]-p["z0"]) for p in parts), case["baseline_volume"])
        slab["layers"][0]["thickness"] = case["event_thickness"]
        parts = self.api["_slab_finish_parts"](slab)
        np.testing.assert_allclose([p["z1"] for p in parts], [case["event_top"]] * 2)

    def test_ramp_flight_carries_the_authored_six_centimetre_slab(self):
        case = self.cases["ramp"]
        ramp = {"tag":"ramp"}
        obj = {"tag":"flight", "guid":"f", "class":"IfcRampFlight", "attributes":{"width":{"value":case["width"]}}, "geometry":{"ramp":"ramp", "from_level":"from", "to_level":"to", "portion":case["portion"], "path":case["path"], "thickness":{"value":case["thickness"]}}}
        result = self.api["_flights"]({"from":case["from"], "to":case["to"], "elements":[obj]}, ramp)[0]
        self.assertEqual(result["thickness"], case["thickness"])
        area = Polygon([[0,0],[case["width"],0],[case["width"],6.8],[0,6.8]]).area
        self.assertAlmostEqual(area * result["thickness"], case["expected_volume"])


if __name__ == "__main__":
    unittest.main()
                                                                                                                                     