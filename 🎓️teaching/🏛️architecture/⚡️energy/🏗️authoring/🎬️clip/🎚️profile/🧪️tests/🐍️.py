"""🧪 Render profiles: schema, explicit activation, fingerprints, cache isolation, CLI and counters."""

from __future__ import annotations

import hashlib
import importlib
import io
import json
import sys
import tempfile
import unittest
from contextlib import redirect_stderr
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

CLIP = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
profile = importlib.import_module(CLIP + ".🎚️profile.🐍️")
cache = importlib.import_module(CLIP + ".🎚️profile.🗃️cache.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
HERE = Path(__file__).resolve().parent
CASES = json.loads((HERE / "🔣️.cases.json").read_text(encoding="utf-8"))
DOCUMENT = json.loads(HERE.parent.joinpath("🔣️.json").read_text(encoding="utf-8"))
SCHEMA = json.loads(HERE.parent.joinpath("🔣️.schema.json").read_text(encoding="utf-8"))


class ProfileCases(unittest.TestCase):
    def setUp(self):
        profile.reset()
        instrument.reset()
        self.addCleanup(profile.reset)
        self.addCleanup(instrument.reset)

    def test_schema_accepts_document_and_rejects_mutations(self):
        try:
            import jsonschema
        except ImportError:
            self.skipTest("jsonschema oracle not installed")
        validator = jsonschema.Draft202012Validator(SCHEMA)
        self.assertEqual(list(validator.iter_errors(DOCUMENT)), [])
        for mutate in (lambda d: d["profiles"].pop("final"), lambda d: d["profiles"]["preview"]["output"].update(width=855),
                       lambda d: d["profiles"]["preview"]["budgets"].update(particles=0), lambda d: d["profiles"]["final"].update(morph="blend"),
                       lambda d: d["profiles"]["final"].update(extra=1)):
            broken = json.loads(json.dumps(DOCUMENT))
            mutate(broken)
            self.assertTrue(list(validator.iter_errors(broken)))

    def test_active_requires_an_explicit_known_profile(self):
        for name in ("missing", "unknown"):
            case = CASES["environment"][name]
            with patch.dict("os.environ", case["env"], clear=True), self.assertRaises(profile.ProfileError) as error:
                profile.active()
            self.assertIn(case["message"], str(error.exception))

    def test_profiles_match_document_and_are_frozen(self):
        for name in ("preview", "final"):
            case = CASES["environment"][name]
            with patch.dict("os.environ", {"SEMIO_TEACHING_PROFILE": name}, clear=True):
                chosen = profile.active()
            entry = DOCUMENT["profiles"][name]
            self.assertEqual((chosen.morph, chosen.lod), (entry["morph"], entry["lod"]))
            self.assertEqual((chosen.width, chosen.height, chosen.fps), (entry["output"]["width"], entry["output"]["height"], entry["output"]["fps"]))
            self.assertEqual(chosen.budgets.curve_samples, entry["budgets"]["curveSamples"])
            limit = entry["budgets"]["particles"]
            self.assertEqual(chosen.cap("particles", 10**6), 10**6 if limit is None else limit)
            self.assertEqual(chosen.cap("particles", 3), 3)
            with self.assertRaises(Exception):
                chosen.morph = "full"
        self.assertEqual((profile.profile("preview").morph, profile.profile("final").morph), ("swap", "full"))
        self.assertTrue(all(value is None for value in vars(profile.profile("final").budgets).values()))
        self.assertEqual(profile.profile("preview").height, 480)
        self.assertEqual((profile.profile("final").width, profile.profile("final").fps), (1920, 30))

    def test_fingerprint_hashes_the_profile_entry_and_differs(self):
        digests = {}
        for name in ("preview", "final"):
            canonical = json.dumps(DOCUMENT["profiles"][name], sort_keys=True, separators=(",", ":"), ensure_ascii=False)
            digests[name] = hashlib.sha256(canonical.encode("utf-8")).hexdigest()
            self.assertEqual(profile.fingerprint(name), digests[name])
        self.assertNotEqual(*digests.values())

    def test_job_is_created_once_and_honours_environment(self):
        for name in ("preview", "final"):
            profile.reset()
            case = CASES["environment"][name]
            with patch.dict("os.environ", case["env"], clear=True):
                first = profile.job()
                self.assertIs(profile.job(), first)
            self.assertEqual((first.profile.name, first.fps, first.width, first.height), (name, case["fps"], case["width"], case["height"]))
            self.assertEqual(first.id, case["env"]["SEMIO_TEACHING_JOB"])

    def test_environment_round_trips_through_job(self):
        env = profile.environment("final", job_id="x", fps=12, resolution="640,360")
        with patch.dict("os.environ", env, clear=True):
            job = profile.job()
        self.assertEqual((job.profile.name, job.id, job.fps, job.width, job.height), ("final", "x", 12.0, 640, 360))

    def test_cache_keys_isolate_profiles(self):
        for parts in CASES["keys"]:
            preview, final = profile.cache_key("preview", *parts), profile.cache_key("final", *parts)
            self.assertNotEqual(preview, final)
            self.assertEqual(preview, profile.cache_key("preview", *parts))
            with patch.dict("os.environ", {"SEMIO_TEACHING_PROFILE": "preview"}, clear=True):
                profile.reset()
                self.assertEqual(profile.job().key(*parts), preview)

    def test_verify_fails_loudly_when_manim_disagrees(self):
        with patch.dict("os.environ", CASES["environment"]["preview"]["env"], clear=True):
            job = profile.job()
        job.verify(SimpleNamespace(frame_rate=15, pixel_width=854, pixel_height=480))
        with self.assertRaises(profile.ProfileError):
            job.verify(SimpleNamespace(frame_rate=60, pixel_width=1920, pixel_height=1080))
        config = SimpleNamespace(frame_rate=60, pixel_width=1920, pixel_height=1080)
        job.apply(config)
        job.verify(config)

    def test_bounded_cache_enforces_entries_and_bytes(self):
        case = CASES["cache"]["entries"]
        entries = cache.BoundedCache("t", max_entries=case["max_entries"])
        for key in case["puts"]:
            entries.put(key, key)
        self.assertEqual([key for key in "abc" if entries.get(key) is not None], case["kept"])
        case = CASES["cache"]["bytes"]
        sized = cache.BoundedCache("t", max_entries=case["max_entries"], max_bytes=case["max_bytes"], sizeof=lambda value: value)
        for key, size in case["puts"]:
            sized.put(key, size)
        self.assertEqual([key for key, _ in case["puts"] if sized.get(key) is not None], case["kept"])
        self.assertLessEqual(sized.stats()["bytes"], case["max_bytes"])
        with self.assertRaises(ValueError):
            cache.BoundedCache("t", max_entries=1, max_bytes=5)
        with self.assertRaises(ValueError):
            cache.BoundedCache("t", max_entries=1, sizeof=len)

    def test_counters_snapshot_and_write(self):
        for name, amount in CASES["counts"]["calls"]:
            instrument.count(name, amount)
        with patch.dict("os.environ", CASES["environment"]["preview"]["env"], clear=True):
            snapshot = instrument.snapshot()
            with tempfile.TemporaryDirectory() as directory:
                written = json.loads(instrument.write(Path(directory) / "s" / "work.json").read_text(encoding="utf-8"))
        self.assertEqual(snapshot, written)
        self.assertEqual((snapshot["job"], snapshot["profile"], snapshot["counts"]), ("job-a", "preview", CASES["counts"]["expected"]))
        self.assertEqual(snapshot["fingerprint"], profile.fingerprint("preview"))

    def test_cache_events_are_counted(self):
        entries = cache.BoundedCache("n", max_entries=1)
        entries.put("a", 1)
        entries.get("a")
        entries.put("b", 2)
        with patch.dict("os.environ", CASES["environment"]["preview"]["env"], clear=True):
            counts = instrument.snapshot()["counts"]
        self.assertEqual(counts, {"cache.create[n]": 2, "cache.evict[n]": 1, "cache.hit[n]": 1})


class DensityCases(unittest.TestCase):
    visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")

    def test_budgets_drive_shared_effect_density(self):
        for case in CASES["density"]:
            for name in ("final", "preview"):
                with patch.dict("os.environ", {"SEMIO_TEACHING_PROFILE": name}, clear=True):
                    self.assertEqual(self.visuals.effect_density(case["budget"], case["authored"], case["units"]), case[name], (case, name))

    def test_effect_density_requires_an_explicit_profile(self):
        with patch.dict("os.environ", {}, clear=True), self.assertRaises(profile.ProfileError):
            self.visuals.effect_density("particles", 5, 1)


class RuntimeCases(unittest.TestCase):
    runtime = importlib.import_module(CLIP + ".⚙️runtime.🐍️")

    def test_provenance_never_crosses_profiles(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            source = folder / "scene.py"
            source.write_text("authored", encoding="utf-8")
            base = {"language": "de", "audience": "speaker", "resolution": None, "fps": None}
            states = {name: self.runtime.clip_state({**base, "profile": name}, [source]) for name in ("preview", "final")}
            self.assertEqual({states[name]["profile"] for name in states}, {profile.fingerprint("preview"), profile.fingerprint("final")})
            clip = folder / "clip.mp4"
            clip.write_bytes(b"0" * 1200)
            clip.with_suffix(".render.json").write_text(json.dumps(states["preview"]), encoding="utf-8")
            self.assertTrue(self.runtime.reusable_clip(clip, states["preview"]))
            self.assertFalse(self.runtime.reusable_clip(clip, states["final"]))
            with self.assertRaises(KeyError):
                self.runtime.clip_state({**base}, [source])

    def test_compose_rejects_unknown_profile(self):
        with self.assertRaises(ValueError):
            self.runtime.compose_videos(language="de", audience="speaker", profile="draft")


class CommandLineCases(unittest.TestCase):
    render = importlib.import_module(CLIP + ".🖌️render.🐍️")

    def test_missing_old_and_unknown_profile_flags_are_rejected(self):
        for name, arguments in CASES["cli"].items():
            with patch.object(self.render, "compose_videos") as compose, redirect_stderr(io.StringIO()) as stderr, self.assertRaises(SystemExit) as exit_:
                self.render.main(arguments)
            self.assertEqual(exit_.exception.code, 2, name)
            compose.assert_not_called()
        with patch.object(self.render, "compose_videos"), redirect_stderr(io.StringIO()) as stderr, self.assertRaises(SystemExit):
            self.render.main(CASES["cli"]["missing_profile"])
        self.assertIn("--profile", stderr.getvalue())

    def test_profile_reaches_the_runtime(self):
        for name in ("preview", "final"):
            with patch.object(self.render, "compose_videos") as compose:
                self.assertEqual(self.render.main(["--language", "de", "--profile", name, "--audience", "speaker", "--no-play"]), 0)
            self.assertEqual(compose.call_args.kwargs["profile"], name)


if __name__ == "__main__":
    unittest.main()
