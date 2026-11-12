"""🧪 Playback: profile-explicit media selection and exact frame access at the terminal sample.

Language-agnostic cases live in `🔣️.cases.json`. PyAV is the independent decoder oracle; the shared timeline evaluator names the identity at every frame, and a lossless movie makes every strategy comparable bit for bit.
"""

from __future__ import annotations

import importlib
import json
import tempfile
import unittest
from pathlib import Path

import numpy as np

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
playback = importlib.import_module(PACKAGE + ".🎞️playback.🐍️")
export = importlib.import_module(PACKAGE + ".📤️export.🐍️")
profile = importlib.import_module(PACKAGE + ".🎚️profile.🐍️")
runtime = importlib.import_module(PACKAGE + ".⚙️runtime.🐍️")
timeline = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.⏱️timeline.🐍️")

CASES = json.loads((Path(__file__).parent / "🔣️.cases.json").read_text(encoding="utf-8"))


def movie(folder: Path, case: dict) -> Path:
    """🎞️ Lossless movie: green channel is the identity (source or target), red channel is the frame index."""
    from PIL import Image

    window, schedule = timeline.Window(case["f0"] / case["fps"], case["count"] / case["fps"]), timeline.Schedule(case["fps"])
    frames = folder / "frames"
    frames.mkdir()
    for frame in range(case["total"]):
        target = timeline.owner(window, schedule, frame) == "target"
        Image.new("RGB", (64, 36), (frame * 4 % 256, 220 if target else 0, 0 if target else 220)).save(frames / f"{frame:04d}.png")
    path = folder / "clip.mp4"
    runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-framerate", str(case["fps"]), "-i", str(frames / "%04d.png"),
                         "-c:v", "libx264", "-crf", "0", "-pix_fmt", "yuv444p", "-g", str(CASES["keyframeInterval"]), "-r", str(case["fps"]), str(path)])
    return path


def identity(pixels: np.ndarray) -> str:
    return "target" if pixels[..., 1].mean() > pixels[..., 2].mean() else "source"


class SeekDeterminism(unittest.TestCase):
    def test_boundary_frames_are_identical_for_forward_reverse_random_and_fresh_seek(self):
        for case in CASES["transitions"]:
            with self.subTest(case["name"]), tempfile.TemporaryDirectory() as folder:
                path = movie(Path(folder), case)
                f0, count = case["f0"], case["count"]
                terminal = f0 + count - 1
                self.assertEqual(terminal, timeline.terminal(timeline.Window(f0 / case["fps"], count / case["fps"]), timeline.Schedule(case["fps"])))
                wanted = sorted({max(0, terminal - 1), terminal, min(case["total"] - 1, terminal + 1), f0, 0, case["total"] - 1})
                forward = playback.sample(path, wanted, "forward")
                reverse = playback.sample(path, wanted, "reverse")
                shuffled = playback.sample(path, wanted, "random", seed=CASES["seed"])
                for index in wanted:
                    fresh = playback.frame_at(path, index)
                    for other in (forward, reverse, shuffled):
                        np.testing.assert_array_equal(other[index], fresh)
                    self.assertAlmostEqual(float(forward[index][..., 0].mean()), index * 4 % 256, delta=2)
                    expected = playback.owner_at(index, start=f0, count=count, fps=case["fps"])
                    self.assertEqual(identity(forward[index]), "target" if expected == "target" else "source", f"frame {index}")
                if count > 1:
                    self.assertEqual(identity(forward[terminal - 1]), "source")
                self.assertEqual(identity(forward[terminal]), "target")
                self.assertEqual(identity(forward[min(case["total"] - 1, terminal + 1)]), "target")

    def test_terminal_frame_survives_skipped_delivery_and_pause_resume(self):
        case = CASES["transitions"][0]
        with tempfile.TemporaryDirectory() as folder:
            path = movie(Path(folder), case)
            terminal = case["f0"] + case["count"] - 1
            before = playback.frame_at(path, terminal - 1)
            for skipped_to in (terminal, terminal + 3):
                resumed = playback.frame_at(path, skipped_to)
                self.assertEqual(identity(resumed), "target")
            np.testing.assert_array_equal(before, playback.frame_at(path, terminal - 1))


class ProfileSelection(unittest.TestCase):
    def publish(self, folder: Path, name: str, **kwargs) -> Path:
        artifact = folder / f"{name}.mp4"
        artifact.write_bytes(b"movie-" + name.encode() * 400)
        record = export.export_record("chapter", artifact, profile=name, language="en", audience="speaker", work={"jobs": [], "counts": {}, "sources": []}, **kwargs)
        export.publish_record(artifact, record)
        return artifact

    def test_media_opens_only_for_its_own_profile_and_labels_it(self):
        with tempfile.TemporaryDirectory() as folder:
            for name, labels in CASES["profiles"].items():
                artifact = self.publish(Path(folder), name)
                media = playback.open_media(artifact, name)
                self.assertEqual((media.profile, media.fingerprint), (name, profile.fingerprint(name)))
                for language, text in labels.items():
                    self.assertEqual(media.label(language), text)
                other = "final" if name == "preview" else "preview"
                with self.assertRaises(export.ProfileMismatch):
                    playback.open_media(artifact, other)

    def test_missing_provenance_modified_file_and_unknown_profile_fail(self):
        with tempfile.TemporaryDirectory() as folder:
            artifact = self.publish(Path(folder), "preview")
            artifact.write_bytes(artifact.read_bytes() + b"x")
            with self.assertRaises(export.ProfileMismatch):
                playback.open_media(artifact, "preview")
            bare = Path(folder) / "bare.mp4"
            bare.write_bytes(b"x" * 2000)
            with self.assertRaises(export.ProfileMismatch):
                playback.open_media(bare, "preview")
            with self.assertRaises(profile.ProfileError):
                playback.locate(Path(folder), "draft", "en", "speaker")

    def test_locate_is_profile_first_and_never_falls_back(self):
        root = Path("out")
        self.assertEqual(playback.locate(root, "preview", "de", "speaker", "x.mp4"), root / "preview" / "de" / "speaker" / "x.mp4")
        self.assertNotEqual(playback.locate(root, "final", "de", "speaker"), playback.locate(root, "preview", "de", "speaker"))

    def test_badge_language_has_no_default(self):
        with tempfile.TemporaryDirectory() as folder:
            media = playback.open_media(self.publish(Path(folder), "final"), "final")
            with self.assertRaises(ValueError):
                media.label("fr")


if __name__ == "__main__":
    unittest.main()
