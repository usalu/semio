"""🤝 Keep an exactly tangent convex stock whole without discarding positive material."""
from copy import deepcopy
from fractions import Fraction
import importlib,json
from pathlib import Path


def inputs():
    """🧾 Decode portable exact coordinates without a Float64 intermediate."""
    from jsonschema import Draft202012Validator
    p=Path(__file__);case=json.loads(p.with_suffix('.json').read_text())
    Draft202012Validator(json.loads(p.with_suffix('.schema.json').read_text())).validate(case)
    def decode(value):
        if isinstance(value,dict):return Fraction(*map(int,value['$rational'])) if '$rational' in value else {k:decode(v) for k,v in value.items()}
        return [decode(v) for v in value] if isinstance(value,list) else value
    return decode(case)


def test_exact_coplanar_core_contact_preserves_one_unchanged_native_material_stock():
    from scipy.spatial import ConvexHull
    api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');case=inputs()['touching'];stock,cutter=case['stock'],case['cutter'];before=deepcopy(stock)
    vertices=stock['mesh']['vertices'];separated=False;contact=False
    for face in cutter['mesh']['faces']:
        a,b,c=[cutter['mesh']['vertices'][i] for i in face[:3]];u=[b[i]-a[i] for i in range(3)];v=[c[i]-a[i] for i in range(3)];n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
        values=[sum(n[i]*(p[i]-a[i]) for i in range(3)) for p in vertices]
        if all(x>=0 for x in values) and any(x>0 for x in values):separated=True;contact|=any(x==0 for x in values)
    assert separated and contact
    actual=api._subtract_convex_material(stock,[cutter])
    assert len(actual)==case['expected_parts'],len(actual)
    expected=[[float(x) for x in p] for p in vertices]
    assert actual[0]['mesh']['vertices']==expected
    api._polyhedron_part(actual[0]['mesh']['vertices'],actual[0]['mesh']['faces'])
    assert abs(api._polyhedron_volume(actual[0])-ConvexHull(expected).volume)<1e-15
    assert stock==before


def test_positive_two_nanometre_material_cut_is_not_misclassified_as_contact():
    from scipy.spatial import ConvexHull
    api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');case=inputs()['positive_cut']
    def box(bounds):
        x,y,z,X,Y,Z=bounds
        return {'material':'neutral','role':'control','outline':[[x,y],[X,y],[X,Y],[x,Y]],'z0':z,'z1':Z}
    raw=box(case['stock_bounds']);stock={**raw,'mesh':{'vertices':api.part_vertices(raw),'faces':api.part_faces(raw)}}
    result=api._subtract_convex_material(stock,[box(case['cutter_bounds'])]);volume=sum(ConvexHull(api.part_vertices(p)).volume for p in result)
    assert abs(volume-case['remaining_volume'])<1e-15
    assert 0<1-volume<3e-9
