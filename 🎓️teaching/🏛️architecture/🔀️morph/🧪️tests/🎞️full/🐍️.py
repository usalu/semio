"""🧪 Tests of the full morph toolkit: every helper ends with exactly the target in the scene, carries colours and stays fast for 140 sources."""

from __future__ import annotations

import importlib
import time
import unittest

import numpy as np

manim = importlib.import_module("manim")
morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️")
tokens = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🪙️token.🐍️")


def play(*start, animation):
    """▶ Play one animation in a tiny skipped scene that already holds `start`; return the scene."""
    with manim.tempconfig({"write_to_movie": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR", "save_last_frame": False, "progress_bar": "none"}):
        scene = manim.Scene(skip_animations=True)
        scene.add(*start)
        scene.play(animation)
    return scene


def capsule(x=0.0, color=manim.BLUE):
    """💊 A detailed multi-face stand-in for a capsule."""
    box = manim.RoundedRectangle(width=1.0, height=0.5, corner_radius=0.2, fill_color=color, fill_opacity=0.8, stroke_color=manim.WHITE)
    window = manim.Square(0.2, fill_color=manim.YELLOW, fill_opacity=0.6).shift(0.25 * manim.LEFT)
    door = manim.Rectangle(width=0.1, height=0.3, fill_color=manim.GREY, fill_opacity=0.9).shift(0.3 * manim.RIGHT)
    return manim.VGroup(box, window, door).shift(x * manim.RIGHT)


def fill_of(mob):
    """🎨 Fill colour of a mobject as RGB."""
    return np.array(manim.ManimColor(mob.get_fill_color()).to_rgb())


class SilhouetteTests(unittest.TestCase):
    """🫥 Outlines and proxies."""

    def test_group_becomes_single_shape_covering_the_group(self):
        group = capsule()
        shape = morph.silhouette(group)
        self.assertIsInstance(shape, manim.VMobject)
        self.assertEqual(len(shape.submobjects), 0)
        np.testing.assert_allclose([shape.width, shape.height], [group.width, group.height], atol=0.03)
        np.testing.assert_allclose(shape.get_center(), group.get_center(), atol=0.03)

    def test_hull_matches_scipy(self):
        spatial = importlib.import_module("scipy.spatial")
        points = np.random.default_rng(7).normal(size=(60, 3)) * [1, 1, 0]
        ours = {tuple(np.round(p, 2)) for p in morph._hull(points)}
        theirs = {tuple(np.round(points[i][:2], 2)) for i in spatial.ConvexHull(points[:, :2]).vertices}
        self.assertEqual(ours, theirs)

    def test_single_shape_is_copied_and_colour_override_applies(self):
        circle = manim.Circle(fill_color=manim.RED, fill_opacity=1)
        copy = morph.silhouette(circle, color=manim.GREEN)
        self.assertIsNot(copy, circle)
        np.testing.assert_allclose(fill_of(copy), np.array(manim.ManimColor(manim.GREEN).to_rgb()), atol=0.01)

    def test_mean_fill_between_parts(self):
        shape = morph.silhouette(capsule(color=manim.BLUE))
        self.assertEqual(shape.get_fill_opacity(), 0.9)

    def test_proxy_is_bounding_rectangle_with_group_colour(self):
        group = capsule(2.0)
        stand_in = morph.proxy(group)
        np.testing.assert_allclose([stand_in.width, stand_in.height], [group.width, group.height], atol=1e-6)
        np.testing.assert_allclose(stand_in.get_center(), group.get_center(), atol=1e-6)


class MorphTests(unittest.TestCase):
    """🔀 Single morphs."""

    def test_group_to_token_leaves_exactly_the_target(self):
        src, dst = capsule(), tokens.token("3", kind="item").shift(3 * manim.RIGHT)
        scene = play(src, animation=morph.morph(src, dst))
        self.assertEqual(scene.mobjects, [dst])
        np.testing.assert_allclose(dst.get_center(), [3, 0, 0], atol=1e-6)

    def test_target_back_to_group_roundtrip(self):
        src, dst = capsule(), tokens.token("{0;1}", kind="path").shift(2 * manim.UP)
        scene = play(src, animation=morph.morph(src, dst))
        back = capsule(-2.0)
        scene = play(dst, animation=morph.morph(dst, back))
        self.assertEqual(scene.mobjects, [back])

    def test_start_carries_source_colour(self):
        src = capsule(color=manim.RED)
        anim = morph.morph(src, tokens.token("1", kind="number", color=manim.GREEN), mode="hull")
        np.testing.assert_allclose(fill_of(anim.mobject), fill_of(morph.silhouette(src)), atol=1e-6)

    def test_colour_interpolates_halfway(self):
        src = manim.Square(fill_color=manim.RED, fill_opacity=1)
        dst = manim.Circle(fill_color=manim.BLUE, fill_opacity=1)
        anim = morph.morph(src, dst)
        scene = manim.Scene(skip_animations=True)
        scene.add(src)
        anim._setup_scene(scene)
        anim.begin()
        anim.interpolate(0.5)
        red, blue = (np.array(manim.ManimColor(c).to_rgb()) for c in (manim.RED, manim.BLUE))
        np.testing.assert_allclose(fill_of(anim.mobject), (red + blue) / 2, atol=0.12)

    def test_ghost_stays_at_requested_opacity(self):
        src, dst = capsule(), tokens.token("7", kind="cell").shift(3 * manim.RIGHT)
        anim = morph.morph(src, dst, ghost=0.25)
        scene = play(src, animation=anim)
        self.assertIn(anim.ghost_mobject, scene.mobjects)
        self.assertEqual(len(scene.mobjects), 2)
        np.testing.assert_allclose(anim.ghost_mobject[0].get_fill_opacity(), 0.8 * 0.25, atol=1e-6)
        np.testing.assert_allclose(anim.ghost_mobject[0].get_stroke_opacity(), 0.25, atol=1e-6)

    def test_keep_source_leaves_source_untouched(self):
        src, dst = capsule(), tokens.token("7", kind="cell").shift(3 * manim.RIGHT)
        scene = play(src, animation=morph.morph(src, dst, keep_source=True))
        self.assertEqual(set(scene.mobjects), {src, dst})

    def test_point_counts_are_robust(self):
        sources = [capsule(), manim.Circle(), manim.Triangle(), manim.Dot(), manim.VGroup(*[manim.Square(0.1).shift(i * manim.RIGHT * 0.2) for i in range(20)]), manim.Text("A1")]
        targets = [tokens.token(label, kind=kind) for label, kind in (("3", "item"), ("B2", "cell"), ("{0;1;3;1}", "path"), ("12", "number"), ("", "plane"), ("abc", "text"))]
        for src in sources:
            for dst in targets:
                dst = dst.copy().shift(2 * manim.RIGHT)
                scene = play(src, animation=morph.morph(src, dst, path_arc=0.5))
                self.assertEqual(scene.mobjects, [dst])
                src = src.copy()

    def test_pairs_leave_exactly_the_target_and_carry_features(self):
        src, dst = capsule(), tokens.token("3", kind="item", color=manim.RED, size=1.6).shift(3 * manim.RIGHT)
        anim = morph.morph(src, dst, pairs=[(src[0], dst[0]), (src[1], dst[1])], dip=0.15)
        scene = manim.Scene(skip_animations=True)
        scene.add(src)
        anim._setup_scene(scene)
        anim.begin()
        anim.interpolate(0.5)
        flying = [f.mobject for f in anim.flights]
        self.assertEqual(len(flying), 2)
        self.assertTrue(all(m.get_fill_opacity() > 0 or m.get_stroke_opacity() > 0 for m in flying))
        self.assertGreater(flying[0].get_center()[0], 0.5)
        self.assertLess(flying[0].get_center()[0], 2.5)
        anim.interpolate(1.0)
        anim.finish()
        anim.clean_up_from_scene(scene)
        self.assertEqual(scene.mobjects, [dst])

    def test_pairs_ghost_stays_and_unpaired_source_fades(self):
        src, dst = capsule(), tokens.token("3", kind="item").shift(3 * manim.RIGHT)
        anim = morph.morph(src, dst, pairs=[(src[1], dst[1])], ghost=0.3)
        scene = play(src, animation=anim)
        self.assertEqual(set(scene.mobjects), {dst, anim.ghost_mobject})
        np.testing.assert_allclose(anim.ghost_mobject[0].get_stroke_opacity(), 0.3, atol=1e-6)
        anim = morph.morph(capsule(), tokens.token("4", kind="cell"), pairs=[])
        self.assertIsInstance(anim, morph.PartMorph)

    def test_empty_and_degenerate_parts_never_crash(self):
        def holey():
            group = capsule()
            group.add(manim.VMobject(), manim.VGroup(), manim.VGroup(manim.VMobject()))
            return group

        self.assertEqual(len(morph.silhouette(manim.VGroup()).submobjects), 0)
        self.assertLessEqual(len(morph.silhouette(manim.VMobject()).points), 4)
        self.assertLessEqual(len(morph.proxy(manim.VGroup(manim.VMobject())).points), 4)
        np.testing.assert_allclose([morph.silhouette(holey()).width, morph.proxy(holey()).width], [1.0, 1.0], atol=0.05)
        dst = tokens.token("3", kind="item").shift(3 * manim.RIGHT)
        src = holey()
        pairs = [(src[3], dst[0]), (src[0], src[4]), (src[1], dst[1]), (src[5], dst[1])]
        scene = play(src, animation=morph.morph(src, dst, pairs=pairs))
        self.assertEqual(scene.mobjects, [dst])
        empty, dst = manim.VGroup(), tokens.token("4", kind="cell")
        self.assertEqual(play(empty, animation=morph.morph(empty, dst)).mobjects, [dst])
        src, hollow = capsule(), manim.VGroup(manim.VMobject())
        self.assertEqual(play(src, animation=morph.morph(src, hollow)).mobjects, [hollow])
        srcs = [holey() for _ in range(3)]
        dsts = [tokens.token(str(i), kind="item").shift(3 * manim.RIGHT) for i in range(3)]
        scene = play(*srcs, animation=morph.scatter(srcs, dsts, pairs_fn=lambda s, d: [(s[3], d[0]), (s[0], d[1])]))
        self.assertEqual(scene.mobjects, dsts)

    def test_capsule_pairs_pick_shell_and_window(self):
        src = capsule()
        src.add(manim.VMobject())
        tok = tokens.token("3", kind="item")
        pairs = morph.capsule_pairs(src, tok)
        self.assertEqual(pairs, [(src[0], tok[0]), (src[1], tok[1])])
        src[2].name = "window"
        self.assertEqual(morph.capsule_pairs(src, tok)[1][0], src[2])
        self.assertEqual(morph.capsule_pairs(manim.VGroup(manim.VMobject()), tok), [])
        nested = manim.VGroup(capsule())
        self.assertEqual(len(morph.capsule_pairs(nested, tok)), 2)
        scene = play(src, animation=morph.morph(src, tok.shift(2 * manim.RIGHT), pairs=morph.capsule_pairs(src, tok)))
        self.assertEqual(scene.mobjects, [tok])

    def test_dip_shrinks_mid_flight_only(self):
        src, dst = manim.Square(1.0, fill_opacity=1), manim.Square(1.0, fill_opacity=1).shift(8 * manim.RIGHT)
        anim = morph.morph(src, dst, dip=0.2)
        scene = manim.Scene(skip_animations=True)
        scene.add(src)
        anim._setup_scene(scene)
        anim.begin()
        anim.interpolate(0.5)
        self.assertLess(anim.mobject.width, 0.85)
        anim.interpolate(1.0)
        self.assertAlmostEqual(anim.mobject.width, 1.0, places=3)

    def test_default_never_builds_a_hull_and_real_shapes_travel_at_midpoint(self):
        hulls = []
        original = morph.silhouette

        def spy(mob, **kwargs):
            if len(morph._members(mob)) > 1:
                hulls.append(mob)
            return original(mob, **kwargs)

        for dst in (tokens.token("3", kind="item"), tokens.token("B2", kind="cell"), tokens.token("{0;1;3;1}", kind="path"), tokens.token("7", kind="plane")):
            src = capsule()
            morph.silhouette = spy
            try:
                anim = morph.morph(src, dst.shift(3 * manim.RIGHT), path_arc=0.4)
            finally:
                morph.silhouette = original
            self.assertEqual(hulls, [])
            self.assertIsInstance(anim, morph.PartMorph)
            scene = manim.Scene(skip_animations=True)
            scene.add(src)
            anim._setup_scene(scene)
            anim.begin()
            anim.interpolate(0.5)
            moving = [f.mobject for f in (*anim.flights, *anim.vanishing, *anim.growing)]
            self.assertGreaterEqual(len(moving), 3)
            self.assertTrue(all(len(m.get_family()) == 1 and len(m.points) <= 512 for m in moving))
            self.assertTrue(all(m.get_fill_opacity() > 0 or m.get_stroke_opacity() > 0 for m in moving[: len(anim.flights)]))
            anim.interpolate(1.0)
            anim.finish()
            anim.clean_up_from_scene(scene)
            self.assertEqual(scene.mobjects, [dst])

    def test_hull_mode_is_still_available(self):
        src, dst = capsule(), tokens.token("3", kind="cell").shift(3 * manim.RIGHT)
        anim = morph.morph(src, dst, mode="hull")
        self.assertIsInstance(anim, morph.Morph)
        self.assertEqual(play(src, animation=anim).mobjects, [dst])
        with self.assertRaises(ValueError):
            morph.morph(src, dst, mode="blob")

    def test_unmatched_parts_shrink_and_grow_keeping_shape(self):
        src = manim.VGroup(*[manim.Square(0.3 + 0.1 * i, fill_color=manim.RED, fill_opacity=0.9).shift(i * 0.8 * manim.RIGHT) for i in range(5)])
        dst = tokens.token("a", kind="cell").shift(3 * manim.UP)
        anim = morph.morph(src, dst)
        self.assertEqual(len(anim.flights), 2)
        self.assertEqual(len(anim.vanishing), 3)
        scene = manim.Scene(skip_animations=True)
        scene.add(src)
        anim._setup_scene(scene)
        anim.begin()
        anim.interpolate(0.6)
        self.assertTrue(all(f.mobject.get_fill_opacity() > 0.5 for f in anim.vanishing))
        anim.interpolate(1.0)
        anim.finish()
        anim.clean_up_from_scene(scene)
        self.assertEqual(scene.mobjects, [dst])
        small, big = manim.VGroup(manim.Square(0.4, fill_opacity=1)), tokens.token("abc", kind="cell")
        grown = morph.morph(small, big.shift(2 * manim.RIGHT))
        self.assertGreaterEqual(len(grown.growing), 1)
        self.assertEqual(play(small, animation=grown).mobjects, [big])

    def test_heavy_groups_travel_as_real_part_silhouettes(self):
        towers = manim.VGroup(*[manim.VGroup(*[manim.Square(0.05, fill_opacity=0.5).shift([i * 0.1, j * 0.06, 0]) for j in range(4)]) for i in range(80)])
        dst = tokens.token("3", kind="cell").shift(3 * manim.RIGHT)
        anim = morph.morph(towers, dst)
        self.assertLessEqual(len(anim.flights) + len(anim.vanishing), 200)
        self.assertTrue(all(len(f.mobject.get_family()) == 1 for f in (*anim.flights, *anim.vanishing)))
        self.assertEqual(play(towers, animation=anim).mobjects, [dst])

    def test_gather_collapses_multi_part_sources_without_hulls(self):
        items = [capsule(i * 1.2) for i in range(4)]
        badge = tokens.token("4", kind="number").shift(3 * manim.UP)
        anim = morph.gather(items, badge)
        self.assertEqual(play(*items, animation=anim).mobjects, [badge])

    def test_proxy_source_is_cheap(self):
        faces = manim.VGroup(*[manim.Square(0.05, fill_opacity=0.5).shift(i * 0.01 * manim.RIGHT) for i in range(3000)])
        started = time.perf_counter()
        morph.silhouette(faces)
        morph.proxy(faces)
        self.assertLess(time.perf_counter() - started, 3.0)


class GroupTests(unittest.TestCase):
    """🎆 Many-to-many, many-to-one and one-to-many."""

    def test_scatter_140_builds_fast_and_leaves_only_targets(self):
        sources = [capsule(i * 0.05) for i in range(140)]
        targets = [tokens.token(str(i), kind="item").move_to([(i % 14) * 0.3 - 2, (i // 14) * 0.3 - 2, 0]) for i in range(140)]
        started = time.perf_counter()
        anim = morph.scatter(sources, targets, ghost=0.0)
        self.assertLess(time.perf_counter() - started, 2.0)
        self.assertLessEqual(anim.run_time, 4.0 + 1e-6)
        scene = play(*sources, animation=anim)
        self.assertEqual(scene.mobjects, targets)

    def test_scatter_with_pairs_fn_leaves_only_targets(self):
        sources = [capsule(i * 0.3) for i in range(12)]
        targets = [tokens.token(str(i), kind="item").move_to([i * 0.4 - 2, -2, 0]) for i in range(12)]
        anim = morph.scatter(sources, targets, pairs_fn=lambda s, d: [(s[0], d[0]), (s[1], d[1])], dip=0.1)
        scene = play(*sources, animation=anim)
        self.assertEqual(scene.mobjects, targets)

    def test_scatter_rejects_length_mismatch(self):
        with self.assertRaises(ValueError):
            morph.scatter([capsule()], [])

    def test_gather_collapses_sources_into_target(self):
        squares = [manim.Square(0.2, fill_color=manim.ORANGE, fill_opacity=0.9).shift(i * 0.3 * manim.RIGHT + 2 * manim.DOWN) for i in range(20)]
        badge = tokens.token("20", kind="number", color=tokens.DATA).shift(2 * manim.UP)
        scene = play(*squares, animation=morph.gather(squares, badge))
        self.assertEqual(scene.mobjects, [badge])

    def test_split_sprouts_targets_and_removes_source(self):
        src = capsule()
        cells = [tokens.token(str(i), kind="cell").shift(i * 1.2 * manim.RIGHT + 2 * manim.DOWN) for i in range(5)]
        scene = play(src, animation=morph.split(src, cells))
        self.assertEqual(scene.mobjects, cells)

    def test_unfold_to_points_and_to_mobjects(self):
        faces = [manim.Square(0.4, fill_opacity=0.7, color=manim.BLUE).shift(i * 0.1 * manim.UP) for i in range(6)]
        spots = [np.array([i * 0.6 - 1.5, -2.0, 0.0]) for i in range(6)]
        scene = play(*faces, animation=morph.unfold(faces, spots))
        self.assertEqual(len(scene.mobjects), 6)
        np.testing.assert_allclose(sorted(m.get_center()[0] for m in scene.mobjects), sorted(p[0] for p in spots), atol=1e-6)
        rows = [manim.Rectangle(width=1.0, height=0.2).shift(i * 0.3 * manim.DOWN) for i in range(6)]
        scene = play(*faces, animation=morph.unfold(faces, rows))
        self.assertEqual(scene.mobjects, rows)

    def test_unfold_rejects_mismatch(self):
        with self.assertRaises(ValueError):
            morph.unfold([manim.Square()], [])


class RetextTests(unittest.TestCase):
    """✏ Formula morphs."""

    def test_retext_leaves_only_target(self):
        a, b = manim.Text("A1+B2"), manim.Text("B2+A1").shift(manim.DOWN)
        scene = play(a, animation=morph.retext(a, b))
        self.assertEqual(scene.mobjects, [b])


if __name__ == "__main__":
    unittest.main()
