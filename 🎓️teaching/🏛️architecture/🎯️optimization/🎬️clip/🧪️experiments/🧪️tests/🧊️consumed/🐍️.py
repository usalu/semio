"""🧪 Verify actual authored experiment cleanup with the existing native animation engine."""
from __future__ import annotations
import importlib
import json
import os
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True

class ConsumedGeometryTest(unittest.TestCase):
    """🎞️ Native skip-animation rendering still runs begin, finish, updaters and fade cleanup."""
    def test_authored_cleanup(self):
        fixture = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "experiment-cleanup"
        engine = importlib.import_module("manim")
        module = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧪️experiments.🐍️")
        for case in fixture["cases"]:
            with self.subTest(**case), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": case["language"], "SEMIO_TEACHING_AUDIENCE": case["audience"]}):
                with engine.tempconfig(dict(media_dir=str(output), dry_run=True, skip_animations=True, disable_caching=True,
                    pixel_width=fixture["profile"]["width"], pixel_height=fixture["profile"]["height"],
                    frame_rate=fixture["profile"]["fps"], progress_bar="none", verbosity="ERROR")):
                    getattr(module, fixture["scene"])().render()

if __name__ == "__main__":
    unittest.main()
