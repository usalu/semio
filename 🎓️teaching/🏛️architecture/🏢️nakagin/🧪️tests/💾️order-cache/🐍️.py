"""💾 Prove the persisted occlusion orders: exact reuse, stale entries impossible, atomic writes."""
import importlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import numpy as np

ROOT = next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir())
sys.path.insert(0, str(ROOT))
V = importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
C = json.loads((ROOT / '🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/💾️order-cache/🔣️.json').read_text(encoding='utf-8'))


def scene(case):
    view = V.make_view(tuple(case['view']), (0, 0, 0), 1, (0, 0, 0))
    items = []
    for key, row in case['faces'].items():
        vertices = np.asarray(row['vertices'], float)
        draw = lambda v, polygons=(vertices,): V._shape([V._screen(v, p) for p in polygons], '#808080', 1, width=0)
        normal = np.asarray(row['normal'], float)
        normal = normal if normal @ np.asarray(view.towards) > 0 else -normal
        items.append(V.Item(0, vertices.mean(axis=0), vertices, draw, normal, (vertices,), 1, 'shell', part_key=key))
    return view, items


def labels(items):
    return [(item.part_key, tuple(np.round(item.verts, 9).ravel())) for item in items]


class OrderCache(unittest.TestCase):
    """🗄️ Stored orderings are keyed by geometry, view and source digest."""

    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        patch = mock.patch.dict(os.environ, {'SEMIO_TEACHING_CACHE_ROOT': self.directory.name})
        patch.start()
        self.addCleanup(patch.stop)
        V._OCCLUSION_ORDERS.clear()

    def files(self, suffix='.json'):
        return sorted(V._order_directory().glob(f'*{suffix}')) if V._order_directory().is_dir() else []

    def test_cold_build_persists_and_warm_build_reuses_without_ordering(self):
        view, items = scene(C)
        cold = V._section_order(items, view)
        self.assertEqual(len(self.files()), C['expected']['files'])
        self.assertEqual(len(self.files('.tmp')), C['expected']['temporary_files'])
        data = json.loads(self.files()[0].read_text(encoding='utf-8'))
        self.assertEqual(any(not isinstance(row, int) for row in data['order']), C['expected']['fragmented'])
        V._OCCLUSION_ORDERS.clear()
        with mock.patch.object(V._fragments, 'ordered_fragments', side_effect=AssertionError('ordering recomputed')):
            warm = V._section_order(items, view)
        self.assertEqual(labels(cold), labels(warm))

    def test_changed_geometry_or_view_never_reuses_an_entry(self):
        view, items = scene(C)
        V._section_order(items, view)
        moved = json.loads(json.dumps(C))
        moved['faces']['x']['vertices'][0][1] = -1.5
        other_view, other_items = scene(moved)
        V._section_order(other_items, other_view)
        turned = dict(C, view=[125, 24])
        turned_view, turned_items = scene(turned)
        V._section_order(turned_items, turned_view)
        self.assertEqual(len(self.files()), 3)

    def test_source_change_corrupt_and_foreign_entries_rebuild(self):
        view, items = scene(C)
        expected = labels(V._section_order(items, view))
        path = self.files()[0]
        for damage in ('{', json.dumps({'key': 'other', 'sources': V._source_digest(), 'order': []}), json.dumps({'key': path.stem, 'sources': 'stale', 'order': []})):
            path.write_text(damage, encoding='utf-8')
            V._OCCLUSION_ORDERS.clear()
            self.assertEqual(labels(V._section_order(items, view)), expected)
            self.assertEqual(json.loads(path.read_text(encoding='utf-8'))['sources'], V._source_digest())

    def test_failed_write_leaves_no_partial_entry(self):
        view, items = scene(C)
        with mock.patch.object(V.os, 'replace', side_effect=OSError('disk')):
            V._section_order(items, view)
        self.assertEqual(self.files(), [])
        self.assertEqual(self.files('.tmp'), [])


if __name__ == '__main__':
    unittest.main()
