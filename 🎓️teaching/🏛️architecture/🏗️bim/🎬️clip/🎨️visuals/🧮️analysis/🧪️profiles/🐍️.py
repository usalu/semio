"""🔍 True mesh outlines preserve supplied section contours in every view."""

import ast
import json
import importlib
from pathlib import Path
import unittest

import numpy as np
from shapely.geometry import Polygon


class MeshTrueProfileContract(unittest.TestCase):
    def test_true_shape_follows_rectangular_section_rather_than_bounding_circle(self):
        directory = Path(__file__).parents[1]
        tree = ast.parse((directory / "🐍️.py").read_text(encoding="utf-8"))
        node = next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "_true_mesh_paths")
        namespace = {}
        exec(compile(ast.Module(body=[node], type_ignores=[]), "true-profile", "exec"), namespace)
        data = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
        paths = namespace["_true_mesh_paths"](data, "top", None)
        self.assertEqual(len(paths), 1)
        self.assertAlmostEqual(Polygon([p[:2] for p in paths[0]]).area, data["area"])
        np.testing.assert_allclose(np.min(np.asarray(paths[0])[:, :2], axis=0), [0, 0])
        np.testing.assert_allclose(np.max(np.asarray(paths[0])[:, :2], axis=0), [.22, .16])
        self.assertEqual(len(namespace["_true_mesh_paths"](data, "axon", None)), 2)
        side = namespace["_true_mesh_paths"](data, "side", None)
        np.testing.assert_allclose([[p[0], p[2]] for path in side for p in path], [[0,0],[0,2.91],[.22,0],[.22,2.91]])

    def test_analytical_plan_marker_uses_the_actual_member_section(self):
        tree = ast.parse((Path(__file__).parents[1]/"🐍️.py").read_text(encoding="utf-8"))
        node = next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name == "_analysis_section_outline")
        api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        namespace = {"_model":api}
        exec(compile(ast.Module(body=[node],type_ignores=[]),"section-glyph","exec"),namespace)
        case = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
        member = {"sections":[{"profile":case["profile"],"base":0,"top":2.91}]}
        outline = namespace["_analysis_section_outline"](member,[1,2,3.025])
        self.assertAlmostEqual(Polygon(outline).area,case["area"])
        np.testing.assert_allclose(Polygon(outline).bounds,[.89,1.92,1.11,2.08])


if __name__ == "__main__":
    unittest.main()
                                  