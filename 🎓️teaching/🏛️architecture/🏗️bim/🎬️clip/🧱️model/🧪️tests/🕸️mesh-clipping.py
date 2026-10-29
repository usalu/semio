"""🕸️ Keep full clipping precision until the analytical node publication boundary."""
import importlib,json,math
from pathlib import Path
import numpy as np
from scipy.spatial import KDTree
from shapely.geometry import Polygon
analysis=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️')

def cases():
    """📂 Read language-neutral nearly parallel cell intersections."""
    return json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def test_unrounded_mesh_clip_matches_independent_geos_domain():
    """🔬 Preserve intersections without amplifying intermediate coordinate quantization."""
    for case in cases()['cases']:
        expected=Polygon(case['subject']).intersection(Polygon(case['clip']))
        assert math.isclose(expected.area,case['expected_geos_area'],abs_tol=1e-12)
        actual=Polygon(analysis._clip(case['subject'],case['clip'],digits=None))
        assert actual.is_valid
        assert actual.symmetric_difference(expected).area<1e-12

def test_mesh_node_publication_obeys_only_declared_half_cell_error():
    """📍 Compare published nodes in both directions using an independent nearest-neighbour tree."""
    data=cases()
    for case in data['cases']:
        expected=Polygon(case['subject']).intersection(Polygon(case['clip']))
        reference=np.asarray([[x,y,case['mid_surface']] for x,y in list(expected.exterior.coords)[:-1]])
        clipped=analysis._clip(case['subject'],case['clip'],digits=None)
        actual=np.round(np.asarray([[x,y,case['mid_surface']] for x,y in clipped]),data['publication_decimals'])
        arithmetic=32*np.spacing(np.max(np.abs(reference)))
        bound=data['publication_half_cell_radius_3d']+arithmetic
        assert max(KDTree(reference).query(actual)[0])<=bound
        assert max(KDTree(actual).query(reference)[0])<=bound
