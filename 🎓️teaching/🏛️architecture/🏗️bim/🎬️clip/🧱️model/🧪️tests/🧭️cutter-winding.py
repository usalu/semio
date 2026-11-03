"""🧭 A thin exact cutter with one inward-wound face must remove only its own volume from stock."""
import importlib,json
from fractions import Fraction
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def _solid(row,role):
    """🧊 Rebuild one exact rational convex solid from its serialized mesh."""
    return {'mesh':{'vertices':[[Fraction(c) for c in p] for p in row['vertices']],'faces':[list(f) for f in row['faces']]},'material':'MAT-RC','role':role}

def test_inward_wound_sliver_removes_only_its_own_volume():
    """🧪 Compare with the independent SciPy convex hull volume of the untouched stock."""
    import numpy as np
    from scipy.spatial import ConvexHull
    stock,cutter=_solid(CASE['stock'],'stock'),_solid(CASE['cutter'],'cutter')
    expected=ConvexHull(np.array([[float(c) for c in p] for p in stock['mesh']['vertices']])).volume
    actual=float(api._native_parts_exact_volume(api._reference_material_parts(stock,[cutter])))
    assert abs(actual-expected)<=CASE['comparison']['volume'],(actual,expected)
