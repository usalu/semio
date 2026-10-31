"""⚡ Verify physical Energy drawing anchors independently from nominal thermal-study dimensions."""
import ast
import dataclasses
import importlib
import json
from pathlib import Path
import sys
import unittest
import numpy as np
import rhino3dm as rh

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
CLIP=ROOT/'🎓️teaching/🏛️architecture/📊️excel/🎬️clip'
M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')
C=json.loads((CLIP/'🧪️cases/⚡️anchors/🔣️.json').read_text(encoding='utf8'))
TREE=ast.parse((CLIP/'⚡️energy/🐍️.py').read_text(encoding='utf8'))

def helpers():
    """📦 Execute only the pure authored geometry helpers, without importing or drawing the scene."""
    selected=[node for node in TREE.body if isinstance(node,ast.FunctionDef) and node.name in C['helper_names']]
    if len(selected)!=len(C['helper_names']): raise AssertionError('Physical Energy anchor/volume helpers are missing')
    namespace={'_nakagin':M}
    exec(compile(ast.Module(body=selected,type_ignores=[]),'energy_geometry_helpers','exec'),namespace)
    return namespace

class EnergyAnchors(unittest.TestCase):
    def test_reference_points_use_physical_boundaries_and_actual_window_circle(self):
        """📏 Match explicit raw reference points and all six actual window midplanes/normals."""
        h=helpers()
        anchors=h['_physical_anchors'](M.capsule_type('A'))
        for key,expected in C['reference_points'].items(): np.testing.assert_allclose(anchors[key][0],expected,atol=C['tolerance'],rtol=0,err_msg=key)
        for family in C['families']:
            window=M.capsule_window(M.capsule_type('A'),family)
            actual=h['_physical_anchors'](M.capsule_type('A'),family)['window']
            np.testing.assert_allclose(actual[0],window.center,atol=C['tolerance'],rtol=0)
            np.testing.assert_allclose(actual[1],window.normal,atol=C['tolerance'],rtol=0)
        nominal=M.capsule_type('A')
        self.assertEqual([nominal.width,nominal.length,nominal.height],C['nominal_energy_dimensions'])

    def test_live_and_explicit_interior_controls_match_raw_bounds_and_rhino(self):
        """🧊 Validate drawing volume bounds with Rhino and NumPy under explicit/missing overrides."""
        h=helpers()
        base=dataclasses.asdict(M.capsule_type('A'))
        for row in C['controls']:
            parameters={key:value for key,value in base.items() if not key.startswith('interior_')}
            parameters.update(row['parameters'])
            points=np.asarray([point for face in h['_usable_volume_faces'](parameters) for point in face])
            expected=row['interior']
            bounds=[points[:,0].min(),points[:,0].max(),points[:,1].min(),points[:,1].max(),points[:,2].min(),points[:,2].max()]
            np.testing.assert_allclose(bounds,expected,atol=C['tolerance'],rtol=0)
            box=rh.BoundingBox(rh.Point3d(*points.min(axis=0)),rh.Point3d(*points.max(axis=0)))
            self.assertAlmostEqual(box.Volume,float(np.prod(points.max(axis=0)-points.min(axis=0))))
            anchors=h['_physical_anchors'](parameters)
            self.assertAlmostEqual(anchors['left'][0][0],row['casing'][0])
            self.assertAlmostEqual(anchors['right'][0][0],row['casing'][1])
            self.assertAlmostEqual(anchors['front'][0][1],row['casing'][3])
            self.assertAlmostEqual(anchors['back'][0][1],row['casing'][2])

    def test_scene_methods_consume_authoritative_helpers_without_nominal_point_literals(self):
        """🔗 Protect the actual arrow, flyer, volume and back-factor annotation call sites."""
        helpers()
        names={node.id for node in ast.walk(TREE) if isinstance(node,ast.Name)}
        self.assertNotIn('POINT',names)
        methods={node.name:node for node in ast.walk(TREE) if isinstance(node,ast.FunctionDef)}
        for name in C['scene_users']:
            calls={getattr(node.func,'id',None) for node in ast.walk(methods[name]) if isinstance(node,ast.Call)}
            self.assertIn('_usable_volume_faces' if name=='_volume_box' else '_physical_anchors',calls)

if __name__=='__main__':
    unittest.main(verbosity=2)
