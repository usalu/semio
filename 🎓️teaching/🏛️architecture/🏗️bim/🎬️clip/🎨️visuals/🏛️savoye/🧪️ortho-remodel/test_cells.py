"""🧪 Portable wall-cell subtraction checked against NumPy raster integration."""
import ast
import json
from pathlib import Path
import unittest

import numpy as np


class WallCellContract(unittest.TestCase):
    """🧱 Cross-language fixtures for arbitrary wall opening counts."""

    def test_cells(self):
        """📐 Compare nonoverlapping authored wall cells with independent sampled occupancy."""
        root = Path(__file__).resolve().parent
        source = ast.parse((root.parent / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        helpers = [node for node in source.body if isinstance(node, ast.FunctionDef) and node.name in ("wall_cells", "cell_edges")]
        self.assertEqual(len(helpers), 2)
        namespace = {}
        exec(compile(ast.Module(body=helpers, type_ignores=[]), str(root), "exec"), namespace)
        for case in json.loads((root / "cases.json").read_text(encoding="utf-8"))["cases"]:
            with self.subTest(case=case["id"]):
                wall = case["wall"]
                cells = namespace["wall_cells"](wall)
                area = sum((b-a)*(d-c) for a,b,c,d in cells)
                self.assertAlmostEqual(area, case["area"])
                edges = namespace["cell_edges"](cells)
                self.assertTrue(all(p != q for p,q in edges))
                self.assertEqual(len(edges), len({tuple(sorted((p,q))) for p,q in edges}))
                x = (np.arange(400)+0.5)*wall["length"]/400
                z = wall["base"]+(np.arange(300)+0.5)*(wall["top"]-wall["base"])/300
                xx, zz = np.meshgrid(x,z)
                filled = np.ones(xx.shape,dtype=bool)
                for opening in wall["openings"]:
                    filled &= ~((xx>opening["start"]) & (xx<opening["end"]) & (zz>opening["sill_z"]) & (zz<opening["head_z"]))
                self.assertAlmostEqual(area,float(filled.mean()*wall["length"]*(wall["top"]-wall["base"])))
                padded = np.pad(filled,1)
                perimeter = float(np.count_nonzero(padded[:,1:] != padded[:,:-1])*(wall["top"]-wall["base"])/300 + np.count_nonzero(padded[1:,:] != padded[:-1,:])*wall["length"]/400)
                self.assertAlmostEqual(sum(abs(p[0]-q[0])+abs(p[1]-q[1]) for p,q in edges),perimeter)


if __name__ == "__main__":
    unittest.main()
