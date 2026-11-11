"""🧹 Compare generated-stage pruning with the Tailwind source scanner."""
import importlib
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
BROWSER = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


class StagingMembership(unittest.TestCase):
    def test_membership_matches_tailwind_and_preserves_authored_hidden_sources(self):
        """📂 Generated staging does not enter CSS inputs, while authored hidden paths remain visible."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / ".git").mkdir()
            (root / ".gitignore").write_text(".p[a-f0-9][a-f0-9][a-f0-9][a-f0-9][a-f0-9][a-f0-9][a-f0-9][a-f0-9]/\n", encoding="utf-8")
            for name in CASES["files"]:
                file = root / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_text("export const colour = 'text-red-500';", encoding="utf-8")
            actual = BROWSER._membership([{"base": str(root), "pattern": CASES["pattern"], "negated": False}])
            paths = sorted(Path(name).relative_to(root).as_posix() for name in actual)
            oracle = subprocess.run(["repo", "tools", "exec", "--light", "node", "--", str(Path(__file__).with_name("🟨️.cjs")), str(root), CASES["pattern"]], cwd=ROOT, text=True, capture_output=True, check=True)
            self.assertEqual(paths, sorted(CASES["expected"]))
            self.assertEqual(paths, json.loads(oracle.stdout))


if __name__ == "__main__":
    unittest.main()
