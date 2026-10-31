"""🪟 Verify the named source window unit, radial mechanism and separate wall luminaire."""
import importlib
import json
import math
from pathlib import Path
import sys
import unittest
import numpy as np
import rhino3dm as rh

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')
C=json.loads((BASE/'🧪️cases/🪟️unit/🔣️.json').read_text(encoding='utf8'))

class WindowUnit(unittest.TestCase):
    def test_named_source_inventory_exists_in_full_and_exterior_assemblies(self):
        """🧾 Require the real numbered hardware instead of generic light/hub stand-ins."""
        for detail in ('full','exterior'):
            unit=M.capsule_window_unit('A','Slash',detail=detail)
            for expected in C['inventory']:
                parts=[p for p in unit.parts if p.key.startswith(expected['prefix']) and p.role==expected['role']]
                self.assertGreaterEqual(len(parts),expected['minimum'],(detail,expected))
            self.assertTrue(all(p.solid.meta.get('window_unit') is True for p in unit.parts))
            integrated=[p for p in M.capsule_assembly('A',detail,'Slash').parts if p.solid.meta.get('window_unit')]
            self.assertEqual({p.key for p in integrated},{p.key for p in unit.parts})
    def test_primary_dimensions_and_inferred_inner_pane_remain_separate(self):
        """📏 Validate exact primary numbers independently of unmeasured sash construction."""
        data=M.load()
        self.assertEqual(data['detail']['glazing_thickness']['value'],C['source_dimensions']['fixed_glass_thickness'])
        self.assertEqual(data['detail']['glazing_thickness']['basis'],'sourced')
        for key,value in C['source_dimensions'].items():
            if key=='fixed_glass_thickness':
                continue
            self.assertEqual(data['capsule']['window_unit'][key]['value'],value)
            self.assertEqual(data['capsule']['window_unit'][key]['basis'],'sourced')
        self.assertEqual(data['capsule']['window_unit']['inner_glass_thickness']['basis'],'assumed')
        self.assertEqual(data['capsule']['window_unit']['sash_limit']['basis'],'assumed')
    def test_all_radial_states_stay_between_distinct_real_glass_planes(self):
        """🪭 Prove closed-state paper never overlaps either actual pane across all six families."""
        data=M.load()
        for family in C['families']:
            circle=M.capsule_window('A',family)
            normal=np.asarray(circle.normal)
            closed={p.key:p for p in M.capsule_window_unit('A',family,state=C['states'][0]).parts}
            outer=np.asarray(closed['window-glazing'].solid.vertices)@normal
            inner=np.asarray(closed['window-sash-glazing'].solid.vertices)@normal
            self.assertGreater(outer.min(),inner.max())
            self.assertAlmostEqual(outer.max()-outer.min(),C['source_dimensions']['fixed_glass_thickness'])
            self.assertAlmostEqual(inner.max()-inner.min(),data['capsule']['window_unit']['inner_glass_thickness']['value'])
            spreads=[]
            for state in C['states']:
                unit={p.key:p for p in M.capsule_window_unit('A',family,state=state).parts}
                paper=np.asarray(unit['window-paper-fan'].solid.vertices)
                depth=paper@normal
                self.assertGreater(depth.min(),inner.max()+C['tolerance'],(family,state,'inner'))
                self.assertLess(depth.max(),outer.min()-C['tolerance'],(family,state,'outer'))
                if state['sash_angle']==0 and state['blind_rotation']==0:
                    spreads.append((state['blind_fraction'],np.prod(np.ptp(paper[:,[0,2] if family not in ('P','S') else [1,2]],axis=0))))
            self.assertLess(spreads[0][1],spreads[-1][1])
    def test_right_hinge_open_pose_matches_independent_rhino_rotation(self):
        """🧭 Validate physical inward rotation around the observed-reference right edge."""
        for family in C['families']:
            closed={p.key:p for p in M.capsule_window_unit('A',family,state=C['states'][0]).parts}
            opened={p.key:p for p in M.capsule_window_unit('A',family,state=C['states'][-1]).parts}
            points=np.asarray(closed['window-sash-glazing'].solid.vertices)
            centre=(points.min(axis=0)+points.max(axis=0))/2
            normal=np.asarray(M.capsule_window('A',family).normal)
            right=np.cross(normal,[0,0,1])
            pivot=centre+right*M.capsule_window('A',family).radius
            transform=rh.Transform.Rotation(math.pi/2,rh.Vector3d(0,0,1),rh.Point3d(*pivot))
            expected=[]
            for vertex in points:
                point=rh.Point3d(*vertex)
                point.Transform(transform)
                expected.append([point.X,point.Y,point.Z])
            np.testing.assert_allclose(expected,opened['window-sash-glazing'].solid.vertices,atol=C['tolerance'],rtol=0)
    def test_long_wall_luminaire_is_a_separate_elongated_supported_device(self):
        """💡 Protect the long cabinet-wall light independently of the round ceiling luminaire."""
        for family in C['families']:
            parts={p.key:p for p in M.capsule_assembly('A','full',family).parts}
            lamp=[p for p in parts.values() if p.key.startswith(C['long_lamp']['prefix'])]
            self.assertTrue(lamp,family)
            points=np.asarray([v for p in lamp for v in p.solid.vertices])
            span=np.ptp(points,axis=0)
            self.assertGreater(span[1]/max(span[0],span[2]),C['long_lamp']['minimum_aspect'])
            self.assertIn('ceiling-light',parts)
            self.assertTrue(any(p.role=='luminaire_diffuser' for p in lamp))
            self.assertTrue(any(p.role=='luminaire_tube' for p in lamp))

    def test_hinge_leaves_contact_actual_faceted_frame_in_all_poses_and_lods(self):
        """🦾 Reuse the independent actual-triangle contact oracle, never analytic-circle endpoints."""
        oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🧪️tests.🔧️mounts.🐍️')
        for family in C['families']:
            for detail in ('full','exterior'):
                for state in C['states']:
                    parts={p.key:p for p in M.capsule_window_unit('A',family,state,detail).parts}
                    for first,second in C['hinge_contacts']:
                        self.assertTrue(oracle.contact(parts[first].solid,parts[second].solid),(family,detail,state,first))

    def test_guides_glass_hub_carriers_and_reveal_have_actual_material_contacts(self):
        """🔗 Verify primary joints on real meshes at stowed, quarter, covered and opened poses."""
        oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🧪️tests.🔧️mounts.🐍️')
        for family in C['families']:
            for detail in ('full','exterior'):
                print('[DEBUG] window contacts',family,detail,flush=True)
                for index in C['contact_states']:
                    state=C['states'][index]
                    parts={p.key:p for p in M.capsule_window_unit('A',family,state,detail).parts}
                    for first,second in C['contact_links']:
                        self.assertTrue(oracle.contact(parts[first].solid,parts[second].solid),(family,detail,state,first,second))
    def test_all_unit_material_is_closed_and_positive_in_every_state(self):
        """🧊 Validate actual source components with independent Rhino meshes and NumPy material volume."""
        for family in C['families']:
            for detail in ('full','exterior'):
                for state in C['states']:
                    for part in M.capsule_window_unit('A',family,state,detail).parts:
                        mesh=rh.Mesh()
                        for vertex in part.solid.vertices:
                            mesh.Vertices.Add(*vertex)
                        triangles=[]
                        for face in part.solid.faces:
                            for i in range(1,len(face)-1):
                                mesh.Faces.AddFace(face[0],face[i],face[i+1])
                                triangles.append([part.solid.vertices[face[0]],part.solid.vertices[face[i]],part.solid.vertices[face[i+1]]])
                        self.assertTrue(mesh.IsValid,(family,detail,state,part.key))
                        self.assertTrue(mesh.IsClosed,(family,detail,state,part.key))
                        triangles=np.asarray(triangles)
                        volume=np.einsum('ij,ij->i',triangles[:,0],np.cross(triangles[:,1],triangles[:,2])).sum()/6
                        self.assertGreater(volume,1e-12,(family,detail,state,part.key))

if __name__=='__main__':
    unittest.main()
