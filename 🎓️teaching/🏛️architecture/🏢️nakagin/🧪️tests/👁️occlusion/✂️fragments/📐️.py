"""🧭 Check retained thin-fragment plane lineage without importing the rendering stack."""
import ast
import json
import math
from pathlib import Path
from types import SimpleNamespace
import unittest
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
C=json.loads((BASE/'🧪️cases/👁️occlusion/✂️fragments/🔣️.json').read_text(encoding='utf-8'))
class PlaneLineage(unittest.TestCase):
    def test_exact_tiny_fragment_exports_its_validated_original_plane(self):
        tree=ast.parse((BASE/'🎨️visuals/🐍️.py').read_text(encoding='utf-8'))
        helper=next((node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name=='_paint_normal'),None)
        self.assertIsNotNone(helper)
        namespace={}
        exec(compile(ast.Module(body=[helper],type_ignores=[]),'<paint-normal>','exec'),namespace)
        case=C['plane_lineage']
        item=SimpleNamespace(origin=object(),normal=case['normal'])
        result=namespace['_paint_normal'](item,case['fragment'])
        self.assertEqual(tuple(result),tuple(case['normal']))
        offsets=[tuple(a-b for a,b in zip(point,case['carrier'][0])) for point in case['fragment']]
        self.assertLessEqual(max(abs(math.fsum(a*b for a,b in zip(point,result))) for point in offsets),case['epsilon'])
        origin=case['fragment'][0]
        offsets=[tuple(a-b for a,b in zip(point,origin)) for point in case['fragment']]
        self.assertGreater(max(abs(math.fsum(a*b for a,b in zip(point,case['unstable_normal']))) for point in offsets),case['recomputed_residual_minimum'])
if __name__=='__main__':
    unittest.main(verbosity=2)
