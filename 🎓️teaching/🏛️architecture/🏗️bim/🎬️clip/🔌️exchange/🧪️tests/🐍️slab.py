"""🧪 Slab consumer contracts run without importing Manim."""
import ast
import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CASE = json.loads((ROOT / "🔌️exchange" / "🧪️cases" / "🔣️slab.json").read_text(encoding="utf-8"))


def method(chapter, name):
    """🔎 Read one scene method without loading its renderer."""
    tree = ast.parse((ROOT / chapter / "🐍️.py").read_text(encoding="utf-8"))
    return next(node for node in ast.walk(tree) if isinstance(node, ast.FunctionDef) and node.name == name)


class SlabConsumers(unittest.TestCase):
    """🕳️ Every reported opening has visible geometry and a source label."""

    def test_source_opening_areas(self):
        """📐 Independently reconstruct registered architectural voids rather than snapshot quantities."""
        import math,numpy as np
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        source=json.loads((ROOT/'🧱️model'/'🔣️.json').read_text())
        slab=next(item for item in source['elements'] if item['tag']==CASE['slab'])
        drafting=ast.parse((ROOT/'🎨️visuals'/'📐️drafting'/'📏️geometry.py').read_text())
        area_function=next(node for node in drafting.body if isinstance(node,ast.FunctionDef) and node.name=='signed_area')
        namespace={'np':np,'P':lambda point:np.array([*point,0])}
        exec(compile(ast.Module(body=[area_function],type_ignores=[]),'actual-area','exec'),namespace)
        authored={item['tag']:item['outline'] for item in slab['geometry']['voids']}
        self.assertEqual(set(authored),{item['tag'] for item in CASE['openings']})
        expected=[]
        for case in CASE['openings']:
            if case['shape']=='rectangle':
                x0,y0,x1,y1=case['bounds']
                points=[[x0,y0],[x1,y0],[x1,y1],[x0,y1]]
                area=(x1-x0)*(y1-y0)
            else:
                cx,cy=case['centre'];r,tail,n=case['radius'],case['straight'],case['segments']
                angles=np.linspace(-math.pi/2,math.pi/2,n+1)
                points=[[cx-tail,cy-r],*[[cx+r*math.cos(a),cy+r*math.sin(a)] for a in angles],[cx-tail,cy+r]]
                area=2*r*tail+n*r*r*math.sin(math.pi/n)/2
            with self.subTest(tag=case['tag']):
                np.testing.assert_allclose(authored[case['tag']],points,rtol=0,atol=1e-12)
                independent=Polygon(points)
                self.assertAlmostEqual(independent.area,area,places=11)
                self.assertAlmostEqual(abs(namespace['signed_area'](authored[case['tag']])),area,places=10)
                expected.append(independent)
        x0,y0,x1,y1=CASE['gross_rectangle']
        boundary=[[x0,y0],[x1,y0],[x1,y1],[x0,y1]]
        np.testing.assert_allclose(slab['geometry']['outline'],boundary,rtol=0,atol=1e-12)
        gross=Polygon(boundary)
        self.assertAlmostEqual(gross.area,(x1-x0)*(y1-y0),places=10)
        voids=unary_union(expected)
        self.assertAlmostEqual(voids.area,sum(p.area for p in expected),places=10)
        self.assertAlmostEqual(gross.difference(voids).area,gross.area-sum(p.area for p in expected),places=10)
        self.assertEqual({item['tag'] for item in source['spaces'] if item['storey']=='first'},set(CASE['first_floor_spaces']))

    def test_exchange_room_context(self):
        source = ast.unparse(method("🔌️exchange", "_beat_slab"))
        self.assertIn("'SP-SALON'", source)
        self.assertNotIn("tones[i]", source)
        self.assertIn("sp['outline']", source)

    def test_all_openings_drawn_and_named(self):
        for chapter, name in (("🔌️exchange", "_beat_slab"), ("🧩️models", "_slab")):
            source = ast.unparse(method(chapter, name))
            self.assertNotIn("['voids'][0]", source)
            self.assertIn("['void_tags']", source)
            self.assertIn("signed_area", source)
            self.assertIn("zip(", source)

    def test_crop_against_independent_geometry(self):
        tree = ast.parse((ROOT / "🔌️exchange" / "🐍️.py").read_text(encoding="utf-8"))
        function = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == "_clip_segment")
        namespace = {}
        exec(compile(ast.Module(body=[function], type_ignores=[]), "clip", "exec"), namespace)
        from shapely.geometry import LineString, box
        for case in CASE["clip_segments"]:
            result = namespace["_clip_segment"](case["a"], case["b"], case["low"], case["high"])
            expected = case["expected"]
            self.assertEqual(None if result is None else [list(point) for point in result], expected)
            independent = LineString([case["a"], case["b"]]).intersection(box(*case["low"], *case["high"]))
            self.assertEqual(None if independent.is_empty else [list(point) for point in independent.coords], expected)

    def test_disconnected_glass(self):
        """🪟 Reject a retired curtain singleton path while accepting each native record's path."""
        node=method('🔌️exchange','_bay')
        key=lambda item:item.slice.value if isinstance(item,ast.Subscript) and isinstance(item.slice,ast.Constant) else None
        loops=[item for item in ast.walk(node) if isinstance(item,ast.For) and key(item.iter)=='paths']
        self.assertEqual(len(loops),1)
        record=loops[0].target.id
        self.assertTrue(any(isinstance(item,ast.Subscript) and isinstance(item.value,ast.Name) and item.value.id==record and key(item)=='path' for item in ast.walk(loops[0])))
        self.assertFalse(any(isinstance(item,ast.Subscript) and key(item)=='path' and key(item.value)=='curtain_wall' for item in ast.walk(node)))

    def test_grid_dimension_is_source_based(self):
        source = ast.unparse(method("🧩️models", "_slab"))
        self.assertIn("['grid']['x']", source)
        self.assertNotIn("1.25", source)


if __name__ == "__main__":
    unittest.main()
