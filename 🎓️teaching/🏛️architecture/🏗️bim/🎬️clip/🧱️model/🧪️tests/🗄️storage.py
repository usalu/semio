"""🗄️ Fixed cabinet occupied material and native identity acceptance."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_fixed_cabinets_have_real_carcasses_shelves_fronts_and_empty_storage():
    """🧰 Replace solid storage masses with source-qualified occupied construction and usable cavities."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,Point
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASE)
    m=api.load_model()
    for row in CASE['cabinets']:
        obj=api.element(row['tag'],m);g=api.solid_geometry(row['tag'],m)
        assert set(CASE['roles'])<={p.get('role') for p in g['parts']},row['tag']
        assert obj['historical_evidence']['status']=='inferred_reconstruction'
        for x,y,z in row['cavities']:
            assert not any(Polygon(r).covers(Point(x,y)) for p in g['parts'] if p['z0']<CASE['floor']+z<p['z1'] for r in api.part_regions(p,CASE['floor']+z)),(row['tag'],x,y,z)

def test_fixed_cabinet_materials_are_disjoint_and_conserve_independent_volume():
    """📐 Compare physical prisms with independent GEOS area integration and swept-volume union."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=api.load_model()
    for row in CASE['cabinets']:
        g=api.solid_geometry(row['tag'],m);parts=g['parts'];zs=sorted({p[k] for p in parts for k in ('z0','z1')})
        independent=sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in parts)
        occupied=sum(unary_union([Polygon(p['outline']) for p in parts if p['z0']<(lo+hi)/2<p['z1']]).area*(hi-lo) for lo,hi in zip(zs,zs[1:]))
        assert math.isclose(independent,occupied,abs_tol=CASE['tolerance']),(row['tag'],independent,occupied)
        assert math.isclose(api.quantities(row['tag'],m)['volume'],occupied,abs_tol=CASE['tolerance'])
        assert {p['material'] for p in parts}==set(CASE['materials'])
        points=[v for p in parts for v in api.part_vertices(p)];bounds=row['bounds']
        assert min(v[0] for v in points)>=bounds[0]-.025 and max(v[0] for v in points)<=bounds[2]+.025
        assert min(v[1] for v in points)>=bounds[1]-.025 and max(v[1] for v in points)<=bounds[3]+.025
        assert abs(min(v[2] for v in points)-CASE['floor']-bounds[4])<1e-8 and abs(max(v[2] for v in points)-CASE['floor']-bounds[5])<1e-8
        for language in ('en','de'):assert api.step_excerpt(row['tag'],m,language=language)['class']=='IfcFurnishingElement'
