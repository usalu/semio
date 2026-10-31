"""🧭 Verify source-confirmed per-storey poses independently of mutable item labels."""
import importlib
import json
from pathlib import Path
import sys
import unittest
import numpy as np
import rhino3dm as rh

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')

class ConfirmedSourcePoses(unittest.TestCase):
    def test_confirmed_physical_poses_match_actual_assemblies(self):
        """📍 Match occupied centres, then compare actual physical shell vertices with Rhino placement."""
        cases=json.loads((BASE/'🧪️cases/🧭️poses/🔣️.json').read_text(encoding='utf8'))
        self.assertTrue(cases['records'])
        for expected in cases['records']:
            with self.subTest(tower=expected['tower'],floor=expected['floor'],center=expected['center']):
                candidates=[s for s in M.slots() if M.towers()[s.tower].key==expected['tower'] and s.floor==expected['floor']]
                matches=[]
                for slot in candidates:
                    cap=M.capsule_type(slot.type)
                    center=np.asarray(slot.plane.origin)+np.asarray(slot.plane.y)*cap.length/2
                    if np.allclose(center,expected['center'],atol=1e-8,rtol=0):
                        matches.append(slot)
                self.assertEqual(len(matches),1)
                slot=matches[0]
                self.assertEqual((slot.layout,slot.sublevel),(expected['family'],expected['phase']))
                self.assertAlmostEqual((slot.rotation-expected['yaw'])%360,0)
                cap=M.capsule_type(slot.type)
                source=M.capsule_assembly(cap,detail='exterior',layout=slot.layout)
                target=M.slot_assembly(slot,detail='exterior')
                world=rh.Plane(rh.Point3d(*slot.plane.origin),rh.Vector3d(*slot.plane.x),rh.Vector3d(*slot.plane.y))
                transform=rh.Transform.PlaneToPlane(rh.Plane.WorldXY(),world)
                by_key={part.key:part for part in target.parts}
                for part in source.parts:
                    if part.role not in ('shell','shell_back'):
                        continue
                    points=[]
                    for vertex in part.solid.vertices:
                        point=rh.Point3d(*vertex)
                        point.Transform(transform)
                        points.append((point.X,point.Y,point.Z))
                    np.testing.assert_allclose(points,by_key[part.key].solid.vertices,atol=1e-9,rtol=0)
    def test_no_duplicate_occupied_source_footprints(self):
        """🔲 Reject duplicate same-level physical footprints despite distinct item labels."""
        occupied=[]
        for slot in M.slots():
            points=np.asarray(M.slot_solid(slot).vertices)
            bounds=np.concatenate((points.min(axis=0),points.max(axis=0)))
            for other,prior in occupied:
                self.assertFalse(np.allclose(bounds,prior,atol=1e-8,rtol=0),(slot,other))
            occupied.append((slot,bounds))

if __name__=='__main__':
    unittest.main()
