"""🧪 Polygon profile cuts checked against independent Shapely intersections."""
import ast
import json
from pathlib import Path
import unittest

import numpy as np
from shapely.geometry import LineString, Polygon


class ProfileCutContract(unittest.TestCase):
    """🏛️ Actual column contours determine every orthographic section."""

    def test_profile_spans(self):
        """✂️ Validate rectangular, chamfered and disconnected cuts using GEOS."""
        root = Path(__file__).resolve().parent
        source = ast.parse((root.parent / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        helpers = [n for n in source.body if isinstance(n,ast.FunctionDef) and n.name in ("profile_cut","model_vertical_bounds","plan_cut_z","adjacent_slabs")]
        self.assertEqual(len(helpers),4)
        scope = {}
        exec(compile(ast.Module(body=helpers,type_ignores=[]),str(root),"exec"),scope)
        fixtures = json.loads((root / "profiles.json").read_text(encoding="utf-8"))
        for case in fixtures["cuts"]:
            with self.subTest(case=case["id"]):
                actual = scope["profile_cut"](case["outline"],case["axis"],case["value"])
                np.testing.assert_allclose(actual,case["expected"],rtol=0,atol=1e-12)
                axis,value = case["axis"],case["value"]
                line = LineString([(value,-100),(value,100)] if axis == 0 else [(-100,value),(100,value)])
                cut = Polygon(case["outline"]).intersection(line)
                pieces = list(cut.geoms) if hasattr(cut,"geoms") else [cut]
                expected = sorted((min(p[1-axis] for p in part.coords),max(p[1-axis] for p in part.coords)) for part in pieces if part.length > 0)
                np.testing.assert_allclose(actual,expected,rtol=0,atol=1e-12)
        for kind,rx,ry in (("circle",1,1),("ellipse",2,.5)):
            angles = np.linspace(0,2*np.pi,97)[:-1]
            ring = np.column_stack((rx*np.cos(angles),ry*np.sin(angles))).tolist()
            actual = scope["profile_cut"](ring,0,rx*.5)
            independent = Polygon(ring).intersection(LineString([(rx*.5,-10),(rx*.5,10)]))
            np.testing.assert_allclose(actual,[[independent.bounds[1],independent.bounds[3]]],rtol=0,atol=1e-12)


    def test_cellar_levels_and_slabs(self):
        """🏢 Validate four storeys, adjacent slabs and below-ground structural bounds."""
        root = Path(__file__).resolve().parent
        tree = ast.parse((root.parent / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        names = {"model_vertical_bounds","plan_cut_z","adjacent_slabs"}
        nodes = [node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in names]
        scope = {}
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(root),"exec"),scope)
        levels = json.loads((root / "profiles.json").read_text(encoding="utf-8"))["levels"]
        model = {"storeys":levels,"box":{"height":9.4},"slabs":[{"tag":tag,"top":top,"thickness":999,"parts":[{"z0":top-.2,"z1":top}]} for tag,top in (("SL-C",-2.4),("SL-G",0),("SL-1",3.5),("SL-R",6.7))],
                 "foundations":[{"parts":[{"outline":[[0,0],[1,0],[1,1],[0,1]],"z0":-3.6,"z1":-2.6}]}],
                 "details":[{"parts":[{"outline":[[0,0],[1,0],[1,1],[0,1]],"z0":9.4,"z1":9.65}]}]}
        expected = {"cellar":("SL-C","SL-G"),"ground":("SL-G","SL-1"),"first":("SL-1","SL-R"),"roof":("SL-R",None)}
        for level,(under,above) in expected.items():
            cut = scope["plan_cut_z"](model,level)
            self.assertAlmostEqual(cut,levels[level]+1.2)
            slabs = scope["adjacent_slabs"](model,cut)
            self.assertEqual([s["tag"] for s,a,u in slabs if u],[under])
            self.assertEqual([s["tag"] for s,a,u in slabs if a],[above] if above else [])
            tops = np.asarray([s["top"] for s in model["slabs"]])
            self.assertAlmostEqual(next(s["top"] for s,a,u in slabs if u),float(tops[tops <= cut].max()))
        np.testing.assert_allclose(scope["model_vertical_bounds"](model),[-3.6,9.65],rtol=0,atol=1e-12)


    def test_finish_and_footing_visibility(self):
        """🪵 Compare finish layering and buried-footing admission with NumPy masks."""
        root = Path(__file__).resolve().parent
        tree = ast.parse((root.parent / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        nodes = [n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ("plan_part_rank","plan_part_visible")]
        scope = {}
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(root),"exec"),scope)
        cases = json.loads((root / "profiles.json").read_text(encoding="utf-8"))["parts"]
        actual = []
        for case in cases:
            data = {"kind":case["kind"],"storey":case["storey"]}
            actual.append(scope["plan_part_visible"](data,case,case["level"],case["band"],True,case["structural"]))
            self.assertEqual(actual[-1],case["visible"])
            self.assertEqual(scope["plan_part_rank"](data),case["rank"])
        finish = np.isin([case["kind"] for case in cases],["parquet","tile","paving","grass_joint"])
        same = np.asarray([case["storey"] == case["level"] for case in cases])
        lower = np.asarray([case["z1"] >= case["band"][0]-1e-6 for case in cases])
        upper = np.asarray([case["z0"] < case["band"][1]-1e-6 for case in cases])
        structural = np.asarray([case["structural"] for case in cases])
        independent = upper & np.where(finish,same,lower | (structural & same))
        np.testing.assert_array_equal(actual,independent)


if __name__ == "__main__":
    unittest.main()
