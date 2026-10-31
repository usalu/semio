"""🚰️ The actual patio collector must have one aligned physical slab and finish bore."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def test_patio_bore_contract_reproduces_original_occupied_overlap_independently():
    """🧬 Compare the original metal/slab intersection with independent GEOS and the native planar kernel."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,Point
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    a,b=CASE['previous']['sleeve'],CASE['previous']['slab'];height=min(a['z1'],b['z1'])-max(a['z0'],b['z0']);expected=Polygon(a['outline']).intersection(Polygon(b['outline'])).area*height;actual=api._area(api._analysis._clip(a['outline'],b['outline']))*height
    assert expected>0 and math.isclose(expected,CASE['previous']['volume'],abs_tol=CASE['comparison']['volume'])
    assert math.isclose(actual,expected,abs_tol=CASE['comparison']['volume'])
    assert Polygon(a['outline']).contains(Point(CASE['previous']['witness'][:2]))

def test_existing_collector_bore_and_axis_follow_the_actual_source_qualified_sleeve():
    """🎯 Align the existing source opening to its actual sleeve with explicit inferred fitting clearance."""
    from shapely.geometry import Polygon
    from shapely import union_all
    m=api.load_model();collector=api._find(m,CASE['collector']);parts=[p for p in api._service(m,collector)['parts'] if p['role']=='inlet_sleeve'];shell=union_all([Polygon(p['outline']) for p in parts]).convex_hull;center=[(shell.bounds[k]+shell.bounds[k+2])/2 for k in range(2)];opening=api._find(m,CASE['opening']);hole=Polygon(api._expand(m,opening['geometry']['outline']))
    assert all(abs(center[k]-CASE['axis'][k])<CASE['comparison']['coordinate'] for k in range(2))
    assert all(abs(collector['geometry']['axis'][k]-center[k])<CASE['comparison']['coordinate'] for k in range(2))
    assert opening['geometry']['slab']==CASE['slab']
    assert hole.buffer(CASE['comparison']['coordinate']).covers(shell.buffer(CASE['installation_fit'])),(hole.bounds,shell.bounds)
    assert not hole.covers(Polygon(CASE['previous']['slab']['outline'])), 'The real slab must not be replaced by a broad void'

def test_actual_slab_finish_and_bore_own_each_point_once_with_an_open_flow_core():
    """🕳️ Independently prove real sleeve clearance, open core and retained adjacent slab and finish support."""
    from shapely.geometry import Polygon,Point
    from shapely import union_all
    m=api.load_model();slab=api._slab_geometry(m,api._find(m,CASE['slab']));collector=api._service(m,api._find(m,CASE['collector']));finish=api._solid_geometry(m,api._find(m,CASE['covering']));hole=Polygon(api._expand(m,api._find(m,CASE['opening'])['geometry']['outline']))
    for body in (slab,finish):
        assert all(Polygon(p['outline'],p.get('holes',())).intersection(hole).area<=CASE['comparison']['volume'] for p in body['parts'])
        for sleeve in (p for p in collector['parts'] if p['role']=='inlet_sleeve'):
            for part in body['parts']:
                height=min(sleeve['z1'],part['z1'])-max(sleeve['z0'],part['z0'])
                if height>0:assert Polygon(sleeve['outline']).intersection(Polygon(part['outline'],part.get('holes',()))).area*height<=CASE['comparison']['volume']
    point=Point(CASE['axis']);assert not any(Polygon(p['outline']).contains(point) for p in collector['parts'])
    control=Point(CASE['axis'][0]+.08,CASE['axis'][1]);assert any(Polygon(p['outline'],p.get('holes',())).contains(control) for p in slab['parts']);assert any(Polygon(p['outline'],p.get('holes',())).contains(control) for p in finish['parts'])
