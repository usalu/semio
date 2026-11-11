"""🧺 Verify adjacent paint batches retain pixels and semantic membership."""
import importlib
import json
import sys
import unittest
from pathlib import Path

import cairo
import numpy as np
from PIL import Image

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
DRAW = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
MANIM = importlib.import_module("manim")


def render(shapes):
    """🖼️ Rasterize compound cubic paths using Cairo and read their pixels with Pillow."""
    width, height = CASES["canvas"]
    surface = cairo.ImageSurface(cairo.FORMAT_ARGB32, width, height)
    context = cairo.Context(surface)
    context.scale(32, 32)
    context.translate(.5, .5)
    context.set_source_rgb(1, .5, 0)
    for shape in shapes:
        end = None
        for points in np.asarray(shape.points).reshape(-1, 4, 3):
            if end is None or not np.allclose(end, points[0]):
                context.move_to(*points[0][:2])
            context.curve_to(*points[1][:2], *points[2][:2], *points[3][:2])
            end = points[3]
        context.fill()
    surface.flush()
    return Image.frombuffer("RGBA", (width, height), surface.get_data(), "raw", "BGRA", surface.get_stride(), 1).copy()


def pair():
    """🧩 Two same-owner opaque facets."""
    group = MANIM.VGroup()
    result = []
    for polygon in CASES["polygons"]:
        points = np.array([[x, y, 0] for x, y in polygon], float)
        shape = DRAW._shape([points], CASES["colour"], 1, width=0)
        shape.paint = ("stone", 1.0, 0, False)
        shape.semantic_role, shape.semantic_key = "shell", "part"
        shape.world_vertices, shape.world_normal = points, np.array([0., 0., 1.])
        shape._batch_groups = (group,)
        group.add(shape)
        result.append(shape)
    return group, result


class PaintBatch(unittest.TestCase):
    def test_batch_preserves_cairo_pixels_and_semantic_group(self):
        """🎨 Consecutive opaque geometry retains exact raster coverage and one recolour target."""
        group, shapes = pair()
        before = render(shapes)
        self.assertTrue(DRAW._batch_pair(*shapes))
        self.assertEqual(before.tobytes(), render([shapes[0]]).tobytes())
        self.assertEqual(group.submobjects, [shapes[0]])
        reference_group, reference_shapes = pair()
        DRAW._recolour(reference_group, "#0077ff", False)
        DRAW._recolour(group, "#0077ff", False)
        self.assertTrue(np.array_equal(shapes[0].fill_rgbas, reference_shapes[0].fill_rgbas))
        self.assertTrue(np.array_equal(shapes[0].fill_rgbas, reference_shapes[1].fill_rgbas))
        self.assertEqual(len(group.submobjects), 1)
        self.assertEqual(len(shapes[0].world_facets), 2)

    def test_different_ownership_or_transparency_stays_separate(self):
        """🚧 A painter boundary or translucent fill must remain separate."""
        group, shapes = pair()
        shapes[1]._batch_groups = (MANIM.VGroup(),)
        self.assertFalse(DRAW._batch_pair(*shapes))
        shapes[1]._batch_groups = (group,)
        shapes[1].set_fill(opacity=.5)
        self.assertFalse(DRAW._batch_pair(*shapes))


    def test_ordered_paint_retains_selection_and_barriers(self):
        """🧭 Painter order merges only adjacent facets with identical selection owners."""
        group, shapes = pair()
        group.remove(*shapes)
        def item(shape):
            def draw(view):
                group.add(shape)
                return shape
            return DRAW.Item(0, np.zeros(3), shape.world_vertices, draw)
        before = render(shapes)
        output = DRAW._paint_ordered([item(shape) for shape in shapes], None)
        self.assertEqual(len(output), 1)
        self.assertEqual(group.submobjects, output)
        self.assertEqual(before.tobytes(), render(output).tobytes())
        self.assertFalse(hasattr(output[0], "_batch_groups"))

    def test_touching_or_oppositely_wound_faces_stay_separate(self):
        """🕳️ Intersecting bounds preserve antialiasing and winding behavior."""
        group, shapes = pair()
        shapes[1].shift(np.array([-1., 0., 0.]))
        shapes[1].points = shapes[1].points[::-1].copy()
        self.assertFalse(DRAW._batch_pair(*shapes))

if __name__ == "__main__":
    unittest.main()
