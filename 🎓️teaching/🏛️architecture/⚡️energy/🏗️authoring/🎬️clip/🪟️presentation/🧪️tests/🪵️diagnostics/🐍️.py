"""🪵 Preserve nested browser failure diagnostics without retaining generated frames."""
import importlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
PRESENTATION = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


class Diagnostics(unittest.TestCase):
    def test_nested_browser_log_survives_owned_attempt_cleanup(self):
        """🧾 The useful stderr survives while image output is discarded."""
        with tempfile.TemporaryDirectory() as directory:
            parent = Path(directory)
            work = parent / "p-12345678"
            nested = work / CASES["nested"]
            nested.parent.mkdir(parents=True)
            nested.write_text(CASES["message"], encoding="utf-8")
            (nested.parent / "frame.png").write_bytes(b"frame")
            PRESENTATION._clean_attempt(work, parent, keep_logs=True)
            self.assertEqual((work / CASES["expected"]).read_text(encoding="utf-8"), CASES["message"])
            self.assertFalse((work / "i").exists())


    def test_failure_log_is_bounded_and_success_removes_work(self):
        """📏 Retained stderr has a fixed upper bound; success leaves no attempt directory."""
        with tempfile.TemporaryDirectory() as directory:
            parent = Path(directory)
            work = parent / "p-12345678"
            log = work / CASES["nested"]
            log.parent.mkdir(parents=True)
            log.write_bytes(b"a" * (CASES["limit"] + 10) + b"tail")
            PRESENTATION._clean_attempt(work, parent, keep_logs=True)
            saved = work / CASES["expected"]
            self.assertEqual(saved.stat().st_size, CASES["limit"])
            self.assertTrue(saved.read_bytes().endswith(b"tail"))
            PRESENTATION._clean_attempt(work, parent, keep_logs=False)
            self.assertFalse(work.exists())

if __name__ == "__main__":
    unittest.main()
