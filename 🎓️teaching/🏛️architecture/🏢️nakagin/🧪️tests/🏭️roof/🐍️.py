"""🏭 Portable roof sheet, standing-seam and flashing contracts with independent NumPy/rhino3dm oracles."""

import importlib.util
import json
import sys
import unittest
from collections import Counter
from dataclasses import replace
from pathlib import Path

import numpy as np
import rhino3dm as rh

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
RAW = json.loads((ROOT / "🔣️.json").read_text(encoding="utf-8"))
CASES = json.loads((ROOT / "🧪️cases/🏭️roof/🔣️.json").read_text(encoding="utf-8"))
spec = importlib.util.spec_from_file_location("nakagin_roof", ROOT / "🐍️.py")
M = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = M
spec.loader.exec_module(M)


def volume(solid):
    """📦 NumPy divergence-theorem volume oracle for indexed polyhedra."""
    v = np.asarray(solid.vertices)
    return sum(float(np.dot(v[face[0]], np.cross(v[face[i]], v[face[i + 1]]))) / 6 for face in solid.faces for i in range(1, len(face) - 1))


def variants():
    """🎛 Apply independent portable width/height controls without changing the shared data."""
    for tower in M.towers():
        for height, width in CASES["dimension_overrides"]:
            yield replace(tower, capital_height=tower.capital_height * height, core_side=tower.core_side * width)


class RoofContracts(unittest.TestCase):
    def test_internal_frame_fits_behind_every_closed_wall_and_roof_facet(self):
        clearance=CASES["framing"]["sheet_clearance"]
        sheet=RAW["capital"]["cladding"]["sheet_thickness"]["value"]
        for tower in variants():
            envelope=np.asarray(M.capital_solid(tower).vertices)
            frames=[p for p in M.capital_assembly(tower).parts if p.role in CASES["framing"]["roles"]]
            self.assertGreaterEqual(sum(p.key.startswith("capital-post-") for p in frames),CASES["framing"]["minimum_posts"])
            low,high=envelope[:4,:2].min(axis=0),envelope[:4,:2].max(axis=0)
            for part in frames:
                vertices=np.asarray(part.solid.vertices)
                self.assertTrue(np.all(vertices[:,:2] >= low+sheet+clearance-CASES["tolerance"]),(tower.key,part.key,"wall-low"))
                self.assertTrue(np.all(vertices[:,:2] <= high-sheet-clearance+CASES["tolerance"]),(tower.key,part.key,"wall-high"))
                self.assertGreaterEqual(float(vertices[:,2].min()),M.capital_base(tower)-CASES["tolerance"])
                for point in vertices:
                    found=False
                    for indices in ((4,7,6),(4,6,5)):
                        triangle=envelope[list(indices)]
                        weights=np.linalg.solve(np.vstack((triangle[:,:2].T,np.ones(3))),np.r_[point[:2],1])
                        if np.all(weights >= -CASES["tolerance"]):
                            normal=np.cross(triangle[1]-triangle[0],triangle[2]-triangle[0])
                            normal=normal/np.linalg.norm(normal)
                            if normal[2]<0:
                                normal=-normal
                            self.assertLessEqual(float((point-triangle[0])@normal),-sheet-clearance+CASES["tolerance"],(tower.key,part.key,"roof"))
                            found=True
                            break
                    self.assertTrue(found,(tower.key,part.key,"roof-domain"))

    def test_every_actual_surface_is_panelized_with_seams_and_edge_flashings(self):
        for tower in M.towers():
            parts = M.capital_assembly(tower).parts
            for role in CASES["roles"]:
                selected = [p for p in parts if p.role == role]
                self.assertTrue(selected, (tower.key, role))
                self.assertEqual({p.solid.meta["surface"] for p in selected}, set(CASES["surfaces"]))
            self.assertGreater(sum(p.role == "capital" for p in parts), 40)

    def test_every_panel_seam_and_flashing_is_closed_and_has_positive_volume(self):
        for tower in variants():
            for part in M.capital_assembly(tower).parts:
                if part.role not in CASES["roles"]:
                    continue
                edges = Counter(tuple(sorted((a, b))) for face in part.solid.faces for a, b in zip(face, face[1:] + face[:1]))
                self.assertTrue(all(count == 2 for count in edges.values()), part.key)
                self.assertGreater(volume(part.solid), CASES["minimum_volume"], (tower.key, part.key))

    def test_projected_seams_join_sheet_plane_and_match_rhino_plane_distances(self):
        for tower in variants():
            for part in M.capital_assembly(tower).parts:
                if part.role not in ("capital_seam", "capital_flashing"):
                    continue
                meta = part.solid.meta
                origin, normal = np.asarray(meta["origin"]), np.asarray(meta["normal"])
                vertices = np.asarray(part.solid.vertices)
                distances = (vertices - origin) @ normal
                plane = rh.Plane(rh.Point3d(*origin), rh.Vector3d(*normal))
                transform = rh.Transform.PlaneToPlane(plane, rh.Plane.WorldXY())
                mapped = []
                for point in vertices:
                    converted = rh.Point3d(*point)
                    converted.Transform(transform)
                    mapped.append(converted.Z)
                expected = np.array(mapped)
                np.testing.assert_allclose(distances, expected, atol=CASES["tolerance"])
                self.assertAlmostEqual(float(distances.min()), -RAW["capital"]["cladding"]["sheet_thickness"]["value"], places=6)
                self.assertAlmostEqual(float(distances.max()), meta["projection"], places=6)
                if part.role == "capital_seam":
                    self.assertGreater(meta["projection"], 0)

    def test_finished_tip_height_is_preserved_under_live_dimensions(self):
        for tower in variants():
            selected = [part for part in M.capital_assembly(tower).parts if part.role in CASES["roles"]]
            heights = np.array([p[2] for part in selected for p in part.solid.vertices])
            tip = M.capital_base(tower) + tower.capital_height
            self.assertAlmostEqual(float(heights.max()), tip, places=7)
            self.assertGreaterEqual(float(heights.min()), M.capital_base(tower) - CASES["tolerance"])

    def test_wall_panels_follow_independent_corner_interpolation(self):
        corner_map = {"east": (0, 1), "south": (1, 2), "west": (2, 3), "north": (3, 0)}
        raw_heights = np.array([q["value"] for q in RAW["capital"]["roof_corners"]])
        for tower in variants():
            half = tower.core_side / 2 + tower.capital_overhang
            corners = np.array(((half, half), (half, -half), (-half, -half), (-half, half))) + tower.center
            heights = M.capital_base(tower) + raw_heights * tower.capital_height / raw_heights.max()
            for part in M.capital_assembly(tower).parts:
                surface = part.solid.meta.get("surface")
                if part.role != "capital" or surface not in corner_map:
                    continue
                a, b = corner_map[surface]
                points = np.asarray(part.solid.vertices)
                along = (points[:, :2] - corners[a]) @ (corners[b] - corners[a]) / np.dot(corners[b] - corners[a], corners[b] - corners[a])
                roof = heights[a] + along * (heights[b] - heights[a])
                self.assertTrue(np.all(points[:, 2] <= roof + CASES["tolerance"]), part.key)

    def test_handcrafted_glyph_strokes_and_open_counters_match_physical_geometry(self):
        tower = next(t for t in M.towers() if t.key == "A")
        data = RAW["capital"]["signage"]
        height_scale = tower.capital_height / max(q["value"] for q in RAW["capital"]["roof_corners"])
        scale = min(height_scale, (tower.core_side + 2 * tower.capital_overhang) / data["reference_base_side"]["value"])
        size, offset, pitch = (data[k]["value"] * scale for k in ("glyph_size", "corner_offset", "vertical_pitch"))
        half = tower.core_side / 2 + tower.capital_overhang
        top = M.capital_base(tower) + tower.capital_height - data["top_offset"]["value"] * height_scale
        parts = [p for p in M.capital_assembly(tower).parts if p.role == "historic_sign"]
        for case in CASES["glyph_queries"]:
            index = data["text"].index(case["character"])
            u, v = case["point"]
            for face in CASES["sign_faces"]:
                horizontal = tower.center[0] - half + offset if face == "north" else tower.center[1] + half - offset
                query = np.array((horizontal + (u - .5) * size, top - index * pitch - v * size))
                selected = [p for p in parts if p.solid.meta["character"] == case["character"] and p.solid.meta["face"] == face]
                hit = False
                for part in selected:
                    points = np.asarray(part.solid.vertices)[:, [0 if face == "north" else 1, 2]]
                    points = np.unique(np.round(points, 10), axis=0)
                    center = points.mean(axis=0)
                    polygon = points[np.argsort(np.arctan2(points[:, 1] - center[1], points[:, 0] - center[0]))]
                    edges, relative = np.roll(polygon, -1, axis=0) - polygon, query - polygon
                    signs = edges[:, 0] * relative[:, 1] - edges[:, 1] * relative[:, 0]
                    hit |= bool(np.all(signs >= -1e-8) or np.all(signs <= 1e-8))
                self.assertEqual(hit, case["solid"], (case, face))

    def test_existing_louvre_has_real_cladding_opening_with_solid_controls(self):
        for tower in M.towers():
            parts = [part for part in M.capital_assembly(tower).parts if part.role in CASES["roles"] and part.solid.meta["surface"] == "south"]
            for case in CASES["louvre_queries"]:
                query = np.array((tower.center[0] + case["x"], M.capital_base(tower) + case["z"]))
                hit = False
                for part in parts:
                    points = np.unique(np.round(np.asarray(part.solid.vertices)[:, [0, 2]], 10), axis=0)
                    center = points.mean(axis=0)
                    polygon = points[np.argsort(np.arctan2(points[:, 1] - center[1], points[:, 0] - center[0]))]
                    edges, relative = np.roll(polygon, -1, axis=0) - polygon, query - polygon
                    signs = edges[:, 0] * relative[:, 1] - edges[:, 1] * relative[:, 0]
                    hit |= bool(np.all(signs >= -1e-8) or np.all(signs <= 1e-8))
                self.assertEqual(hit, case["solid"], (tower.key, case))

    def test_vertical_panel_rhythm_respects_authored_maximum_pitch(self):
        for tower in variants():
            for part in M.capital_assembly(tower).parts:
                if part.role != "capital" or part.solid.meta["surface"].startswith("roof"):
                    continue
                normal = np.asarray(part.solid.meta["normal"])
                tangent = np.array((-normal[1], normal[0], 0))
                positions = np.asarray(part.solid.vertices) @ tangent
                self.assertLessEqual(np.ptp(positions), CASES["panel_pitch"] + CASES["tolerance"])

    def test_sign_character_geometry_and_faces_remain_preserved(self):
        tower = next(t for t in M.towers() if t.key == "A")
        glyphs = [p for p in M.capital_assembly(tower).parts if p.role == "historic_sign"]
        self.assertEqual({p.solid.meta["character"] for p in glyphs}, set(CASES["sign_text"]))
        self.assertEqual({p.solid.meta["face"] for p in glyphs}, set(CASES["sign_faces"]))
        self.assertTrue(all(volume(part.solid) > 0 for part in glyphs))


if __name__ == "__main__":
    unittest.main(verbosity=2)
