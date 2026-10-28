"""🧪 Verify audience selection, isolated provenance and independently decoded subtitle pixels."""

from __future__ import annotations

import importlib
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class AudienceTest(unittest.TestCase):
    def test_contract_and_invalid_audiences(self):
        audience = importlib.import_module(PACKAGE + ".🎬️clip.👥️audience.🐍️")
        self.assertEqual(list(audience.audiences()), CASES["variants"])
        for value, visible in CASES["subtitles"].items():
            self.assertEqual(audience.subtitles_visible(value), visible)
        for value in CASES["invalid"]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                audience.subtitles_visible(value)

    def test_cli_defaults_to_both_and_deduplicates_selection(self):
        render = importlib.import_module(PACKAGE + ".🎬️clip.🖌️render.🐍️")
        for selection, expected in (([], CASES["variants"]), (["spectator", "spectator"], ["spectator"])):
            arguments = ["--language", "de", "--video", "physical_fundamentals", "--no-play"]
            for audience in selection:
                arguments.extend(["--audience", audience])
            with patch.object(render, "compose_videos") as compose:
                self.assertEqual(render.main(arguments), 0)
            self.assertEqual([call.kwargs["audience"] for call in compose.call_args_list], expected)

    def test_cache_and_output_paths_are_isolated_for_lessons_and_chapters(self):
        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        segment = {"script": "fixture.py", "scene": "Fixture"}
        video = {"id": "fixture", "chapters": [{"scene": "Chapter", "segments": [segment]}], "output": "Fixture"}
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            calls, envelopes = [], []

            def render(segment, media, settings, **kwargs):
                calls.append((media, settings.copy(), kwargs["output_root"]))
                movie = media / "v" / "Fixture.mp4"
                movie.parent.mkdir(parents=True, exist_ok=True)
                movie.write_bytes(b"0" * 1200)
                runtime._write_state(movie.with_suffix(".render.json"), runtime.segment_state(segment, settings))
                return movie

            def concat(clips, output, list_path):
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_bytes(b"0" * 1200)

            def envelope(video_id, content, output, content_state, settings, language, **kwargs):
                envelopes.append((output, settings["audience"]))
                return output

            with patch.object(runtime, "load_manifest", return_value=[video]), patch.object(runtime, "segment_state", side_effect=lambda segment, settings: {"segment": segment, "settings": settings}), patch.object(runtime, "render_segment", side_effect=render), patch.object(runtime, "concat_videos", side_effect=concat), patch.object(runtime, "publish_presentation", side_effect=envelope):
                for audience in CASES["variants"]:
                    runtime.compose_videos(language="de", audience=audience, output_root=folder)
                    runtime.compose_videos(language="de", audience=audience, output_root=folder, concat_only=True)
                    runtime.compose_videos(language="de", audience=audience, output_root=folder, chapter_scenes=["Chapter"], concat_only=True)
            self.assertEqual(len(calls), 2)
            self.assertNotEqual(calls[0][0], calls[1][0])
            self.assertNotEqual(calls[0][2], calls[1][2])
            self.assertEqual([settings["audience"] for _, settings, _ in calls], CASES["variants"])
            for output, audience in envelopes:
                self.assertIn("_" + audience + "_", output.name)

    def test_segment_passes_audience_to_subprocess(self):
        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        segment = runtime.segment_plan()[0]["segment"]
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            for audience in CASES["variants"]:
                media = Path(directory) / audience
                clip = media / "v" / f"{segment['scene']}.mp4"
                clip.parent.mkdir(parents=True)
                clip.write_bytes(b"0" * 1200)
                settings = {"quality": "-ql", "resolution": None, "fps": None, "audience": audience}
                with patch.object(runtime, "run_command") as command:
                    runtime.render_segment(segment, media, settings)
                self.assertEqual(command.call_args.kwargs["env"]["SEMIO_TEACHING_AUDIENCE"], audience)

    def test_audience_changes_invalidate_source_and_presentation_provenance(self):
        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        segment = runtime.segment_plan()[0]["segment"]
        states, envelopes = [], []
        for audience in CASES["variants"]:
            settings = {"quality": "-ql", "resolution": None, "fps": None, "audience": audience}
            states.append(runtime.segment_state(segment, settings))
            envelopes.append(runtime.presentation_state("physical_fundamentals", settings, "de"))
        self.assertNotEqual(states[0], states[1])
        self.assertNotEqual(envelopes[0], envelopes[1])

    @unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable native Manim/PyAV audience validation")
    def test_real_subprocess_renders_use_the_selected_audience(self):
        import av
        import numpy as np

        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            script = folder / "source" / "scene.py"
            script.parent.mkdir()
            script.write_text(
                "from manim import Scene, Square, FadeIn\n"
                "from importlib import import_module\n"
                "import sys\n"
                f"sys.path.insert(0, {str(runtime._WORKSPACE_ROOT)!r})\n"
                f"visuals = import_module({PACKAGE + '.🎨️visuals.🐍️'!r})\n"
                "class Fixture(Scene):\n"
                "    def construct(self):\n"
                "        self.add(Square(side_length=1, fill_opacity=1, color='#ff1493'))\n"
                f"        caption = visuals.caption_bar({CASES['render']['clause']!r})\n"
                "        self.play(FadeIn(caption), run_time=0.2)\n"
                "        self.wait(0.4)\n", encoding="utf-8")
            frames, clocks = {}, {}
            for audience in CASES["variants"]:
                settings = {"quality": "-ql", "resolution": CASES["render"]["resolution"],
                            "fps": CASES["render"]["fps"], "audience": audience}
                movie = runtime.render_segment({"script": str(script), "scene": "Fixture"}, folder / audience, settings)
                with av.open(movie) as container:
                    decoded = list(container.decode(video=0))
                    frames[audience] = decoded[4].to_ndarray(format="rgb24")
                    clocks[audience] = [float(frame.time) for frame in decoded]
            np.testing.assert_allclose(clocks["speaker"], clocks["spectator"], atol=CASES["duration_tolerance"])
            self.assertLessEqual(int(frames["spectator"][140:].max()), 5)
            self.assertGreater(np.count_nonzero(frames["speaker"][140:] > 20), CASES["subtitle_difference_min"])

    @unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable native Manim/PyAV audience validation")
    def test_real_caption_transitions_have_identical_timing_and_no_spectator_pixels(self):
        import av
        import numpy as np
        from manim import FadeIn, FadeOut, Scene, Square, tempconfig

        visuals = importlib.import_module(PACKAGE + ".🎨️visuals.🐍️")
        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        case = CASES["render"]

        class Fixture(Scene):
            def construct(self):
                self.add(Square(side_length=1, fill_opacity=1, color="#ff1493"))
                caption = visuals.caption_bar(case["clause"])
                self.play(FadeIn(caption), run_time=0.2)
                self.wait(case["hold"])
                caption = visuals.swap_caption(self, caption, case["swap"], run_time=0.4)
                self.wait(case["hold"])
                self.play(FadeOut(caption), run_time=0.2)

        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            decoded, times = {}, {}
            for audience in CASES["variants"]:
                media = Path(directory) / audience
                runtime.render_config(media)
                with patch.dict(os.environ, {"SEMIO_TEACHING_AUDIENCE": audience}), tempconfig({
                    "media_dir": str(media), "video_dir": str(media / "v"), "partial_movie_dir": str(media / "p"),
                    "text_dir": str(media / "t"), "tex_dir": str(media / "x"), "pixel_width": 320,
                    "pixel_height": 180, "frame_rate": case["fps"], "disable_caching": True,
                    "verbosity": "WARNING", "progress_bar": "none",
                }):
                    Fixture().render()
                with av.open(media / "v" / "Fixture.mp4") as movie:
                    frames = list(movie.decode(video=0))
                    decoded[audience] = [frame.to_ndarray(format="rgb24") for frame in frames]
                    times[audience] = [float(frame.time) for frame in frames]
            np.testing.assert_allclose(times["speaker"], times["spectator"], atol=CASES["duration_tolerance"])
            reference = decoded["spectator"][0]
            for frame in decoded["spectator"]:
                np.testing.assert_array_equal(frame[140:], reference[140:])
                self.assertLessEqual(int(frame[140:].max()), 5)
            speaker, spectator = decoded["speaker"][4], decoded["spectator"][4]
            self.assertGreater(np.count_nonzero(np.abs(speaker[140:].astype(int) - spectator[140:].astype(int)) > 20), CASES["subtitle_difference_min"])
            self.assertLess(float(np.abs(speaker[:100].astype(int) - spectator[:100].astype(int)).mean()), 1)


if __name__ == "__main__":
    unittest.main()
