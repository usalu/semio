"""🎥 Compare bounded native path caching with the unchanged third-party Cairo camera."""
import gc
import importlib
import json
from pathlib import Path
import unittest
import weakref

CASES = json.loads(Path(__file__).with_name('🔣️.json').read_text(encoding='utf-8'))


def native():
    """🔌 Load the private adapter and native reference only for actual rendering contracts."""
    engine = importlib.import_module('manim')
    adapter = importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎥️camera.🐍️')
    return engine, adapter._CairoPathCamera


class BimNativePathCacheContract(unittest.TestCase):
    """🖼️ Require native pixel identity, full path invalidation and finite weak ownership."""

    def test_exact_pixels_for_geometry_style_and_camera_mutations(self):
        """🎨 Match unmodified native pixels through overlap, opacity, geometry and camera changes."""
        import numpy as np
        from PIL import Image, ImageChops
        engine, cached = native()
        case = CASES['canvas']
        cameras = [kind(pixel_width=case['width'], pixel_height=case['height'], background_color=case['background'])
                   for kind in (engine.Camera, cached)]
        shapes = [engine.Polygon(*row['points'], fill_color=row['fill'], fill_opacity=row['opacity'],
                                 stroke_color=row['stroke'], stroke_width=row['width']) for row in CASES['shapes']]
        for operation in CASES['operations']:
            with self.subTest(operation=operation):
                kind = operation['kind']
                shape = shapes[operation.get('index', 0)]
                if kind == 'opacity':
                    shape.set_opacity(operation['value'])
                elif kind in ('shift', 'scale', 'rotate'):
                    getattr(shape, kind)(operation['value'])
                elif kind == 'fill':
                    shape.set_fill(operation['color'], operation['opacity'])
                elif kind == 'stroke':
                    shape.set_stroke(operation['color'], operation['width'], operation['opacity'])
                elif kind.startswith('frame_'):
                    for camera in cameras:
                        setattr(camera, kind, np.array(operation['value']) if kind == 'frame_center' else operation['value'])
                        camera.pixel_array_to_cairo_context.clear()
                elif kind == 'resolution':
                    for camera in cameras:
                        camera.reset_pixel_shape(operation['value'][1], operation['value'][0])
                for camera in cameras:
                    camera.reset()
                    camera.capture_mobjects(shapes)
                expected, actual = [camera.pixel_array.copy() for camera in cameras]
                self.assertTrue(np.array_equal(expected, actual), int(np.abs(expected.astype(int)-actual.astype(int)).max()))
                difference = ImageChops.difference(Image.fromarray(expected), Image.fromarray(actual))
                self.assertIsNone(difference.getbbox())
        self.assertGreater(cameras[1]._path_cache_hits, 0)

    def test_cache_keys_include_native_path_semantics_and_baseline_edge_cases(self):
        """🔑 Preserve native path construction for matrix, curve, tolerance and method mutations."""
        import cairo
        import numpy as np
        from types import MethodType
        engine, cached = native()
        cameras = [engine.Camera(), cached()]
        surfaces = [cairo.ImageSurface(cairo.FORMAT_ARGB32, 100, 100) for _ in cameras]
        contexts = [cairo.Context(surface) for surface in surfaces]
        shape = engine.VMobject().set_points(np.array(CASES['path_points']))
        def compare():
            for camera, context in zip(cameras, contexts):
                camera.set_cairo_context_path(context, shape)
            self.assertEqual([list(context.copy_path()) for context in contexts][0], [list(context.copy_path()) for context in contexts][1])
        for matrix in CASES['matrices']:
            for context in contexts:
                context.set_matrix(cairo.Matrix(*matrix))
            for tolerance in CASES['tolerances']:
                shape.tolerance_for_point_equality = tolerance
                for size in CASES['curve_sizes']:
                    shape.n_points_per_cubic_curve = size
                    compare()
                    compare()
        shape.n_points_per_cubic_curve = 4
        shape.get_subpath_split_indices_from_points = MethodType(lambda self, points, n_dims=3: np.array([[0, len(points)]]), shape)
        compare()
        shape.path_mode = False
        shape.get_subpath_split_indices_from_points = MethodType(
            lambda self, points, n_dims=3: np.array([[0, 8], [8, len(points)]]) if self.path_mode else np.array([[0, len(points)]]), shape)
        compare()
        shape.path_mode = True
        compare()
        shape.consider_points_equals_2d = MethodType(lambda self, a, b: False, shape)
        compare()
        del shape.get_subpath_split_indices_from_points
        del shape.consider_points_equals_2d
        for points in (np.empty((0, 3)), np.array([[np.nan, 0, 0]]), np.array([[np.inf, 0, 0]])):
            shape.set_points(points)
            for context in contexts:
                context.move_to(.4, .6)
                context.line_to(.8, .9)
            compare()
            compare()
        self.assertGreater(cameras[1]._path_cache_hits, 0)
        self.assertGreater(cameras[1]._path_cache_misses, 1)

    def test_cache_is_bounded_and_releases_collected_shapes(self):
        """♻️ Evict within declared entry/byte bounds and retain no dead scene objects."""
        engine, cached = native()
        limits = CASES['limits']
        camera = cached(path_cache_entries=limits['entries'], path_cache_bytes=limits['bytes'])
        shapes = [engine.Square().shift([index*.1, 0, 0]) for index in range(6)]
        for shape in shapes:
            camera.capture_mobjects([shape])
            self.assertLessEqual(len(camera._path_cache), limits['entries'])
            self.assertLessEqual(camera._path_cache_size, limits['bytes'])
        reference = weakref.ref(shapes[-1])
        del shape
        shapes.clear()
        gc.collect()
        self.assertIsNone(reference())
        self.assertEqual(len(camera._path_cache), 0)
        self.assertEqual(camera._path_cache_size, 0)


if __name__ == '__main__':
    unittest.main()
