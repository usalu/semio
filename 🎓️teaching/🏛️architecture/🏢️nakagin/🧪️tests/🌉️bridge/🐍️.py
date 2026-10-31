"""🌉 Native-profile bridge contracts independently checked with NumPy and Rhino geometry."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
import unittest
import numpy as np
import rhino3dm as rh

ROOT=Path(__file__).resolve().parents[2]
CASES=json.loads((ROOT/'🧪️cases/🌉️bridge/🔣️.json').read_text(encoding='utf8'))
spec=importlib.util.spec_from_file_location('bridge_model',ROOT/'🐍️.py')
M=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=M
spec.loader.exec_module(M)


class BridgeTests(unittest.TestCase):
    def test_native_profile_quantities(self):
        data=M._data()['detail']['bridge_geometry']
        for name,key in (('native_span','span'),('native_width','width'),('native_return_length','return_length'),('native_return_width','return_width'),('rail_height','rail_height'),('midrail_height','midrail_height'),('brace_height','brace_height')):
            self.assertAlmostEqual(data[name]['value'],CASES['native'][key])
        self.assertEqual(CASES['native']['steps'],0)

    def test_actual_flat_deck_and_split_bearings(self):
        a,b=sorted(M.towers(),key=lambda t:t.center[0])
        for level in M._data()['detail']['bridge_storeys']:
            parts=M.bridge_assembly(level).parts
            deck=next(p for p in parts if p.role=='bridge')
            points=np.asarray(deck.solid.vertices)
            expected=M.bridge_elevation(level)
            self.assertAlmostEqual(points[:,2].max(),expected)
            self.assertAlmostEqual(points[:,0].min(),a.center[0]+a.core_side/2)
            self.assertAlmostEqual(points[:,0].max(),b.center[0]-b.core_side/2)
            self.assertEqual(deck.solid.meta['walking_axis'],CASES['assembly']['walking_axis'])
            bases=[p for p in parts if p.role=='bridge_base']
            self.assertEqual(len(bases),CASES['assembly']['bases'])
            bounds=[np.asarray(p.solid.vertices) for p in bases]
            self.assertAlmostEqual(bounds[1][:,0].min()-bounds[0][:,0].max(),M._v('detail.bridge_geometry.expansion_gap'))
            self.assertTrue(all(np.max(v[:,2])<expected for v in bounds))

    def test_native_width_center_and_south_return_match_actual_footprint(self):
        a,b=sorted(M.towers(),key=lambda t:t.center[0])
        expected=CASES['footprint']
        x0=a.center[0]+a.core_side/2
        low=expected['center_y']-expected['width']/2
        for level in M._data()['detail']['bridge_storeys']:
            parts=M.bridge_assembly(level).parts
            deck=np.asarray(next(p for p in parts if p.role=='bridge').solid.vertices)
            np.testing.assert_allclose((deck[:,1].min(),deck[:,1].max()),(low,expected['center_y']+expected['width']/2))
            return_part=next(p for p in parts if p.role=='bridge_return')
            points=np.asarray(return_part.solid.vertices)
            np.testing.assert_allclose((points[:,0].min(),points[:,0].max(),points[:,1].min(),points[:,1].max()),(x0,x0+expected['return_width'],low-expected['return_length'],low))
            self.assertAlmostEqual(points[:,2].max(),M.bridge_elevation(level))
            self.assertEqual(return_part.solid.meta['walking_axis'],expected['junction_axis'])
            route=return_part.solid.meta['walking_route']
            self.assertGreater(route[1][1]-(low-expected['return_length']),CASES['headroom']['strip_width']/2)

    def test_return_junction_preserves_full_body_clearance(self):
        tower=min(M.towers(),key=lambda t:t.center[0])
        x0=tower.center[0]+tower.core_side/2
        low=CASES['footprint']['center_y']-CASES['footprint']['width']/2
        middle=x0+CASES['footprint']['return_width']/2
        radius=CASES['headroom']['strip_width']/2
        for level in M._data()['detail']['bridge_storeys']:
            z=M.bridge_elevation(level)
            query=np.array(((middle-radius,low-.4,z+1e-4),(middle+radius,low+.4,z+CASES['headroom']['minimum'])))
            for part in M.bridge_assembly(level).parts:
                if part.role in ('bridge','bridge_base','bridge_return','bridge_return_base'):
                    continue
                points=np.asarray(part.solid.vertices)
                self.assertFalse(np.all(np.minimum(query[1],points.max(0))-np.maximum(query[0],points.min(0))>1e-7),part.key)

    def test_portal_route_clears_chamfer_before_aligning_with_deck(self):
        expected=CASES['footprint']
        self.assertGreaterEqual(M._v('detail.bridge_geometry.passage_width'),expected['minimum_passage_width'])
        for tower in M.towers():
            sign=1 if tower.key=='A' else -1
            for part in (p for p in M.core_assembly(tower).parts if p.role=='bridge_approach'):
                route=np.asarray(part.solid.meta['walking_route'])
                np.testing.assert_allclose(route[:2,1],tower.center[1]+expected['portal_y'])
                self.assertAlmostEqual(route[-1,1],tower.center[1]+expected['center_y'])
                self.assertGreater(sign*(route[1,0]-tower.center[0]),tower.core_side/2)
                self.assertGreater(sign*(route[-1,0]-route[1,0]),0)

    def test_tubular_v_frames_are_closed_and_positive(self):
        parts=M.bridge_assembly(6).parts
        diagonals=[p for p in parts if p.role=='bridge_diagonal']
        self.assertEqual(len(diagonals),CASES['assembly']['diagonals'])
        for role,key in (('bridge_mast','masts'),('bridge_crossbar','crossbars'),('bridge_post','posts')):
            self.assertEqual(sum(p.role==role for p in parts),CASES['assembly'][key])
        self.assertEqual(sum('-midrail-' in p.key for p in parts),CASES['assembly']['midrails'])
        self.assertEqual(sum('-rail-' in p.key for p in parts),CASES['assembly']['rails'])
        slopes=[]
        for part in parts:
            mesh=rh.Mesh()
            for v in part.solid.vertices:
                mesh.Vertices.Add(*v)
            volume=0.
            vertices=np.asarray(part.solid.vertices)
            for face in part.solid.faces:
                for i in range(1,len(face)-1):
                    mesh.Faces.AddFace(face[0],face[i],face[i+1])
                    a,b,c=vertices[[face[0],face[i],face[i+1]]]
                    volume+=np.dot(a,np.cross(b,c))/6
            self.assertTrue(mesh.IsClosed,part.key)
            self.assertTrue(mesh.IsValid,part.key)
            self.assertGreater(volume,0,part.key)
            if part in diagonals:
                axis=np.linalg.svd(vertices-vertices.mean(0))[2][0]
                self.assertLess(abs(axis[1]),1e-6)
                self.assertGreater(abs(axis[2]),.4)
                slopes.append(np.sign(axis[0]*axis[2]))
                self.assertGreater(len(vertices),12)
        self.assertEqual(slopes.count(-1),2)
        self.assertEqual(slopes.count(1),2)

    def test_members_clear_continuous_centre_passage(self):
        parts=M.bridge_assembly(6).parts
        z=M.bridge_elevation(6)
        for part in parts:
            if part.role in ('bridge','bridge_base'):
                continue
            points=np.asarray(part.solid.vertices)
            lateral=np.min(abs(points[:,1]-M._v('detail.bridge_geometry.lateral_offset')))
            if lateral<CASES['headroom']['strip_width']/2:
                self.assertGreater(points[:,2].min()-z,CASES['headroom']['minimum'],part.key)


if __name__=='__main__':
    unittest.main(verbosity=2)
