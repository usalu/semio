"""❄️ Prove the lean tower memory model: lean faces equal plain manim shapes, a frozen cached tower thaws to exactly the drawing a memory clone gives, whole-building orders stay on disk."""
import importlib
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import numpy as np

ROOT = next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir())
sys.path.insert(0, str(ROOT))
V = importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
F = importlib.import_module('🎓️teaching.🏛️architecture.🧲️fit.🐍️')
C = json.loads((ROOT / '🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/❄️frozen/🔣️.json').read_text(encoding='utf-8'))
M = V._engine


def shapes(drawing):
    return [member for member in drawing.get_family() if len(member.points)]


def pixels(group):
    camera = M.Camera(background_color=V.BACKGROUND)
    camera.capture_mobjects([group])
    return camera.pixel_array.copy()


class LeanFaces(unittest.TestCase):
    """🪶 A lean face is the manim shape with its defaults moved to the class."""

    def make(self):
        lean = C['lean']
        return V._shape([np.asarray(lean['polygon'], float)], lean['fill'], 1.0, lean['stroke'], lean['width'])

    def plain(self):
        lean = C['lean']
        shape = M.VMobject()
        shape.set_points(V._poly_points([np.asarray(lean['polygon'], float)]))
        shape.set_fill(lean['fill'], 1.0)
        shape.set_stroke(lean['stroke'], lean['width'], 1.0)
        return shape

    def test_lean_face_equals_plain_manim_shape_and_is_small(self):
        lean, plain = self.make(), self.plain()
        self.assertIsInstance(lean, M.VMobject)
        np.testing.assert_array_equal(lean.points, plain.points)
        for name in ('fill_rgbas', 'stroke_rgbas', 'background_stroke_rgbas'):
            np.testing.assert_array_equal(getattr(lean, name), getattr(plain, name))
        self.assertEqual((lean.get_stroke_width(), lean.get_fill_opacity(), lean.joint_type, lean.sheen_factor), (plain.get_stroke_width(), plain.get_fill_opacity(), plain.joint_type, plain.sheen_factor))
        self.assertLessEqual(len(vars(lean)), C['lean']['maximum_attributes'])
        np.testing.assert_array_equal(pixels(M.VGroup(lean)), pixels(M.VGroup(plain)))

    def test_copies_are_independent_and_share_only_world_data(self):
        shape = self.make()
        shape.world_vertices = V._frozen(np.asarray(C['lean']['polygon'], float))
        twin = shape.copy()
        self.assertIs(twin.world_vertices, shape.world_vertices)
        self.assertIsNot(twin.points, shape.points)
        twin.set_fill(C['recolour'], 1.0)
        twin.shift(np.array([1.0, 0, 0]))
        self.assertEqual(shape.get_fill_color().to_hex().lower(), C['lean']['fill'])
        np.testing.assert_array_equal(shape.points, self.plain().points)
        with self.assertRaises(ValueError):
            twin.world_vertices[0, 0] = 5.0


class Orders(unittest.TestCase):
    """🧠 Only small orderings stay in memory."""

    def test_orders_above_the_limit_stay_on_disk(self):
        V._OCCLUSION_ORDERS.clear()
        with mock.patch.object(V, '_ORDER_MEMORY_ROWS', C['memory']['limit']):
            V._remember_order(b'large', tuple(range(C['memory']['order_rows'])))
            V._remember_order(b'small', tuple(range(C['memory']['limit'])))
        self.assertNotIn(b'large', V._OCCLUSION_ORDERS)
        self.assertIn(b'small', V._OCCLUSION_ORDERS)
        V._OCCLUSION_ORDERS.clear()


class FrozenTower(unittest.TestCase):
    """❄️ A frozen template thaws to the drawing a memory template clones to."""

    @classmethod
    def setUpClass(cls):
        cls.directory = tempfile.TemporaryDirectory()
        cls.addClassCleanup(cls.directory.cleanup)
        patch = mock.patch.dict('os.environ', {'SEMIO_TEACHING_CACHE_ROOT': cls.directory.name})
        patch.start()
        cls.addClassCleanup(patch.stop)

    def build(self, threshold, **extra):
        V._TOWERS.clear()
        fit = F.building_fit(C['fit']['width'], C['fit']['height'], centre=(0, 0, 0), towers=(C['tower'],), podium=False, capital=False, core=False)
        where = lambda slot: slot.tower == C['tower'] and slot.storey < C['storeys']
        options = dict(towers=(C['tower'],), where=where, core=False, podium=False, capital=False)
        with mock.patch.object(V, '_FROZEN_SHAPES', threshold):
            draw = V.NakaginDrawing()
            drawing = draw.tower_axon(**fit, types=False, **options, **extra)
            capsules = draw.tower_capsules(sorted(drawing.capsules)[:C['capsules']], C['recolour'], **fit, **options)
            second = draw.tower_axon(**fit, types=False, **options)
            template = next(iter(V._TOWERS.values()))
        return drawing, capsules, second, template

    def test_thawed_tower_is_the_same_drawing_as_a_memory_clone(self):
        memory, memory_capsules, memory_second, template = self.build(10**9)
        self.assertNotIsInstance(template, V._Frozen)
        frozen, frozen_capsules, frozen_second, template = self.build(0)
        self.assertIsInstance(template, V._Frozen)
        self.assertEqual(sorted(memory.capsules), sorted(frozen.capsules))
        left, right = shapes(memory), shapes(frozen)
        self.assertEqual(len(left), len(right))
        self.assertGreater(len(left), 100)
        for a, b in zip(left, right):
            np.testing.assert_array_equal(a.points, b.points)
            for name in ('fill_rgbas', 'stroke_rgbas'):
                np.testing.assert_array_equal(getattr(a, name), getattr(b, name))
            self.assertEqual((a.get_stroke_width(), len(a.submobjects), getattr(a, 'semantic_role', None), getattr(a, 'paint', None)), (b.get_stroke_width(), len(b.submobjects), getattr(b, 'semantic_role', None), getattr(b, 'paint', None)))
            self.assertEqual(hasattr(a, 'world_depth'), hasattr(b, 'world_depth'))
            if hasattr(a, 'world_depth'):
                self.assertEqual(a.world_depth, b.world_depth)
                np.testing.assert_array_equal(a.world_vertices, b.world_vertices)
                self.assertNotIn('world_vertices', vars(b))
        np.testing.assert_array_equal(pixels(memory), pixels(frozen))
        for a, b in zip(memory_capsules, frozen_capsules):
            self.assertEqual(len(shapes(a)), len(shapes(b)))
            np.testing.assert_array_equal(pixels(a), pixels(b))
        np.testing.assert_array_equal(pixels(memory_second), pixels(frozen_second))

    def test_every_thaw_is_independent_of_the_cache(self):
        frozen, _, second, _ = self.build(0)
        before = pixels(second)
        frozen.shift(np.array([1.0, 0.5, 0]))
        frozen.set_fill(C['recolour'], 1.0)
        np.testing.assert_array_equal(pixels(second), before)
        self.assertEqual(len(shapes(second)), len(shapes(frozen)))


if __name__ == '__main__':
    unittest.main()
