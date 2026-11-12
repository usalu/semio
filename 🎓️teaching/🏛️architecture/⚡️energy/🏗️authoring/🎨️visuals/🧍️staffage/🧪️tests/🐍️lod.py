"""🧊 Staffage levels of detail in the Python bridge: the shared vectors, the resolver, cache isolation per level, the profile-driven level and coverage of every cast a clip draws.

The schedule, revision and role vectors live in `🧍️staffage/🧫️fixtures/🧊️cast-lod/🔣️.json`; the Vitest twin `🧍️staffage/🧪️tests/🧊️cast-lod` runs the same vectors against the TypeScript levels.
"""

import importlib
import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import numpy as np

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
PROFILE = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
INSTRUMENT = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
SUITE = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️")
VECTORS = json.loads((S._SOURCES / "🧫️fixtures/🧊️cast-lod/🔣️.json").read_text(encoding="utf-8"))
TEACHING = next(path for path in Path(__file__).resolve().parents if path.name == "🎓️teaching")
CAST = "cooling-internal-gains-walker"


def setUpModule():
    """🔭 Draw every cast of this module at the full level unless a test names another."""
    S._SCOPE.append(SUITE.LOD)


def tearDownModule():
    S._SCOPE.remove(SUITE.LOD)


def stamped(lod: S.ResolvedLod, frames: int, fps: float) -> dict:
    """🔖 A hand-made index of the walker cast at `lod`, with the provenance the exporter would write."""
    provenance = S.expected_provenance(CAST, lod)
    provenance["asset"]["category"] = "humans"
    clock = {"fps": fps, "durationS": 1.2, "loop": True, "offsetS": 0, "frames": frames}
    return {**SUITE.CASE["index"], "cast": CAST, "fps": fps, "durationS": 1.2, "loop": True, "frames": [f"{n:04d}.svg" for n in range(frames)],
            "lod": {"id": lod.id, "revision": lod.revision, "canonicalFrames": 18, "canonicalFps": 15}, "provenance": {**provenance, "clock": clock}}


class Vectors(unittest.TestCase):
    def test_every_schedule_vector_is_sampled_the_way_the_typescript_twin_samples_it(self):
        for vector in VECTORS["schedules"]:
            with self.subTest(vector["name"]):
                canonical = vector["canonical"]
                schedule = S.lod_schedule(canonical, S.lod_level(vector["lod"], vector["outputFps"]))
                expected = vector["expected"]
                self.assertEqual(schedule["count"], expected["count"])
                self.assertEqual(len(schedule["times"]), expected["count"])
                self.assertAlmostEqual(schedule["fps"], expected["fps"], places=9)
                self.assertAlmostEqual(schedule["times"][0], expected["first"], places=9)
                self.assertAlmostEqual(schedule["times"][-1], expected["last"], places=9)

    def test_the_resolved_levels_have_the_revisions_the_typescript_twin_computes(self):
        for vector in VECTORS["resolved"]:
            with self.subTest(vector):
                lod = S.lod_level(vector["lod"], vector["outputFps"])
                self.assertEqual(lod.fps_cap, vector["fpsCap"])
                self.assertEqual(lod.revision, vector["revision"])
                self.assertEqual(lod.level["roles"], VECTORS["roles"][vector["lod"]])

    def test_an_unknown_level_or_rate_is_refused_with_an_actionable_message(self):
        for vector in VECTORS["invalid"]:
            with self.subTest(vector["name"]), self.assertRaisesRegex(ValueError, vector["message"]):
                S.lod_level(vector["lod"], vector["outputFps"])

    def test_the_levels_file_offers_the_levels_the_profiles_name(self):
        self.assertEqual(sorted(S._levels()), sorted(PROFILE.LODS))


class Cache(unittest.TestCase):
    def test_a_level_never_serves_another(self):
        full, preview, poster = (S.lod_level(name, 15) for name in ("full", "preview", "poster"))
        folders = {S._directory(CAST, lod) for lod in (full, preview, poster, S.lod_level("preview", 30))}
        self.assertEqual(len(folders), 4)
        self.assertEqual(len({folder.parent for folder in folders}), 1)
        locks = {S._lock(CAST, lod).path for lod in (full, preview, poster)}
        self.assertEqual(len(locks), 3)

    def test_an_index_of_another_level_is_refused(self):
        full, preview = S.lod_level("full", 15), S.lod_level("preview", 15)
        index = stamped(preview, 9, 7.5)
        S._provenance(CAST, index, preview)
        with self.assertRaisesRegex(ValueError, "provenance lod"):
            S._provenance(CAST, index, full)

    def test_a_level_that_sampled_a_different_number_of_frames_is_refused(self):
        preview = S.lod_level("preview", 15)
        with self.assertRaisesRegex(ValueError, "samples 9 frames"):
            S._provenance(CAST, stamped(preview, 12, 7.5), preview)

    def test_the_exporter_is_asked_for_the_level_and_the_rate_and_the_load_is_counted(self):
        calls = []
        preview = S.lod_level("preview", 15)

        def fake(command, progress=None, **_):
            calls.append(command)
            out = Path(command[command.index("--out") + 1])
            for name in command[command.index("--ids") + 1].split(","):
                (out / name).mkdir(parents=True)
                (out / name / S._INDEX).write_text(json.dumps(stamped(preview, 9, 7.5)), encoding="utf-8")

        INSTRUMENT.reset()
        with tempfile.TemporaryDirectory() as root, patch.dict(os.environ, {"SEMIO_TEACHING_CACHE_ROOT": root}), patch.multiple(S, _run=fake, _repo=lambda: "repo"):
            folders = S.ensure_casts([CAST], None, preview)
            S.ensure_casts([CAST], None, preview)
        self.assertEqual(len(calls), 1)
        command = calls[0]
        self.assertEqual(command[command.index("--lod") + 1], "preview")
        self.assertEqual(float(command[command.index("--output-fps") + 1]), 15.0)
        self.assertTrue(folders[CAST].name.startswith("preview-"))
        self.assertEqual(INSTRUMENT.snapshot.__module__, INSTRUMENT.__name__)
        self.assertEqual(INSTRUMENT._COUNTS.get("asset.load[preview]"), 1)
        self.assertNotIn("asset.load[full]", INSTRUMENT._COUNTS)

    def test_the_slot_count_reads_the_index_and_no_frame(self):
        with tempfile.TemporaryDirectory() as root:
            folder = SUITE.synthetic(Path(root))
            for frame in folder.glob("*.svg"):
                frame.unlink()
            S._INDEXES.pop(folder, None)
            self.assertEqual(S._slot_count(folder), SUITE.CASE["index"]["logical"]["maxPaths"])


class Level(unittest.TestCase):
    def test_the_level_comes_from_the_render_profile_at_the_job_rate(self):
        S._SCOPE.clear()
        try:
            for name, fps, expected in (("preview", "15", S.lod_level("preview", 15)), ("final", "30", S.lod_level("full", 30))):
                with patch.dict(os.environ, {PROFILE.PROFILE_ENV: name, PROFILE.FPS_ENV: fps}):
                    PROFILE.reset()
                    try:
                        self.assertEqual(S.active_lod(), expected)
                    finally:
                        PROFILE.reset()
        finally:
            S._SCOPE.append(SUITE.LOD)

    def test_without_a_profile_there_is_no_level_and_the_error_says_so(self):
        environment = {key: value for key, value in os.environ.items() if key != PROFILE.PROFILE_ENV}
        with patch.dict(os.environ, environment, clear=True):
            PROFILE.reset()
            S._SCOPE.clear()
            try:
                with self.assertRaisesRegex(PROFILE.ProfileError, "no render profile is active"):
                    S.active_lod()
            finally:
                S._SCOPE.append(SUITE.LOD)
                PROFILE.reset()

    def test_a_scope_names_the_level_of_its_block_only(self):
        with S.lod_scope("poster", 15) as poster:
            self.assertEqual(S.active_lod(), poster)
            with S.lod_scope("preview", 30) as inner:
                self.assertEqual(S.active_lod(), inner)
            self.assertEqual(S.active_lod(), poster)
        self.assertEqual(S.active_lod(), SUITE.LOD)


class Resolver(unittest.TestCase):
    def resolved(self, **declared):
        with tempfile.TemporaryDirectory() as root:
            folder = SUITE.synthetic(Path(root))
            with SUITE.serving(folder):
                return S.resolve(CAST, **declared)

    def test_the_descriptor_is_the_logical_truth_of_the_index_without_decoding_a_frame(self):
        result = self.resolved()
        logical = SUITE.CASE["index"]["logical"]
        self.assertEqual(result.descriptor["heightM"], logical["heightM"])
        self.assertEqual(result.descriptor["figureBoundsM"], logical["figureBoundsM"])
        self.assertEqual(result.descriptor["widthM"], 5.0)
        self.assertEqual(result.descriptor["partsM"], logical["partsM"])
        self.assertEqual(result.lod, SUITE.LOD)
        self.assertEqual(result.frame_count, len(SUITE.CASE["frames"]))

    def test_declared_axes_that_match_the_manifest_resolve(self):
        entry = S._entry(CAST)
        result = self.resolved(view=entry["view"], viewpoint=entry["viewpoint"], scale=f"1:{entry['scale']}", pose=entry["clip"], scenario="thermal", revision=S.asset_revision(entry["asset"]))
        self.assertEqual(result.cast, CAST)

    def test_an_axis_the_manifest_does_not_hold_names_the_casts_that_do(self):
        for declared in ({"view": "forces"}, {"scale": "1:200"}, {"pose": "run-cycle"}, {"scenario": "structural"}, {"viewpoint": "plan"}):
            with self.subTest(declared), self.assertRaisesRegex(S.ResolveError, "does not provide"):
                self.resolved(**declared)
        with self.assertRaisesRegex(S.ResolveError, r"matching the declared axes: none; every cast of the asset: .*cooling-internal-gains-walker \(thermal/elevation 1:50 walk-cycle\)"):
            self.resolved(pose="walk-cycle", view="thermal", scale="1:200")

    def test_a_stale_asset_revision_is_refused(self):
        with self.assertRaisesRegex(S.ResolveError, "authored against asset revision"):
            self.resolved(revision="sha256:" + "0" * 64)

    def test_an_unknown_cast_names_the_known_ones(self):
        with self.assertRaisesRegex(KeyError, "Unknown cast"):
            S.resolve("nobody", lod="preview", output_fps=15)

    def test_a_level_without_an_output_rate_is_refused(self):
        with self.assertRaisesRegex(S.ResolveError, "output frame rate"):
            S.resolve(CAST, lod="preview")


class Binder(unittest.TestCase):
    class Stage:
        def __init__(self):
            self.calls = []

        def bind(self, entity_id, value, **options):
            self.calls.append((entity_id, value, options))
            return self

    def test_the_staffage_binder_freezes_the_logical_bounds_and_binds_a_factory_per_level(self):
        stage_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")
        self.assertIs(stage_module.Stage.binders["staffage"], S.bind_staffage)
        with tempfile.TemporaryDirectory() as root, patch.dict(os.environ, {PROFILE.PROFILE_ENV: "preview", PROFILE.FPS_ENV: "15"}):
            PROFILE.reset()
            try:
                folder = SUITE.synthetic(Path(root))
                stage = self.Stage()
                with SUITE.serving(folder):
                    S.bind_staffage(stage, "tower.worker", CAST, unit=0.5, anchor=np.array([1.0, 2.0, 0.0]), rate=1.0)
                    (entity, factory, options), = stage.calls
                    box = SUITE.CASE["index"]["logical"]["figureBoundsM"]
                    np.testing.assert_allclose(options["bounds"], [[1.0 + box["minX"] * 0.5, 2.0 + box["minY"] * 0.5, 0.0], [1.0 + box["maxX"] * 0.5, 2.0 + box["maxY"] * 0.5, 0.0]])
                    np.testing.assert_allclose(options["anchors"]["origin"], [1.0, 2.0, 0.0])
                    self.assertIn("part:slat-0", options["anchors"])
                    self.assertEqual(entity, "tower.worker")
                    self.assertIn("1:50", options["provides"]["scale"])
                    self.assertIn("thermal", options["provides"]["scenario"])
                    built = factory("preview")
                self.assertEqual(len(S._index(folder)["frames"]), len(built.frames))
            finally:
                PROFILE.reset()


class Coverage(unittest.TestCase):
    def test_every_cast_a_clip_draws_resolves_at_every_level_to_its_own_folder(self):
        sources = sorted(path for path in (TEACHING / "🏛️architecture").rglob("🐍️.py") if "🎬️clip" in path.parts and "🧪️tests" not in path.parts)
        named = S.casts_in(sources)
        self.assertGreater(len(named), 50)
        for cast in named:
            entry = S._entry(cast)
            self.assertEqual(S.entry_issues(entry), [], cast)
            folders = {S._directory(cast, S.lod_level(level, 15)) for level in S._levels()}
            self.assertEqual(len(folders), len(S._levels()), cast)
            axes = S._axes(entry)
            self.assertTrue(all(axes[axis] for axis in axes), cast)

    def test_no_clip_names_a_level_of_detail(self):
        for path in (TEACHING / "🏛️architecture").rglob("🐍️.py"):
            if "🎬️clip" not in path.parts or "🧪️tests" in path.parts:
                continue
            source = path.read_text(encoding="utf-8")
            for forbidden in ("lod_scope", "lod_level", "active_lod", "output_fps"):
                self.assertNotIn(forbidden, source, f"{path} names the level of detail through {forbidden}; the render profile chooses it")


if __name__ == "__main__":
    unittest.main()
