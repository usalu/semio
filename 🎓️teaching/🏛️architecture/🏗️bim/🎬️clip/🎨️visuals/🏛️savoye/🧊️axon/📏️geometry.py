"""📏 Painter primitives preserve registered profiles and regional slab solids."""

import importlib
import json
from pathlib import Path
import unittest

import numpy as np
from shapely.geometry import Polygon


class AxonPhysicalGeometryContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.cases = json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        cls.production = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")

    def test_rectangular_column_is_never_replaced_by_circle(self):
        case = self.cases["column"]
        pieces = self.production._column_prims(case, None, 3.17, 6.6)
        self.assertEqual(len(pieces), 1)
        face = next(face for face in pieces[0].faces if face[2][2] > .9)
        polygon = Polygon(np.asarray(face[1][0])[:, :2])
        self.assertTrue(polygon.equals_exact(Polygon(case["parts"][0]["outline"]), 1e-12))
        self.assertAlmostEqual(polygon.area * (case["parts"][0]["z1"] - case["parts"][0]["z0"]), self.cases["expected_column_volume"])

    def test_mixed_slab_keeps_each_regional_depth(self):
        case = self.cases["slab"]
        pieces = self.production._slab_prims(case, None, 2)
        self.assertEqual(len(pieces), 2)
        volume = sum(Polygon(np.asarray(next(face for face in p.faces if face[2][2] > .9)[1][0])[:, :2]).area * (source["z1"] - source["z0"]) for p, source in zip(pieces, case["parts"]))
        self.assertAlmostEqual(volume, self.cases["expected_slab_volume"])
        ranges = [(min(v[2] for face in p.faces for ring in face[1] for v in ring), max(v[2] for face in p.faces for ring in face[1] for v in ring)) for p in pieces]
        np.testing.assert_allclose(ranges, [[2.91, 3.14], [2.91, 3.08]])

    def test_buried_columns_split_at_the_terrain_occlusion_plane(self):
        pieces = self.production._column_prims(self.cases["buried_column"],None,3.17,6.6)
        self.assertEqual(len(pieces),2)
        bands = [[min(v[2] for face in p.faces for ring in face[1] for v in ring), max(v[2] for face in p.faces for ring in face[1] for v in ring)] for p in pieces]
        np.testing.assert_allclose(bands,self.cases["expected_column_bands"])
        entries = [(0,stage) for stage in self.cases["ground_stages"]]
        self.assertEqual(self.production._ground_index(entries),self.cases["ground_insert_index"])


    def test_native_polygon_caps_sides_and_volumes_match_independent_oracles(self):
        import importlib
        from scipy.spatial import ConvexHull

        production = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
        for case in self.cases["prisms"]:
            with self.subTest(case=case["name"]):
                faces = production._box(case["outline"], case["z0"], case["z1"], "wall")
                self.assertEqual(len(faces), len(case["outline"]) + 2)
                volume = 0.0
                for _, rings, normal, role in faces:
                    for ring in rings:
                        winding = sum((np.cross(ring[i], ring[(i + 1) % len(ring)]) for i in range(len(ring))), np.zeros(3))
                        orientation = 1 if np.dot(winding, normal) >= 0 else -1
                        volume += orientation * sum(float(np.dot(ring[0], np.cross(ring[i], ring[i + 1]))) / 6 for i in range(1, len(ring) - 1))
                expected = Polygon(case["outline"]).area * (case["z1"] - case["z0"])
                self.assertAlmostEqual(abs(volume), expected)
                self.assertAlmostEqual(expected, case["volume"])
                if case["convex"]:
                    vertices = np.array([[*p, z] for z in (case["z0"], case["z1"]) for p in case["outline"]])
                    self.assertAlmostEqual(expected, ConvexHull(vertices).volume)


    def test_public_geometry_coordinates_are_owned_builtin_containers(self):
        import importlib

        prefix = "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals."
        measure = importlib.import_module(prefix + "📐️drafting.📏️geometry")
        physical = importlib.import_module(prefix + "🏛️savoye.🧱️core.📐️geometry")
        production = importlib.import_module(prefix + "🏛️savoye.🧊️axon.🧩️primitives")
        def owned(value):
            if type(value) in (tuple, list):
                for child in value:
                    owned(child)
            else:
                self.assertIn(type(value), (float, int, str, bool, type(None)))
        case = self.cases["native_containers"]
        for point in case["points"]:
            actual = measure.P(np.asarray(point["input"]))
            self.assertEqual(actual, tuple(float(v) for v in point["output"]))
            owned(actual)
        wall = case["wall"]
        frame = physical.wall_frame(wall, {})
        self.assertEqual(frame, (tuple(wall["expected_origin"]), tuple(wall["expected_along"]), tuple(wall["expected_normal"]), wall["expected_length"]))
        owned(frame)
        owned(production.UP)
        ring = np.array([[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]])
        faces = [("poly", [ring], np.array([0., 0., 1.]), "wall"), ("cyl", np.array([0., 0., 0., 1., .2]), None, "post"), ("line", ring, np.float64(1.4), "foot", .1), ("segs", [(ring[0], ring[1])], 1.2, "metal")]
        primitive = production.Prim("native", {}, 0, np.array([.5, .5]), faces)
        self.assertEqual([f[0] for f in primitive.faces], case["face_kinds"])
        owned(primitive.faces)
        owned(primitive.foot)


if __name__ == "__main__":
    unittest.main()
