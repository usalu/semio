"""🧱 Verify exact perforated perimeter coverage with independent NumPy/Rhino material oracles."""
import dataclasses
import importlib
import importlib.util
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
C=json.loads((BASE/'🧪️cases/🧱️panel/🔣️.json').read_text(encoding='utf8'))
spec=importlib.util.spec_from_file_location('panel_material_ray_oracle',BASE/'🧪️tests/📦️casing/🐍️.py')
O=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=O
spec.loader.exec_module(O)

def area(points):
    """🧮 Reproduce planar material area with the independent shoelace formula."""
    points=np.asarray(points)
    return abs(float(np.sum(points[:,0]*np.roll(points[:,1],-1)-points[:,1]*np.roll(points[:,0],-1)))/2)

def volume(solid):
    """📦 Sum actual oriented material triangle volumes independently with NumPy determinants."""
    return sum(float(np.linalg.det(np.asarray([solid.vertices[face[0]],solid.vertices[face[i]],solid.vertices[face[i+1]]])))/6 for face in solid.faces for i in range(1,len(face)-1))

def cap_area(solid,axis,depth):
    """📐 Measure only actual outer cap faces, never its envelope or control-point bounds."""
    axes=[i for i in range(3) if i!=axis]
    return sum(area(np.asarray(solid.vertices)[list(face)][:,axes]) for face in solid.faces if np.allclose(np.asarray(solid.vertices)[list(face),axis],depth,atol=1e-11,rtol=0))

class PerforatedCoverage(unittest.TestCase):
    def assert_material(self,solid,label):
        """🔒 Preserve positive closed material and oppositely oriented two-face edges."""
        edges={}
        balance={}
        mesh=rh.Mesh()
        for point in solid.vertices: mesh.Vertices.Add(*point)
        for face in solid.faces:
            for a,b in zip(face,face[1:]+face[:1]):
                edge=tuple(sorted((a,b)))
                edges[edge]=edges.get(edge,0)+1
                balance[edge]=balance.get(edge,0)+(1 if a<b else -1)
            for i in range(1,len(face)-1): mesh.Faces.AddFace(face[0],face[i],face[i+1])
        self.assertEqual(set(edges.values()),{C['edge_incidence']},label)
        self.assertEqual(set(balance.values()),{C['directed_balance']},label)
        self.assertTrue(mesh.IsClosed,label)
        self.assertTrue(mesh.IsValid,label)
        self.assertGreater(volume(solid),0,label)

    def test_shell_rectangle_minus_unchanged_aperture_has_exact_corners_area_volume_and_rays(self):
        """🔲 Reject missing flat-corner triangles while preserving the original N-angle aperture exactly."""
        for radius in C['radii']:
            t=dataclasses.replace(M.capsule_type('A'),corner_radius=radius)
            casing=M.capsule_casing(t)
            for detail,count in C['details'].items():
                for family in C['families']:
                    side=family in ('P','S')
                    key='shell-right' if side else 'shell-front'
                    part=next(part for part in M.capsule_assembly(t,detail,family).parts if part.key==key)
                    axis=0 if side else 1
                    axes=[i for i in range(3) if i!=axis]
                    lo,hi=(casing.back+radius,casing.front-radius) if side else (casing.left+radius,casing.right-radius)
                    bounds=(lo,hi,radius,casing.height-radius)
                    depth=casing.right if side else casing.front
                    centre=(t.length/2+.6+t.window_u,t.window_v) if side else (t.window_u,t.window_v)
                    points=np.asarray(part.solid.vertices)
                    projected=points[:,axes]
                    for x in bounds[:2]:
                        for z in bounds[2:]:
                            self.assertLess(np.linalg.norm(projected-[x,z],axis=1).min(),C['tolerance'],(radius,detail,family,'corner',x,z))
                            witness=np.zeros(3)
                            witness[axis]=depth
                            witness[axes]=[x+math.copysign(C['corner_inset'],(lo+hi)/2-x),z+math.copysign(C['corner_inset'],casing.height/2-z)]
                            direction=np.eye(3)[axis]*C['ray_depth']
                            self.assertTrue(O.material_segment_hit([part],witness-direction,witness+direction),(radius,detail,family,'corner ray'))
                    circle=projected[np.isclose(np.linalg.norm(projected-centre,axis=1),C['aperture_radius'],atol=1e-10,rtol=0)]
                    unique=np.unique(np.round(circle,10),axis=0)
                    expected=np.array(centre)+C['aperture_radius']*np.column_stack((np.cos(2*np.pi*np.arange(count)/count),np.sin(2*np.pi*np.arange(count)/count)))
                    self.assertEqual(len(unique),count,(radius,detail,family,'aperture count'))
                    self.assertTrue(all(np.linalg.norm(unique-point,axis=1).min()<C['tolerance'] for point in expected))
                    hole=count*C['aperture_radius']**2*math.sin(2*math.pi/count)/2
                    outer=(hi-lo)*(casing.height-2*radius)
                    self.assertAlmostEqual(cap_area(part.solid,axis,depth),outer-hole,places=9,msg=str((radius,detail,family,'area')))
                    inner_r=max(radius,C['steel_skin'])
                    inner_width=(casing.length if side else casing.width)-2*inner_r
                    inner_height=casing.height-2*inner_r
                    middle=((hi-lo+inner_width)/2)*((casing.height-2*radius+inner_height)/2)
                    expected_volume=((outer+4*middle+inner_width*inner_height)/6-hole)*C['steel_skin']
                    self.assertAlmostEqual(volume(part.solid),expected_volume,places=9,msg=str((radius,detail,family,'volume')))
                    self.assert_material(part.solid,(radius,detail,family))

    def test_lining_front_matches_actual_quarter_band_vertices_and_material_area(self):
        """🧩 Match the end plate to every actual lining quarter-band vertex rather than radial shortcuts."""
        for detail,count in C['details'].items():
            segments=4 if detail=='full' else 2
            for family in C['families']:
                assembly=M.capsule_assembly('A',detail,family)
                if detail=='exterior':
                    t=M.capsule_type('A')
                    parts=M._shell_parts(t,family,0,C['lining_thickness'],count,'lining','lining')
                else: parts=assembly.parts
                front=next(part for part in parts if part.key=='lining-front')
                lining=M.capsule_interior('A')
                width,height=lining.width+2*C['lining_thickness'],lining.height+2*C['lining_thickness']
                radius=.05+C['lining_thickness']
                profile=[]
                for (x,z),start in (((width/2-radius,radius),-90),((width/2-radius,height-radius),0),((-width/2+radius,height-radius),90),((-width/2+radius,radius),180)):
                    profile.extend((x+radius*np.cos(np.radians(start+90*i/segments)),lining.bottom-C['lining_thickness']+z+radius*np.sin(np.radians(start+90*i/segments))) for i in range(segments+1))
                projected=np.asarray(front.solid.vertices)[:,[0,2]]
                for point in profile: self.assertLess(np.linalg.norm(projected-point,axis=1).min(),C['tolerance'],(detail,family,'quarter vertex',point))
                hole=0 if family in ('P','S') else count*.73**2*math.sin(2*math.pi/count)/2
                expected=area(profile)-hole
                self.assertAlmostEqual(cap_area(front.solid,1,lining.front+C['lining_thickness']),expected,places=9)
                self.assertAlmostEqual(volume(front.solid),expected*C['lining_thickness'],places=9)
                self.assert_material(front.solid,(detail,family,'lining'))

    def test_every_lining_patch_retains_coherent_closed_material(self):
        """🔒 Guard offset rectangular doors after the lining receives its exact quarter perimeter."""
        for detail,count in C['details'].items():
            for family in C['families']:
                parts=M._shell_parts(M.capsule_type('A'),family,0,C['lining_thickness'],count,'lining','lining')
                for part in parts:
                    if part.role==C['lining_material_role']:
                        self.assert_material(part.solid,(detail,family,part.key))

if __name__=='__main__':
    unittest.main(verbosity=2)
