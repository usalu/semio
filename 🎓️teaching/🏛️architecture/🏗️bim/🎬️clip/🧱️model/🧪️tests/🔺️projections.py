"""🔺 Preserve positive projected turns and validate convex silhouettes independently."""
import importlib,json
from fractions import Fraction
from pathlib import Path


def test_convex_projection_preserves_tiny_positive_turns_and_matches_geos():
    import numpy as np
    from jsonschema import Draft202012Validator
    from shapely.geometry import MultiPoint,Polygon
    api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
    p=Path(__file__);cases=json.loads(p.with_suffix('.json').read_text())
    Draft202012Validator(json.loads(p.with_suffix('.schema.json').read_text())).validate(cases)
    for case in cases['cases']:
        actual=api._convex_hull(case['points'])
        assert set(map(tuple,actual))==set(map(tuple,case['required'])),(case['id'],actual)
        expected=MultiPoint(case['points']).convex_hull
        assert expected.geom_type==case.get('geos_kind','Polygon')
        if expected.geom_type=='Polygon':assert Polygon(actual).equals(expected),(case['id'],actual,expected.wkt)
        def exact_area(points):
            return abs(sum(Fraction(a[0])*Fraction(b[1])-Fraction(a[1])*Fraction(b[0]) for a,b in zip(points,points[1:]+points[:1])))/2
        q=np.array([[Fraction(v) for v in p] for p in case['required']],dtype=object)
        independent=abs(np.sum(q[:,0]*np.roll(q[:,1],-1)-q[:,1]*np.roll(q[:,0],-1)))/2
        assert exact_area(actual)==independent>0
        if expected.geom_type=='Polygon':assert exact_area(actual)==exact_area(list(expected.exterior.coords)[:-1])
        if 'area' in case:assert abs(float(exact_area(actual))-case['area'])<=max(1e-30,case['area']*1e-14)
