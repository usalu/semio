"""🌱 Local terrain context never replaces physical surface occlusion or the ±0 datum."""
import importlib
import json
import os
from pathlib import Path
import unittest
from unittest.mock import patch

p = importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives')


def inputs():
    """🧾 Load domain-neutral terrain and wearing controls with their portable schema."""
    from jsonschema import Draft202012Validator
    path = Path(__file__).with_name('🔣️.json')
    case = json.loads(path.read_text(encoding='utf-8'))
    Draft202012Validator(json.loads(path.with_name('🧬️.schema.json').read_text(encoding='utf-8'))).validate(case)
    terrain = {'tag':case['context']['source_tag'],'class':'IfcCovering','predefined_type':'FLOORING','attributes':{'reference_surface_elevation':{'value':case['context']['elevation']}},'psets':{'Pset_TerrainReconstruction':{'UniformExtension':{'value':True}}}}
    return case, {'elements':[terrain]}, {'box':case['context']['box']}


class TerrainRenderingContract(unittest.TestCase):
    def test_source_bound_context_and_surface_stages(self):
        """🪵 Keep native ground wearing above context and cellar surfaces below it."""
        case, model, geometry = inputs()
        context = p._ground_context(geometry,model)
        self.assertEqual(context['elevation'],case['context']['elevation'])
        self.assertFalse(context['surveyed'])
        self.assertEqual(context['source_tags'],(case['context']['source_tag'],))
        self.assertEqual(context['outline'],((0.,0.),(6,0.),(6,6),(0.,6)))
        self.assertIsNone(p._ground_context(geometry,{'elements':[]}))
        model['elements'][0]['psets']['Pset_TerrainReconstruction']['UniformExtension']['value']=False
        graded=p._ground_context(geometry,model)
        self.assertEqual(graded['elevation'],context['elevation'])
        self.assertFalse(graded['uniform_surface'])
        for item in case['surfaces']:
            with self.subTest(item=item):
                self.assertEqual(p._surface_stage(item,item,3.17,6.6,context['elevation']),item['expected_stage'])

    def test_exposed_column_band_and_native_split_conservation(self):
        """🏛️ Split at terrain, preserving the exposed negative band and actual native volume."""
        import numpy as np
        from scipy.spatial import ConvexHull
        case,model,_ = inputs()
        source = {'outline':[[0,0],[1,0],[1,1],[0,1]],'z0':case['column']['z0'],'z1':case['column']['z1'],'radius':.1,'profile':{'shape':'circle'}}
        item = {'tag':'C','guid':'C','x':.5,'y':.5,'parts':[source]}
        with patch.object(p._c,'membership',return_value={}):
            pieces = p._column_prims(item,model,3.17,6.6,case['context']['elevation'])
        self.assertEqual([[piece.faces[0][1][2],piece.faces[0][1][3]] for piece in pieces],case['column']['expected_bands'])
        self.assertEqual([piece.stage for piece in pieces],case['column']['expected_stages'])
        tall = {**item,'parts':[{**source,'z1':7.}]}
        with patch.object(p._c,'membership',return_value={}):
            upper = p._column_prims(tall,model,3.17,6.6,case['context']['elevation'])
        self.assertEqual([piece.stage for piece in upper],[-1,1,4,6])
        self.assertEqual([(piece.faces[0][1][2],piece.faces[0][1][3]) for piece in upper],[(-2.5,-.15),(-.15,3.17),(3.17,6.6),(6.6,7.)])

        mesh = p._c._model._polyhedron_part([[x,y,-1.] for x,y in ((0.,0.),(1.,0.),(1.,1.),(0.,1.))]+[[x,y,1.+x*.2] for x,y in ((0.,0.),(1.,0.),(1.,1.),(0.,1.))])
        bands = p._column_band(mesh,-1.,-.15)+p._column_band(mesh,-.15,1.2)
        self.assertAlmostEqual(sum(ConvexHull(np.asarray(p._c._model.part_vertices(part),float)).volume for part in bands),ConvexHull(np.asarray(p._c._model.part_vertices(mesh),float)).volume,places=12)
        self.assertEqual(len(bands),2)
        self.assertEqual(mesh['z0'],-1.)
        self.assertEqual(mesh['z1'],1.2)

    def test_actual_source_ground_flooring_record(self):
        """🏠 Record every authored ground flooring body against its canonical terrain context."""
        model = p._c._model.load_model()
        context = p._ground_context({'box':{'length':19.,'width':21.5}},model)
        self.assertIsNotNone(context)
        records = []
        for item in model['elements']:
            if item['class']!='IfcCovering' or item.get('predefined_type')!='FLOORING' or item['storey']!='ground' or item['geometry']['kind']!='flooring':
                continue
            for world in p._c._model._solid_geometry(model,item)['parts']:
                stage = p._surface_stage({**item,'kind':item['geometry']['kind']},world,3.17,6.6,context['elevation'])
                self.assertGreater(stage,p.STAGE_PLANE)
                records.append((item['tag'],stage,world['z0'],world['z1']))
        self.assertGreaterEqual(len({row[0] for row in records}),12)
        self.assertIn(context['source_tags'][0],{row[0] for row in records})
        if os.environ.get('SEMIO_TEACHING_OUTPUT_ROOT'):
            output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']);output.mkdir(parents=True,exist_ok=True)
            (output/'terrain-source-stage-record.json').write_text(json.dumps({'context':context,'records':records},ensure_ascii=True,indent=2),encoding='utf-8')

    def test_ground_producer_replacement_invalidates_primitive_entry(self):
        """♻️ Rebuild pure primitives when their actual terrain/stage producer changes."""
        _,terrain,geometry = inputs()
        calls = []
        def build(g,model):
            calls.append(1)
            return [len(calls)]
        with patch.object(p._c,'geometry_context',side_effect=lambda model:(model,geometry)),patch.object(p,'_build',build):
            self.assertEqual(p.prims({}),[1])
            self.assertEqual(p.prims({}),[1])
            for name in ('_ground_context','_surface_stage','_column_band'):
                with patch.object(p,name,lambda *args:None):
                    self.assertEqual(p.prims({}),[len(calls)])
            self.assertEqual(len(calls),4)
            self.assertEqual(p.prims(terrain),[5])
            terrain["elements"][0]["attributes"]["reference_surface_elevation"]["value"]=-.25
            self.assertEqual(p.prims(terrain),[6])

    def test_native_context_does_not_fill_real_void_and_floor_masks_cellar(self):
        """🎨 Compare actual Cairo pixels for physical cover and a void, recording true context coordinates."""
        import numpy as np
        axon = importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️')
        case,model,geometry = inputs()
        context = p._ground_context(geometry,model)
        engine = axon._engine
        calls = []
        def view(x,y,z):
            calls.append((x,y,z))
            return np.asarray([x-3,y-3,0.])
        drawing = axon.AxonDrawing(view)
        buried = engine.Square(side_length=2,fill_color='#0000ff',fill_opacity=1,stroke_width=0)
        drawing.add(buried)
        axon.Axon.__new__(axon.Axon)._ground(drawing,geometry,view,context)
        self.assertEqual(drawing.ground_context['elevation'],-.15)
        self.assertTrue(calls)
        self.assertTrue(all(point[2]==-.15 for point in calls))
        self.assertEqual({point[:2] for point in calls},set(context['outline']))
        self.assertTrue(all(np.allclose(shape.get_fill_opacity(),0) for shape in drawing.ground.submobjects))
        camera_backend=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎥️camera.🐍️")
        camera = camera_backend._CairoPathCamera(pixel_width=160,pixel_height=90)
        camera.capture_mobjects([drawing])
        void_pixels = camera.pixel_array.copy()
        floor = axon._d.polygon([view(x,y,-.15) for x,y in case['raster']['surface_outline']],case['raster']['surface_color'],1.,None)
        drawing.add(floor)
        camera.reset();camera.capture_mobjects([drawing])
        covered_pixels = camera.pixel_array.copy()
        reference = engine.Camera(pixel_width=160,pixel_height=90)
        reference.capture_mobjects([drawing])
        self.assertTrue(np.array_equal(covered_pixels,reference.pixel_array))
        if os.environ.get('SEMIO_TEACHING_OUTPUT_ROOT'):
            from PIL import Image
            output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']);output.mkdir(parents=True,exist_ok=True)
            Image.fromarray(void_pixels).save(output/'terrain-void.png')
            Image.fromarray(covered_pixels).save(output/'terrain-cover.png')
            (output/'terrain-projection-record.json').write_text(json.dumps({'context':context,'points':calls},ensure_ascii=True,indent=2),encoding='utf-8')
        self.assertTrue(np.any(void_pixels!=covered_pixels))
        centre = (covered_pixels.shape[0]//2,covered_pixels.shape[1]//2)
        self.assertGreater(int(void_pixels[centre][2]),int(void_pixels[centre][0]))
        self.assertGreater(int(covered_pixels[centre][0]),int(covered_pixels[centre][2]))
        self.assertTrue(np.array_equal(drawing.at(3,3,-.15),view(3,3,-.15)))

