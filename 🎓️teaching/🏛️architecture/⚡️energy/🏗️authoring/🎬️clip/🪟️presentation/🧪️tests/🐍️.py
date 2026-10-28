"""🪟 Verify portable presentation geometry, boundaries and narrated content envelopes."""

from __future__ import annotations

import importlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"


class PresentationContractTest(unittest.TestCase):
    def setUp(self):
        self.presentation = importlib.import_module(MODULE + ".🪟️presentation.🐍️")

    def test_portable_order_locales_chrome_crop_and_timing(self):
        contract = self.presentation.presentation_contract()
        for key in ("viewport", "chrome", "timing", "restoreTiming"):
            self.assertEqual(contract[key], CASES[key])
        self.assertEqual(contract["sourceCrop"], CASES["crop"])
        self.assertEqual([topic["id"] for topic in contract["topics"]], [topic["id"] for topic in CASES["topics"]])
        for topic, case in zip(contract["topics"], CASES["topics"], strict=True):
            self.assertEqual([video["id"] for video in topic["videos"]], [video["id"] for video in case["videos"]])
            for video, expected in zip(topic["videos"], case["videos"], strict=True):
                self.assertEqual(video["title"], expected["title"])
        self.assertAlmostEqual(sum(contract["timing"].values()), 3)
        self.assertEqual(contract["contentCamera"]["duration"], CASES["contentCamera"]["duration"])

    def test_focus_geometry_matches_independent_numpy_affine_oracle(self):
        import numpy as np

        for width, height in CASES["resolutions"]:
            scale = np.array([width / CASES["viewport"]["width"], height / CASES["viewport"]["height"]] * 2)
            np.testing.assert_allclose(
                list(self.presentation.focus_viewport(width, height).values()),
                np.array(list(CASES["focus"].values())) * scale,
            )

    def test_energy_curriculum_has_five_topic_owned_videos_and_no_institute_intro_segments(self):
        runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        energy = next(topic for topic in CASES["topics"] if topic["id"] == "energy")
        selected = [video["id"] for video in energy["videos"]]
        videos = [video for video in runtime.load_manifest(source_ids=selected) if video["id"] in selected]
        self.assertEqual([video["id"] for video in videos], [video["id"] for video in energy["videos"]])
        self.assertTrue(all("intro" not in video for video in videos))
        plan = runtime.segment_plan(videos)
        self.assertTrue(all("👋️intro" not in job["segment"]["script"] for job in plan))
        self.assertTrue(all(job["chapter_index"] >= 0 for job in plan))

    def test_language_is_explicit_in_presentation_and_runtime(self):
        import inspect

        runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        parameter = inspect.signature(runtime.compose_videos).parameters["language"]
        self.assertIs(parameter.default, inspect.Parameter.empty)
        self.assertIs(parameter.kind, inspect.Parameter.KEYWORD_ONLY)
        with self.assertRaises(ValueError):
            self.presentation.render_overview(Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "missing-language.png", {"resolution": "320,180", "fps": 4})


    def test_certificate_write_failure_cannot_leave_old_certificate_on_new_movie(self):
        from unittest.mock import patch

        runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        settings = {"audience": "speaker", "quality": "-ql", "resolution": "320,180", "fps": 4}
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            content, output = Path(directory) / "raw.mp4", Path(directory) / "lesson.mp4"
            content.write_bytes(b"raw" * 500)
            output.write_bytes(b"old-movie" * 200)
            sidecar = output.with_suffix(".render.json")
            sidecar.write_bytes(b"old-certificate")

            def render(video_id, source, candidate, settings, *, session=None):
                candidate.write_bytes(b"valid-new-movie" * 200)
                return candidate

            with patch.object(runtime, "presentation_state", return_value={"version": 1}), \
                    patch.object(runtime, "reusable_clip", return_value=True), \
                    patch.object(runtime, "render_presentation", side_effect=render), \
                    patch.object(runtime, "_write_state", side_effect=OSError("disk full")), \
                    self.assertRaises(OSError):
                runtime.publish_presentation("heating", content, output, {"raw": True}, settings, "de", force=True)
            self.assertEqual(output.read_bytes(), b"valid-new-movie" * 200)
            self.assertFalse(sidecar.exists())

    def test_changed_inputs_preserve_previous_movie_and_certificate(self):
        from unittest.mock import patch

        runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        settings = {"audience": "speaker", "quality": "-ql", "resolution": "320,180", "fps": 4}
        for changed in ("presentation", "content", "source", "cursor"):
            with self.subTest(changed=changed), tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
                folder = Path(directory)
                content, output = folder / "content.mp4", folder / "lesson.mp4"
                content.write_bytes(b"raw-content" * 200)
                runtime.publish_cursor_intervals(content, [])
                previous = b"old-certified-movie" * 200
                certificate = b'{"previous": "certificate"}'
                output.write_bytes(previous)
                sidecar = output.with_suffix(".render.json")
                sidecar.write_bytes(certificate)
                fingerprints = [{"version": 1}, {"version": 2 if changed == "presentation" else 1}]
                from hashlib import sha256

                dependency = folder / "authored.py"
                dependency.write_bytes(b"original-source")
                content_state = {"chapters": [{"sources": {dependency.resolve().as_posix(): sha256(dependency.read_bytes()).hexdigest()}}]}

                def render(video_id, source, candidate, settings, *, session=None):
                    if changed == "source":
                        dependency.write_bytes(b"mutated-source")
                    if changed == "cursor":
                        runtime.publish_cursor_intervals(content, [{"start": 0, "end": 0.5}])
                    candidate.write_bytes(b"new-uncertified-movie" * 200)
                    return candidate

                with patch.object(runtime, "presentation_state", side_effect=fingerprints), \
                        patch.object(runtime, "reusable_clip", return_value=changed != "content"), \
                        patch.object(runtime, "render_presentation", side_effect=render), \
                        self.assertRaises(RuntimeError):
                    runtime.publish_presentation("heating", content, output, content_state, settings, "de", force=True)
                self.assertEqual(output.read_bytes(), previous)
                self.assertEqual(sidecar.read_bytes(), certificate)

    def test_presentation_cache_includes_locale_and_all_visual_source_hashes(self):
        from hashlib import sha256

        runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        settings = {"audience": "speaker", "quality": "-ql", "resolution": "320,180", "fps": 4}
        de = runtime.presentation_state("heating", settings, "de")
        self.assertNotEqual(de, runtime.presentation_state("heating", settings, "en"))
        catalog_path = Path(self.presentation.__file__).with_name("🔣️.json").resolve()
        self.assertEqual(de["catalog"],self.presentation.presentation_contract())
        for path in self.presentation.source_files():
            with self.subTest(source=path.name):
                if path.resolve() == catalog_path:
                    self.assertNotIn(path.resolve().as_posix(),de["sources"])
                else:
                    self.assertEqual(de["sources"][path.resolve().as_posix()], sha256(path.read_bytes()).hexdigest())

    def test_publication_state_certifies_the_defensive_captured_catalog_without_duplicate_live_file_identity(self):
        from copy import deepcopy
        from unittest.mock import patch

        runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        hierarchy = self.presentation._hierarchy
        first_model = hierarchy.hierarchy_contract()
        second_model = deepcopy(first_model)
        second_model["identity"]["brand"] = "Another captured model"
        settings = {"audience":"speaker","quality":"-ql","resolution":"320,180","fps":4}
        with patch.object(self.presentation,"source_files",return_value=hierarchy.source_files()):
            with hierarchy.capture_catalog(first_model):
                first = runtime.presentation_state("heating",settings,"de")
            with hierarchy.capture_catalog(second_model):
                second = runtime.presentation_state("heating",settings,"de")
        self.assertEqual(first["catalog"],first_model)
        self.assertEqual(second["catalog"],second_model)
        self.assertEqual(first["sources"],second["sources"])
        self.assertNotEqual(first,second)
        catalog_path = Path(self.presentation.__file__).with_name("🔣️.json").resolve().as_posix()
        self.assertNotIn(catalog_path,first["sources"])
        first["catalog"]["identity"]["brand"] = "Changed result"
        self.assertNotEqual(first_model["identity"]["brand"],"Changed result")

    def test_runtime_prepares_ordered_content_before_one_collection_publication_transaction(self):
        from copy import deepcopy
        from unittest.mock import patch

        runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        video = deepcopy(runtime.load_manifest()[0])
        video["chapters"] = [video["chapters"][0]]
        events = []
        output_root = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])

        def concatenate(clips, output, list_path):
            events.append("concat")
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(b"raw-content" * 200)
            runtime.publish_cursor_intervals(output, [])

        def present(video_id, content, output, settings, *, session=None):
            events.append("presentation")
            self.assertEqual(video_id, video["id"])
            self.assertEqual(settings["language"], "de")
            self.assertTrue(content.read_bytes().startswith(b"raw-content"))
            self.assertNotEqual(content, output)
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(b"enveloped" * 200)
            return output

        original_write = runtime._write_state

        def publish(path, state):
            if "presentation" in state:
                events.append("published")
                self.assertIn("presentation", state)
            return original_write(path, state)

        with tempfile.TemporaryDirectory(dir=output_root) as directory:
            chapter = Path(directory) / "de" / "speaker" / "1" / "v" / (video["chapters"][0]["scene"] + ".mp4")
            chapter.parent.mkdir(parents=True, exist_ok=True)
            chapter.write_bytes(b"raw-chapter" * 200)
            runtime.publish_cursor_intervals(chapter, [])
            with patch.object(runtime, "_logical_state", return_value={"fixture": True}), \
                    patch.object(runtime, "load_manifest", return_value=[video]), \
                    patch.object(runtime, "reusable_clip", side_effect=lambda clip, state: "presentation" not in state and ("video" not in state or events[:1] == ["concat"])), \
                    patch.object(runtime, "execute_jobs", return_value=({}, {})), \
                    patch.object(runtime, "concat_videos", side_effect=concatenate), \
                    patch.object(runtime, "render_presentation", side_effect=present) as envelope, \
                    patch.object(runtime, "_write_state", side_effect=publish):
                result = runtime.compose_videos([video["id"]], language="de", audience="speaker", output_root=Path(directory))
            self.assertEqual(events, ["concat", "presentation", "published"])
            envelope.assert_called_once()
            self.assertTrue(result[video["id"]].read_bytes().startswith(b"enveloped"))

class PresentationNativeTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.presentation = importlib.import_module(MODULE + ".🪟️presentation.🐍️")
        cls.runtime = importlib.import_module(MODULE + ".⚙️runtime.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        cls.directory = tempfile.TemporaryDirectory(dir=output)
        cls.output = Path(cls.directory.name)
        cls.settings = {"audience": "speaker", "quality": "-ql", "resolution": "320,180", "fps": 4, "language": "de"}

    @classmethod
    def tearDownClass(cls):
        cls.directory.cleanup()

    def test_native_overview_is_deterministic_with_exact_pillow_pixels(self):
        from PIL import Image, ImageChops

        self.enterContext(self.presentation._hierarchy.capture_catalog())
        paths = [self.output / f"overview-{index}.png" for index in range(2)]
        for path in paths:
            self.presentation.render_overview(path, self.settings)
        with Image.open(paths[0]) as first, Image.open(paths[1]) as second:
            self.assertEqual(first.size, (320, 180))
            self.assertIsNone(ImageChops.difference(first.convert("RGB"), second.convert("RGB")).getbbox())
            self.assertGreater(len(first.convert("RGB").getcolors(320 * 180)), 5)

    def render_chapter_source(self, content, output, settings=None, session=None):
        """🎞️ Isolate finite chapter encoding from the separately tested publication certificates."""
        with tempfile.TemporaryDirectory(dir=self.output) as directory:
            return self.presentation._render_chapter("PhysicalFundamentals_FullSection", content, output,
                settings or self.settings, session or self.runtime.ToolSession(), Path(directory))

    def assert_full_frame_source_order(self, content, frames, case, padding):
        """🎯 Match full output pixels and original ordering through independent Pillow and PyAV decoding."""
        import av
        import numpy as np
        from PIL import Image

        with av.open(str(content)) as container:
            originals = [frame.to_image().convert("RGB") for frame in container.decode(video=0)]
        expected = [np.asarray(frame.resize((case["width"], case["height"]), Image.Resampling.BICUBIC)).astype(float)
                    for frame in originals]
        errors = []
        start = round(padding * case["fps"])
        for index in range(len(originals)):
            actual = np.asarray(frames[start + index]).astype(float)
            errors.append([float(np.mean(np.abs(actual - source))) for source in expected])
            self.assertLess(errors[-1][index], case["maxMeanRgbError"])
            for edge in (np.s_[:8, :], np.s_[-8:, :], np.s_[:, :8], np.s_[:, -8:]):
                self.assertLess(float(np.mean(np.abs(actual[edge] - expected[index][edge]))),
                                CASES["contentCamera"]["maxEdgeMeanRgbError"])
        self.assertEqual([int(np.argmin(row)) for row in errors], list(range(len(originals))))
        return errors

    def test_encoded_chapter_preserves_body_camera_handoffs_audio_and_contextual_boundaries(self):
        import av
        import numpy as np
        from PIL import Image, ImageChops

        self.enterContext(self.presentation._hierarchy.capture_catalog())
        case = CASES["smoke"]
        content = self.output / "content.mp4"
        self.runtime.run_command([*self.runtime.ffmpeg_command(), "-y", "-v", "error", "-f", "lavfi", "-i",
                                  "testsrc2=s=320x180:r=4:d=2", "-f", "lavfi", "-i",
                                  "sine=frequency=440:sample_rate=48000:duration=2",
                                  "-c:v", "libx264", "-preset", "ultrafast", "-crf", "0", "-pix_fmt", "yuv420p",
                                  "-c:a", "aac", "-b:a", "192k", str(content)])
        boundary = None
        padding = round(self.presentation.transition_duration("focus", "PhysicalFundamentals_FullSection") * case["fps"]) / case["fps"]
        restore = round(self.presentation.transition_duration("restore", "PhysicalFundamentals_FullSection") * case["fps"]) / case["fps"]
        expected_seconds = case["contentSeconds"] + padding + restore
        lessons = next(topic for topic in CASES["topics"] if topic["id"] == "energy")["videos"][:1]
        catalog_snapshot = self.presentation.presentation_contract()
        for window in lessons:
            with self.subTest(lesson=window["id"]):
                self.assertEqual(self.presentation.presentation_contract(),catalog_snapshot)
                output = self.output / (window["id"] + ".mp4")
                self.runtime.publish_cursor_intervals(content, case["cursorIntervals"] if window["id"] == lessons[0]["id"] else [])
                import threading

                session = self.runtime.ToolSession()
                deadline = threading.Timer(300, session.cancel)
                deadline.start()
                try:
                    self.render_chapter_source(content, output, session=session)
                finally:
                    deadline.cancel()
                self.assertEqual(self.presentation.presentation_contract(),catalog_snapshot)
                with av.open(str(output)) as container:
                    frames = [frame.to_image().convert("RGB") for frame in container.decode(video=0)]
                evidence = self.output.parent / "native-evidence"
                evidence.mkdir(exist_ok=True)
                import shutil

                shutil.copy2(output, evidence / output.name)
                for label, index in (("overview", 0), ("topic", 4), ("focused", 16), ("return", 28)):
                    frames[index].save(evidence / (window["id"] + "-" + label + ".png"))
                self.assertEqual(len(frames), round(expected_seconds * case["fps"]))
                self.assertEqual(ImageChops.difference(frames[0], frames[-1]).getbbox() is None, CASES["boundary"]["firstEqualsLast"])
                if boundary is None:
                    boundary = frames[0]
                self.assertIsNone(ImageChops.difference(boundary, frames[0]).getbbox())
                with av.open(str(output)) as container:
                    stream = container.streams.audio[0]
                    rate = stream.codec_context.sample_rate
                    blocks = [frame.to_ndarray().astype(float) for frame in container.decode(audio=0)]
                    audio = np.concatenate([block.mean(axis=0) for block in blocks])
                    if np.max(np.abs(audio)) > 2:
                        audio /= 32768
                self.assertLess(abs(len(audio) / rate - expected_seconds), 0.08)
                rms = lambda samples: float(np.sqrt(np.mean(np.square(samples))))
                self.assertLess(rms(audio[:round((padding - 0.1) * rate)]), 0.002)
                self.assertLess(rms(audio[round((padding + case["contentSeconds"] + 0.1) * rate):]), 0.002)
                middle = audio[round((padding + 0.2) * rate):round((padding + 1.8) * rate)]
                self.assertGreater(rms(middle), 0.05)
                self.assertLess(rms(middle), 0.12)
                frequencies = np.fft.rfftfreq(len(middle), d=1 / rate)
                peak = frequencies[np.argmax(np.abs(np.fft.rfft(middle)))]
                self.assertAlmostEqual(peak, case["toneHz"], delta=1)
                focus = frames[round((padding + 1) * case["fps"])]
                self.assertIsNotNone(ImageChops.difference(boundary, focus).getbbox())
                self.assert_full_frame_source_order(content, frames, case, padding)
                first_body = round(padding * case["fps"])
                last_body = first_body + round(case["contentSeconds"] * case["fps"]) - 1
                for before, after in ((first_body - 1, first_body), (last_body, last_body + 1)):
                    delta = np.abs(np.asarray(frames[before]).astype(float) - np.asarray(frames[after]).astype(float))
                    self.assertLess(float(delta.mean()), CASES["contentCamera"]["maxHandoffMeanRgbError"])

    def test_silent_source_preserves_every_full_frame_and_has_a_finite_silent_audio_stream(self):
        import av
        import numpy as np
        from PIL import Image
        from unittest.mock import patch

        case = CASES["smoke"]
        content, output = self.output / "silent-content.mp4", self.output / "silent-lesson.mp4"
        self.runtime.run_command([*self.runtime.ffmpeg_command(), "-y", "-v", "error", "-f", "lavfi", "-i",
                                  "testsrc2=s=320x180:r=4:d=2", "-an", "-c:v", "libx264", "-preset", "ultrafast",
                                  "-crf", "0", "-pix_fmt", "yuv420p", str(content)])

        def transition(video_id, snapshot, work, settings, session, *, phase):
            directory = work / phase
            directory.mkdir()
            for index in range(2):
                Image.new("RGB", (case["width"], case["height"]), "magenta").save(directory / f"{index:04d}.png")
            return directory, 2

        with patch.object(self.presentation, "_transition", side_effect=transition):
            self.render_chapter_source(content, output)
        with av.open(str(output)) as container:
            frames = [frame.to_image().convert("RGB") for frame in container.decode(video=0)]
        self.assertEqual(len(frames), 12)
        self.assert_full_frame_source_order(content, frames, case, 0.5)
        with av.open(str(output)) as container:
            stream = container.streams.audio[0]
            blocks = [frame.to_ndarray().astype(float) for frame in container.decode(audio=0)]
            samples = np.concatenate(blocks, axis=1)
            self.assertLess(abs(samples.shape[1] / stream.codec_context.sample_rate - 3), 0.08)
            self.assertLess(float(np.abs(samples).max()), 0.002)

    def test_native_content_camera_zooms_after_open_and_reveals_restore_chip_before_pointer_press(self):
        import numpy as np
        from PIL import Image

        case = CASES["smoke"]
        yy, xx = np.indices((case["height"], case["width"]))
        pixels = np.stack((xx * 3 % 256, yy * 5 % 256, (xx + yy) * 7 % 256), axis=2).astype("uint8")
        snapshot = self.output / "camera-source.png"
        Image.fromarray(pixels).save(snapshot)
        samples = CASES["contentCamera"]["samples"]
        requests = [{"video_id": "PhysicalFundamentals_FullSection", "phase": sample["phase"],
                     "t": sample["seconds"] / self.presentation.transition_duration(sample["phase"], "PhysicalFundamentals_FullSection"), "snapshot": snapshot,
                     "output": self.output / f"camera-{index}.png"}
                    for index, sample in enumerate(samples)]
        metadata = self.presentation._hierarchy_render.frame_batch(requests, self.settings)
        for sample, frame in zip(samples, metadata, strict=True):
            with self.subTest(phase=sample["phase"], seconds=sample["seconds"]):
                self.assertAlmostEqual(frame["timeline"]["content_camera"], sample["progress"], places=6)
                camera = frame["browser"]["content_camera"]
                self.assertAlmostEqual(camera["progress"], sample["progress"], places=6)
                self.assertEqual(bool(frame["timeline"]["pressed"]), sample["phase"] == "restore" and sample["seconds"] == 1.1)
                if sample["progress"] > 0:
                    self.assertIsNone(frame["browser"]["cursor"]["asset"])
                if sample["progress"] == 1:
                    with Image.open(frame["path"]) as image:
                        np.testing.assert_array_equal(np.asarray(image.convert("RGB")), pixels)
                    target = camera["target"]
                    self.assertIsNotNone(target)
                    self.assertAlmostEqual(camera["scale"] * target["width"], 1600, delta=0.1)
                    self.assertAlmostEqual(camera["scale"] * target["height"], 900, delta=0.1)
                    self.assertAlmostEqual(camera["translation"][0] + camera["scale"] * target["x"], 0, delta=0.1)
                    self.assertAlmostEqual(camera["translation"][1] + camera["scale"] * target["y"], 0, delta=0.1)
        for index in (1, 4):
            camera = metadata[index]["browser"]["content_camera"]
            self.assertGreater(camera["scale"], 1)
            with Image.open(metadata[index]["path"]) as image:
                self.assertGreater(float(np.abs(np.asarray(image.convert("RGB")).astype(float) - pixels).mean()), 1)
        press, release = metadata[-2:]
        self.assertEqual([event["type"] for event in press["browser"]["pointer_events"]], ["pointerdown"])
        self.assertEqual([event["type"] for event in release["browser"]["pointer_events"]], ["pointerup", "click"])
        self.assertTrue(all(event["target"] == "PhysicalFundamentals_FullSection:fullscreen"
                            for frame in (press, release) for event in frame["browser"]["pointer_events"]))
        self.assertEqual(press["browser"]["content_camera"]["progress"], 0)
        window = next(window for window in press["browser"]["windows"] if window["id"] == "PhysicalFundamentals_FullSection")
        self.assertTrue(window["control_in_title_chip"])
        self.assertEqual(window["right_cap_control_count"], 0)
        for axis, size in (("x", "width"), ("y", "height")):
            self.assertGreaterEqual(window["control_rect"][axis], window["title_chip_rect"][axis] - 0.1)
            self.assertLessEqual(window["control_rect"][axis] + window["control_rect"][size],
                window["title_chip_rect"][axis] + window["title_chip_rect"][size] + 0.1)

    def test_resampled_content_uses_the_same_selected_source_frames_at_both_camera_handoffs(self):
        import av
        import numpy as np
        from PIL import Image

        case = CASES["frameRateConversion"]
        folder = self.output / "resampled-source"
        folder.mkdir()
        for index in range(8):
            Image.new("RGB", (320, 180), (20 + index * 25, 210 - index * 20, 40 + index * 15)).save(folder / f"{index:04d}.png")
        content, output = self.output / "resampled-source.mp4", self.output / "resampled-lesson.mp4"
        self.runtime.run_command([*self.runtime.ffmpeg_command(), "-y", "-v", "error", "-framerate", str(case["sourceFps"]),
                                  "-i", str(folder / "%04d.png"), "-c:v", "libx264", "-preset", "ultrafast",
                                  "-crf", "0", "-pix_fmt", "yuv420p", str(content)])

        def transition(video_id, snapshot, work, settings, session, *, phase):
            directory = work / phase
            directory.mkdir()
            with Image.open(snapshot) as image:
                for index in range(2):
                    image.save(directory / f"{index:04d}.png")
            return directory, 2

        from unittest.mock import patch

        with patch.object(self.presentation, "_transition", side_effect=transition):
            self.render_chapter_source(content, output, {**self.settings, "fps": case["outputFps"]})
        with av.open(str(content)) as container:
            originals = [np.asarray(frame.to_image().convert("RGB")).astype(float) for frame in container.decode(video=0)]
        with av.open(str(output)) as container:
            frames = [np.asarray(frame.to_image().convert("RGB")).astype(float) for frame in container.decode(video=0)]
        self.assertEqual(len(frames), 2 + len(case["sourceIndices"]) + 2)
        errors = [[float(np.abs(frame - source).mean()) for source in originals] for frame in frames[2:-2]]
        self.assertEqual([int(np.argmin(row)) for row in errors], case["sourceIndices"])
        for before, after in ((1, 2), (-3, -2)):
            self.assertLess(float(np.abs(frames[before] - frames[after]).mean()), case["maxHandoffMeanRgbError"])

    def test_edited_mp4_counts_decoded_frames_and_keeps_audio_duration_equal_to_visible_video(self):
        import av
        import numpy as np
        from PIL import Image
        from unittest.mock import patch

        case = CASES["editedSource"]
        base, content, output = (self.output / name for name in ("edit-base.mp4", "edit-content.mp4", "edit-lesson.mp4"))
        self.runtime.run_command([*self.runtime.ffmpeg_command(), "-y", "-v", "error", "-f", "lavfi", "-i",
                                  f'testsrc2=s=320x180:r={case["fps"]}:d={case["sourceSeconds"]}',
                                  "-c:v", "libx264", "-preset", "fast", "-crf", "0", "-pix_fmt", "yuv420p", str(base)])
        self.runtime.run_command([*self.runtime.ffmpeg_command(), "-y", "-v", "error", "-ss", str(case["seekSeconds"]),
                                  "-i", str(base), "-t", str(case["trimSeconds"]), "-c", "copy", str(content)])
        with av.open(str(content)) as container:
            samples = sum(packet.size > 0 for packet in container.demux(video=0))
        with av.open(str(content)) as container:
            decoded = sum(1 for frame in container.decode(video=0))
        self.assertEqual(samples, case["packetSamples"])
        self.assertEqual(decoded, case["decodedFrames"])

        def transition(video_id, snapshot, work, settings, session, *, phase):
            directory = work / phase
            directory.mkdir()
            with Image.open(snapshot) as image:
                for index in range(case["paddingFrames"]):
                    image.save(directory / f"{index:04d}.png")
            return directory, case["paddingFrames"]

        with patch.object(self.presentation, "_transition", side_effect=transition):
            self.render_chapter_source(content, output, {**self.settings, "fps": case["fps"]})
        with av.open(str(output)) as container:
            frames = [frame.to_image().convert("RGB") for frame in container.decode(video=0)]
        expected_frames = decoded + 2 * case["paddingFrames"]
        self.assertEqual(len(frames), expected_frames)
        self.assert_full_frame_source_order(content, frames, {**CASES["smoke"], "fps": case["fps"]},
                                             case["paddingFrames"] / case["fps"])
        with av.open(str(output)) as container:
            stream = container.streams.audio[0]
            blocks = [frame.to_ndarray().astype(float) for frame in container.decode(audio=0)]
            samples = np.concatenate(blocks, axis=1)
            self.assertLess(abs(samples.shape[1] / stream.codec_context.sample_rate - expected_frames / case["fps"]), 0.08)


if __name__ == "__main__":
    unittest.main()
