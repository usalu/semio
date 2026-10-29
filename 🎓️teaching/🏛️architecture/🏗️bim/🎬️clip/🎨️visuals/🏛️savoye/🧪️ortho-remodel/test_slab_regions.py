"""🧪 Regional slab thickness and void sections checked independently with GEOS."""
import ast
from copy import deepcopy
import json
from pathlib import Path
import unittest

import numpy as np
from shapely.geometry import Polygon, box, LineString
from shapely.ops import unary_union


class RegionalSlabContract(unittest.TestCase):
    """🟫 Root slab thickness is deliberately poisonous metadata, never physical geometry."""

    def test_region_cuts(self):
        """✂️ Compare mixed-thickness cuts, crop clipping and union boundaries with Shapely."""
        root = Path(__file__).resolve().parent
        fixture = json.loads((root / "slab-regions.json").read_text(encoding="utf-8"))
        tree = ast.parse((root.parent / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        names = {"profile_cut","slab_cut","rect_union_cells","cell_edges","point_in_ring","slab_sides"}
        nodes = [n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in names]
        self.assertEqual(len(nodes),6)
        scope = {}
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(root),"exec"),scope)
        slab = fixture["slab"]
        occupied = unary_union([Polygon(part["outline"]) for part in slab["parts"]])
        expected_surface = Polygon(slab["outline"],slab["voids"])
        self.assertAlmostEqual(occupied.symmetric_difference(expected_surface).area,0)
        for case in fixture["cuts"]:
            with self.subTest(case=case):
                cells = scope["slab_cut"](slab,case["axis"],case["value"],case.get("clip"))
                actual = unary_union([box(a,c,b,d) for a,b,c,d in cells])
                independent = []
                value,axis = case["value"],case["axis"]
                line = LineString([(value,-100),(value,100)] if axis == 0 else [(-100,value),(100,value)])
                for part in slab["parts"]:
                    cut = Polygon(part["outline"]).intersection(line)
                    pieces = list(cut.geoms) if hasattr(cut,"geoms") else [cut]
                    for segment in pieces:
                        if segment.is_empty or segment.length == 0:
                            continue
                        hs = np.asarray(segment.coords)[:,1-axis]
                        independent.append(box(float(hs.min()),part["z0"],float(hs.max()),part["z1"]))
                expected = unary_union(independent)
                if "clip" in case:
                    expected = expected.intersection(box(case["clip"][0],-10,case["clip"][1],10))
                self.assertAlmostEqual(actual.symmetric_difference(expected).area,0)
                self.assertAlmostEqual(actual.area,case["area"])
                merged = scope["rect_union_cells"](cells)
                edges = scope["cell_edges"](merged)
                length = sum(np.linalg.norm(np.asarray(a)-np.asarray(b)) for a,b in edges)
                self.assertAlmostEqual(length,expected.length)


    def test_exposed_region_faces(self):
        """🧱 Exposed sides match GEOS perimeter integration across all height bands."""
        root = Path(__file__).resolve().parent
        slab = json.loads((root / "slab-regions.json").read_text(encoding="utf-8"))["slab"]
        tree = ast.parse((root.parent / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        nodes = [n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ("point_in_ring","slab_sides")]
        scope = {}
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(root),"exec"),scope)
        faces = scope["slab_sides"](slab)
        actual = sum(float(np.linalg.norm(np.asarray(a)-np.asarray(b)))*(z1-z0) for a,b,z0,z1,n in faces)
        zs = sorted({p[key] for p in slab["parts"] for key in ("z0","z1")})
        independent = 0
        for z0,z1 in zip(zs,zs[1:]):
            occupied = unary_union([Polygon(p["outline"]) for p in slab["parts"] if p["z0"] < (z0+z1)/2 < p["z1"]])
            independent += occupied.length*(z1-z0)
        self.assertAlmostEqual(actual,independent)


    def test_level_finish_from_source_parts(self):
        """🧱 Actual finish volumes follow regional tops and a source paving-thickness event."""
        root = Path(__file__).resolve().parent
        slab = json.loads((root / "slab-regions.json").read_text(encoding="utf-8"))["slab"]
        for part in slab["parts"]:
            if part["region"] == "thick":
                part["z1"],part["top"],part["z0"] = .06,.06,-.54
        slab["layers"] = [{"id":"paving","thickness":.09}]
        model_source = ast.parse((root.parents[2] / "🧱️model/🐍️.py").read_text(encoding="utf-8"))
        nodes = [n for n in model_source.body if isinstance(n,ast.FunctionDef) and n.name == "_slab_finish_parts"]
        self.assertEqual(len(nodes),1)
        scope = {"deepcopy":deepcopy,"_round":lambda value:round(value,9)}
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(root),"exec"),scope)
        initial = scope["_slab_finish_parts"](slab)
        np.testing.assert_allclose([p["z1"] for p in initial],.09,rtol=0,atol=1e-12)
        np.testing.assert_allclose([p["z1"]-p["z0"] for p in initial],[.09 if p["region"] == "thin" else .03 for p in initial],rtol=0,atol=1e-12)
        initial_volume = sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in initial)
        slab["layers"][0]["thickness"] = .14
        changed = scope["_slab_finish_parts"](slab)
        np.testing.assert_allclose([p["z1"] for p in changed],.14,rtol=0,atol=1e-12)
        changed_volume = sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in changed)
        area = unary_union([Polygon(p["outline"]) for p in slab["parts"]]).area
        self.assertAlmostEqual(changed_volume-initial_volume,area*.05)
        plan_source = ast.parse((root.parent / "📋️plan/🐍️.py").read_text(encoding="utf-8"))
        helpers = [n for n in plan_source.body if isinstance(n,ast.FunctionDef) and n.name in ("point_in_ring","slab_sides")]
        exec(compile(ast.Module(body=helpers,type_ignores=[]),str(root),"exec"),scope)
        occupied = slab["parts"]+changed
        faces = scope["slab_sides"](slab,occupied)+scope["slab_sides"]({"parts":changed},occupied)
        actual_sides = sum(float(np.linalg.norm(np.asarray(a)-np.asarray(b)))*(z1-z0) for a,b,z0,z1,n in faces)
        zs = sorted({p[key] for p in occupied for key in ("z0","z1")})
        independent_sides = sum(unary_union([Polygon(p["outline"]) for p in occupied if p["z0"] < (z0+z1)/2 < p["z1"]]).length*(z1-z0) for z0,z1 in zip(zs,zs[1:]))
        self.assertAlmostEqual(actual_sides,independent_sides)



if __name__ == "__main__":
    unittest.main()
