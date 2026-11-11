"""🧸 Verify captured capsule sprites against Cairo and animation anchors."""
import importlib
import json
import unittest
from pathlib import Path
import numpy as np
from PIL import Image
from manim import AnimationGroup, Camera, FadeIn, FadeOut, Scene, Square, VGroup, tempconfig

sprites = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️sprites")
case = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))

def camera():
    return Camera(pixel_width=case["pixel_width"], pixel_height=case["pixel_height"], frame_width=case["frame_width"],
                  frame_height=case["frame_height"], background_color=case["background"], background_opacity=1)

class SpriteTest(unittest.TestCase):
    def test_capture_pixels_bounds_and_source(self):
        shapes = [Square(side_length=row["size"], stroke_width=0, fill_color=row["color"], fill_opacity=1).shift([row["x"], row["y"], 0]) for row in case["facets"]]
        group = VGroup(*shapes)
        original = [shape.points.copy() for shape in shapes]
        view = camera()
        view.capture_mobjects([group])
        expected = np.array(view.get_image())
        sprite = sprites.capture(group, view)
        view.reset()
        view.capture_mobjects([sprite])
        actual = np.array(view.get_image())
        self.assertLessEqual(int(np.abs(expected.astype(int) - actual.astype(int)).max()), 2)
        self.assertTrue(all(np.array_equal(a, b.points) for a, b in zip(original, shapes)))
        self.assertLess(sprite.width, case["frame_width"])
        alpha = sprite.pixel_array[:, :, 3]
        self.assertGreater(int(alpha.max()), 0)
        oracle = Image.alpha_composite(Image.new("RGBA", (case["pixel_width"], case["pixel_height"]), case["background"]), Image.fromarray(np.array(sprites.full_frame(sprite, view))))
        self.assertLessEqual(int(np.abs(expected.astype(int) - np.asarray(oracle, dtype=int)).max()), 2)

    def test_recolored_sprite_leaves_original_unchanged(self):
        view = camera()
        shape = Square(side_length=1, fill_color="#2266aa", fill_opacity=1, stroke_width=0).shift([1, 0, 0])
        original = shape.fill_rgbas.copy()
        yellow = shape.copy().set_color("#ffb000")
        sprite = sprites.capture(yellow, view)
        self.assertTrue(np.array_equal(shape.fill_rgbas, original))
        pixel = sprite.pixel_array[sprite.pixel_array.shape[0] // 2, sprite.pixel_array.shape[1] // 2]
        self.assertTrue(np.allclose(pixel[:3], [255, 176, 0], atol=1))
        self.assertEqual(int(pixel[3]), 255)

    def test_group_flights_keep_original_geometry(self):
        view = camera()
        shapes = [Square(side_length=.6, fill_color=row["color"], fill_opacity=1, stroke_width=0).shift([row["x"], row["y"], 0]) for row in case["facets"]]
        saved = [shape.points.copy() for shape in shapes]
        flying = [sprites.capture(shape, view) for shape in shapes]
        target = Square(side_length=.4).shift([2, 1, 0])
        animation = AnimationGroup(*[FadeOut(sprite, target_position=target, scale=.15) for sprite in flying], lag_ratio=.01, run_time=2)
        animation.begin()
        animation.interpolate(.5)
        view.capture_mobjects(flying)
        self.assertTrue(np.any(np.asarray(view.get_image())[:, :, :3] != np.asarray(view.get_image())[0, 0, :3]))
        animation.interpolate(1)
        for sprite in flying:
            self.assertTrue(np.allclose(sprite.get_center(), target.get_center()))
        self.assertTrue(all(np.array_equal(points, shape.points) for points, shape in zip(saved, shapes)))

    def test_scene_gather_and_split_cleanup(self):
        with tempconfig({"dry_run": True, "pixel_width": 160, "pixel_height": 90, "frame_rate": 4}):
            scene = Scene()
            shapes = [Square(side_length=.5, fill_opacity=1, stroke_width=0).shift([i - 1, 0, 0]) for i in range(3)]
            original = [shape.points.copy() for shape in shapes]
            crowd = [sprites.capture(shape, scene.camera) for shape in shapes]
            target = Square(side_length=.4).shift([2, 1, 0])
            scene.add(*crowd)
            scene.play(AnimationGroup(*[FadeOut(sprite, target_position=target, scale=.15) for sprite in crowd],
                                      FadeIn(target, scale=.85), lag_ratio=.01, run_time=.5))
            self.assertTrue(all(sprite not in scene.mobjects for sprite in crowd))
            self.assertIn(target, scene.mobjects)
            arriving = [sprites.capture(shape, scene.camera) for shape in shapes]
            scene.play(AnimationGroup(*[FadeIn(sprite, target_position=target, scale=.15) for sprite in arriving],
                                      FadeOut(target), lag_ratio=.01, run_time=.5))
            self.assertTrue(all(sprite in scene.mobjects for sprite in arriving))
            self.assertNotIn(target, scene.mobjects)
            self.assertTrue(all(np.array_equal(points, shape.points) for points, shape in zip(original, shapes)))

    def test_fade_motion_anchors(self):
        view = camera()
        shape = Square(side_length=1, fill_opacity=1, stroke_width=0).shift([2, 1, 0])
        sprite = sprites.capture(shape, view)
        original = sprite.get_center().copy()
        destination = np.array([-2., -1., 0.])
        fade = FadeOut(sprite, target_position=destination, scale=.15)
        fade.begin()
        fade.interpolate(1)
        self.assertTrue(np.allclose(sprite.get_center(), destination))
        self.assertAlmostEqual(sprite.width, shape.width * .15, delta=.02)
        view.capture_mobjects([sprite])
        arrival = sprites.capture(shape, view)
        fadein = FadeIn(arrival, target_position=destination, scale=.15)
        fadein.begin()
        fadein.interpolate(1)
        self.assertTrue(np.allclose(arrival.get_center(), original))
        view.reset()
        view.capture_mobjects([arrival])

if __name__ == "__main__":
    unittest.main()
