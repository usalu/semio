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
        special = {e["storey"]: e["facades"] for e in tower["exceptions"]}
        for storey in range(tower["capsule_storeys"]["value"]):
            for facade in special.get(storey, tower["facades"]):
                branches.append((0, ti, storey, facade))
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


class DataTests(unittest.TestCase):
    def test_load_and_value_paths(self):
        self.assertEqual(M.load()["version"], 1)
        v = M.value("capsule.width")
        self.assertEqual((v.value, v.unit, v.basis, v.source), (2.5, "m", "sourced", "wikipedia"))
        self.assertEqual(M.value("types.D.rotation").value, 5.0)
        self.assertEqual(M.value("towers.1.capsule_storeys").value, 11)
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
        self.assertEqual(len({t.window_diameter for t in types}), 2)
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
        self.assertTrue(all(len(items) == 2 for items in tr.values()))
        self.assertEqual([len(tr[b]) for b in sorted(tr)], [2] * 70)
        self.assertEqual(per_tower, CASES["counts"]["branches_per_tower"])
        self.assertEqual({k: sum(len(v) for p, v in tr.items() if p[1] == i) for i, k in enumerate("AB")}, CASES["counts"]["per_tower"])
        for path in CASES["absent"]:
            self.assertNotIn(tuple(path["path"]), tr)

    def test_slots_follow_tree_order_and_paths(self):
        slots = M.slots()
        self.assertEqual([s.path for s in slots[::2]], sorted({s.path for s in slots}))
        self.assertEqual([(s.item) for s in slots[:4]], [0, 1, 0, 1])
        self.assertTrue(all(s.building == 0 for s in slots))
        self.assertEqual({s.facade for s in slots if s.tower == 0}, {0, 1, 2})
        self.assertEqual(max(s.storey for s in slots if s.tower == 1), 10)
        self.assertEqual({s.facade for s in slots if s.tower == 1 and s.storey == 10}, {0, 1, 2})

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

    def test_capsule_back_face_sits_on_core_facade(self):
        for s in M.slots():
            tower = M.towers()[s.tower]
            placed = M.slot_solid(s)
            back = np.asarray(placed.bolts)
            half = tower.core_side / 2
            normal = np.array(RAW["facades"][s.facade]["normal"] + [0])
            distance = (back[:, :2] - np.array(tower.center)) @ normal[:2]
            self.assertTrue(np.all(np.abs(distance - half) < (1e-9 if s.rotation == 0 else 0.2)))
            self.assertTrue(np.all(back[:, 2] > tower.base_elevation + s.storey * tower.storey_height))

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
            self.assertAlmostEqual(depth.min(), 0.0, places=9)
            touching += 1
            if s.rotation:
                self.assertAlmostEqual(abs(np.dot(s.plane.y, s.facade_plane.y)), math.cos(math.radians(abs(s.rotation))))
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
            self.assertAlmostEqual(cap.meta["base_side"], 5.5 + 0.5)
            self.assertAlmostEqual(cap.meta["roof_side"], 5.0)
            h, a, b = tower.capital_height, cap.meta["base_side"] ** 2, cap.meta["roof_side"] ** 2
            self.assertAlmostEqual(signed_volume(cap), h / 3 * (a + b + math.sqrt(a * b)))
        p = np.asarray(M.podium_solid().vertices)
        self.assertTrue(np.allclose(p.max(axis=0) - p.min(axis=0), [31.4, 20.7, 5.5]))
        self.assertAlmostEqual(signed_volume(M.podium_solid()), 31.4 * 20.7 * 5.5)

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


if __name__ == "__main__":
    unittest.main(verbosity=2)
