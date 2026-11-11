"""🧪 Finish sections of prism and mesh cells checked against a fixture and GEOS."""
import ast
from pathlib import Path
import importlib
import json
import unittest

from shapely.geometry import Polygon, box, LineString
from shapely.ops import unary_union


class FinishSectionContract(unittest.TestCase):
    """🧵 A published finish cell sections from its mesh when it has no prism outline."""

    def test_finish_sections(self):
        """✂️ Compare prism scanlines and mesh plane cuts with Shapely polygons."""
        root = Path(__file__).resolve().parent
        fixture = json.loads((root / "finish-section.json").read_text(encoding="utf-8"))
        tree = ast.parse((root.parent / "📋️plan" / "🐍️.py").read_text(encoding="utf-8"))
        names = {"profile_cut", "vertical_prism", "occupied_section_rings", "ring_edges"}
        nodes = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name in names]
        self.assertEqual({node.name for node in nodes}, names)
        model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        scope = {"_c": type("Core", (), {"_model": model})()}
        exec(compile(ast.Module(body=nodes, type_ignores=[]), str(root), "exec"), scope)
        prism = fixture["prism"]
        part, axis, value = prism["part"], prism["axis"], prism["value"]
        rings = scope["occupied_section_rings"]([part], axis, value)
        actual = unary_union([Polygon(ring) for ring in rings])
        line = LineString([(-100, value), (100, value)])
        cut = Polygon(part["outline"]).intersection(line)
        pieces = list(cut.geoms) if hasattr(cut, "geoms") else [cut]
        expected = unary_union([box(float(min(p[0] for p in piece.coords)), part["z0"], float(max(p[0] for p in piece.coords)), part["z1"]) for piece in pieces if not piece.is_empty and piece.length])
        self.assertAlmostEqual(actual.symmetric_difference(expected).area, 0)
        for case in (fixture["mesh"], fixture["wedge"]):
            with self.subTest(case=case["part"]["layer"]):
                self.assertNotIn("outline", case["part"])
                rings = scope["occupied_section_rings"]([case["part"]], case["axis"], case["value"])
                self.assertEqual(len(rings), 1)
                actual = Polygon(rings[0])
                expected = Polygon(case["ring"])
                self.assertAlmostEqual(actual.symmetric_difference(expected).area, 0)
                edges = scope["ring_edges"](rings)
                length = sum(LineString([a, b]).length for a, b in edges)
                self.assertAlmostEqual(length, expected.exterior.length)
        shared = scope["ring_edges"]([[(0, 0), (1, 0), (1, 1), (0, 1)], [(1, 0), (2, 0), (2, 1), (1, 1)]])
        length = sum(LineString([a, b]).length for a, b in shared)
        union = unary_union([box(0, 0, 1, 1), box(1, 0, 2, 1)])
        self.assertAlmostEqual(length, union.exterior.length)
