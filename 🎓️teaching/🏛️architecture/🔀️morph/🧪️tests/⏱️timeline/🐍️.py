"""🧪 Tests of the canonical morph timeline: language-agnostic JSON cases plus Manim and NumPy oracles for counts, samples, staggering and easing."""

from __future__ import annotations

import decimal
import importlib
import json
import math
import random
import unittest
from pathlib import Path

import numpy as np

timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")
manim = importlib.import_module("manim")
clock = importlib.import_module("🎓️teaching.🧰️interop.🕐️clock.🐍️")

CASES = Path(__file__).parent / "🧫️cases"
RATES = (15, 24, 25, 30, 60)


def cases(name):
    return json.loads((CASES / f"{name}.json").read_text(encoding="utf-8"))["cases"]


def schedule_of(case):
    return None if case["fps"] is None else timeline.Schedule(case["fps"], case.get("origin", 0.0))


def window_of(data):
    return timeline.Window(data["start"], data["duration"])


class TerminalCases(unittest.TestCase):
    def test_cases(self):
        for case in cases("terminal"):
            with self.subTest(case["name"]):
                schedule, window = schedule_of(case), window_of(case["window"])
                self.assertEqual(timeline.count(window, schedule), case["count"])
                expected = case["frames"]
                self.assertEqual(timeline.frames(window, schedule), None if expected is None else tuple(expected))
                if "boundary" in case:
                    self.assertAlmostEqual(timeline.boundary(window, schedule), case["boundary"], places=9)
                for s in case["samples"]:
                    got = timeline.sample(window, schedule, s["frame"])
                    self.assertEqual(got.owner, s["owner"], s)
                    self.assertAlmostEqual(got.progress, s["progress"], places=12, msg=str(s))

    def test_ten_frames_source_until_108_target_on_109(self):
        window, schedule = timeline.Window(10.0, 1.0), timeline.Schedule(10)
        owners = [timeline.owner(window, schedule, f) for f in range(99, 112)]
        self.assertEqual(owners, ["before"] + ["source"] * 9 + ["target"] * 3)

    def test_zero_duration_has_no_frames_and_switches_atomically(self):
        window, schedule = timeline.Window(2.0, 0.0), timeline.Schedule(24)
        self.assertIsNone(timeline.frames(window, schedule))
        self.assertEqual(timeline.count(window, schedule), 0)
        self.assertEqual([timeline.owner(window, schedule, f) for f in (47, 48, 49)], ["before", "target", "target"])

    def test_negative_duration_and_fps_rejected(self):
        with self.assertRaises(ValueError):
            timeline.Window(0.0, -1.0)
        with self.assertRaises(ValueError):
            timeline.Schedule(0)


class ContinuousCases(unittest.TestCase):
    def test_cases(self):
        for case in cases("continuous"):
            with self.subTest(case["name"]):
                window, schedule = window_of(case["window"]), schedule_of(case)
                for s in case.get("samples", []) + case.get("sequence", []):
                    got = timeline.at(window, s["t"], schedule)
                    self.assertEqual(got.owner, s["owner"], s)
                    if "progress" in s:
                        self.assertAlmostEqual(got.progress, s["progress"], places=9, msg=str(s))

    def test_offline_and_continuous_agree_at_frame_times(self):
        for fps in RATES:
            schedule = timeline.Schedule(fps, 0.25)
            window = timeline.Window(0.25 + 7 / fps, 0.9)
            f0, f1 = timeline.frames(window, schedule)
            for frame in range(f0 - 3, f1 + 4):
                a = timeline.sample(window, schedule, frame)
                b = timeline.at(window, schedule.time(frame), schedule)
                self.assertEqual(a.owner, b.owner, (fps, frame))
                self.assertAlmostEqual(a.progress, b.progress, places=9)

    def test_skipped_terminal_frame_shows_target_at_next_evaluation(self):
        for fps in RATES:
            schedule = timeline.Schedule(fps)
            window = timeline.Window(1.0, 0.5)
            tb = timeline.boundary(window, schedule)
            before, after = tb - 1.5 / fps, tb + 0.4 / fps
            self.assertEqual(timeline.at(window, before, schedule).owner, "source")
            self.assertEqual(timeline.at(window, after, schedule).owner, "target")
            self.assertEqual(timeline.at(window, tb + 10, schedule).progress, 1.0)

    def test_order_independence_variable_cadence(self):
        schedule, window = timeline.Schedule(30), timeline.Window(2.0, 1.0)
        times = [1.5 + i * 0.013 for i in range(200)]
        forward = [timeline.at(window, t, schedule) for t in times]
        order = list(range(len(times)))
        random.Random(7).shuffle(order)
        shuffled = {i: timeline.at(window, times[i], schedule) for i in order}
        self.assertEqual(forward, [shuffled[i] for i in range(len(times))])
        self.assertEqual(forward, [timeline.at(window, t, schedule) for t in times])
        owners = [s.owner for s in forward]
        self.assertEqual(owners, sorted(owners, key={"before": 0, "source": 1, "target": 2}.get))


class StatelessEvaluation(unittest.TestCase):
    def setUp(self):
        self.schedule, self.window = timeline.Schedule(24), timeline.Window(10.0, 2.0)
        f0, f1 = timeline.frames(self.window, self.schedule)
        self.span = range(f0 - 5, f1 + 6)
        self.forward = [timeline.sample(self.window, self.schedule, f) for f in self.span]

    def test_reverse_iteration(self):
        backward = [timeline.sample(self.window, self.schedule, f) for f in reversed(self.span)]
        self.assertEqual(self.forward, backward[::-1])

    def test_random_seek_order(self):
        frames = list(self.span)
        random.Random(11).shuffle(frames)
        got = {f: timeline.sample(self.window, self.schedule, f) for f in frames}
        self.assertEqual(self.forward, [got[f] for f in self.span])

    def test_trimmed_range_evaluates_original_timeline(self):
        f0, f1 = timeline.frames(self.window, self.schedule)
        for frame in (f0 + 17, f1 - 1, f1):
            alone = timeline.sample(self.window, self.schedule, frame)
            self.assertEqual(alone, self.forward[frame - self.span.start])
        self.assertEqual(timeline.sample(self.window, self.schedule, f0 + 17).progress, 17 / (f1 - f0))

    def test_loop_restart_and_pause_resume_repeat_exactly(self):
        looped = [timeline.sample(self.window, self.schedule, f) for _ in range(3) for f in self.span]
        self.assertEqual(looped, self.forward * 3)
        again = timeline.sample(self.window, self.schedule, self.span.start + 9)
        self.assertEqual(again, self.forward[9])

    def test_frame_zero_restart_before_window(self):
        self.assertEqual(timeline.sample(self.window, self.schedule, 0), timeline.Sample("before", 0.0))


def decimal_count(start, duration, fps):
    """🧮 Independent oracle: grid points in `[start, start + T)` by decimal arithmetic, at least one for T > 0."""
    rate = decimal.Decimal(str(fps))
    first, end = math.ceil(decimal.Decimal(str(start)) * rate), math.ceil((decimal.Decimal(str(start)) + decimal.Decimal(str(duration))) * rate)
    return max(end - first, 1)


class CountOracle(unittest.TestCase):
    def test_count_is_exact_where_numpy_arange_picks_up_float_noise(self):
        noisy = [(25, 0.28), (25, 0.56), (25, 1.12), (25, 4.44), (60, 4.15)]
        quiet = [(30, 0.6), (24, 0.3), (15, 0.2), (10, 0.3), (30, 0.1)]
        for fps, duration in noisy:
            with self.subTest((fps, duration)):
                exact = timeline.count(timeline.Window(0.0, duration), timeline.Schedule(fps))
                self.assertEqual(exact, decimal_count(0, duration, fps))
                self.assertEqual(len(np.arange(0, duration, 1 / fps)), exact + 1)
        for fps, duration in quiet:
            with self.subTest((fps, duration)):
                self.assertEqual(timeline.count(timeline.Window(0.0, duration), timeline.Schedule(fps)), len(np.arange(0, duration, 1 / fps)))

    def test_count_matches_decimal_oracle_on_a_dense_grid_and_numpy_never_undercounts(self):
        for fps in RATES + (12, 48, 50, 120):
            schedule = timeline.Schedule(fps)
            for hundredths in range(1, 1500):
                duration = hundredths / 100
                got = timeline.count(timeline.Window(0.0, duration), schedule)
                self.assertEqual(got, decimal_count(0, duration, fps), (fps, duration))
                self.assertIn(len(np.arange(0, duration, 1 / fps)) - got, (0, 1), (fps, duration))

    def test_counts_on_the_absolute_grid_add_up_to_the_total(self):
        rng = random.Random(5)
        for fps in RATES:
            schedule = timeline.Schedule(fps)
            for _ in range(200):
                durations = [rng.randrange(7, 400) / 100 for _ in range(rng.randrange(1, 12))]
                start, total = 0.0, 0
                for d in durations:
                    total += timeline.count(timeline.Window(start, d), schedule)
                    start = round(start + d, 6)
                self.assertEqual(total, math.ceil(decimal.Decimal(str(round(sum(durations), 6))) * fps), (fps, durations))

    def test_count_sub_frame_duration_is_one_frame(self):
        for fps in RATES:
            self.assertEqual(timeline.count(timeline.Window(0.0, 0.25 / fps), timeline.Schedule(fps)), 1)

    def test_numpy_reimplementation_of_owner_and_progress(self):
        for fps in RATES:
            for duration in (0.2, 0.5, 1.0, 1.28, 2.0 + 1 / 3):
                start = 4.0
                schedule, window = timeline.Schedule(fps), timeline.Window(start, duration)
                n = decimal_count(start, round(duration, 9), fps)
                f0 = round(start * fps)
                frames = f0 + np.arange(n)
                expected_owner = np.where(np.arange(n) < n - 1, "source", "target")
                expected_progress = np.arange(n) / (n - 1) if n > 1 else np.ones(1)
                got = [timeline.sample(window, schedule, int(f)) for f in frames]
                self.assertEqual([s.owner for s in got], list(expected_owner))
                np.testing.assert_allclose([s.progress for s in got], expected_progress, rtol=0, atol=1e-12)


class _Recorder(manim.Animation):
    def __init__(self, mobject, **kwargs):
        super().__init__(mobject, **kwargs)
        self.alphas, self.phases = [], []

    def interpolate(self, alpha):
        self.alphas.append(alpha)
        self.phases.append(clock.SCENE_CLOCK.phase)


OPTIONS = {"write_to_movie": False, "disable_caching": True, "verbosity": "ERROR", "save_last_frame": False, "progress_bar": "none", "pixel_width": 64, "pixel_height": 36}


def played_alphas(run_time, fps):
    with manim.tempconfig({**OPTIONS, "frame_rate": fps}):
        scene = manim.Scene()
        mob = manim.Dot()
        scene.add(mob)
        recorder = _Recorder(mob, run_time=run_time)
        scene.play(recorder)
    return recorder.alphas


def played_sequence(durations, fps, skip_from=None):
    options = {**OPTIONS, "frame_rate": fps, **({} if skip_from is None else {"from_animation_number": skip_from})}
    with manim.tempconfig(options):
        scene = manim.Scene()
        mob = manim.Dot()
        scene.add(mob)
        recorders = [_Recorder(mob, run_time=d) for d in durations]
        for recorder in recorders:
            scene.play(recorder)
    return recorders


class ManimOracle(unittest.TestCase):
    def test_real_scene_play_matches_count_and_terminal_sample(self):
        for fps, duration in ((15, 1.0), (24, 0.5), (25, 0.28), (30, 0.6), (60, 0.1), (30, 0.01), (30, 1.0 / 30)):
            with self.subTest((fps, duration)):
                alphas = played_alphas(duration, fps)
                n = timeline.count(timeline.Window(0.0, max(duration, 1 / fps)), timeline.Schedule(fps))
                self.assertEqual(alphas[0], 0)
                self.assertEqual(len(alphas) - 2, n)
                samples = [timeline.sample_alpha(a, duration, fps) for a in alphas[1:-1]]
                self.assertTrue(all(s.owner == "source" for s in samples[:-1]))
                self.assertEqual(samples[-1].owner, "target")
                self.assertEqual(timeline.sample_alpha(alphas[-1], duration, fps).owner, "target")
                self.assertEqual(alphas[-1], 1)

    def test_chained_plays_off_the_grid_sample_each_terminal_frame_once(self):
        fps, durations = 15, [0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.4, 0.4]
        recorders = played_sequence(durations, fps)
        start, schedule, total = 0.0, timeline.Schedule(fps), 0
        for index, (recorder, duration) in enumerate(zip(recorders, durations)):
            window = timeline.Window(start, duration)
            n = timeline.count(window, schedule)
            frames = recorder.alphas[1:-1]
            self.assertEqual(len(frames), n, (index, recorder.alphas))
            samples = [timeline.sample_alpha(a, duration, fps, p) for a, p in zip(frames, recorder.phases[1:-1])]
            self.assertTrue(all(s.owner == "source" for s in samples[:-1]), (index, samples))
            self.assertEqual(samples[-1].owner, "target", (index, samples))
            self.assertEqual(timeline.sample_alpha(recorder.alphas[-1], duration, fps, recorder.phases[-1]).owner, "target")
            total += n
            start = round(start + duration, 6)
        self.assertEqual(total, math.ceil(decimal.Decimal(str(round(sum(durations), 6))) * fps))

    def test_skipped_plays_reach_the_same_clock_and_terminal_target(self):
        fps, durations = 15, [0.1, 0.1, 0.1, 0.1, 0.1, 0.1]
        rendered, skipped = played_sequence(durations, fps), played_sequence(durations, fps, skip_from=3)
        for a, b in zip(rendered[3:], skipped[3:]):
            self.assertEqual(a.phases[1], b.phases[1])
        for recorder, duration in zip(skipped[:3], durations):
            self.assertEqual(timeline.sample_alpha(recorder.alphas[-1], duration, fps, recorder.phases[-1]).owner, "target")

    def test_frame_of_alpha_recovers_every_scheduled_frame(self):
        for fps in RATES:
            for duration in (0.28, 0.6, 1.0, 1.7, 3.3):
                times = np.arange(0, duration, 1 / fps)
                for k, t in enumerate(times):
                    self.assertEqual(timeline.frame_of_alpha(t / duration, duration, fps), k, (fps, duration, k))

    def test_skipped_animations_sample_target(self):
        self.assertEqual(timeline.sample_alpha(1.0, 1.2, 30).owner, "target")


class StaggerOracle(unittest.TestCase):
    def test_cases(self):
        for case in cases("stagger"):
            with self.subTest(case["name"]):
                window = window_of(case["window"])
                windows = timeline.expand(window, case["size"], case["lag"], case["limit"])
                self.assertEqual(len(windows), case["size"])
                self.assertAlmostEqual(timeline.stagger_lag(window.duration, case["size"], case["lag"], case["limit"]), case["lag_effective"], places=12)
                self.assertAlmostEqual(timeline.span(windows).duration, case["group_duration"], places=9)
                self.assertEqual(windows[0].start, window.start)
                self.assertTrue(all(w.duration == window.duration for w in windows))

    def test_matches_manim_lagged_start_timings(self):
        for case in cases("stagger"):
            with self.subTest(case["name"]):
                window = window_of(case["window"])
                windows = timeline.expand(window, case["size"], case["lag"], case["limit"])
                lag = timeline.stagger_lag(window.duration, case["size"], case["lag"], case["limit"])
                ratio = lag / window.duration if window.duration else 0.0
                dummies = [manim.Animation(manim.Dot(), run_time=window.duration) for _ in range(case["size"])]
                group = manim.LaggedStart(*dummies, lag_ratio=ratio)
                timings = group.anims_with_timings
                np.testing.assert_allclose([w.start - window.start for w in windows], timings["start"], rtol=0, atol=1e-12)
                np.testing.assert_allclose([w.end - window.start for w in windows], timings["end"], rtol=0, atol=1e-12)
                self.assertAlmostEqual(group.max_end_time, timeline.span(windows).duration, places=9)

    def test_matches_morph_staggered_when_available(self):
        for module_name in ("🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️", "🎓️teaching.🏛️architecture.🔀️morph.🐍️"):
            try:
                module = importlib.import_module(module_name)
            except ImportError:
                continue
            staggered = getattr(module, "_staggered", None)
            if staggered is None:
                continue
            for run_time, size, lag, limit in ((1.2, 140, 0.012, 2.0), (1.0, 4, 0.1, 4.0), (1.5, 10, 0.2, 1.0)):
                dummies = [manim.Animation(manim.Dot(), run_time=run_time) for _ in range(size)]
                group = staggered(dummies, run_time, lag, limit)
                windows = timeline.expand(timeline.Window(0.0, run_time), size, lag, limit)
                self.assertAlmostEqual(group.max_end_time, timeline.span(windows).duration, places=9)
            return
        self.skipTest("no _staggered in the morph modules")

    def test_element_progress_finishes_every_element_exactly_at_group_end(self):
        windows = timeline.expand(timeline.Window(0.0, 1.2), 140, 0.012, 2.0)
        self.assertEqual(set(timeline.element_progress(windows, 1.0)), {1.0})
        self.assertEqual(set(timeline.element_progress(windows, 0.0)), {0.0})
        mid = timeline.element_progress(windows, 0.5)
        self.assertEqual(list(mid), sorted(mid, reverse=True))

    def test_element_progress_matches_manim_group_alpha(self):
        window = timeline.Window(0.0, 1.0)
        windows = timeline.expand(window, 5, 0.1, 4.0)
        for p in (0.0, 0.2, 0.55, 0.9, 1.0):
            now = p * timeline.span(windows).duration
            expected = [min(max((now - w.start) / w.duration, 0.0), 1.0) for w in windows]
            np.testing.assert_allclose(timeline.element_progress(windows, p), expected, atol=1e-12)


class ChainCases(unittest.TestCase):
    def test_cases(self):
        for case in cases("chain"):
            with self.subTest(case["name"]):
                schedule = timeline.Schedule(case["fps"], case["origin"])
                windows = [window_of(w) for w in case["windows"]]
                links = timeline.chain(windows, schedule)
                for link, expected in zip(links, case["links"]):
                    self.assertEqual((link.index, link.f0, link.f1, link.until, link.terminal_owned), (expected["index"], expected["f0"], expected["f1"], expected["until"], expected["terminalOwned"]))
                for frame, index, owner in case["owners"]:
                    got = timeline.resolve(windows, schedule, frame)
                    self.assertEqual((got.index, got.sample.owner), (index, owner), frame)
                for frame in case.get("before", []):
                    self.assertIsNone(timeline.resolve(windows, schedule, frame))

    def test_no_double_or_missing_owner_on_random_chains(self):
        rng = random.Random(3)
        for fps in RATES:
            schedule = timeline.Schedule(fps)
            starts = sorted(round(rng.uniform(0, 6) * fps) / fps for _ in range(8))
            windows = [timeline.Window(s, rng.choice([0.0, 0.01, 0.3, 0.5, 1.0])) for s in starts]
            links = timeline.chain(windows, schedule)
            for frame in range(links[0].f0, links[-1].f1 + 40):
                owned = [link for link in links if link.f0 <= frame and (link.until is None or frame <= link.until)]
                self.assertEqual(len(owned), 1, (fps, frame))
                self.assertEqual(timeline.resolve(windows, schedule, frame).index, owned[0].index)
                t = schedule.time(frame)
                self.assertEqual(timeline.resolve_at(windows, t, schedule).index, owned[0].index)

    def test_unordered_windows_rejected(self):
        with self.assertRaises(ValueError):
            timeline.chain([timeline.Window(2.0, 1.0), timeline.Window(0.0, 1.0)], timeline.Schedule(10))

    def test_resolve_at_without_schedule(self):
        windows = [timeline.Window(0.0, 1.0), timeline.Window(1.0, 1.0)]
        self.assertIsNone(timeline.resolve_at(windows, -0.5))
        self.assertEqual(timeline.resolve_at(windows, 0.999).index, 0)
        self.assertEqual((timeline.resolve_at(windows, 1.0).index, timeline.resolve_at(windows, 1.0).sample.owner), (1, "source"))


class EasingOracle(unittest.TestCase):
    MANIM = {
        "ease-in": "ease_in_cubic", "ease-out": "ease_out_cubic", "ease-in-out": "ease_in_out_cubic",
        "overshoot": "ease_out_back", "ease-out-back": "ease_out_back", "ease-in-back": "ease_in_back",
    }

    def manim_name(self, name):
        return self.MANIM.get(name, name.replace("-", "_"))

    def test_every_easing_matches_manim_rate_function(self):
        grid = [i / 64 for i in range(65)] + [0.1, 0.333, 0.777]
        for name in timeline.EASINGS:
            rate = getattr(manim.rate_functions, self.manim_name(name))
            for p in grid:
                self.assertAlmostEqual(timeline.ease(name, p), float(rate(p)), places=12, msg=f"{name}({p})")

    def test_smooth_is_bit_identical(self):
        for i in range(0, 1001):
            p = i / 1000
            self.assertEqual(timeline.ease("smooth", p), float(manim.rate_functions.smooth(p)))

    def test_endpoints_and_clamping(self):
        for name in timeline.EASINGS:
            self.assertAlmostEqual(timeline.ease(name, -3.0), 0.0, places=12)
        for name in ("linear", "smooth", "ease-in", "ease-out", "overshoot", "double-smooth", "rush-into", "slow-into"):
            self.assertAlmostEqual(timeline.ease(name, 1.0), 1.0, places=12)
            self.assertAlmostEqual(timeline.ease(name, 9.0), 1.0, places=12)
        self.assertAlmostEqual(timeline.ease("there_and_back", 1.0), 0.0, places=12)

    def test_unknown_easing_is_actionable(self):
        with self.assertRaisesRegex(ValueError, "unknown easing 'bounce'.*known: "):
            timeline.ease("bounce", 0.5)

    def test_cases(self):
        for name, p, expected in (("smooth", 0.5, 0.5), ("linear", 0.3, 0.3), ("overshoot", 0.8, 1.04645056), ("there-and-back", 0.5, 1.0), ("there-and-back-with-pause", 0.5, 1.0)):
            self.assertAlmostEqual(timeline.ease(name, p), expected, places=7, msg=name)

    def test_easing_never_switches_identity_early(self):
        for fps in RATES:
            schedule, window = timeline.Schedule(fps), timeline.Window(1.0, 1.0)
            f0, f1 = timeline.frames(window, schedule)
            for name in timeline.EASINGS:
                for frame in range(f0 - 2, f1 + 3):
                    s = timeline.sample(window, schedule, frame)
                    eased = timeline.ease(name, s.progress)
                    expected = "before" if frame < f0 else "source" if frame < f1 else "target"
                    self.assertEqual(s.owner, expected, (name, fps, frame, eased))

    def test_overshoot_exceeds_one_and_early_one_while_source(self):
        window, schedule = timeline.Window(0.0, 1.0), timeline.Schedule(30)
        f0, f1 = timeline.frames(window, schedule)
        over = [timeline.ease("overshoot", timeline.progress(window, schedule, f)) for f in range(f0, f1)]
        self.assertGreater(max(over), 1.0)
        pause = [(timeline.ease("there-and-back-with-pause", timeline.progress(window, schedule, f)), timeline.owner(window, schedule, f)) for f in range(f0, f1)]
        self.assertTrue(any(value == 1 and who == "source" for value, who in pause))


if __name__ == "__main__":
    unittest.main()
