"""🏬 Portable podium/profile and historic signage contracts with NumPy geometry oracles."""

import importlib.util
import json
import sys
import unittest
from collections import Counter
from dataclasses import replace
from pathlib import Path

import numpy as np

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
CASES = json.loads((ROOT / "🧪️cases/🏬️podium/🔣️.json").read_text(encoding="utf-8"))
RAW = json.loads((ROOT / "🔣️.json").read_text(encoding="utf-8"))
spec = importlib.util.spec_from_file_location("nakagin_podium", ROOT / "🐍️.py")
M = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = M
spec.loader.exec_module(M)


def volume(solid):
    """📐 NumPy divergence-theorem oracle is independent from runtime mesh construction."""
    vertices = np.asarray(solid.vertices)
    return sum(float(np.dot(vertices[face[0]], np.cross(vertices[face[i]], vertices[face[i + 1]]))) / 6 for face in solid.faces for i in range(1, len(face) - 1))


def bounds(part):
    """📏 Derive geometric bounds from actual authored vertices."""
    points = np.asarray(part.solid.vertices)
    return points.min(axis=0), points.max(axis=0)


class PodiumContracts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.parts = M.podium_assembly().parts

    def test_every_part_is_closed_with_positive_independent_volume(self):
        for part in self.parts:
            edges = Counter(tuple(sorted((a, b))) for face in part.solid.faces for a, b in zip(face, face[1:] + face[:1]))
            self.assertTrue(all(count == 2 for count in edges.values()), part.key)
            self.assertGreater(volume(part.solid), CASES["minimum_closed_volume"], part.key)

    def test_window_strip_matches_portable_profile_and_replaces_full_glass(self):
        low, high = (q["value"] for q in RAW["podium"]["levels"][1:])
        expected = low + (high - low) * np.asarray(CASES["window_band"])
        glazing = [part for part in self.parts if part.role == "podium_glazing"]
        self.assertTrue(glazing)
        for part in glazing:
            a, b = bounds(part)
            np.testing.assert_allclose((a[2], b[2]), expected, atol=1e-8)
            self.assertLessEqual((b[2] - a[2]) / (high - low), CASES["maximum_glazing_fraction"])
        self.assertTrue(any(p.role == "podium_cladding" for p in self.parts))
        self.assertTrue(any(p.role == "podium_panel_joint" for p in self.parts))

    def test_fascia_has_real_curved_return_vertices(self):
        self.assertEqual(sum(p.role == "podium_fascia_support" for p in self.parts), CASES["fascia_supports_per_edge"] * len(RAW["podium"]["footprint"]))
        for part in (p for p in self.parts if p.role == "podium_fascia"):
            vertices = np.asarray(part.solid.vertices)
            self.assertGreater(len(np.unique(np.round(vertices[:, 2], 8))), 4)
            self.assertGreater(np.ptp(vertices[:, 2]), CASES["fascia_radius"] - 1e-8)
            edge = int(part.key.split("-")[3])
            polygon = np.array([[q["value"] for q in point] for point in RAW["podium"]["footprint"]])
            a, b = polygon[edge], polygon[(edge + 1) % len(polygon)]
            normal = np.array((b[1] - a[1], a[0] - b[0])) / np.linalg.norm(b - a)
            radial = (vertices[:, :2] - a) @ normal
            np.testing.assert_allclose((radial.min(), radial.max()), CASES["fascia_radial_bounds"], atol=1e-8)

    def test_programs_are_source_grounded_and_room_footprints_stay_inside_podium(self):
        rooms = RAW["podium"]["architecture"]["rooms"]
        self.assertEqual(dict(Counter(room["program"] for room in rooms)), CASES["room_programs"])
        polygon = np.array([[q["value"] for q in point] for point in RAW["podium"]["footprint"]])
        for room in rooms:
            x0, x1, y0, y1 = (q["value"] for q in room["bounds"])
            points = np.array(((x0, y0), (x1, y0), (x1, y1), (x0, y1)))
            outline = polygon if room["level"] else polygon.mean(axis=0) + .88 * (polygon - polygon.mean(axis=0))
            a, b = outline, np.roll(outline, -1, axis=0)
            for point in points:
                crossing = (a[:, 1] > point[1]) != (b[:, 1] > point[1])
                indices = np.flatnonzero(crossing)
                intersections = a[indices, 0] + (point[1] - a[indices, 1]) * (b[indices, 0] - a[indices, 0]) / (b[indices, 1] - a[indices, 1])
                self.assertEqual(int(np.count_nonzero(intersections > point[0])) % 2, 1, room["key"])
            self.assertEqual(room["layout_basis"], "assumed_from_program_without_recovered_room_plan")
        for role in ("reception_desk", "hall_bench", "cafe_table", "cafe_chair", "cafe_counter", "office_desk", "office_chair", "office_storage"):
            self.assertTrue(any(part.role == role for part in self.parts), role)

    def test_south_core_approaches_remain_clear(self):
        for x, y in CASES["clear_ground_approaches"]:
            query = np.array(((x - .5, y - .3, .05), (x + .5, y + .3, 2.15)))
            for part in self.parts:
                if part.role in ("floor", "podium_finish", "roof"):
                    continue
                a, b = bounds(part)
                self.assertFalse(np.all(np.minimum(query[1], b) - np.maximum(query[0], a) > 1e-7), (part.key, x, y))

    def test_fittings_leave_each_room_entrance_clear(self):
        rooms = {room["key"]: room for room in RAW["podium"]["architecture"]["rooms"]}
        for case in CASES["room_door_approaches"]:
            room = rooms[case["room"]]
            z = RAW["podium"]["levels"][room["level"]]["value"]
            x0, x1, y0, y1 = case["bounds"]
            query = np.array(((x0, y0, z + .04), (x1, y1, z + 2.1)))
            for part in self.parts:
                if not part.key.startswith(case["room"]) or part.role in ("podium_finish", "podium_partition"):
                    continue
                a, b = bounds(part)
                self.assertFalse(np.all(np.minimum(query[1], b) - np.maximum(query[0], a) > 1e-7), (case["room"], part.key))

    def test_every_room_fitting_has_a_connected_support_path(self):
        for role, count in CASES["fitout_roles"].items():
            self.assertEqual(sum(part.role == role for part in self.parts), count)
        for room in RAW["podium"]["architecture"]["rooms"]:
            parts = [part for part in self.parts if part.solid.meta.get("room") == room["key"]]
            boxes = np.array([bounds(part) for part in parts])
            reached = {i for i, part in enumerate(parts) if part.role == "podium_finish"}
            while True:
                old = set(reached)
                for i in tuple(reached):
                    overlap = np.minimum(boxes[:, 1], boxes[i, 1]) - np.maximum(boxes[:, 0], boxes[i, 0])
                    adjacent = np.all(overlap >= -CASES["support_contact_tolerance"], axis=1) & (np.sum(overlap > CASES["support_contact_tolerance"], axis=1) >= 2)
                    reached.update(np.flatnonzero(adjacent).tolist())
                if reached == old:
                    break
            self.assertEqual(len(reached), len(parts), [part.key for i, part in enumerate(parts) if i not in reached])

    def test_sign_mounts_and_strokes_fit_live_oblique_roof_faces(self):
        original = next(t for t in M.towers() if t.key == CASES["sign"]["tower"])
        for height_scale, width_scale in CASES["sign"]["dimension_overrides"]:
            tower = replace(original, capital_height=original.capital_height * height_scale, core_side=original.core_side * width_scale)
            half = tower.core_side / 2 + tower.capital_overhang
            roof = np.asarray(M.capital_solid(tower).vertices)
            for part in (part for part in M.capital_assembly(tower).parts if part.role in ("historic_sign", "sign_mount")):
                points = np.asarray(part.solid.vertices)
                face = part.solid.meta.get("face", part.key.split("-")[1])
                horizontal = points[:, 0] - tower.center[0] if face == "north" else points[:, 1] - tower.center[1]
                self.assertTrue(np.all(np.abs(horizontal) <= half + 1e-8), (part.key, height_scale, width_scale))
                start, end = (roof[7, 2], roof[4, 2]) if face == "north" else (roof[6, 2], roof[7, 2])
                roof_height = start + (horizontal + half) / (2 * half) * (end - start)
                self.assertTrue(np.all(points[:, 2] <= roof_height + 1e-8), (part.key, height_scale, width_scale))

    def test_parapet_coping_has_its_own_elevation_above_the_flat_roof(self):
        roof = RAW["podium"]["levels"][-1]["value"]
        top = max(point[2] for part in self.parts for point in part.solid.vertices)
        self.assertAlmostEqual(top, roof + CASES["roof_edge_height"])
        slabs = [part for part in self.parts if part.role == "roof"]
        self.assertAlmostEqual(max(point[2] for part in slabs for point in part.solid.vertices), roof)
        for role in ("podium_coping", "podium_parapet"):
            parts = [part for part in self.parts if part.role == role]
            self.assertEqual(len(parts), len(RAW["podium"]["footprint"]))
            for part in parts:
                points = np.asarray(part.solid.vertices)
                self.assertGreaterEqual(points[:, 2].min(), roof - 1e-8)
                self.assertLessEqual(points[:, 2].max(), roof + CASES["roof_edge_height"] + 1e-8)
        ceilings = [part for part in self.parts if part.role == "podium_ceiling"]
        self.assertLess(max(point[2] for part in ceilings for point in part.solid.vertices), roof)

    def test_historic_sign_characters_faces_and_positive_solids(self):
        expected = CASES["sign"]
        for tower in M.towers():
            parts = M.capital_assembly(tower).parts
            glyphs = [part for part in parts if part.role == "historic_sign"]
            if tower.key != expected["tower"]:
                self.assertFalse(glyphs)
                continue
            self.assertEqual({part.solid.meta["character"] for part in glyphs}, set(expected["text"]))
            self.assertEqual({part.solid.meta["face"] for part in glyphs}, set(expected["faces"]))
            self.assertEqual(len([p for p in parts if p.role == "sign_mount"]), 2)
            for part in glyphs:
                self.assertGreater(volume(part.solid), CASES["minimum_closed_volume"])
                low, high = bounds(part)
                self.assertGreater(low[2], M.capital_base(tower))
                self.assertLess(high[2], M.capital_base(tower) + tower.capital_height)


if __name__ == "__main__":
    unittest.main(verbosity=2)
