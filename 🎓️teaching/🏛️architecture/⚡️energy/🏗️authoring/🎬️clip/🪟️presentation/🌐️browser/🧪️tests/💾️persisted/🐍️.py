"""💾 Verify immutable actual-UI handoff assets using independent image decoding."""
from __future__ import annotations

import concurrent.futures
import importlib
import json
import os
from pathlib import Path
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

import numpy as np
from PIL import Image

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
BROWSER = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🐍️")
OUTPUT = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))

class BoundaryAssetContract(unittest.TestCase):
    """🧪 Enforce lossless, versioned first-writer boundary identity across fresh owners."""
    def setUp(self):
        """📁 Own each native raster fixture beneath the active ticket output."""
        OUTPUT.mkdir(parents=True, exist_ok=True)
        self.work = tempfile.TemporaryDirectory(dir=OUTPUT, prefix="a-")
        self.root = Path(self.work.name)
        self.addCleanup(self.work.cleanup)

    def image(self, name, value):
        """🖼️ Independently encode a complete asymmetric RGB raster."""
        file = self.root / (name + ".png")
        data = np.arange(16 * 9 * 3, dtype=np.uint16).reshape(9, 16, 3)
        data = ((data + value) % 256).astype(np.uint8)
        Image.fromarray(data).save(file)
        return file

    def test_cold_owner_uses_exact_first_paint(self):
        """🔁 A new owner retrieves the first actual paint despite native jitter."""
        first, second = self.image("first", 1), self.image("second", 3)
        expected = np.array(Image.open(first))
        BROWSER._boundary_asset(self.root / "cache", "same-inputs", first, {"owner": 1, "output": str(first)}, (16, 9))
        metadata = BROWSER._boundary_asset(self.root / "cache", "same-inputs", second, {"owner": 2}, (16, 9))
        self.assertTrue(np.array_equal(np.array(Image.open(second)), expected))
        self.assertEqual(metadata["owner"], 1)
        self.assertEqual(metadata["output"], str(second))
        third = self.image("third", 5)
        BROWSER._boundary_asset(self.root / "cache", "new-inputs", third, {"owner": 3}, (16, 9))
        self.assertFalse(np.array_equal(np.array(Image.open(third)), expected))

    def test_atomic_first_writer_race(self):
        """🏁 Concurrent producers publish one complete immutable boundary record."""
        gate = threading.Barrier(4)
        paths = [self.image(str(index), index * 3) for index in range(4)]
        def publish(index):
            gate.wait(timeout=10)
            return BROWSER._boundary_asset(self.root / "cache", "race", paths[index], {"owner": index}, (16, 9))
        with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
            results = list(pool.map(publish, range(4)))
        self.assertEqual(len({item["owner"] for item in results}), 1)
        self.assertTrue(all(np.array_equal(np.array(Image.open(paths[0])), np.array(Image.open(file))) for file in paths))

    def test_corrupted_asset_is_rejected(self):
        """🔒 Corruption cannot silently establish a different pixel handoff."""
        first = self.image("first", 1)
        BROWSER._boundary_asset(self.root / "cache", "integrity", first, {}, (16, 9))
        blob = next((self.root / "cache").glob("*.png"))
        blob.write_bytes(b"corrupt")
        with self.assertRaisesRegex(RuntimeError, "boundary"):
            BROWSER._boundary_asset(self.root / "cache", "integrity", first, {}, (16, 9))

    def test_cancelled_capture_performs_no_work(self):
        """🛑 A cancelled boundary operation starts no native browser or asset write."""
        from types import SimpleNamespace
        event = threading.Event()
        event.set()
        with patch.object(BROWSER, "_call", side_effect=AssertionError("Native work started")):
            with self.assertRaises(InterruptedError):
                BROWSER.render_batch({"frames": []}, session=SimpleNamespace(cancelled=event))

if __name__ == "__main__":
    unittest.main()
