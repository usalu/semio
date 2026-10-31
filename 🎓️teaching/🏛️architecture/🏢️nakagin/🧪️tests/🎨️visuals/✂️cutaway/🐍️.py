"""✂️ Verify exact cutaway source material against SciPy and Pillow."""
import importlib
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch
import numpy as np
from PIL import Image,ImageDraw
from scipy.spatial import ConvexHull

ROOT=next(path for path in Path(__file__).resolve().parents if (path/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
CASES=json.loads((BASE/'🧪️cases/🎨️visuals/✂️cutaway/🔣️.json').read_text(encoding='utf-8'))

class CutawayMaterial(unittest.TestCase):
    def test_clipped_material_matches_portable_areas_and_independent_rasters(self):
        for case in CASES['cases']:
            with self.subTest(case=case['name']):
                vertices=np.asarray(case['vertices'],float)
                mesh=V.Mesh(vertices,np.asarray([case['normal']],float),(np.arange(len(vertices)),),('source-boundary',))
                clipped=V._clip_mesh(mesh,case['planes'])
                self.assertEqual(len(clipped.faces),case['loops'])
                axis=int(np.argmax(np.abs(case['normal'])))
                area=0
                canvas=Image.new('1',(769,769))
                draw=ImageDraw.Draw(canvas)
                for face,normal,name in zip(clipped.faces,clipped.N,clipped.names):
                    polygon=clipped.V[face]
                    projected=np.delete(polygon,axis,axis=1)
                    area+=ConvexHull(projected).volume
                    draw.polygon([tuple(point*192+96) for point in projected],fill=1)
                    np.testing.assert_array_equal(normal,case['normal'])
                    self.assertEqual(name,'source-boundary')
                    for plane,limit in case['planes']:
                        self.assertTrue((polygon@plane<=limit+1e-12).all())
                self.assertAlmostEqual(area,case['area'],places=11)
                if case['name']=='disconnected-concave':
                    for x,inside in ((.5,True),(1.5,False),(2.5,True)):
                        self.assertEqual(bool(canvas.getpixel((int(x*192+96),int(2.5*192+96)))),inside)
                if case['name']=='unchanged':
                    self.assertIs(clipped,mesh)

    def test_source_boundary_identity_removes_only_roundoff_contacts_and_keeps_positive_material(self):
        from fractions import Fraction
        import math
        case=CASES['boundary_identity']
        for contact in case['contacts']:
            with self.subTest(contact=contact['vertices']):
                points=np.asarray(contact['vertices'],float)
                mesh=V.Mesh(points,np.asarray([contact['normal']],float),(np.arange(len(points)),),('source-contact',))
                clipped=V._clip_mesh(mesh,contact['planes'])
                self.assertEqual(len(clipped.faces),0,'One-ULP corner/edge contact must not become a new material wedge')
                epsilon=case['ulp_multiplier']*math.ulp(float(np.max(np.abs(points))))
                self.assertEqual(epsilon,V._fragments._epsilon(tuple(map(tuple,points))))
        contact=case['contacts'][0]
        points=np.asarray(contact['vertices'],float)
        mesh=V.Mesh(points,np.asarray([contact['normal']],float),(np.arange(len(points)),),('positive-material',))
        planes=[(normal,case['positive_limit'] if i==0 else limit) for i,(normal,limit) in enumerate(contact['planes'])]
        clipped=V._clip_mesh(mesh,planes)
        self.assertEqual(len(clipped.faces),1)
        polygon=clipped.V[clipped.faces[0]]
        planar=polygon[:,(0,2)]
        span=float(np.ptp(planar,axis=0).max())
        actual=float(ConvexHull((planar-planar[0])/span).volume*span**2)
        a,b,c=points
        delta=Fraction(case['positive_limit'])-Fraction(float(b[2]))
        exact=abs(delta*delta*(Fraction(float(a[0]))-Fraction(float(b[0])))/(Fraction(float(a[2]))-Fraction(float(b[2])))/2)
        perimeter=sum(np.linalg.norm(second-first) for first,second in zip(polygon,np.roll(polygon,-1,axis=0)))
        bound=case['ulp_multiplier']*math.ulp(float(np.max(np.abs(points))))*perimeter
        self.assertGreater(actual,0)
        self.assertLessEqual(abs(actual-float(exact)),bound)
        normal=V._polygon_normal(polygon,contact['normal'])
        prepared=V._fragments.ordered_fragments((V._fragments.Face(0,0,tuple(map(tuple,polygon)),tuple(normal)),),(0,1,0))
        self.assertTrue(prepared)
        self.assertTrue(all(part.source==0 and part.polygon==0 for part in prepared))

    def test_public_cutaway_clips_intersecting_source_faces_and_keeps_uncut_meshes(self):
        model=V._model
        original=V._assembly_items
        for layout in ('Slash','Backslash','J','L','P','S'):
            captured=[]
            def observe(parts,**options):
                captured.extend(parts)
                return []
            with self.subTest(layout=layout),patch.object(V,'_assembly_items',side_effect=observe):
                V.NakaginDrawing().capsule_axon(model.capsule_type('A'),cutaway=True,layout=layout,bolts=True)
            parameters=V._cap_parameters(V._cap(model.capsule_type('A')))
            casing,usable=model.capsule_casing(parameters),model.capsule_interior(parameters)
            for part,mesh in captured:
                if part.role not in ('shell','lining','steel_truss'):
                    continue
                bounds=usable if part.role=='lining' else casing
                depth=.2 if part.role=='steel_truss' else max(.15,float(part.solid.meta.get('wall_thickness',.1))*1.5)
                for face in mesh.faces:
                    polygon=mesh.V[face]
                    self.assertTrue((polygon[:,2]<=bounds.top-depth+1e-12).all(),(layout,part.key,'roof'))
                    self.assertTrue((polygon[:,0]<=bounds.right-depth+1e-12).all(),(layout,part.key,'wall'))
            source=dict((part.key,(part,mesh)) for part,mesh in V._cap_parts(V._cap(model.capsule_type('A')),'full','reference',layout))
            captured=[]
            with patch.object(V,'_assembly_items',side_effect=observe):
                V.NakaginDrawing().capsule_axon(model.capsule_type('A'),cutaway=False,layout=layout,bolts=True)
            self.assertEqual({part.key for part,mesh in captured},set(source))
            for part,mesh in captured:
                expected_part,expected=source[part.key]
                self.assertIs(part,expected_part)
                self.assertIs(mesh,expected)

    def test_every_emitted_cutaway_facet_prepares_for_both_wall_signs(self):
        model=V._model
        cap=V._cap(model.capsule_type('A'))
        parameters=V._cap_parameters(cap)
        casing,usable=model.capsule_casing(parameters),model.capsule_interior(parameters)
        for layout in ('Slash','Backslash','J','L','P','S'):
            for sign in (-1,1):
                emitted=0
                for part,source in V._cap_parts(cap,'full','reference',layout):
                    if part.role not in ('shell','lining','steel_truss'):
                        continue
                    bounds=usable if part.role=='lining' else casing
                    depth=.2 if part.role=='steel_truss' else max(.15,float(part.solid.meta.get('wall_thickness',.1))*1.5)
                    edge=bounds.right if sign>0 else -bounds.left
                    mesh=V._clip_mesh(source,(((0,0,1),bounds.top-depth),((sign,0,0),edge-depth)))
                    for index,face in enumerate(mesh.faces):
                        for facet in V._polygon_facets(mesh.V[face]):
                            with self.subTest(layout=layout,sign=sign,part=part.key,face=index):
                                normal=V._polygon_normal(facet,mesh.N[index])
                                prepared=V._fragments.ordered_fragments((V._fragments.Face(0,0,tuple(map(tuple,facet)),tuple(normal)),),(0,1,0))
                                self.assertTrue(prepared)
                                emitted+=1
                self.assertGreater(emitted,0)

if __name__=='__main__':
    unittest.main(verbosity=2)
