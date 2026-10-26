"""🧪 Validate the portable video contract against authored scenes and Manim."""

from __future__ import annotations

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

import ast
import importlib.util
import json
import os
import tempfile
import threading
import time
import unittest
from unittest.mock import patch

ROOT = _WORKSPACE_ROOT / "🎓️teaching"
CONTRACT = json.loads((Path(__file__).with_name("🔣️.cases.json")).read_text(encoding="utf-8"))


def process_running(pid):
    """🔎 Query an owned test process without signaling it."""
    if os.name == "nt":
        import ctypes
        from ctypes import wintypes

        kernel = ctypes.WinDLL("kernel32", use_last_error=True)
        kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        kernel.OpenProcess.restype = wintypes.HANDLE
        kernel.WaitForSingleObject.argtypes = [wintypes.HANDLE, wintypes.DWORD]
        kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        handle = kernel.OpenProcess(0x100000, False, pid)
        if not handle:
            return False
        try:
            return kernel.WaitForSingleObject(handle, 0) == 258
        finally:
            kernel.CloseHandle(handle)
    try:
        os.kill(pid, 0)
        status = Path(f"/proc/{pid}/stat")
        return not (status.is_file() and status.read_text().split()[2] == "Z")
    except ProcessLookupError:
        return False


class VideoContractTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        output = os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")
        if output:
            Path(output).mkdir(parents=True, exist_ok=True)

    def test_canonical_modules_and_asset_paths(self):
        case = CONTRACT["canonical_paths"]
        runtime = importlib.import_module(case["runtime"])
        fonts = importlib.import_module(case["fonts"])
        visuals = importlib.import_module(case["visuals"])
        self.assertEqual(Path(runtime.__file__).name, case["kind_leaf"])
        self.assertNotEqual(fonts.__file__, visuals.__file__)
        self.assertIs(fonts, importlib.import_module(case["fonts"]))
        self.assertEqual(runtime.teaching_root(Path(fonts.__file__)), ROOT)
        self.assertTrue((ROOT / case["intro_pdf"]).is_file())
        for video in runtime.load_manifest():
            self.assertEqual(Path(video["script"]).name, case["kind_leaf"])
            for chapter in video["chapters"]:
                for segment in chapter["segments"]:
                    self.assertEqual(Path(segment["script"]).name, case["kind_leaf"])

    def test_trace_facets_merge_without_source_outputs(self):
        speech = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            expected = {}
            for record in CONTRACT["trace_facets"]["records"]:
                marks = {record["clause"]: {"start": record["start"], "window": record["window"]}}
                expected[record["beat"]] = marks
                path = folder / record["beat"] / "🔣️.json"
                path.parent.mkdir()
                path.write_text(json.dumps({"beats": {record["beat"]: marks}}), encoding="utf-8")
                self.assertEqual(speech.read_vo_trace(path), {record["beat"]: marks})
            self.assertEqual(speech.read_vo_trace(folder), expected)

    def test_trace_output_separates_identical_beat_names_by_chapter(self):
        from hashlib import sha256
        visuals = importlib.import_module(CONTRACT["canonical_paths"]["visuals"])
        speech = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🐍️")
        case = CONTRACT["trace_facets"]["collision"]
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            output = folder / "output"
            for owner, window in zip(case["owners"], case["windows"], strict=True):
                source = folder / owner / "🐍️.py"
                marks = {"a": {"start": 0, "window": window}}
                with patch.object(visuals, "_WORKSPACE_ROOT", folder), patch.object(visuals, "_VO_TRACE", {case["beat"]: marks}), patch.object(visuals, "_VO_TRACE_PATHS", {case["beat"]: str(source)}), patch.dict(os.environ, {"SEMIO_TEACHING_OUTPUT_ROOT": str(output)}):
                    visuals._dump_vo_trace()
            for owner, window in zip(case["owners"], case["windows"], strict=True):
                identity = sha256((folder / owner).relative_to(folder).as_posix().encode()).hexdigest()[:10]
                self.assertEqual(speech.read_vo_trace(output / "trace" / identity), {case["beat"]: {"a": {"start": 0, "window": window}}})

    def test_curriculum_contract(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        load_manifest = _video_module.load_manifest

        videos = load_manifest()
        self.assertEqual([v["id"] for v in videos], CONTRACT["curriculum"]["ids"])
        self.assertEqual([len(v["chapters"]) for v in videos], CONTRACT["curriculum"]["chapter_counts"])
        self.assertEqual([[len(chapter["segments"]) for chapter in v["chapters"]] for v in videos], CONTRACT["curriculum"]["segment_counts"])
        self.assertEqual(len({v["id"] for v in videos}), 5)
        self.assertEqual(len({v["intro"]["scene"] for v in videos}), CONTRACT["curriculum"]["intro_count"])
        intros = {n.name for n in ast.parse((ROOT / "🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py").read_text(encoding="utf-8")).body if isinstance(n, ast.ClassDef)}
        for video in videos:
            with self.subTest(video=video["id"]):
                classes = {n.name for n in ast.parse((ROOT / video["script"]).read_text(encoding="utf-8")).body if isinstance(n, ast.ClassDef)}
                self.assertTrue({chapter["scene"] for chapter in video["chapters"]}.issubset(classes))
                self.assertIn(video["intro"]["scene"], intros)

    def test_segment_plan_has_stable_global_identifiers(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        segment_plan = _video_module.segment_plan

        plan = segment_plan()
        self.assertEqual(len(plan), CONTRACT["curriculum"]["render_segment_count"])
        self.assertEqual([job["ordinal"] for job in plan], list(range(1, len(plan) + 1)))
        self.assertEqual(len({(job["video_index"], job["chapter_scene"]) for job in plan}), CONTRACT["curriculum"]["logical_clip_count"])
        first = [min(job["ordinal"] for job in plan if job["video_id"] == video) for video in CONTRACT["curriculum"]["ids"]]
        self.assertEqual(first, CONTRACT["curriculum"]["first_segment_ordinals"])

    def test_global_worker_bound_and_failure_isolation(self):
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        case = CONTRACT["render_scheduling"]
        lock = threading.Lock()
        active = peak = 0
        entered = []

        def worker(job, session):
            nonlocal active, peak
            with lock:
                active += 1
                peak = max(peak, active)
                entered.append(job)
            try:
                code = 7 if job == case["failed_job"] else 0
                runtime.run_command([sys.executable, "-c", f"import time; time.sleep(0.1); raise SystemExit({code})"], session=session)
                return job
            finally:
                with lock:
                    active -= 1

        completed, failures = runtime.execute_jobs(case["jobs"], worker, max_workers=case["workers"])
        self.assertEqual(sorted(completed.values()), case["completed"])
        self.assertEqual(sorted(entered), case["jobs"])
        self.assertEqual(list(failures), [case["failed_job"]])
        self.assertEqual(peak, case["workers"])

    def test_batch_cancellation_stops_active_groups_and_never_starts_queued_jobs(self):
        import subprocess
        import signal
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        case = CONTRACT["render_scheduling"]
        owned = []
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)

            def worker(job, session):
                target = folder / str(job)
                target.mkdir()
                runtime.run_command([sys.executable, str(Path(__file__)), "--process-tree-fixture", "child", str(target)], session=session)

            def interrupt(futures):
                deadline = time.monotonic() + CONTRACT["process_cancellation"]["ready_timeout"]
                paths = [folder / str(job) / f"{role}.json" for job in case["jobs"][:case["workers"]] for role in ("child", "grandchild")]
                while not all(path.is_file() for path in paths) and time.monotonic() < deadline:
                    time.sleep(0.05)
                self.assertTrue(all(path.is_file() for path in paths))
                owned.extend(json.loads(path.read_text())["pid"] for path in paths)
                raise KeyboardInterrupt()
                yield

            try:
                with patch.object(runtime, "as_completed", side_effect=interrupt), self.assertRaises(KeyboardInterrupt):
                    runtime.execute_jobs(case["jobs"], worker, max_workers=case["workers"])
                deadline = time.monotonic() + CONTRACT["process_cancellation"]["shutdown_timeout"]
                while any(process_running(pid) for pid in owned) and time.monotonic() < deadline:
                    time.sleep(0.05)
                self.assertFalse(any(process_running(pid) for pid in owned))
                self.assertFalse(any((folder / str(job)).exists() for job in case["queued_after_cancellation"]))
            finally:
                for pid in owned:
                    if process_running(pid):
                        if os.name == "nt":
                            subprocess.run(["taskkill", "/PID", str(pid), "/T", "/F"], capture_output=True, creationflags=subprocess.CREATE_NO_WINDOW, check=False, timeout=10)
                        else:
                            os.kill(pid, signal.SIGKILL)

    def test_whole_chapters_reuse_without_retaining_segment_media(self):
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            intro = {"script": "🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py", "scene": "Intro"}
            beat = {"script": "chapter/scene.py", "scene": "Beat"}
            for name in (intro["script"], beat["script"], "🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py", "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py"):
                path = folder / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("authored render input", encoding="utf-8")
            video = {"id": "fixture", "script": "wrapper.py", "playlist": "PLAYLIST", "intro": intro,
                     "chapters": [{"scene": "Chapter", "segments": [beat]}], "output": "Full_Fixture"}
            settings = {"quality": "-ql", "resolution": None, "fps": None}
            media = folder / "media"
            with patch.object(runtime, "teaching_root", return_value=folder), patch.object(runtime, "load_manifest", return_value=[video]):
                for scene, segments in (("Intro", [intro]), ("Chapter", [beat])):
                    clip = media / "v" / f"{scene}.mp4"
                    clip.parent.mkdir(parents=True, exist_ok=True)
                    clip.write_bytes(b"0" * 1200)
                    clip.with_suffix(".render.json").write_text(json.dumps(runtime._logical_state(scene, segments, settings)), encoding="utf-8")

                def concatenate(clips, output, list_path):
                    output.parent.mkdir(parents=True, exist_ok=True)
                    output.write_bytes(b"0" * 1200)

                with patch.object(runtime, "run_command") as command, patch.object(runtime, "concat_videos", side_effect=concatenate) as concat:
                    result = runtime.compose_video("fixture", media_dir=media, output_dir=folder / "out", concat_only=True)
                self.assertTrue(result.is_file())
                command.assert_not_called()
                concat.assert_called_once()
                self.assertFalse((media / "s").exists())

    @unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable real segment rendering with SEMIO_TEST_MANIM=1")
    def test_real_manim_segments_preserve_order_and_full_frame_counts(self):
        import av
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        case = CONTRACT["segment_integration"]
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            script = folder / "🎬️scene/🐍️.py"
            script.parent.mkdir()
            script.write_text("from manim import Scene, Square, GREEN, RED, BLUE\n"
                              "class FixtureIntro(Scene):\n    def construct(self):\n        self.add(Square(color=GREEN, fill_opacity=1))\n        self.wait(0.2)\n"
                              "class FixtureA(Scene):\n    def construct(self):\n        self.add(Square(color=RED, fill_opacity=1))\n        self.wait(0.2)\n"
                              "class FixtureB(Scene):\n    def construct(self):\n        self.add(Square(color=BLUE, fill_opacity=1))\n        self.wait(0.2)\n", encoding="utf-8")
            for name in ("🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py", "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py"):
                (folder / name).parent.mkdir(parents=True, exist_ok=True)
                (folder / name).write_text("authored fixture dependency", encoding="utf-8")
            segments = [{"script": "🎬️scene/🐍️.py", "scene": name} for name in ("FixtureIntro", "FixtureA", "FixtureB")]
            video = {"id": "fixture", "script": "wrapper.py", "playlist": "PLAYLIST", "intro": segments[0],
                     "chapters": [{"scene": "FixtureChapter", "segments": segments[1:]}], "output": "Full_Fixture"}
            output = folder / "output"
            with patch.object(runtime, "teaching_root", return_value=folder), patch.object(runtime, "load_manifest", return_value=[video]):
                movie = runtime.compose_videos(output_root=output, jobs=2, log_output=True)["fixture"]
            for ordinal, segment in enumerate(segments, 1):
                with av.open(output / "s" / str(ordinal) / "v" / f"{segment['scene']}.mp4") as container:
                    self.assertEqual(sum(1 for _ in container.decode(video=0)), case["frames_per_segment"])
            with av.open(output / "1/v/FixtureChapter.mp4") as container:
                self.assertEqual(sum(1 for _ in container.decode(video=0)), case["chapter_frames"])
            with av.open(movie) as container:
                frames = list(container.decode(video=0))
            self.assertEqual(len(frames), case["movie_frames"])
            for ordinal, channel in enumerate(case["dominant_channels"]):
                pixels = frames[ordinal * case["frames_per_segment"]].to_ndarray(format="rgb24")
                self.assertEqual(int(pixels[pixels.shape[0] // 2, pixels.shape[1] // 2].argmax()), channel)

    def test_root_ignores_product_package(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        teaching_root = _video_module.teaching_root

        self.assertEqual(teaching_root(ROOT / "🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🐍️.py"), ROOT)

    def test_active_python_is_manim_interface(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        manim_command = _video_module.manim_command

        expected = [sys.executable if part == "<active-python>" else part for part in CONTRACT["manim_command"]]
        self.assertEqual(manim_command(), expected)

    def test_concat_paths_are_portable(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        concat_entry = _video_module.concat_entry

        entry = concat_entry(ROOT / "space's clip.mp4")
        self.assertIn("space'\\''s clip.mp4", entry)
        self.assertNotIn("\\", entry.replace("'\\''", ""))
        self.assertTrue(entry.startswith("file '"))

    def test_render_config_removes_only_broken_direct_text_cache_svg(self):
        import xml.etree.ElementTree as element_tree
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        render_config = _video_module.render_config

        case = CONTRACT["text_cache_preparation"]
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            for entry in case["files"]:
                path = folder / entry["path"]
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(entry["content"], encoding="utf-8")
            render_config(folder / "media")
            for entry in case["files"]:
                path = folder / entry["path"]
                with self.subTest(path=entry["path"]):
                    self.assertEqual(path.exists(), entry["keep"])
                    if entry["keep"]:
                        self.assertEqual(path.read_text(encoding="utf-8"), entry["content"])
            valid = folder / case["files"][0]["path"]
            self.assertEqual(element_tree.parse(valid).getroot().tag, "{http://www.w3.org/2000/svg}svg")
            if os.environ.get("SEMIO_TEST_MANIM") == "1":
                from manim import SVGMobject

                self.assertTrue(SVGMobject(str(valid)).family_members_with_points())

    def test_render_config_preserves_text_cache_symlinks_and_targets(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        render_config = _video_module.render_config

        case = CONTRACT["text_cache_preparation"]
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            asset = folder / "authored.svg"
            asset.write_text("<svg>", encoding="utf-8")
            outside = folder / "outside-cache"
            outside.mkdir()
            broken = outside / "broken.svg"
            broken.write_bytes(b"")
            file_link = folder / case["file_link"]
            directory_link = folder / case["directory_link"]
            file_link.parent.mkdir(parents=True)
            directory_link.parent.mkdir(parents=True)
            try:
                file_link.symlink_to(asset)
                directory_link.symlink_to(outside, target_is_directory=True)
            except OSError as error:
                self.skipTest(f"Symbolic links unavailable: {error}")
            render_config(folder / "media")
            render_config(directory_link.parent)
            self.assertTrue(file_link.is_symlink())
            self.assertTrue(directory_link.is_symlink())
            self.assertEqual(asset.read_text(encoding="utf-8"), "<svg>")
            self.assertEqual(broken.read_bytes(), b"")

    def test_render_config_does_not_follow_link_metadata(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        render_config = _video_module.render_config

        case = CONTRACT["text_cache_preparation"]
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            file_link = folder / case["file_link"]
            directory_link = folder / case["directory_link"]
            file_link.parent.mkdir(parents=True)
            directory_link.mkdir(parents=True)
            file_link.write_bytes(b"")
            nested = directory_link / "broken.svg"
            nested.write_bytes(b"")
            original = Path.is_symlink
            with patch.object(Path, "is_symlink", lambda path: path in (file_link, directory_link) or original(path)):
                render_config(folder / "media")
                render_config(directory_link.parent)
            self.assertEqual(file_link.read_bytes(), b"")
            self.assertEqual(nested.read_bytes(), b"")

    def test_changed_intro_asset_invalidates_clip(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        clip_state = _video_module.clip_state
        reusable_clip = _video_module.reusable_clip

        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            case = CONTRACT["intro_asset_changed"]
            folder = Path(directory)
            pdf = folder / "welfenschloss.pdf"
            pdf.write_bytes(case["before"].encode())
            clip = folder / "intro.mp4"
            clip.write_bytes(b"0" * 1001)
            settings = {"quality": "-ql", "resolution": None, "fps": None}
            clip.with_suffix(".render.json").write_text(json.dumps(clip_state(settings, [pdf])), encoding="utf-8")
            self.assertEqual(reusable_clip(clip, clip_state(settings, [pdf])), case["reusable_before"])
            pdf.write_bytes(case["after"].encode())
            self.assertEqual(reusable_clip(clip, clip_state(settings, [pdf])), case["reusable_after"])

    def test_interrupted_quality_change_keeps_only_completed_matching_clips(self):
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        clip_state = _video_module.clip_state
        reusable_clip = _video_module.reusable_clip

        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            case = CONTRACT["interrupted_quality_switch"]
            folder = Path(directory)
            source = folder / "scene.py"
            source.write_text("authored scene", encoding="utf-8")
            low = {"quality": case["previous_quality"], "resolution": None, "fps": None}
            high = {**low, "quality": case["target_quality"]}
            clips = [folder / "completed.mp4", folder / "pending.mp4"]
            for clip, settings in zip(clips, [high, low]):
                clip.write_bytes(b"0" * 1001)
                clip.with_suffix(".render.json").write_text(json.dumps(clip_state(settings, [source])), encoding="utf-8")
            self.assertEqual(reusable_clip(clips[0], clip_state(high, [source])), case["completed_clip_reusable"])
            self.assertEqual(reusable_clip(clips[1], clip_state(high, [source])), case["pending_clip_reusable"])

    def test_render_cache_publication_requires_completed_unchanged_sources(self):
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        for case in CONTRACT["render_cache_publication"]:
            with self.subTest(case=case["id"]), tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
                folder = Path(directory)
                intro = folder / "🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py"
                intro.parent.mkdir(parents=True)
                intro.write_text(case["source_before"], encoding="utf-8")
                common = [folder / "🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py", folder / "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py"]
                for source in common:
                    source.parent.mkdir(parents=True, exist_ok=True)
                    source.write_text("authored dependency", encoding="utf-8")
                media = folder / "media"
                clip = media / "v" / "Intro.mp4"
                clip.parent.mkdir(parents=True)
                clip.write_bytes(b"0" * case["cached_bytes"])
                settings = {"quality": "-ql", "resolution": None, "fps": None}
                segment = {"script": "🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py", "scene": "Intro"}
                with patch.object(runtime, "teaching_root", return_value=folder):
                    state = runtime.segment_state(segment, settings)
                sidecar = clip.with_suffix(".render.json")
                sidecar.write_text(json.dumps(state), encoding="utf-8")
                self.assertEqual(runtime.reusable_clip(clip, state), case["reusable_before"])
                def rerender(*args, **kwargs):
                    clip.write_bytes(b"1" * case["replacement_bytes"])
                    if case["interrupted"]:
                        raise InterruptedError("render interrupted")
                    intro.write_text(case["source_after"], encoding="utf-8")

                error = InterruptedError if case["interrupted"] else RuntimeError
                with patch.object(runtime, "teaching_root", return_value=folder), patch.object(runtime, "run_command", side_effect=rerender), patch.object(runtime, "concat_videos") as concat:
                    with self.assertRaises(error):
                        runtime.render_segment(segment, media, settings, force=True)
                self.assertEqual(runtime.reusable_clip(clip, state), case["reusable_after"])
                self.assertFalse(sidecar.exists())
                concat.assert_not_called()

    def test_non_reusable_segment_disables_manim_partial_cache(self):
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        case = CONTRACT["non_reusable_segment_render"]
        for sidecar_kind in case["sidecars"]:
            with self.subTest(sidecar=sidecar_kind), tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
                folder = Path(directory)
                intro = folder / "🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py"
                intro.parent.mkdir(parents=True)
                intro.write_text("authored intro", encoding="utf-8")
                media = folder / "media"
                clip = media / "v" / "Intro.mp4"
                clip.parent.mkdir(parents=True)
                clip.write_bytes(b"0" * 1200)
                if sidecar_kind == "mismatched":
                    clip.with_suffix(".render.json").write_text(json.dumps({"stale": True}), encoding="utf-8")
                settings = {"quality": case["quality"], "resolution": None, "fps": None}
                (media / "settings.json").write_text(json.dumps(settings), encoding="utf-8")
                segment = {"script": "🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py", "scene": "Intro"}
                with patch.object(runtime, "teaching_root", return_value=folder), patch.object(runtime, "run_command") as command:
                    runtime.render_segment(segment, media, settings, force=case["force"])
                command.assert_called_once()
                self.assertIn(case["required_flag"], command.call_args.args[0])

    def test_command_success_and_error_status_are_preserved(self):
        import subprocess
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        run_command = _video_module.run_command

        for code in CONTRACT["command_exit_codes"]:
            with self.subTest(code=code):
                command = [sys.executable, "-c", f"raise SystemExit({code})"]
                if code:
                    with self.assertRaises(subprocess.CalledProcessError) as error:
                        run_command(command)
                    self.assertEqual(error.exception.returncode, code)
                    self.assertEqual(error.exception.cmd, command)
                else:
                    self.assertIsNone(run_command(command))

    def test_cancellation_stops_owned_child_and_grandchild(self):
        import signal
        import subprocess
        import time
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        case = CONTRACT["process_cancellation"]
        owned = []

        running = process_running

        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            original_wait = subprocess.Popen.wait
            interrupted = False

            def interrupt(process, *args, **kwargs):
                nonlocal interrupted
                if not interrupted and "--process-tree-fixture" in process.args:
                    interrupted = True
                    deadline = time.monotonic() + case["ready_timeout"]
                    paths = [folder / f"{role}.json" for role in case["roles"]]
                    while not all(path.is_file() for path in paths) and time.monotonic() < deadline:
                        time.sleep(0.05)
                    self.assertTrue(all(path.is_file() for path in paths), "Owned process tree did not become ready")
                    owned.extend(json.loads(path.read_text())["pid"] for path in paths)
                    self.assertTrue(all(running(pid) for pid in owned))
                    raise KeyboardInterrupt()
                return original_wait(process, *args, **kwargs)

            try:
                with patch.object(subprocess.Popen, "wait", interrupt), self.assertRaises(KeyboardInterrupt):
                    runtime.run_command([sys.executable, str(Path(__file__)), "--process-tree-fixture", "child", str(folder)])
                deadline = time.monotonic() + case["shutdown_timeout"]
                while any(running(pid) for pid in owned) and time.monotonic() < deadline:
                    time.sleep(0.05)
                self.assertEqual([running(pid) for pid in owned], case["expected_running_after"])
            finally:
                for role in case["roles"]:
                    path = folder / f"{role}.json"
                    if path.is_file():
                        pid = json.loads(path.read_text())["pid"]
                        if not running(pid):
                            continue
                        if os.name == "nt":
                            subprocess.run(["taskkill", "/PID", str(pid), "/T", "/F"], capture_output=True, creationflags=subprocess.CREATE_NO_WINDOW, check=False, timeout=10)
                        else:
                            try:
                                os.kill(pid, signal.SIGKILL)
                            except ProcessLookupError:
                                pass

    def test_cli_help_and_errors_use_utf8_under_hostile_encoding(self):
        import subprocess

        case = CONTRACT["cli_encoding"]
        environment = {**os.environ, "PYTHONIOENCODING": case["encoding"], "PYTHONUTF8": "0", "PYTHONDONTWRITEBYTECODE": "1"}
        for arguments, field, expected_code in ((["--help"], "stdout", 0), (["--video", case["invalid_video"]], "stderr", case["invalid_exit"])):
            with self.subTest(arguments=arguments):
                result = subprocess.run([sys.executable, str(ROOT / "🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🖌️render/🐍️.py"), *arguments], env=environment, capture_output=True, check=False, timeout=30)
                self.assertEqual(result.returncode, expected_code)
                self.assertIn(case["help_symbol"], getattr(result, field).decode("utf-8"))
                self.assertNotIn(b"UnicodeEncodeError", result.stderr)

    @unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable third-party Manim validation with SEMIO_TEST_MANIM=1")
    def test_manifest_matches_manim_discovery(self):
        from manim import Scene
        from manim.utils.module_ops import get_scene_classes_from_module
        _video_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        load_manifest = _video_module.load_manifest

        for video in load_manifest():
            with self.subTest(video=video["id"]):
                module = importlib.import_module("🎓️teaching." + video["script"][:-3].replace("/", "."))
                discovered = {scene.__name__ for scene in get_scene_classes_from_module(module)}
                self.assertTrue({chapter["scene"] for chapter in video["chapters"]}.issubset(discovered))
                playlist = getattr(module, video["playlist"])
                self.assertEqual(len(playlist), len(video["chapters"]))
                for chapter, (_, beats) in zip(video["chapters"], playlist, strict=True):
                    self.assertTrue(issubclass(getattr(module, chapter["scene"]), Scene))
                    actual = [{"script": Path(sys.modules[beat.__module__].__file__).resolve().relative_to(ROOT).as_posix(), "scene": beat.__name__} for beat in beats]
                    self.assertEqual(chapter["segments"], actual)
                    for beat in beats:
                        found = {scene.__name__ for scene in get_scene_classes_from_module(sys.modules[beat.__module__])}
                        self.assertIn(beat.__name__, found)


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--process-tree-fixture":
        import subprocess
        import time

        role, folder = sys.argv[2], Path(sys.argv[3])
        if role == "child":
            subprocess.Popen([sys.executable, str(Path(__file__)), "--process-tree-fixture", "grandchild", str(folder)])
        temporary = folder / f"{role}.tmp"
        temporary.write_text(json.dumps({"pid": os.getpid()}), encoding="utf-8")
        temporary.replace(folder / f"{role}.json")
        while True:
            time.sleep(0.1)
    unittest.main()
