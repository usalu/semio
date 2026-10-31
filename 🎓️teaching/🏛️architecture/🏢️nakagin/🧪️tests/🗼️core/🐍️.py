"""🗼 Source-topology contracts with independent NumPy polygon and mesh-volume checks."""
import importlib.util
import json
from pathlib import Path
import sys
import unittest

import numpy as np

ROOT = Path(__file__).resolve().parents[2]
CASES = json.loads((ROOT/'🧪️cases/🗼️core/🔣️.json').read_text(encoding='utf8'))
SPEC = importlib.util.spec_from_file_location('nakagin_core', ROOT/'🐍️.py')
MODEL = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODEL
SPEC.loader.exec_module(MODEL)
PROOF_SPEC = importlib.util.spec_from_file_location('nakagin_existing_proof', ROOT/'🧪️tests/🐍️.py')
PROOF = importlib.util.module_from_spec(PROOF_SPEC)
PROOF_SPEC.loader.exec_module(PROOF)


def volume(solid):
    """🔬 NumPy signed tetrahedra independently validate closed profile extrusions."""
    vertices = np.asarray(solid.vertices)
    return abs(sum(np.dot(vertices[face[0]],np.cross(vertices[face[i]],vertices[face[i+1]]))/6 for face in solid.faces for i in range(1,len(face)-1)))


class CoreFidelity(unittest.TestCase):
    def test_reflection_preserves_outward_winding_and_matches_numpy_and_rhino(self):
        local = MODEL._box(*CASES['handing']['test_bounds'])
        plane = MODEL.Plane((0,0,0),(-1,0,0),(0,1,0),(0,0,1))
        placed = MODEL.place(local,plane)
        expected = np.asarray(local.vertices)@np.asarray(CASES['handing']['B']).T
        self.assertTrue(np.allclose(placed.vertices,expected))
        transform = PROOF.rh.Transform.Mirror(PROOF.rh.Point3d(0,0,0),PROOF.rh.Vector3d(1,0,0))
        points = [PROOF.rh.Point3d(*p) for p in local.vertices]
        for point in points:
            point.Transform(transform)
        self.assertTrue(np.allclose(placed.vertices,[[p.X,p.Y,p.Z] for p in points]))
        self.assertGreater(PROOF.signed_volume(placed),0)
        self.assertAlmostEqual(PROOF.signed_volume(placed),volume(local))

    def test_b_core_frames_reflect_a_before_each_stair_rotation(self):
        a,b = MODEL.towers()
        mirror = np.asarray(CASES['handing']['B'])
        for angle in (0,np.pi/2,np.pi,3*np.pi/2):
            frame_a,frame_b = MODEL._core_frame(a,angle),MODEL._core_frame(b,angle)
            basis_a,basis_b = np.asarray((frame_a.x,frame_a.y,frame_a.z)),np.asarray((frame_b.x,frame_b.y,frame_b.z))
            self.assertTrue(np.allclose(basis_b,basis_a@mirror.T))
            self.assertAlmostEqual(float(np.linalg.det(basis_b)),-1)

    def test_reflected_feature_points_and_circle_normal_match_rhino_vectors(self):
        case = CASES['handing']
        box = MODEL._box(*case['test_bounds'])
        circle = MODEL.Circle(tuple(case['circle_center']),tuple(case['circle_normal']),.3)
        local = MODEL.Solid(box.vertices,box.faces,box.edges,circle,(tuple(case['bolt']),),{'test_feature':True})
        placed = MODEL.place(local,MODEL.Plane((0,0,0),(-1,0,0),(0,1,0),(0,0,1)))
        matrix = np.asarray(case['B'])
        self.assertTrue(np.allclose(placed.window.center,matrix@np.asarray(case['circle_center'])))
        self.assertTrue(np.allclose(placed.bolts[0],matrix@np.asarray(case['bolt'])))
        normal = PROOF.rh.Vector3d(*case['circle_normal'])
        normal.Transform(PROOF.rh.Transform.Mirror(PROOF.rh.Point3d(0,0,0),PROOF.rh.Vector3d(1,0,0)))
        self.assertTrue(np.allclose(placed.window.normal,(normal.X,normal.Y,normal.Z)))
        self.assertEqual(placed.window.radius,.3)
        self.assertEqual(placed.meta,local.meta)

    def test_b_compound_cabin_and_basement_network_match_mirrored_a(self):
        a,b = MODEL.towers()
        mirror = np.asarray(CASES['handing']['B'])
        networks = ((MODEL._lift_parts(a,0,4,((1,0),)),MODEL._lift_parts(b,0,4,((1,0),))), (MODEL._stair_ring(a,(-5,0),'full'),MODEL._stair_ring(b,(-5,0),'full')))
        for left,right in networks:
            self.assertEqual([p.key for p in left],[p.key for p in right])
            for pa,pb in zip(left,right):
                local = np.asarray(pa.solid.vertices)-np.asarray((*a.center,0))
                expected = local@mirror.T+np.asarray((*b.center,0))
                self.assertTrue(np.allclose(pb.solid.vertices,expected),pb.key)
                self.assertGreater(PROOF.signed_volume(pb.solid),0,pb.key)
                if pa.solid.meta.get('walking_route'):
                    route = np.asarray(pa.solid.meta['walking_route'])-np.asarray(a.center)
                    self.assertTrue(np.allclose(pb.solid.meta['walking_route'],route@mirror[:2,:2].T+np.asarray(b.center)))

    def test_sparse_staggered_floor_recovers_its_unoccupied_normal_datum(self):
        case = CASES['logical_floor']
        floor = {'groups':[{'items':[{'center':[{}, {}, {'value':z}], 'sublevel':phase} for z,phase in zip(case['bases'],case['sublevels'])]}]}
        self.assertAlmostEqual(MODEL._logical_floor_sill(floor,case['pitch']),case['sill'])
        raw = np.asarray(case['bases'])-np.asarray(case['sublevels'])*case['pitch']/3+.2
        self.assertTrue(np.allclose(raw,case['sill'],atol=1e-6))

    def test_primary_profile_quantities_are_explicit_and_inferred(self):
        data = MODEL._data()['core']['profile']
        for key,value in CASES['profile'].items():
            self.assertEqual(data[key]['value'],value)
            self.assertEqual(data[key]['basis'],'assumed')
            self.assertEqual(data[key]['source'],CASES['source'])

    def test_four_triangular_corner_piers_per_core_match_numpy(self):
        for tower in MODEL.towers():
            piers = [p for p in MODEL.core_assembly(tower).parts if p.role=='core_corner_pier']
            self.assertEqual(len(piers),CASES['expected']['corner_pier_count_per_core'])
            for part in piers:
                self.assertEqual(len(part.solid.vertices),6)
                leg = CASES['profile']['pier_leg']
                self.assertAlmostEqual(volume(part.solid),leg*leg/2*tower.height,places=6)

    def test_compound_has_clipped_outline_and_two_separate_service_chambers(self):
        for tower in MODEL.towers():
            parts = MODEL.core_assembly(tower).parts
            chambers = [p for p in parts if p.role=='service_compartment']
            self.assertEqual(len(chambers),CASES['expected']['service_compartment_count_per_core'])
            self.assertTrue(all(p.solid.meta['unidentified_service'] for p in chambers))
            diagonals = [p for p in parts if p.role=='lift_shaft' and 'compound-chamfer' in p.key]
            self.assertEqual(len(diagonals),4)
            for part in diagonals:
                self.assertGreater(volume(part.solid),0)
                self.assertTrue(any(abs(a[0]-b[0])>.01 and abs(a[1]-b[1])>.01 for a,b in zip(part.solid.vertices,part.solid.vertices[1:])))

    def test_parked_reference_cabin_is_supported_and_complete(self):
        for tower in MODEL.towers():
            parts = MODEL.core_assembly(tower).parts
            floors = [p for p in parts if p.role=='lift_cabin_floor']
            self.assertEqual(len(floors),1)
            points = np.asarray(floors[0].solid.vertices)
            self.assertTrue(np.allclose(points.max(axis=0)-points.min(axis=0),[CASES['cabin']['width'],CASES['cabin']['depth'],.08]))
            self.assertAlmostEqual(float(points[:,2].max()),0)
            self.assertEqual(floors[0].solid.meta['reference_position'],'parked_ground_floor')
            for role in ('lift_cabin_wall','lift_cabin_door','lift_cabin_handrail','lift_call_button','lift_reference_support'):
                self.assertTrue(any(p.role==role for p in parts),role)

    def test_office_opening_follows_authored_podium_level(self):
        level = MODEL._data()['podium']['levels'][1]['value']
        for tower in MODEL.towers():
            parts = [p for p in MODEL.core_assembly(tower).parts if p.role=='office_threshold']
            self.assertEqual(len(parts),1)
            self.assertAlmostEqual(max(p[2] for p in parts[0].solid.vertices),level)

    def test_office_door_has_real_clear_aperture_and_positive_jamb_controls(self):
        level = MODEL._data()['podium']['levels'][1]['value']
        case = CASES['office_entry']
        for tower in MODEL.towers():
            walls = [p.solid for p in MODEL.core_assembly(tower).parts if p.role=='core_wall']
            cx,cy = tower.center
            for x in case['clear_width_samples']:
                for z in case['clear_height_samples']:
                    self.assertFalse(any(PROOF.segment_hits_solid(w,(cx+x,cy-2.85,level+z),(cx+x,cy-2.35,level+z)) for w in walls))
            for x in case['jamb_samples']:
                self.assertTrue(any(PROOF.segment_hits_solid(w,(cx+x,cy-2.85,level+1),(cx+x,cy-2.35,level+1)) for w in walls))

    def test_basement_section_divisions_are_explicit_without_invented_equipment(self):
        parts = MODEL.foundation_assembly().parts
        partitions = [p for p in parts if p.role=='basement_partition']
        self.assertGreaterEqual(len(partitions),2)
        for part in partitions:
            self.assertEqual(part.solid.meta['layout_basis'],'inferred_section_visible_division')
            self.assertGreater(volume(part.solid),0)


if __name__=='__main__':
    unittest.main()
