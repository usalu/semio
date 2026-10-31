"""📦 Preserve intrinsic source grouping under rigid placement without changing world material."""
import importlib
import json
from collections import Counter
from pathlib import Path
import sys
import unittest
import numpy as np
import rhino3dm
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
C=json.loads((ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/👁️occlusion/📦️grouping/🔣️.json').read_text(encoding='utf-8'))

class LocalGrouping(unittest.TestCase):
    def test_exact_intrinsic_groups_are_translation_invariant_and_preserve_world_polygons(self):
        slots=tuple(V._model.slots())
        for case in C['cases']:
            with self.subTest(case=case['id']):
                previous=None
                previous_groups=None
                for placement in case['placements']:
                    slot=slots[placement['slot']]
                    self.assertEqual(slot.layout,placement['family'])
                    np.testing.assert_array_equal(slot.plane.origin,placement['origin'])
                    axes=np.asarray(placement['axes'],float)
                    np.testing.assert_array_equal([slot.plane.x,slot.plane.y,slot.plane.z],axes)
                    parts=V._cap_parts(V._cap(V._model.capsule_type(placement['type'])),layout=placement['family'])
                    groups={}
                    items=V._assembly_items(parts,base=V.CAPSULE,groups=groups,plane=slot.plane,lod=True,glass_opacity=1)
                    if previous_groups is not None:
                        self.assertEqual(set(groups),set(previous_groups))
                        self.assertTrue(all(groups[key] is not previous_groups[key] for key in groups))
                        self.assertTrue(all(groups[key].material_underpaint is not previous_groups[key].material_underpaint for key in groups))
                    previous_groups=groups
                    expected=Counter()
                    transform=rhino3dm.Transform.Identity()
                    for i in range(3):
                        for j in range(3):
                            setattr(transform,'M'+str(i)+str(j),float(axes[j,i]))
                        setattr(transform,'M'+str(i)+'3',float(placement['origin'][i]))
                    for part,mesh in parts:
                        world=np.asarray(placement['origin'])+mesh.V@axes
                        oracle=[]
                        for row in mesh.V:
                            point=rhino3dm.Point3d(*row)
                            point.Transform(transform)
                            oracle.append((point.X,point.Y,point.Z))
                        np.testing.assert_allclose(world,oracle,rtol=0,atol=1e-12)
                        expected.update((part.key,tuple(map(tuple,world[face]))) for face in mesh.faces)
                    actual=Counter((item.part_key,tuple(map(tuple,polygon))) for item in items for polygon in item.polygons)
                    self.assertEqual(actual,expected)
                    membership=tuple((item.geometry_key,tuple(len(p) for p in item.polygons)) for item in items)
                    if previous is not None:
                        self.assertEqual(membership,previous)
                    previous=membership

if __name__=='__main__':
    unittest.main(verbosity=2)
