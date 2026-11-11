"""📐 Validate the bilingual capsule slider layout against viewport and card."""
import ast
import importlib
import json
import unittest
from pathlib import Path
from types import SimpleNamespace
import jsonschema
import numpy as np
from manim import Camera, FadeIn, FadeOut, Square, VGroup, ValueTracker, Wait, tempconfig
from shapely.geometry import box

m = importlib.import_module('🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️')
HERE = Path(__file__).parent
CASE = json.loads((HERE / '🔣️.json').read_text(encoding='utf-8'))
SCHEMA = json.loads((HERE / '🔣️.schema.json').read_text(encoding='utf-8'))


class SliderLayout(unittest.TestCase):
    def test_types_stage_retires_controls(self):
        tree = ast.parse(Path(m.__file__).read_text(encoding='utf-8'))
        cls = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == 'Beat2_Capsule')
        method = next(n for n in cls.body if isinstance(n, ast.FunctionDef) and n.name == 'present_capsule')
        types = next(n for n in ast.walk(method) if isinstance(n, ast.Call) and isinstance(n.func, ast.Attribute)
                     and n.func.attr == 'step' and n.args and isinstance(n.args[0], ast.Constant) and n.args[0].value == 'types')
        stage_code = compile(ast.Expression(types.args[1]), m.__file__, 'eval')
        cur = next(n for n in ast.walk(method) if isinstance(n, ast.Assign)
                   and any(isinstance(t, ast.Name) and t.id == 'cur' for t in n.targets)
                   and 'table' in ast.unparse(n.value) and 'typetag' in ast.unparse(n.value))
        cleanup_code = compile(ast.Expression(cur.value), m.__file__, 'eval')
        table_node = next(n for n in ast.walk(method) if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'table' for t in n.targets))
        table_code = compile(ast.Expression(table_node.value), m.__file__, 'eval')
        steps_node = next(n for n in ast.walk(method) if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'steps' for t in n.targets))
        steps_code = compile(ast.Expression(steps_node.value), m.__file__, 'eval')
        with tempconfig({'dry_run': True, 'pixel_width': 320, 'pixel_height': 180, 'frame_rate': 3}):
            scene = m.Beat2_Capsule()
            scene.language = lambda: 'en'
            tracker = ValueTracker(3.5)
            trackers = {k: ValueTracker(3.5) for k in m.IDS}
            trackers['w'] = tracker
            scene.st = SimpleNamespace(vals=trackers, wv={}, cv={})
            definition = SimpleNamespace(spec=lambda k: SimpleNamespace(kind='slider'),
                obj=lambda k: SimpleNamespace(settings={'value': 2.6, 'min': 1, 'max': 4}))
            control = scene._node('w', (*m.SLIDER_AT['w'], 0), definition, lambda: None).view
            table = eval(table_code, {'self': scene})
            cb = box(control.get_left()[0], control.get_bottom()[1], control.get_right()[0], control.get_top()[1])
            tb = box(table.get_left()[0], table.get_bottom()[1], table.get_right()[0], table.get_top()[1])
            self.assertGreater(cb.intersection(tb).area, 0)
            col, minis, marker, typetag = [control], VGroup(Square(.1)), VGroup(Square(.1)), VGroup(Square(.1))
            others = [VGroup(Square(.1)) for _ in range(4)]
            rv = {key: ValueTracker(0) for key in ('over', 'env', 'hit')}
            hold = {'src': [], 'dst': []}
            scene.add(control, *others)
            env = {'_Seq': m._Seq, '_engine': m._engine, 'self': scene, 'fout': lambda xs: [FadeOut(x) for x in xs],
                   'card': others[0], 'limit_badge': others[1], 'red_box': others[2], 'request': others[3],
                   'col': col, 'table': table, 'minis': minis, 'marker': marker, 'typetag': typetag,
                   'rv': rv, 'vals': trackers, 'width_hi': 3.5, 'value': lambda _: 2.6,
                   'minis_ready': lambda: None, 'hold': hold, '_morph': SimpleNamespace(scatter=lambda *a, **kw: Wait(.1))}
            seq = eval(stage_code, env)
            control_family = {id(x) for x in control.get_family()}
            table_family = {id(x) for x in table.get_family()}
            violations = []
            original = scene.renderer.update_frame
            def frame(*args, **kwargs):
                result = original(*args, **kwargs)
                visible = {id(x) for x in scene.get_mobject_family_members()}
                if visible & control_family and visible & table_family:
                    violations.append(True)
                return result
            scene.renderer.update_frame = frame
            for part in seq.parts:
                if callable(part):
                    part()
                else:
                    animations, seconds = part
                    scene.play(*(animations() if callable(animations) else animations), run_time=seconds)
            self.assertFalse(violations, 'table and control families were displayed together')
            self.assertAlmostEqual(tracker.get_value(), 2.6)
            programs = m._nk.capsule_types()
            order = [programs[0], programs[1], programs[3], programs[2]]
            ty = ValueTracker(0)
            env.update(order=order, ty=ty, _reference_controls=m._reference_controls)
            scene.play(m._engine.Succession(*eval(steps_code, env)), run_time=1.0)
            self.assertAlmostEqual(ty.get_value(), 3.0)
            for key, expected in m._reference_controls(order[-1]).items():
                self.assertAlmostEqual(trackers[key].get_value(), expected)
            self.assertFalse(control_family & {id(x) for x in scene.get_mobject_family_members()})
            scene.play(*[FadeOut(x) for x in eval(cleanup_code, env)], run_time=.34)
            self.assertFalse(control_family & {id(x) for x in scene.get_mobject_family_members()})

    def test_viewport_card_and_wire(self):
        jsonschema.validate(CASE, SCHEMA)
        with tempconfig({'dry_run': True, 'pixel_width': 854, 'pixel_height': 480,
                         'frame_width': CASE['frame_width'], 'frame_height': CASE['frame_height']}):
            for lang in CASE['languages']:
                scene = m.Beat2_Capsule()
                scene.language = lambda: lang
                scene.st = SimpleNamespace(vals={k: ValueTracker(1) for k in CASE['ids']}, wv={}, cv={})
                definition = SimpleNamespace(spec=lambda k: SimpleNamespace(kind='slider'),
                    obj=lambda k: SimpleNamespace(settings={'value': 1, 'min': 0, 'max': 4}))
                rows = [(scene.tr(*pair), key) for key, pair in (
                    ('w', ('Width', 'Breite')), ('l', ('Length', 'Länge')), ('h', ('Height', 'Höhe')),
                    ('r', ('Corner radius', 'Eckenradius')), ('d', ('Window diameter', 'Fensterdurchmesser')),
                    ('wu', ('Window offset', 'Fensterversatz')))]
                card = scene._params_card(rows, m.PARAMS_AT)
                viewport = box(-scene.camera.frame_width/2+CASE['minimum_margin'], -scene.camera.frame_height/2+CASE['minimum_margin'],
                    scene.camera.frame_width/2-CASE['minimum_margin'], scene.camera.frame_height/2-CASE['minimum_margin'])
                controls = []
                for key in CASE['ids']:
                    at = m.SLIDER_AT[key] if key in m.SLIDER_AT else (-5.05, 1.9-.8*m.BOLT_IDS.index(key))
                    widget = scene._node(key, (*at, 0), definition, lambda: None).view
                    bounds = box(widget.get_left()[0], widget.get_bottom()[1], widget.get_right()[0], widget.get_top()[1])
                    self.assertTrue(viewport.covers(bounds), (lang, key, bounds.bounds))
                    if key in m.IDS:
                        self.assertGreaterEqual(card.get_left()[0] - widget.get_right()[0], CASE['card_margin'], (lang,key))
                        controls.append(widget)
                hook = scene.gh_slider(scene.tr(*m.NAMES['w']), 1, 0, 4, at=(m.SLIDER_X, .6, 0), width=m.SLIDER_W)
                self.assertGreaterEqual(hook.get_left()[0], -scene.camera.frame_width/2 + CASE['minimum_margin'])
                target = scene.gh_component('Center Box', at=(-1.7,.6,0))
                wire = scene.gh_wire(hook.grip_out(), target.grip_in('X'))
                self.assertTrue(np.allclose(wire.submobjects[0].get_start(), hook.grip_out().get_center()))
                self.assertTrue(np.allclose(wire.submobjects[0].get_end(), target.grip_in('X').get_center()))
                camera = Camera(pixel_width=854, pixel_height=480, frame_width=scene.camera.frame_width,
                                frame_height=scene.camera.frame_height, background_color='#000000', background_opacity=1)
                camera.capture_mobjects(controls)
                pixels = np.asarray(camera.get_image())
                self.assertEqual(int(pixels[:, :4, :3].max()), 0, lang)


if __name__ == '__main__':
    unittest.main()
