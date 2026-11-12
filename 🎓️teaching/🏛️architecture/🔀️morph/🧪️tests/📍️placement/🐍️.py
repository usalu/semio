"""🧪 Tests of the placement module: language-agnostic JSON cases, a shapely.affinity oracle, an independent NumPy oracle, and the interpolation, degenerate and nested-frame rules."""

from __future__ import annotations

import importlib
import json
import pathlib
import unittest

import numpy as np

placement = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📍️placement.🐍️")
shapely_geometry = importlib.import_module("shapely.geometry")
shapely_affinity = importlib.import_module("shapely.affinity")

CASES = json.loads((pathlib.Path(__file__).parent / "🧫️cases" / "🔣️.json").read_text(encoding="utf-8"))["cases"]
ANCHOR_AXIS = {"center": (0.5, 0.5), "bottom": (0.5, 0.0), "top": (0.5, 1.0), "left": (0.0, 0.5), "right": (1.0, 0.5)}


def bounds_of(spec):
    """📦 Bounds of a JSON case endpoint."""
    return placement.Bounds(tuple(spec["lo"]), tuple(spec["hi"]), tuple(spec["origin"]) if "origin" in spec else None)


def frames_of(case):
    """🗺 Frames of a JSON case."""
    return {k: placement.Frame(tuple(v["scale"]), tuple(v["translation"]), v["parent"]) for k, v in case.get("frames", {}).items()}


def run(case):
    """🎯 Affine of one JSON case."""
    return placement.affine(bounds_of(case["source"]), bounds_of(case["target"]), placement.Placement(**case["placement"]), frames=frames_of(case))


def anchor_in(bounds, anchor):
    """⚓ Independent anchor point of (minx, miny, maxx, maxy) bounds."""
    fx, fy = ANCHOR_AXIS[anchor]
    return np.array([bounds[0] + fx * (bounds[2] - bounds[0]), bounds[1] + fy * (bounds[3] - bounds[1])])


class CaseTests(unittest.TestCase):
    """📋 Every JSON case matches its hand-derived expectation."""

    def test_cases(self):
        for case in CASES:
            with self.subTest(case["name"]):
                if "error" in case:
                    with self.assertRaises(placement.PlacementError) as caught:
                        run(case)
                    self.assertIn(case["error"], str(caught.exception))
                    continue
                result = run(case)
                np.testing.assert_allclose(result.scale, case["expect"]["scale"], atol=1e-12)
                np.testing.assert_allclose(result.translation, case["expect"]["translation"], atol=1e-12)

    def test_shapely_places_anchor_on_anchor_and_respects_fit(self):
        for case in CASES:
            if "error" in case or case["placement"]["space"] != "scene":
                continue
            with self.subTest(case["name"]):
                a = run(case)
                s, t, p = case["source"], case["target"], case["placement"]
                source = shapely_geometry.box(*s["lo"], *s["hi"])
                moved = shapely_affinity.affine_transform(source, [a.scale[0], 0, 0, a.scale[1], a.translation[0], a.translation[1]])
                if p["anchor"] != "origin":
                    np.testing.assert_allclose(anchor_in(moved.bounds, p["anchor"]), anchor_in((*t["lo"], *t["hi"]), p["anchor"]), atol=1e-9)
                if p["fit"] == "contain" and p["anchor"] == "center" and source.bounds[2] > source.bounds[0] and source.bounds[3] > source.bounds[1] and t["hi"][0] > t["lo"][0] and t["hi"][1] > t["lo"][1]:
                    mx0, my0, mx1, my1 = moved.bounds
                    self.assertGreaterEqual(mx0, t["lo"][0] - 1e-9)
                    self.assertLessEqual(mx1, t["hi"][0] + 1e-9)
                    self.assertGreaterEqual(my0, t["lo"][1] - 1e-9)
                    self.assertLessEqual(my1, t["hi"][1] + 1e-9)


class RandomisedOracleTests(unittest.TestCase):
    """🎲 Seeded random bounds against an independent NumPy computation and shapely."""

    def test_contain_uniform_touches_target_on_the_tight_axis(self):
        rng = np.random.default_rng(11)
        for _ in range(200):
            source = np.sort(rng.uniform(-9, 9, (2, 2)), axis=0) + [[0, 0], [0.1, 0.1]]
            target = np.sort(rng.uniform(-9, 9, (2, 2)), axis=0) + [[0, 0], [0.1, 0.1]]
            a = placement.affine(placement.Bounds.of(source), placement.Bounds.of(target), placement.Placement(anchor="center", fit="contain", uniform=True))
            moved = shapely_affinity.affine_transform(shapely_geometry.box(*source[0], *source[1]), [a.scale[0], 0, 0, a.scale[1], *a.translation])
            ratio = min((target[1] - target[0]) / (source[1] - source[0]))
            self.assertAlmostEqual(a.scale[0], ratio, places=9)
            self.assertAlmostEqual(a.scale[1], ratio, places=9)
            np.testing.assert_allclose(np.array(moved.bounds).reshape(2, 2).mean(0), target.mean(0), atol=1e-9)
            width, height = moved.bounds[2] - moved.bounds[0], moved.bounds[3] - moved.bounds[1]
            self.assertLessEqual(max(width - (target[1, 0] - target[0, 0]), height - (target[1, 1] - target[0, 1])), 1e-9)

    def test_bounds_scale_with_parent_frame_in_scene_space(self):
        rng = np.random.default_rng(5)
        for _ in range(100):
            f = rng.uniform(0.3, 3, 2)
            u = rng.uniform(-5, 5, 2)
            local_s = np.sort(rng.uniform(-3, 3, (2, 2)), axis=0) + [[0, 0], [0.1, 0.1]]
            local_t = np.sort(rng.uniform(-3, 3, (2, 2)), axis=0) + [[0, 0], [0.1, 0.1]]
            frames = {"e": placement.Frame(tuple(f), tuple(u))}
            to_scene = lambda b: b * f + u
            nested = placement.affine(placement.Bounds.of(to_scene(local_s)), placement.Bounds.of(to_scene(local_t)), placement.Placement(space="entity:e", anchor="bottom", fit="contain", uniform=True), frames=frames)
            direct = placement.affine(placement.Bounds.of(local_s), placement.Bounds.of(local_t), placement.Placement(anchor="bottom", fit="contain", uniform=True))
            conjugated = placement.compose(placement.Affine(tuple(f), tuple(u)), placement.compose(direct, placement.inverse(placement.Affine(tuple(f), tuple(u)))))
            np.testing.assert_allclose(nested.scale, conjugated.scale, atol=1e-9)
            np.testing.assert_allclose(nested.translation, conjugated.translation, atol=1e-9)


class InterpolationTests(unittest.TestCase):
    """⏱ `at` and the affine algebra."""

    def setUp(self):
        self.a = placement.Affine((2.0, 3.0), (4.0, -6.0))

    def test_exact_endpoints(self):
        self.assertEqual(placement.at(self.a, 0.0), placement.IDENTITY)
        self.assertEqual(placement.at(self.a, 1.0), self.a)

    def test_midpoint_is_linear_in_scale_and_translation(self):
        mid = placement.at(self.a, 0.5)
        np.testing.assert_allclose(mid.scale, (1.5, 2.0))
        np.testing.assert_allclose(mid.translation, (2.0, -3.0))

    def test_overshoot_and_undershoot_extrapolate(self):
        np.testing.assert_allclose(placement.at(self.a, 1.25).scale, (2.25, 3.5))
        np.testing.assert_allclose(placement.at(self.a, -0.5).translation, (-2.0, 3.0))

    def test_apply_leaves_z_and_input_untouched(self):
        points = np.array([[1.0, 1.0, 7.0]])
        moved = placement.apply(self.a, points)
        np.testing.assert_allclose(moved, [[6.0, -3.0, 7.0]])
        np.testing.assert_allclose(points, [[1.0, 1.0, 7.0]])

    def test_inverse_round_trip(self):
        points = np.random.default_rng(1).normal(size=(5, 3))
        np.testing.assert_allclose(placement.apply(placement.inverse(self.a), placement.apply(self.a, points)), points, atol=1e-12)

    def test_union_and_bounds_of_array(self):
        union = placement.union([placement.Bounds.of([[0, 0, 0], [1, 1, 0]]), placement.Bounds.of([[3, -2, 0], [4, 0, 0]])])
        self.assertEqual((union.lo, union.hi), ((0.0, -2.0), (4.0, 1.0)))
        with self.assertRaises(placement.PlacementError):
            placement.union([])
        with self.assertRaises(placement.PlacementError):
            placement.Bounds.of(np.zeros((3, 2)))


if __name__ == "__main__":
    unittest.main()
