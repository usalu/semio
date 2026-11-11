"""🖼️ Check immutable painter-order snapshots against Cairo and Pillow."""
import importlib
import json
import unittest
from pathlib import Path
import numpy as np
from PIL import Image
from manim import Camera, FadeIn, Square, VGroup

snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
fixture = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))

def camera(opacity):
    return Camera(pixel_width=fixture["width"], pixel_height=fixture["height"], frame_width=fixture["frame_width"], frame_height=fixture["frame_height"], background_color=fixture["background"], background_opacity=opacity)

class SnapshotTest(unittest.TestCase):
    def test_pixels_and_semantics(self):
        shapes = [Square(side_length=row["size"], stroke_width=0, fill_opacity=1, fill_color=row["color"]).shift([row["x"], row["y"], 0]) for row in fixture["facets"]]
        drawing = VGroup(*shapes)
        original = [shape.points.copy() for shape in shapes]
        reference = camera(1)
        reference.capture_mobjects([drawing])
        expected = np.asarray(reference.get_image(), dtype=int).copy(); image = snapshot.capture(drawing, reference)
        self.assertTrue(all(np.array_equal(a, s.points) for a, s in zip(original, shapes)))
        self.assertEqual(tuple(image.pixel_array.shape), (fixture["height"], fixture["width"], 4))
        self.assertEqual(int(image.pixel_array[0, 0, 3]), 0)
        self.assertEqual(int(image.pixel_array[fixture["height"] // 2, fixture["width"] // 2, 3]), 255)
        self.assertAlmostEqual(image.width, reference.frame_width)
        self.assertAlmostEqual(image.height, reference.frame_height)
        actual = reference; actual.reset()
        actual.capture_mobjects([image])
        delta = np.abs(np.asarray(actual.get_image(), dtype=int) - expected)
        self.assertLessEqual(int(delta.max()), 2)
        background = Image.new("RGBA", (fixture["width"], fixture["height"]), fixture["background"])
        oracle = Image.alpha_composite(background, Image.fromarray(image.pixel_array))
        self.assertLessEqual(int(np.abs(np.asarray(oracle, dtype=int) - expected).max()), 2)
        saved = image.pixel_array.copy()
        shapes[0].set_fill("#ffffff")
        self.assertFalse(np.array_equal(np.asarray(actual.get_image()), np.asarray(camera(1).get_image())))
        self.assertTrue(np.array_equal(image.pixel_array, saved))

    def test_group_fade_and_fixed_alignment(self):
        camera_ = camera(1)
        image = snapshot.capture(Square(fill_opacity=1), camera_)
        animation = FadeIn(image)
        animation.begin()
        animation.interpolate(.5)
        self.assertEqual(int(image.pixel_array[:, :, 3].max()), 127)
        camera_.capture_mobjects([image])
        image.shift([1, 0, 0])
        with self.assertRaisesRegex(ValueError, "alignment"):
            camera_.capture_mobjects([image])

if __name__ == "__main__":
    unittest.main()
