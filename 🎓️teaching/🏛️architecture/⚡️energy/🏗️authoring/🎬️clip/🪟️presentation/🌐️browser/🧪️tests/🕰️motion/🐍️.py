"""🕰️ Run the portable shared-border motion fixture through the Bun implementation."""
import subprocess
import unittest
from pathlib import Path
class Motion(unittest.TestCase):
    def test_transition_lifecycle(self):
        """🔄 Active and paused transitions stay tracked; finished or removed ones clear."""
        root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
        result = subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", str(Path(__file__).with_name("🟦️.ts"))], cwd=root, capture_output=True, text=True, check=True)
        self.assertIn("PASS removed-cleared", result.stdout)
if __name__ == "__main__":
    unittest.main()
