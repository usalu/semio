"""🏗️ Beam joints share slab-owned concrete without fictitious floor openings."""

import ast
from copy import deepcopy
import json
from pathlib import Path
import unittest

from shapely.geometry import Polygon


class BeamSharedOwnershipContract(unittest.TestCase):
    def test_shared_and_exclusive_parts_remain_distinct_and_form_complete_beam(self):
        case = json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        tree = ast.parse((Path(__file__).parents[1]/"🐍️.py").read_text(encoding="utf-8"))
        node = next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "_solid_geometry")
        namespace = {"deepcopy":deepcopy,"_z":lambda m,key:m[key],"_round":lambda n:round(n,9),"_find":lambda m,key:m[key],"_slab_geometry":lambda m,slab:slab}
        exec(compile(ast.Module(body=[node],type_ignores=[]),"beam","exec"),namespace)
        obj = {"tag":case["beam"],"guid":"b","class":"IfcBeam","storey":"ground","name":{},"metadata":{"domains":case["domains"]},"geometry":{"kind":"beam","base_level":"ground","parts":[case["exclusive"]],"embedded_slabs":[case["slab"]]}}
        model = {"ground":0,case["slab"]:{"parts":[case["shared"]]}}
        result = namespace["_solid_geometry"](model,obj)
        self.assertEqual(len(result["parts"]),1)
        self.assertEqual(len(result["shared_parts"]),1)
        volumes = {key:sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in result[field]) for key,field in (("exclusive","parts"),("shared","shared_parts"))}
        self.assertAlmostEqual(volumes["exclusive"],case["volumes"]["exclusive"])
        self.assertAlmostEqual(volumes["shared"],case["volumes"]["shared"])
        self.assertAlmostEqual(sum(volumes.values()),case["volumes"]["total"])
        obj["geometry"]["parts"] = []
        flush = namespace["_solid_geometry"](model,obj)
        self.assertEqual(flush["parts"],[])
        self.assertEqual(flush["shared_parts"],[case["shared"]])

    def test_solid_embedded_beam_strip_has_full_concrete_self_weight(self):
        from types import SimpleNamespace
        import numpy as np
        case = json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["concrete"]
        tree = ast.parse((Path(__file__).parents[1]/"🧮️analysis/🐍️.py").read_text(encoding="utf-8"))
        node = next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name == "_composite_weight")
        namespace = {"LOAD_NAMES":{"self_weight":["Self weight","Eigengewicht"]},"_entry":lambda value,basis,*names,**extra:{"value":value,"basis":basis,**extra}}
        exec(compile(ast.Module(body=[node],type_ignores=[]),"beam-weight","exec"),namespace)
        model = {"rc":{"density":{"value":case["density"],"basis":"assumed"}},"hollow":{"density":{"value":case["hollow_density"],"basis":"assumed"}}}
        composite = {"compression_layer":{"value":case["compression"]},"rib_width":{"value":case["rib_width"]},"rib_pitch":{"value":case["rib_pitch"]},"concrete":"rc","ceramic":"hollow"}
        core = SimpleNamespace(_find=lambda m,key:m[key])
        result = namespace["_composite_weight"](core,model,composite,case["depth"],"rc")
        self.assertAlmostEqual(result["value"],case["expected_weight"])
        self.assertAlmostEqual(float(np.dot([case["depth"]],[case["density"]])),result["value"])
        self.assertEqual(result["components"]["ceramic"]["value"],0)


if __name__ == "__main__":
    unittest.main()
