"""📚 Validate authored chapter coverage, recursive paths and exact neighboring contexts."""
import importlib
import json
from pathlib import Path
import sys
import unittest

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
PRESENTATION = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation"
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"


class RecursiveChapterPacketContract(unittest.TestCase):
    """📚 Require authored four-level identities and deterministic chapter handoffs."""

    def test_browser_schema_accepts_four_level_open_and_reveal_paths(self):
        """🧬 Require recursive array controls in the actual browser protocol schema."""
        schema = json.loads((PRESENTATION / "🌐️browser/🧬️schema.json").read_text(encoding="utf-8"))
        controls = schema["$defs"]["hierarchy"]
        for name in ("opened_path", "revealed_path"):
            self.assertIn(name, controls["properties"])
            self.assertEqual(controls["properties"][name]["type"], "array")
        self.assertFalse({"opened_topic", "opened_video"} & set(controls.get("required", ())))

    def test_portable_coverage_matches_authored_sources_and_locales(self):
        """🔗 Join each authored course, topic, clip, chapter and canonical segment exactly once."""
        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        metadata = importlib.import_module(PACKAGE + ".📚️collection.🐍️").collection_contract()
        teaching = ROOT / "🎓️teaching"
        course = json.loads((teaching / metadata["course"]).read_text(encoding="utf-8"))
        manifest = json.loads((teaching / metadata["manifest"]).read_text(encoding="utf-8"))["videos"]
        self.assertEqual(course["id"], CASES["course"])
        self.assertEqual([video["id"] for video in manifest], [video["id"] for video in CASES["clips"]])
        expected = {chapter["scene"] for video in CASES["clips"] for chapter in video["chapters"]}
        self.assertEqual(set(metadata["chapters"]), expected)
        catalog = hierarchy.hierarchy_contract()
        self.assertEqual(len(catalog["courses"]), 1)
        actual_course = catalog["courses"][0]
        self.assertEqual(actual_course["id"], course["id"])
        self.assertEqual(actual_course["title"], course["title"])
        self.assertEqual({topic["id"] for topic in actual_course["topics"]}, {topic["id"] for topic in course["topics"]})
        sources, occurrences, clauses = set(), 0, 0
        for video, golden in zip(manifest, CASES["clips"], strict=True):
            self.assertEqual(video["chapters"], golden["chapters"])
            topic = next(topic for topic in actual_course["topics"] if topic["id"] == golden["topic"])
            projected = next(clip for clip in topic["videos"] if clip["id"] == video["id"])
            self.assertEqual([chapter["id"] for chapter in projected["chapters"]], [chapter["scene"] for chapter in golden["chapters"]])
            for chapter in projected["chapters"]:
                self.assertEqual(chapter["title"], metadata["chapters"][chapter["id"]])
                self.assertEqual(set(chapter["title"]), set(CASES["locales"]))
                self.assertTrue(all(chapter["title"][locale].strip() for locale in CASES["locales"]))
            for chapter in golden["chapters"]:
                for segment in chapter["segments"]:
                    sources.add((segment["script"], segment["scene"]))
                    occurrences += 1
                    clauses += "clauses" in segment
        self.assertEqual(len(expected), CASES["counts"]["chapters"])
        self.assertEqual(len(sources), CASES["counts"]["canonical_sources"])
        self.assertEqual(occurrences, CASES["counts"]["segment_occurrences"])
        self.assertEqual(clauses, CASES["counts"]["clause_selections"])

    def test_paths_neighbors_and_boundaries_match_independent_numpy_ancestors(self):
        """📐 Compare all thirty-nine boundaries with independently projected ancestor depths."""
        import numpy as np

        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        entries = [(chapter["scene"], [CASES["course"], video["topic"], video["id"], chapter["scene"]])
                   for video in CASES["clips"] for chapter in video["chapters"]]
        identifiers = [entry[0] for entry in entries]
        self.assertEqual([chapter["id"] for chapter in hierarchy.ordered_chapters()], identifiers)
        paths = np.asarray([entry[1] for entry in entries])
        depths = np.r_[0, np.cumprod(paths[:-1, :3] == paths[1:, :3], axis=1).sum(axis=1), 0].astype(int)
        self.assertEqual(depths.tolist(), [len(case["path"]) for case in CASES["boundaries"]])
        for index, (identifier, path) in enumerate(entries):
            with self.subTest(chapter=identifier):
                self.assertEqual(hierarchy.path(identifier), path)
                chapter = hierarchy.chapter(identifier)
                self.assertEqual(chapter["id"], identifier)
                self.assertEqual(chapter["clip_id"], path[2])
                self.assertEqual(hierarchy.chapter_neighbors(identifier), {
                    "previous": identifiers[index - 1] if index else None,
                    "next": identifiers[index + 1] if index + 1 < len(identifiers) else None})
        for case in CASES["boundaries"]:
            with self.subTest(left=case["left"], right=case["right"]):
                self.assertEqual(hierarchy.boundary(case["left"], case["right"]), {"path": case["path"], "level": case["level"]})
        for function in (hierarchy.chapter, hierarchy.path, hierarchy.chapter_neighbors):
            with self.assertRaises((ValueError, KeyError)):
                function("unknown-chapter")


    def test_routes_commit_only_real_clicks_and_preserve_boundary_context(self):
        """🖱️ Verify every opening and closing route against independent stage boundaries."""
        import numpy as np

        renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        catalog = hierarchy.hierarchy_contract()
        self.assertEqual(set(renderer.initial_hierarchy()), {"opened_path", "revealed_path", "camera"})
        for index, chapter in enumerate(hierarchy.ordered_chapters()):
            scene = chapter["id"]
            branch = hierarchy.path(scene)
            controls = {operation: {identity: [100 + 170 * depth, 180 + 70 * depth] for depth, identity in enumerate(branch)}
                        for operation in ("open", "close", "preview")}
            for phase in ("focus", "restore"):
                with self.subTest(chapter=scene, phase=phase):
                    case = CASES["boundaries"][index if phase == "focus" else index + 1]
                    prefix = case["path"]
                    steps = branch[len(prefix):]
                    if phase == "restore":
                        steps = list(reversed(steps))
                    timing = catalog["timing" if phase == "focus" else "restoreTiming"]
                    stage = np.array([timing[key] for key in ("topicTravel", "topicReveal", "topicPress", "topicHold")])
                    camera = catalog["contentCamera"]["duration"]
                    duration = len(steps) * stage.sum() + timing["focusHold"] + camera
                    self.assertAlmostEqual(renderer.transition_duration(scene, phase), duration)
                    beginning = renderer.choreography(scene, phase, 0, controls)
                    ending = renderer.choreography(scene, phase, 1, controls)
                    self.assertEqual(beginning["hierarchy"]["opened_path"], prefix if phase == "focus" else branch)
                    self.assertEqual(ending["hierarchy"]["opened_path"], branch if phase == "focus" else prefix)
                    self.assertEqual(ending["hierarchy"]["revealed_path"], [])
                    self.assertFalse(ending["pressed"])
                    self.assertEqual([effect["target"] for effect in ending["effects"]], steps)
                    self.assertEqual([event["type"] for event in ending["events"]], ["move", "press", "release", "click"] * len(steps))
                    offset = camera if phase == "restore" else 0
                    for ordinal, identity in enumerate(steps):
                        released = offset + ordinal * stage.sum() + stage[:3].sum()
                        before = renderer.choreography(scene, phase, (released - 1e-7) / duration, controls)
                        after = renderer.choreography(scene, phase, (released + 1e-7) / duration, controls)
                        self.assertEqual(len(before["effects"]), ordinal)
                        self.assertEqual(len(after["effects"]), ordinal + 1)
                        self.assertTrue(before["pressed"])
                        self.assertFalse(after["pressed"])
                        self.assertEqual([event["type"] for event in after["events"]][-2:], ["release", "click"])
                    camera_start = duration - camera if phase == "focus" else 0
                    middle = renderer.choreography(scene, phase, (camera_start + camera / 2) / duration, controls)
                    self.assertAlmostEqual(middle["content_camera"], 0.5)
                    self.assertEqual(beginning["content_camera"], 0 if phase == "focus" else 1)
                    self.assertEqual(ending["content_camera"], 1 if phase == "focus" else 0)


class RecursiveChapterBrowserContract(unittest.TestCase):
    """🌐 Measure the actual shared UI at every recursive collection depth."""

    def test_actual_dom_matches_course_topic_clip_chapter_and_leaf_paths(self):
        """🖼️ Capture both locales through the real browser adapter and inspect actual DOM records."""
        import os
        import tempfile

        browser = importlib.import_module(PACKAGE + ".🌐️browser.🐍️")
        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        catalog = hierarchy.hierarchy_contract()
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()
        output.mkdir(parents=True, exist_ok=True)
        work = Path(tempfile.mkdtemp(prefix="chapter-dom-", dir=output))
        golden = next(video for video in CASES["clips"] if video["id"] == "ventilation")
        selected = golden["chapters"][1]["scene"]
        full = [CASES["course"], golden["topic"], golden["id"], selected]
        expected_cards = [[CASES["course"]], CASES["topics"],
                          [video["id"] for video in CASES["clips"] if video["topic"] == golden["topic"]],
                          [chapter["scene"] for chapter in golden["chapters"]]]
        for language in CASES["locales"]:
            frames = [{"output": str(work / (language + "-" + str(depth) + ".png")), "media": [],
                       "hierarchy": {"opened_path": full[:depth], "revealed_path": [], "camera": {"x": 0.5, "y": 0.5}},
                       "cursor": {"point": None}, "transition_elapsed": 1800} for depth in range(5)]
            result = browser.render_batch({"schema": "semio-presentation-browser/v1", "viewport": {"width": 1600, "height": 900},
                                           "catalog": catalog, "language": language, "work_directory": str(work), "frames": frames})
            (work / (language + "-dom.json")).write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
            self.assertEqual(len(result["metadata"]), 5)
            for depth, actual in enumerate(result["metadata"]):
                with self.subTest(language=language, depth=depth):
                    self.assertEqual(actual["hierarchy"], {"opened_path": full[:depth], "revealed_path": []})
                    if depth < 4:
                        level = "root" if depth == 0 else full[depth - 1]
                        layer = next(layer for layer in actual["layers"] if layer["id"] == level)
                        self.assertTrue(layer["interactive"])
                        self.assertEqual([card["id"] for card in layer["cards"]], expected_cards[depth])
                        self.assertTrue(all(card["fullscreen_in_title_chip"] for card in layer["cards"]))
                    else:
                        leaf = next(window for window in actual["windows"] if window["id"] == selected)
                        self.assertEqual(leaf["title_text"].strip(), hierarchy.chapter(selected)["title"][language])
                        self.assertTrue(leaf["control_in_title_chip"])
                    self.assertEqual(len(actual["footer_ancestors"]), depth)
        print("[DEBUG] Actual chapter DOM evidence", work, flush=True)


    def test_actual_choreography_has_exact_adjacent_lca_pixels_and_content_only_endpoint(self):
        """🎞️ Compare actual endpoint captures and a solid authored body with Pillow and NumPy."""
        import os
        import tempfile

        import numpy as np
        from PIL import Image

        renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()
        output.mkdir(parents=True, exist_ok=True)
        work = Path(tempfile.mkdtemp(prefix="chapter-join-", dir=output))
        selected = [next(case for case in CASES["boundaries"] if case["level"] == level)
                    for level in ("chapters", "clips", "topics")]
        requests = [{"video_id": CASES["boundaries"][0]["right"], "phase": "focus", "t": 0, "output": work / "first.png"}]
        pairs = []
        for index, case in enumerate(selected):
            left, right = work / (str(index) + "-left.png"), work / (str(index) + "-right.png")
            requests += [{"video_id": case["left"], "phase": "restore", "t": 1, "output": left},
                         {"video_id": case["right"], "phase": "focus", "t": 0, "output": right}]
            pairs.append((left, right, case["path"]))
        requests += [{"video_id": CASES["boundaries"][-1]["left"], "phase": "restore", "t": 1, "output": work / "last.png"}]
        settings = {"language": "de", "audience": "spectator", "resolution": "480,270", "fps": 4.0, "quality": "-ql"}
        measured = renderer.frame_batch(requests, settings)
        self.assertEqual(len(measured), len(requests))
        for request, result in zip(requests, measured, strict=True):
            self.assertEqual(result["timeline"]["hierarchy"]["revealed_path"], [])
            self.assertEqual(result["timeline"]["content_camera"], 0)
        for left, right, prefix in pairs + [(work / "first.png", work / "last.png", [])]:
            np.testing.assert_array_equal(np.asarray(Image.open(left)), np.asarray(Image.open(right)))
        color = [31, 47, 59]
        body = work / "body.png"
        Image.new("RGB", (1600, 900), tuple(color)).save(body)
        target = work / "content.png"
        renderer.frame_batch([{"video_id": CASES["boundaries"][0]["right"], "phase": "focus", "t": 1,
                               "snapshot": body, "output": target}], settings)
        actual = np.asarray(Image.open(target).convert("RGB"), dtype=np.int16)
        self.assertLessEqual(int(np.abs(actual - color).max()), 1)
        print("[DEBUG] Actual chapter join evidence", work, flush=True)


    def capture_uncached_browser_process_endpoint_pixels(self):
        """🪟 Capture each ancestor independently twice without the Python raster cache."""
        import os
        import tempfile

        import numpy as np
        from PIL import Image

        browser = importlib.import_module(PACKAGE + ".🌐️browser.🐍️")
        hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
        renderer = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()
        output.mkdir(parents=True, exist_ok=True)
        work = Path(tempfile.mkdtemp(prefix="chapter-fresh-", dir=output))
        cases = [next(case for case in CASES["boundaries"] if case["level"] == level)
                 for level in ("courses", "topics", "clips", "chapters")]
        packets = []
        for capture in range(2):
            directory = work / str(capture)
            frames = []
            for index, case in enumerate(cases):
                branch = hierarchy.path(case["right"])
                controls = {operation: {identity: [100, 200] for identity in branch} for operation in ("open", "close", "preview")}
                projected = renderer.choreography(case["right"], "focus", 0, controls)
                self.assertEqual(projected["hierarchy"]["opened_path"], case["path"])
                frames.append({"output": str(directory / (str(index) + ".png")), "media": [],
                               "hierarchy": projected["hierarchy"], "canonical_overview": True,
                               "cursor": {"point": projected["point"], "kind": "arrow", "pressed": False},
                               "transition_elapsed": 2000})
            packets.append({"schema": "semio-presentation-browser/v1", "viewport": {"width": 480, "height": 270},
                            "catalog": hierarchy.hierarchy_contract(), "language": "de", "work_directory": str(directory), "frames": frames})
        for packet in packets:
            directory = Path(packet["work_directory"])
            directory.mkdir(parents=True, exist_ok=True)
            (directory / "packet.json").write_text(json.dumps(packet, ensure_ascii=False, indent=2), encoding="utf-8")
            result = browser._call(packet)
            (directory / "dom.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
        for index, case in enumerate(cases):
            with self.subTest(level=case["level"]):
                np.testing.assert_array_equal(np.asarray(Image.open(work / "0" / (str(index) + ".png"))),
                                              np.asarray(Image.open(work / "1" / (str(index) + ".png"))))
        print("[DEBUG] Independent-process chapter endpoint evidence", work, flush=True)

    def test_public_boundary_producer_and_consumer_processes_share_exact_pixels(self):
        """🪟 Prove four actual ancestor assets survive fresh Python and browser processes."""
        import os
        import tempfile
        from unittest.mock import patch

        import numpy as np
        from PIL import Image

        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        browser = importlib.import_module(PACKAGE + ".🌐️browser.🐍️")
        work = Path(tempfile.mkdtemp(prefix="boundary-process-", dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]))
        script = Path(__file__).with_name("🐍️process.py")
        with patch.dict(os.environ, {"SEMIO_TEACHING_OUTPUT_ROOT": str(work / "shared")}):
            for phase in ("producer", "consumer"):
                runtime.run_command([browser._repo(), "tools", "exec", "--light", "python", "--", "-X", "utf8",
                    str(script), phase, str(work)], cwd=ROOT, log=work / (phase + ".log"))
        producer, consumer = (json.loads((work / phase / "result.json").read_text(encoding="utf-8"))
                              for phase in ("producer", "consumer"))
        self.assertNotEqual(producer["pid"], consumer["pid"])
        self.assertEqual(len(producer["frames"]), 4)
        for first, second in zip(producer["frames"], consumer["frames"], strict=True):
            self.assertFalse(first["handoff"]["reused"])
            self.assertTrue(second["handoff"]["reused"])
            self.assertEqual(first["sha256"], second["sha256"])
            self.assertEqual(Path(first["path"]).read_bytes(), Path(second["path"]).read_bytes())
            np.testing.assert_array_equal(np.asarray(Image.open(first["path"])), np.asarray(Image.open(second["path"])))
        print("[DEBUG] Public cross-process boundary evidence", work, flush=True)


class ChapterMediaPortContract(unittest.TestCase):
    """📽️ Compare finite production media ports with independent PyAV fixtures."""

    def test_repeated_body_measurements_share_an_owned_work_directory(self):
        """🔢 Measure the same independent finite source twice without colliding on diagnostic logs."""
        from fractions import Fraction
        import os
        import tempfile

        import av
        import numpy as np

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()
        output.mkdir(parents=True, exist_ok=True)
        work = Path(tempfile.mkdtemp(prefix="chapter-clock-", dir=output))
        source = work / "body.mp4"
        case = CASES["media_probe"]
        with av.open(str(source), "w") as container:
            stream = container.add_stream("libx264", rate=case["fps"])
            stream.width, stream.height, stream.pix_fmt = case["width"], case["height"], "yuv420p"
            stream.options = {"crf": "0", "threads": "1"}
            pixels = np.full((case["height"], case["width"], 3), case["color"], dtype=np.uint8)
            for index in range(case["frames"]):
                frame = av.VideoFrame.from_ndarray(pixels, format="rgb24")
                frame.pts, frame.time_base = index, Fraction(1, case["fps"])
                for packet in stream.encode(frame):
                    container.mux(packet)
            for packet in stream.encode():
                container.mux(packet)
        collection = importlib.import_module(PACKAGE + ".📚️collection.🐍️")
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        settings = {"language": "de", "audience": "spectator", "resolution": "480,270", "fps": 4.0, "quality": "-ql"}
        session = runtime.ToolSession()
        for expected in case["measurements"]:
            self.assertEqual(collection.body_frames(source, settings, work, session), expected)


    def test_cancelled_chapter_stops_before_resolving_any_ports(self):
        """🛑 Reject a pre-cancelled publication before imports, hashes or child work."""
        from types import SimpleNamespace
        from unittest.mock import patch
        import threading

        collection = importlib.import_module(PACKAGE + ".📚️collection.🐍️")
        cancelled = threading.Event()
        cancelled.set()
        with patch.object(collection, "_ports", side_effect=AssertionError("cancelled work resolved ports")):
            with self.assertRaises(InterruptedError):
                collection.publish_chapter("physical_fundamentals", Path("missing-body.mp4"), Path("missing-output.mp4"),
                    {}, {"language": "de"}, SimpleNamespace(cancelled=cancelled))

    def test_parent_cancel_during_render_preserves_current_publication(self):
        """🧯 Preserve the prior complete output and certificate when the shared session cancels."""
        import os
        from types import SimpleNamespace
        import tempfile
        import threading
        from unittest.mock import patch

        collection = importlib.import_module(PACKAGE + ".📚️collection.🐍️")
        directory = Path(tempfile.mkdtemp(prefix="chapter-cancel-", dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]))
        output = directory / "published.mp4"
        certificate = output.with_suffix(".render.json")
        output.write_text(CASES["cancellation"]["preserve_movie"], encoding="utf-8")
        certificate.write_text(CASES["cancellation"]["preserve_certificate"], encoding="utf-8")
        session = SimpleNamespace(cancelled=threading.Event())
        calls = []

        def render(video_id, raw, candidate, settings, **kwargs):
            """🎬 Simulate a completed candidate after the enclosing session was cancelled."""
            self.assertIs(kwargs["session"], session)
            calls.append(candidate)
            candidate.write_bytes(b"x" * 2000)
            session.cancelled.set()

        runtime = SimpleNamespace(_cursor_hash=lambda raw: "cursor", presentation_state=lambda *args: {"current": True},
                                  reusable_clip=lambda *args: False, state_inputs_current=lambda *args: True)
        hierarchy = SimpleNamespace(chapter=lambda scene: {"clip_id": "physical_fundamentals"})
        with patch.object(collection, "_ports", return_value=(runtime, SimpleNamespace(render_presentation=render), hierarchy)):
            with self.assertRaises(InterruptedError):
                collection.publish_chapter("physical_fundamentals", directory / "raw.mp4", output,
                    {"scene": "PhysicalFundamentals_FullSection"}, {"language": "de"}, session)
        self.assertEqual(len(calls), 1)
        self.assertFalse(calls[0].exists())
        self.assertEqual(output.read_text(encoding="utf-8"), CASES["cancellation"]["preserve_movie"])
        self.assertEqual(certificate.read_text(encoding="utf-8"), CASES["cancellation"]["preserve_certificate"])


    def test_native_encoder_preserves_all_finite_transition_and_body_frames(self):
        """🎞️ Decode real FFmpeg output against an independent PyAV body and real finite PNG sequences."""
        from fractions import Fraction
        import os
        import tempfile
        import shutil
        from unittest.mock import patch

        import av
        import numpy as np
        from PIL import Image

        work = Path(tempfile.mkdtemp(prefix="chapter-encode-", dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]))
        source, output = work / "body.mp4", work / "chapter.mp4"
        case = CASES["encoder"]
        with av.open(str(source), "w") as container:
            stream = container.add_stream("libx264", rate=4)
            stream.width, stream.height, stream.pix_fmt = 480, 270, "yuv420p"
            stream.options = {"crf": "0", "threads": "1"}
            for index, value in enumerate(case["body_values"]):
                frame = av.VideoFrame.from_ndarray(np.full((270, 480, 3), value, dtype=np.uint8), format="rgb24")
                frame.pts, frame.time_base = index, Fraction(1, 4)
                for packet in stream.encode(frame):
                    container.mux(packet)
            for packet in stream.encode():
                container.mux(packet)
        presentation = importlib.import_module(PACKAGE + ".🐍️")
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        settings = {"language": "de", "audience": "spectator", "resolution": "480,270", "fps": 4.0, "quality": "-ql"}
        render_work = work / "render"
        render_work.mkdir()

        def transition(scene, snapshot, directory, settings, session, *, phase):
            """🖼️ Supply actual finite raster files while isolating only the previously proven UI renderer."""
            folder = directory / phase
            folder.mkdir()
            count = case["focus_frames" if phase == "focus" else "restore_frames"]
            color = case["focus_color" if phase == "focus" else "restore_color"]
            for index in range(count):
                target = folder / (str(index).zfill(4) + ".png")
                if phase == "focus" and index == count - 1 or phase == "restore" and index == 0:
                    shutil.copyfile(snapshot, target)
                else:
                    Image.new("RGB", (480, 270), tuple(color)).save(target)
            return folder, count

        with patch.object(presentation, "_transition", side_effect=transition):
            presentation._render_chapter("PhysicalFundamentals_FullSection", source, output, settings, runtime.ToolSession(), render_work)
        with av.open(str(source)) as media:
            original = [frame.to_ndarray(format="rgb24") for frame in media.decode(video=0)]
        with av.open(str(output)) as media:
            decoded = list(media.decode(video=0))
            self.assertEqual(len(decoded), case["frames"])
            self.assertEqual(float(media.streams.video[0].average_rate), 4)
            pixels = [frame.to_ndarray(format="rgb24") for frame in decoded]
            np.testing.assert_allclose([float(frame.pts * frame.time_base) for frame in decoded], np.arange(case["frames"]) / 4, atol=1e-8)
        for actual, expected in zip(pixels[case["focus_frames"]:case["focus_frames"] + len(original)], original, strict=True):
            np.testing.assert_array_equal(actual, expected)
        for index, frame in enumerate(pixels[:case["focus_frames"]]):
            expected = original[0] if index == case["focus_frames"] - 1 else case["focus_color"]
            self.assertLessEqual(int(np.abs(frame.astype(np.int16) - expected).max()), 2)
        for index, frame in enumerate(pixels[-case["restore_frames"]:]):
            expected = original[-1] if index == 0 else case["restore_color"]
            self.assertLessEqual(int(np.abs(frame.astype(np.int16) - expected).max()), 2)
        print("[DEBUG] Finite native encoder evidence", work, flush=True)


def load_tests(loader, tests, pattern):
    """🧪 Include the nearby native pointer driver in the registered chapter feature."""
    tests.addTests(loader.loadTestsFromName(PACKAGE + ".🥞️hierarchy.🧪️tests.🧩️chapters.🐍️pointer"))
    return tests


if __name__ == "__main__":
    unittest.main()
