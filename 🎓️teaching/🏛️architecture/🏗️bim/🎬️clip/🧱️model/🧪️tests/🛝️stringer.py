"""🏗️ The original single garage-side ramp stringer retains its attested concrete section."""
import importlib,json
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def test_original_stringer_neutral_section_and_registered_supports():
    """🧬 Compare the sourced12x26 section with independent GEOS and its actual three registered supports."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    d=CASE['section'];a=CASE['alignment'];width=d['width'];depth=d['vertical_depth']
    profile=[[a['y0'],a['rc_top_start']-depth],[a['y1'],a['rc_top_end']-depth],[a['y1'],a['rc_top_end']],[a['y0'],a['rc_top_start']]]
    assert abs(Polygon(profile).area*width-(a['y1']-a['y0'])*width*depth)<1e-12
    source=api.load_model()
    for tag in CASE['supports']:assert api._find(source,tag)['geometry']['axis'][0]==a['x']

def test_single_attested_ramp_stringer_exists_with_qualified_section_and_native_material():
    """🪚 Restore one real beam, not four speculative mirror copies or a visual line."""
    from scipy.spatial import ConvexHull
    source=api.load_model();obj=api._find(source,CASE['tag'])
    assert obj['class']==CASE['class'] and obj['storey']==CASE['storey'] and obj['material']==CASE['material']
    assert obj['attributes']['width']['value']==CASE['section']['width']
    assert obj['attributes']['height']['value']==CASE['section']['vertical_depth']
    assert all(obj['attributes'][k]['basis']=='sourced' for k in ('width','height'))
    body=api._solid_geometry(source,obj);assert body['parts']
    volume=0.
    for part in body['parts']:
        assert part.get('material',obj['material'])==CASE['material']
        independent=ConvexHull(api.part_vertices(part)).volume
        assert independent>0 and abs(independent-api._polyhedron_volume(part))<1e-9
        volume+=independent
    a=CASE['alignment'];d=CASE['section'];assert 0<volume<=(a['y1']-a['y0'])*d['width']*d['vertical_depth']+1e-9
