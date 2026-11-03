"""🧪 Staffage bridge acceptance: cache keys, SVG-to-world placement, motion stepping and colour rules without needing the exporter."""

import _thread
import contextlib
import hashlib
import importlib
import io
import json
import os
import re
import subprocess
import sys
import tempfile
import threading
import time
import unittest
from pathlib import Path
from unittest.mock import patch

import numpy as np
from manim import VGroup

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


def synthetic(root: Path, *, view: str = "dark-line") -> Path:
    """🧱 Write a rendered cast folder the way the exporter publishes it."""
    folder = root / "case"
    folder.mkdir(parents=True)
    names = [f"{frame:04d}.svg" for frame in range(len(CASE["frames"]))]
    (folder / "🔣️.json").write_text(json.dumps({**CASE["index"], "view": view, "frames": names}), encoding="utf-8")
    for name, body in zip(names, CASE["frames"]):
        head = f'<svg xmlns="http://www.w3.org/2000/svg" width="{CASE["width"]}mm" height="{CASE["height"]}mm" viewBox="0 0 {CASE["width"]} {CASE["height"]}">'
        (folder / name).write_text(head + body + "</svg>", encoding="utf-8")
    return folder


def stamped(index: dict, cast: str) -> dict:
    """🔖 Add the provenance the exporter writes for a manifest cast to a hand-made index."""
    provenance = S.expected_provenance(cast)
    provenance["asset"]["category"] = "humans"
    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}
    return {**index, "cast": cast, "provenance": {**provenance, "clock": clock}}


def pieces(group):
    """🔍 Return the bounding extents of the rectangle and the line of a placed frame."""
    members = list(group.family_members_with_points())
    boxes = [(m.get_all_points().min(axis=0), m.get_all_points().max(axis=0)) for m in members]
    return members, boxes


class Placement(unittest.TestCase):
    def test_world_metres_times_unit_land_on_the_anchor(self):
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            members, boxes = pieces(S._place(folder, 0, CASE["placement"]["unit"], CASE["placement"]["anchor"], None, 1.0))
            rectangle = next(box for box in boxes if box[1][1] - box[0][1] > 1)
            line = next(box for box in boxes if box[1][1] - box[0][1] < 1e-6)
            expected = CASE["expected"]
            np.testing.assert_allclose([rectangle[0][0], rectangle[1][0]], expected["rectangle"]["x"], atol=1e-6)
            np.testing.assert_allclose([rectangle[0][1], rectangle[1][1]], expected["rectangle"]["y"], atol=1e-6)
            np.testing.assert_allclose([line[0][0], line[1][0]], expected["lineX"], atol=1e-6)
            self.assertAlmostEqual(line[0][1], expected["lineY"][0], places=6)
            self.assertTrue(all(abs(float(m.get_all_points()[:, 2].max()) - CASE["placement"]["anchor"][2]) < 1e-9 for m in members))

    def test_matches_an_independent_svg_parser(self):
        import svgelements

        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            index = json.loads((folder / "🔣️.json").read_text(encoding="utf-8"))
            unit, anchor = CASE["placement"]["unit"], np.array(CASE["placement"]["anchor"], dtype=float)
            for frame in range(len(index["frames"])):
                parsed = svgelements.SVG.parse(str(folder / index["frames"][frame]))
                shapes = [e for e in parsed.elements() if isinstance(e, svgelements.Shape) and e.bbox()]
                _, boxes = pieces(S._place(folder, frame, unit, anchor, None, 1.0))
                expect = []
                for shape in shapes:
                    pixels = float(parsed.width) / CASE["width"]
                    x0, y0, x1, y1 = (value / pixels for value in shape.bbox())
                    scale = unit / index["mmPerMetre"]
                    ox, oy = index["originMm"]
                    expect.append(sorted([anchor[0] + (x0 - ox) * scale, anchor[1] + (oy - y1) * scale, anchor[0] + (x1 - ox) * scale, anchor[1] + (oy - y0) * scale]))
                got = sorted(sorted([b[0][0], b[0][1], b[1][0], b[1][1]]) for b in boxes)
                np.testing.assert_allclose(got, sorted(expect), atol=1e-6)

    def test_stroke_width_follows_the_physical_scale(self):
        with tempfile.TemporaryDirectory() as root:
            members, _ = pieces(S._place(synthetic(Path(root)), 0, CASE["placement"]["unit"], [0, 0, 0], None, 1.0))
            for member in members:
                self.assertAlmostEqual(float(np.max(member.get_stroke_width())), CASE["expected"]["strokeWidth"], places=6)
            thin, _ = pieces(S._place(synthetic(Path(root) / "thin"), 0, 0.1, [0, 0, 0], None, 1.0))
            self.assertTrue(all(float(np.max(m.get_stroke_width())) >= S._MIN_STROKE for m in thin))

    def test_opacity_scales_strokes(self):
        with tempfile.TemporaryDirectory() as root:
            members, _ = pieces(S._place(synthetic(Path(root)), 0, 1.0, [0, 0, 0], None, 0.5))
            self.assertTrue(all(abs(float(m.get_stroke_opacity()) - 0.5) < 1e-9 for m in members))


class Parts(unittest.TestCase):
    def test_shape_polygons_follow_the_group_transform_in_world_metres(self):
        import svgelements

        body = ('<g id="staffage-cast" transform="matrix(-1 0 0 1 100 0)"><g id="shape-slat-0" data-bone="slat-0">'
                '<path d="M60 40L80 50L80 52L60 42Z" fill="#000000" fill-rule="evenodd"/></g>'
                '<g id="shape-box" data-bone="box"><path d="M40 10L60 10L60 20L40 20Z" fill="#000000"/></g></g>')
        with tempfile.TemporaryDirectory() as root:
            folder = Path(root) / "case"
            folder.mkdir()
            index = {**CASE["index"], "frames": ["0000.svg"]}
            (folder / "🔣️.json").write_text(json.dumps(index), encoding="utf-8")
            head = '<svg xmlns="http://www.w3.org/2000/svg" width="100mm" height="100mm" viewBox="0 0 100 100">'
            (folder / "0000.svg").write_text(head + body + "</svg>", encoding="utf-8")
            parts = S._parts(folder, "slat")
            self.assertEqual(list(parts), ["slat-0"])
            parsed = svgelements.SVG.parse(str(folder / "0000.svg"))
            pixels = float(parsed.width) / 100
            slat = next(e for e in parsed.elements() if isinstance(e, svgelements.Shape) and abs((e.bbox()[3] - e.bbox()[1]) / pixels - 12) < 1e-6)
            ox, oy = index["originMm"]
            scale = index["mmPerMetre"]
            x0, y0, x1, y1 = (value / pixels for value in slat.bbox())
            polygon = parts["slat-0"]
            np.testing.assert_allclose([polygon[:, 0].min(), polygon[:, 0].max(), polygon[:, 1].min(), polygon[:, 1].max()],
                                       [(x0 - ox) / scale, (x1 - ox) / scale, (oy - y1) / scale, (oy - y0) / scale], atol=1e-9)
            self.assertAlmostEqual(float(polygon[:, 0].min()), (20 - ox) / scale, places=9)


class Window(unittest.TestCase):
    def test_only_elements_fully_inside_the_window_remain(self):
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            unit, anchor = CASE["placement"]["unit"], CASE["placement"]["anchor"]
            placed = S._place(folder, 0, unit, anchor, None, 1.0)
            everything = S._crop(placed, unit, anchor, (-10.0, -10.0, 10.0, 10.0))
            self.assertEqual(len(list(everything.family_members_with_points())), 2)
            tall = S._crop(placed, unit, anchor, (-10.0, 0.0, 10.0, 3.0))
            members = list(tall.family_members_with_points())
            self.assertEqual(len(members), 1)
            self.assertAlmostEqual((float(members[0].get_all_points()[:, 1].max()) - anchor[1]) / unit, 2.5, places=6)
            self.assertEqual(len(list(S._crop(placed, unit, anchor, (-10.0, 5.0, 10.0, 6.0)).family_members_with_points())), 0)


class Colour(unittest.TestCase):
    def test_line_views_are_recoloured(self):
        with tempfile.TemporaryDirectory() as root:
            members, _ = pieces(S._place(synthetic(Path(root)), 0, 1.0, [0, 0, 0], "#00FF00", 1.0))
            self.assertTrue(all(str(m.get_stroke_color()).upper() == "#00FF00" for m in members))

    def test_other_views_keep_their_colours(self):
        for view in ("dark-solid", "light-solid", "thermal", "forces", "moments"):
            with tempfile.TemporaryDirectory() as root:
                members, _ = pieces(S._place(synthetic(Path(root), view=view), 0, 1.0, [0, 0, 0], "#00FF00", 1.0))
                self.assertEqual({str(m.get_stroke_color()).upper() for m in members}, {"#000000", "#FF0000"}, view)

    def test_no_colour_keeps_the_drawing(self):
        with tempfile.TemporaryDirectory() as root:
            members, _ = pieces(S._place(synthetic(Path(root)), 0, 1.0, [0, 0, 0], None, 1.0))
            self.assertEqual({str(m.get_stroke_color()).upper() for m in members}, {"#000000", "#FF0000"})


class Motion(unittest.TestCase):
    def build(self, root, rate=1.0):
        folder = synthetic(Path(root))
        S._INDEXES.pop(folder, None)
        return S._Motion(folder, CASE["placement"]["unit"], CASE["placement"]["anchor"], rate, None)

    def line_y(self, motion):
        _, boxes = pieces(VGroup(*motion.slots))
        return min(box[0][1] for box in boxes if box[1][1] - box[0][1] < 1e-6)

    def test_frames_step_loop_and_scale_with_rate(self):
        with tempfile.TemporaryDirectory() as root:
            motion = self.build(root)
            S.SCENE_CLOCK.now = 0.0
            motion.sync()
            for step in CASE["steps"]:
                motion.rate = step["rate"]
                S.SCENE_CLOCK.now += step["advance"]
                motion.sync()
                self.assertEqual(motion.frame, step["frame"], step)
                self.assertAlmostEqual(self.line_y(motion), CASE["expected"]["lineY"][step["frame"]], places=6)

    def test_clips_without_loop_hold_the_last_frame(self):
        with tempfile.TemporaryDirectory() as root:
            motion = self.build(root)
            motion.loops = False
            motion.seek(10.0)
            self.assertEqual(motion.frame, len(motion.frames) - 1)

    def test_group_transforms_carry_over_to_later_frames(self):
        with tempfile.TemporaryDirectory() as root:
            motion = self.build(root)
            motion.shift(np.array([3.0, -1.0, 0.0]))
            motion.seek(0.15)
            self.assertAlmostEqual(self.line_y(motion), CASE["expected"]["lineY"][1] - 1.0, places=6)
            _, boxes = pieces(VGroup(*motion.slots))
            self.assertAlmostEqual(min(b[0][0] for b in boxes), CASE["expected"]["rectangle"]["x"][0] + 3.0, places=6)

    def test_fading_the_group_carries_over_to_later_frames(self):
        with tempfile.TemporaryDirectory() as root:
            motion = self.build(root)
            motion.set_opacity(0.25)
            motion.seek(0.15)
            members = [slot for slot in motion.slots if slot.has_points()]
            self.assertTrue(all(abs(float(m.get_stroke_opacity()) - 0.25) < 1e-9 for m in members))

    def test_set_opacity_never_gives_an_unfilled_path_a_fill(self):
        fade = CASE["fade"]
        with tempfile.TemporaryDirectory() as root:
            motion = self.build(root)
            placed = S._place(synthetic(Path(root) / "placed"), 0, 1.0, np.zeros(3), None, 0.5)
            for group, members in ((motion, motion.slots), (placed, placed.submobjects)):
                group.set_opacity(fade["opacity"])
                self.assertTrue(members)
                for member in members:
                    self.assertEqual(member.get_fill_opacity(), fade["unfilledFillOpacity"])
                    self.assertAlmostEqual(float(member.get_stroke_opacity()), fade["strokeOpacity"])
            motion.seek(0.15)
            self.assertTrue(all(slot.get_fill_opacity() == fade["unfilledFillOpacity"] for slot in motion.slots if slot.has_points()))

    def test_the_updater_advances_with_scene_time(self):
        with tempfile.TemporaryDirectory() as root:
            motion = self.build(root)
            S.SCENE_CLOCK.now = 5.0
            motion.update(0.0)
            S.SCENE_CLOCK.now = 5.12
            motion.update(0.0)
            self.assertEqual(motion.frame, 1)


class Playback(unittest.TestCase):
    VECTORS = json.loads(Path(__file__).parents[5].joinpath("🧍️staffage", "🧫️fixtures", "🎬️playback-clock", "🔣️.json").read_text(encoding="utf-8"))

    def test_the_shared_vectors_give_the_authored_frames(self):
        for row in self.VECTORS["playback"]:
            for sample in row["samples"]:
                frame = S.playback_frame(sample["t"], row["fps"], row["count"], row["durationS"], row["loop"])
                self.assertEqual(frame, sample["frame"], (row["name"], sample))

    def test_an_accumulating_clock_and_a_seek_agree_on_every_frame(self):
        for row in self.VECTORS["playback"]:
            clock = 0.0
            for i in range(400):
                stepped = S.playback_frame(clock, row["fps"], row["count"], row["durationS"], row["loop"])
                seeked = S.playback_frame(i / 24, row["fps"], row["count"], row["durationS"], row["loop"])
                self.assertEqual(stepped, seeked, (row["name"], i))
                clock += 1 / 24

    def test_a_motion_shows_the_exported_frame_n_at_n_over_fps_and_wraps_without_a_seam(self):
        with tempfile.TemporaryDirectory() as root:
            motion = S._Motion(synthetic(Path(root)), CASE["placement"]["unit"], CASE["placement"]["anchor"], 1.0, None)
            S.SCENE_CLOCK.now = 7.0
            motion.sync()
            for n in range(9):
                S.SCENE_CLOCK.now = 7.0 + n / motion.fps
                motion.sync()
                self.assertEqual(motion.frame, n % 3)
            S.SCENE_CLOCK.now = 7.0 + 3 / motion.fps
            motion.sync()
            self.assertEqual(motion.frame, 0)
            _, boxes = pieces(VGroup(*motion.slots))
            self.assertAlmostEqual(min(box[0][1] for box in boxes if box[1][1] - box[0][1] < 1e-6), CASE["expected"]["lineY"][0], places=6)

    def test_seeking_equals_playing_the_same_scene_time(self):
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            played = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
            seeked = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
            S.SCENE_CLOCK.now = 2.0
            played.sync()
            for i in range(1, 40):
                S.SCENE_CLOCK.now = 2.0 + i / 15
                played.sync()
                seeked.seek(i / 15)
                self.assertEqual(played.frame, seeked.frame, i)
                self.assertEqual([slot.get_points().tolist() for slot in played.slots], [slot.get_points().tolist() for slot in seeked.slots], i)

    def test_a_real_scene_keeps_the_motion_on_video_time_through_waits_and_plays(self):
        from manim import FadeIn, Scene, tempconfig

        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            seen = []

            class Probe(Scene):
                def construct(scene):
                    motion = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
                    scene.add(motion)
                    for run in (1.0, 0.33, 0.7):
                        scene.wait(run)
                        seen.append((float(scene.renderer.time), motion.elapsed))
                    scene.play(FadeIn(motion), run_time=0.41)
                    seen.append((float(scene.renderer.time), motion.elapsed))
                    scene.wait(0.5)
                    seen.append((float(scene.renderer.time), motion.elapsed))

            with tempconfig({"frame_rate": 15, "write_to_movie": False, "dry_run": True, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none"}):
                Probe().render()
            for video, cast in seen:
                self.assertAlmostEqual(cast, video, places=9, msg=seen)

    def test_a_removed_motion_keeps_its_phase_and_restart_begins_again(self):
        with tempfile.TemporaryDirectory() as root:
            motion = S._Motion(synthetic(Path(root)), 1.0, [0, 0, 0], 1.0, None)
            S.SCENE_CLOCK.now = 1.0
            motion.sync()
            S.SCENE_CLOCK.now = 1.1
            motion.sync()
            self.assertEqual(motion.frame, 1)
            S.SCENE_CLOCK.now = 10.1
            motion.sync()
            self.assertEqual(motion.frame, 1)
            motion.restart()
            S.SCENE_CLOCK.now = 10.2
            motion.sync()
            self.assertEqual(motion.frame, 1)
            self.assertAlmostEqual(motion.elapsed, 0.1, places=9)


class Parser(unittest.TestCase):
    def test_paths_match_manims_svg_import_for_lines_curves_subpaths_and_mirrors(self):
        from manim import SVGMobject

        body = ('<g id="staffage-cast" transform="matrix(-1 0 0 1 90 2)">'
                '<path d="M10 10L20 10L20 30Z" fill="#ff0000" fill-rule="evenodd" stroke="#00ff00" stroke-width="0.5" stroke-opacity="0.5"/>'
                '<path d="M1 1L5 1M2 2L6 8C7 9 8 9 9 8L9 3H4V6" fill="none" stroke="#123456" stroke-width="0.25"/>'
                '<path d="M30 30L40 30L40 40L30 40L30 30Z" fill="#abc" fill-opacity="0.25"/></g>')
        head = '<svg xmlns="http://www.w3.org/2000/svg" width="100mm" height="100mm" viewBox="0 0 100 100">'
        index = {**CASE["index"], "mmPerMetre": 10, "originMm": [12.0, 70.0]}
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "frame.svg"
            path.write_text(head + body + "</svg>", encoding="utf-8")
            reference = SVGMobject(str(path), should_center=False, height=None, width=None)
            centre = float(reference.get_center()[1])
            members = list(VGroup(*reference.submobjects).family_members_with_points())
            frame = S._parse_frame(path.read_text(encoding="utf-8"), index)
        self.assertEqual(len(members), len(frame.shapes))
        for member, (low, high), shape in zip(members, frame.spans, frame.shapes):
            expected = np.column_stack(((member.points[:, 0] - 12.0) / 10, (70.0 - 2 * centre + member.points[:, 1]) / 10))
            np.testing.assert_allclose(frame.points[low:high], expected, atol=1e-9)
            self.assertAlmostEqual(float(member.get_fill_opacity()), 0.0 if shape.fill is None else shape.fill_opacity, delta=0.005)
            if shape.stroke is not None:
                self.assertEqual(str(member.get_stroke_color()).upper(), shape.stroke)
                self.assertAlmostEqual(float(member.get_stroke_opacity()), shape.stroke_opacity, delta=0.005)
                self.assertAlmostEqual(float(np.max(member.get_stroke_width())), shape.stroke_mm, places=9)
        self.assertEqual([shape.fill for shape in frame.shapes], ["#FF0000", None, "#AABBCC"])

    def test_the_compact_frame_form_parses_like_the_long_form(self):
        head = '<svg xmlns="http://www.w3.org/2000/svg" width="100mm" height="100mm" viewBox="0 0 100 100">'
        long = ('<g id="staffage-cast" transform="translate(3 4)"><g id="staffage-fill" inkscape:groupmode="layer">'
                '<g id="shape-a" data-bone="a"><path d="M10 10L20 10L20 30L10 30Z" fill="#ff0000" fill-rule="evenodd" stroke="#001117" stroke-width="0.5" stroke-linecap="round" stroke-linejoin="round"/></g>'
                '<path id="shape-b" d="M0.5 -0.5L5 1L-0.25 8C7 9 8 9 9 8C9 7 9 6 9 5Z" fill="none" stroke="#123456" stroke-width="0.25" stroke-linecap="round" stroke-linejoin="round"/></g></g>')
        compact = ('<g id="staffage-cast" transform="translate(3 4)"><g id="staffage-fill" inkscape:groupmode="layer" fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round">'
                   '<g id="shape-a" data-bone="a"><path d="M10 10L20 10 20 30 10 30Z" fill="#ff0000" stroke="#001117" stroke-width="0.5"/></g>'
                   '<path d="M.5-.5L5 1-.25 8C7 9 8 9 9 8 9 7 9 6 9 5Z" fill="none" stroke="#123456" stroke-width="0.25"/></g></g>')
        index = {**CASE["index"], "mmPerMetre": 10, "originMm": [12.0, 70.0]}
        a, b = (S._parse_frame(head + body + "</svg>", index) for body in (long, compact))
        np.testing.assert_array_equal(a.points, b.points)
        self.assertEqual((a.spans, a.shapes), (b.spans, b.shapes))
        with tempfile.TemporaryDirectory() as root:
            parts = []
            for body in (long, compact):
                folder = Path(root) / str(len(parts))
                folder.mkdir()
                (folder / "🔣️.json").write_text(json.dumps({**index, "frames": ["0000.svg"]}), encoding="utf-8")
                (folder / "0000.svg").write_text(head + body + "</svg>", encoding="utf-8")
                parts.append(S._parts(folder, ""))
        self.assertEqual(list(parts[0]), list(parts[1]))
        np.testing.assert_array_equal(parts[0]["a"], parts[1]["a"])
        self.assertEqual(len(parts[1]["a"]), 4)


class FrameSwap(unittest.TestCase):
    def build(self, root):
        folder = synthetic(Path(root))
        S._INDEXES.pop(folder, None)
        return S._Motion(folder, CASE["placement"]["unit"], CASE["placement"]["anchor"], 1.0, None)

    def test_the_family_is_fixed_and_frames_swap_points_and_styles_in_place(self):
        with tempfile.TemporaryDirectory() as root:
            motion = self.build(root)
            members = list(motion.get_family())
            slots = list(motion.slots)
            for frame in (1, 2, 0, 2):
                motion.seek(frame / motion.fps + 1e-6)
                self.assertEqual(motion.frame, frame)
                self.assertEqual(list(motion.get_family()), members)
                self.assertEqual(list(motion.slots), slots)
            self.assertEqual(len(slots), 2)

    def test_the_parsed_frame_cache_is_bounded_and_revisits_are_exact(self):
        with tempfile.TemporaryDirectory() as root:
            saved = S._FRAME_CACHE
            S._FRAME_CACHE = 1
            try:
                motion = self.build(root)
                before = [slot.points.copy() for slot in motion.slots]
                for frame in (1, 2, 0):
                    motion.seek(frame / motion.fps + 1e-6)
                    self.assertLessEqual(len(S._FRAMES), 1)
                for slot, points in zip(motion.slots, before):
                    np.testing.assert_allclose(slot.points, points, atol=1e-12)
            finally:
                S._FRAME_CACHE = saved

    def test_repeated_frame_names_share_one_parse_and_one_slot_count(self):
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            index = json.loads((folder / "🔣️.json").read_text(encoding="utf-8"))
            index["frames"] = ["0000.svg", "0001.svg"] * 50
            (folder / "🔣️.json").write_text(json.dumps(index), encoding="utf-8")
            S._INDEXES.pop(folder, None)
            motion = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
            self.assertEqual(len(motion.frames), 100)
            for step in range(100):
                motion.seek(step / motion.fps + 1e-6)
            self.assertEqual(len({key[0] for key in S._FRAMES if key[0].startswith(str(folder))}), 2)

    def test_the_play_hash_is_small_deterministic_and_independent_of_the_frame_count(self):
        from manim.utils.hashing import get_json

        def fingerprint(motion):
            return get_json(motion, include_pixel_array=True)

        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            S._INDEXES.pop(folder, None)
            first = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
            second = S._Motion(folder, 1.0, [0, 0, 0], 1.0, None)
            self.assertEqual(fingerprint(first), fingerprint(second))
            self.assertLess(len(fingerprint(first)), 20000)
            second.seek(0.15)
            self.assertNotEqual(fingerprint(first), fingerprint(second))


def serving(folder: Path):
    """🧲 Make the cast API read `case` from a synthetic folder instead of running the exporter."""
    from unittest.mock import patch

    S._INDEXES.pop(folder, None)
    return patch.object(S, "ensure_casts", lambda ids=None: {cast: folder for cast in (ids or ["case"])})


class Schema(unittest.TestCase):
    SCHEMA = S._SOURCES / "🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json"
    VECTORS = json.loads((S._SOURCES / "🧫️fixtures/🎭️cast-export/🔣️.json").read_text(encoding="utf-8"))

    def validator(self):
        import jsonschema

        schema = json.loads(self.SCHEMA.read_text(encoding="utf-8"))
        jsonschema.Draft7Validator.check_schema(schema)
        return jsonschema.Draft7Validator(schema)

    def test_the_course_manifest_is_valid_and_its_assets_exist(self):
        document = S._manifest()
        errors = [f"{'/'.join(map(str, error.path))}: {error.message}" for error in self.validator().iter_errors(document)]
        self.assertEqual(errors, [])
        for cast, entry in document["casts"].items():
            with self.subTest(cast=cast):
                self.assertTrue(list((S._SOURCES / "🗂️catalog").glob(f"*/{entry['asset']}/{S._INDEX}")))

    def test_the_language_agnostic_vectors_agree_with_the_typescript_suite(self):
        validator = self.validator()
        self.assertEqual(list(validator.iter_errors(self.VECTORS["manifest"])), [])
        for vector in self.VECTORS["invalid"]:
            with self.subTest(case=vector["name"]):
                self.assertTrue(list(validator.iter_errors({"version": 1, "casts": {"bad": vector["cast"]}})))

    def test_the_manifest_keys_are_cast_slugs(self):
        validator = self.validator()
        self.assertTrue(list(validator.iter_errors({"version": 1, "casts": {"Bad Id": self.VECTORS["manifest"]["casts"]["radiator-rated"]}})))
        self.assertTrue(list(validator.iter_errors({"version": 2, "casts": {}})))


class Reference(unittest.TestCase):
    def test_the_lazy_parser_agrees_with_an_xml_parser_on_paths_group_and_slots(self):
        import xml.etree.ElementTree as ElementTree

        body = ('<g id="staffage-cast" transform="matrix(-1 0 0 1 90 2)"><g id="shape-a"><path d="M10 10L20 10L20 30Z" fill="#ff0000"/></g>'
                '<path d="M1 1L5 1M2 2L6 8" fill="none" stroke="#123456" stroke-width="0.25"/>'
                '<path d="M30 30L40 30L40 40L30 40L30 30Z" fill="#abc"/></g>')
        head = '<svg xmlns="http://www.w3.org/2000/svg" width="100mm" height="100mm" viewBox="0 0 100 100">'
        index = {**CASE["index"], "mmPerMetre": 10, "originMm": [12.0, 70.0]}
        with tempfile.TemporaryDirectory() as root:
            folder = Path(root) / "case"
            folder.mkdir()
            (folder / "0000.svg").write_text(head + body + "</svg>", encoding="utf-8")
            (folder / "🔣️.json").write_text(json.dumps({**index, "frames": ["0000.svg"]}), encoding="utf-8")
            tree = ElementTree.fromstring(head + body + "</svg>")
            paths = tree.findall(".//{http://www.w3.org/2000/svg}path")
            group = tree.find(".//{http://www.w3.org/2000/svg}g[@id='staffage-cast']")
            matrix = [float(value) for value in group.get("transform")[len("matrix("):-1].split()]
            frame = S._parse_frame(head + body + "</svg>", index)
            self.assertEqual(len(frame.shapes), len(paths))
            S._SLOT_COUNTS.pop(folder, None)
            S._INDEXES.pop(folder, None)
            self.assertEqual(S._slot_count(folder), len(paths))
            first = paths[0].get("d").split("L")[0][1:].split()
            x = matrix[0] * float(first[0]) + matrix[2] * float(first[1]) + matrix[4]
            y = matrix[1] * float(first[0]) + matrix[3] * float(first[1]) + matrix[5]
            np.testing.assert_allclose(frame.points[0], [(x - 12.0) / 10, (70.0 - y) / 10], atol=1e-9)
            self.assertEqual([shape.stroke for shape in frame.shapes], [None, "#123456", None])


class Fit(unittest.TestCase):
    def test_the_unit_is_the_largest_that_fits_and_matches_an_independent_bounding_box(self):
        import svgelements

        extent = CASE["fit"]["worldMetres"]
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            parsed = svgelements.SVG.parse(str(folder / "0000.svg"))
            pixels = float(parsed.width) / CASE["width"]
            boxes = [e.bbox() for e in parsed.elements() if isinstance(e, svgelements.Shape) and e.bbox()]
            width = (max(b[2] for b in boxes) - min(b[0] for b in boxes)) / pixels / CASE["index"]["mmPerMetre"]
            self.assertAlmostEqual(width, extent["width"], places=6)
            with serving(folder):
                for case in CASE["fit"]["cases"]:
                    fitted = V.fitted_staffage("case", width=case["width"], height=case["height"])
                    self.assertAlmostEqual(fitted.width, width * case["unit"], places=6, msg=case)
                    self.assertLessEqual(fitted.height, (case["height"] or 1e9) + 1e-9)

    def test_rate_turns_the_cast_into_a_motion_and_loop_repeats_a_clip(self):
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            with serving(folder):
                self.assertNotIsInstance(V.fitted_staffage("case", width=1.0), S._Motion)
                for case in CASE["fit"]["motion"]:
                    index = json.loads((folder / "🔣️.json").read_text(encoding="utf-8"))
                    (folder / "🔣️.json").write_text(json.dumps({**index, "loop": case["castLoops"]}), encoding="utf-8")
                    S._INDEXES.pop(folder, None)
                    motion = V.fitted_staffage("case", width=1.0, rate=case["rate"], loop=case["loop"])
                    self.assertIsInstance(motion, S._Motion)
                    self.assertEqual(motion.loops, case["loops"], case)
                    self.assertEqual(motion.rate, case["rate"])


class Rates(unittest.TestCase):
    def test_the_rate_multiplies_scene_time_into_cast_time(self):
        with tempfile.TemporaryDirectory() as root:
            folder = synthetic(Path(root))
            for case in CASE["rates"]:
                with serving(folder):
                    motion = S.staffage_motion("case", unit=1.0, rate=case["rate"])
                    S.SCENE_CLOCK.now = 3.0
                    motion.update(0.0)
                    S.SCENE_CLOCK.now = 3.0 + case["advance"]
                    motion.update(0.0)
                    self.assertEqual(motion.frame, case["frame"], case)
                    self.assertAlmostEqual(motion.elapsed, case["advance"] * case["rate"], places=9)


class Cache(unittest.TestCase):
    def test_keys_are_stable_and_content_addressed(self):
        entry = CASE["key"]["entry"]
        with tempfile.TemporaryDirectory() as root:
            tree = Path(root)
            (tree / "🔨️modules").mkdir()
            (tree / "🧬️schema").mkdir()
            (tree / "🗂️catalog/humans/case-asset").mkdir(parents=True)
            (tree / "🔨️modules/a.ts").write_text("a", encoding="utf-8")
            (tree / "🧬️schema/b.ts").write_text("b", encoding="utf-8")
            (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text("{}", encoding="utf-8")
            first = S.cast_key(entry, tree)
            self.assertEqual(len(first), CASE["key"]["length"])
            self.assertEqual(first, S.cast_key(dict(reversed(list(entry.items()))), tree))
            self.assertNotEqual(first, S.cast_key({**entry, "pose": "other"}, tree))

    def test_the_viewpoint_is_required_in_indexes_and_entries(self):
        case = CASE["viewpoints"]
        entry = CASE["key"]["entry"]
        with tempfile.TemporaryDirectory() as root:
            tree = Path(root)
            (tree / "🔨️modules").mkdir()
            (tree / "🧬️schema").mkdir()
            (tree / "🗂️catalog/humans/case-asset").mkdir(parents=True)
            (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text("{}", encoding="utf-8")
            keys = {form["viewpoint"]: S.cast_key({**entry, "viewpoint": form["viewpoint"]}, tree) for form in case["forms"]}
            self.assertEqual(len(set(keys.values())), len(case["valid"]))
            for form in case["forms"]:
                with self.subTest(form=form["viewpoint"]):
                    folder = Path(root) / form["viewpoint"]
                    folder.mkdir()
                    (folder / S._INDEX).write_text(json.dumps({**CASE["index"], "frames": [], "viewpoint": form["viewpoint"]}), encoding="utf-8")
                    self.assertEqual(S._index(folder)["viewpoint"], form["viewpoint"])
            for name, content in {"missing": {k: v for k, v in CASE["index"].items() if k != "viewpoint"}, "bad": {**CASE["index"], "viewpoint": case["invalid"]}}.items():
                folder = Path(root) / name
                folder.mkdir()
                (folder / S._INDEX).write_text(json.dumps({**content, "frames": []}), encoding="utf-8")
                with self.assertRaises(ValueError):
                    S._index(folder)
            for bad in ({k: v for k, v in entry.items() if k != "viewpoint"}, {**entry, "viewpoint": case["invalid"]}):
                with self.assertRaises(ValueError):
                    S.cast_key(bad, tree)

    def test_each_input_changes_the_key(self):
        entry = CASE["key"]["entry"]
        for relative in ("🔨️modules/a.ts", "🧬️schema/b.ts", "🗂️catalog/humans/case-asset/🔣️.json"):
            with tempfile.TemporaryDirectory() as root:
                tree = Path(root)
                for folder in ("🔨️modules", "🧬️schema", "🗂️catalog/humans/case-asset"):
                    (tree / folder).mkdir(parents=True)
                (tree / "🔨️modules/a.ts").write_text("a", encoding="utf-8")
                (tree / "🧬️schema/b.ts").write_text("b", encoding="utf-8")
                (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text("{}", encoding="utf-8")
                before = S.cast_key(entry, tree)
                S._source_digest.cache_clear()
                S._catalog_bytes.cache_clear()
                (tree / relative).write_text("changed", encoding="utf-8")
                self.assertNotEqual(before, S.cast_key(entry, tree), relative)

    def test_a_clip_names_the_content_key_of_every_cast_it_draws_so_a_catalog_edit_changes_it(self):
        entry = CASE["key"]["entry"]
        manifest = {"casts": {"case-cast": entry, "other-cast": {**entry, "viewpoint": "plan"}}}
        with tempfile.TemporaryDirectory() as root:
            tree = Path(root)
            for folder in ("🔨️modules", "🧬️schema", "🗂️catalog/humans/case-asset"):
                (tree / folder).mkdir(parents=True)
            (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text("{}", encoding="utf-8")
            clip = tree / "clip.py"
            clip.write_text('draw("case-cast")\nprefix = "other-"\nunrelated = "case-asset"\n', encoding="utf-8")
            before = S.casts_in([clip, tree / "missing.py"], manifest, tree)
            self.assertEqual(sorted(before), ["case-cast", "other-cast"])
            self.assertNotEqual(before["case-cast"], before["other-cast"])
            S._catalog_bytes.cache_clear()
            (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text('{"changed": true}', encoding="utf-8")
            after = S.casts_in([clip], manifest, tree)
            self.assertNotEqual(before["case-cast"], after["case-cast"])
            self.assertNotEqual(before["other-cast"], after["other-cast"])
            clip.write_text('draw("other-cast")\n', encoding="utf-8")
            self.assertEqual(sorted(S.casts_in([clip], manifest, tree)), ["other-cast"])

    def test_a_print_pattern_paint_reads_as_no_fill_and_any_other_url_is_refused(self):
        self.assertIsNone(S._colour("url(#staffage-pattern-hot-001117)", "#112233"))
        self.assertEqual(S._colour("#abc", None), "#AABBCC")
        with self.assertRaises(ValueError):
            S._colour("url(#gradient-1)", None)

    def test_prune_removes_old_stale_folders_files_and_empty_keys_without_raising(self):
        with tempfile.TemporaryDirectory() as root:
            cache = Path(root)
            for name in ("live", "stale", "empty", ".tmp-abandoned"):
                (cache / name).mkdir()
            (cache / "stale" / "🔣️.json").write_text("{}", encoding="utf-8")
            (cache / "loose-file").write_text("x", encoding="utf-8")
            S._prune(cache, {"live"}, now=time.time() + S._PRUNE_AGE_S + 10)
            self.assertEqual(sorted(path.name for path in cache.iterdir()), ["live", "loose-file"])
            S._prune(cache / "missing", {"live"})

    def test_the_key_ignores_line_endings_and_the_case_rule_of_the_path_order(self):
        entry = CASE["key"]["entry"]
        endings = CASE["key"]["lineEndings"]
        names = CASE["key"]["order"]["files"]

        def key(text: str, order: list[str]) -> str:
            with tempfile.TemporaryDirectory() as root:
                tree = Path(root)
                for folder in ("🔨️modules", "🧬️schema", "🗂️catalog/humans/case-asset"):
                    (tree / folder).mkdir(parents=True)
                for name in order:
                    (tree / "🔨️modules" / name).write_bytes(text.encode("utf-8"))
                (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_bytes(text.encode("utf-8"))
                S._source_digest.cache_clear()
                S._catalog_bytes.cache_clear()
                return S.cast_key(entry, tree)

        self.assertEqual(key(endings["lf"], names), key(endings["crlf"], names))
        self.assertEqual(key(endings["lf"], names), key(endings["lf"], list(reversed(names))))
        with tempfile.TemporaryDirectory() as root:
            tree = Path(root)
            (tree / "🔨️modules").mkdir()
            for name in names:
                (tree / "🔨️modules" / name).write_text("x", encoding="utf-8")
            S._source_digest.cache_clear()
            seen: list[str] = []
            real = hashlib.sha256

            class Spy:
                def __init__(self):
                    self.inner = real()

                def update(self, data):
                    seen.append(data.split(b"\0")[0].decode("utf-8"))
                    self.inner.update(data)

                def hexdigest(self):
                    return self.inner.hexdigest()

            with patch.object(S.hashlib, "sha256", Spy):
                S._source_digest(tree)
            S._source_digest.cache_clear()
        self.assertEqual([Path(name).name for name in seen if name], sorted(names))

    def test_bundled_inputs_outside_the_sources_change_the_key(self):
        entry = CASE["key"]["entry"]
        for relative in CASE["key"]["bundled"]:
            with tempfile.TemporaryDirectory() as root:
                tree = Path(root)
                for folder in ("🔨️modules", "🧬️schema", "🗂️catalog/humans/case-asset", Path(relative).parent.as_posix()):
                    (tree / folder).mkdir(parents=True, exist_ok=True)
                (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text("{}", encoding="utf-8")
                (tree / relative).write_text("{}", encoding="utf-8")
                before = S.cast_key(entry, tree)
                S._source_digest.cache_clear()
                (tree / relative).write_text('{"changed": true}', encoding="utf-8")
                self.assertNotEqual(before, S.cast_key(entry, tree), relative)

    def test_only_the_environment_presets_a_cast_names_enter_its_key(self):
        spec = CASE["environments"]

        def key(entry, presets):
            with tempfile.TemporaryDirectory() as root:
                tree = Path(root)
                for folder in ("🔨️modules", "🧬️schema", "🗂️catalog/humans/case-asset", "🗂️catalog/🌤️environments"):
                    (tree / folder).mkdir(parents=True)
                (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text("{}", encoding="utf-8")
                (tree / "🗂️catalog/🌤️environments/🔣️.json").write_text(json.dumps({"presets": [{"id": slug, "airTempC": value} for slug, value in presets.items()]}), encoding="utf-8")
                return S.cast_key(entry, tree)

        base = key(spec["constant"], spec["presets"])
        self.assertEqual(base, key(spec["constant"], {**spec["presets"], spec["unused"]: 99}))
        self.assertNotEqual(base, key(spec["constant"], {**spec["presets"], spec["used"]: 99}))
        timeline = key(spec["timeline"], spec["presets"])
        self.assertNotEqual(timeline, key(spec["timeline"], {**spec["presets"], spec["unused"]: 99}))
        self.assertEqual(key(spec["plain"], spec["presets"]), key(spec["plain"], {**spec["presets"], spec["used"]: 99}))

    def test_a_transient_lock_on_the_rendered_folder_does_not_lose_the_batch(self):
        failures = CASE["publish"]["failures"]
        with tempfile.TemporaryDirectory() as root:
            source, target = Path(root, "source"), Path(root, "target")
            source.mkdir()
            (source / "🔣️.json").write_text("{}", encoding="utf-8")
            real, calls = Path.rename, []

            def locked(self, destination):
                calls.append(self)
                if len(calls) <= failures:
                    raise PermissionError(13, "Access is denied")
                return real(self, destination)

            with patch.object(Path, "rename", locked), patch.object(S.time, "sleep"):
                S._publish(source, target)
            self.assertEqual(len(calls), failures + 1)
            self.assertTrue((target / "🔣️.json").is_file())
            calls.clear()
            with patch.object(Path, "rename", lambda self, destination: calls.append(self) or (_ for _ in ()).throw(PermissionError(13, "locked"))), patch.object(S.time, "sleep"):
                with self.assertRaises(PermissionError):
                    S._publish(Path(root, "other"), Path(root, "elsewhere"))
            self.assertEqual(len(calls), S._PUBLISH_ATTEMPTS)

    def test_only_keys_without_a_cast_and_older_than_a_day_are_pruned(self):
        now = time.time()
        with tempfile.TemporaryDirectory() as root:
            tree = Path(root)
            for name, hours in CASE["prune"]["folders"].items():
                (tree / name / "cast").mkdir(parents=True)
                os.utime(tree / name, (now - hours * 3600, now - hours * 3600))
            (tree / "file.txt").write_text("kept", encoding="utf-8")
            S._prune(tree, set(CASE["prune"]["live"]), now)
            self.assertEqual(sorted(path.name for path in tree.iterdir()), sorted(CASE["prune"]["kept"]))

    def test_the_manifest_casts_have_real_assets_and_stable_keys(self):
        casts = S._manifest()["casts"]
        self.assertTrue(casts)
        for cast, entry in casts.items():
            self.assertEqual(S.cast_key(entry), S.cast_key(json.loads(json.dumps(entry))), cast)

    def test_missing_casts_render_in_one_batch_and_land_atomically(self):
        calls = []

        def fake(command, progress=None):
            calls.append(command)
            out = Path(command[command.index("--out") + 1])
            for cast in command[command.index("--ids") + 1].split(","):
                (out / cast).mkdir(parents=True)
                (out / cast / "🔣️.json").write_text(json.dumps(stamped({**CASE["index"], "frames": []}, cast)), encoding="utf-8")

        with tempfile.TemporaryDirectory() as root:
            saved = (S._run, S._repo, os.environ.get("SEMIO_TEACHING_CACHE_ROOT"))
            S._run, S._repo = fake, lambda: "repo"
            os.environ["SEMIO_TEACHING_CACHE_ROOT"] = root
            try:
                ids = list(S._manifest()["casts"])[:2]
                folders = S.ensure_casts(ids)
                self.assertEqual(len(calls), 1)
                self.assertEqual(calls[0][:6], ["repo", "tools", "exec", "--light", "bun", "--"])
                self.assertEqual(calls[0][calls[0].index("--ids") + 1], ",".join(ids))
                self.assertNotIn("--lang", calls[0])
                self.assertTrue(all((folder / "🔣️.json").is_file() for folder in folders.values()))
                S.ensure_casts(ids)
                self.assertEqual(len(calls), 1)
                self.assertEqual([p.name for p in Path(root, "staffage").iterdir() if p.name.startswith(".")], [])
            finally:
                S._run, S._repo = saved[:2]
                os.environ.pop("SEMIO_TEACHING_CACHE_ROOT", None)
                if saved[2] is not None:
                    os.environ["SEMIO_TEACHING_CACHE_ROOT"] = saved[2]

    def test_the_visuals_package_reexports_the_api(self):
        for name in ("staffage", "staffage_motion", "staffage_height", "staffage_parts", "ensure_casts"):
            self.assertIs(getattr(V, name), getattr(S, name))


WORKER = """
import hashlib, importlib, json, sys, time
from pathlib import Path
S = importlib.import_module(sys.argv[1])
def stamped(index, cast):
    provenance = S.expected_provenance(cast)
    provenance["asset"]["category"] = "humans"
    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}
    return {**index, "cast": cast, "provenance": {**provenance, "clock": clock}}
counter, spec, cast = Path(sys.argv[2]), json.loads(sys.argv[3]), sys.argv[4]
def fake(command, progress=None):
    with counter.open("a") as handle:
        handle.write("x")
    time.sleep(spec["exportS"])
    out = Path(command[command.index("--out") + 1])
    for name in command[command.index("--ids") + 1].split(","):
        (out / name).mkdir(parents=True)
        (out / name / S._INDEX).write_text(json.dumps(stamped(spec["index"], name)), encoding="utf-8")
S._run, S._repo = fake, lambda: "repo"
print("ready", flush=True)
sys.stdin.readline()
lines = []
folders = S.ensure_casts([cast], lines.append)
print(json.dumps({"sha": hashlib.sha256((folders[cast] / S._INDEX).read_bytes()).hexdigest(), "lines": lines}), flush=True)
"""

OWNER = """
import importlib, sys, time
S = importlib.import_module(sys.argv[1])
lock = S._lock(sys.argv[2])
print(lock.acquire(), flush=True)
print(lock.path, flush=True)
time.sleep(60)
"""


class Exclusive(unittest.TestCase):
    SPEC = CASE["exclusive"]

    @staticmethod
    def reap(process: subprocess.Popen) -> None:
        if sys.platform == "win32":
            subprocess.run(["taskkill", "/PID", str(process.pid), "/T", "/F"], capture_output=True, check=False)
        process.kill()
        process.wait()
        process.stdin.close()
        process.stdout.close()

    def spawn(self, code: str, *arguments: str, root: str) -> subprocess.Popen:
        environment = {**os.environ, "PYTHONUTF8": "1", "SEMIO_TEACHING_CACHE_ROOT": root}
        return subprocess.Popen([sys.executable, "-c", code, S.__name__, *arguments], cwd=S._WORKSPACE, env=environment, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, encoding="utf-8")

    def cast(self) -> str:
        return next(iter(S._manifest()["casts"]))

    @contextlib.contextmanager
    def cache(self, root: str):
        with patch.dict(os.environ, {"SEMIO_TEACHING_CACHE_ROOT": root}):
            yield

    def fake(self, calls: list[list[str]]):
        def run(command, progress=None):
            calls.append(command)
            out = Path(command[command.index("--out") + 1])
            for name in command[command.index("--ids") + 1].split(","):
                (out / name).mkdir(parents=True)
                (out / name / S._INDEX).write_text(json.dumps(stamped(self.SPEC["index"], name)), encoding="utf-8")

        return patch.multiple(S, _run=run, _repo=lambda: "repo")

    def test_renderers_in_separate_processes_export_a_cast_once_and_get_identical_bytes(self):
        with tempfile.TemporaryDirectory() as root:
            counter = Path(root, "calls")
            workers = [self.spawn(WORKER, str(counter), json.dumps(self.SPEC), self.cast(), root=root) for _ in range(self.SPEC["processes"])]
            try:
                for worker in workers:
                    self.assertEqual(worker.stdout.readline().strip(), "ready")
                for worker in workers:
                    worker.stdin.write("\n")
                    worker.stdin.flush()
                results = [json.loads(worker.stdout.readline()) for worker in workers]
            finally:
                for worker in workers:
                    self.reap(worker)
            self.assertEqual(len(counter.read_text()), self.SPEC["exporterCalls"])
            self.assertEqual(len({result["sha"] for result in results}), 1)
            waited = [line for result in results for line in result["lines"] if line.startswith(self.SPEC["waitPrefix"])]
            self.assertTrue(waited)
            self.assertTrue(all(self.cast() in line for line in waited))
            self.assertEqual([path.name for path in Path(root, "staffage").iterdir() if path.name.startswith(".")], [])

    def test_a_lock_of_a_dead_owner_is_recovered(self):
        with tempfile.TemporaryDirectory() as root, self.cache(root):
            owner = self.spawn(OWNER, self.cast(), root=root)
            try:
                self.assertEqual(owner.stdout.readline().strip(), "True")
                contender = S._Lock(Path(owner.stdout.readline().strip()))
                self.assertFalse(contender.acquire())
                self.reap(owner)
                deadline = time.monotonic() + self.SPEC["ownerDeathRecoveredWithinS"]
                while not contender.acquire():
                    self.assertLess(time.monotonic(), deadline, "the lock of a dead owner stayed")
                    time.sleep(0.1)
                contender.release()
                calls: list[list[str]] = []
                with self.fake(calls):
                    folders = S.ensure_casts([self.cast()])
                self.assertEqual(len(calls), 1)
                self.assertTrue((folders[self.cast()] / S._INDEX).is_file())
            finally:
                self.reap(owner)

    def test_a_waiting_renderer_takes_the_result_of_the_owner_without_exporting(self):
        with tempfile.TemporaryDirectory() as root, self.cache(root):
            cast = self.cast()
            owner = S._lock(cast)
            self.assertTrue(owner.acquire())
            folder = S._directory(cast)

            def finish():
                folder.mkdir(parents=True)
                (folder / S._INDEX).write_text(json.dumps(stamped(self.SPEC["index"], cast)), encoding="utf-8")
                owner.release()

            timer = threading.Timer(self.SPEC["ownerReleaseAfterS"], finish)
            timer.start()
            lines: list[str] = []
            calls: list[list[str]] = []
            try:
                with self.fake(calls):
                    S.ensure_casts([cast], lines.append)
            finally:
                timer.join()
            self.assertEqual(calls, [])
            self.assertEqual(lines, [f"{self.SPEC['waitPrefix']}{cast}"])

    def test_waiting_honours_cancellation(self):
        class Stop(Exception):
            pass

        def progress(line: str) -> None:
            raise Stop

        with tempfile.TemporaryDirectory() as root, self.cache(root):
            owner = S._lock(self.cast())
            self.assertTrue(owner.acquire())
            calls: list[list[str]] = []
            try:
                with self.fake(calls), self.assertRaises(Stop):
                    S.ensure_casts([self.cast()], progress)
            finally:
                owner.release()
            self.assertEqual(calls, [])
            again = S._lock(self.cast())
            self.assertTrue(again.acquire())
            again.release()

    def test_a_failed_export_hands_the_lock_on(self):
        with tempfile.TemporaryDirectory() as root, self.cache(root):
            with patch.multiple(S, _run=lambda command, progress=None: (_ for _ in ()).throw(RuntimeError("boom")), _repo=lambda: "repo"):
                with self.assertRaises(RuntimeError):
                    S.ensure_casts([self.cast()])
            again = S._lock(self.cast())
            self.assertTrue(again.acquire())
            again.release()


def alive(pid: int) -> bool:
    """🔎 Tell whether a process still runs, without a third-party library."""
    if sys.platform == "win32":
        listing = subprocess.run(["tasklist", "/FI", f"PID eq {pid}", "/NH", "/FO", "CSV"], capture_output=True, text=True, check=False).stdout
        return f'"{pid}"' in listing
    state = subprocess.run(["ps", "-o", "stat=", "-p", str(pid)], capture_output=True, text=True, check=False).stdout.strip()
    return bool(state) and not state.startswith("Z")


def snapshot(root: Path) -> dict[str, bytes | None]:
    """📸 Capture every folder and file below `root` with its bytes, leaving out the empty `.lock-*` files of the renderers and the key folders that hold nothing else."""
    def kept(path: Path) -> bool:
        if path.name.startswith(".lock-"):
            return False
        children = list(path.iterdir()) if path.is_dir() else []
        return not children or not all(child.name.startswith(".lock-") for child in children)

    return {path.relative_to(root).as_posix(): path.read_bytes() if path.is_file() else None for path in sorted(root.rglob("*")) if kept(path)}


class Cancellation(unittest.TestCase):
    VECTORS = json.loads((S._SOURCES / "🧫️fixtures/🎭️cast-cancellation/🔣️.json").read_text(encoding="utf-8"))

    def run_with(self, root: Path, ids, progress):
        manifest = root / "manifest.json"
        manifest.write_text(json.dumps(self.VECTORS["manifest"]), encoding="utf-8")
        environment = {"SEMIO_TEACHING_CACHE_ROOT": str(root / "cache")}
        with patch.dict(os.environ, environment), patch.object(S, "_MANIFEST", manifest), contextlib.redirect_stderr(io.StringIO()):
            return S.ensure_casts(ids, progress)

    def test_progress_reaches_the_caller_and_the_cache_gets_the_cast(self):
        lines: list[str] = []
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            folders = self.run_with(root, self.VECTORS["completeIds"], lines.append)
            for expected in self.VECTORS["expectedProgress"]["quick"]:
                self.assertTrue(any(line.startswith(expected) for line in lines), expected)
            self.assertTrue(any(re.match(self.VECTORS["progress"]["pid"], line) for line in lines))
            self.assertTrue(all((folder / "🔣️.json").is_file() for folder in folders.values()))
            self.assertEqual([path.name for path in (root / "cache/staffage").iterdir() if path.name.startswith(".")], [])

    def test_an_interrupt_mid_export_kills_the_process_tree_and_leaves_the_cache_untouched(self):
        lines: list[str] = []
        spawned: list[int] = []
        real = subprocess.Popen

        class Recording(real):
            def __init__(self, *args, **kwargs):
                super().__init__(*args, **kwargs)
                spawned.append(self.pid)

        def progress(line: str) -> None:
            lines.append(line)
            if re.match(self.VECTORS["progress"]["cast"], line):
                timer = threading.Timer(0.5, interrupt)
                timers.append(timer)
                timer.start()

        def interrupt() -> None:
            interrupted.append(time.monotonic())
            _thread.interrupt_main()

        interrupted: list[float] = []
        timers: list[threading.Timer] = []
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            keep = root / "cache/staffage/keep"
            keep.mkdir(parents=True)
            (keep / "🔣️.json").write_text("{}", encoding="utf-8")
            before = snapshot(root / "cache")
            with patch.object(subprocess, "Popen", Recording):
                try:
                    with self.assertRaises(KeyboardInterrupt):
                        self.run_with(root, self.VECTORS["slowIds"], progress)
                finally:
                    for timer in timers:
                        timer.cancel()
            self.assertLess(time.monotonic() - interrupted[0], 60)
            self.assertEqual(snapshot(root / "cache"), before)
            total = self.VECTORS["slowFrames"] * len(self.VECTORS["slowIds"])
            self.assertFalse([line for line in lines if line.startswith(f"[{total}/{total}]")])
        pids = {spawned[0]}
        for line in lines:
            found = re.match(self.VECTORS["progress"]["pid"], line)
            if found:
                pids.update(int(group) for group in found.groups() if int(group) != os.getpid())
        self.assertGreaterEqual(len(pids), 2, lines)
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline and any(alive(pid) for pid in pids):
            time.sleep(0.2)
        self.assertEqual([pid for pid in pids if alive(pid)], [], "orphaned exporter processes")

    def test_a_grandchild_of_the_command_dies_with_it(self):
        code = "import subprocess,sys,time;c=subprocess.Popen([sys.executable,'-c','import time;time.sleep(300)']);print('[pid %d] ppid %d'%(c.pid,c.pid),file=sys.stderr,flush=True);time.sleep(300)"
        spawned: list[int] = []
        real = subprocess.Popen

        class Recording(real):
            def __init__(self, *args, **kwargs):
                super().__init__(*args, **kwargs)
                spawned.append(self.pid)

        class Stop(Exception):
            pass

        def progress(line: str) -> None:
            found = re.match(self.VECTORS["progress"]["pid"], line)
            if found:
                spawned.append(int(found.group(1)))
                raise Stop

        with patch.object(subprocess, "Popen", Recording), contextlib.redirect_stderr(io.StringIO()):
            with self.assertRaises(Stop):
                S._run([sys.executable, "-c", code], progress)
        self.assertEqual(len(spawned), 2)
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline and any(alive(pid) for pid in spawned):
            time.sleep(0.2)
        self.assertEqual([pid for pid in spawned if alive(pid)], [])

    def test_a_failing_progress_callback_cancels_like_an_interrupt(self):
        class Stop(Exception):
            pass

        def progress(line: str) -> None:
            if re.match(self.VECTORS["progress"]["cast"], line):
                raise Stop

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with self.assertRaises(Stop):
                self.run_with(root, self.VECTORS["slowIds"], progress)
            self.assertEqual(snapshot(root / "cache"), {"staffage": None})

    def test_a_terminate_signal_stops_the_tree_and_the_handlers_come_back(self):
        import signal

        number = signal.SIGTERM
        before = signal.getsignal(number)
        seen: list[object] = []
        with S._terminable():
            seen.append(signal.getsignal(number))
            with self.assertRaises(SystemExit) as raised:
                signal.getsignal(number)(number, None)
        self.assertIsNot(seen[0], before)
        self.assertEqual(raised.exception.code, 128 + number)
        self.assertIs(signal.getsignal(number), before)
        done: list[object] = []
        worker = threading.Thread(target=lambda: done.append(S._terminable().__enter__()))
        worker.start()
        worker.join()
        self.assertIs(signal.getsignal(number), before)

    def test_a_terminate_signal_during_the_export_leaves_no_process_behind(self):
        code = "import subprocess,sys,time;c=subprocess.Popen([sys.executable,'-c','import time;time.sleep(300)']);print('[pid %d] ppid %d'%(c.pid,c.pid),file=sys.stderr,flush=True);time.sleep(300)"
        pids: list[int] = []
        real = subprocess.Popen

        class Recording(real):
            def __init__(self, *args, **kwargs):
                super().__init__(*args, **kwargs)
                pids.append(self.pid)

        def progress(line: str) -> None:
            found = re.match(self.VECTORS["progress"]["pid"], line)
            if found:
                pids.append(int(found.group(1)))
                threading.Timer(0.2, lambda: _thread.interrupt_main() if sys.platform == "win32" else os.kill(os.getpid(), 15)).start()

        with patch.object(subprocess, "Popen", Recording), contextlib.redirect_stderr(io.StringIO()):
            with self.assertRaises((SystemExit, KeyboardInterrupt)):
                S._run([sys.executable, "-c", code], progress)
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline and any(alive(pid) for pid in pids):
            time.sleep(0.2)
        self.assertEqual([pid for pid in pids if alive(pid)], [])

    def test_the_exporter_is_told_which_process_to_watch(self):
        commands: list[list[str]] = []
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with patch.object(S, "_run", lambda command, progress=None: commands.append(command)):
                try:
                    self.run_with(root, self.VECTORS["completeIds"], None)
                except OSError:
                    pass
        self.assertEqual(len(commands), 1)
        self.assertEqual(commands[0][commands[0].index("--parent") + 1], str(os.getpid()))


if __name__ == "__main__":
    unittest.main()
