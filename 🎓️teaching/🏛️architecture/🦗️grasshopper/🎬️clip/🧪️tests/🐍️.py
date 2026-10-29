"""🧪 Portable Grasshopper engine contracts with independent oracles (tree walk, NumPy, rhino3dm)."""

import importlib.util
import itertools
import json
import math
import re
import sys
import unittest
from collections import defaultdict
from pathlib import Path

import numpy as np
import rhino3dm as rh

sys.dont_write_bytecode = True
CLIP = Path(__file__).resolve().parent.parent
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
MODEL = json.loads((CLIP / "🧱️model/🔣️.json").read_text(encoding="utf-8"))
TOL = CASES["tolerance"]
sys.path.insert(0, str(next(parent for parent in CLIP.parents if (parent / "🧰️framework").is_dir())))


def load_engine():
    """🧱 Load the engine without importing drawing or speech engines."""
    spec = importlib.util.spec_from_file_location("grasshopper_engine_under_test", CLIP / "🧱️model/🐍️.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


GH = load_engine()


def key(path):
    return "{" + ";".join(map(str, path)) + "}"


def oracle_paths(text_dict):
    """🧭 Parse {a;b} keys with a regex into tuples (independent of the engine parser)."""
    return {tuple(int(n) for n in re.findall(r"\d+", k)): list(v) for k, v in text_dict.items()}


def oracle_text(d):
    return {"{" + ";".join(str(i) for i in p) + "}": list(v) for p, v in d.items()}


def o_flatten(d, path=(0,)):
    return {tuple(path): [x for v in d.values() for x in v]}


def o_graft(d):
    return {p + (i,): [x] for p, v in d.items() for i, x in enumerate(v)}


def o_simplify(d, mode="all"):
    paths = list(d)
    if len(paths) == 1:
        return {(0,): d[paths[0]]}
    cols = list(zip(*paths))
    width = len(cols)
    const = [len(set(c)) == 1 for c in cols]
    lead = 0
    if mode in ("all", "front"):
        while lead < width - 1 and const[lead]:
            lead += 1
    trail = 0
    if mode in ("all", "back"):
        while trail < width - 1 - lead and const[width - 1 - trail]:
            trail += 1
    return {p[lead:width - trail]: v for p, v in d.items()}


def o_flip(d):
    rows = list(d.values())
    return {(j,): [c for c in col] for j, col in enumerate(itertools.zip_longest(*rows))}


def o_entwine(trees):
    return {p + (k,): v for k, t in enumerate(trees) for p, v in t.items()}


MAPPING = re.compile(r"^\{(.*?)\}(?:\((.*?)\))?\s*(?:->|→)\s*\{(.*?)\}(?:\((.*?)\))?$")


def o_map(d, mapping):
    """🗺️ Path Mapper by eval with an empty builtin table (independent of the engine's AST walk)."""
    left, item_name, right, right_index = MAPPING.match(mapping.strip()).groups()
    names = [n.strip() for n in left.split(";")]
    targets = [t.strip() for t in right.split(";")] if right.strip() else []
    groups = defaultdict(list)
    for n, (path, items) in enumerate(d.items()):
        if len(path) != len(names):
            continue
        env = {"path_count": len(d), "item_count": len(items), "path_index": n}
        if any(name.isdigit() and int(name) != i for name, i in zip(names, path)):
            continue
        env.update({name: i for name, i in zip(names, path) if not name.isdigit()})
        for i, item in enumerate(items):
            local = dict(env, **({item_name.strip(): i} if item_name else {}))
            target = tuple(eval(t.replace("/", "//"), {"__builtins__": {}}, local) for t in targets)
            order = eval(right_index.replace("/", "//"), {"__builtins__": {}}, local) if right_index else 0
            groups[target].append((order, item))
    return {p: [x for _, x in (sorted(rows, key=lambda r: r[0]) if right_index else rows)] for p, rows in groups.items()}


def o_longest_list(columns):
    """📏 Longest-list matching: shorter lists repeat their last item."""
    n = max(len(c) for c in columns)
    return [tuple(c[min(i, len(c) - 1)] for c in columns) for i in range(n)]


def approx(a, b, strict=False, tol=TOL):
    if isinstance(b, dict) and isinstance(a, dict):
        return a.keys() == b.keys() and all(approx(a[k], b[k], strict, tol) for k in b)
    if isinstance(b, (list, tuple)) and isinstance(a, (list, tuple)):
        return len(a) == len(b) and all(approx(x, y, strict, tol) for x, y in zip(a, b))
    if b is None or a is None:
        return a is b
    if isinstance(b, bool) or isinstance(a, bool):
        return isinstance(a, bool) and isinstance(b, bool) and a == b
    if isinstance(b, str):
        return a == b
    if strict and isinstance(b, int) != isinstance(a, int):
        return False
    return abs(a - b) <= tol * max(1.0, abs(b))


def literal(x):
    return GH.decode(x) if isinstance(x, dict) else x


def pins(canvas, extra=None):
    return {**CASES["canvases"]["pins"][canvas], **(extra or {})}


def build_tree(d):
    return GH.Tree.from_dict({k: [literal(i) for i in v] for k, v in d.items()})


def rh_plane(p):
    return rh.Plane(rh.Point3d(*p["origin"]), rh.Vector3d(*p["x"]), rh.Vector3d(*p["y"]))


def random_plane(rng):
    q, _ = np.linalg.qr(rng.normal(size=(3, 3)))
    if np.linalg.det(q) < 0:
        q[:, 2] *= -1
    o = rng.uniform(-5, 5, 3)
    return {"origin": o.tolist(), "x": q[:, 0].tolist(), "y": q[:, 1].tolist(), "z": q[:, 2].tolist()}


def engine_plane(p):
    return GH.Plane(GH.Point(*p["origin"]), GH.Vector(*p["x"]), GH.Vector(*p["y"]), GH.Vector(*p["z"]))


def corners(box_plain):
    pl = box_plain["plane"]
    o, x, y, z = (np.array(pl[k]) for k in ("origin", "x", "y", "z"))
    return np.array([o + a * x + b * y + c * z for a in box_plain["x"] for b in box_plain["y"] for c in box_plain["z"]])


class TestPaths(unittest.TestCase):
    def test_format_and_parse(self):
        for case in CASES["paths"]:
            self.assertEqual(GH.format_path(tuple(case["path"])), case["text"])
            self.assertEqual(GH.parse_path(case["text"]), tuple(case["path"]))

    def test_invalid_paths(self):
        for text in ("0;1", "{a}", "{0;}", "{;0}", "{-1}"):
            with self.assertRaises(ValueError, msg=text):
                GH.parse_path(text)


class TestTrees(unittest.TestCase):
    def run_engine(self, case):
        t = GH.Tree.from_dict(case["tree"]) if "tree" in case else None
        a = case.get("args", {})
        op = case["op"]
        if op == "flatten":
            return t.flatten(tuple(a.get("path", (0,))))
        if op == "simplify":
            return t.simplify(a.get("mode", "all"))
        if op == "entwine":
            return GH.Tree.entwine([GH.Tree.from_dict(x) for x in case["trees"]])
        if op == "merge":
            return t.merge(GH.Tree.from_dict(case["other"]))
        if op == "explode":
            return [[list(p), list(v)] for p, v in t.explode()]
        if op == "stats":
            s = t.stats()
            return {**s, "counts": {str(p): n for p, n in s["counts"].items()}}
        if op == "item":
            return t.item(tuple(a["path"]), a["index"], a["wrap"])
        return getattr(t, op)()

    def oracle(self, case):
        d = oracle_paths(case["tree"]) if "tree" in case else None
        a = case.get("args", {})
        op = case["op"]
        if op == "flatten":
            return oracle_text(o_flatten(d, tuple(a.get("path", (0,)))))
        if op == "graft":
            return oracle_text(o_graft(d))
        if op == "simplify":
            return oracle_text(o_simplify(d, a.get("mode", "all")))
        if op == "flip":
            return oracle_text(o_flip(d))
        if op == "entwine":
            return oracle_text(o_entwine([oracle_paths(x) for x in case["trees"]]))
        return None

    def test_cases_against_engine_and_oracle(self):
        for case in CASES["trees"]:
            with self.subTest(case["id"]):
                if case.get("error"):
                    with self.assertRaises((ValueError, IndexError)):
                        self.run_engine(case)
                    continue
                got = self.run_engine(case)
                if isinstance(got, GH.Tree):
                    got = got.to_dict()
                self.assertEqual(got, case["expected"])
                oracle = self.oracle(case)
                if oracle is not None:
                    self.assertEqual(oracle, case["expected"])

    def test_graft_then_flatten_keeps_items(self):
        rng = np.random.default_rng(3)
        for _ in range(20):
            d = {(0, i): rng.integers(0, 9, rng.integers(1, 5)).tolist() for i in range(rng.integers(1, 6))}
            t = GH.Tree({p: v for p, v in d.items()})
            self.assertEqual(t.graft().flatten().items(), tuple(x for v in d.values() for x in v))
            self.assertEqual(t.graft().to_dict(), oracle_text(o_graft(d)))
            if len({len(v) for v in d.values()}) == 1:
                self.assertEqual(t.flip().to_dict(), oracle_text(o_flip(d)))

    def test_simplify_matches_oracle_on_random_equal_depth(self):
        rng = np.random.default_rng(11)
        for _ in range(60):
            width = int(rng.integers(2, 5))
            consts = rng.integers(0, 2, width)
            paths = set()
            for _ in range(int(rng.integers(2, 7))):
                paths.add(tuple(int(consts[c] * 3) if consts[c] else int(rng.integers(0, 3)) for c in range(width)))
            d = {p: [i] for i, p in enumerate(sorted(paths))}
            if len(d) < 2:
                continue
            for mode in ("all", "front", "back"):
                self.assertEqual(GH.Tree(d).simplify(mode).to_dict(), oracle_text(o_simplify(d, mode)), (d, mode))

    def test_double_flip_is_identity_for_rectangular(self):
        d = {(0,): [1, 2, 3], (1,): [4, 5, 6]}
        self.assertEqual(GH.Tree(d).flip().flip().to_dict(), oracle_text(d))

    def test_mapper_cases(self):
        for case in CASES["mapper"]:
            with self.subTest(case["id"]):
                engine = GH.Tree.from_dict(case["tree"]).map(case["mapping"]).to_dict()
                self.assertEqual(engine, case["expected"])
                self.assertEqual(oracle_text(o_map(oracle_paths(case["tree"]), case["mapping"])), case["expected"])

    def test_mapper_two_argument_form_and_errors(self):
        t = GH.Tree.from_dict({"{0;1}": [1]})
        self.assertEqual(t.map("{A;B}(i)", "{B}").to_dict(), {"{1}": [1]})
        with self.assertRaises(ValueError):
            GH.Tree.from_dict({"{0}": [1]}).map("{A} to {A}")
        with self.assertRaises(ValueError):
            GH.Tree.from_dict({"{0}": [1]}).map("{A}(i) -> {A+__import__('os')}")

    def test_oracle_longest_list(self):
        self.assertEqual(o_longest_list([[1, 2, 3], [10, 20]]), [(1, 10), (2, 20), (3, 20)])


class TestCasting(unittest.TestCase):
    def test_cases(self):
        for case in CASES["casts"]:
            with self.subTest(case["id"]):
                result = GH.cast(literal(case["value"]), case["target"], case.get("culture", "en"))
                self.assertEqual(result.ok, case["ok"])
                self.assertEqual(result.rule, case["rule"])
                if "basis" in case:
                    self.assertEqual(result.basis, case["basis"])
                if case["ok"]:
                    self.assertTrue(approx(GH.plain(result.value), case["expected"], strict=not isinstance(case["expected"], (list, dict))), (GH.plain(result.value), case["expected"]))
                else:
                    self.assertIsNone(result.value)
                    self.assertEqual(result.message["en"], case["message"])
                    self.assertIn(case["message"], result.message["de"])
                    self.assertTrue(result.message["de"].strip())

    def test_number_to_integer_matches_numpy_rint(self):
        rng = np.random.default_rng(5)
        values = np.concatenate([np.arange(-6, 7) + 0.5, rng.uniform(-50, 50, 200)])
        for v in values:
            self.assertEqual(GH.cast(float(v), "Integer").value, int(np.rint(v)), v)

    def test_text_to_number_matches_regex_float(self):
        pattern = re.compile(r"^\s*[-+]?(\d+(\.\d*)?|\.\d+)([eE][-+]?\d+)?\s*$")
        for text in ["2.5", "-3", "+4.", ".5", "1e3", "abc", "", "1,5", "1 2", "0x10", "nan", "inf", "--1", "2.5.1"]:
            r = GH.cast(text, "Number")
            self.assertEqual(r.ok, bool(pattern.match(text)), text)
            if r.ok:
                self.assertEqual(r.value, float(text))

    def test_boolean_roundtrip_with_integer(self):
        for b in (True, False):
            self.assertIs(GH.cast(GH.cast(b, "Integer").value, "Boolean").value, b)

    def test_line_to_plane_is_orthonormal_and_right_handed_in_rhino(self):
        rng = np.random.default_rng(9)
        for _ in range(25):
            a, b = rng.uniform(-4, 4, 3), rng.uniform(-4, 4, 3)
            plane = GH.cast(GH.Line(GH.Point(*a), GH.Point(*b)), "Plane").value
            m = np.array([plane.x.t, plane.y.t, plane.z.t])
            self.assertTrue(np.allclose(m @ m.T, np.eye(3), atol=1e-12))
            self.assertTrue(np.allclose(np.cross(m[0], m[1]), m[2], atol=1e-12))
            z = (b - a) / np.linalg.norm(b - a)
            self.assertTrue(np.allclose(m[2], z, atol=1e-12))
            r = rh.Plane(rh.Point3d(*a), rh.Vector3d(*m[0]), rh.Vector3d(*m[1]))
            self.assertTrue(np.allclose([r.ZAxis.X, r.ZAxis.Y, r.ZAxis.Z], z, atol=1e-6))

    def test_every_converter_has_a_documented_rule_and_sources(self):
        rules = MODEL["casting"]["rules"]
        for (src, dst) in GH._CONVERT:
            self.assertIn(f"{src}>{dst}", rules)
        for rid, rule in rules.items():
            self.assertIn(rule["source"], MODEL["casting"]["sources"], rid)
            self.assertTrue(rule["de"].strip() and rule["en"].strip())

    def test_basis_marks_follow_the_research(self):
        r = MODEL["casting"]["rules"]
        for rid in ("Number>Domain", "Circle>Curve", "Box>Brep", "Integer>Boolean", "Text>Boolean", "Text>Point", "Text>Number"):
            self.assertEqual(r[rid]["basis"], "unverified", rid)
        for rid in ("Boolean>Integer", "Boolean>Text", "Text>Number!", "Text>Integer!"):
            self.assertEqual(r[rid]["basis"], "verified", rid)
        for rid in ("Point>Plane", "Line>Plane"):
            self.assertEqual(r[rid]["basis"], "community", rid)
        self.assertEqual(r["Number>Integer"]["basis"], "conflicting")
        self.assertEqual(r["Number>Integer"]["source"], "rutten-2010")
        self.assertEqual(r["Number>Integer"]["sources"], ["rutten-2010", "rutten-2012"])
        for sid in r["Number>Integer"]["sources"]:
            self.assertIn(sid, MODEL["casting"]["sources"])
        self.assertIn("2.5 gives 2", r["Number>Integer"]["en"])

    def test_german_is_never_empty_and_keeps_the_english_message(self):
        for src, dst in (("x", "Number"), (None, "Point"), (3.0, "Line"), ("a", "Plane")):
            r = GH.cast(src, dst)
            if not r.ok:
                self.assertIn("Data conversion failed from", r.message["de"])
                self.assertIn("lässt den Platz leer", r.message["de"])


class TestSynthetic(unittest.TestCase):
    def test_cases(self):
        for case in CASES["synthetic"]:
            with self.subTest(case["id"]):
                defn = GH.define(case)
                sol = GH.evaluate(defn)
                for e in case.get("expect", []):
                    tree = sol.tree(e["object"], e["output"])
                    self.assertTrue(approx(GH.plain(tree), e["tree"]), (case["id"], GH.plain(tree), e["tree"]))
                for oid, state in case.get("expect_state", {}).items():
                    self.assertEqual(sol.state(oid), state, oid)
                for oid, text in case.get("expect_message", {}).items():
                    self.assertIn(text, [m["en"] for m in sol.msgs(oid)])
                    self.assertTrue(all(m["de"].strip() for m in sol.msgs(oid)))

    def test_matching_against_independent_longest_list_oracle(self):
        rng = np.random.default_rng(21)
        for _ in range(40):
            a = rng.integers(1, 9, int(rng.integers(1, 6))).tolist()
            b = rng.integers(1, 9, int(rng.integers(1, 6))).tolist()
            spec = {"objects": [{"id": "a", "component": "Panel", "settings": {"lines": [str(x) for x in a]}},
                                {"id": "b", "component": "Panel", "settings": {"lines": [str(x) for x in b]}},
                                {"id": "m", "component": "Multiplication"}], "wires": ["a.Stream>m.A", "b.Stream>m.B"]}
            got = GH.evaluate(GH.define(spec)).tree("m", "Result").items()
            self.assertEqual(list(got), [float(x * y) for x, y in o_longest_list([a, b])])

    def test_graft_matching_against_oracle(self):
        rng = np.random.default_rng(4)
        for _ in range(20):
            n = int(rng.integers(1, 5))
            b = rng.integers(1, 9, int(rng.integers(1, 5))).tolist()
            spec = {"objects": [{"id": "n", "component": "Panel", "settings": {"lines": [str(n)]}}, {"id": "s", "component": "Series"}, {"id": "g", "component": "Graft Tree"},
                                {"id": "b", "component": "Panel", "settings": {"lines": [str(x) for x in b]}}, {"id": "m", "component": "Addition"}],
                    "wires": ["n.Stream>s.Count", "s.Series>g.Tree", "g.Tree>m.A", "b.Stream>m.B"]}
            tree = GH.evaluate(GH.define(spec)).tree("m", "Result")
            expected = {(0, i): [float(x + i) for x in b] for i in range(n)}
            self.assertEqual({p: list(v) for p, v in tree._b.items()}, expected)

    def test_unknown_wire_endpoints_are_rejected(self):
        with self.assertRaises(ValueError):
            GH.define({"objects": [{"id": "a", "component": "Panel"}], "wires": ["a.Stream>zz.Stream"]})
        with self.assertRaises(KeyError):
            GH.define({"objects": [{"id": "a", "component": "Panel"}, {"id": "b", "component": "Panel"}], "wires": ["a.Nope>b.Stream"]})
        with self.assertRaises(ValueError):
            GH.define({"objects": [{"id": "a", "component": "Addition"}, {"id": "b", "component": "Addition"}], "wires": ["a.Result>b.A", "b.Result>a.A"]}).order()

    def test_overrides_by_key(self):
        spec = {"objects": [{"id": "s", "component": "Number Slider", "settings": {"min": 0, "max": 5, "value": 1, "rounding": "float", "decimals": 1}}], "wires": []}
        defn = GH.define(spec)
        self.assertEqual(GH.evaluate(defn, {"s": 3.14}).value("s"), 3.1)
        self.assertEqual(GH.evaluate(defn, {"s.max": 2, "s": 4}).value("s"), 2.0)


class TestCanvases(unittest.TestCase):
    def test_outputs(self):
        for case in CASES["canvases"]["outputs"]:
            with self.subTest(case["id"]):
                defn = GH.canvas(case["canvas"])
                sol = GH.evaluate(defn, pins(case["canvas"], {**(GH.tower_overrides(case["tower"]) if "tower" in case else {}), **case.get("overrides", {})}))
                name = case.get("output")
                tree = sol.tree(case["object"], name)
                if "tree" in case:
                    self.assertTrue(approx(GH.plain(tree), case["tree"]), (GH.plain(tree), case["tree"]))
                if "plain" in case:
                    self.assertTrue(approx(GH.plain(tree), case["plain"]))
                if "volumes" in case:
                    self.assertTrue(approx([b.volume() for b in tree.items()], case["volumes"], tol=1e-9))
                if "tags" in case:
                    self.assertEqual([b.tag["type"] for b in tree.items()], case["tags"])
                if "origins" in case:
                    self.assertTrue(approx([GH.plain(p.origin) for p in tree.items()], case["origins"], tol=1e-9))
                if "count" in case:
                    self.assertEqual(tree.count, case["count"])
                if "items" in case:
                    self.assertEqual(tree.count, case["items"])
                if "box_volume" in case:
                    self.assertAlmostEqual(tree.items()[0].volume if isinstance(tree.items()[0], GH.Box) else tree.items()[0].root_box().volume, case["box_volume"], places=9)
                if "prefix_text" in case:
                    self.assertEqual(list(tree.items()[:len(case["prefix_text"])]), case["prefix_text"])
                if "state" in case:
                    self.assertEqual(sol.state(case["object"]), case["state"])
                if "messages" in case:
                    self.assertEqual([m["en"] for m in sol.msgs(case["object"])], case["messages"])
                if "materials" in case:
                    count = defaultdict(int)
                    for b in tree.items():
                        count[b.tag["material"]] += 1
                    self.assertEqual(dict(count), case["materials"])

    def test_only_the_failing_demo_is_not_ok(self):
        for key_ in GH.canvas_keys():
            sol = GH.evaluate(GH.canvas(key_))
            bad = {o: sol.state(o) for o in sol.definition.objects if sol.state(o) != "ok"}
            self.assertEqual(bad, {"casting": {"cbx": "error", "tdn": "warning"}, "anatomy": {"cbw": "warning", "cbe": "error"}}.get(key_, {}), key_)

    def test_views_are_readable(self):
        for key_ in GH.canvas_keys():
            defn = GH.canvas(key_)
            for name, view in defn.views.items():
                self.assertLessEqual(len(view["objects"]), 12, (key_, name))
                self.assertEqual(set(view["layout"]), set(view["objects"]), (key_, name))
                pts = [o.at for o in defn.view(name)]
                self.assertEqual(len(set(pts)), len(pts), (key_, name, "overlapping positions"))
                self.assertTrue(defn.view_wires(name) or (key_, name) == ("casting", "types"), (key_, name, "a view without wires"))
                x0, y0, x1, y1 = defn.bounds(name)
                self.assertGreaterEqual(x1 - x0, 0)

    def test_view_flow_runs_left_to_right(self):
        for key_ in GH.canvas_keys():
            defn = GH.canvas(key_)
            for name in defn.views:
                at = {o.id: o.at for o in defn.view(name)}
                for w in defn.view_wires(name):
                    self.assertLessEqual(at[w.src][0], at[w.dst][0], (key_, name, w))

    def test_object_positions_exist_in_a_view(self):
        for key_ in GH.canvas_keys():
            defn = GH.canvas(key_)
            used = {i for v in defn.views.values() for i in v["objects"]}
            self.assertEqual(used, set(defn.objects), key_)

    def test_every_wire_is_type_castable_or_generic(self):
        for key_ in GH.canvas_keys():
            defn = GH.canvas(key_)
            for w in defn.wires:
                out = defn.spec(w.src).output(w.out).type
                inp = defn.spec(w.dst).input(w.inp).type
                self.assertTrue(out == inp or "Generic" in (out, inp) or inp == "Geometry" or (out, inp) in GH._CONVERT, (key_, w, out, inp))

    def test_five_canvases_with_the_required_story_objects(self):
        self.assertEqual(GH.canvas_keys(), ("anatomy", "capsule", "tower", "casting", "trees"))
        needs = {"anatomy": {"Number Slider", "Center Box", "Panel", "Series", "Param Viewer", "Number Param"},
                 "capsule": {"Center Box", "Fillet Edge", "Solid Difference", "Cylinder", "Circle", "Orient", "List Item", "Value List", "Merge", "Construct Point", "Rotate"},
                 "tower": {"Series", "Graft Tree", "Flatten Tree", "Move", "Orient", "Rectangle", "Extrude", "Tree Statistics", "Center Box"},
                 "casting": {"Floor", "Construct Plane", "Divide Domain", "Boolean Toggle", "Value List", "Orient"},
                 "trees": {"Path Mapper", "Flip Matrix", "Simplify Tree", "Graft Tree", "Flatten Tree", "Tree Statistics", "Param Viewer", "Custom Preview", "Tree Item"}}
        for key_, comps in needs.items():
            self.assertLessEqual(comps, {o.component for o in GH.canvas(key_).objects.values()}, key_)

    def test_slider_bounds_hold_the_pinned_value(self):
        for key_ in GH.canvas_keys():
            for o in GH.canvas(key_).objects.values():
                if o.component == "Number Slider":
                    self.assertLessEqual(o.settings["min"], o.settings["value"], (key_, o.id))
                    self.assertLessEqual(o.settings["value"], o.settings["max"], (key_, o.id))


class TestGeometry(unittest.TestCase):
    def test_orient_matrix_matches_rhino_plane_to_plane(self):
        rng = np.random.default_rng(1)
        for _ in range(30):
            a, b = random_plane(rng), random_plane(rng)
            mine = np.array(GH.plane_to_plane(engine_plane(a), engine_plane(b)))
            ref = np.array(rh.Transform.PlaneToPlane(rh_plane(a), rh_plane(b)).ToFloatArray(True)).reshape(4, 4)
            self.assertTrue(np.allclose(mine, ref, atol=1e-6))

    def test_rotation_matrix_matches_rhino(self):
        rng = np.random.default_rng(2)
        for _ in range(30):
            angle, axis, center = rng.uniform(-6, 6), rng.normal(size=3), rng.uniform(-3, 3, 3)
            mine = np.array(GH.rotation(angle, GH.Vector(*axis), GH.Point(*center)))
            ref = np.array(rh.Transform.Rotation(angle, rh.Vector3d(*(axis / np.linalg.norm(axis))), rh.Point3d(*center)).ToFloatArray(True)).reshape(4, 4)
            self.assertTrue(np.allclose(mine, ref, atol=1e-6))

    def test_translation_and_invert(self):
        m = GH.translation(GH.Vector(1, 2, 3))
        self.assertTrue(np.allclose(np.array(m)[:3, 3], [1, 2, 3]))
        rng = np.random.default_rng(6)
        a, b = random_plane(rng), random_plane(rng)
        m = GH.plane_to_plane(engine_plane(a), engine_plane(b))
        self.assertTrue(np.allclose(np.array(GH._mul(m, GH.invert(m))), np.eye(4), atol=1e-12))

    def test_planes_are_orthonormal_right_handed(self):
        for p in (GH.Plane.world_xy(), GH.Plane.world_xz(GH.Point(1, 2, 3)), GH.Plane.world_yz(), GH.Plane.from_axes(GH.Point(), GH.Vector(1, 1, 0), GH.Vector(0, 1, 1))):
            m = np.array([p.x.t, p.y.t, p.z.t])
            self.assertTrue(np.allclose(m @ m.T, np.eye(3), atol=1e-12))
            self.assertTrue(np.allclose(np.cross(m[0], m[1]), m[2]))
        xz = GH.Plane.world_xz()
        self.assertEqual(xz.z.t, (0.0, -1.0, 0.0))

    def test_capsule_chain_equals_direct_construction(self):
        sol = GH.evaluate(GH.canvas("capsule"), pins("capsule"))
        chain = sol.value("cut", "Result")
        t = GH.capsule_type("A")
        direct = GH.make_capsule(2.5, 4.0, 2.5, 0.25, 1.3, GH.capsule_type("A")["wall"])
        self.assertEqual(chain.solid, direct.solid)
        self.assertEqual(chain.xform, direct.xform)
        self.assertAlmostEqual(t["window_u"], 0.0)

    def test_rounded_box_volume_against_a_numpy_voxel_count(self):
        w, l, h, r = 2.5, 4.0, 2.5, 0.25
        n = (100, 160, 100)
        half = np.array([w, l, h]) / 2
        xs = [(np.arange(k) + 0.5) / k * 2 * half[i] - half[i] for i, k in enumerate(n)]
        inside = 0
        for x in xs[0]:
            gx, gy, gz = np.meshgrid([x], xs[1], xs[2], indexing="ij")
            q = np.maximum(np.stack([np.abs(gx), np.abs(gy), np.abs(gz)]) - (half[:, None, None, None] - r), 0)
            inside += int((np.sqrt((q ** 2).sum(axis=0)) <= r).sum())
        voxel = np.prod(2 * half / np.array(n))
        engine = GH.solid_volume({"kind": "fillet", "base": {"kind": "box", "box": GH.Box(GH.Plane.world_xy(), GH.Domain(-w / 2, w / 2), GH.Domain(-l / 2, l / 2), GH.Domain(-h / 2, h / 2))}, "radius": r})
        self.assertAlmostEqual(inside * voxel, engine, delta=0.004 * engine)

    def test_window_cut_volume_against_a_numpy_disc_grid(self):
        radius, depth, k = 0.65, 0.3, 800
        xs = (np.arange(k) + 0.5) / k * 2 * radius - radius
        gx, gy = np.meshgrid(xs, xs)
        area = float(((gx ** 2 + gy ** 2) <= radius ** 2).sum()) * (2 * radius / k) ** 2
        base = GH.solid_volume(GH.make_capsule(2.5, 4.0, 2.5, 0.25, 1.3, 0.3).solid["base"])
        full = GH.make_capsule(2.5, 4.0, 2.5, 0.25, 1.3, 0.3).volume()
        self.assertAlmostEqual(base - full, area * depth, delta=1e-3)

    def test_oriented_capsule_corners_match_rhino_transform(self):
        sol = GH.evaluate(GH.canvas("capsule"), pins("capsule"))
        planes = [GH.plain(p) for p in sol.items("rot", "Geometry")]
        capsules = sol.items("ori", "Geometry")
        for plane, capsule in zip(planes, capsules):
            local = GH.make_capsule_of(capsule.tag["type"]).root_box()
            local_corners = corners(GH.plain(local))
            xform = rh.Transform.PlaneToPlane(rh.Plane.WorldXY(), rh_plane(plane))
            expected = []
            for c in local_corners:
                p = rh.Point3d(*c)
                p.Transform(xform)
                expected.append([p.X, p.Y, p.Z])
            self.assertTrue(np.allclose(corners(GH.plain(capsule.root_box())), np.array(expected), atol=1e-6))

    def test_facade_planes_match_rhino_rotation(self):
        sol = GH.evaluate(GH.canvas("capsule"), pins("capsule"))
        planes = sol.items("rot", "Geometry")
        for k, plane in enumerate(planes):
            xf = rh.Transform.Rotation(math.radians(-90 * k), rh.Vector3d(0, 0, 1), rh.Point3d(0, 0, 0))
            o = rh.Point3d(0, 2.75, 0)
            o.Transform(xf)
            self.assertTrue(np.allclose(plane.origin.t, [o.X, o.Y, o.Z], atol=1e-6))
            x = rh.Vector3d(1, 0, 0)
            x.Transform(xf)
            self.assertTrue(np.allclose(plane.x.t, [x.X, x.Y, x.Z], atol=1e-6))

    def test_boxes_have_rhino_bounding_box_volumes(self):
        for cid, oid, out in (("tower", "core", "Box"), ("tower", "capital", "Box")):
            box = GH.evaluate(GH.canvas(cid), pins(cid)).value(oid, out)
            c = corners(GH.plain(box))
            bb = rh.BoundingBox(*c.min(axis=0), *c.max(axis=0))
            self.assertAlmostEqual(bb.Volume, box.volume, places=6)

    def test_window_circle_matches_rhino_circle(self):
        sol = GH.evaluate(GH.canvas("capsule"), pins("capsule"))
        circle = sol.value("cir", "Circle")
        plane = rh_plane(GH.plain(circle.plane))
        ref = rh.Circle(plane.Origin, circle.radius)
        self.assertAlmostEqual(ref.Radius, 0.65, places=6)
        self.assertAlmostEqual(ref.Circumference, 2 * math.pi * 0.65, places=5)
        self.assertTrue(np.allclose([plane.ZAxis.X, plane.ZAxis.Y, plane.ZAxis.Z], [0, -1, 0]))
        brep = GH.make_capsule_of("A")
        self.assertTrue(np.allclose(GH.plain(brep.window())["plane"]["origin"], [0, 4, 1.25]))

    def test_podium_extrusion_volume(self):
        v = GH.evaluate(GH.canvas("tower"), pins("tower")).value("ext", "Extrusion").volume()
        self.assertAlmostEqual(v, 31.4 * 20.7 * 5.5, places=9)


class TestNakagin(unittest.TestCase):
    def test_bound_sliders_read_the_nakagin_model(self):
        nak = GH._nakagin()
        for key_ in GH.canvas_keys():
            for o in GH.canvas(key_).objects.values():
                if "bind" in o.settings:
                    self.assertEqual(o.settings["value"], nak.value(o.settings["bind"]).value, (key_, o.id))

    def test_aggregated_tree_equals_nakagin_slots_by_an_independent_walk(self):
        nak = GH._nakagin()
        tree = GH.aggregate_tree()
        raw = json.loads((CLIP.parents[1] / "🏢️nakagin/🔣️.json").read_text(encoding="utf-8"))
        expected = defaultdict(int)
        for ti, tower in enumerate(raw["towers"]):
            special = {e["storey"]: e["facades"] for e in tower["exceptions"]}
            for s in range(tower["capsule_storeys"]["value"]):
                for f in special.get(s, tower["facades"]):
                    expected[(0, ti, s, f)] = raw["items"]["per_facade"]["value"]
        self.assertEqual({p: len(v) for p, v in tree._b.items()}, dict(expected))
        self.assertEqual((tree.path_count, tree.count), (70, 140))
        by_path = nak.tree()
        for path, slots in by_path.items():
            for slot, brep in zip(slots, tree.branch(path)):
                self.assertEqual(brep.tag["type"], slot.type)
                self.assertEqual(brep.tag["slot"], (slot.building, slot.tower, slot.storey, slot.facade, slot.item))
                local = np.array(GH.plain(GH.make_capsule_of(slot.type).root_box().center))
                world = np.array(slot.plane.origin) + local[0] * np.array(slot.plane.x) + local[1] * np.array(slot.plane.y) + local[2] * np.array(slot.plane.z)
                self.assertTrue(np.allclose(GH.plain(brep.root_box().center), world, atol=1e-9))

    def test_capsule_types_match_nakagin(self):
        nak = GH._nakagin()
        for t in nak.capsule_types():
            mine = GH.capsule_type(t.key)
            for field in ("width", "length", "height", "corner_radius", "window_diameter", "window_u", "window_v"):
                self.assertEqual(mine[field], getattr(t, field), (t.key, field))

    def test_uneven_top_storey_is_visible_in_the_tree_statistics(self):
        sol = GH.evaluate(GH.canvas("trees"))
        lengths = list(sol.items("pms", "Length"))
        self.assertEqual(sum(lengths), 140)
        self.assertEqual(lengths[-1], 6)
        self.assertEqual(lengths[-2], 8)
        self.assertEqual(sol.value("pts", "Length"), 54)


class TestTowerSlots(unittest.TestCase):
    def canvas_slots(self, key_):
        sol = GH.evaluate(GH.canvas("tower"), GH.tower_overrides(key_))
        return sol, sol.tree("slots", "Geometry")

    def test_canvas_reproduces_every_nakagin_slot_plane(self):
        nak = GH._nakagin()
        mine = []
        for key_ in ("A", "B"):
            sol, tree = self.canvas_slots(key_)
            self.assertEqual(sol.state("slots"), "ok")
            self.assertEqual([sol.state(o) for o in ("cull", "yrot", "ori")], ["ok"] * 3)
            mine += list(tree.items())
        theirs = nak.slots()
        self.assertEqual((len(mine), len(theirs)), (140, 140))
        self.assertEqual(sum(len(GH.evaluate(GH.canvas("tower"), GH.tower_overrides(k)).items("slots", "Geometry")) for k in "AB"), 54 + 86)
        for plane, slot in zip(mine, theirs):
            got = np.array([plane.origin.t, plane.x.t, plane.y.t, plane.z.t])
            want = np.array([slot.plane.origin, slot.plane.x, slot.plane.y, slot.plane.z])
            self.assertTrue(np.allclose(got, want, atol=1e-9, rtol=0), (slot.tower, slot.storey, slot.facade, slot.item))

    def test_oriented_capsules_have_the_slot_types_and_world_boxes(self):
        nak = GH._nakagin()
        caps = []
        for key_ in "AB":
            caps += list(self.canvas_slots(key_)[0].items("ori", "Geometry"))
        for cap, slot in zip(caps, nak.slots()):
            self.assertEqual(cap.tag["type"], slot.type)
            local = np.array(GH.plain(GH.make_capsule_of(slot.type).root_box().center))
            p = slot.plane
            world = np.array(p.origin) + local[0] * np.array(p.x) + local[1] * np.array(p.y) + local[2] * np.array(p.z)
            self.assertTrue(np.allclose(GH.plain(cap.root_box().center), world, atol=1e-9))

    def test_cull_index_removes_exactly_the_west_planes_of_the_top_storey(self):
        sol, _ = self.canvas_slots("B")
        before = sol.tree("mv", "Geometry")
        after = sol.tree("cull", "List")
        self.assertEqual([len(v) for v in before.branches()], [8] * 11)
        self.assertEqual([len(v) for v in after.branches()], [8] * 10 + [6])
        self.assertEqual(list(sol.tree("cullp", "Integer").branch((0, 10))), [6, 7])
        self.assertEqual(sum(len(v) for v in GH.evaluate(GH.canvas("tower"), GH.tower_overrides("A")).tree("cullp", "Integer").branches()), 0)

    def test_views_show_the_cull_and_each_stage(self):
        views = GH.canvas("tower").views
        self.assertIn("Cull Index", {o.component for o in GH.canvas("tower").view("cull")})
        for name in ("stagger", "centre", "storeys", "cull", "yaw", "clearance", "orient"):
            self.assertIn(name, views)


class TestCatalogue(unittest.TestCase):
    REAL_NAMES = {"Number Slider", "Panel", "Boolean Toggle", "Value List", "Param Viewer", "Center Box", "Domain Box", "Cylinder", "Circle", "Extrude", "Fillet Edge", "Solid Difference",
                  "Orient", "Rotate", "Move", "Series", "Range", "Construct Domain", "Construct Point", "XY Plane", "Plane Origin", "List Item", "List Length", "Repeat Data", "Dispatch",
                  "Merge", "Entwine", "Graft Tree", "Flatten Tree", "Simplify Tree", "Path Mapper", "Flip Matrix", "Explode Tree", "Tree Statistics"}

    def test_real_grasshopper_names_are_present(self):
        self.assertLessEqual(self.REAL_NAMES, set(GH.catalogue()))

    def test_every_component_is_documented_and_implemented(self):
        for name, spec in GH.catalogue().items():
            self.assertTrue(spec.description["en"] and spec.description["de"], name)
            self.assertTrue(spec.nickname and spec.category and spec.subcategory, name)
            names = [p.name for p in spec.inputs]
            self.assertEqual(len(names), len(set(names)), name)
            self.assertTrue(all(p.access in ("item", "list", "tree") for p in spec.inputs + spec.outputs), name)
            if spec.kind == "component":
                self.assertIn(name, GH.IMPL, name)
        self.assertEqual(set(GH.IMPL) - set(GH.catalogue()), set())

    def test_parameter_types_are_casting_targets(self):
        types = set(MODEL["casting"]["types"]) | {"Transform", "Generic"}
        for name, spec in GH.catalogue().items():
            for p in spec.inputs + spec.outputs:
                self.assertIn(p.type, types, (name, p.name))

    def test_component_lookup(self):
        box = GH.catalogue()["Center Box"]
        self.assertEqual((box.category, box.subcategory), ("Surface", "Primitive"))
        self.assertEqual([p.name for p in box.inputs], ["Base", "X", "Y", "Z"])
        self.assertEqual(GH.catalogue()["Series"].output("Series").access, "list")
        self.assertEqual(GH.catalogue()["Graft Tree"].input("Tree").access, "tree")

    def test_paths_stay_short(self):
        for path in (CLIP / "🧱️model/🔣️.json", CLIP / "🧪️cases/🔣️.json", CLIP / "🧪️tests/🐍️.py", CLIP / "🧱️model/🧬️schema/🔣️.schema.json"):
            self.assertLess(len(str(path)), 256)


class TestPanelDisplay(unittest.TestCase):
    def test_panel_rows_show_paths_and_indices(self):
        rows = GH.panel_rows(GH.Tree.from_dict({"{0;0}": [1.5, 2.0], "{0;1}": ["a"]}))
        self.assertEqual(rows, (("{0;0}", 0, "1.5"), ("{0;0}", 1, "2"), ("{0;1}", 0, "a")))
        self.assertEqual(GH.panel_rows(GH.Tree.from_dict({"{0}": [True]}), paths=False), ((None, 0, "True"),))

    def test_number_format_follows_language(self):
        self.assertEqual(GH.format_number(1250.5, "en"), "1250.5")
        self.assertEqual(GH.format_number(1250.5, "de"), "1250,5")
        self.assertEqual(GH.format_number(3.0), "3")


if __name__ == "__main__":
    unittest.main(verbosity=1)
