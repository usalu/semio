"""🔻️ Preserve positive near-coplanar native material with conditioned independent volume."""
import importlib,itertools,json,math
from fractions import Fraction as Q
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def determinant(a,b,c):
    """📐 Evaluate the exact determinant of three source vectors."""
    return a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0])

def independent_volume(vertices):
    """🔬 Affinely condition published binary64 points before independent SciPy integration."""
    from scipy.spatial import ConvexHull
    points=[[Q(c) for c in p] for p in vertices];origin=points[0];vectors=[[p[k]-origin[k] for k in range(3)] for p in points[1:]]
    a,b,c=max(itertools.combinations(vectors,3),key=lambda v:abs(determinant(*v)));scale=determinant(a,b,c)
    if not scale:return 0.
    mapped=[[float(determinant(v,b,c)/scale),float(determinant(a,v,c)/scale),float(determinant(a,b,v)/scale)] for v in [[Q(0)]*3,*vectors]]
    return ConvexHull(mapped).volume*float(abs(scale))

def test_positive_near_coplanar_volume_matches_geos_and_affinely_conditioned_scipy():
    """🧬 Reproduce each language-neutral prism with GEOS and independently transformed SciPy."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    for row in CASE['cases']:
        p=api._polyhedron_part(row['mesh']['vertices'],row['mesh']['faces']);vertices=row['mesh']['vertices'];z0=min(v[2] for v in vertices);z1=max(v[2] for v in vertices)
        lower=[[v[0],v[1]] for v in vertices if v[2]==z0];polygon=Polygon(lower).convex_hull;geos=polygon.area*(z1-z0);scipy=independent_volume(vertices);actual=api._polyhedron_volume(p)
        assert geos>0 and scipy>0 and actual>0,row['id']
        assert math.isclose(geos,scipy,rel_tol=CASE['comparison']['relative'],abs_tol=CASE['comparison']['absolute']),(row['id'],geos,scipy)
        assert math.isclose(actual,scipy,rel_tol=CASE['comparison']['relative'],abs_tol=CASE['comparison']['absolute']),(row['id'],actual,scipy)
