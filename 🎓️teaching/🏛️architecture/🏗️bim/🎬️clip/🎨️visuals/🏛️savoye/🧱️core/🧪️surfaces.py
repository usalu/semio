"""🎨 Portable material-role contracts without a graphical runtime."""
import importlib
import json
from pathlib import Path
import unittest


class SurfaceRoleContract(unittest.TestCase):
    """🪟 Glazing and finish surfaces must preserve their physical role in every provider."""

    def test_portable_material_dispatch(self):
        directory = Path(__file__).parent
        physical = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.📐️geometry")
        for case in json.loads((directory / "🧪️surfaces.json").read_text(encoding="utf-8"))["cases"]:
            self.assertEqual(physical.surface_role({"kind": case["kind"]}), case["role"])


if __name__ == "__main__":
    unittest.main()
