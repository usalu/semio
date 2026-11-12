"""🧪 End-to-end stage tests: a real transitions JSON is loaded, entities are bound and the transition plays in a real Manim scene at a tiny resolution under SEMIO_TEACHING_PROFILE=preview and =final.

Asserts frame identity (source-only before f1, target-only from f1 in preview), per-job work counters (preview: no full-morph work; final: real work), the exact target at the final job's terminal frame, immutable shared entity definitions, lazy LoD factories and no contamination when the profile switches rapidly inside one process (acceptance case 8).
"""

from __future__ import annotations

import importlib
import json
import os
import tempfile
import unittest
from pathlib import Path

import numpy as np

manim = importlib.import_module("manim")
stage_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
profile_module = importlib.import_module(stage_module.PROFILE_MODULE)

DOCUMENT = Path(__file__).resolve().parent / "🧫️cases" / "🔣️.json"
CONFIG = {"write_to_movie": False, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none", "save_last_frame": False, "pixel_width": 32, "pixel_height": 18}
FPS = 10


def tile(x, y, color=manim.RED, size=1.0):
    """▭ A filled square centred at (x, y)."""
    return manim.Square(size, fill_color=color, fill_opacity=0.9, stroke_color=color).move_to([x, y, 0])


class Definitions:
    """📚 Immutable entity definitions shared by every job: a plain source mapping and a LoD-aware target factory."""

    def __init__(self):
        self.blocks = {"a": tile(-4, -2, manim.BLUE), "b": tile(-2, -2, manim.BLUE)}
        self.tile_bounds = [[2.5, 1.5, 0], [5.5, 2.5, 0]]
        self.tile_part_bounds = {"0": [[2.5, 1.5, 0], [3.5, 2.5, 0]], "1": [[4.5, 1.5, 0], [5.5, 2.5, 0]]}
        self.lods = []

    def tiles(self, lod):
        """🏭 Target factory: the detail grows with the lod but the logical bounds stay."""
        self.lods.append(lod)
        extra = 5 if lod == "full" else 0
        group = manim.VGroup(tile(3, 2), tile(5, 2))
        group.add(*[manim.Dot([3 + i * 0.01, 2, 0], radius=0.01, color=manim.RED) for i in range(extra)])
        return manim.VGroup(manim.VGroup(group[0], *group[2:]), group[1]) if extra else group

    def snapshot(self):
        """📸 Point arrays and colours of the plain definitions."""
        return [(m.points.copy(), m.get_fill_color(), m.get_fill_opacity()) for part in self.blocks.values() for m in part.get_family() if len(m.points)]


def visible_ids(scene):
    """👁 Identities of everything the scene draws."""
    return {id(m) for top in scene.mobjects for m in top.get_family()}


def points_of(mobjects):
    """📍 Sorted rounded points of everything under the given mobjects."""
    group = manim.VGroup(*mobjects)
    pts = np.concatenate([m.points for m in group.get_family() if len(m.points)])
    return np.unique(np.round(pts, 6), axis=0)


def run(profile, definitions, transition="demo.blocks-to-tiles", duration_frames=True):
    """▶ Play one transition in a fresh scene under `profile`; return everything the assertions need."""
    os.environ["SEMIO_TEACHING_PROFILE"] = profile
    profile_module.reset()
    instrument.reset()
    frames = []
    with manim.tempconfig({**CONFIG, "frame_rate": FPS}):
        scene = manim.Scene()
        stage = stage_module.stage(scene, DOCUMENT)
        stage.bind("demo.blocks", definitions.blocks)
        stage.bind("demo.tiles", definitions.tiles, bounds=definitions.tile_bounds, part_bounds=definitions.tile_part_bounds)
        group = stage.mobject("demo.blocks")
        scene.add(group)
        target = stage.mobject("demo.tiles")
        source_ids = {id(m) for m in group.get_family()}
        target_ids = {id(m) for m in target.get_family()}
        if transition.endswith("cut-to-tiles"):
            stage.cut(transition)
        else:
            original = scene.update_to_time

            def spy(t):
                original(t)
                seen = visible_ids(scene)
                frames.append({"source": bool(seen & source_ids), "target": bool(target_ids <= seen), "all": [m for top in scene.mobjects for m in top.get_family() if len(m.points) and (m.get_fill_opacity() > 0.01 or m.get_stroke_opacity() > 0.01)]})

            scene.update_to_time = spy
            scene.play(stage.transition(transition))
    work = instrument.snapshot()
    return {"scene": scene, "frames": frames, "target": target, "work": work, "group": group, "stage": stage}


def owners(frames):
    """🔤 `S`/`T`/`?`/`!` per frame: source only, exact target only, neither, both."""
    return "".join("!" if f["source"] and f["target"] else "S" if f["source"] else "T" if f["target"] else "?" for f in frames)


class StageEndToEndTests(unittest.TestCase):
    """🎭 One real document, both profiles."""

    def setUp(self):
        self.saved = os.environ.get("SEMIO_TEACHING_PROFILE")
        self.addCleanup(self.restore)

    def restore(self):
        if self.saved is None:
            os.environ.pop("SEMIO_TEACHING_PROFILE", None)
        else:
            os.environ["SEMIO_TEACHING_PROFILE"] = self.saved
        profile_module.reset()
        instrument.reset()

    def test_preview_shows_only_the_source_until_f1_and_only_the_target_from_f1(self):
        definitions = Definitions()
        result = run("preview", definitions)
        self.assertEqual(owners(result["frames"]), "S" * (FPS - 1) + "T")
        self.assertEqual(list(result["scene"].mobjects), [result["target"]])
        self.assertEqual(set(definitions.lods), {"preview"})
        self.assertEqual(definitions.lods.count("preview"), 1)

    def test_preview_job_does_no_full_morph_work_and_final_job_does(self):
        definitions = Definitions()
        preview = run("preview", definitions)
        final = run("final", definitions)
        counts = lambda r: r["work"]["counts"]
        self.assertEqual(counts(preview).get("full.prepare", 0), 0)
        self.assertEqual(counts(preview).get("full.evaluate", 0), 0)
        self.assertEqual(counts(preview)["preview.swap"], 1)
        self.assertGreater(counts(final).get("full.prepare", 0), 0)
        self.assertGreater(counts(final).get("full.evaluate", 0), 0)
        self.assertEqual(counts(final).get("preview.swap", 0), 0)
        self.assertEqual((preview["work"]["profile"], final["work"]["profile"]), ("preview", "final"))
        self.assertNotEqual(preview["work"]["fingerprint"], final["work"]["fingerprint"])

    def test_work_json_is_written_per_job(self):
        definitions = Definitions()
        run("preview", definitions)
        with tempfile.TemporaryDirectory() as folder:
            written = json.loads(instrument.write(Path(folder) / "work.json").read_text(encoding="utf-8"))
        self.assertEqual(written["profile"], "preview")
        self.assertEqual(written["counts"].get("full.prepare", 0), 0)

    def test_final_terminal_frame_is_the_exact_target_at_full_lod(self):
        definitions = Definitions()
        result = run("final", definitions)
        self.assertEqual(set(definitions.lods), {"full"})
        self.assertEqual(list(result["scene"].mobjects), [result["target"]])
        last = result["frames"][-1]
        drawn = manim.VGroup(*{id(m): m for m in last["all"]}.values())
        np.testing.assert_allclose(np.array([drawn.get_corner(manim.DL), drawn.get_corner(manim.UR)]), np.array([result["target"].get_corner(manim.DL), result["target"].get_corner(manim.UR)]), atol=1e-6)
        self.assertEqual(owners(result["frames"][:-1]).count("T"), 0)
        np.testing.assert_allclose(points_of(list(drawn.submobjects)), points_of([result["target"]]), atol=1e-6)

    def test_jobs_share_immutable_definitions_without_mutation(self):
        definitions = Definitions()
        before = definitions.snapshot()
        bound_bounds = None
        for profile in ("preview", "final", "preview"):
            result = run(profile, definitions)
            entity = result["stage"].bound("demo.tiles")
            self.assertFalse(entity.bounds.flags.writeable)
            bound_bounds = bound_bounds if bound_bounds is not None else entity.bounds.copy()
            np.testing.assert_array_equal(entity.bounds, bound_bounds)
            np.testing.assert_array_equal(entity.bounds, np.array(definitions.tile_bounds, dtype=float))
        after = definitions.snapshot()
        self.assertEqual(len(before), len(after))
        for (p0, c0, o0), (p1, c1, o1) in zip(before, after):
            np.testing.assert_array_equal(p0, p1)
            self.assertEqual((c0, o0), (c1, o1))

    def test_rapid_profile_switching_has_no_contamination(self):
        definitions = Definitions()
        expected = {}
        for profile in ("preview", "final", "preview", "final", "final", "preview"):
            result = run(profile, definitions)
            counts = result["work"]["counts"]
            self.assertEqual(result["work"]["profile"], profile)
            self.assertEqual(definitions.lods[-1], "preview" if profile == "preview" else "full")
            final_points = points_of([result["target"]])
            key = owners(result["frames"]) if profile == "preview" else None
            if profile == "preview":
                self.assertEqual(counts.get("full.prepare", 0) + counts.get("full.evaluate", 0), 0)
                self.assertEqual(counts["preview.swap"], 1)
            else:
                self.assertGreater(counts.get("full.evaluate", 0), 0)
                self.assertNotIn("preview.swap", counts)
            expected.setdefault(profile, (key, final_points))
            self.assertEqual(expected[profile][0], key)
            np.testing.assert_array_equal(expected[profile][1], final_points)
            self.assertEqual(list(result["scene"].mobjects), [result["target"]])
        self.assertNotEqual(expected["preview"][1].shape, expected["final"][1].shape)

    def test_zero_duration_cut_is_atomic_in_both_profiles(self):
        for profile in ("preview", "final"):
            with self.subTest(profile):
                definitions = Definitions()
                result = run(profile, definitions, "demo.blocks-cut-to-tiles")
                self.assertEqual(list(result["scene"].mobjects), [result["target"]])
                self.assertEqual(result["frames"], [])
                counts = result["work"]["counts"]
                self.assertEqual(counts.get("full.prepare", 0) + counts.get("full.evaluate", 0), 0)
                self.assertEqual(counts.get("preview.swap", 0), 1 if profile == "preview" else 0)

    def test_stage_never_builds_the_final_lod_in_preview(self):
        definitions = Definitions()
        run("preview", definitions)
        self.assertNotIn("full", definitions.lods)


if __name__ == "__main__":
    unittest.main()
