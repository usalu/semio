"""🕳️ Horizontal native openings cut each occupied regional slab thickness."""

from copy import deepcopy
import importlib
import json
from pathlib import Path
import unittest

from shapely.geometry import Polygon
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class SlabOpeningContract(unittest.TestCase):
    def setUp(self):
        self.case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        self.model=api.load_model()
        source=deepcopy(api._find(self.model,"SL-1"))
        source.update(tag=self.case["host"],guid=api.global_id(self.case["host"]),storey="ground")
        source["geometry"]={"level":"ground","outline":self.case["outline"],"voids":[],"top_offset":{"value":0,"unit":"m","basis":"inferred"},"regions":[{"tag":r["tag"],"outline":r["outline"],"thickness":{"value":r["thickness"],"unit":"m","basis":"inferred"},"top_offset":{"value":0,"unit":"m","basis":"inferred"}} for r in self.case["regions"]]}
        source.pop("material",None)
        self.model["elements"].append(source)
        opening=deepcopy(next(e for e in self.model["elements"] if e["class"]=="IfcOpeningElement"))
        opening.update(tag=self.case["opening"],guid=api.global_id(self.case["opening"]),storey="ground",geometry={"slab":self.case["host"],"outline":self.case["aperture"]},attributes={})
        self.model["elements"].append(opening)
        self.host,self.opening=source,opening

    def test_native_opening_removes_registered_slab_material(self):
        before=deepcopy(self.model)
        g=api._slab_geometry(self.model,self.host)
        actual=unary_union([Polygon(p["outline"]) for p in g["parts"]])
        expected=Polygon(self.case["outline"]).difference(Polygon(self.case["aperture"]))
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)
        self.assertAlmostEqual(sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in g["parts"]),self.case["expected"]["slab_volume"])
        self.assertIn(self.opening["tag"],g["void_tags"])
        self.assertEqual(before,self.model)

    def test_opening_quantity_and_all_projections_keep_regional_depths(self):
        q=api.quantities(self.opening["tag"],self.model)
        expected=sum(Polygon(r["outline"]).intersection(Polygon(self.case["aperture"])).area*r["thickness"] for r in self.case["regions"])
        self.assertAlmostEqual(expected,self.case["expected"]["opening_volume"])
        self.assertAlmostEqual(q["volume"],expected)
        self.assertAlmostEqual(q["area"],self.case["expected"]["opening_area"])
        self.assertEqual(q["host"],self.host["tag"])
        for kind in ("axis","footprint","plan","section","elevation","3d","body"):
            result=api.represent(self.opening["tag"],kind,self.model)
            self.assertEqual(result["derived"]["host"],self.host["tag"])
        body=api.represent(self.opening["tag"],"body",self.model)
        self.assertEqual(sorted(round(p["depth"],6) for p in body["primitives"]),[.2,.3])

    def test_opening_edits_invalidate_host_cache(self):
        first=api._slab_geometry(self.model,self.host)
        self.opening["geometry"]["outline"]=[[1,1],[2,1],[2,2],[1,2]]
        second=api._slab_geometry(self.model,self.host)
        self.assertAlmostEqual(sum(api._area(p["outline"]) for p in second["parts"])-sum(api._area(p["outline"]) for p in first["parts"]),1.)

    def test_slab_and_wall_hosts_are_distinct_schema_contracts(self):
        import jsonschema
        schema=json.loads((Path(__file__).parents[1]/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
        jsonschema.Draft202012Validator(schema).validate(self.model)
        self.opening["geometry"]["wall"]="W-S"
        self.assertFalse(jsonschema.Draft202012Validator(schema).is_valid(self.model))

    def test_openings_must_be_inside_host_and_disjoint(self):
        self.opening["geometry"]["outline"]=[[3,1],[5,1],[5,2],[3,2]]
        with self.assertRaisesRegex(ValueError,"inside"):
            api._slab_geometry(self.model,self.host)
        self.opening["geometry"]["outline"]=self.case["aperture"]
        second=deepcopy(self.opening)
        second.update(tag="OP-OVERLAPPING-SLAB",guid=api.global_id("OP-OVERLAPPING-SLAB"))
        self.model["elements"].append(second)
        self.assertGreater(Polygon(self.opening["geometry"]["outline"]).intersection(Polygon(second["geometry"]["outline"])).area,0)
        with self.assertRaisesRegex(ValueError,"Slab openings must declare every shared geometric hole and no absent intersections"):
            api._slab_geometry(self.model,self.host)
