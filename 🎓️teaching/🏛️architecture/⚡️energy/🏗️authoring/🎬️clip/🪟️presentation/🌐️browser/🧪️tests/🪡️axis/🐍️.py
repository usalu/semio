"""🪡 Independently verify the expected rectilinear segment using Shapely."""
import json
import subprocess
import unittest
from pathlib import Path
from shapely.geometry import LineString
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
class Axis(unittest.TestCase):
    def test_expected_projection(self):
        """📐 The small horizontal drift is projected onto the starting vertical line."""
        start, end = CASES["expected"]["start"], CASES["expected"]["end"]
        line = LineString([start, end])
        self.assertEqual(line.bounds[0], line.bounds[2])
        self.assertAlmostEqual(line.length, abs(end[1] - start[1]))
        root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
        result = subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", str(Path(__file__).with_name("🟦️.ts"))], cwd=root, capture_output=True, text=True, check=True)
        self.assertIn("PASS axis-aligned snapping", result.stdout)
if __name__ == "__main__":
    unittest.main()
