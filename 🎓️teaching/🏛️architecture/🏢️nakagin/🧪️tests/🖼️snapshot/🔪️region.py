"""🔪 Crop-first snapshot and sprite captures equal the full-frame render, pixel for pixel, at tiny resolution."""
import importlib
import json
import unittest
from pathlib import Path

import numpy as np
from manim import Camera, Polygon, VGroup, Circle, Line

snapshot = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️snapshot")
sprites = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️sprites")
cases = json.loads(Path(__file__).with_name("🔣️.region.json").read_text(encoding="utf-8"))


def camera(case):
    return Camera(pixel_width=case["width"], pixel_height=case["height"], frame_width=case["frameWidth"], frame_height=case["frameHeight"],
                  frame_center=np.array(case["center"], float), background_opacity=0)


def drawing(case):
    rng = np.random.default_rng(case["seed"])
    members = []
    for _ in range(case["shapes"]):
        centre = rng.uniform(-case["spread"], case["spread"], 2)
        kind = int(rng.integers(0, 3))
        stroke = float(rng.uniform(0, case["maxStroke"]))
        if kind == 0:
            shape = Polygon(*[np.array([*(centre + rng.uniform(-1.5, 1.5, 2)), 0.0]) for _ in range(int(rng.integers(3, 6)))])
        elif kind == 1:
            shape = Circle(radius=float(rng.uniform(.1, 1.2))).shift([*centre, 0])
        else:
            shape = Line([*centre, 0], [*(centre + rng.uniform(-2, 2, 2)), 0])
        shape.set_stroke("#223344", stroke, rng.uniform(.3, 1)).set_fill("#cc7733", float(rng.uniform(0, 1)) if kind != 2 else 0)
        members.append(shape)
    return VGroup(*members)


def oracle(shape, cam):
    target = Camera(frame_center=np.array(cam.frame_center, copy=True), pixel_width=cam.pixel_width, pixel_height=cam.pixel_height, frame_width=cam.frame_width,
                    frame_height=cam.frame_height, background_opacity=0)
    target.capture_mobjects([shape])
    rgba = np.array(target.get_image())
    alpha = rgba[:, :, 3].astype(float)
    rgba[:, :, :3] = np.where(alpha[:, :, None] > 0, np.minimum(255, np.rint(rgba[:, :, :3].astype(float) * 255 / np.maximum(alpha[:, :, None], 1))), 0).astype(np.uint8)
    return rgba


class RegionCapture(unittest.TestCase):
    def test_snapshot_equals_full_frame_render(self):
        for case in cases["cases"]:
            with self.subTest(case["name"]):
                cam, shape = camera(case), drawing(case)
                np.testing.assert_array_equal(snapshot.capture(shape, cam).pixel_array, oracle(shape, cam))

    def test_sprite_equals_the_cropped_full_frame_render(self):
        for case in cases["cases"]:
            with self.subTest(case["name"]):
                cam, shape = camera(case), drawing(case)
                full = oracle(shape, cam)
                rows, cols = np.nonzero(full[:, :, 3])
                sprite = sprites.capture(shape, cam)
                if not len(rows):
                    continue
                expected = full[rows.min():rows.max() + 1, cols.min():cols.max() + 1]
                np.testing.assert_array_equal(sprite.pixel_array, expected)

    def test_empty_and_offscreen_drawings_are_transparent(self):
        cam = camera(cases["cases"][0])
        self.assertEqual(int(snapshot.capture(VGroup(), cam).pixel_array.max()), 0)
        far = Circle(radius=.5).shift([500, 0, 0])
        self.assertEqual(int(snapshot.capture(far, cam).pixel_array.max()), 0)

    def test_the_rendered_region_is_smaller_than_the_frame(self):
        case = cases["cases"][0]
        cam = camera(case)
        small = Circle(radius=.4).set_stroke(width=4)
        x0, y0, x1, y1 = snapshot.region(small, cam)
        self.assertLess((x1 - x0) * (y1 - y0), case["width"] * case["height"] / 8)


if __name__ == "__main__":
    unittest.main()
