"""🧪 The canonical scene clock against Manim's own frame writer and a skipping scene."""
import importlib
import json
import math
import re
import unittest
from fractions import Fraction
from pathlib import Path

from jsonschema import Draft202012Validator
from manim import Mobject, Scene, ValueTracker, tempconfig

clock = importlib.import_module("🎓️teaching.🧰️interop.🕐️clock.🐍️")
here = Path(__file__).parent
cases = json.loads((here / "🔣️.cases.json").read_text(encoding="utf-8"))
schema = json.loads((here / "🔣️.schema.json").read_text(encoding="utf-8"))


def run(skip_from=None):
    """🎬 Play the schedule; return per play the frames Manim's writer received, the scene time after it and the times its updater saw."""
    plan = cases["schedule"]
    settings = {"dry_run": True, "pixel_width": 320, "pixel_height": 180, "frame_rate": plan["fps"], "progress_bar": "none"}
    if skip_from is not None:
        settings["from_animation_number"] = skip_from
    with tempconfig(settings):
        scene, mob, seen, counter = Scene(), Mobject(), [], [0]
        mob.add_updater(lambda m: seen.append(clock.SCENE_CLOCK.now))
        scene.add(mob)

        def write_frame(frame, repeat=1):
            counter[0] += repeat

        scene.renderer.file_writer.write_frame = write_frame
        result = []
        for step in plan["plays"]:
            before, mark = counter[0], len(seen)
            if step["kind"] == "play":
                scene.play(mob.animate.shift([1, 0, 0]), run_time=step["duration"])
            else:
                scene.wait(step["duration"], frozen_frame=step["frozen"])
            result.append({"written": counter[0] - before, "time": scene.renderer.time, "seen": seen[mark:]})
        return result


class SceneClock(unittest.TestCase):
    """🔬 Frame counts, drift and render/skip parity."""

    def test_schema(self):
        Draft202012Validator(schema).validate(cases)

    def test_frame_count(self):
        for case in cases["counts"]:
            self.assertEqual(clock.frame_count(case["duration"], case["fps"]), case["frames"], case)

    def test_sequences_land_on_the_absolute_grid_without_accumulating(self):
        for case in cases["sequences"]:
            with self.subTest(case["name"]):
                total, done, got = Fraction(0), 0, []
                for duration in case["durations"]:
                    total += Fraction(duration).limit_denominator(10**6)
                    frames = clock.frame_count(duration, case["fps"], total - Fraction(duration).limit_denominator(10**6), done)
                    got.append(frames)
                    done += frames
                self.assertEqual(got, case["frames"])
                self.assertEqual(done, math.ceil(total * Fraction(case["fps"]).limit_denominator(1001)))

    def test_real_scene_writes_the_grid_frames_of_each_sequence(self):
        for case in cases["sequences"]:
            if case.get("direct"):
                continue
            with self.subTest(case["name"]), tempconfig({"dry_run": True, "pixel_width": 320, "pixel_height": 180, "frame_rate": case["fps"], "progress_bar": "none"}):
                scene, mob, counter, got = Scene(), Mobject(), [0], []
                mob.add_updater(lambda m: None)
                scene.add(mob)
                scene.renderer.file_writer.write_frame = lambda frame, repeat=1: counter.__setitem__(0, counter[0] + repeat)
                for index, duration in enumerate(case["durations"]):
                    before = counter[0]
                    scene.wait(duration, frozen_frame=index % 2 == 1)
                    got.append(counter[0] - before)
                self.assertEqual(got, case["frames"])

    def test_written_frames_match_the_declared_count(self):
        for step, result in zip(cases["schedule"]["plays"], run()):
            self.assertEqual(result["written"], step["frames"], step)

    def test_time_is_the_exact_frame_total(self):
        fps, total = Fraction(cases["schedule"]["fps"]), 0
        for step, result in zip(cases["schedule"]["plays"], run()):
            total += step["frames"]
            self.assertEqual(result["time"], float(total / fps), step)

    def test_skipped_plays_end_on_the_rendered_clock(self):
        rendered, skipped = run(), run(cases["schedule"]["skipFrom"])
        for index, (a, b) in enumerate(zip(rendered, skipped)):
            self.assertEqual(a["time"], b["time"], index)
        for index in range(cases["schedule"]["skipFrom"], len(rendered)):
            self.assertEqual(rendered[index]["seen"], skipped[index]["seen"], index)
        first = cases["schedule"]["skipFrom"] - 1
        self.assertEqual(skipped[first]["seen"][-1:], rendered[first]["seen"][-1:])

    def test_samples_are_the_frame_over_the_rate(self):
        fps = Fraction(cases["schedule"]["fps"])
        seen = list(dict.fromkeys(run()[0]["seen"]))
        self.assertEqual(seen[:8], [float(Fraction(k) / fps) for k in range(8)])

    def test_origin_is_the_play_start_during_a_play_only(self):
        with tempconfig({"dry_run": True, "frame_rate": 15, "progress_bar": "none"}):
            scene, mob, origins = Scene(), Mobject(), []
            mob.add_updater(lambda m: origins.append(clock.SCENE_CLOCK.origin))
            scene.add(mob)
            scene.wait(0.2, frozen_frame=False)
            scene.play(mob.animate.shift([1, 0, 0]), run_time=0.2)
            self.assertEqual(origins[:3], [0.0] * 3)
            self.assertEqual(origins[-4:-1], [float(Fraction(3, 15))] * 3)
            self.assertIsNone(clock.SCENE_CLOCK.origin)


class DtIntegrators(unittest.TestCase):
    """🔬 Timers read canonical time; no clip integrates `dt`."""

    def timer_values(self, skip_from=None):
        plan = cases["schedule"]
        settings = {"dry_run": True, "pixel_width": 320, "pixel_height": 180, "frame_rate": plan["fps"], "progress_bar": "none"}
        if skip_from is not None:
            settings["from_animation_number"] = skip_from
        with tempconfig(settings):
            scene, timer, mob, values = Scene(), clock.scene_timer(0.8), Mobject(), []
            mob.add_updater(lambda m: None)
            scene.add(timer, mob)
            for step in plan["plays"]:
                if step["kind"] == "play":
                    scene.play(mob.animate.shift([1, 0, 0]), run_time=step["duration"])
                else:
                    scene.wait(step["duration"], frozen_frame=step["frozen"])
                values.append(timer.get_value())
            return values

    def test_timer_is_rate_times_scene_time_in_every_mode(self):
        fps, total, expected = Fraction(cases["schedule"]["fps"]), 0, []
        for step in cases["schedule"]["plays"]:
            total += step["frames"]
            expected.append(0.8 * float(total / fps))
        rendered, skipped = self.timer_values(), self.timer_values(cases["schedule"]["skipFrom"])
        self.assertEqual(len(rendered), len(expected))
        live = [index for index, step in enumerate(cases["schedule"]["plays"]) if not step.get("frozen", False)]
        for index in live:
            self.assertAlmostEqual(rendered[index], expected[index], places=9)
        first = cases["schedule"]["skipFrom"]
        self.assertEqual(rendered[first:], skipped[first:])
        self.assertAlmostEqual(skipped[first], expected[first], places=9)

    def integral_values(self, skip_from=None):
        settings = {"dry_run": True, "pixel_width": 320, "pixel_height": 180, "frame_rate": 15, "progress_bar": "none"}
        if skip_from is not None:
            settings["from_animation_number"] = skip_from
        with tempconfig(settings):
            scene, rate, mob, ends = Scene(), ValueTracker(0.0), Mobject(), []
            phase, last = clock.rate_integral(0.5), []
            mob.add_updater(lambda m: last.append(phase(rate.get_value())))
            scene.add(rate, mob)
            for target, run in ((1.0, 1.0), (1.0, 0.5), (0.0, 0.8), (0.4, 0.6)):
                scene.play(rate.animate.set_value(target), run_time=run, rate_func=lambda a: a * a)
                ends.append(last[-1])
            return ends

    def test_rate_integral_is_piecewise_linear_between_play_boundaries_and_equal_when_skipping(self):
        rendered, skipped = self.integral_values(), self.integral_values(2)
        expected, total = [], 0.0
        for r0, r1, run in ((0.0, 1.0, 15 / 15), (1.0, 1.0, 8 / 15), (1.0, 0.0, 12 / 15), (0.0, 0.4, 9 / 15)):
            total += 0.5 * (r0 + r1) / 2 * run
            expected.append(total)
        for value, want in zip(rendered, expected):
            self.assertAlmostEqual(value, want, places=9)
        for value, want in zip(skipped, expected):
            self.assertAlmostEqual(value, want, places=9)

    def test_no_clip_accumulates_dt(self):
        root = here.parents[2] / "🏛️architecture"
        pattern = re.compile(r"increment_value\([^)\n]*\bdt\b|\[.\w+.\]\s*\+=\s*dt\b")
        offenders = sorted(path.parent.name for path in root.rglob("🐍️.py")
                           if "🎬️clip" in path.parts and ".venv" not in path.parts and pattern.search(path.read_text(encoding="utf-8")))
        self.assertEqual(offenders, [])


if __name__ == "__main__":
    unittest.main()
