"""🧪 Solid architectural material against independent analytic and raster oracles."""

from __future__ import annotations

import importlib
import json
import unittest
from pathlib import Path

import numpy as np

VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️sections.json").read_text(encoding="utf-8"))


def rectangle(left, right, bottom, top):
    return [(left, bottom), (right, bottom), (right, top), (left, top)]


def polygon_mask(vertices, x, y):
    mask = np.zeros(np.broadcast_shapes(x.shape, y.shape), dtype=bool)
    for a, b in zip(vertices, vertices[1:] + vertices[:1], strict=True):
        if a[1] != b[1]:
            crossing = ((a[1] > y) != (b[1] > y)) & (x < (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1]) + a[0])
            mask ^= crossing
    return mask


def house_polygons(scale):
    case = CASES["house"]
    w, h, wall = (case[key] * scale for key in ("width", "height", "wall"))
    left, right, bottom, top = -w / 2, w / 2, -h / 2, h / 2
    eave, rise, depth = (case[key] * scale for key in ("eave", "roof_rise", "roof_depth"))
    outer = top + rise * eave / (w / 2 + eave) - depth
    inner = outer + rise * wall / (w / 2 + eave)
    half_window = case["window_height"] * scale / 2
    openings = [(bottom + fraction * h - half_window, bottom + fraction * h + half_window) for fraction in (0.25, 0.75)]
    spans = [(bottom, openings[0][0]), (openings[0][1], openings[1][0])]
    return [
        rectangle(left, right, bottom - case["floor"] * scale, bottom),
        rectangle(left + wall, right - wall, -case["deck"] * scale / 2, case["deck"] * scale / 2),
        *(rectangle(left, left + wall, a, b) for a, b in spans),
        [(left, openings[-1][1]), (left + wall, openings[-1][1]), (left + wall, inner), (left, outer)],
        [(right - wall, bottom), (right, bottom), (right, outer), (right - wall, inner)],
        [(left - eave, top), (0, top + rise), (right + eave, top),
         (right + eave, top - depth), (0, top + rise - depth), (left - eave, top - depth)],
    ]


def room_polygons(side):
    case = CASES["room"]
    w, h, wall, slab = (case[key] for key in ("width", "height", "wall", "slab"))
    left, right, bottom, top = -w / 2, w / 2, -h / 2, h / 2
    lo, hi = (bottom + fraction * h for fraction in case["window"])
    polygons = [rectangle(left - wall, right + wall, bottom - slab, bottom),
                rectangle(left - wall, right + wall, top, top + slab)]
    for name, a, b in (("left", left - wall, left), ("right", right, right + wall)):
        spans = [(bottom, lo), (hi, top)] if side in {name, "both"} else [(bottom, top)]
        polygons.extend(rectangle(a, b, c, d) for c, d in spans)
    return polygons


class SectionMaterialTest(unittest.TestCase):
    def assert_material(self, group):
        from manim import ManimColor

        case = CASES["material"]
        for part in group.get_family():
            if len(getattr(part, "points", [])):
                for contour in part.get_subpaths():
                    np.testing.assert_allclose(contour[0], contour[-1])
                self.assertEqual(part.get_fill_color(), ManimColor(VISUALS.color(case["color"])))
                self.assertEqual(part.get_fill_opacity(), case["fill_opacity"])
                self.assertEqual(part.get_stroke_width(), case["stroke_width"])
                self.assertEqual(part.get_stroke_opacity(), case["stroke_opacity"])

    def assert_uniform_raster(self, material, polygons):
        from manim import Camera, Rectangle
        from scipy.ndimage import binary_erosion, label

        case = CASES["raster"]
        pixels, size = case["pixels"], case["frame_size"]
        camera = Camera(pixel_width=pixels, pixel_height=pixels, frame_height=size, frame_width=size,
                        frame_center=np.array([0, 0.5, 0]), background_color=VISUALS.P_DEEP_DARK)
        camera.capture_mobjects([material])
        image = np.asarray(camera.get_image())[:, :, :3].astype(int)
        coordinates = np.arange(pixels) + 0.5
        x, y = (coordinates[None, :] / pixels - 0.5) * size, 0.5 + (0.5 - coordinates[:, None] / pixels) * size
        expected = np.zeros((pixels, pixels), dtype=bool)
        for polygon in polygons:
            expected |= polygon_mask(polygon, x, y)
        interior = binary_erosion(expected, iterations=case["interior_pixels"])
        oracle = Rectangle(width=1, height=1, stroke_width=0, fill_color=VISUALS.P_WHITE, fill_opacity=1).move_to([0, 0.5, 0])
        camera.reset()
        camera.capture_mobjects([oracle])
        color = np.asarray(camera.get_image())[pixels // 2, pixels // 2, :3].astype(int)
        errors = np.max(np.abs(image - color), axis=2)
        worst = int(errors[interior].max())
        self.assertLessEqual(worst, case["maximum_channel_error"],
                             f"Nonuniform section interior: maximum channel error {worst}, "
                             f"bad pixels {np.count_nonzero(interior & (errors > case['maximum_channel_error']))}")
        actual = np.max(np.abs(image - color), axis=2) <= case["maximum_channel_error"]
        self.assertEqual(label(actual)[1], label(expected)[1])

    def test_portable_contract_requires_one_opaque_unstroked_material(self):
        contract = json.loads(Path(VISUALS.__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
        self.assertEqual(contract["section_material"], CASES["material"])

    def test_public_style_replaces_translucency_and_outlines(self):
        from manim import Rectangle, VGroup

        material = VGroup(Rectangle(fill_color=VISUALS.P_RED, fill_opacity=0.2, stroke_width=4),
                          Rectangle(fill_opacity=0.7, stroke_width=1.3).shift([0.8, 0, 0]))
        self.assertIs(VISUALS.section_material(material), material)
        self.assert_material(material)

    def test_house_floor_walls_roof_and_reveals_are_one_material(self):
        from manim import VGroup

        house = VISUALS.house_section()
        self.assert_material(VGroup(house["floor"], house["walls"], house["roof"], *(window["reveal"] for window in house["windows"])))

    def test_room_shell_and_window_reveals_are_one_material(self):
        room = VISUALS.room_section([0, 0, 0])
        self.assert_material(room["shell"])
        window = VISUALS.window_glyph(room["glass_x"], room["win_lo"], room["win_hi"])
        self.assert_material(window["reveal"])
        self.assertGreater(window["sash"][0].get_stroke_width(), 0)
        self.assertLess(window["sash"][0].get_fill_opacity(), 1)

    def test_house_joints_and_reveal_overlaps_have_uniform_raster(self):
        from manim import VGroup

        for scale in CASES["house"]["scales"]:
            with self.subTest(scale=scale):
                house = VISUALS.house_section(scale=scale)
                self.assert_uniform_raster(house["material"], house_polygons(scale))

    def test_room_corners_and_reveal_overlaps_have_uniform_raster(self):
        from manim import VGroup

        case = CASES["room"]
        for side in case["sides"]:
            with self.subTest(side=side):
                room = VISUALS.room_section([0, 0, 0], w=case["width"], h=case["height"],
                                           wall=case["wall"], slab=case["slab"], window=case["window"], window_side=side)
                reveals = []
                for name, x, inward in (("left", room["glass_x"], 1), ("right", room["right_glass_x"], -1)):
                    if side in {name, "both"}:
                        reveals.append(VISUALS.window_glyph(x, room["win_lo"], room["win_hi"],
                                                          wall=case["wall"], inward=inward)["reveal"])
                self.assert_uniform_raster(VGroup(room["shell"], *reveals), room_polygons(side))

    def test_assembly_follows_partial_fades_moves_removal_and_replacement(self):
        from manim import Camera, FadeOut, Rectangle

        case = CASES["assembly"]

        def material_box(box):
            left, right, bottom, top = box
            return VISUALS.section_material(Rectangle(width=right - left, height=top - bottom)).move_to(
                [(left + right) / 2, (bottom + top) / 2, 0],
            )

        left, right = (material_box(box) for box in case["initial"])
        original = rectangle(*case["initial"][0])
        material = VISUALS.section_assembly(left, right)
        self.assertIs(material[0], left)
        self.assertIs(material[1], right)
        self.assert_uniform_raster(material, [rectangle(*box) for box in case["initial"]])
        animation = FadeOut(right)
        animation.begin()
        animation.interpolate(case["partial_fraction"])
        material.update(0)
        camera = Camera(pixel_width=300, pixel_height=300, frame_width=3, frame_height=3,
                        frame_center=np.array([0, 0.5, 0]), background_color=VISUALS.P_DEEP_DARK)
        camera.capture_mobjects([material])
        image = np.asarray(camera.get_image())
        camera.reset()
        camera.capture_mobjects([right])
        oracle = np.asarray(camera.get_image())
        np.testing.assert_array_equal(image[150, 200], oracle[150, 200])
        animation.finish()
        material.update(0)
        self.assert_uniform_raster(material, [original])
        right.set_opacity(1).shift(case["move"])
        material.update(0)
        shifted = [(x + case["move"][0], y + case["move"][1]) for x, y in rectangle(*case["initial"][1])]
        self.assert_uniform_raster(material, [original, shifted])
        right.become(material_box(case["replacement"]))
        material.update(0)
        self.assert_uniform_raster(material, [original, rectangle(*case["replacement"])])
        material.remove(right)
        material.update(0)
        self.assert_uniform_raster(material, [original])

    def test_assembly_normalizes_winding_and_preserves_whole_group_animations(self):
        from manim import Create, FadeIn, Rectangle

        case = CASES["assembly"]
        parts = []
        for left, right, bottom, top in case["overlap"]:
            parts.append(VISUALS.section_material(Rectangle(width=right - left, height=top - bottom)).move_to(
                [(left + right) / 2, (bottom + top) / 2, 0],
            ))
        parts[-1].reverse_direction()
        material = VISUALS.section_assembly(*parts)
        polygons = [rectangle(*box) for box in case["overlap"]]
        for animation_type in (Create, FadeIn):
            animation = animation_type(material)
            animation.begin()
            animation.interpolate(case["partial_fraction"])
            animation.finish()
            material.update(0)
            self.assert_uniform_raster(material, polygons)
        transform = case["transform"]
        center = material.get_center()
        copied = material.copy().scale(transform["scale"]).shift(transform["shift"])
        copied.update(0)
        transformed = [[tuple((np.array([x, y, 0]) - center)[:2] * transform["scale"]
                             + center[:2] + transform["shift"][:2]) for x, y in polygon] for polygon in polygons]
        self.assert_uniform_raster(copied, transformed)

    def test_room_corner_raster_stays_uniform_after_nonpixel_transform(self):
        case = CASES["room"]
        room = VISUALS.room_section([0, 0, 0], w=case["width"], h=case["height"],
                                   wall=case["wall"], slab=case["slab"], window=case["window"], window_side="both")
        transform = CASES["assembly"]["transform"]
        scale = transform["scale"]
        dx, dy = transform["shift"][:2]
        room["shell"].scale(scale, about_point=np.zeros(3)).shift([dx, dy, 0])
        room["shell"].update(0)
        polygons = [[(x * scale + dx, y * scale + dy) for x, y in polygon] for polygon in room_polygons("both")]
        self.assert_uniform_raster(room["shell"], polygons)


if __name__ == "__main__":
    unittest.main()
