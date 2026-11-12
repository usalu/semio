"""🧪 Export jobs: profile isolation, provenance, work counters and canonical stills.

Language-agnostic cases live in `🔣️.cases.json`; jsonschema validates provenance and PyAV decodes the exported movies as independent oracles.
"""

from __future__ import annotations

import importlib
import json
import tempfile
import unittest
from pathlib import Path

import numpy as np

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
export = importlib.import_module(PACKAGE + ".📤️export.🐍️")
stills = importlib.import_module(PACKAGE + ".📤️export.🖼️stills.🐍️")
profile = importlib.import_module(PACKAGE + ".🎚️profile.🐍️")
runtime = importlib.import_module(PACKAGE + ".⚙️runtime.🐍️")
timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")

HERE = Path(__file__).parent
CASES = json.loads((HERE / "🔣️.cases.json").read_text(encoding="utf-8"))
SCHEMA = json.loads((HERE.parent / "🧬️schema" / "🔣️.schema.json").read_text(encoding="utf-8"))
SOURCE, TARGET = (220, 30, 30), (30, 30, 220)


def resolve(value):
    if isinstance(value, dict):
        return {key: resolve(item) for key, item in value.items()}
    if isinstance(value, list):
        return [resolve(item) for item in value]
    if isinstance(value, str) and value.startswith("$"):
        return profile.fingerprint(value[1:])
    return value


def certify(movie: Path, name: str) -> None:
    movie.with_suffix(".render.json").write_text(json.dumps({"settings": {"profile": name}, "profile": profile.fingerprint(name), "sources": {}}), encoding="utf-8")


def synthetic_movie(folder: Path, name: str, *, fps: int, start: float, duration: float, total: int, shade: bool = False) -> Path:
    """🎞️ Encode a lossless movie whose frame colour is the shared evaluator's owner for that frame."""
    from PIL import Image

    window, schedule = timeline.Window(start, duration), timeline.Schedule(fps)
    frames = folder / "frames"
    frames.mkdir()
    for frame in range(total):
        colour = TARGET if timeline.owner(window, schedule, frame) == "target" else SOURCE
        if shade:
            colour = tuple(min(255, value + frame % 7) for value in colour)
        Image.new("RGB", (64, 36), colour).save(frames / f"{frame:04d}.png")
    movie = folder / f"{name}.mp4"
    runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-framerate", str(fps), "-i", str(frames / "%04d.png"),
                         "-c:v", "libx264", "-crf", "0", "-pix_fmt", "yuv444p", "-r", str(fps), str(movie)])
    return movie


def decoded(movie: Path, index: int) -> np.ndarray:
    import av

    with av.open(str(movie)) as container:
        for position, frame in enumerate(container.decode(video=0)):
            if position == index:
                return frame.to_ndarray(format="rgb24")
    raise AssertionError("frame beyond the movie")


def image(path: Path) -> np.ndarray:
    from PIL import Image

    with Image.open(path) as picture:
        return np.asarray(picture.convert("RGB"))


class PosterPolicyTest(unittest.TestCase):
    def test_cases(self):
        for case in CASES["poster"]:
            windows = {key: tuple(value) for key, value in case.get("windows", {}).items()} or None
            with self.subTest(case["name"]):
                if "error" in case:
                    with self.assertRaisesRegex(stills.StillError, case["error"]):
                        stills.poster_frame(case["policy"], frames=case["frames"], fps=case["fps"], windows=windows)
                else:
                    self.assertEqual(stills.poster_frame(case["policy"], frames=case["frames"], fps=case["fps"], windows=windows), case["expect"])

    def test_terminal_matches_numpy_frame_ownership(self):
        for fps in (15, 24, 25, 30, 60):
            for start, duration in ((0.0, 1.0), (3.2, 0.6), (7.0, 2.4)):
                times = np.arange(0, duration, 1 / fps)
                first = round(start * fps)
                oracle = first + len(times) - 1
                got = stills.poster_frame({"kind": "transition", "id": "t"}, frames=10_000, fps=fps, windows={"t": (start, duration)})
                with self.subTest(fps=fps, start=start, duration=duration):
                    self.assertEqual(got, oracle)


class ProfileIsolationTest(unittest.TestCase):
    def test_cases(self):
        for case in CASES["profileStates"]:
            with self.subTest(case["name"]):
                state = resolve(case["state"])
                if case["accept"]:
                    export.require_state_profile(state, case["job"])
                else:
                    with self.assertRaisesRegex(export.ProfileMismatch, case["message"]):
                        export.require_state_profile(state, case["job"])

    def test_final_job_rejects_preview_partials_and_outputs(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            partial = folder / "Beat1.mp4"
            partial.write_bytes(b"x" * 2000)
            certify(partial, "preview")
            with self.assertRaisesRegex(export.ProfileMismatch, "preview but this is a final job"):
                export.require_profile(partial, "final")
            with self.assertRaises(export.ProfileMismatch):
                runtime.concat_videos([partial], folder / "out.mp4", folder / "list.txt", profile="final")
            self.assertFalse((folder / "out.mp4").exists())
            export.require_profile(partial, "preview")

    def test_uncertified_partial_is_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            partial = Path(directory) / "Beat1.mp4"
            partial.write_bytes(b"x" * 2000)
            with self.assertRaisesRegex(export.ProfileMismatch, "no render certificate"):
                export.require_profile(partial, "final")

    def test_stills_refuse_other_profile(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            movie = synthetic_movie(folder, "m", fps=10, start=0.5, duration=0.4, total=12)
            certify(movie, "preview")
            with self.assertRaises(export.ProfileMismatch):
                stills.make_stills(movie, folder / "out", "m", profile="final")


class WorkTest(unittest.TestCase):
    def test_cases(self):
        for case in CASES["work"]:
            with self.subTest(case["name"]):
                work = {"jobs": 1, "counts": case["counts"], "sources": []}
                if case["ok"]:
                    export.verify_work(case["job"], work, transitions=case["transitions"])
                else:
                    with self.assertRaisesRegex(export.WorkMismatch, case["message"]):
                        export.verify_work(case["job"], work, transitions=case["transitions"])

    def write_work(self, folder: Path, name: str, counts: dict, job="j", fingerprint=None) -> Path:
        folder.mkdir(parents=True, exist_ok=True)
        (folder / "work.json").write_text(json.dumps({"job": job, "profile": name, "fingerprint": fingerprint or profile.fingerprint(name), "counts": counts}), encoding="utf-8")
        return folder

    def test_collect_sums_and_rejects_contamination(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first = self.write_work(root / "1", "final", {"full.evaluate": 5, "full.prepare": 1}, job="a")
            second = self.write_work(root / "2", "final", {"full.evaluate": 7}, job="b")
            total = export.collect_work([first, second, first], "final")
            self.assertEqual(total["counts"], {"full.evaluate": 12, "full.prepare": 1})
            self.assertEqual(total["jobs"], 2)
            self.assertEqual(len(total["sources"]), 2)
            stray = self.write_work(root / "3", "preview", {"preview.swap": 2})
            with self.assertRaises(export.ProfileMismatch):
                export.collect_work([first, stray], "final")
            with self.assertRaisesRegex(export.WorkMismatch, "Missing work.json"):
                export.collect_work([root / "absent"], "final")

    def test_declared_transitions(self):
        with tempfile.TemporaryDirectory() as directory:
            script = Path(directory) / "clip" / "🐍️.py"
            script.parent.mkdir()
            self.assertEqual(export.declared_transitions(script), 0)
            (script.parent / "🔀️transitions").mkdir()
            (script.parent / "🔀️transitions" / "🔣️.json").write_text(json.dumps({"version": 1, "transitions": [{"id": "a"}, {"id": "b"}]}), encoding="utf-8")
            self.assertEqual(export.declared_transitions(script), 2)


class ProvenanceTest(unittest.TestCase):
    def movie(self, folder: Path) -> Path:
        movie = folder / "lesson.mp4"
        movie.write_bytes(b"movie" * 400)
        return movie

    def record(self, movie: Path, name: str, **extra) -> dict:
        work = {"jobs": 1, "counts": {"full.evaluate": 3} if name == "final" else {"preview.swap": 1}, "sources": [{"file": "s/1/work.json", "job": "j", "counts": {"preview.swap": 1}}]}
        return export.export_record("master", movie, profile=name, language="en", audience="speaker", work=work, inputs=[movie.with_name("a.mp4")], **extra)

    def test_record_contains_profile_and_matches_schema(self):
        import jsonschema

        with tempfile.TemporaryDirectory() as directory:
            movie = self.movie(Path(directory))
            for name in ("preview", "final"):
                record = self.record(movie, name)
                jsonschema.Draft202012Validator(SCHEMA).validate(record)
                chosen = profile.profile(name)
                self.assertEqual(record["profile"], {"name": name, "fingerprint": chosen.fingerprint, "morph": chosen.morph, "lod": chosen.lod})
                self.assertEqual((record["render"]["width"], record["render"]["height"], record["render"]["fps"]), (chosen.width, chosen.height, chosen.fps))
                self.assertEqual(record["render"]["budgets"]["curveSamples"], chosen.budgets.curve_samples)
                path = export.publish_record(movie, record)
                self.assertEqual(path.name, "lesson.export.json")
                self.assertEqual(export.read_record(movie), record)
                self.assertEqual(export.require_record_profile(movie, name)["profile"]["name"], name)

    def test_overrides_are_recorded(self):
        with tempfile.TemporaryDirectory() as directory:
            record = self.record(self.movie(Path(directory)), "final", resolution="1280,720", fps=24.0, seed=7)
            self.assertEqual(record["render"]["resolutionOverride"], "1280,720")
            self.assertEqual((record["render"]["width"], record["render"]["height"], record["render"]["fps"], record["render"]["fpsOverride"], record["render"]["seed"]), (1280, 720, 24.0, 24.0, 7))

    def test_preview_record_never_satisfies_final(self):
        with tempfile.TemporaryDirectory() as directory:
            movie = self.movie(Path(directory))
            export.publish_record(movie, self.record(movie, "preview"))
            with self.assertRaisesRegex(export.ProfileMismatch, "preview export but this is a final job"):
                export.require_record_profile(movie, "final")

    def test_tampering_and_stale_fingerprint_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            movie = self.movie(Path(directory))
            record = self.record(movie, "final")
            export.publish_record(movie, record)
            movie.write_bytes(b"other" * 400)
            with self.assertRaisesRegex(export.ProfileMismatch, "changed after"):
                export.read_record(movie)
            stale = {**record, "profile": {**record["profile"], "fingerprint": "0" * 64}}
            with self.assertRaisesRegex(ValueError, "stale fingerprint"):
                export.publish_record(movie, stale)
            mixed = {**record, "inputs": [{**record["inputs"][0], "profile": "preview"}]}
            with self.assertRaisesRegex(ValueError, "inputs must share"):
                export.publish_record(movie, mixed)


class PublishExportTest(unittest.TestCase):
    def chapter(self, folder: Path, name: str, scene: str, profile_name: str, counts: dict) -> Path:
        clip = folder / f"{scene}.mp4"
        clip.write_bytes(b"clip" * 400)
        export.write_work(clip, {"jobs": 1, "counts": counts, "sources": [{"file": f"s/{scene}/work.json", "job": scene, "counts": counts}]}, profile_name)
        return clip

    def settings(self, name: str) -> dict:
        return {"language": "en", "audience": "speaker", "profile": name, "resolution": None, "fps": None}

    def test_master_provenance_sums_chapter_work_and_posters_the_profile_frame(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            script = folder / "clip" / "🐍️.py"
            (script.parent / "🔀️transitions").mkdir(parents=True)
            (script.parent / "🔀️transitions" / "🔣️.json").write_text(json.dumps({"version": 1, "transitions": [{"id": "a"}]}), encoding="utf-8")
            first = self.chapter(folder, "a", "A", "final", {"full.prepare": 1, "full.evaluate": 30})
            second = self.chapter(folder, "b", "B", "final", {"full.evaluate": 12})
            master = synthetic_movie(folder, "master", fps=30, start=1.0, duration=0.5, total=90)
            certify(master, "final")
            record = export.publish_export("master", master, settings=self.settings("final"), movies=[first, second], scripts=[script, script], inputs=[first, second],
                                           stills={"folder": folder / "posters", "name": "master", "policy": {"kind": "transition", "id": "a"}}, windows={"a": (1.0, 0.5)})
            self.assertEqual(record["work"]["counts"], {"full.evaluate": 42, "full.prepare": 1})
            self.assertEqual((record["work"]["jobs"], len(record["work"]["sources"])), (2, 2))
            self.assertEqual(record["profile"]["name"], "final")
            self.assertEqual(record["stills"]["frame"], 44)
            self.assertEqual(record["stills"]["profile"], record["profile"])
            self.assertEqual(export.read_record(master), record)
            self.assertEqual(image(folder / "posters" / record["stills"]["poster"]["file"]).shape[:2], (1080, 1920))

    def test_final_export_without_full_work_is_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            script = folder / "clip" / "🐍️.py"
            (script.parent / "🔀️transitions").mkdir(parents=True)
            (script.parent / "🔀️transitions" / "🔣️.json").write_text(json.dumps({"version": 1, "transitions": [{"id": "a"}]}), encoding="utf-8")
            clip = self.chapter(folder, "a", "A", "final", {"preview.swap": 3})
            master = folder / "master.mp4"
            master.write_bytes(b"movie" * 400)
            with self.assertRaisesRegex(export.WorkMismatch, "ran no full morph"):
                export.publish_export("master", master, settings=self.settings("final"), movies=[clip], scripts=[script])
            self.assertFalse(master.with_suffix(".export.json").exists())

    def test_preview_export_with_full_work_is_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            clip = self.chapter(folder, "a", "A", "preview", {"full.evaluate": 3})
            master = folder / "master.mp4"
            master.write_bytes(b"movie" * 400)
            with self.assertRaisesRegex(export.WorkMismatch, "full-morph work"):
                export.publish_export("chapter", master, settings=self.settings("preview"), movies=[clip], scripts=[])

    def test_final_export_never_consumes_preview_assembled_work(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            clip = self.chapter(folder, "a", "A", "preview", {"preview.swap": 3})
            master = folder / "master.mp4"
            master.write_bytes(b"movie" * 400)
            with self.assertRaisesRegex(export.ProfileMismatch, "preview but this is a final job"):
                export.publish_export("master", master, settings=self.settings("final"), movies=[clip], scripts=[])
            with self.assertRaisesRegex(export.WorkMismatch, "Missing"):
                export.publish_export("master", master, settings=self.settings("preview"), movies=[folder / "nowhere.mp4"], scripts=[])


class StillsTest(unittest.TestCase):
    def test_poster_at_terminal_time_is_the_evaluators_target_state(self):
        for fps, start, duration in ((10, 1.0, 1.0), (15, 0.8, 1.2), (30, 0.5, 0.2)):
            with self.subTest(fps=fps), tempfile.TemporaryDirectory() as directory:
                folder = Path(directory)
                movie = synthetic_movie(folder, "m", fps=fps, start=start, duration=duration, total=fps * 3)
                certify(movie, "preview")
                window, schedule = timeline.Window(start, duration), timeline.Schedule(fps)
                f1 = timeline.terminal(window, schedule)
                record = stills.make_stills(movie, folder / "out", "m", profile="preview", policy={"kind": "transition", "id": "t"}, windows={"t": (start, duration)}, size=(64, 36))
                self.assertEqual(record["frame"], f1)
                self.assertEqual(record["profile"]["name"], "preview")
                poster = image(folder / "out" / record["poster"]["file"])
                self.assertLessEqual(np.abs(poster.astype(int) - np.array(TARGET)).max(), 3)
                before = image(stills.extract(movie, f1 - 1, folder / "before.png"))
                self.assertLessEqual(np.abs(before.astype(int) - np.array(SOURCE)).max(), 3)
                self.assertEqual(timeline.owner(window, schedule, f1 - 1), "source")
                np.testing.assert_array_equal(poster, decoded(movie, f1))

    def test_extract_equals_pyav_for_every_frame_class(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            movie = synthetic_movie(folder, "m", fps=25, start=0.4, duration=0.4, total=75, shade=True)
            for index in (0, 1, 2, 24, 37, 49, 73, 74):
                with self.subTest(index=index):
                    np.testing.assert_array_equal(image(stills.extract(movie, index, folder / f"{index}.png")), decoded(movie, index))
            with self.assertRaises(stills.StillError):
                stills.extract(movie, 75, folder / "x.png")

    def test_poster_is_not_the_last_delivered_frame(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            movie = synthetic_movie(folder, "m", fps=15, start=1.0, duration=0.4, total=45, shade=True)
            certify(movie, "preview")
            record = stills.make_stills(movie, folder / "out", "m", profile="preview", size=(64, 36))
            self.assertEqual((record["frame"], record["frames"]), (22, 45))
            np.testing.assert_array_equal(image(folder / "out" / record["poster"]["file"]), decoded(movie, 22))
            self.assertEqual(record["thumbnail"]["width"], 320)
            self.assertTrue(stills.stills_current(record, movie, folder / "out", profile="preview"))
            self.assertFalse(stills.stills_current(record, movie, folder / "out", profile="preview", policy={"kind": "fraction", "value": 1}))
            self.assertFalse(stills.stills_current(record, movie, folder / "out", profile="final"))

    def test_poster_size_follows_the_profile(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            movie = synthetic_movie(folder, "m", fps=10, start=0.2, duration=0.2, total=10)
            certify(movie, "final")
            record = stills.make_stills(movie, folder / "out", "m", profile="final")
            chosen = profile.profile("final")
            self.assertEqual((record["poster"]["width"], record["poster"]["height"]), (chosen.width, chosen.height))
            self.assertEqual(image(folder / "out" / record["poster"]["file"]).shape[:2], (chosen.height, chosen.width))


if __name__ == "__main__":
    unittest.main()
