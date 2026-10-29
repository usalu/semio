"""🧪 Sparse support and non-convex surface contracts checked independently with NumPy and SciPy."""
import importlib
import json
import math
from pathlib import Path
import unittest

import numpy as np
from scipy.spatial import cKDTree

api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
analysis = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


class SparseSupportContract(unittest.TestCase):
    """🏗️ Physical column levels and sparse cell coverage replace the reference-grid abstraction."""

    def test_portable_voronoi_matches_scipy_nearest_neighbours(self):
        for case in CASES["voronoi"]:
            supports = [{"x": p[0], "y": p[1]} for p in case["supports"]]
            cells = analysis._voronoi(supports, case["outline"])
            points = np.array([[x, y] for x in np.arange(0.01, 8, 0.02) for y in np.arange(0.01, 6, 0.02)])
            nearest = cKDTree(np.asarray(case["supports"])).query(points)[1]
            for index, (cell, expected) in enumerate(zip(cells, case["areas"])):
                self.assertAlmostEqual(analysis._area(cell), expected, places=8)
                self.assertAlmostEqual(np.count_nonzero(nearest == index) * 0.0004, expected, delta=0.05)

    def test_nonconvex_cut_is_not_its_convex_hull(self):
        case = CASES["nonconvex"]
        pieces = analysis._subtract(case["outline"], case["cut"])
        self.assertAlmostEqual(sum(analysis._area(p) for p in pieces), case["remaining_area"], places=8)
        triangles = analysis._convex_parts(case["cut"])
        vertices = np.asarray(case["cut"])
        expected = abs(np.dot(vertices[:, 0], np.roll(vertices[:, 1], -1)) - np.dot(vertices[:, 1], np.roll(vertices[:, 0], -1))) / 2
        self.assertAlmostEqual(sum(analysis._area(p) for p in triangles), expected, places=8)

    def test_authored_storeys_have_separate_complete_support_partitions(self):
        model = api.load_model()
        geometry = api.building_geometry(model)
        groups = []
        for level, storey in (("first", "ground"), ("roof", "first")):
            result = analysis.tributary(model, level)
            from shapely.geometry import Point, Polygon
            slab = next(s for s in geometry["slabs"] if s["tag"] == result["slab_tag"])
            expected = {column["tag"] for column in geometry["pilotis"] if any(part["z0"] < surface["z0"] - 1e-9 and part["z1"] >= surface["z0"] - 1e-9 and Polygon(part["outline"]).intersection(Polygon(surface["outline"])).area > 1e-9
                        for part in column["parts"] for surface in slab["full_parts"])}
            self.assertEqual({p["tag"] for p in result["pilotis"]}, expected)
            self.assertTrue(result["balanced"])
            self.assertAlmostEqual(result["total_area"], result["slab_area"], places=6)
            self.assertAlmostEqual(sum(z["reaction"] for z in result["zones"]), result["total_reaction"], places=6)
            groups.append(expected)
        self.assertFalse(groups[0] & groups[1])

    def test_embedded_roof_posts_bear_on_full_structural_domain(self):
        from shapely.geometry import Polygon

        model = api.load_model()
        geometry = api.building_geometry(model)
        slab = next(item for item in geometry["slabs"] if item["tag"] == api.role("roof", model))
        full = slab["full_parts"]
        self.assertNotEqual(full, slab["parts"])
        supports = {column["tag"] for column in analysis.tributary(model, "roof")["pilotis"]}
        for tag in ("10-FIRST", "15-FIRST"):
            column = next(item for item in geometry["pilotis"] if item["tag"] == tag)
            def bears(surfaces):
                return any(part["z0"] < surface["z0"] - 1e-9 and part["z1"] >= surface["z0"] - 1e-9 and Polygon(part["outline"], part.get("holes", [])).intersection(Polygon(surface["outline"])).area > 1e-9
                           for part in column["parts"] for surface in surfaces)
            self.assertTrue(bears(full), tag)
            self.assertFalse(bears(slab["parts"]), tag)
            self.assertIn(tag, supports)

    def test_bearing_profile_intersection_matches_shapely(self):
        from shapely.geometry import Point, Polygon
        case = CASES["bearing"]
        pieces = [case["outline"]]
        for void in case["voids"]:
            pieces = [part for piece in pieces for part in analysis._subtract(piece, void)]
        slab = {"outline": case["outline"], "voids": case["voids"], "top": 3, "thickness": .2,
                "parts": [{"outline": piece, "z0":2.8, "z1":3., "thickness":.2, "region":"TEST"} for piece in pieces]}
        domain = Polygon(case["outline"], case["voids"])
        for item in case["cases"]:
            x, y = item["point"]
            outline = [[x + item["radius"] * math.cos(i * math.pi / 48), y + item["radius"] * math.sin(i * math.pi / 48)] for i in range(96)]
            column = {"x": x, "y": y, "parts": [{"outline": outline, "z0": 0, "z1": 2.8}]}
            occupied = domain.intersection(Point(x, y).buffer(item["radius"])).area > 1e-9
            self.assertEqual(occupied, item["supported"])
            self.assertEqual(analysis._bearing(column, slab), occupied)

    def test_translation_is_a_complete_first_floor_mesh_with_voids(self):
        model = api.load_model()
        result = analysis.translate_structure(model)
        self.assertAlmostEqual(sum(s["area"] for s in result["shells"]), api.quantities("SL-1", model)["area"], places=6)
        self.assertEqual(len({n["id"] for n in result["nodes"]}), len(result["nodes"]))
        nodes = {n["id"] for n in result["nodes"]}
        self.assertTrue(all(set(s["nodes"]) <= nodes for s in result["shells"]))
        self.assertTrue(all(m["length"] > 0 for m in result["members"] if m["kind"] == "curve"))


if __name__ == "__main__":
    unittest.main()
