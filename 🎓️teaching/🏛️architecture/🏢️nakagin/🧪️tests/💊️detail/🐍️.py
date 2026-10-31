"""🛁 Reference capsule detail contracts checked with NumPy solid angles and Rhino mesh topology."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
import unittest
import numpy as np
import rhino3dm

ROOT = Path(__file__).resolve().parents[2]
CASES = json.loads((ROOT/'🧪️cases/💊️detail/🔣️.json').read_text(encoding='utf8'))
spec = importlib.util.spec_from_file_location('nakagin_detail', ROOT/'🐍️.py')
MODEL = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = MODEL
spec.loader.exec_module(MODEL)


def inside(solid, point):
    """🔬 Independently sum oriented triangle solid angles around the sample point."""
    vertices = np.asarray(solid.vertices)-point
    angle = 0.0
    for face in solid.faces:
        for i in range(1,len(face)-1):
            a,b,c = vertices[[face[0],face[i],face[i+1]]]
            lengths = np.linalg.norm([a,b,c],axis=1)
            denominator = np.prod(lengths)+np.dot(a,b)*lengths[2]+np.dot(b,c)*lengths[0]+np.dot(c,a)*lengths[1]
            angle += 2*np.arctan2(np.dot(a,np.cross(b,c)),denominator)
    return abs(angle)>2*np.pi


class DetailTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.parts = MODEL.capsule_assembly({'key':'B','width':2.5,'length':4.,'height':2.5},layout=CASES['layout']).parts
        cls.by_key = {p.key:p for p in cls.parts}

    def test_real_open_cavities(self):
        for case in CASES['cavities']:
            with self.subTest(case['key']):
                solid = self.by_key[case['key']].solid
                self.assertFalse(inside(solid,np.asarray(case['void'])))
                self.assertTrue(inside(solid,np.asarray(case['wall'])))
                self.assertTrue(inside(solid,np.asarray(case['below'])))
                if case['bounds']:
                    bounds = np.asarray(solid.vertices)
                    np.testing.assert_allclose(np.ravel(np.stack([bounds.min(0),bounds.max(0)],axis=1)),case['bounds'],atol=1e-7)

    def test_actual_closed_fixture_meshes(self):
        for part in self.parts:
            if part.key.startswith(('tub-','basin-','toilet-','tap-','shower-','telephone-','entry-','cladding-','speaker-grille-','tv-')) or part.key=='washbasin':
                with self.subTest(part.key):
                    mesh = rhino3dm.Mesh()
                    for v in part.solid.vertices:
                        mesh.Vertices.Add(*v)
                    for f in part.solid.faces:
                        for i in range(1,len(f)-1):
                            mesh.Faces.AddFace(f[0],f[i],f[i+1])
                    self.assertTrue(mesh.IsClosed)
                    self.assertTrue(mesh.IsValid)

    def test_physical_detail_and_study_separation(self):
        for role,count in CASES['minimum_roles'].items():
            self.assertGreaterEqual(sum(p.role==role for p in self.parts),count,role)
        study = MODEL.capsule_assembly('B',interior='study')
        self.assertFalse(any(p.key=='tub-bowl' for p in study.parts))
        self.assertNotEqual(len(study.parts),len(self.parts))

    def test_actual_shell_stays_inside_rounded_physical_casing(self):
        case = CASES['physical_body_profile']
        for family in MODEL.capsule_families():
            parts = MODEL.capsule_assembly(dict(key='B',**dict(zip(('width','length','height'),CASES['nominal_dimensions']))),layout=family).parts
            for part in parts:
                if part.role not in ('shell','shell_back'):
                    continue
                vertices = np.asarray(part.solid.vertices)
                q = abs(vertices[:,[0,2]]-[0,case['height']/2])-[case['width']/2-case['radius'],case['height']/2-case['radius']]
                sdf = np.linalg.norm(np.maximum(q,0),axis=1)+np.minimum(np.maximum(q[:,0],q[:,1]),0)-case['radius']
                self.assertLessEqual(float(sdf.max()),case['tolerance'],(family,part.key))

    def test_rounded_rear_is_closed_and_all_six_actual_doors_are_open(self):
        case = CASES['physical_body_profile']
        thickness = MODEL._detail_value('steel_skin_thickness')
        t = dict(key='B',**dict(zip(('width','length','height'),CASES['nominal_dimensions'])))
        for family in MODEL.capsule_families():
            parts = MODEL._shell_parts(t,family,0,thickness,48,'shell','shell')
            back = next(p.solid for p in parts if p.key=='shell-back')
            mesh = rhino3dm.Mesh()
            for v in back.vertices:
                mesh.Vertices.Add(*v)
            volume = 0.
            vertices = np.asarray(back.vertices)
            for face in back.faces:
                for i in range(1,len(face)-1):
                    mesh.Faces.AddFace(face[0],face[i],face[i+1])
                    a,b,c = vertices[[face[0],face[i],face[i+1]]]
                    volume += np.dot(a,np.cross(b,c))/6
            self.assertTrue(mesh.IsValid,family)
            self.assertTrue(mesh.IsClosed,family)
            self.assertGreater(volume,0,family)
            layout = MODEL._family(family)
            center = layout['entry_coordinate']['value']
            if layout['entry_wall']=='back':
                opening = np.asarray([center,case['back']+thickness/2,1.0])
                shoulder = np.asarray([center,case['back']+thickness/2,case['rear_material_z']])
            else:
                sign = -1 if layout['entry_wall']=='left' else 1
                opening = np.asarray([sign*(case['width']/2-thickness/2),center,1.0])
                shoulder = np.asarray([opening[0],center,2.20])
            self.assertFalse(any(inside(p.solid,opening) for p in parts),(family,'actual door opening'))
            self.assertTrue(any(inside(p.solid,shoulder) for p in parts),(family,'material around opening'))

    def test_front_silhouette_has_empty_rounded_corner_pixels(self):
        from PIL import Image, ImageDraw
        case = CASES['physical_body_profile']
        size = case['raster']
        def pixel(point):
            return (round((point[0]/case['width']+.5)*(size-1)),round((1-point[1]/case['height'])*(size-1)))
        for family in MODEL.capsule_families():
            image = Image.new('1',(size,size))
            draw = ImageDraw.Draw(image)
            for part in MODEL.capsule_assembly(dict(key='B',**dict(zip(('width','length','height'),CASES['nominal_dimensions']))),layout=family).parts:
                if part.role in ('shell','shell_back'):
                    for face in part.solid.faces:
                        draw.polygon([pixel((part.solid.vertices[i][0],part.solid.vertices[i][2])) for i in face],fill=1)
            for point in case['empty_corner_pixels']:
                self.assertEqual(image.getpixel(pixel(point)),0,(family,point))
            for point in case['occupied_edge_pixels']:
                self.assertEqual(image.getpixel(pixel(point)),1,(family,point))

    def test_positive_closed_material_volumes(self):
        for part in self.parts:
            vertices = np.asarray(part.solid.vertices)
            volume = 0.0
            for face in part.solid.faces:
                for i in range(1,len(face)-1):
                    a,b,c = vertices[[face[0],face[i],face[i+1]]]
                    volume += np.dot(a,np.cross(b,c))/6
            self.assertGreater(volume,0,part.key)

    def test_open_rims_and_grille_passages(self):
        for case in CASES['apertures']:
            self.assertFalse(inside(self.by_key[case['key']].solid,np.asarray(case['point'])))
        for speaker,y in enumerate((1.02,2.55)):
            wires = [p.solid for p in self.parts if p.key.startswith(f'speaker-grille-{speaker}-')]
            self.assertEqual(len(wires),11)
            self.assertFalse(any(inside(s,np.asarray([.731,y+.0085,1.63])) for s in wires))
            self.assertTrue(any(inside(s,np.asarray([.731,y,1.63])) for s in wires))
        for i in range(3):
            bounds = np.asarray(self.by_key[f'entry-hinge-{i}'].solid.vertices)
            self.assertGreater(np.ptp(bounds[:,2]),.08)
        for key in ('tub-bowl','washbasin','toilet-pan'):
            solid = self.by_key[key].solid
            self.assertTrue(solid.meta['cavity'])
            self.assertGreater(solid.meta['rim'],solid.meta['floor'])

    def test_curved_plumbing_and_recessed_controls(self):
        for key in ('tap-basin-spout','shower-hose','telephone-cord'):
            solid = self.by_key[key].solid
            self.assertGreater(len(solid.vertices),80)
            self.assertGreater(np.linalg.matrix_rank(np.asarray(solid.vertices)-solid.vertices[0]),2)
        screen = self.by_key['tv-screen'].solid
        self.assertGreater(len(set(round(v[0],5) for v in screen.vertices)),3)
        for family in MODEL.capsule_families():
            parts = MODEL.capsule_assembly('B',layout=family).parts
            self.assertTrue(all(np.isfinite(p.solid.vertices).all() for p in parts))


if __name__=='__main__':
    unittest.main(verbosity=2)
