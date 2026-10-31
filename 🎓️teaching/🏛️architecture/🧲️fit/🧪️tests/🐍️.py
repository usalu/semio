"""🧪 Tests of the building fit: the scale and position from model numbers keep the whole Nakagin drawing inside a bounded area, validated against rhino3dm and the raw model JSON."""

from __future__ import annotations

import importlib
import json
import math
import sys
import unittest
from pathlib import Path

import numpy as np
import rhino3dm as rh

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = next(parent for parent in HERE.parents if (parent / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
CASES = json.loads((HERE / "🔣️.cases.json").read_text(encoding="utf-8"))
MODEL = json.loads((HERE.parents[1] / "🏢️nakagin/🔣️.json").read_text(encoding="utf-8"))
TOL = CASES["tolerance"]
fit = importlib.import_module("🎓️teaching.🏛️architecture.🧲️fit.🐍️")
drawing = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
CENTRE = (1.0, -2.0, 0.0)


def view_plane(origin, azimuth: float, elevation: float) -> rh.Plane:
    """🔭 The axonometric picture plane: X to the right of the picture, Y up the picture."""
    phi, eps = math.radians(azimuth), math.radians(elevation)
    right = rh.Vector3d(-math.sin(phi), math.cos(phi), 0)
    up = rh.Vector3d(-math.sin(eps) * math.cos(phi), -math.sin(eps) * math.sin(phi), math.cos(eps))
    return rh.Plane(rh.Point3d(*origin), right, up)


def screen(points, origin, view, at, scale) -> np.ndarray:
    """📍 Picture coordinates of world points, projected and scaled with rhino3dm."""
    to_view = rh.Transform.PlaneToPlane(view_plane(origin, view["azimuth"], view["elevation"]), rh.Plane.WorldXY())
    out = []
    for point in points:
        p = rh.Point3d(*map(float, point))
        p.Transform(to_view)
        out.append((at[0] + scale * p.X, at[1] + scale * p.Y))
    return np.array(out)


def middle_of(points) -> tuple:
    """🎯 Centre of the world bounding box by rhino3dm."""
    cloud = rh.PointCloud()
    for point in points:
        cloud.Add(rh.Point3d(*map(float, point)))
    box = cloud.GetBoundingBox()
    return ((box.Min.X + box.Max.X) / 2, (box.Min.Y + box.Max.Y) / 2, (box.Min.Z + box.Max.Z) / 2)


def model_points(towers, podium: bool, capital: bool) -> list:
    """🏛️ Core, capital, podium and capsule-centre points straight from the model JSON."""
    out = []
    core, over = MODEL["core"]["side"]["value"] / 2, MODEL["capital"]["overhang"]["value"]
    for index in towers:
        tower = MODEL["towers"][index]
        cx, cy = tower["center"]["x"]["value"], tower["center"]["y"]["value"]
        top, head = tower["roof_elevation"]["value"], tower["capital_height"]["value"]
        out += [(cx + sx * core, cy + sy * core, z) for sx in (-1, 1) for sy in (-1, 1) for z in (0.0, top)]
        if capital:
            roof = [q["value"] for q in MODEL["capital"]["roof_corners"]]
            s = core + over
            out += [(cx + sx * s, cy + sy * s, top + h * head / max(roof)) for (sx, sy), h in zip(((1, 1), (1, -1), (-1, -1), (-1, 1)), roof)]
            out += [(cx + sx * s, cy + sy * s, top) for sx in (-1, 1) for sy in (-1, 1)]
        out += [tuple(v["value"] for v in item["center"]) for floor in tower["floors"] for group in floor["groups"] for item in group["items"]]
    if podium:
        height = MODEL["podium"]["levels"][-1]["value"]
        out += [(*(v["value"] for v in corner), z) for corner in MODEL["podium"]["footprint"] for z in (0.0, height)]
    return out


class TestBuildingFit(unittest.TestCase):
    def test_axon_case_is_the_drawing_view(self):
        view = CASES["views"][0]
        self.assertEqual(drawing.VIEWS[view["name"]], (view["azimuth"], view["elevation"]))

    def test_every_area_is_filled_tightly_and_centred_like_rhino3dm(self):
        view = CASES["views"][0]
        for case in CASES["areas"]:
            parts = {**case["parts"], **({"towers": tuple(case["parts"]["towers"])} if "towers" in case["parts"] else {})}
            result = fit.building_fit(case["width"], case["height"], centre=CENTRE, top=case["top"], bottom=case["bottom"], margin=case["margin"], **parts)
            points = fit.building_points(**parts)
            flat = screen(points, middle_of(points), view, result["at"], result["scale"])
            low, high = flat.min(axis=0), flat.max(axis=0)
            free_low = np.array([CENTRE[0] - case["width"] / 2 + case["margin"], CENTRE[1] - case["height"] / 2 + case["bottom"] + case["margin"]])
            free_high = np.array([CENTRE[0] + case["width"] / 2 - case["margin"], CENTRE[1] + case["height"] / 2 - case["top"] - case["margin"]])
            with self.subTest(case["name"]):
                self.assertTrue(np.all(low >= free_low - TOL) and np.all(high <= free_high + TOL))
                self.assertAlmostEqual(float(min(np.min(low - free_low), np.min(free_high - high))), 0.0, delta=1e-7)
                self.assertTrue(np.allclose((low + high) / 2, (free_low + free_high) / 2, atol=1e-7))

    def test_core_capital_podium_and_capsule_centres_of_the_model_json_stay_inside(self):
        view = CASES["views"][0]
        for case in CASES["areas"]:
            towers = tuple(case["parts"].get("towers", (0, 1)))
            podium, capital = case["parts"].get("podium", True), case["parts"].get("capital", True)
            parts = {"towers": towers, "podium": podium, "capital": capital}
            result = fit.building_fit(case["width"], case["height"], centre=CENTRE, top=case["top"], bottom=case["bottom"], margin=case["margin"], **parts)
            points = fit.building_points(**parts)
            flat = screen(model_points(towers, podium, capital), middle_of(points), view, result["at"], result["scale"])
            free_low = np.array([CENTRE[0] - case["width"] / 2 + case["margin"], CENTRE[1] - case["height"] / 2 + case["bottom"] + case["margin"]])
            free_high = np.array([CENTRE[0] + case["width"] / 2 - case["margin"], CENTRE[1] + case["height"] / 2 - case["top"] - case["margin"]])
            with self.subTest(case["name"]):
                self.assertTrue(np.all(flat.min(axis=0) >= free_low - 1e-6) and np.all(flat.max(axis=0) <= free_high + 1e-6))

    def test_fewer_parts_never_shrink_the_drawing(self):
        whole = fit.building_fit(4.4, 3.64, top=0.45)["scale"]
        shaft = fit.building_fit(4.4, 3.64, top=0.45, towers=(0,), podium=False, capital=False)["scale"]
        self.assertGreater(shaft, whole)

    def test_elevation_views_fit_too(self):
        for face in ("north", "south", "east", "west"):
            result = fit.building_fit(5.0, 4.0, view=face)
            self.assertGreater(result["scale"], 0.0)
            self.assertEqual(result["at"].shape, (3,))


if __name__ == "__main__":
    unittest.main()
