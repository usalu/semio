"""📚 Bind every conservative CSS candidate without repeating stable tool discovery."""
import importlib
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
BROWSER = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))

class CandidateClosureContract(unittest.TestCase):
    """🧪 Detect content and membership changes across a stable dependency-cache lifecycle."""
    def test_conservative_candidates_are_hashed_and_reused(self):
        """🔍 A compiler-ignored candidate remains observed without forcing every warm discovery."""
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output, prefix="c-") as directory:
            work = Path(directory)
            for name in ("a.ts", "b.ts", "c.ts"):
                (work / name).write_text(name, encoding="utf-8")
            members = []
            result = {"source_files": [str(work / "a.ts")], "scan_sources": [{"base": str(work), "pattern": "**/*.ts", "negated": False}]}
            with patch.multiple(BROWSER, _SOURCES=(), _IDENTITY=None, _SCAN_SOURCES=(), _MEMBERSHIP=None), \
                    patch.object(BROWSER, "_membership", side_effect=lambda _: tuple(str(work / name) for name in members)), \
                    patch.object(BROWSER, "_call", return_value=result) as discover:
                for step in CASES["steps"]:
                    members[:] = step["members"]
                    if "edit" in step:
                        (work / step["edit"]).write_text("changed candidate", encoding="utf-8")
                    closure = set(BROWSER.source_files())
                    self.assertTrue({work / name for name in members}.issubset(closure))
                    self.assertEqual(discover.call_count, step["calls"])

if __name__ == "__main__":
    unittest.main()
