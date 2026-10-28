"""🖼️ Verify finite encoder clocks across actual RGB/RGBA decoder-property changes."""
from pathlib import Path
import hashlib
import importlib
import json
import os
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
runtime = importlib.import_module(PACKAGE + ".⚙️runtime.🐍️")
presentation = importlib.import_module(PACKAGE + ".🪟️presentation.🐍️")
collection = importlib.import_module(PACKAGE + ".🪟️presentation.📚️collection.🐍️")

class ImageStreamClock(unittest.TestCase):
    """🎞️ Keep exact finite frames and original RGB values without cache mutation."""
    def setUp(self):
        """📁 Give each native case a private generated scope and owned cancellation port."""
        base = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        base.mkdir(parents=True, exist_ok=True)
        self.directory = tempfile.TemporaryDirectory(prefix="stream-", dir=base)
        self.work = Path(self.directory.name)
        self.session = runtime.ToolSession()
        self.addCleanup(self.directory.cleanup)
        self.addCleanup(self.session.cancel)
        self.settings = dict(CASES["settings"])

    def run_video(self, arguments, name):
        """🔌 Route all native image/video work through the existing FFmpeg port."""
        runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-y", *arguments],
            session=self.session, log=self.work / name)

    def body(self):
        """🌈 Encode distinguishable native frames with the real Manim SAR contract."""
        result = self.work / "body.mp4"
        self.run_video(["-f", "lavfi", "-i", "testsrc2=size=480x270:rate=4", "-vf", "setsar=0",
            "-frames:v", str(CASES["body_frames"]), "-an", "-c:v", "libx264", "-preset", "fast",
            "-crf", "0", "-pix_fmt", "yuv420p", str(result)], "body-create.log")
        return result

    def hashes(self, clip, name, start=0, end=None):
        """🔎 Compare independent native RGB frame hashes, including exact body ordering."""
        output = self.work / name
        trim = f"trim=start_frame={start}" + (f":end_frame={end}" if end is not None else "")
        self.run_video(["-i", str(clip), "-map", "0:v:0", "-vf", trim + ",format=rgb24",
            "-an", "-f", "framemd5", str(output)], name + ".log")
        return [line.rsplit(",", 1)[-1].strip() for line in output.read_text().splitlines() if not line.startswith("#")]

    def test_mixed_png_frames_preserve_exact_long_body_clock(self):
        """⏲️ Reproduce real intermediate-alpha screenshots and forbid any dropped frames."""
        from PIL import Image
        body = self.body()
        def transition(chapter, snapshot, output, settings, session, *, phase):
            """📷 Keep RGB snapshots and varied native screenshots with two intermediate alpha frames."""
            count = CASES["focus_frames" if phase == "focus" else "restore_frames"]
            directory = output / ("i" if phase == "focus" else "o")
            directory.mkdir(parents=True, exist_ok=True)
            for index in range(count):
                target = directory / f"{index:04d}.png"
                if index == (count - 1 if phase == "focus" else 0):
                    shutil.copyfile(snapshot, target)
                else:
                    rgba = phase == "restore" and index in CASES["mixed_restore_frames"]
                    Image.new("RGBA" if rgba else "RGB", (480, 270),
                        (index * 11, 70, 130, CASES["alpha"]) if rgba else (index * 11, 70, 130)).save(target)
            return directory, count
        reference = self.hashes(body, "body.hashes")
        for language in CASES["languages"]:
            with self.subTest(language=language):
                final = self.work / ("window-" + language + ".mp4")
                settings = dict(self.settings, language=language)
                locale_work = self.work / language
                locale_work.mkdir()
                with patch.object(presentation, "_transition", transition):
                    presentation._render_chapter("EnergyBalance_07_TheorieVsPraxis", body, final,
                        settings, self.session, locale_work)
                actual = self.hashes(final, "window-" + language + ".hashes")
                self.assertEqual(len(actual), CASES["expected_frames"])
                start = CASES["focus_frames"]
                self.assertEqual(actual[start:start + CASES["body_frames"]], reference)

    def test_normalized_pngs_preserve_rgb_and_source_bytes(self):
        """🎨 Compare new opaque RGB storage with independent NumPy/Pillow pixel values."""
        from PIL import Image
        import numpy as np
        source = self.work / "source"
        source.mkdir()
        for index, mode in enumerate(("RGB", "RGBA")):
            Image.new(mode, (480, 270), (70, 110, 140) if mode == "RGB" else (30, 170, 210, CASES["alpha"])).save(source / f"{index:04d}.png")
        before = [hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(source.glob("*.png"))]
        normalized = presentation._normalize_transition_frames(source, self.work / "normalized", 2, self.settings, self.session)
        for original in sorted(source.glob("*.png")):
            with Image.open(original) as image, Image.open(normalized / original.name) as actual:
                self.assertEqual(actual.mode, CASES["normalization"]["mode"])
                self.assertEqual(actual.info, CASES["normalization"]["metadata"])
                np.testing.assert_array_equal(np.asarray(image.convert("RGB")), np.asarray(actual))
        self.assertEqual(before, [hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(source.glob("*.png"))])

    def test_native_count_guard_rejects_valid_but_short_movie(self):
        """🛑 A healthy MP4 with the wrong decoded clock cannot be published."""
        body = self.body()
        collection.require_frames(body, CASES["body_frames"], self.settings, self.work, self.session)
        with self.assertRaisesRegex(RuntimeError, "frame"):
            collection.require_frames(body, CASES["expected_frames"], self.settings, self.work, self.session)

if __name__ == "__main__":
    unittest.main()
