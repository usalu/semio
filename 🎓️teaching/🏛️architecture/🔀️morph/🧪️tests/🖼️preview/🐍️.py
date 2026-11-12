"""🧪 Tests of the preview swap: Manim renders at a tiny resolution while a spy records the visible identities and the source placement of every sampled frame.

Oracles: JSON schedule cases (language-agnostic), `manim.utils.rate_functions` for easing, independent NumPy rectangle arithmetic and shapely.affinity for placement.
"""

from __future__ import annotations

import ast
import importlib
import json
import pathlib
import random
import unittest
from types import SimpleNamespace as NS

import numpy as np

manim = importlib.import_module("manim")
preview = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🖼️preview.🐍️")
placement = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📍️placement.🐍️")
shapely_geometry = importlib.import_module("shapely.geometry")
shapely_affinity = importlib.import_module("shapely.affinity")

CASES = json.loads((pathlib.Path(__file__).parent / "🧫️cases" / "🔣️.json").read_text(encoding="utf-8"))["cases"]
EASING = {"linear": manim.linear, "smooth": manim.smooth, "overshoot": manim.rate_functions.ease_out_back}
CONFIG = {"write_to_movie": False, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none", "save_last_frame": False, "pixel_width": 32, "pixel_height": 18}


def rect(cx, cy, w, h, color=manim.BLUE):
    """▭ A filled rectangle centred at (cx, cy)."""
    return manim.Rectangle(width=w, height=h, fill_color=color, fill_opacity=0.9).move_to([cx, cy, 0])


def box(mob):
    """📦 Measured (2, 3) bounds of a mobject."""
    return np.array([mob.get_corner(manim.DL), mob.get_corner(manim.UR)])


def subject(mob, parts=None, bounds=None, anchors=None, part_bounds=None):
    """🎯 A stage-subject stand-in with frozen logical bounds (default: measured now)."""
    frozen = box(mob) if bounds is None else np.array(bounds, dtype=float)
    measured = {name: box(part) for name, part in (parts or {}).items()} if part_bounds is None else part_bounds
    return NS(id="x", mobject=mob, parts=parts or {}, bounds=frozen, anchors=anchors or {}, part_bounds=measured, lod=None, revision=1)


def transition(duration=1.0, easing="smooth", relation="one-to-one", recipe="parts", kind="identity", pairs=None, lag=0.0, limit=None, space="scene", anchor="center", fit="none", uniform=True, source="snapshot", target="snapshot"):
    """🔀 A spec-transition stand-in."""
    return NS(id="t", copy=False, recipe=recipe, relation=relation, correspondence=NS(kind=kind, pairs=pairs or []), timing=NS(duration=duration, easing=easing, lag=lag, limit=limit), placement=NS(space=space, anchor=anchor, fit=fit, uniform=uniform), source=NS(sampling=source), target=NS(sampling=target))


def visible(scene, mob):
    """👁 Whether a mobject is drawn by the scene (member of some scene mobject's family)."""
    return any(mob is member for top in scene.mobjects for member in top.get_family())


def play(src, tgt, spec, fps, before=(), after=(), hook=None):
    """▶ Play the swap in a scene with `before` mobjects around the source; return (scene, frames) with one record per sampled frame."""
    frames = []
    with manim.tempconfig({**CONFIG, "frame_rate": fps}):
        scene = manim.Scene()
        scene.add(*before, src.mobject, *after)
        animation = preview.PreviewSwap(src, tgt, transition=spec, fps=fps)
        original = scene.update_to_time

        def spy(t):
            original(t)
            frames.append({"ids": {id(m) for top in scene.mobjects for m in top.get_family()}, "t": t, "source": visible(scene, src.mobject), "target": visible(scene, tgt.mobject), "box": box(src.mobject) if visible(scene, src.mobject) and len(src.mobject.get_all_points()) else None, "scene": scene})
            if hook:
                hook(scene, t)

        scene.update_to_time = spy
        scene.play(animation)
    return scene, frames


def owners(frames):
    """🔤 `S`/`T`/`?`/`!` per frame: source only, target only, neither, both."""
    return "".join("!" if f["source"] and f["target"] else "S" if f["source"] else "T" if f["target"] else "?" for f in frames)


class ScheduleCaseTests(unittest.TestCase):
    """📋 JSON schedule cases rendered by real Manim."""

    def test_cases(self):
        for case in CASES:
            with self.subTest(case["name"]):
                src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(2, 0, 1, 1, manim.RED))
                _, frames = play(src, tgt, transition(case["duration"], case["easing"]), case["fps"])
                n = len(frames)
                self.assertEqual(owners(frames), "S" * (n - 1) + "T")
                self.assertEqual(len(case["owners"]), n)
                self.assertEqual(owners(frames), case["owners"])

    def test_source_x_follows_declared_easing_and_terminal_p_is_one(self):
        for easing, rate in EASING.items():
            with self.subTest(easing):
                src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(4, 0, 1, 1, manim.RED))
                _, frames = play(src, tgt, transition(1.0, easing), 10)
                for k, f in enumerate(frames[:-1]):
                    self.assertAlmostEqual(f["box"][:, 0].mean(), -2 + 6 * rate(k / 9), places=9)


class PlacementTests(unittest.TestCase):
    """📍 Placement of the source against independent oracles."""

    def test_contain_asymmetric_bounds_match_shapely_every_frame(self):
        src = subject(rect(0, 0, 2, 1))
        tgt = subject(rect(6, 3, 8, 8, manim.RED))
        _, frames = play(src, tgt, transition(1.0, "linear", fit="contain"), 10)
        s = 4.0
        for k, f in enumerate(frames[:-1]):
            e = k / 9
            expected = shapely_affinity.affine_transform(shapely_geometry.box(-1, -0.5, 1, 0.5), [1 + (s - 1) * e, 0, 0, 1 + (s - 1) * e, 6 * e, 3 * e])
            np.testing.assert_allclose([*f["box"][0][:2], *f["box"][1][:2]], expected.bounds, atol=1e-9)

    def test_bottom_anchor_keeps_the_foot_line_on_the_target_foot_line(self):
        src = subject(rect(0, 1, 1, 2))
        tgt = subject(rect(5, 2, 2, 4, manim.RED))
        _, frames = play(src, tgt, transition(1.0, "linear", anchor="bottom", fit="contain"), 10)
        e = 8 / 9
        np.testing.assert_allclose(frames[8]["box"][0][1], 0 + (0 - 0) * e, atol=1e-9)
        np.testing.assert_allclose(frames[8]["box"][:, 0].mean(), 5 * e, atol=1e-9)

    def test_nested_entity_space_matches_the_placement_oracle(self):
        frames_map = {"e": placement.Frame((2.0, 1.0), (1.0, 0.0))}
        src, tgt = subject(rect(0, 0, 2, 1)), subject(rect(10, 0, 4, 2, manim.RED))
        spec = transition(1.0, "linear", space="entity:e", fit="contain")
        with manim.tempconfig({**CONFIG, "frame_rate": 10}):
            scene = manim.Scene()
            scene.add(src.mobject)
            animation = preview.PreviewSwap(src, tgt, transition=spec, fps=10, frames=frames_map)
            animation._setup_scene(scene)
            animation.begin()
            oracle = placement.affine(placement.Bounds.of(src.bounds), placement.Bounds.of(tgt.bounds), spec.placement, frames=frames_map)
            animation.interpolate(0.4)
            expected = placement.apply(placement.at(oracle, 4 / 9), [[-1, -0.5, 0], [1, 0.5, 0]])
            np.testing.assert_allclose(box(src.mobject)[:, :2], expected[:, :2], atol=1e-9)

    def test_zero_size_source_bounds_translate_only(self):
        dot = manim.Dot([3, 3, 0], radius=0.2)
        src = subject(dot, bounds=[[3, 3, 0], [3, 3, 0]])
        tgt = subject(rect(-1, 1, 4, 2, manim.RED))
        _, frames = play(src, tgt, transition(1.0, "linear", fit="contain"), 10)
        for k, f in enumerate(frames[:-1]):
            np.testing.assert_allclose(f["box"].mean(0)[:2], [3 + (-1 - 3) * k / 9, 3 + (1 - 3) * k / 9], atol=1e-9)
            self.assertAlmostEqual(f["box"][1][0] - f["box"][0][0], 0.4, places=9)

    def test_invalid_bounds_raise_actionable_errors(self):
        src = subject(rect(0, 0, 1, 1), bounds=[[0, 0, 0], [np.nan, 1, 0]])
        with self.assertRaises(placement.PlacementError):
            preview.PreviewSwap(src, subject(rect(2, 0, 1, 1)), transition=transition(), fps=10)

    def test_placement_uses_frozen_logical_bounds_not_target_geometry(self):
        src = subject(rect(0, 0, 2, 2), bounds=[[-1, -1, 0], [1, 1, 0]])
        proxy = rect(0, 0, 0.1, 0.1, manim.RED)
        tgt = subject(proxy, bounds=[[8, 8, 0], [12, 12, 0]])
        _, frames = play(src, tgt, transition(1.0, "linear", fit="contain"), 10)
        np.testing.assert_allclose(frames[7]["box"].mean(0)[:2], np.array([10, 10]) * 7 / 9, atol=1e-9)
        self.assertAlmostEqual(frames[7]["box"][1][0] - frames[7]["box"][0][0], 2 * (1 + (2 - 1) * 7 / 9), places=9)


class IdentityTests(unittest.TestCase):
    """👁 Atomic swap, scene hand-over and exclusivity."""

    def test_scene_holds_exactly_the_target_at_the_same_index_and_source_is_restored(self):
        a, b = rect(-4, 3, 1, 1, manim.GREEN), rect(4, 3, 1, 1, manim.GREEN)
        src, tgt = subject(manim.VGroup(rect(-1, 0, 1, 1), rect(1, 0, 1, 1))), subject(rect(2, -2, 2, 2, manim.RED))
        natural = [m.points.copy() for m in src.mobject.get_family() if len(m.points)]
        scene, frames = play(src, tgt, transition(0.5, "overshoot", fit="contain"), 10, before=[a], after=[b])
        self.assertEqual([m for m in scene.mobjects], [a, tgt.mobject, b])
        self.assertEqual(len(src.mobject.submobjects), 2)
        for expected, now in zip(natural, [m.points for m in src.mobject.get_family() if len(m.points)]):
            np.testing.assert_array_equal(expected, now)
        self.assertEqual(owners(frames), "SSSST")

    def test_no_early_target_no_overlap_no_blank_with_overshooting_ease(self):
        src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(2, 0, 1, 1, manim.RED))
        _, frames = play(src, tgt, transition(1.0, "overshoot"), 30)
        text = owners(frames)
        self.assertEqual(text.count("!"), 0)
        self.assertEqual(text.count("?"), 0)
        self.assertEqual(text.index("T"), len(text) - 1)

    def test_zero_duration_is_atomic_and_plays_no_frame(self):
        src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(2, 0, 1, 1, manim.RED))
        scene = manim.Scene()
        scene.add(src.mobject)
        animation = preview.PreviewSwap(src, tgt, transition=transition(0.0), fps=10)
        animation.swap_now(scene)
        self.assertEqual(list(scene.mobjects), [tgt.mobject])
        np.testing.assert_allclose(box(src.mobject)[:, :2].mean(0), [-2, 0])

    def test_target_is_the_same_bound_object_not_a_copy(self):
        src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(2, 0, 1, 1, manim.RED))
        before = tgt.mobject.points.copy()
        scene, _ = play(src, tgt, transition(0.5), 10)
        self.assertIs(scene.mobjects[0], tgt.mobject)
        np.testing.assert_array_equal(tgt.mobject.points, before)
        self.assertEqual(tgt.mobject.get_fill_color(), manim.ManimColor(manim.RED))


class SeekTests(unittest.TestCase):
    """🔀 Stateless evaluation: any order of `interpolate(alpha)` calls gives the same pose."""

    def pose(self, animation, alpha, src, tgt, scene):
        animation.interpolate(alpha)
        return visible(scene, tgt.mobject), None if visible(scene, tgt.mobject) else box(src.mobject).copy()

    def test_arbitrary_order_reverse_and_repeats_match_forward_evaluation(self):
        src, tgt = subject(manim.VGroup(rect(-1, 0, 1, 1), rect(1, 0.5, 1, 2))), subject(rect(5, 2, 3, 3, manim.RED))
        with manim.tempconfig({**CONFIG, "frame_rate": 10}):
            scene = manim.Scene()
            scene.add(src.mobject)
            animation = preview.PreviewSwap(src, tgt, transition=transition(1.0, "overshoot", fit="contain"), fps=10)
            animation._setup_scene(scene)
            animation.begin()
            alphas = [k / 10 for k in range(10)] + [1.0]
            forward = [self.pose(animation, a, src, tgt, scene) for a in alphas]
            order = list(range(len(alphas)))
            random.Random(3).shuffle(order)
            for i in order + order[::-1] + [10, 0, 10, 5, 5]:
                shown, bounds = self.pose(animation, alphas[i], src, tgt, scene)
                self.assertEqual(shown, forward[i][0])
                if bounds is not None:
                    np.testing.assert_array_equal(bounds, forward[i][1])
            self.assertEqual([f[0] for f in forward], [False] * 9 + [True, True])

    def test_seeking_back_from_target_restores_the_source_only_state(self):
        src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(2, 0, 1, 1, manim.RED))
        with manim.tempconfig({**CONFIG, "frame_rate": 10}):
            scene = manim.Scene()
            scene.add(src.mobject)
            animation = preview.PreviewSwap(src, tgt, transition=transition(1.0), fps=10)
            animation._setup_scene(scene)
            animation.begin()
            animation.interpolate(1.0)
            self.assertTrue(visible(scene, tgt.mobject) and not visible(scene, src.mobject))
            animation.interpolate(0.0)
            self.assertTrue(visible(scene, src.mobject) and not visible(scene, tgt.mobject))
            np.testing.assert_allclose(box(src.mobject)[:, :2].mean(0), [-2, 0], atol=1e-12)


class SamplingTests(unittest.TestCase):
    """🧊 Live updaters run on the natural pose, snapshot suspends them for the window."""

    def drift(self, source_sampling):
        src, tgt = subject(rect(0, 0, 1, 1)), subject(rect(6, 0, 1, 1, manim.RED))
        src.mobject.add_updater(lambda m, dt: m.shift(manim.UP * dt))
        _, frames = play(src, tgt, transition(1.0, "linear", source=source_sampling), 10)
        return src, frames

    def test_live_source_keeps_its_own_animation_and_is_placed_on_top(self):
        src, frames = self.drift("live")
        for k, f in enumerate(frames[:-1]):
            np.testing.assert_allclose(f["box"].mean(0)[:2], [6 * k / 9, k / 10], atol=1e-9)
        self.assertFalse(src.mobject.updating_suspended)

    def test_snapshot_source_ignores_updaters_during_the_window(self):
        src, frames = self.drift("snapshot")
        for k, f in enumerate(frames[:-1]):
            np.testing.assert_allclose(f["box"].mean(0)[:2], [6 * k / 9, 0], atol=1e-9)
        self.assertFalse(src.mobject.updating_suspended)
        np.testing.assert_allclose(box(src.mobject).mean(0)[:2], [0, 0], atol=1e-9)

    def test_snapshot_target_is_restored_to_its_captured_pose(self):
        src, tgt = subject(rect(0, 0, 1, 1)), subject(rect(6, 0, 1, 1, manim.RED))
        pose = tgt.mobject.points.copy()
        scene, _ = play(src, tgt, transition(0.5, target="snapshot"), 10, hook=lambda s, t: None if visible(s, tgt.mobject) else tgt.mobject.shift(manim.UP * 5))
        np.testing.assert_array_equal(tgt.mobject.points, pose)


class LaneTests(unittest.TestCase):
    """🛤 One-to-many, many-to-one and per-element n:n windows."""

    def parts(self, xs, y):
        return {f"p{i}": rect(x, y, 1, 1, manim.RED if y > 0 else manim.BLUE) for i, x in enumerate(xs)}

    def test_many_to_one_moves_every_source_and_target_appears_at_terminal(self):
        sp = self.parts([-3, -1, 1], -2)
        src = subject(manim.VGroup(*sp.values()), parts=sp)
        tgt = subject(rect(4, 3, 2, 2, manim.RED))
        _, frames = play(src, tgt, transition(1.0, "linear", relation="many-to-one", kind="recipe", fit="contain"), 10)
        s = 2 / 5.0 * 1.0
        for k, f in enumerate(frames[:-1]):
            self.assertFalse(f["target"])
            e = k / 9
            scale = 1 + (min(2 / 5, 2 / 1) - 1) * e
            np.testing.assert_allclose(f["box"].mean(0)[:2], np.array([-1, -2]) + np.array([5, 5]) * e, atol=1e-9)
            self.assertAlmostEqual(f["box"][1][0] - f["box"][0][0], 5 * scale, places=9)
        self.assertTrue(frames[-1]["target"] and not frames[-1]["source"])

    def test_one_to_many_moves_toward_the_union_bounds_of_the_targets(self):
        tp = self.parts([5, 7], 3)
        src = subject(rect(0, 0, 1, 1))
        tgt = subject(manim.VGroup(*tp.values()), parts=tp)
        self.assertEqual(tuple(tgt.bounds[0]), (4.5, 2.5, 0.0))
        _, frames = play(src, tgt, transition(1.0, "linear", relation="one-to-many", kind="recipe"), 10)
        for k, f in enumerate(frames[:-1]):
            np.testing.assert_allclose(f["box"].mean(0)[:2], np.array([6, 3]) * k / 9, atol=1e-9)
            self.assertFalse(f["target"])
        self.assertTrue(all(visible(frames[-1]["scene"], p) for p in tp.values()))

    def test_n_to_n_each_element_swaps_in_its_own_window(self):
        sp, tp = self.parts([-3, 0, 3], -2), self.parts([-3, 0, 3], 2)
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(manim.VGroup(*tp.values()), parts=tp)
        spec = transition(0.5, "linear", relation="many-to-many", kind="key", lag=0.1)
        _, frames = play(src, tgt, spec, 10)
        self.assertEqual(len(frames), 7)
        rows = [("".join("T" if id(tp[f"p{i}"]) in f["ids"] else "S" if id(sp[f"p{i}"]) in f["ids"] else "?" for i in range(3))) for f in frames]
        self.assertEqual(rows, ["SSS", "SSS", "SSS", "SSS", "TSS", "TTS", "TTT"])
        for f in frames[:-1]:
            self.assertNotIn(id(tgt.mobject), f["ids"])
        self.assertIn(id(tgt.mobject), frames[-1]["ids"])
        self.assertEqual(len(src.mobject.submobjects), 3)

    def test_n_to_n_element_placement_uses_per_part_bounds_and_window_progress(self):
        sp, tp = self.parts([-3, 0], -2), self.parts([-3, 0], 2)
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(manim.VGroup(*tp.values()), parts=tp)
        spec = transition(0.5, "linear", relation="many-to-many", kind="order", lag=0.1)
        _, frames = play(src, tgt, spec, 10)
        centre = lambda mob: box(mob)[:, :2].mean(0)
        with manim.tempconfig({**CONFIG, "frame_rate": 10}):
            scene = manim.Scene()
            scene.add(src.mobject)
            animation = preview.PreviewSwap(src, tgt, transition=spec, fps=10)
            animation._setup_scene(scene)
            animation.begin()
            animation.interpolate(1 / 6)
            np.testing.assert_allclose(centre(sp["p0"]), [-3, -2 + 4 * 1 / 4], atol=1e-9)
            np.testing.assert_allclose(centre(sp["p1"]), [0, -2], atol=1e-9)

    def test_recipe_correspondence_pairs_by_order_only_when_the_counts_match(self):
        sp, tp = self.parts([-3, 0], -2), self.parts([-3, 0], 2)
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(manim.VGroup(*tp.values()), parts=tp)
        animation = preview.PreviewSwap(src, tgt, transition=transition(0.5, relation="many-to-many", kind="recipe", lag=0.1), fps=10)
        self.assertEqual(len(animation.lanes), 2)
        self.assertAlmostEqual(animation.run_time, 0.6)
        extra = self.parts([-3, 0, 3], 2)
        wide = subject(manim.VGroup(*extra.values()), parts=extra)
        animation = preview.PreviewSwap(src, wide, transition=transition(0.5, relation="many-to-many", kind="recipe", lag=0.1), fps=10)
        self.assertEqual(len(animation.lanes), 1)
        self.assertEqual(animation.run_time, 0.5)

    def test_whole_recipes_never_pair_parts_and_never_run_full_morph_code(self):
        sp, tp = self.parts([-3, 0], -2), self.parts([-3, 0], 2)
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(manim.VGroup(*tp.values()), parts=tp)
        animation = preview.PreviewSwap(src, tgt, transition=transition(0.5, relation="many-to-many", recipe="retext", kind="recipe"), fps=10)
        self.assertEqual(len(animation.lanes), 1)

    def test_unknown_pair_names_are_actionable(self):
        sp, tp = self.parts([-3, 0], -2), self.parts([-3, 0], 2)
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(manim.VGroup(*tp.values()), parts=tp)
        with self.assertRaises(placement.PlacementError) as caught:
            preview.PreviewSwap(src, tgt, transition=transition(0.5, relation="many-to-many", kind="pairs", pairs=[["p0", "nope"]]), fps=10)
        self.assertIn("nope", str(caught.exception))


class LifecycleAndLodTests(unittest.TestCase):
    """♻ Idempotent restoration and LoD-invariant placement."""

    def test_release_is_idempotent_and_runs_from_clean_up_and_on_errors(self):
        sp = {f"p{i}": rect(i * 2, 0, 1, 1) for i in range(3)}
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(rect(5, 5, 1, 1, manim.RED))
        natural = [m.points.copy() for m in src.mobject.get_family() if len(m.points)]
        scene = manim.Scene()
        scene.add(src.mobject)
        animation = preview.PreviewSwap(src, tgt, transition=transition(1.0, fit="contain"), fps=10)
        animation._setup_scene(scene)
        animation.begin()
        animation.interpolate(0.5)
        animation.clean_up_from_scene(scene)
        animation.release()
        animation.release()
        self.assertEqual(len(src.mobject.submobjects), 3)
        for before, now in zip(natural, [m.points for m in src.mobject.get_family() if len(m.points)]):
            np.testing.assert_array_equal(before, now)
        failing = preview.PreviewSwap(src, tgt, transition=transition(1.0, fit="contain"), fps=10)
        failing.begin()
        failing.lanes[0].affine = None
        with self.assertRaises(Exception):
            failing.interpolate(0.5)
        self.assertEqual(len(src.mobject.submobjects), 3)
        self.assertFalse(src.mobject.updating_suspended)

    def test_root_keeps_its_children_after_a_partial_swap_and_seek_to_target(self):
        sp, tp = {f"p{i}": rect(i * 2, 0, 1, 1) for i in range(3)}, {f"p{i}": rect(i * 2, 4, 1, 1, manim.RED) for i in range(3)}
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(manim.VGroup(*tp.values()), parts=tp)
        scene = manim.Scene()
        scene.add(src.mobject)
        animation = preview.PreviewSwap(src, tgt, transition=transition(0.5, relation="many-to-many", kind="key", lag=0.1), fps=10)
        animation._setup_scene(scene)
        animation.begin()
        animation.interpolate(4 / 7)
        self.assertEqual(len(src.mobject.submobjects), 2)
        animation.interpolate(1.0)
        self.assertEqual(len(src.mobject.submobjects), 3)

    def test_same_transition_at_preview_and_full_lod_yields_identical_affine(self):
        full_parts = {f"p{i}": rect(i * 2, 0, 1, 1) for i in range(2)}
        proxy_parts = {f"p{i}": manim.Dot([i * 2, 0, 0], radius=0.05) for i in range(2)}
        frozen = {f"p{i}": [[i * 2 - 0.5, -0.5, 0], [i * 2 + 0.5, 0.5, 0]] for i in range(2)}
        target_full = {f"p{i}": rect(i * 2, 4, 1, 1, manim.RED) for i in range(2)}
        target_proxy = {f"p{i}": manim.Dot([i * 2, 4, 0], radius=0.05) for i in range(2)}
        target_frozen = {f"p{i}": [[i * 2 - 0.5, 3.5, 0], [i * 2 + 0.5, 4.5, 0]] for i in range(2)}
        whole_s, whole_t = [[-0.5, -0.5, 0], [2.5, 0.5, 0]], [[-0.5, 3.5, 0], [2.5, 4.5, 0]]
        spec = transition(0.5, relation="many-to-many", kind="order", lag=0.1, fit="contain")
        lanes = []
        for sp, tp in ((full_parts, target_full), (proxy_parts, target_proxy)):
            src = subject(manim.VGroup(*sp.values()), parts=sp, bounds=whole_s, part_bounds=frozen)
            tgt = subject(manim.VGroup(*tp.values()), parts=tp, bounds=whole_t, part_bounds=target_frozen)
            lanes.append([lane.affine for lane in preview.PreviewSwap(src, tgt, transition=spec, fps=10).lanes])
        self.assertEqual(lanes[0], lanes[1])

    def test_missing_frozen_part_bounds_is_an_error_not_a_measurement(self):
        sp, tp = {f"p{i}": rect(i * 2, 0, 1, 1) for i in range(2)}, {f"p{i}": rect(i * 2, 4, 1, 1, manim.RED) for i in range(2)}
        src, tgt = subject(manim.VGroup(*sp.values()), parts=sp), subject(manim.VGroup(*tp.values()), parts=tp)
        src.part_bounds = {}
        with self.assertRaises(placement.PlacementError) as caught:
            preview.PreviewSwap(src, tgt, transition=transition(0.5, relation="many-to-many", kind="order"), fps=10)
        self.assertIn("part_bounds", str(caught.exception))


class MovingListTests(unittest.TestCase):
    """🖌 The Cairo renderer draws the flat moving list: it must follow the stage state on every call, in any order, and exported pixels must be deterministic (FD-005)."""

    def moving_ids(self, scene):
        return {id(m) for m in scene.moving_mobjects}

    def test_source_is_in_the_moving_list_after_a_premature_target_evaluation(self):
        src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(2, 0, 1, 1, manim.RED))
        with manim.tempconfig({**CONFIG, "frame_rate": 10}):
            scene = manim.Scene()
            scene.add(src.mobject)
            animation = preview.PreviewSwap(src, tgt, transition=transition(1.0), fps=10)
            animation._setup_scene(scene)
            animation.begin()
            animation.interpolate(1.0)
            scene.moving_mobjects = [m for top in scene.mobjects for m in top.get_family()]
            for alpha in (0.0, 0.5, 1.0, 0.0, 0.3, 0.9, 1.0, 0.2):
                animation.interpolate(alpha)
                moving = self.moving_ids(scene)
                members = {id(m) for m in src.mobject.get_family() if len(m.points)}
                targets = {id(m) for m in tgt.mobject.get_family() if len(m.points)}
                if alpha < 0.9:
                    self.assertTrue(members <= moving and not targets & moving, alpha)
                else:
                    self.assertTrue(targets <= moving and not members & moving, alpha)

    def render_pixels(self, fps=10, run_time=None, wait=0.0):
        frames = []
        src, tgt = subject(rect(-2, 0, 2, 2, manim.BLUE)), subject(rect(2, 0, 2, 2, manim.RED))
        with manim.tempconfig({**CONFIG, "frame_rate": fps, "pixel_width": 160, "pixel_height": 90}):
            scene = manim.Scene()
            scene.add(src.mobject)
            if wait:
                scene.wait(wait)
            original = scene.renderer.render

            def render(*args, **kwargs):
                original(*args, **kwargs)
                frames.append(np.array(scene.renderer.camera.pixel_array))

            scene.renderer.render = render
            frames.clear()
            kwargs = {} if run_time is None else {"run_time": run_time}
            scene.play(preview.PreviewSwap(src, tgt, transition=transition(1.0, "smooth"), fps=fps), **kwargs)
        return frames

    @staticmethod
    def colours(frame):
        red, blue = frame[..., 0].astype(int), frame[..., 2].astype(int)
        return int(((red > blue + 80) & (red > 120)).sum()), int(((blue > red + 80) & (blue > 120)).sum())

    def test_exported_pixels_show_the_source_until_f1_and_the_target_at_f1(self):
        for wait in (0.0, 0.25, 0.23):
            with self.subTest(wait=wait):
                frames = self.render_pixels(wait=wait)
                self.assertEqual(len(frames), 10)
                for k, frame in enumerate(frames):
                    red, blue = self.colours(frame)
                    if k < 9:
                        self.assertEqual((red == 0, blue > 0), (True, True), (k, red, blue))
                    else:
                        self.assertEqual((red > 0, blue == 0), (True, True), (k, red, blue))

    def test_repeated_renders_are_frame_identical_including_run_time_overrides(self):
        for kwargs in ({}, {"run_time": 1.5}, {"wait": 0.23}):
            with self.subTest(**kwargs):
                runs = [self.render_pixels(**kwargs) for _ in range(4)]
                for other in runs[1:]:
                    self.assertEqual(len(other), len(runs[0]))
                    for a, b in zip(runs[0], other):
                        np.testing.assert_array_equal(a, b)
                red, blue = self.colours(runs[0][-1])
                self.assertTrue(red > 0 and blue == 0)
                self.assertTrue(all(self.colours(f)[0] == 0 for f in runs[0][:-1]))

    def test_run_time_override_stretches_the_swap_uniformly(self):
        frames = self.render_pixels(run_time=2.0)
        self.assertEqual(len(frames), 20)
        reds = [self.colours(f)[0] > 0 for f in frames]
        self.assertEqual(reds, [False] * 19 + [True])


class NoFullMorphTests(unittest.TestCase):
    """🚫 The preview module never reaches for full-morph machinery."""

    BANNED = {"silhouette", "_plan", "PartMorph", "Flight", "_hull", "Morph", "Swarm", "Transform", "ReplacementTransform", "FadeIn", "FadeOut", "capsule_pairs", "scatter", "gather", "unfold", "retext", "proxy"}

    def test_module_names_and_attributes_exclude_full_morph_helpers(self):
        path = pathlib.Path(preview.__file__)
        names = set()
        for node in ast.walk(ast.parse(path.read_text(encoding="utf-8"))):
            if isinstance(node, ast.Name):
                names.add(node.id)
            elif isinstance(node, ast.Attribute):
                names.add(node.attr)
            elif isinstance(node, ast.Constant) and isinstance(node.value, str) and "morph.🐍️" in node.value:
                names.add(node.value)
        self.assertEqual(names & self.BANNED, set())
        self.assertFalse([n for n in names if "morph.🐍️" in n])

    def test_play_leaves_no_crossfade_or_opacity_blend(self):
        src, tgt = subject(rect(-2, 0, 1, 1)), subject(rect(2, 0, 1, 1, manim.RED))
        opacity = src.mobject.get_fill_opacity()
        color = src.mobject.get_fill_color()
        _, frames = play(src, tgt, transition(1.0), 10, hook=lambda s, t: None)
        self.assertEqual(src.mobject.get_fill_opacity(), opacity)
        self.assertEqual(src.mobject.get_fill_color(), color)
        self.assertEqual(tgt.mobject.get_fill_opacity(), 0.9)
        self.assertEqual(len(frames), 10)


if __name__ == "__main__":
    unittest.main()
