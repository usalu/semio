"""🔺 Tapered foundation solids agree with independent hull and slice geometry."""

import importlib
import json
from pathlib import Path
import unittest

import numpy as np
from scipy.integrate import quad
from scipy.spatial import ConvexHull
from shapely.geometry import LineString, Polygon
from shapely.ops import unary_union

api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class FrustumSolidContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.cases = json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))

    def test_tapered_volumes_and_vertices_match_independent_hulls(self):
        for case in self.cases["cases"]:
            part = case["part"]
            vertices = np.asarray(api.part_vertices(part))
            self.assertAlmostEqual(ConvexHull(vertices).volume,case["volume"])
            self.assertAlmostEqual(api._occupied_volume([part]),case["volume"],places=9)
            mid = (np.asarray(part["outline"])+np.asarray(part["top_outline"]))/2
            np.testing.assert_allclose(api.part_outline_at(part,(part["z0"]+part["z1"])/2),mid)
            self.assertTrue(Polygon(api.part_footprint(part)).covers(Polygon(part["outline"])))

    def test_tapered_union_matches_independent_integrated_slice_union(self):
        part = self.cases["cases"][0]["part"]
        box = self.cases["union_box"]
        def area(z):
            t=(z-part["z0"])/(part["z1"]-part["z0"])
            outline=(1-t)*np.asarray(part["outline"])+t*np.asarray(part["top_outline"])
            return unary_union([Polygon(outline),Polygon(box["outline"])]).area
        expected = quad(area,part["z0"],part["z1"],epsabs=1e-11,points=[-1.1,-1.05],limit=200)[0]
        self.assertAlmostEqual(api._occupied_volume([part,box]),expected,places=8)

    def test_inclined_section_has_true_sides(self):
        part = self.cases["cases"][1]["part"]
        actual = Polygon(api.part_section(part,.5))
        expected = Polygon([[0,0],[2,0],[1.5,.6],[.5,.6]])
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)
        self.assertEqual(api.part_section(part,2),[])
        self.assertLess(Polygon(api.part_elevation(part)).symmetric_difference(expected).area,1e-10)

    def test_every_representation_preserves_the_taper(self):
        case = self.cases["cases"][1]
        source = api.load_model()
        obj = api._find(source,"FIX-HALL-BASIN")
        obj["geometry"] = {"kind":"foundation","base_level":"ground","parts":[case["part"]]}
        solid = api.represent(obj["tag"],"3d",source)["primitives"][0]
        self.assertEqual(solid["kind"],"loft")
        np.testing.assert_allclose(solid["bottom"],case["part"]["outline"])
        np.testing.assert_allclose(solid["top"],case["part"]["top_outline"])
        body = api.represent(obj["tag"],"body",source)["primitives"][0]
        self.assertEqual(body["kind"],"extruded_area_tapered")
        expected = Polygon([[0,0],[2,0],[1.5,.6],[.5,.6]])
        for view in ("section","elevation"):
            shapes = api.represent(obj["tag"],view,source)["primitives"]
            self.assertLess(unary_union([Polygon(p["points"]) for p in shapes]).symmetric_difference(expected).area,1e-10)
        self.assertAlmostEqual(api.quantities(obj["tag"],source)["volume"],case["volume"])

    def test_invalid_loft_correspondence_is_rejected(self):
        for case in self.cases["invalid"]:
            with self.subTest(case=case["name"]),self.assertRaises(ValueError):
                api.part_vertices(case["part"])


if __name__ == "__main__":
    unittest.main()
       