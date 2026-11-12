"""🎥 Verify paused camera frames use committed real silhouette geometry."""
import importlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

from PIL import Image

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
RENDER = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🖌️render.🐍️")


class CameraFrame(unittest.TestCase):
    def test_paused_camera_frames_have_committed_borders(self):
        """🪟 Real focus and restore screenshots are captured only after the compositor settles."""
        output = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", ROOT / ".🧬semio/🎓️teaching/⚡️cache"))
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix="camera-", dir=output) as directory:
            work = Path(directory)
            snapshot = work / "snapshot.png"
            Image.new("RGB", tuple(CASES["viewport"]), "#222222").save(snapshot)
            requests = [{"video_id": CASES["scene"], **frame, "output": work / f"{index}.png", "snapshot": snapshot}
                        for index, frame in enumerate(CASES["frames"])]
            settings = {"language": "en", "profile": "preview", "resolution": None, "fps": 15, "audience": "speaker"}
            results = RENDER.frame_batch(requests, settings)
            self.assertEqual(len(results), len(requests))
            for result in results:
                self.assertTrue(result["browser"]["border_compositor"]["ready_before_capture"])
                self.assertEqual(result["browser"]["motion_policy"]["css_transitions"], 0)
                self.assertGreater(result["browser"]["motion_policy"]["keyframe_animations"], 0)
                self.assertTrue(result["browser"]["windows"])
                with Image.open(result["path"]) as image:
                    self.assertEqual(image.size, tuple(CASES["viewport"]))
                    if result["timeline"]["content_camera"] < 1:
                        self.assertTrue(any(low < high for low, high in image.convert("RGB").getextrema()))


if __name__ == "__main__":
    unittest.main()
