"""🧪 Portable Nakagin teaching contracts with independent oracles (NumPy, rhino3dm, raw JSON counting)."""

import importlib.util
import json
import math
import sys
import unittest
from pathlib import Path

import numpy as np
import rhino3dm as rh

sys.dont_write_bytecode = True
NAKAGIN = Path(__file__).resolve().parent.parent
CASES = json.loads((NAKAGIN / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
RAW = json.loads((NAKAGIN / "🔣️.json").read_text(encoding="utf-8"))


def model_api():
    """🧱 Load the semantic model without any drawing engine."""
    spec = importlib.util.spec_from_file_location("architecture_nakagin_model", NAKAGIN / "🐍️.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules["architecture_nakagin_model"] = module
    spec.loader.exec_module(module)
    return module


M = model_api()


def count_oracle():
    """🔢 Count branches and capsules by walking the raw JSON, independent of the model code."""
    branches, per_tower = [], {}
    for ti, tower in enumerate(RAW["towers"]):
        for storey, floor in enumerate(tower["floors"]):
            for group in floor["groups"]:
                branches.append((0, ti, storey, group["facade"]))
        per_tower[tower["key"]] = sum(1 for b in branches if b[1] == ti)
    return branches, per_tower


def rhino_plane(plane):
    """🦏 Build a rhino3dm Plane from model axes."""
    return rh.Plane(rh.Point3d(*plane.origin), rh.Vector3d(*plane.x), rh.Vector3d(*plane.y))


def rhino_map(points, plane):
    """🦏 Map local points with Transform.PlaneToPlane(World XY, plane)."""
    xf = rh.Transform.PlaneToPlane(rh.Plane.WorldXY(), rhino_plane(plane))
    out = []
    for p in points:
        q = rh.Point3d(*p)
        q.Transform(xf)
        out.append((q.X, q.Y, q.Z))
    return np.array(out)


def signed_volume(solid):
    """📦 Volume of a closed polyhedron by the divergence theorem with fan triangulation (NumPy)."""
    v = np.asarray(solid.vertices, dtype=float)
    total = 0.0
    for face in solid.faces:
        for i in range(1, len(face) - 1):
            total += np.dot(v[face[0]], np.cross(v[face[i]], v[face[i + 1]])) / 6
    return total


def shoelace(points):
    """📐 Polygon area by the shoelace formula."""
    p = np.asarray(points, dtype=float)
    return abs(float(np.dot(p[:, 0], np.roll(p[:, 1], -1)) - np.dot(p[:, 1], np.roll(p[:, 0], -1)))) / 2


def segment_hits_solid(solid, start, end):
    """🎯 Independently intersect a finite segment with face triangles using NumPy Möller–Trumbore tests."""
    origin, direction = np.asarray(start, dtype=float), np.asarray(end, dtype=float) - np.asarray(start, dtype=float)
    vertices = np.asarray(solid.vertices, dtype=float)
    for face in solid.faces:
        for i in range(1, len(face) - 1):
            a, b, c = vertices[[face[0], face[i], face[i + 1]]]
            e1, e2 = b - a, c - a
            cross = np.cross(direction, e2)
            determinant = np.dot(e1, cross)
            if abs(determinant) < 1e-12:
                continue
            offset = origin - a
            u = np.dot(offset, cross) / determinant
            if u < -1e-9 or u > 1 + 1e-9:
                continue
            q = np.cross(offset, e1)
            v = np.dot(direction, q) / determinant
            t = np.dot(e2, q) / determinant
            if v >= -1e-9 and u + v <= 1 + 1e-9 and 1e-8 < t < 1 - 1e-8:
                return True
    return False


class DataTests(unittest.TestCase):
    def test_load_and_value_paths(self):
        self.assertEqual(M.load()["version"], 1)
        v = M.value("capsule.width")
        self.assertEqual((v.value, v.unit, v.basis, v.source), (2.5, "m", "sourced", "wikipedia"))
        self.assertEqual(M.value("types.D.rotation").value, 0.0)
        self.assertEqual(M.value("towers.1.capsule_storeys").value, 9)
        self.assertEqual(M.value("energy.u_window").source, "tokyo-window-2019")
        self.assertIn("jma-tokyo-normals", M.sources())
        with self.assertRaises(KeyError):
            M.value("capsule.nothing")
        with self.assertRaises(KeyError):
            M.value("capsule")

    def test_every_leaf_has_basis_and_known_source(self):
        def walk(node):
            if isinstance(node, dict):
                if "value" in node and "unit" in node:
                    yield node
                else:
                    for child in node.values():
                        yield from walk(child)
            elif isinstance(node, list):
                for child in node:
                    yield from walk(child)
        leaves = list(walk({k: v for k, v in RAW.items() if k != "sources"}))
        self.assertGreater(len(leaves), 100)
        for leaf in leaves:
            self.assertIn(leaf["basis"], ("sourced", "assumed", "derived"))
            if leaf["basis"] == "sourced":
                self.assertIn(leaf["source"], RAW["sources"])

    def test_validation_rejects_broken_data(self):
        broken = json.loads(json.dumps(RAW))
        broken["capsule"]["width"].pop("source")
        with self.assertRaises(ValueError):
            M._validate(broken)
        broken = json.loads(json.dumps(RAW))
        broken["types"][0]["window_diameter"]["value"] = 2.6
        with self.assertRaises(ValueError):
            M._validate(broken)
        broken = json.loads(json.dumps(RAW))
        del broken["climate"]
        with self.assertRaises(ValueError):
            M._validate(broken)


class CapsuleTests(unittest.TestCase):
    def test_four_types_are_variants_of_one_capsule(self):
        types = M.capsule_types()
        self.assertEqual([t.key for t in types], ["A", "B", "C", "D"])
        self.assertEqual({(t.width, t.length, t.height) for t in types}, {(2.5, 4.0, 2.5)})
        self.assertEqual(len({t.window_diameter for t in types}), 1)
        self.assertEqual({t.key: t.count for t in types}, CASES["counts"]["per_type"])

    def test_solid_topology_and_features(self):
        s = M.capsule_solid(M.capsule_type("A"))
        c = CASES["solid"]
        self.assertEqual((len(s.vertices) // 2, len(s.vertices), len(s.faces), len(s.edges)), (c["profile_points"], c["vertices"], c["faces"], c["edges"]))
        self.assertEqual(np.asarray(s.window.center).tolist(), c["window_center"])
        self.assertEqual(np.asarray(s.bolts).tolist(), c["bolts"])
        self.assertEqual(len(s.bolts), CASES["counts"]["bolts_per_capsule"])
        v = np.asarray(s.vertices)
        self.assertTrue(np.allclose(v.min(axis=0), [-1.25, 0, 0]) and np.allclose(v.max(axis=0), [1.25, 4, 2.5]))
        edge_use = {}
        for face in s.faces:
            for a, b in zip(face, face[1:] + face[:1]):
                edge_use[(min(a, b), max(a, b))] = edge_use.get((min(a, b), max(a, b)), 0) + 1
        self.assertEqual(set(edge_use.values()), {2})
        self.assertEqual(set(edge_use), set(s.edges))

    def test_volume_matches_numpy_prism_and_rhino_box(self):
        t = M.capsule_type("A")
        s = M.capsule_solid(t)
        v = np.asarray(s.vertices)
        profile = v[: len(v) // 2][:, [0, 2]]
        self.assertAlmostEqual(signed_volume(s), shoelace(profile) * t.length, places=9)
        sharp = t.width * t.height - (4 - math.pi) * t.corner_radius ** 2
        self.assertLess(abs(shoelace(profile) - sharp) / sharp, 0.002)
        box = rh.Box(rh.BoundingBox(-1.25, 0, 0, 1.25, 4, 2.5))
        self.assertAlmostEqual(box.Volume, 25.0)
        analytic = box.Volume - (4 - math.pi) * t.corner_radius ** 2 * t.length
        self.assertLess(abs(signed_volume(s) - analytic) / analytic, 0.002)

    def test_window_lies_on_front_face_and_fits(self):
        for t in M.capsule_types():
            s = M.capsule_solid(t)
            cx, cy, cz = s.window.center
            r = s.window.radius
            self.assertEqual((cy, s.window.normal), (t.length, (0.0, 1.0, 0.0)))
            self.assertAlmostEqual(r * 2, t.window_diameter)
            self.assertLessEqual(abs(cx) + r, t.width / 2)
            self.assertTrue(0 <= cz - r and cz + r <= t.height)
            self.assertAlmostEqual(rh.Circle(rh.Point3d(*s.window.center), r).Diameter, t.window_diameter)

    def test_parameter_dict_overrides_the_type(self):
        s = M.capsule_solid({"key": "A", "width": 3.0, "length": 5.0, "height": 2.8})
        v = np.asarray(s.vertices)
        self.assertTrue(np.allclose(v.max(axis=0) - v.min(axis=0), [3.0, 5.0, 2.8]))

    def test_place_matches_rhino_plane_to_plane(self):
        for slot in (M.slots()[0], M.slots()[37], M.slots()[-1]):
            local = M.capsule_solid(M.capsule_type(slot.type))
            placed = M.place(local, slot.plane)
            expected = rhino_map(local.vertices, slot.plane)
            self.assertTrue(np.allclose(np.asarray(placed.vertices), expected, atol=1e-9))
            self.assertTrue(np.allclose(np.asarray(placed.bolts), rhino_map(local.bolts, slot.plane), atol=1e-9))
            self.assertTrue(np.allclose(placed.window.center, rhino_map([local.window.center], slot.plane)[0], atol=1e-9))
            mid = rhino_map([(0, 2, 1.25)], slot.plane)[0]
            self.assertTrue(np.allclose(np.asarray(placed.vertices).mean(axis=0), mid, atol=1e-9))


class TowerTests(unittest.TestCase):
    def test_tree_counts_match_independent_oracle(self):
        branches, per_tower = count_oracle()
        tr = M.tree()
        self.assertEqual(len(branches), CASES["counts"]["branches"])
        self.assertEqual(set(tr), set(branches))
        self.assertEqual(len(M.slots()), CASES["counts"]["slots"])
        self.assertTrue(all(len(items) in CASES["counts"]["items_per_branch"] for items in tr.values()))
        self.assertEqual(sum(len(v) == 1 for v in tr.values()), CASES["counts"]["singletons"])
        self.assertEqual(per_tower, CASES["counts"]["branches_per_tower"])
        self.assertEqual({k: sum(len(v) for p, v in tr.items() if p[1] == i) for i, k in enumerate("AB")}, CASES["counts"]["per_tower"])
        for path in CASES["absent"]:
            self.assertNotIn(tuple(path["path"]), tr)

    def test_slots_follow_tree_order_and_paths(self):
        slots = M.slots()
        self.assertEqual(list(M.tree()), sorted(M.tree()))
        self.assertEqual(len(slots), 140)
        self.assertTrue(all(s.building == 0 for s in slots))
        self.assertEqual({s.sublevel for s in slots}, {0, 1, 2})
        self.assertEqual({s.layout for s in slots}, {"Slash", "Backslash", "J", "L", "P", "S"})
        for tower in M.towers():
            actual = [sum(s.tower == tower.index and s.floor == floor for s in slots) for floor in range(3, tower.storeys + 1)]
            self.assertEqual(actual, CASES["floor_counts"][tower.key])
            for storey, facades in enumerate(M.facade_storeys(tower)):
                for facade in facades:
                    self.assertEqual(tuple(s.item for s in M.tree()[(0, tower.index, storey, facade)]), M.facade_items(tower, storey, facade))

    def test_restored_a_capsules_have_actual_l_pose_and_unique_footprints(self):
        case = CASES["restored_a_l"]
        tower = next(t for t in RAW["towers"] if t["key"] == "A")
        for floor in (f for f in tower["floors"] if f["floor"] in case["floors"]):
            items = floor["groups"]
            selected = next(item for group in items if group["facade"] == case["facade"] for item in group["items"] if item["item"] == case["item"])
            self.assertEqual((selected["family"], selected["source_item"], selected["sublevel"]), (case["family"], case["source_item"], case["sublevel"]))
            np.testing.assert_allclose([q["value"] for q in selected["center"][:2]], case["center_xy"])
            self.assertAlmostEqual(selected["rotation"]["value"], case["rotation"])
            centers = np.array([[q["value"] for q in item["center"]] for group in items for item in group["items"]])
            self.assertEqual(len(np.unique(np.round(centers, 8), axis=0)), len(centers))

    def test_type_pattern_and_counts(self):
        order = RAW["assignment"]["order"]
        for s in M.slots():
            self.assertEqual(s.type, order[(s.tower + s.storey + 2 * s.facade + s.item) % 4])
        counts = {}
        for s in M.slots():
            counts[s.type] = counts.get(s.type, 0) + 1
        self.assertEqual(counts, CASES["counts"]["per_type"])
        self.assertEqual(sum(counts.values()), 140)

    def test_planes_are_orthonormal_and_right_handed(self):
        for s in M.slots():
            for plane in (s.plane, s.facade_plane):
                a = np.array([plane.x, plane.y, plane.z])
                self.assertTrue(np.allclose(a @ a.T, np.eye(3), atol=1e-12))
                self.assertGreater(np.linalg.det(a), 0.999999)
                rp = rhino_plane(plane)
                self.assertTrue(np.allclose([rp.ZAxis.X, rp.ZAxis.Y, rp.ZAxis.Z], plane.z, atol=1e-12))

    def test_sample_planes_match_cases(self):
        tr = M.tree()
        for case in CASES["slot_samples"]:
            slot = tr[tuple(case["path"])][case["item"]]
            self.assertEqual(slot.type, case["type"])
            self.assertTrue(np.allclose(slot.plane.origin, case["origin"], atol=1e-3))
            self.assertTrue(np.allclose(slot.plane.x, case["x"], atol=1e-8))
            self.assertTrue(np.allclose(slot.plane.y, case["y"], atol=1e-8))
            self.assertAlmostEqual(slot.rotation, case["rotation"])

    def test_actual_door_ports_sit_on_attachment_facades(self):
        for slot in M.slots():
            tower = M.towers()[slot.tower]
            normal = np.array(RAW["facades"][slot.facade]["normal"])
            distance = (np.array(slot.facade_plane.origin[:2]) - np.array(tower.center)) @ normal
            self.assertAlmostEqual(distance, tower.core_side / 2, places=6)
            self.assertEqual(tuple(slot.facade_plane.y[:2]), tuple(normal))

    def test_no_capsule_penetrates_any_core_and_rotated_ones_touch(self):
        cores = []
        for tower in M.towers():
            v = np.asarray(M.core_solid(tower).vertices)
            cores.append((v.min(axis=0), v.max(axis=0), rh.BoundingBox(*(v.min(axis=0) + 1e-6), *(v.max(axis=0) - 1e-6))))
        touching = 0
        for s in M.slots():
            placed = M.slot_solid(s)
            v = np.asarray(placed.vertices)
            for lo, hi, box in cores:
                inside = np.all((v > lo + 1e-9) & (v < hi - 1e-9), axis=1)
                self.assertFalse(inside.any(), s.path)
                self.assertFalse(any(box.Contains(rh.Point3d(*p)) for p in v), s.path)
            tower = M.towers()[s.tower]
            normal = np.array(RAW["facades"][s.facade]["normal"], dtype=float)
            depth = (v[:, :2] - np.array(tower.center)) @ normal - tower.core_side / 2
            self.assertGreaterEqual(depth.min(), -1e-9, s.path)
            self.assertLessEqual(depth.min(), .100001)
            touching += 1
            self.assertIn(round(abs(float(np.dot(s.plane.y, s.facade_plane.y)))), (0, 1))
        self.assertEqual(touching, 140)

    def test_capsules_do_not_overlap_in_plan_box_checks(self):
        boxes = []
        for s in M.slots():
            v = np.asarray(M.slot_solid(s).vertices)
            boxes.append((v.min(axis=0), v.max(axis=0), s.tower))
        pairs = 0
        for i in range(len(boxes)):
            for j in range(i + 1, len(boxes)):
                lo = np.maximum(boxes[i][0], boxes[j][0])
                hi = np.minimum(boxes[i][1], boxes[j][1])
                if np.all(hi - lo > 1e-6):
                    pairs += 1
        self.assertEqual(pairs, 0)

    def test_core_capital_podium(self):
        for tower in M.towers():
            expected = CASES["tower"][tower.key]
            self.assertEqual((tower.storeys, tower.height, tower.base_elevation), (expected["storeys"], expected["height"], expected["base_elevation"]))
            core = np.asarray(M.core_solid(tower).vertices)
            self.assertTrue(np.allclose(core.max(axis=0) - core.min(axis=0), [5.5, 5.5, tower.height]))
            self.assertAlmostEqual(signed_volume(M.core_solid(tower)), 5.5 * 5.5 * tower.height)
            cap = M.capital_solid(tower)
            v = np.asarray(cap.vertices)
            self.assertAlmostEqual(v[:, 2].max(), expected["capital_top"])
            self.assertAlmostEqual(cap.meta["base_side"], 5.5)
            self.assertAlmostEqual(cap.meta["roof_side"], 5.5)
            self.assertEqual(list(cap.meta["roof_corners"]), CASES["assembly"]["capital"]["roof_corners"][tower.key])
            self.assertGreater(signed_volume(cap), 0)
        p = M.podium_solid()
        self.assertAlmostEqual(np.asarray(p.vertices)[:, 2].max(), CASES["assembly"]["podium"]["levels"][-1])
        area = shoelace(np.asarray(p.vertices)[:len(p.vertices) // 2, :2])
        self.assertAlmostEqual(area, CASES["assembly"]["podium"]["roof_area"], places=3)
        self.assertAlmostEqual(signed_volume(p), area * M.podium_height(), places=5)

    def test_all_solids_have_outward_faces(self):
        solids = [M.capsule_solid(t) for t in M.capsule_types()] + [M.core_solid(0), M.capital_solid(1), M.podium_solid()]
        for s in solids:
            self.assertGreater(signed_volume(s), 0)
            v = np.asarray(s.vertices)
            center = v.mean(axis=0)
            for face in s.faces:
                p = v[list(face)]
                normal = np.cross(p[1] - p[0], p[2] - p[1])
                self.assertGreater(np.dot(normal, p.mean(axis=0) - center), -1e-9)


class ProgramEnergyTests(unittest.TestCase):
    def test_program_matches_cases(self):
        for key, expected in CASES["program"].items():
            p = M.program(key)
            self.assertAlmostEqual(p["net_floor_area"], expected["net_floor_area"])
            self.assertAlmostEqual(p["volume"], expected["volume"])
            self.assertAlmostEqual(p["fitting_area"], expected["fitting_area"])
            self.assertEqual(p["occupants"], expected["occupants"])
            self.assertAlmostEqual(p["free_area"], p["net_floor_area"] - p["fitting_area"])

    def test_climate_matches_numpy(self):
        c = M.climate()
        means = np.array([m["mean"]["value"] for m in RAW["climate"]["months"]])
        days = np.array([m["days"]["value"] for m in RAW["climate"]["months"]])
        hdd = np.clip(18.0 - means, 0, None) * days
        self.assertAlmostEqual(c["hdd"], hdd.sum(), places=9)
        self.assertAlmostEqual(c["hdd"], CASES["climate"]["hdd"], places=6)
        for month in c["months"]:
            self.assertAlmostEqual(month["hdd"], CASES["climate"]["monthly_hdd"][month["key"]], places=6)
        self.assertEqual(sum(days), 365)

    def test_envelope_and_heating_match_numpy(self):
        e = RAW["energy"]
        for t in M.capsule_types():
            raw = next(x for x in RAW["types"] if x["key"] == t.key)
            w, l, h = 2.5, 4.0, 2.5
            d = raw["window_diameter"]["value"]
            win = np.pi * (d / 2) ** 2
            area = np.array([w * l, w * l, h * l, h * l, w * h - win, win, w * h])
            u = np.array([e["u_roof"]["value"], e["u_floor"]["value"], e["u_wall"]["value"], e["u_wall"]["value"], e["u_wall"]["value"], e["u_window"]["value"], e["u_wall"]["value"]])
            factor = np.array([1, 1, 1, 1, 1, 1, e["attached_factor"]["value"]])
            vol = 2.3 * 3.8 * 2.1
            h_t = float((area * u * factor).sum())
            h_v = 0.34 * 0.5 * vol
            env = M.envelope(t)
            self.assertEqual([s["key"] for s in env["surfaces"]], ["roof", "floor", "wall_left", "wall_right", "front", "window", "back"])
            self.assertTrue(np.allclose([s["area"] for s in env["surfaces"]], area))
            self.assertAlmostEqual(env["h_t"], h_t, places=9)
            self.assertAlmostEqual(env["h_v"], h_v, places=9)
            self.assertAlmostEqual(env["window_area"], win)
            self.assertAlmostEqual(env["exposed_area"] + env["attached_area"], 2 * w * l + 2 * h * l + 2 * w * h)
            hot = M.heating(t)
            hdd = CASES["climate"]["hdd"]
            self.assertAlmostEqual(hot["load"], (h_t + h_v) * 20.0, places=9)
            self.assertAlmostEqual(hot["annual"], (h_t + h_v) * hdd * 24 / 1000, places=6)
            self.assertAlmostEqual(hot["annual_per_area"], hot["annual"] / (2.3 * 3.8), places=9)
        a = CASES["heating"]["A"]
        env = M.envelope("A")
        for key in ("window_area", "h_t", "h_v", "h"):
            self.assertAlmostEqual(env[key], a[key], places=9)
        self.assertAlmostEqual(M.heating("A")["load"], a["load"], places=9)
        self.assertAlmostEqual(M.heating("A")["annual"], a["annual"], places=6)
        self.assertAlmostEqual(M.envelope("C")["h"], CASES["heating"]["C"]["h"], places=9)

    def test_totals_sum_over_slots(self):
        t = M.totals()
        self.assertEqual(t["count"], 140)
        self.assertAlmostEqual(t["area"], 140 * 8.74)
        self.assertEqual({k: v["count"] for k, v in t["per_tower"].items()}, CASES["counts"]["per_tower"])
        self.assertAlmostEqual(sum(v["annual"] for v in t["per_tower"].values()), t["annual"], places=6)
        self.assertAlmostEqual(t["annual_per_area"], t["annual"] / t["area"])

    def test_better_window_lowers_demand(self):
        base = M.envelope("A")["h"]
        w = M.envelope("A")["window_area"]
        self.assertLess(base - w * (2.33 - 1.0), base)


class VariantTests(unittest.TestCase):
    def test_variant_lookup_and_names(self):
        self.assertEqual(M.variants(), tuple(CASES["variants"]["keys"]))
        for key in M.variants():
            v = M.variant(key)
            self.assertTrue(v["name"]["en"] and v["name"]["de"] and v["description"]["en"] and v["description"]["de"])
            self.assertEqual({k: o.value for k, o in v["overrides"].items()}, CASES["variants"]["overrides"][key])
            self.assertTrue(all(o.basis in ("sourced", "assumed", "derived") and o.unit for o in v["overrides"].values()))
        with self.assertRaises(KeyError):
            M.variant("nothing")

    def test_heating_under_variants_matches_numpy(self):
        e = {k: v["value"] for k, v in RAW["energy"].items()}
        win = np.pi * 0.65 ** 2
        vol = 2.3 * 3.8 * 2.1
        hdd = CASES["climate"]["hdd"]
        for row in CASES["variants"]["heating_A"]:
            u = dict(e)
            for name in row["variant"] or []:
                for k, leaf in next(x for x in RAW["variants"] if x["key"] == name)["overrides"].items():
                    u[k] = leaf["value"]
            area = np.array([10, 10, 10, 10, 6.25 - win, win, 6.25])
            coef = np.array([u["u_roof"], u["u_floor"], u["u_wall"], u["u_wall"], u["u_wall"], u["u_window"], u["u_wall"] * u["attached_factor"]])
            h_t = float(area @ coef)
            h = h_t + 0.34 * u["air_change"] * vol
            got = M.heating("A", row["variant"] and (row["variant"][0] if len(row["variant"]) == 1 else tuple(row["variant"])))
            self.assertAlmostEqual(got["h_t"], h_t, places=9)
            self.assertAlmostEqual(got["h"], h, places=9)
            self.assertAlmostEqual(got["h"], row["h"], places=9)
            self.assertAlmostEqual(got["annual"], h * hdd * 24 / 1000, places=6)
            self.assertAlmostEqual(got["annual"], row["annual"], places=6)

    def test_variants_change_the_right_surfaces_only(self):
        base = {s["key"]: s["ua"] for s in M.envelope("A")["surfaces"]}
        better = {s["key"]: s["ua"] for s in M.envelope("A", "window_upgrade")["surfaces"]}
        self.assertEqual({k for k in base if base[k] != better[k]}, {"window"})
        self.assertAlmostEqual(base["window"] - better["window"], (2.33 - 1.1) * np.pi * 0.65 ** 2)
        wrong = {s["key"]: s["ua"] for s in M.envelope("A", "back_wall_exposed")["surfaces"]}
        self.assertAlmostEqual(wrong["back"], 2 * base["back"])
        shared = {s["key"]: s["ua"] for s in M.envelope("A", "shared_roof_floor")["surfaces"]}
        self.assertEqual((shared["roof"], shared["floor"]), (0.0, 0.0))
        self.assertGreater(M.heating("A")["annual"], M.heating("A", "window_upgrade")["annual"])
        self.assertGreater(M.heating("A", "back_wall_exposed")["annual"], M.heating("A")["annual"])
        self.assertIsNone(M.heating("A")["variant"])
        self.assertEqual(M.heating("A", "window_upgrade")["variant"], "window_upgrade")

    def test_data_is_not_mutated_by_variants(self):
        before = M.heating("A")["annual"]
        M.heating("A", ("window_upgrade", "shared_roof_floor"))
        self.assertEqual(M.heating("A")["annual"], before)
        self.assertEqual(M.value("energy.u_window").value, 2.33)

    def test_climate_source_and_period(self):
        c = M.climate()
        src = CASES["climate_source"]
        self.assertEqual((c["source"], c["period"]["en"], c["hdd_value"].unit), (src["source"], src["period_en"], src["unit"]))
        self.assertTrue(c["period"]["de"] and c["method"]["en"] and c["method"]["de"])
        self.assertAlmostEqual(c["hdd_value"].value, c["hdd"], places=6)
        self.assertAlmostEqual(c["hdd_value"].value, src["hdd"], places=6)
        self.assertEqual(c["sources"][0]["url"], RAW["sources"]["jma-tokyo-normals"]["url"])




class AssemblyTests(unittest.TestCase):
    def test_capsule_parts_match_portable_contract(self):
        expected = CASES["assembly"]["capsule"]
        assembly = M.capsule_assembly("A")
        roles = [p.role for p in assembly.parts]
        self.assertTrue(set(expected["required_roles"]).issubset(roles))
        self.assertEqual(roles.count("bolt_shaft"), expected["bolts"])
        self.assertEqual(roles.count("bolt_head"), expected["bolts"])
        self.assertEqual(len({p.key for p in assembly.parts}), len(assembly.parts))
        for part in assembly.parts:
            self.assertGreater(signed_volume(part.solid), 0, part.key)
            uses = {}
            for face in part.solid.faces:
                for a, b in zip(face, face[1:] + face[:1]):
                    edge = tuple(sorted((a, b)))
                    uses[edge] = uses.get(edge, 0) + 1
            self.assertEqual(set(uses.values()), {2}, part.key)

    def test_shell_has_real_window_aperture(self):
        assembly = M.capsule_assembly("A")
        shell = next(p.solid for p in assembly.parts if p.role == "shell")
        t = M.capsule_type("A")
        centre = np.array([t.window_u, t.length, t.window_v])
        triangles = [np.array([shell.vertices[i] for i in face]) for face in shell.faces]
        for polygon in triangles:
            if np.allclose(polygon[:, 1], t.length):
                self.assertGreater(np.linalg.norm(polygon[:, [0, 2]].mean(axis=0) - centre[[0, 2]]), t.window_diameter / 2 * .95)
        glazing = next(p.solid for p in assembly.parts if p.role == "glazing")
        disk = rh.Circle(rh.Point3d(*centre), t.window_diameter / 2)
        self.assertAlmostEqual(glazing.meta["radius"], disk.Radius)
        self.assertLess(np.ptp(np.asarray(glazing.vertices)[:, 1]), .02)

    def test_assembly_placement_matches_rhino(self):
        slot = M.slots()[37]
        local = M.capsule_assembly(slot.type)
        placed = M.place_assembly(local, slot.plane)
        for a, b in zip(local.parts, placed.parts):
            self.assertEqual((a.key, a.role, a.material), (b.key, b.role, b.material))
            self.assertTrue(np.allclose(np.asarray(b.solid.vertices), rhino_map(a.solid.vertices, slot.plane)))

    def test_structure_clears_capsules_with_rhino_bounds(self):
        structure = M.structure_assembly("exterior")
        blockers = [p for p in structure.parts if p.role in ("bridge", "bridge_rail", "capital", "roof")]
        for slot in M.slots():
            vertices = np.asarray(M.slot_solid(slot).vertices)
            lo, hi = vertices.min(axis=0), vertices.max(axis=0)
            for part in blockers:
                points = np.asarray(part.solid.vertices)
                lower, upper = points.min(axis=0), points.max(axis=0)
                overlap = np.minimum(hi, upper) - np.maximum(lo, lower)
                self.assertFalse(np.all(overlap > 1e-8), (slot.path, slot.item, part.key))
                inner = rh.BoundingBox(*(lo + 1e-7), *(hi - 1e-7))
                self.assertFalse(any(inner.Contains(rh.Point3d(*p)) for p in points), part.key)

    def test_every_capsule_entry_opens_through_core_wall(self):
        cores = {t.index: M.core_assembly(t, "exterior") for t in M.towers()}
        count = 0
        for slot in M.slots():
            local = (M.value("detail.door_offset").value, -.1, 1.0)
            point = rhino_map([(0, -.1, 1.0)], slot.facade_plane)[0]
            for part in cores[slot.tower].parts:
                if part.role != "core_wall":
                    continue
                vertices = np.asarray(part.solid.vertices)
                box = rh.BoundingBox(*(vertices.min(axis=0) + 1e-7), *(vertices.max(axis=0) - 1e-7))
                self.assertFalse(box.Contains(rh.Point3d(*point)), slot.path)
            count += 1
        self.assertEqual(count, CASES["assembly"]["core"]["door_openings"])

    def test_distinct_fitting_roles_do_not_interpenetrate(self):
        checked = set(CASES["assembly"]["capsule"]["independent_fitting_roles"])
        for t in M.capsule_types():
            parts = [p for p in M.capsule_assembly(t, interior="study").parts if p.role in checked]
            for i, a in enumerate(parts):
                av = np.asarray(a.solid.vertices)
                for b in parts[i + 1:]:
                    if a.role == b.role:
                        continue
                    bv = np.asarray(b.solid.vertices)
                    overlap = np.minimum(av.max(axis=0), bv.max(axis=0)) - np.maximum(av.min(axis=0), bv.min(axis=0))
                    self.assertFalse(np.all(overlap > 1e-8), (t.key, a.key, b.key))

    def test_all_six_families_have_closed_parts_and_correct_window_wall(self):
        for layout, expected in CASES["assembly"]["capsule"]["families"].items():
            assembly = M.capsule_assembly("A", "full", layout)
            glazing = next(p.solid for p in assembly.parts if p.role == "glazing")
            bounds = np.asarray(glazing.vertices)
            self.assertEqual(int(np.argmin(np.ptp(bounds, axis=0))), expected["window_axis"])
            circle = rh.Circle(rh.Point3d(*bounds.mean(axis=0)), CASES["assembly"]["capsule"]["historical_window_diameter"] / 2)
            self.assertAlmostEqual(glazing.meta["radius"], circle.Radius)
            barriers = [p for p in assembly.parts if p.role in ("shell", "lining", "steel_truss")]
            for u, v in ((0, 0), (.5, 0), (-.5, 0), (0, .5), (0, -.5)):
                wall = "right" if expected["window_axis"] == 0 else "front"
                window = CASES["assembly"]["capsule"]["analytic_window"][wall]
                normal = np.asarray(window["normal"])
                tangent = np.array([0, 1, 0]) if wall == "right" else np.array([1, 0, 0])
                point = np.asarray(window["center"]) + u * tangent + np.array([0, 0, v])
                start, end = point + .2 * normal, point - .3 * normal
                self.assertFalse(any(segment_hits_solid(p.solid, start, end) for p in barriers), (layout, u, v, "window"))
            for part in assembly.parts:
                self.assertGreater(signed_volume(part.solid), 0, (layout, part.key))
                uses = {}
                for face in part.solid.faces:
                    for a, b in zip(face, face[1:] + face[:1]):
                        edge = tuple(sorted((a, b)))
                        uses[edge] = uses.get(edge, 0) + 1
                self.assertEqual(set(uses.values()), {2}, (layout, part.key))

    def test_slot_envelope_metadata_matches_family_glazing_and_rhino(self):
        for layout, expected in CASES["assembly"]["capsule"]["families"].items():
            slot = next(s for s in M.slots() if s.layout == layout)
            solid = M.slot_solid(slot)
            wall = "right" if expected["window_axis"] == 0 else "front"
            local = CASES["assembly"]["capsule"]["analytic_window"][wall]
            centre = rh.Point3d(*local["center"])
            xf = rh.Transform.PlaneToPlane(rh.Plane.WorldXY(), rhino_plane(slot.plane))
            centre.Transform(xf)
            self.assertTrue(np.allclose(solid.window.center, (centre.X, centre.Y, centre.Z)))
            local_center = rh.Point3d(*local["center"])
            circle = rh.Circle(local_center, CASES["assembly"]["capsule"]["historical_window_diameter"] / 2)
            xy = rh.Plane(local_center, rh.Vector3d(1, 0, 0), rh.Vector3d(0, 1, 0))
            oriented = rh.Plane(local_center, rh.Vector3d(0, 1, 0), rh.Vector3d(0, 0, 1)) if wall == "right" else rh.Plane(local_center, rh.Vector3d(1, 0, 0), rh.Vector3d(0, 0, -1))
            self.assertTrue(circle.Transform(rh.Transform.PlaneToPlane(xy, oriented)))
            self.assertTrue(circle.Transform(xf))
            self.assertTrue(np.allclose(solid.window.center, (circle.Center.X, circle.Center.Y, circle.Center.Z)))
            self.assertTrue(np.allclose(solid.window.normal, (circle.Normal.X, circle.Normal.Y, circle.Normal.Z)))
            self.assertAlmostEqual(solid.window.radius, circle.Radius)
            expected_normal = np.asarray(local["normal"]) @ np.asarray((slot.plane.x, slot.plane.y, slot.plane.z))
            self.assertTrue(np.allclose(solid.window.normal, expected_normal))
            assembly = M.slot_assembly(slot)
            glazing = next(p.solid for p in assembly.parts if p.role == "glazing")
            self.assertTrue(np.allclose(solid.window.center, np.asarray(glazing.vertices).mean(axis=0)))
            self.assertEqual(solid.meta["window_reference"], "glazing_midplane")
            self.assertEqual(len(solid.bolts), 4)

    def test_all_140_placed_families_match_rhino_and_actual_entry_plane(self):
        prototypes = {layout: M.capsule_assembly("A", "exterior", layout) for layout in CASES["assembly"]["capsule"]["families"]}
        for slot in M.slots():
            placed = M.slot_assembly(slot)
            local = prototypes[slot.layout]
            glass = next(p.solid for p in placed.parts if p.role == "glazing")
            source = next(p.solid for p in local.parts if p.role == "glazing")
            self.assertTrue(np.allclose(glass.vertices, rhino_map(source.vertices, slot.plane)))
            door = next(p.solid for p in placed.parts if p.role == "door")
            centre = np.asarray(door.vertices).mean(axis=0)
            delta = centre - np.asarray(slot.facade_plane.origin)
            self.assertLess(abs(float(delta @ np.asarray(slot.facade_plane.y))), .151)
            self.assertLess(abs(float(delta @ np.asarray(slot.facade_plane.x))), .051)

    def test_partial_core_assemblies_do_not_include_higher_capsule_landings(self):
        from dataclasses import replace
        tower = M.towers()[0]
        for storeys in CASES["assembly"]["core"]["partial_storeys"]:
            partial = replace(tower, storeys=storeys, height=storeys * tower.storey_height)
            vertices = np.concatenate([np.asarray(p.solid.vertices) for p in M.core_assembly(partial).parts])
            self.assertLessEqual(vertices[:, 2].max(), partial.height + 1.0)
            self.assertGreater(vertices[:, 2].max(), partial.height + .65)

    def test_steel_skin_and_internal_structural_layers(self):
        assembly = M.capsule_assembly("A")
        roles = {p.role for p in assembly.parts}
        self.assertTrue(set(CASES["assembly"]["capsule"]["full_structural_roles"]).issubset(roles))
        shell = next(p.solid for p in assembly.parts if p.role == "shell")
        self.assertAlmostEqual(shell.meta["wall_thickness"], CASES["assembly"]["capsule"]["steel_skin_thickness"])

    def test_historical_capsules_share_nominal_porthole(self):
        diameter = CASES["assembly"]["capsule"]["historical_window_diameter"]
        for t in M.capsule_types():
            assembly = M.capsule_assembly(t)
            glazing = next(p.solid for p in assembly.parts if p.role == "glazing")
            self.assertAlmostEqual(glazing.meta["radius"] * 2, diameter)

    def test_program_fittings_match_detailed_geometry(self):
        for t in M.capsule_types():
            roles = [p.role for p in M.capsule_assembly(t, interior="study").parts]
            fittings = {f["key"] for f in t.fittings}
            self.assertEqual("bed" in roles, "bed" in fittings)
            self.assertEqual("desk" in roles, "desk" in fittings)
            self.assertEqual("sofa" in roles, "sofa" in fittings)
            self.assertEqual("bathroom" in roles, "bath" in fittings)
            self.assertTrue(all(abs(s.rotation) % 90 == 0 for s in M.slots()))

    def test_reference_interior_and_handed_entry_clearance(self):
        for layout in CASES["assembly"]["capsule"]["families"]:
            assembly = M.capsule_assembly("A", layout=layout)
            self.assertTrue(set(CASES["assembly"]["capsule"]["reference_roles"]).issubset({p.role for p in assembly.parts}))
            point = np.array((-.8, .75, 1.0) if layout in ("J", "S") else (.8, .75, 1.0) if layout == "L" else (.45 if layout == "Backslash" else -.45, .45, 1.0))
            for part in assembly.parts:
                if part.role not in ("wetroom_enclosure", "media_cabinet", "cabinet"):
                    continue
                v = np.asarray(part.solid.vertices)
                self.assertFalse(np.all(point > v.min(axis=0) + 1e-8) and np.all(point < v.max(axis=0) - 1e-8), (layout, part.key))

    def test_whole_door_approaches_are_clear_and_top_heads_fit(self):
        for tower in M.towers():
            parts = [p for p in M.core_assembly(tower).parts if p.role in ("floor", "landing", "stair_tread", "stair_rail")]
            for slot in (s for s in M.slots() if s.tower == tower.index):
                self.assertLessEqual(slot.facade_plane.origin[2] + M.value("detail.door_height").value, tower.height)
                volume = M.place(M._box(-.36, .36, -CASES["assembly"]["core"]["approach_depth"], -.02, .01, 1.95), slot.facade_plane)
                av = np.asarray(volume.vertices)
                for part in parts:
                    bv = np.asarray(part.solid.vertices)
                    overlap = np.minimum(av.max(axis=0), bv.max(axis=0)) - np.maximum(av.min(axis=0), bv.min(axis=0))
                    self.assertFalse(np.all(overlap > 1e-7), (slot.path, slot.item, part.key))

    def test_podium_core_holes_pilotis_and_foundations(self):
        podium = M.podium_assembly()
        for tower in M.towers():
            for part in podium.parts:
                if part.role not in ("floor", "roof"):
                    continue
                v = np.asarray(part.solid.vertices)
                point = rh.Point3d(*tower.center, float(v[:, 2].mean()))
                box = rh.BoundingBox(*(v.min(axis=0) + 1e-7), *(v.max(axis=0) - 1e-7))
                self.assertFalse(box.Contains(point), part.key)
        self.assertTrue(all(np.asarray(p.solid.vertices)[:, 2].min() >= 4 for p in podium.parts if p.role == "podium_glazing"))
        foundation = M.foundation_assembly()
        self.assertTrue(set(CASES["assembly"]["foundation_roles"]).issubset({p.role for p in foundation.parts}))
        for part in foundation.parts:
            self.assertGreater(signed_volume(part.solid), 0, part.key)
        for tower in M.towers():
            for phase, fraction in enumerate(CASES["assembly"]["core"]["basement_turn_fractions"], 1):
                platform = next(p for p in foundation.parts if p.key == f"{tower.key}-basement-turning-landing-0-{phase}-0")
                bounds = np.asarray(platform.solid.vertices)
                self.assertAlmostEqual(bounds[:, 2].max(), -M.value("foundation.basement_depth").value * (1 - fraction))
                np.testing.assert_allclose(np.ptp(bounds[:, :2], axis=0), CASES["assembly"]["core"]["basement_turn_extents"][phase - 1])

    def test_roof_vent_curb_bears_on_shaft_and_retains_its_aperture(self):
        expected = CASES["assembly"]["core"]["vent_curb"]
        for tower in M.towers():
            parts = M.core_assembly(tower).parts
            curbs = [np.asarray(p.solid.vertices) for p in parts if p.role == "vent_support"]
            shaft = [np.asarray(p.solid.vertices) for p in parts if p.role == "lift_shaft"]
            vent = np.asarray(next(p for p in parts if p.key == "roof-vent").solid.vertices)
            self.assertEqual(len(curbs), expected["parts"])
            for curb in curbs:
                self.assertAlmostEqual(curb[:, 2].max(), vent[:, 2].min())
                self.assertTrue(any(abs(curb[:, 2].min() - wall[:, 2].max()) < 1e-8 and np.all(np.minimum(curb[:, :2].max(axis=0), wall[:, :2].max(axis=0)) - np.maximum(curb[:, :2].min(axis=0), wall[:, :2].min(axis=0)) > 1e-8) for wall in shaft))
                self.assertFalse(np.all(np.array(tower.center) > curb[:, :2].min(axis=0)) and np.all(np.array(tower.center) < curb[:, :2].max(axis=0)))
            lo, hi = np.array(tower.center) - np.array(expected["opening"]) / 2, np.array(tower.center) + np.array(expected["opening"]) / 2
            self.assertTrue(all(not np.all(np.minimum(hi, curb[:, :2].max(axis=0)) - np.maximum(lo, curb[:, :2].min(axis=0)) > 1e-8) for curb in curbs))

    def test_roof_tanks_bear_on_frames_connected_to_the_roof_ring(self):
        expected = CASES["assembly"]["capital"]["tank_support"]
        for tower in M.towers():
            capital = M.capital_assembly(tower).parts
            supports = [p for p in capital if p.role == "tank_support"]
            self.assertEqual(len(supports), expected["count_per_tower"])
            tank = next(p for p in capital if p.role == "water_tank")
            tv = np.asarray(tank.solid.vertices)
            cradle = np.asarray(next(p for p in supports if p.key == "tank-cradle").solid.vertices)
            self.assertAlmostEqual(tv[:, 2].min(), cradle[:, 2].max())
            self.assertAlmostEqual(tv[:, 2].min() - tower.height, .2 + expected["clearance_lift"])
            self.assertGreaterEqual(np.ptp(cradle[:, 0]), expected["minimum_contact_width"])
            self.assertGreaterEqual(np.ptp(cradle[:, 1]), expected["minimum_contact_length"])
            side = -1 if tower.key == "A" else 1
            for dx, dy in expected["bearing_fractions"]:
                point = np.array((tower.center[0] + side * (.9 + dx), tower.center[1] + .2 + dy))
                self.assertTrue(np.all(point >= cradle[:, :2].min(axis=0) - 1e-8))
                self.assertTrue(np.all(point <= cradle[:, :2].max(axis=0) + 1e-8))
                self.assertLess(dx * dx + dy * dy, .6 ** 2)
            for i in range(2):
                post = np.asarray(next(p for p in supports if p.key == f"tank-post-{i}").solid.vertices)
                beam = np.asarray(next(p for p in supports if p.key == f"tank-beam-{i}").solid.vertices)
                self.assertAlmostEqual(post[:, 2].max(), beam[:, 2].min())
                self.assertAlmostEqual(beam[:, 2].max(), cradle[:, 2].min())
                for first, second in ((post, beam), (beam, cradle)):
                    self.assertTrue(np.all(np.minimum(first[:, :2].max(axis=0), second[:, :2].max(axis=0)) - np.maximum(first[:, :2].min(axis=0), second[:, :2].min(axis=0)) > 1e-8))
            roofs = [np.asarray(p.solid.vertices) for p in M.core_assembly(tower).parts if p.role == "roof"]
            for post in (p for p in supports if p.key.startswith("tank-post")):
                pv = np.asarray(post.solid.vertices)
                self.assertTrue(any(abs(pv[:, 2].min() - rv[:, 2].max()) < 1e-8 and np.all(np.minimum(pv[:, :2].max(axis=0), rv[:, :2].max(axis=0)) - np.maximum(pv[:, :2].min(axis=0), rv[:, :2].min(axis=0)) > 0) for rv in roofs))
            for p in supports:
                vertices = np.asarray(p.solid.vertices)
                self.assertGreaterEqual(np.abs(vertices[:, 0] - tower.center[0]).min(), expected["shaft_clear_half_width"])

    def test_bridge_approaches_share_the_actual_deck_centreline(self):
        for tower in M.towers():
            approaches = [part for part in M.core_assembly(tower).parts if part.role == "bridge_approach"]
            self.assertEqual(len(approaches), 3)
            for part in approaches:
                vertices = np.asarray(part.solid.vertices)
                self.assertAlmostEqual((vertices[:, 1].min() + vertices[:, 1].max()) / 2, CASES["assembly"]["core"]["bridge_center_y"])

    def test_turning_stair_centreline_has_required_headroom(self):
        headroom = CASES["assembly"]["core"]["required_headroom"]
        for tower in M.towers():
            parts = M.core_assembly(tower).parts
            overhead = [p for p in parts if p.role in ("floor", "landing", "stair_tread", "roof")]
            boxes = np.array([[np.asarray(p.solid.vertices).min(axis=0), np.asarray(p.solid.vertices).max(axis=0)] for p in overhead])
            for part in (p for p in parts if p.role == "stair_tread"):
                v = np.asarray(part.solid.vertices)
                point = v[:, :2].mean(axis=0)
                top = v[:, 2].max()
                above = np.all(point > boxes[:, 0, :2] + 1e-7, axis=1) & np.all(point < boxes[:, 1, :2] - 1e-7, axis=1) & (boxes[:, 0, 2] > top + .3) & (boxes[:, 0, 2] < top + headroom)
                self.assertFalse(above.any(), (tower.key, part.key, [overhead[i].key for i in np.flatnonzero(above)]))

    def test_capsules_are_above_and_clear_of_the_podium(self):
        podium = M.podium_assembly().parts
        roof = M.podium_height()
        for slot in M.slots():
            av = np.asarray(M.slot_solid(slot).vertices)
            self.assertGreaterEqual(av[:, 2].min(), roof - 1e-8)
            for part in podium:
                bv = np.asarray(part.solid.vertices)
                overlap = np.minimum(av.max(axis=0), bv.max(axis=0)) - np.maximum(av.min(axis=0), bv.min(axis=0))
                self.assertFalse(np.all(overlap > 1e-7), (slot.path, slot.item, part.key))

    def test_side_entry_apertures_clear_the_lower_rounded_corner(self):
        expected = CASES["assembly"]["capsule"]["side_entry_aperture"]
        for layout in expected["families"]:
            parts = [p for p in M.capsule_assembly("A", layout=layout).parts if p.role in ("shell", "lining", "steel_truss")]
            sign = 1 if layout == "L" else -1
            for y in expected["width_samples"]:
                for z in expected["clear_heights"]:
                    start, end = (sign * 1.27, y, z), (sign * 1.1, y, z)
                    self.assertFalse(any(segment_hits_solid(p.solid, start, end) for p in parts), (layout, y, z))
            for y, z in ((expected["outside_y"], .22), (expected["y"], expected["solid_header_height"])):
                self.assertTrue(any(segment_hits_solid(p.solid, (sign * 1.27, y, z), (sign * 1.1, y, z)) for p in parts), (layout, y, z))

    def test_ground_core_entries_have_full_width_and_headroom(self):
        expected = CASES["assembly"]["core"]["ground_entry"]
        for tower in M.towers():
            walls = [p for p in M.core_assembly(tower).parts if p.role == "core_wall"]
            cx, cy = tower.center
            for x in expected["clear_width_samples"]:
                for z in expected["clear_height_samples"]:
                    start, end = (cx + x, cy - tower.core_side / 2 - .1, z), (cx + x, cy - tower.core_side / 2 + .4, z)
                    self.assertFalse(any(segment_hits_solid(p.solid, start, end) for p in walls), (tower.key, x, z))
            for x in (-.7, .7):
                self.assertTrue(any(segment_hits_solid(p.solid, (cx + x, cy - 2.85, 1), (cx + x, cy - 2.35, 1)) for p in walls), (tower.key, x))

    def test_study_primary_footprints_reproduce_every_authored_dimension_and_count(self):
        for t in M.capsule_types():
            for layout in M.capsule_families():
                assembly = M.capsule_assembly(t, layout=layout, interior="study")
                primaries = [p for p in assembly.parts if p.solid.meta.get("primary_footprint")]
                self.assertEqual(len(primaries), sum(row[2] for row in CASES["program_geometry"][t.key].values()))
                area = 0
                boxes = []
                for key, (width, length, count) in CASES["program_geometry"][t.key].items():
                    parts = [p for p in primaries if p.solid.meta["program_fitting"] == key]
                    self.assertEqual(len(parts), count, (t.key, layout, key))
                    for part in parts:
                        v = np.asarray(part.solid.vertices)
                        dims = np.ptp(v[:, :2], axis=0)
                        self.assertTrue(np.allclose(sorted(dims), sorted((width, length))), (t.key, layout, key, dims))
                        area += float(np.prod(dims))
                        lo, hi = v.min(axis=0), v.max(axis=0)
                        self.assertTrue(np.all(lo[:2] >= (-1.15, .1)) and np.all(hi[:2] <= (1.15, 3.9)), (t.key, layout, key))
                        wall = CASES["assembly"]["capsule"]["families"][layout]["door_wall"]
                        half = CASES["assembly"]["capsule"]["study_entry_clearance"]["width"] / 2
                        depth = CASES["assembly"]["capsule"]["study_entry_clearance"]["depth"]
                        if wall == "back":
                            dx = .45 if layout == "Backslash" else -.45
                            entry_lo, entry_hi = np.array([dx - half, 0]), np.array([dx + half, depth])
                        elif wall == "left":
                            entry_lo, entry_hi = np.array([-1.25, .75 - half]), np.array([-1.25 + depth, .75 + half])
                        else:
                            entry_lo, entry_hi = np.array([1.25 - depth, .75 - half]), np.array([1.25, .75 + half])
                        self.assertFalse(np.all(np.minimum(hi[:2], entry_hi) - np.maximum(lo[:2], entry_lo) > 1e-8), (t.key, layout, key, "entry"))
                        boxes.append((lo, hi, key))
                self.assertAlmostEqual(area, CASES["program"][t.key]["fitting_area"])
                for i, (lo, hi, key) in enumerate(boxes):
                    for other_lo, other_hi, other_key in boxes[i + 1:]:
                        self.assertFalse(np.all(np.minimum(hi[:2], other_hi[:2]) - np.maximum(lo[:2], other_lo[:2]) > 1e-8), (t.key, layout, key, other_key))

    def test_finish_floor_supports_reference_furniture_and_joins_thresholds(self):
        for layout in CASES["assembly"]["capsule"]["families"]:
            assembly = M.capsule_assembly("A", layout=layout)
            floor = next(p.solid for p in assembly.parts if p.role == "floor_finish")
            threshold = next(p.solid for p in assembly.parts if p.role == "entry_threshold")
            fv, tv = np.asarray(floor.vertices), np.asarray(threshold.vertices)
            self.assertAlmostEqual(fv[:, 2].max(), CASES["assembly"]["capsule"]["finish_support"]["floor_height"])
            overlap = np.minimum(fv.max(axis=0), tv.max(axis=0)) - np.maximum(fv.min(axis=0), tv.min(axis=0))
            self.assertTrue(np.all(overlap > 1e-8), layout)
            for part in assembly.parts:
                if part.role not in ("bed", "bathroom", "media_cabinet"):
                    continue
                v = np.asarray(part.solid.vertices)
                if abs(v[:, 2].min() - .2) < 1e-8:
                    self.assertTrue(np.all(v[:, :2].min(axis=0) >= fv[:, :2].min(axis=0) - 1e-8), (layout, part.key))
                    self.assertTrue(np.all(v[:, :2].max(axis=0) <= fv[:, :2].max(axis=0) + 1e-8), (layout, part.key))

    def test_all_threshold_collars_touch_the_authored_core_door_plane(self):
        for slot in M.slots():
            threshold = next(p.solid for p in M.slot_assembly(slot).parts if p.role == "entry_threshold")
            v = np.asarray(threshold.vertices)
            port = np.array(slot.facade_plane.origin) - np.array([0, 0, .09])
            self.assertTrue(np.all(port >= v.min(axis=0) - 1e-7) and np.all(port <= v.max(axis=0) + 1e-7), (slot.path, slot.item))
            box = rh.BoundingBox(*(v.min(axis=0) - 1e-7), *(v.max(axis=0) + 1e-7))
            self.assertTrue(box.Contains(rh.Point3d(*port)))

    def test_basement_core_walls_connect_footings_to_ground_core(self):
        foundation = M.foundation_assembly().parts
        depth = M.value("foundation.basement_depth").value
        for tower in M.towers():
            walls = [p for p in foundation if p.key.startswith(tower.key + "-basement-core-") and p.role == "basement_core_wall"]
            self.assertEqual(len(walls), 4)
            footings = [p for p in foundation if p.key.startswith(tower.key + "-footing-")]
            for wall in walls:
                v = np.asarray(wall.solid.vertices)
                self.assertAlmostEqual(v[:, 2].min(), -depth)
                self.assertAlmostEqual(v[:, 2].max(), 0)
                supported = any(np.all(np.minimum(v[:, :2].max(axis=0), np.asarray(p.solid.vertices)[:, :2].max(axis=0)) - np.maximum(v[:, :2].min(axis=0), np.asarray(p.solid.vertices)[:, :2].min(axis=0)) > 1e-6) for p in footings)
                self.assertTrue(supported, wall.key)
            floors = [p for p in foundation if p.key.startswith(tower.key + "-basement-core-floor-")]
            self.assertEqual(len(floors), 4)
            point = np.array((*tower.center, -depth - .1))
            for part in floors:
                v = np.asarray(part.solid.vertices)
                self.assertFalse(np.all(point > v.min(axis=0)) and np.all(point < v.max(axis=0)), part.key)

    def test_building_reports_progress_and_cancels(self):
        expected = CASES["assembly"]["progress"]
        with self.assertRaises(M.AssemblyCancelled):
            M.building_assembly(cancelled=lambda: True)
        events = []
        with self.assertRaises(M.AssemblyCancelled):
            M.building_assembly(progress=events.append, cancelled=lambda: any(e.phase == "capsules" and e.completed >= expected["cancel_after_capsules"] for e in events))
        self.assertEqual(events[-1].completed, expected["cancel_after_capsules"])
        events = []
        assembly = M.building_assembly(progress=events.append)
        self.assertEqual(events[-1].phase, "complete")
        self.assertEqual((events[-1].completed, events[-1].total), (len(assembly.parts), len(assembly.parts)))
        self.assertEqual({e.phase for e in events}, set(expected["phases"]))
        self.assertEqual(sum(e.phase == "capsules" and e.completed > 0 for e in events), expected["capsules"])

    def test_core_parts_and_bridge_structure(self):
        expected = CASES["assembly"]["core"]
        for tower in M.towers():
            parts = M.core_assembly(tower).parts
            roles = [p.role for p in parts]
            self.assertTrue(set(expected["required_roles"]).issubset(roles))
            self.assertGreater(roles.count("stair_tread"), tower.storeys * 12)
            self.assertEqual(tower.storeys, expected["storeys"][tower.key])
            for part in parts:
                self.assertGreater(signed_volume(part.solid), 0, part.key)
        bridges = [p for p in M.structure_assembly().parts if p.role == "bridge"]
        self.assertEqual(len(bridges), CASES["assembly"]["building"]["bridges"])
        for part, storey in zip(bridges, CASES["assembly"]["building"]["bridge_storeys"]):
            bounds = np.asarray(part.solid.vertices)
            self.assertAlmostEqual(bounds[:, 2].max(), M.bridge_elevation(storey))

if __name__ == "__main__":
    unittest.main(verbosity=2)
