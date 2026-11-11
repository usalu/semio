"""🖱️ Run shared-border hover deduplication through JSDOM."""
import subprocess
import unittest
from pathlib import Path
class Hover(unittest.TestCase):
    def test_effective_owner_changes(self):
        """🏷️ Internal descendant movement preserves ownership; leaving schedules a repaint."""
        root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
        result = subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", str(Path(__file__).with_name("🟦️.ts"))], cwd=root, capture_output=True, text=True, check=True)
        self.assertIn("PASS effective-hover-owners", result.stdout)
if __name__ == "__main__":
    unittest.main()
