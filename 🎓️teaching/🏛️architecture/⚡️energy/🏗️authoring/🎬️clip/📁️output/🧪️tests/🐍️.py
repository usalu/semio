"""🧳 Portable output contracts validated against Manim and Git."""

import importlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

MODULE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️"
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
WORKSPACE = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())


class OutputTest(unittest.TestCase):
    def test_generated_narration_is_read_and_invalidates_render_cache(self):
        import wave
        output = importlib.import_module(MODULE)
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        case = CASES["narration"]
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            folder = Path(directory)
            source = folder / "scene.py"
            narration = [(case["key"], case["english"], case["german"])]
            source.write_text(f"class {case['scene']}:\n    NARRATION = {narration!r}\n", encoding="utf-8")
            with patch.dict(os.environ, {"SEMIO_TEACHING_OUTPUT_ROOT": str(folder)}), patch.object(runtime, "teaching_root", return_value=folder):
                clause = output.speech_root(folder) / case["file"]
                clause.parent.mkdir(parents=True)
                with wave.open(str(clause), "wb") as audio:
                    audio.setparams((1, 2, 8000, 0, "NONE", "not compressed"))
                    audio.writeframes(b"\0" * 1600)
                segment = {"script": "scene.py", "scene": case["scene"]}
                self.assertEqual(runtime.source_narration(segment), [(case["key"], clause)])
                before = runtime.segment_state(segment, {})
                clause.write_bytes(clause.read_bytes() + b"\0\0")
                self.assertNotEqual(before, runtime.segment_state(segment, {}))

    def test_narration_writers_and_composition_readers_share_generated_paths(self):
        output = importlib.import_module(MODULE)
        energy = WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy"
        for path in sorted(energy.glob("*/🎬️clip/*/🔊️speech/🐍️.py")):
            name = ".".join(path.relative_to(WORKSPACE).with_suffix("").parts)
            with self.subTest(module=name):
                speech = importlib.import_module(name)
                self.assertEqual(speech.SPEECH_DIR, output.speech_root(speech.BASE_DIR))
                if hasattr(speech, "TIMING_MANIFEST"):
                    scene = importlib.import_module(name.rsplit(".", 2)[0] + ".🐍️")
                    self.assertEqual(speech.TIMING_MANIFEST, scene._VO_TIMING)
                    self.assertEqual(speech.TIMING_MANIFEST, speech.SPEECH_DIR / "timing.json")
                composition = path.parent.parent / "🏗️composition/🐍️.py"
                if composition.is_file():
                    module = importlib.import_module(".".join(composition.relative_to(WORKSPACE).with_suffix("").parts))
                    if hasattr(module, "SPEECH_DIR"):
                        self.assertEqual(module.SPEECH_DIR, speech.SPEECH_DIR)

    def test_output_precedence_is_independent_of_working_directory(self):
        output = importlib.import_module(MODULE)
        for case in CASES["roots"]:
            with self.subTest(case=case), patch.dict(os.environ, case["environment"], clear=True):
                self.assertEqual(output.output_root(), WORKSPACE / case["output"])

    def test_speech_outputs_are_separate_from_authored_sources(self):
        output = importlib.import_module(MODULE)
        owners = [WORKSPACE / "🎓️teaching/a/🎬️clip/beat", WORKSPACE / "🎓️teaching/b/🎬️clip/beat"]
        paths = [output.speech_root(owner) for owner in owners]
        self.assertNotEqual(*paths)
        for path in paths:
            self.assertEqual(path.parent, output.output_root() / CASES["speech"]["directory"])
            self.assertEqual(len(path.name), CASES["speech"]["identity_length"])

    def test_default_manim_paths_and_real_pango_output(self):
        from manim import Text, config, tempconfig
        output = importlib.import_module(MODULE)
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            folder = Path(directory)
            with patch.dict(os.environ, {"SEMIO_TEACHING_OUTPUT_ROOT": str(folder)}), tempconfig({"media_dir": "./media"}):
                output.configure_manim()
                self.assertEqual(Path(config.media_dir), folder / "m")
                for key, suffix in CASES["directories"].items():
                    self.assertEqual(config[key], "{media_dir}/" + suffix)
                text = Text(CASES["text"])
                self.assertGreater(text.width, 0)
                self.assertTrue(list((folder / "m/t").glob("*.svg")))
                self.assertFalse((folder / "media").exists())

    def test_explicit_manim_media_directory_is_preserved(self):
        from manim import config, tempconfig
        output = importlib.import_module(MODULE)
        with tempconfig({"media_dir": "explicit-render", "text_dir": "explicit-text"}):
            output.configure_manim()
            self.assertEqual(config.media_dir, "explicit-render")
            self.assertEqual(config.text_dir, "explicit-text")

    def test_generated_render_config_matches_upstream_path_resolution(self):
        from manim import config
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            folder = Path(directory)
            generated = runtime.render_config(folder / "m")
            oracle = config.copy()
            oracle.digest_file(str(generated))
            for key, suffix in CASES["directories"].items():
                self.assertEqual(oracle.get_dir(key, scene_name="Probe"), folder / "m" / suffix.replace("{scene_name}", "Probe"))

    def test_font_import_redirects_unconfigured_manim_from_another_directory(self):
        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
            folder = Path(directory)
            program = (
                "import importlib; from pathlib import Path; "
                "importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️'); "
                "from manim import Text, config; Text('Output isolation'); "
                "assert Path(config.media_dir).is_relative_to(Path('" + folder.as_posix() + "')); "
                "assert not Path('media').exists()"
            )
            environment = {**os.environ, "PYTHONPATH": str(WORKSPACE), "SEMIO_TEACHING_OUTPUT_ROOT": str(folder), "PYTHONDONTWRITEBYTECODE": "1"}
            result = subprocess.run([__import__("sys").executable, "-B", "-c", program], cwd=folder, env=environment, capture_output=True, text=True, encoding="utf-8", timeout=60)
            self.assertEqual(result.returncode, 0, result.stderr)

    def test_git_ignores_generated_paths_and_keeps_authored_svg(self):
        for ignored, paths in ((True, CASES["ignored"]), (False, CASES["authored"])):
            for path in paths:
                with self.subTest(path=path):
                    result = subprocess.run(["repo", "tools", "exec", "--light", "git", "--", "check-ignore", "--no-index", "-q", path], cwd=WORKSPACE, capture_output=True)
                    self.assertEqual(result.returncode, 0 if ignored else 1, result.stderr)
