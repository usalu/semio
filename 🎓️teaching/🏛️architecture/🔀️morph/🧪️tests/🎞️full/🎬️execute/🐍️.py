"""🧪 Tests of `🎞️full.execute`: exact target at the terminal frame, genuine intermediates, incompatible topologies, no mutation of the bound entities, deterministic re-evaluation, lazy work and instrumentation. Cases are plain JSON in `🧫️cases`."""

from __future__ import annotations

import copy
import importlib
import json
import math
import random
import unittest
from dataclasses import dataclass, field
from pathlib import Path
from types import SimpleNamespace

import numpy as np

manim = importlib.import_module("manim")
full = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️")
tokens = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🪙️token.🐍️")
timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")

CASES = Path(__file__).resolve().parents[1] / "🧫️cases"


def load(name):
    """📂 Language-agnostic cases of a JSON file."""
    return json.loads((CASES / name).read_text(encoding="utf-8"))


@dataclass(frozen=True)
class Subject:
    """🎯 Stand-in for the stage's bound entity (duck-typed `id`, `mobject`, `parts`, `anchors`)."""

    id: str
    mobject: object
    parts: dict = field(default_factory=dict)
    anchors: dict = field(default_factory=dict)


def shape(spec):
    """🔷 One plain shape from its JSON description."""
    kind, size = spec["shape"], spec["size"]
    if kind == "square":
        mob = manim.Square(size)
    elif kind == "rect":
        mob = manim.Rectangle(width=size[0], height=size[1])
    else:
        mob = manim.Circle(radius=size)
    return mob.set_fill(spec["color"], opacity=spec.get("opacity", 0.9)).set_stroke(spec["color"], width=2).move_to([*spec["at"], 0])


def entity(name, spec):
    """🎯 Bound entity of a JSON side: a group of named plain shapes."""
    parts = {key: shape(value) for key, value in spec["parts"].items()}
    group = manim.VGroup(*parts.values())
    return Subject(name, group, parts, {"center": group.get_center()})


def transition(case):
    """📋 Duck-typed transition of a case (the real spec dataclasses expose the same attributes)."""
    c = case["correspondence"] or {"kind": "identity"}
    return SimpleNamespace(
        copy=case.get("copy", False),
        recipe=case["recipe"],
        relation=case["relation"],
        correspondence=SimpleNamespace(kind=c["kind"], pairs=tuple(tuple(p) for p in c.get("pairs", ())) or None, name=c.get("name")),
        timing=SimpleNamespace(duration=case["duration"], easing=case.get("easing", "smooth"), lag=case.get("lag", 0.04), limit=None),
        final=SimpleNamespace(path_arc=case.get("pathArc", 0.0), dip=case.get("dip", 0.0), ghost=case.get("ghost", 0.0)),
    )


def build(case):
    """🏗 Source, target (None for absence), transition and a fresh scene holding the source."""
    source = entity("source", case["source"])
    target = None if case.get("target", True) is None else entity("target", case["target"])
    scene = manim.Scene(skip_animations=True)
    scene.add(source.mobject)
    return source, target, transition(case), scene


def start(animation, scene):
    """▶ Begin an animation the way `Scene.play` does."""
    if not animation.is_introducer():
        scene.add(animation.mobject)
    animation._setup_scene(scene)
    animation.begin()


def visible(mobs):
    """👁 Visible pointed members (centre, size, fill rgb and opacity) of mobjects or a scene."""
    families = mobs.get_mobject_family_members() if isinstance(mobs, manim.Scene) else mobs.get_family()
    rows = []
    for m in families:
        if isinstance(m, manim.VMobject) and len(m.points) and (m.get_fill_opacity() > 1e-3 or (m.get_stroke_opacity() > 1e-3 and m.get_stroke_width() > 0)):
            c = m.get_center()
            rows.append([c[0], c[1], m.width, m.height, *manim.ManimColor(m.get_fill_color()).to_rgb(), m.get_fill_opacity()])
    return sorted(rows, key=lambda r: tuple(round(v, 1) for v in r))


def box(rows):
    """📦 Bounding box (min x, min y, max x, max y) of signature rows."""
    a = np.array(rows)
    return np.array([(a[:, 0] - a[:, 2] / 2).min(), (a[:, 1] - a[:, 3] / 2).min(), (a[:, 0] + a[:, 2] / 2).max(), (a[:, 1] + a[:, 3] / 2).max()])


def frames(animation, fps):
    """🎞 Number of rendered frames of an animation."""
    return timeline.count(timeline.Window(0.0, animation.run_time), timeline.Schedule(fps))


def evaluate(case, order=None):
    """🎬 Evaluate the frames of a case in the given order; returns animation, scene, entities and the signature per frame."""
    source, target, tr, scene = build(case)
    fps = load("recipes.json")["fps"]
    animation = full.execute(source, target, transition=tr, fps=fps)
    start(animation, scene)
    n = frames(animation, fps)
    sigs = {}
    for k in order(n) if order else range(n):
        animation.interpolate(k / (animation.run_time * fps))
        sigs[k] = visible(scene)
    return animation, scene, source, target, sigs, n


def snapshot(mob):
    """🧬 Deep description of a mobject: points, colours, opacities, updaters and structure."""
    return [(len(m.submobjects), len(m.updaters), m.points.copy(), m.get_fill_color().to_hex() if isinstance(m, manim.VMobject) and len(m.points) else "", m.get_fill_opacity() if isinstance(m, manim.VMobject) else 0.0, m.get_stroke_opacity() if isinstance(m, manim.VMobject) else 0.0) for m in mob.get_family()]


class ExecuteCases(unittest.TestCase):
    """🎞 Every JSON case shows the exact target at f1 and genuine frames before it."""

    def test_terminal_frame_is_the_exact_target(self):
        for case in load("recipes.json")["cases"]:
            with self.subTest(case["name"]):
                animation, scene, source, target, sigs, n = evaluate(case)
                last = sigs[n - 1]
                expected = [] if target is None else visible(target.mobject)
                if case.get("keepsSource"):
                    expected = sorted(expected + visible(source.mobject), key=lambda r: tuple(round(v, 1) for v in r))
                self.assertEqual(len(last), len(expected), f"{case['name']} shows {len(last)} shapes at f1, expected {len(expected)}")
                if expected:
                    np.testing.assert_allclose(np.array(last), np.array(expected), atol=2e-3)

    def test_first_frame_covers_the_source_and_middle_differs_from_both_endpoints(self):
        for case in load("recipes.json")["cases"]:
            with self.subTest(case["name"]):
                animation, scene, source, target, sigs, n = evaluate(case)
                self.assertGreaterEqual(n, 5)
                np.testing.assert_allclose(box(sigs[0]), box(visible(source.mobject)), atol=0.06)
                key = lambda rows: {tuple(round(v, 2) for v in r) for r in rows}
                target_rows = [] if target is None else visible(target.mobject)
                source_rows = visible(source.mobject)
                for k in range(1, n - 1):
                    self.assertNotEqual(key(sigs[k]), key(source_rows), f"frame {k} equals the source")
                    if target_rows:
                        self.assertNotEqual(key(sigs[k]), key(target_rows), f"frame {k} equals the target")
                self.assertTrue(sigs[n // 2])
                self.assertNotEqual(key(sigs[n // 2]), key(source_rows) | key(target_rows))

    def test_scene_holds_exactly_the_bound_target_after_clean_up(self):
        for case in load("recipes.json")["cases"]:
            with self.subTest(case["name"]):
                animation, scene, source, target, sigs, n = evaluate(case)
                animation.finish()
                animation.clean_up_from_scene(scene)
                expected = [] if target is None else [target.mobject]
                if case.get("keepsSource"):
                    expected = [source.mobject, *expected]
                self.assertEqual(scene.mobjects, expected)

    def test_bound_entities_are_never_mutated(self):
        for case in load("recipes.json")["cases"]:
            with self.subTest(case["name"]):
                source, target, tr, scene = build(case)
                before = [copy.deepcopy(source.mobject)] + ([] if target is None else [copy.deepcopy(target.mobject)])
                animation = full.execute(source, target, transition=tr, fps=10)
                start(animation, scene)
                for k in range(frames(animation, 10)):
                    animation.interpolate(k / (animation.run_time * 10))
                animation.finish()
                animation.clean_up_from_scene(scene)
                after = [source.mobject] + ([] if target is None else [target.mobject])
                for was, now in zip(before, after):
                    a, b = snapshot(was), snapshot(now)
                    self.assertEqual(len(a), len(b))
                    for x, y in zip(a, b):
                        self.assertEqual(x[:2], y[:2])
                        np.testing.assert_allclose(x[2], y[2], atol=1e-9)
                        self.assertEqual(x[3:], y[3:])

    def test_arbitrary_alpha_order_is_deterministic(self):
        for case in load("recipes.json")["cases"]:
            with self.subTest(case["name"]):
                *_, ordered, n = evaluate(case)
                shuffled = lambda size: random.Random(5).sample(range(size), size)
                *_, mixed, _ = evaluate(case, order=shuffled)
                again = lambda size: [*range(size), *reversed(range(size))]
                *_, twice, _ = evaluate(case, order=again)
                for k in range(n):
                    self.assertEqual(len(ordered[k]), len(mixed[k]), f"frame {k} differs in order")
                    np.testing.assert_allclose(np.array(ordered[k]), np.array(mixed[k]), atol=1e-6)
                    np.testing.assert_allclose(np.array(ordered[k]), np.array(twice[k]), atol=1e-6)


class RemapTests(unittest.TestCase):
    """⏱ Boundary remap: frame k of n shows progress k/(n-1), f1 exactly the target, duration unchanged."""

    def test_progress_follows_the_schedule(self):
        for case in load("remap.json")["cases"]:
            with self.subTest(case["duration"], fps=case["fps"]):
                spec = {"recipe": "move", "relation": "one-to-one", "correspondence": {"kind": "identity"}, "duration": case["duration"], "easing": "linear",
                        "source": {"parts": {"a": {"shape": "square", "at": [-3, 0], "size": 0.5, "color": "#ff0000"}}},
                        "target": {"parts": {"a": {"shape": "square", "at": [3, 0], "size": 0.5, "color": "#ff0000"}}}}
                source, target, tr, scene = build(spec)
                animation = full.execute(source, target, transition=tr, fps=case["fps"])
                self.assertAlmostEqual(animation.run_time, case["duration"])
                self.assertEqual(frames(animation, case["fps"]), case["frames"])
                start(animation, scene)
                for k, p in enumerate(case["progress"]):
                    animation.interpolate(k / (animation.run_time * case["fps"]))
                    self.assertAlmostEqual(animation.mobject.get_center()[0], -3 + 6 * p, places=6)
                animation.interpolate(1.0)
                self.assertAlmostEqual(animation.mobject.get_center()[0], 3.0, places=6)

    def test_overshooting_easing_still_ends_exactly_on_the_target(self):
        spec = {"recipe": "move", "relation": "one-to-one", "correspondence": {"kind": "identity"}, "duration": 1.0, "easing": "overshoot",
                "source": {"parts": {"a": {"shape": "square", "at": [-3, 0], "size": 0.5, "color": "#ff0000"}}},
                "target": {"parts": {"a": {"shape": "square", "at": [3, 0], "size": 0.5, "color": "#ff0000"}}}}
        source, target, tr, scene = build(spec)
        animation = full.execute(source, target, transition=tr, fps=10)
        start(animation, scene)
        xs = []
        for k in range(10):
            animation.interpolate(k / 10)
            xs.append(animation.mobject.get_center()[0])
        self.assertGreater(max(xs), 3.0)
        self.assertAlmostEqual(xs[-1], 3.0, places=6)


class CorrespondenceTests(unittest.TestCase):
    """🔗 Named recipes, errors and the lazy and bounded work of the executor."""

    def test_capsule_item_recipe_scatters_capsule_drawings_into_tokens(self):
        self.assertIn("capsule-items", full.CORRESPONDENCES)

        def capsule(x):
            return manim.VGroup(manim.RoundedRectangle(width=1.0, height=0.5, corner_radius=0.2, fill_color=manim.BLUE, fill_opacity=0.8), manim.Square(0.2, fill_color=manim.YELLOW, fill_opacity=0.6).shift(0.25 * manim.LEFT), manim.Rectangle(width=0.1, height=0.3, fill_opacity=0.9).shift(0.3 * manim.RIGHT)).shift(x * manim.RIGHT + 2 * manim.LEFT)

        srcs = {str(i): capsule(i * 1.3) for i in range(3)}
        dsts = {str(i): tokens.token(str(i), kind="item").move_to([3, 1 - i, 0]) for i in range(3)}
        source = Subject("caps", manim.VGroup(*srcs.values()), srcs, {"center": np.zeros(3)})
        target = Subject("leaves", manim.VGroup(*dsts.values()), dsts, {"center": np.zeros(3)})
        tr = SimpleNamespace(recipe="scatter", relation="many-to-many", correspondence=SimpleNamespace(kind="recipe", pairs=None, name="capsule-items"), timing=SimpleNamespace(duration=0.8, easing="smooth", lag=0.05, limit=None), final=SimpleNamespace(path_arc=0.3, dip=0.0, ghost=0.0))
        animation = full.execute(source, target, transition=tr, fps=10)
        scene = manim.Scene(skip_animations=True)
        scene.add(source.mobject)
        start(animation, scene)
        n = frames(animation, 10)
        animation.interpolate(1.0)
        np.testing.assert_allclose(np.array(visible(scene)), np.array(visible(target.mobject)), atol=2e-3)
        animation.clean_up_from_scene(scene)
        self.assertEqual(scene.mobjects, [target.mobject])
        self.assertGreater(n, 3)
        self.assertEqual(len(full._couples(tr, source, target)), 3)
        self.assertTrue(all(len(sub) == 2 for _, _, sub in full._couples(tr, source, target)))

    def test_errors_are_actionable(self):
        case = load("recipes.json")["cases"][0]
        source, target, tr, scene = build(case)
        tr.recipe = "teleport"
        with self.assertRaisesRegex(ValueError, "not supported"):
            full.execute(source, target, transition=tr, fps=10)
        tr.recipe, tr.timing.duration = "parts", 0.0
        with self.assertRaisesRegex(ValueError, "atomic"):
            full.execute(source, target, transition=tr, fps=10)
        tr.timing.duration = 0.5
        tr.correspondence = SimpleNamespace(kind="pairs", pairs=(("nope", "ring"),), name=None)
        with self.assertRaisesRegex(ValueError, "no part 'nope'"):
            full.execute(source, target, transition=tr, fps=10)
        tr.correspondence = SimpleNamespace(kind="recipe", pairs=None, name="mystery")
        with self.assertRaisesRegex(ValueError, "unknown correspondence"):
            full.execute(source, target, transition=tr, fps=10)

    def test_n_to_n_sub_morphs_are_built_when_the_group_starts(self):
        case = next(c for c in load("recipes.json")["cases"] if c["name"] == "scatter-key")
        built = []
        original = full._build

        def spy(*args, **kwargs):
            built.append(1)
            return original(*args, **kwargs)

        full._build = spy
        try:
            source, target, tr, scene = build(case)
            animation = full.execute(source, target, transition=tr, fps=10)
            self.assertEqual(built, [])
            start(animation, scene)
            self.assertEqual(len(built), 3)
            animation.interpolate(1.0)
            animation.finish()
            animation.clean_up_from_scene(scene)
        finally:
            full._build = original

    def test_heavy_groups_morph_real_silhouettes_never_rectangle_proxies(self):
        proxied = []
        original = full.proxy
        full.proxy = lambda *a, **k: proxied.append(1) or original(*a, **k)
        try:
            heavy = manim.VGroup(*[manim.VGroup(*[manim.Square(0.05, fill_opacity=0.5).shift([i * 0.12 + j * 0.02, j * 0.06 + (i % 3) * 0.2, 0]) for j in range(6)]) for i in range(60)])
            token = tokens.token("3", kind="cell").shift(3 * manim.RIGHT)
            anim = full.morph(heavy, token)
            self.assertEqual(proxied, [])
            self.assertEqual(len(anim.flights) + len(anim.vanishing), 60)
            self.assertTrue(all(len(f.mobject.family_members_with_points()) == 1 for f in (*anim.flights, *anim.vanishing)))
            tops = manim.VGroup(*[manim.VGroup(*[manim.Square(0.05).shift([i * 0.1, j * 0.1, 0]) for j in range(2)]) for i in range(260)])
            with self.assertRaisesRegex(ValueError, "too large"):
                full.morph(tops, token)
        finally:
            full.proxy = original

    def test_flight_carries_the_colour_of_its_partner(self):
        case = next(c for c in load("recipes.json")["cases"] if c["name"] == "flight-pairs-colour-carry")
        animation, scene, source, target, sigs, n = evaluate(case)
        red, blue = np.array(manim.ManimColor("#ff0000").to_rgb()), np.array(manim.ManimColor("#0000ff").to_rgb())
        mid = [r for r in sigs[n // 2] if 0 < r[2] < 1.5]
        fliers = [np.array(r[4:7]) for r in mid if not np.allclose(r[4:7], red, atol=0.02)]
        self.assertTrue(fliers)
        self.assertTrue(any(0.05 < f[0] < 0.95 and 0.05 < f[2] < 0.95 for f in fliers))
        self.assertTrue(np.allclose(sigs[n - 1][0][4:7], blue, atol=0.02))

    def test_retext_ends_on_the_target(self):
        a, b = manim.Text("A1+B2"), manim.Text("B2+A1").shift(manim.DOWN)
        source = Subject("a", a, {}, {"center": a.get_center()})
        target = Subject("b", b, {}, {"center": b.get_center()})
        tr = SimpleNamespace(recipe="retext", relation="one-to-one", correspondence=SimpleNamespace(kind="identity", pairs=None, name=None), timing=SimpleNamespace(duration=0.6, easing="smooth", lag=0.0, limit=None), final=SimpleNamespace(path_arc=0.0, dip=0.0, ghost=0.0))
        animation = full.execute(source, target, transition=tr, fps=10)
        scene = manim.Scene(skip_animations=True)
        scene.add(a)
        start(animation, scene)
        animation.interpolate(1.0)
        animation.finish()
        animation.clean_up_from_scene(scene)
        self.assertEqual(scene.mobjects, [b])


class AppearTests(unittest.TestCase):
    """🌱 A target without source (`appear`) grows into view and ends exactly on the bound target."""

    def test_appear_ends_on_the_bound_target(self):
        target = entity("target", {"parts": {"a": {"shape": "circle", "at": [2, 1], "size": 0.5, "color": "#00ff00"}}})
        tr = transition({"recipe": "appear", "relation": "one-to-one", "correspondence": None, "duration": 0.6, "source": None})
        animation = full.execute(None, target, transition=tr, fps=10)
        scene = manim.Scene(skip_animations=True)
        start(animation, scene)
        animation.interpolate(1.0)
        np.testing.assert_allclose(np.array(visible(scene)), np.array(visible(target.mobject)), atol=2e-3)
        animation.finish()
        animation.clean_up_from_scene(scene)
        self.assertEqual(scene.mobjects, [target.mobject])


class InstrumentTests(unittest.TestCase):
    """📊 Work counters: preparation and per-frame evaluation are counted."""

    def test_prepare_and_evaluate_are_counted(self):
        instrument.reset()
        case = next(c for c in load("recipes.json")["cases"] if c["name"] == "parts-key-extra-source-part")
        *_, n = evaluate(case)
        counts = instrument._COUNTS
        self.assertGreater(counts.get("full.prepare", 0), 0)
        self.assertGreaterEqual(counts.get("full.evaluate", 0), n)
        instrument.reset()
        self.assertEqual(instrument._COUNTS, {})


class HullOracleTests(unittest.TestCase):
    """🔺 The vectorised hull matches SciPy on language-agnostic point sets."""

    def test_hull_matches_stored_scipy_vertices(self):
        for case in load("hull.json")["cases"]:
            with self.subTest(case["name"]):
                points = np.array([[x, y, 0.0] for x, y in case["points"]])
                ours = sorted([round(x, 2), round(y, 2)] for x, y in full._hull(points))
                self.assertEqual(ours, case["hull"])

    def test_hull_matches_live_scipy(self):
        spatial = importlib.import_module("scipy.spatial")
        points = np.random.default_rng(3).normal(size=(500, 3)) * [1, 1, 0]
        ours = {tuple(np.round(p, 2)) for p in full._hull(points)}
        theirs = {tuple(np.round(points[i][:2], 2)) for i in spatial.ConvexHull(points[:, :2]).vertices}
        self.assertEqual(ours, theirs)

    def test_outline_evaluates_all_curves(self):
        group = manim.VGroup(manim.Square(1.0), manim.Circle(radius=0.5).shift(2 * manim.RIGHT))
        outline = full._outline(group)
        curves = sum(len(m.points) // 4 for m in group.get_family() if len(m.points))
        self.assertEqual(len(outline), curves * len(full.SAMPLES))
        self.assertAlmostEqual(outline[:, 0].max(), 2.5, places=2)


if __name__ == "__main__":
    unittest.main()
