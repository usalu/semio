"""🗂️ Portable address buckets preserve rendered ownership and ordered batching."""
from __future__ import annotations
import ast
import importlib
import json
from pathlib import Path
from types import SimpleNamespace
import unittest


class Group:
    """🧩 Minimal ordered identity container with observable insertion work."""

    calls = 0

    def __init__(self, *children):
        self.submobjects = []
        if children:
            self.add(*children)

    def add(self, *children):
        type(self).calls += 1
        for child in children:
            self.submobjects = [x for x in self.submobjects if x is not child]
            self.submobjects.append(child)
        return self


def classes():
    """🔬 Execute the production classes against an independent identity container."""
    base = Path(__file__).parent.parent
    clip = base.parents[1]
    namespace = {'_engine': SimpleNamespace(VGroup=Group), 'P': lambda x: x}
    for path, name in ((base / '🐍️.py', 'Drawing'), (clip / '🎨️visuals' / '🏛️savoye' / '🧊️axon' / '🐍️.py', 'AxonDrawing')):
        tree = ast.parse(path.read_text(encoding='utf-8'))
        node = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == name)
        module = ast.Module(body=[ast.ImportFrom(module='__future__',names=[ast.alias(name='annotations')],level=0),node],type_ignores=[])
        exec(compile(ast.fix_missing_locations(module), str(path), 'exec'),namespace)
        namespace['_d'] = SimpleNamespace(Drawing=namespace['Drawing'])
    return namespace['Drawing'], namespace['AxonDrawing']


class DrawingAssemblyContract(unittest.TestCase):
    def test_selector_bucket_does_not_mutate_rendered_shape_or_other_alias(self):
        Drawing, _ = classes()
        drawing = Drawing()
        a, b = Group(Group()), Group(Group())
        drawing.part('A', a)
        drawing.part('P', a)
        drawing.part('B', b)
        drawing.part('P', b)
        self.assertEqual(len(a.submobjects), 1)
        self.assertEqual(drawing.parts['A'].submobjects, [a])
        self.assertEqual(drawing.parts['P'].submobjects, [a,b])
        self.assertIsNot(drawing.parts['A'], drawing.parts['P'])
        bucket = drawing.parts['A']
        self.assertIs(drawing.part('A',bucket),bucket)
        self.assertEqual(bucket.submobjects,[a])

    def test_batched_order_layer_alias_membership_and_bounded_registration(self):
        _, AxonDrawing = classes()
        case = json.loads(Path(__file__).with_name('🔣️.json').read_text(encoding='utf-8'))
        drawing = AxonDrawing(None)
        shapes = {r['shape']: Group(Group()) for r in case['records']}
        names = {id(v):k for k,v in shapes.items()}
        records = [(shapes[r['shape']],tuple(r['layers']),tuple([r['tag'],*r['aliases']])) for r in case['records']]
        Group.calls = 0
        drawing._assemble(records)
        self.assertEqual([names[id(x)] for x in drawing.submobjects],case['expected']['root'])
        for key,expected in case['expected']['parts'].items():
            self.assertEqual([names[id(x)] for x in drawing.parts[key].submobjects],expected)
        for key,expected in case['expected']['layers'].items():
            self.assertEqual([names[id(x)] for x in drawing.layers[key].submobjects],expected)
        self.assertEqual(drawing.layer_parts['structure']['A'].submobjects,[shapes['a1'],shapes['a2']])
        self.assertTrue(all(len(x.submobjects)==1 for x in shapes.values()))
        self.assertLessEqual(Group.calls,1+len(drawing.parts)+len(drawing.layers)+sum(map(len,drawing.layer_parts.values())))

    def test_registration_calls_depend_on_buckets_not_piece_count(self):
        """📈 Register a thousand physical pieces with one insertion per address bucket."""
        _, AxonDrawing = classes()
        drawing = AxonDrawing(None)
        shapes = [Group(Group()) for _ in range(1000)]
        Group.calls = 0
        drawing._assemble([(shape,('structure',),('A','P')) for shape in shapes])
        self.assertEqual(drawing.submobjects,shapes)
        self.assertEqual(drawing.parts['A'].submobjects,shapes)
        self.assertEqual(drawing.parts['P'].submobjects,shapes)
        self.assertTrue(all(len(shape.submobjects)==1 for shape in shapes))
        self.assertLessEqual(Group.calls,6)

    def test_native_cairo_pixels_order_and_copy_isolation(self):
        """🎨 Compare complete ordered pixels with the native Cairo reference implementation."""
        import numpy as np
        from jsonschema import Draft202012Validator
        path = Path(__file__).with_name('🔣️.json')
        case = json.loads(path.read_text(encoding='utf-8'))
        Draft202012Validator(json.loads(path.with_name('🧬️.schema.json').read_text(encoding='utf-8'))).validate(case)
        axon = importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️')
        engine = axon._engine
        shapes = [engine.VGroup(engine.Rectangle(width=2,height=2,fill_color=color,fill_opacity=1,stroke_width=0).shift([index*.6,0,0])) for index,color in enumerate(('#ff0000','#00ff00','#0000ff'))]
        reference = engine.VGroup(*(shape.copy() for shape in shapes))
        drawing = axon.AxonDrawing(None)
        drawing._assemble([(shape,tuple(record['layers']),tuple([record['tag'],*record['aliases']])) for shape,record in zip(shapes,case['records'])])
        def pixels(shape):
            camera = engine.Camera(pixel_width=160,pixel_height=90)
            camera.capture_mobjects([shape])
            return camera.pixel_array.copy()
        self.assertTrue(np.array_equal(pixels(drawing),pixels(reference)))
        before = pixels(drawing)
        drawing.copy().shift([3,2,0])
        self.assertTrue(np.array_equal(pixels(drawing),before))
        self.assertEqual(len(drawing.parts['A'].submobjects),2)
        self.assertEqual(len(shapes[0].submobjects),1)
