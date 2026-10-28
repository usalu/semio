"""🖱️ Verify portable cursor hit trees, event commits and canonical visual states."""

from __future__ import annotations

import copy
import importlib
import json
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction"


def nodes_for(case):
    """🌳 Build an independent fixture tree with explicit disabled nodes and stacking cases."""
    nodes = copy.deepcopy(CASES["nodes"] + case.get("extra", []))
    for node in nodes:
        node["enabled"] = node["id"] not in case.get("disabled", [])
        if node["id"] in case.get("outlines", {}):
            node["outline"] = copy.deepcopy(case["outlines"][node["id"]])
    return nodes


def mask_hit_path(point, nodes):
    """🎭 Resolve half-open hit regions independently with Pillow masks and NumPy intersection."""
    import numpy as np
    from PIL import Image, ImageChops, ImageDraw

    width, height = CASES["viewport"]
    x, y = point
    if x < 0 or y < 0 or x >= width or y >= height:
        return []
    masks = {}
    for node in nodes:
        rectangle = node["rect"]
        image = Image.new("1", (width, height), 0)
        ImageDraw.Draw(image).rectangle((rectangle["x"], rectangle["y"], rectangle["x"] + rectangle["width"] - 1,
                                        rectangle["y"] + rectangle["height"] - 1), fill=1)
        if "outline" in node:
            polygon = Image.new("1", (width, height), 0)
            ImageDraw.Draw(polygon).polygon([tuple(point) for point in node["outline"]], fill=1)
            image = ImageChops.logical_and(image,polygon)
        masks[node["id"]] = np.asarray(image, dtype=bool)
    candidates = []
    for index, node in enumerate(nodes):
        current, path, eligible = node, [], True
        while current:
            path.insert(0, current["id"])
            eligible = eligible and current.get("enabled", True) and bool(masks[current["id"]][int(y), int(x)])
            current = next((entry for entry in nodes if entry["id"] == current["parent"]), None)
        if eligible:
            candidates.append(((node.get("z", 0), len(path), index), path))
    return max(candidates, default=((0, 0, 0), []))[1]


class CursorInteractionTest(unittest.TestCase):
    def setUp(self):
        self.interaction = importlib.import_module(MODULE + ".🐍️")

    def test_portable_hit_targets_match_pillow_masks_and_full_parent_chain(self):
        for case in CASES["hits"]:
            with self.subTest(case=case["id"]):
                nodes = nodes_for(case)
                self.assertEqual(mask_hit_path(case["point"], nodes), case["path"])
                self.assertEqual(self.interaction.hit_path(case["point"], nodes), case["path"])
                actual = self.interaction.hit_test(nodes, case["point"])
                self.assertEqual(actual["id"] if actual else None, case["path"][-1] if case["path"] else None)

    def test_portable_gestures_commit_only_after_enabled_same_target_release_and_click(self):
        for case in CASES["gestures"]:
            with self.subTest(case=case["id"]):
                nodes = nodes_for(case)
                state = self.interaction.initial_state()
                for index, event in enumerate(case["events"]):
                    before, tree = copy.deepcopy(state), copy.deepcopy(nodes)
                    after = self.interaction.apply_event(state, event, nodes)
                    self.assertEqual(state, before)
                    self.assertEqual(nodes, tree)
                    state = after
                    self.assertEqual(state["fullscreen"], case["expected"]["fullscreen"][index])
                    self.assertEqual([effect["type"] for effect in state["effects"]], case["expected"]["effectTypes"][index])
                self.assertEqual(state["toggles"], case["expected"]["finalToggles"])
                self.assertEqual([event["type"] for event in state["events"]], [event["type"] for event in case["events"]])

    def test_disabled_between_release_and_click_cancels_armed_activation(self):
        nodes = nodes_for({})
        rect = next(node["rect"] for node in nodes if node["id"] == "fullscreen")
        point = [rect["x"] + rect["width"]/2,rect["y"] + rect["height"]/2]
        state = self.interaction.initial_state()
        for kind in ("enter", "press", "release"):
            state = self.interaction.apply_event(state, {"type": kind, "point": point}, nodes)
        next(node for node in nodes if node["id"] == "fullscreen")["enabled"] = False
        state = self.interaction.apply_event(state, {"type": "click", "point": point}, nodes)
        self.assertIsNone(state["fullscreen"])
        self.assertEqual(state["effects"], [])

    def test_canonical_visual_roles_and_parent_emphasis(self):
        for case in CASES["visuals"]:
            with self.subTest(role=case["role"], state=case["state"]):
                actual = self.interaction.resolve_visual(case["role"], case["state"])
                for key, expected in case["expected"].items():
                    self.assertEqual(actual[key], expected)
        for role in ("root", "navbar", "footer", "canvas", "window", "tree"):
            with self.subTest(parent=role):
                normal = self.interaction.resolve_visual(role, {})
                hovered = self.interaction.resolve_visual(role, {"hovered": True})
                self.assertEqual(normal["fill"], hovered["fill"])

    def test_direct_cursor_assets_do_not_pull_unrelated_sibling_trees_into_source_state(self):
        import tempfile
        from unittest.mock import patch

        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            folder = Path(directory)
            script = folder / "authored" / "scene.py"
            script.parent.mkdir()
            script.write_text("pass", encoding="utf-8")
            assets = folder / "canonical"
            assets.mkdir()
            direct = assets / "cursor.svg"
            unrelated = assets / "unrelated.svg"
            direct.write_text("<svg/>", encoding="utf-8")
            unrelated.write_text("<svg/>", encoding="utf-8")
            settings = {"language": "en", "audience": "speaker", "quality": "-ql", "resolution": None, "fps": None}
            with patch.object(runtime, "teaching_root", return_value=folder), \
                    patch.object(runtime, "source_dependencies", return_value=[script]), \
                    patch.object(runtime, "source_narration", return_value=[]), \
                    patch.object(self.interaction, "source_files", return_value=(direct,)):
                first = runtime.segment_state({"script": "authored/scene.py", "scene": "Fixture"}, settings)
                unrelated.write_text("<svg id='unrelated-change'/>", encoding="utf-8")
                self.assertEqual(runtime.segment_state({"script": "authored/scene.py", "scene": "Fixture"}, settings), first)
                direct.write_text("<svg id='real-cursor-change'/>", encoding="utf-8")
                self.assertNotEqual(runtime.segment_state({"script": "authored/scene.py", "scene": "Fixture"}, settings), first)

    def test_presentation_cache_covers_interaction_contract_and_canonical_assets(self):
        presentation = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🐍️")
        expected = {path.resolve() for path in self.interaction.source_files()}
        actual = {path.resolve() for path in presentation.source_files()}
        self.assertEqual(expected - actual, set())
        package = MODULE.rsplit(".🖱️interaction", 1)[0]
        for name in ("📐️geometry", "🔤️typography"):
            owner = importlib.import_module(package + "." + name + ".🐍️")
            self.assertEqual({path.resolve() for path in owner.source_files()} - actual, set())

    def test_cursor_outlines_match_independent_native_svg_vertices_and_colors(self):
        import numpy as np
        from manim import SVGMobject

        contract = self.interaction.interaction_contract()
        for kind in ("arrow", "pointer", "selectable"):
            with self.subTest(cursor=kind):
                outline = self.interaction.cursor_outline(kind)
                source = ROOT / contract["cursors"][kind]["asset"]
                native = SVGMobject(source, should_center=False, height=None, width=None)
                self.assertEqual(len(outline["paths"]), len(native.submobjects))
                for expected, actual in zip(outline["paths"], native.submobjects, strict=True):
                    vertices = np.concatenate((actual.get_start_anchors(), actual.get_end_anchors()[-1:]))
                    vertices = vertices[:, :2].copy()
                    vertices[:, 1] = native.get_top()[1] + native.get_bottom()[1] - vertices[:, 1]
                    np.testing.assert_allclose(expected["points"], vertices, rtol=0, atol=1e-5)
                    self.assertEqual(expected["fill"].lower(), actual.get_fill_color().to_hex().lower())
                    self.assertEqual(expected["stroke"].lower(), actual.get_stroke_color().to_hex().lower())

    def test_css_border_transition_matches_independent_numpy_bezier_roots(self):
        import numpy as np
        from PIL import ImageColor

        case = CASES["transition"]
        before, after = (np.array(ImageColor.getrgb(case[key]), dtype=float) for key in ("before", "after"))
        x1, y1, x2, y2 = case["controlPoints"]
        for sample in case["samples"]:
            with self.subTest(elapsed=sample["elapsed"]):
                progress = min(1, max(0, sample["elapsed"] / case["duration"]))
                roots = np.roots([3 * x1 - 3 * x2 + 1, 3 * x2 - 6 * x1, 3 * x1, -progress])
                parameter = next(float(root.real) for root in roots if abs(root.imag) < 1e-8 and -1e-8 <= root.real <= 1 + 1e-8)
                value = 3 * (1 - parameter) ** 2 * parameter * y1 + 3 * (1 - parameter) * parameter ** 2 * y2 + parameter ** 3
                channels = np.floor(before + (after - before) * value + 0.5).astype(int)
                reference = "#" + "".join(f"{channel:02x}" for channel in channels)
                self.assertEqual(reference, sample["expected"])
                self.assertEqual(self.interaction.transition_color(case["before"], case["after"], sample["elapsed"]), reference)
        for elapsed in case["invalidElapsed"]:
            with self.assertRaises(ValueError):
                self.interaction.transition_color(case["before"], case["after"], float(elapsed))
        for color in case["invalidColors"]:
            with self.assertRaises(ValueError):
                self.interaction.transition_color(color, case["after"], 0.06)

    def test_active_hover_blend_matches_independent_numpy_srgb_oracle(self):
        import numpy as np
        from PIL import ImageColor

        case = CASES["blend"]
        mixed = np.floor(np.array(ImageColor.getrgb(case["primary"])) * case["primaryRatio"] +
                         np.array(ImageColor.getrgb(case["light"])) * (1 - case["primaryRatio"]) + 0.5).astype(int)
        expected = "#" + "".join(f"{channel:02x}" for channel in mixed)
        self.assertEqual(expected, case["expected"])
        self.assertEqual(self.interaction.resolve_visual("button", {"selected": True, "hovered": True})["fill"], expected)



class CursorIntervalTest(unittest.TestCase):
    def setUp(self):
        self.runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")

    def test_portable_interval_normalization_and_excerpt_rebasing(self):
        import tempfile

        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            clip = Path(directory) / "source.mp4"
            for case in CASES["intervals"]["normalize"]:
                with self.subTest(case=case["id"]):
                    self.runtime.publish_cursor_intervals(clip, case["input"])
                    self.assertEqual(self.runtime.cursor_intervals(clip), case["expected"])
            for case in CASES["intervals"]["clip"]:
                with self.subTest(case=case["id"]):
                    import numpy as np

                    intervals = np.array([[part["start"], part["end"]] for part in case["input"]], dtype=float)
                    clipped = np.column_stack((np.maximum(intervals[:, 0], case["range"][0]),
                                               np.minimum(intervals[:, 1], case["range"][1])))
                    clipped = clipped[clipped[:, 1] > clipped[:, 0]] - case["range"][0]
                    reference = [{"start": start, "end": end} for start, end in clipped.tolist()]
                    self.assertEqual(reference, case["expected"])
                    self.assertEqual(self.runtime.clip_cursor_intervals(case["input"], *case["range"]), reference)
            for start, end in CASES["intervals"]["invalid"]:
                with self.subTest(invalid=(start, end)), self.assertRaises((ValueError, TypeError)):
                    self.runtime.publish_cursor_intervals(clip, [{"start": float(start), "end": float(end)}])

    def test_actual_concat_and_excerpt_commands_transport_cursor_intervals(self):
        import tempfile
        from unittest.mock import patch

        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            folder = Path(directory)
            first, second, joined, excerpt = (folder / name for name in ("a.mp4", "b.mp4", "joined.mp4", "excerpt.mp4"))
            for clip in (first, second):
                clip.write_bytes(b"fixture" * 300)
            self.runtime.publish_cursor_intervals(first, [{"start": 0.25, "end": 1.5}])
            self.runtime.publish_cursor_intervals(second, [{"start": 0, "end": 0.5}])
            durations = {first: 2, second: 1, joined: 3, excerpt: 1.75}

            def command(arguments, **kwargs):
                Path(arguments[-1]).write_bytes(b"encoded-fixture" * 200)

            with patch.object(self.runtime, "clip_duration", side_effect=lambda clip: durations[Path(clip)]), \
                    patch.object(self.runtime, "run_command", side_effect=command):
                self.runtime.concat_videos([first, second], joined, folder / "concat.txt")
                self.assertEqual(self.runtime.cursor_intervals(joined), [{"start": 0.25, "end": 1.5}, {"start": 2, "end": 2.5}])
                self.runtime.cut_clip(joined, excerpt, 0.5, 2.25)
            self.assertEqual(self.runtime.cursor_intervals(excerpt), [{"start": 0, "end": 1}, {"start": 1.5, "end": 1.75}])

    def test_concat_offsets_preserve_ranges_and_empty_inputs_need_no_probe(self):
        import tempfile
        from unittest.mock import patch

        case = CASES["intervals"]["concat"]
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            paths = [Path(directory) / f"input-{index}.mp4" for index in range(len(case["durations"]))]
            output = Path(directory) / "result.mp4"
            for clip, intervals in zip(paths, case["input"], strict=True):
                self.runtime.publish_cursor_intervals(clip, intervals)
            durations = dict(zip(paths, case["durations"], strict=True))
            with patch.object(self.runtime, "clip_duration", side_effect=lambda clip: durations[clip]) as probe:
                self.runtime.propagate_cursor_intervals(paths, output)
                self.assertEqual(probe.call_count, len(paths))
            self.assertEqual(self.runtime.cursor_intervals(output), case["expected"])
            for clip in paths:
                self.runtime.publish_cursor_intervals(clip, [])
            with patch.object(self.runtime, "clip_duration") as probe:
                self.runtime.propagate_cursor_intervals(paths, output)
                probe.assert_not_called()
            self.assertEqual(self.runtime.cursor_intervals(output), [])
            self.runtime.publish_cursor_intervals(paths[0], [{"start": 0, "end": case["durations"][0] + 1}])
            with patch.object(self.runtime, "clip_duration", side_effect=lambda clip: durations[clip]), self.assertRaises(ValueError):
                self.runtime.propagate_cursor_intervals(paths, output)

class CursorSceneHelperTest(unittest.TestCase):
    def test_fused_cap_controls_keep_silhouette_owned_border_through_click_callback(self):
        from types import SimpleNamespace

        helper = importlib.import_module(MODULE + ".🎬️scene.🐍️")
        renderer = importlib.import_module(MODULE + ".🖌️render.🐍️")
        core = importlib.import_module(MODULE + ".🐍️")
        case = CASES["borderOwnership"]
        fused = renderer._button(case["rect"],{**core.resolve_visual(case["role"],{}),"stroke_width":case["fusedWidth"]},icon="maximize-2")
        self.assertEqual(fused._cursor_border_width,case["fusedWidth"])
        standalone = renderer._button(case["rect"],core.resolve_visual(case["role"],{}),icon="maximize-2")
        self.assertEqual(standalone._cursor_border_width,case["standaloneWidth"])
        for state in case["states"]:
            for target,width in ((fused,case["fusedWidth"]),(standalone,case["standaloneWidth"])):
                helper._draw_control(target,case["role"],**state,selected=False,disabled=False)
                self.assertEqual(target[0].get_stroke_width(),width)
        scene = SimpleNamespace(camera=SimpleNamespace(frame_width=14.2222222222,frame_height=8,frame_center=[0,0,0]),
                                renderer=SimpleNamespace(time=0),add=lambda *_:None,remove=lambda *_:None,
                                play=lambda *_,**__:None,wait=lambda *_:None)
        callbacks = []
        self.assertTrue(helper.cursor_click(scene,fused,lambda:callbacks.append(fused[0].get_stroke_width()),role=case["role"],
                                           move_time=0,hover_time=0,press_time=0,release_time=0))
        self.assertEqual(callbacks,[case["fusedWidth"]])
        self.assertEqual(fused[0].get_stroke_width(),case["fusedWidth"])

    def test_authored_cursor_uses_actual_viewport_scale_and_offset_camera_with_explicit_parent_frame(self):
        from types import SimpleNamespace
        from unittest.mock import patch
        from manim import Rectangle, VGroup

        helper = importlib.import_module(MODULE + ".🎬️scene.🐍️")
        presentation = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🐍️")
        target = Rectangle(width=1,height=0.5).move_to([40,-15,0])
        untouched = Rectangle(width=2,height=1,stroke_width=5).move_to([40,-15,0])
        frame = Rectangle(width=4,height=3).move_to([40,-15,0])
        parent = VGroup(untouched,frame)
        parent._cursor_frame = frame
        scene = SimpleNamespace(camera=SimpleNamespace(frame_width=14,frame_height=8,frame_center=[40,-15,0]),
                                renderer=SimpleNamespace(time=0),add=lambda *_:None,remove=lambda *_:None,
                                play=lambda *_,**__:None,wait=lambda *_:None)
        events, activated = [], []
        with patch.object(helper,"cursor_shape",wraps=helper.cursor_shape) as shape, patch.object(helper,"_record") as record:
            self.assertTrue(helper.cursor_click(scene,target,lambda:activated.append(True),parents=(parent,),on_event=events.append,
                                               move_time=0.1,hover_time=0.1,press_time=0.1,release_time=0.1))
        unit = 8 / presentation.presentation_contract()["focus"]["viewport"]["height"]
        self.assertTrue(shape.call_args_list)
        for call in shape.call_args_list:
            self.assertAlmostEqual(call.args[1],unit)
        self.assertEqual(activated,[True])
        self.assertEqual(events[0]["point"],[47.6,-15])
        self.assertEqual(events[-1]["point"],[47.6,-15])
        self.assertEqual(next(event["point"] for event in events if event["type"] == "click"),[40,-15])
        interval = record.call_args.args[2]
        entry = 0.1 * 0.6 / 7.6
        self.assertAlmostEqual(interval["start"],entry)
        self.assertAlmostEqual(interval["end"],0.5-entry)
        self.assertEqual(untouched.get_stroke_width(),5)

    def test_fullscreen_icon_respects_svg_viewport_instead_of_ink_bounds(self):
        import numpy as np
        import xml.etree.ElementTree as ET
        from svgelements import SVG, Path as SVGPath

        renderer = importlib.import_module(MODULE + ".🖌️render.🐍️")
        core = importlib.import_module(MODULE + ".🐍️")
        presentation = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🐍️")
        rect = {"x": 10, "y": 10, "width": 24, "height": 24}
        case = CASES["iconViewport"]
        for name in case["icons"]:
            with self.subTest(icon=name):
                source = ROOT / core.interaction_contract()["icons"][name]
                viewport = [float(value) for value in ET.parse(source).getroot().attrib["viewBox"].split()]
                left, top, right, bottom = SVG.parse(str(source)).bbox()
                self.assertEqual(viewport, case["viewBox"])
                self.assertEqual([left, top, right, bottom], case["inkBounds"])
                button = renderer._button(rect, core.resolve_visual("fullscreen", {}), icon=name)
                ink = [part for part in button[1].get_family()
                       if len(part.points) and part.get_stroke_width() > 0 and part.get_stroke_opacity() > 0]
                points = np.concatenate([part.points for part in ink])
                actual = np.ptp(points[:, :2], axis=0) / presentation._UNIT
                np.testing.assert_allclose(np.array([button[1].width, button[1].height]) / presentation._UNIT,
                                           np.array(viewport[2:]) / viewport[2] * case["cssSize"], rtol=0, atol=1e-7)
                np.testing.assert_allclose((np.min(points, axis=0) + np.max(points, axis=0)) / 2,
                                           button[0].get_center(), rtol=0, atol=1e-7)
                expected = np.array([right - left, bottom - top]) / np.array(viewport[2:]) * case["cssSize"]
                np.testing.assert_allclose(expected, case["expectedVisibleSize"], rtol=0, atol=1e-7)
                np.testing.assert_allclose(actual, expected, rtol=0, atol=1e-7)
                drawing = SVG.parse(str(source))
                source_paths = [part for part in drawing.elements() if isinstance(part, SVGPath)]
                self.assertEqual(len(ink), len(source_paths))
                scale = case["cssSize"] / viewport[2] * presentation._UNIT
                center = np.array([viewport[0] + viewport[2] / 2, viewport[1] + viewport[3] / 2])
                stroke = float(ET.parse(source).getroot().attrib["stroke-width"]) * case["cssSize"] / viewport[2]
                for actual_path, source_path in zip(ink, source_paths, strict=True):
                    source_points = np.array([[segment.end.x, segment.end.y] for segment in source_path if segment.end is not None])
                    target = (source_points - center) * np.array([1, -1]) * scale + button[0].get_center()[:2]
                    vertices = np.concatenate((actual_path.get_start_anchors(), actual_path.get_end_anchors()[-1:]))[:, :2]
                    np.testing.assert_allclose(vertices, target, rtol=0, atol=1e-7)
                    self.assertAlmostEqual(actual_path.get_stroke_width(), stroke)


    def test_native_controls_keep_transparency_and_relative_disabled_opacity(self):
        from manim import Line, Rectangle, VGroup

        core = importlib.import_module(MODULE + ".🐍️")
        renderer = importlib.import_module(MODULE + ".🖌️render.🐍️")
        helper = importlib.import_module(MODULE + ".🎬️scene.🐍️")
        rect = {"x": 10, "y": 10, "width": 24, "height": 24}
        for state in ({}, {"hovered": True}, {"pressed": True}, {"disabled": True}, {"disabled": True, "selected": True}):
            with self.subTest(renderer=state):
                style = core.resolve_visual("button", state)
                button = renderer._button(rect, style, icon="maximize-2")
                self.assertAlmostEqual(button[0].get_fill_opacity(), style["fill_opacity"] * style["opacity"])
                self.assertAlmostEqual(button[0].get_stroke_opacity(), style["opacity"])
                for path in button[1].get_family():
                    if len(path.points):
                        self.assertEqual(path.get_fill_opacity(), 0)
                        if path.get_stroke_width() > 0:
                            self.assertAlmostEqual(path.get_stroke_opacity(), style["opacity"])
        frame = Rectangle(width=1, height=0.5, fill_opacity=0)
        open_path = Line([0, 0, 0], [0.2, 0.2, 0], stroke_opacity=0.7, fill_opacity=0)
        translucent = Rectangle(width=0.2, height=0.2, fill_opacity=0.3, stroke_opacity=0.8)
        target = VGroup(frame, VGroup(open_path, translucent))
        for disabled in (False, True, True, False):
            with self.subTest(helper_disabled=disabled):
                helper._draw_control(target, "button", hovered=False, pressed=False, selected=False, disabled=disabled)
                alpha = 0.5 if disabled else 1
                self.assertEqual(frame.get_fill_opacity(), 0)
                self.assertEqual(open_path.get_fill_opacity(), 0)
                self.assertAlmostEqual(open_path.get_stroke_opacity(), 0.7 * alpha)
                self.assertAlmostEqual(translucent.get_fill_opacity(), 0.3 * alpha)
                self.assertAlmostEqual(translucent.get_stroke_opacity(), 0.8 * alpha)

    def test_scene_callback_follows_committed_click_and_disabled_targets_cancel(self):
        from manim import Rectangle, Scene, tempconfig

        helper = importlib.import_module(MODULE + ".🎬️scene.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        with tempconfig({"media_dir": str(output / "helper"), "text_dir": str(output / "helper" / "text")}):
            class SpyScene(Scene):
                def play(self, *animations, **kwargs):
                    for animation in animations:
                        if hasattr(animation, "build"):
                            animation = animation.build()
                        if hasattr(animation, "_setup_scene"):
                            animation._setup_scene(self)
                        animation.begin()
                        animation.interpolate(1)
                        animation.finish()
                    self.renderer.time += kwargs.get("run_time", max((getattr(animation, "run_time", 1) for animation in animations), default=0))

                def wait(self, *args, **kwargs):
                    self.renderer.time += kwargs.get("duration", args[0] if args else 1)

            for enabled in (True, False):
                with self.subTest(enabled=enabled):
                    events = []
                    times = {}
                    scene = SpyScene()

                    def record(event):
                        events.append(event["type"])
                        times.setdefault(event["type"], []).append(scene.renderer.time)
                    target = Rectangle(width=1, height=0.5)
                    parent = Rectangle(width=3, height=2)
                    scene.add(parent, target)
                    result = helper.cursor_click(scene, target, lambda: events.append("activated"),
                                                 parents=(parent,), enabled=enabled,
                                                 on_event=record,
                                                 move_time=0.01, hover_time=0.01, press_time=0.01, release_time=0.01)
                    self.assertEqual(result, enabled)
                    if enabled:
                        for kind in ("enter", "move", "press", "release", "click", "activated"):
                            self.assertIn(kind, events)
                        self.assertLess(events.index("press"), events.index("release"))
                        self.assertLess(events.index("release"), events.index("click"))
                        self.assertLess(events.index("click"), events.index("activated"))
                        self.assertEqual(events.count("activated"), 1)
                        self.assertAlmostEqual(times["release"][-1], times["click"][-1])
                    else:
                        self.assertNotIn("activated", events)


class CursorNativeFrameTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.presentation = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🐍️")
        cls.interaction = importlib.import_module(MODULE + ".🐍️")
        cls.output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "native-frames"
        cls.output.mkdir(parents=True, exist_ok=True)
        cls.settings = {"audience": "speaker", "quality": "-ql", "resolution": ",".join(map(str, CASES["native"]["resolution"])),
                        "fps": CASES["native"]["fps"], "language": CASES["native"]["language"]}

    def frame(self, name, t=0, *, phase="focus", point=None, pressed=None, menu_open=False):
        """🖼️ Capture a chapter frame with an explicit full navbar branch when its menu is open."""
        renderer = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🥞️hierarchy.🖌️render.🐍️")
        hierarchy = renderer.initial_hierarchy() if point is not None and phase == "focus" and t == 0 or menu_open else None
        if menu_open:
            hierarchy["opened_path"] = CASES["native"]["branch"][:3]
        return self.presentation.render_interaction_frame(CASES["native"]["chapter"], phase, t, self.output / (name + ".png"),
            self.settings, point=point, pressed=pressed, menu_open=menu_open, hierarchy=hierarchy)

    def crop(self, result, rect):
        from PIL import Image

        scale = [CASES["native"]["resolution"][0] / 1600, CASES["native"]["resolution"][1] / 900]
        bounds = (round(rect["x"] * scale[0]), round(rect["y"] * scale[1]),
                  round((rect["x"] + rect["width"]) * scale[0]), round((rect["y"] + rect["height"]) * scale[1]))
        return Image.open(result["path"]).convert("RGB").crop(bounds)

    def ancestors(self, node, nodes):
        lookup = {part["id"]: part for part in nodes}
        result = []
        while node:
            result.insert(0, node)
            node = lookup.get(node.get("parent"))
        return result

    def test_actual_popup_leaf_hover_preserves_sibling_ink_and_transparent_glass(self):
        from PIL import Image

        normal = self.frame("popup-paint-normal",point=CASES["idlePoint"],menu_open=True)
        rows = normal["browser"]["menu"]["treeRows"]
        self.assertGreaterEqual(len(rows),3)
        leaf_path, sibling_path = CASES["popupPaths"]["leaf"], CASES["popupPaths"]["sibling"]
        leaf = next(row for row in rows if row["id"] == leaf_path[-1])
        sibling = next(row for row in rows if row["id"] == sibling_path[-1])
        point = [leaf["rect"]["x"] + leaf["rect"]["width"]/2,leaf["rect"]["y"] + leaf["rect"]["height"]/2]
        rect = sibling["rect"]
        next_point = [rect["x"]+rect["width"]/2,rect["y"]+rect["height"]/2]
        renderer = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🥞️hierarchy.🖌️render.🐍️")
        requests = [{"video_id":CASES["native"]["chapter"],"phase":"focus","t":0,
                     "output":self.output/(name+".png"),"point":location,"menu_open":True,
                     "hierarchy":{**renderer.initial_hierarchy(),"opened_path":CASES["native"]["branch"][:3]},"transition_elapsed":1000}
                    for name,location in (("popup-paint-hover",point),("popup-paint-next",next_point))]
        hover,next_hover = renderer.frame_batch(requests,self.settings)
        painted = {row["id"]:row for row in hover["browser"]["menu"]["treeRows"]}
        golden = CASES["popup"]
        for row in rows:
            self.assertEqual(row["foreground"],golden["normalForeground"])
            self.assertEqual(row["row_background"],golden["rowBackground"])
            self.assertEqual(row["label_background"],golden["labelBackground"])
        for row in painted.values():
            expected = row["id"] in leaf_path
            self.assertEqual(row["foreground"], golden["emphasizedForeground"] if expected else golden["normalForeground"])
            self.assertEqual(row["hover_path"], "row" if expected else None)
        for row in painted.values():
            self.assertEqual(row["row_background"],golden["rowBackground"])
            self.assertEqual(row["label_background"],golden["labelBackground"])
        for result in (normal,hover):
            self.assertEqual(result["browser"]["menu"]["background"],golden["menuBackground"])
            self.assertEqual(result["browser"]["menu"]["inside_navbar"],golden["insideNavbar"])
        with Image.open(normal["path"]) as first,Image.open(hover["path"]) as second:
            scale = [first.width/1600,first.height/900]
            rect = sibling["rect"]
            box = (round((rect["x"]+rect["width"]*.75)*scale[0]),round((rect["y"]+4)*scale[1]),
                   round((rect["x"]+rect["width"]-6)*scale[0]),round((rect["y"]+rect["height"]-4)*scale[1]))
            self.assertNotEqual(second.convert("RGB").getpixel((box[0],box[1])),(96,105,102))
        next_rows = {row["id"]:row for row in next_hover["browser"]["menu"]["treeRows"]}
        for row in next_rows.values():
            expected = row["id"] in sibling_path
            self.assertEqual(row["foreground"], golden["emphasizedForeground"] if expected else golden["normalForeground"])
            self.assertEqual(row["hover_path"], "row" if expected else None)


    def test_layered_navigation_traverses_all_ancestors_and_preserves_adjacent_boundary_pixels(self):
        """🪜 Measure all four native grids and the actual shared adjacent-chapter endpoint."""
        from PIL import Image, ImageChops

        renderer = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🥞️hierarchy.🖌️render.🐍️")
        scene, branch = CASES["native"]["chapter"], CASES["native"]["branch"]
        duration = self.presentation.transition_duration("focus", scene)
        self.assertAlmostEqual(duration, CASES["native"]["phaseSeconds"]["focus"])
        requests = [{"video_id": scene, "phase": "focus", "t": seconds / duration, "output": self.output / (name + ".png")}
                    for name, seconds in CASES["native"]["layeredSampleSeconds"].items()]
        samples = dict(zip(CASES["native"]["layeredSampleSeconds"], renderer.frame_batch(requests, self.settings), strict=True))
        self.assertEqual(samples["overview"]["timeline"]["hierarchy"]["opened_path"], [])
        for depth, name in enumerate(("course", "topic", "clip", "chapter")):
            hover = samples[name + "Hover"]
            state = hover["timeline"]["hierarchy"]
            self.assertEqual(state["opened_path"], branch[:depth])
            self.assertEqual(state["revealed_path"], branch[:depth + 1])
            layer = next(layer for layer in hover["browser"]["layers"] if layer["id"] == (branch[depth - 1] if depth else "root"))
            pane = next(pane for pane in layer["panes"] if pane["id"] == branch[depth])
            self.assertTrue(pane["inert"])
            self.assertFalse(pane["opened"])
            for key in ("x", "y", "width", "height"):
                self.assertAlmostEqual(pane["rect"][key], layer["rect"][key], delta=.5)
            if depth < 3:
                self.assertEqual(samples[name + "Opened"]["timeline"]["hierarchy"]["opened_path"], branch[:depth + 1])
        self.assertEqual(samples["focused"]["timeline"]["hierarchy"]["opened_path"], branch)
        boundary = renderer.frame_batch([
            {"video_id": scene, "phase": "restore", "t": 1, "output": self.output / "restored-boundary.png"},
            {"video_id": CASES["native"]["nextChapter"], "phase": "focus", "t": 0, "output": self.output / "next-boundary.png"}], self.settings)
        for frame in boundary:
            self.assertEqual(frame["timeline"]["hierarchy"]["opened_path"], CASES["native"]["boundary"])
        with Image.open(boundary[0]["path"]) as first, Image.open(boundary[1]["path"]) as last:
            self.assertIsNone(ImageChops.difference(first.convert("RGB"), last.convert("RGB")).getbbox())

    def test_four_fps_transitions_capture_current_native_clicks_at_every_required_level(self):
        """🖱️ Verify exact native press/release/click events and the context-sensitive return path."""
        renderer = importlib.import_module(MODULE.rsplit(".🖱️interaction", 1)[0] + ".🥞️hierarchy.🖌️render.🐍️")
        scene = CASES["native"]["chapter"]
        for phase, expected in CASES["native"]["clickTargets"].items():
            with self.subTest(phase=phase):
                duration = self.presentation.transition_duration(phase, scene)
                self.assertAlmostEqual(duration, CASES["native"]["phaseSeconds"][phase])
                count = max(3, round(duration * 4))
                requests = [{"video_id": scene, "phase": phase, "t": index / (count - 1),
                    "output": self.output / f"clicks-{phase}-{index}.png"} for index in range(count)]
                result = renderer.frame_batch(requests, self.settings)
                clicks = [event["target"] for frame in result for event in frame["browser"]["pointer_events"] if event["type"] == "click"]
                self.assertEqual(clicks, expected)
                for target in expected:
                    events = [event["type"] for frame in result for event in frame["browser"]["pointer_events"] if event["target"] == target]
                    self.assertEqual(events, ["pointerdown", "pointerup", "click"])
                if phase == "restore":
                    final = result[-1]
                    self.assertEqual(final["timeline"]["hierarchy"]["opened_path"], CASES["native"]["boundary"])
                    self.assertEqual(final["timeline"]["hierarchy"]["revealed_path"], [])
                    self.assertEqual(final["timeline"]["point"], self.interaction.interaction_contract()["timeline"]["park"])
                    layer = next(layer for layer in final["browser"]["layers"] if layer["id"] == CASES["native"]["boundary"][-1])
                    self.assertFalse(any(pane["opened"] for pane in layer["panes"]))

    def test_native_navbar_footer_and_tree_owner_rows_emphasize_only_hit_ancestors(self):
        from PIL import ImageChops

        base = self.frame("outside", point=CASES["idlePoint"])
        for role in ("navbar", "footer"):
            with self.subTest(container=role):
                bar = next(node for node in base["nodes"] if node["role"] == role)
                candidates = [node for node in base["nodes"] if node["role"] in ("button", "toggle") and node.get("enabled", True)
                              and any(parent["id"] == bar["id"] for parent in self.ancestors(node, base["nodes"]))]
                self.assertTrue(candidates)
                node = candidates[0]
                rect = node.get("paint_rect", node["rect"])
                point = [rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2]
                active = self.frame(role + "-hover", point=point)
                self.assertIn(bar["id"], self.interaction.hit_path(point, active["nodes"]))
                self.assertEqual(active["styles"][bar["id"]]["stroke"], "#f7f3e3")
                self.assertIsNotNone(ImageChops.difference(self.crop(base, bar["rect"]), self.crop(active, bar["rect"])).getbbox())
        menu = self.frame("tree-normal", point=CASES["idlePoint"], menu_open=True)
        rows = menu["browser"]["menu"]["treeRows"]
        self.assertGreaterEqual(len(rows), 2)
        leaf = next(row for row in rows if "physical_fundamentals" in row["id"])
        rect = leaf["rect"]
        point = [rect["x"] + rect["width"] / 2, rect["y"] + rect["height"] / 2]
        active = self.frame("tree-hover", point=point, menu_open=True)
        painted = {row["id"]:row for row in active["browser"]["menu"]["treeRows"]}
        owners = [row for row in painted.values() if row["hover_path"] == "row"]
        self.assertGreaterEqual(len(owners), 2)
        for row in rows:
            style = painted[row["id"]]
            if style["hover_path"] == "row":
                self.assertEqual(style["foreground"], CASES["popup"]["emphasizedForeground"])
                if row["id"] != leaf["id"]:
                    bounds = row.get("paint_rect", row["rect"])
                    self.assertIsNotNone(ImageChops.difference(self.crop(menu, bounds), self.crop(active, bounds)).getbbox())
            else:
                self.assertEqual(style["foreground"], row["foreground"])
                self.assertEqual(style["row_background"], row["row_background"])


if __name__ == "__main__":
    unittest.main()
