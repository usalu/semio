"""⚡ Validate that an operation-local memo changes only the cost of exact reference material, never its result."""
import importlib
import json
from pathlib import Path
import unittest
from unittest.mock import patch

import numpy as np
from scipy.spatial import ConvexHull

CASES = json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf-8'))


class FinishMemoContract(unittest.TestCase):
    """🧊 Keep the memoized and the plain construction identical and prove the shared operands are derived once."""

    def setUp(self):
        self.api = importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')

    def hull(self, corners):
        """📐 Measure a convex solid with SciPy, independently of the native exact kernel."""
        return ConvexHull(np.array(corners, dtype=float)).volume

    def test_memo_keeps_the_exact_remainder_and_matches_the_independent_hull_volume(self):
        """🔬 Subtract two overlapping cutters from the stock and compare with hull volumes of the stock and of each overlap."""
        expected = self.hull(CASES['stockCorners']) - sum(self.hull(corners) for corners in CASES['overlapCorners'])
        plain = self.api._reference_material_parts(CASES['stock'], CASES['cutters'])
        memo = {}
        remembered = self.api._reference_material_parts(CASES['stock'], CASES['cutters'], memo=memo)
        for parts in (plain, remembered):
            self.assertAlmostEqual(float(self.api._native_parts_exact_volume(parts)), expected, delta=CASES['tolerance'])
        self.assertEqual(json.dumps(plain, sort_keys=True, default=str), json.dumps(remembered, sort_keys=True, default=str))
        self.assertTrue(memo)

    def test_a_shared_memo_derives_the_cells_of_every_operand_once(self):
        """♻️ Repeat the same operation with one memo: the native cells of the stock and of both cutters are built exactly once."""
        original = self.api._native_material_cells
        calls = []

        def counting(part):
            calls.append(id(part))
            return original(part)

        memo = {}
        with patch.object(self.api, '_native_material_cells', counting):
            for _ in range(3):
                self.api._reference_material_parts(CASES['stock'], CASES['cutters'], memo=memo)
        self.assertEqual(len(calls), 1 + len(CASES['cutters']))
        with patch.object(self.api, '_native_material_cells', counting):
            calls.clear()
            for _ in range(3):
                self.api._reference_material_parts(CASES['stock'], CASES['cutters'])
        self.assertEqual(len(calls), 3 * (1 + len(CASES['cutters'])))


if __name__ == '__main__':
    unittest.main()
