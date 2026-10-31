"""🧺 Completed-plan laundry pair with real troughs, drains, supports and operable taps."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_completed_plan_laundry_pair_has_native_programme_and_longitudinal_stations():
    """📜 Retain two real longitudinal troughs without transferring an earlier design as executed geometry."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASE)
    m=api.load_model();obj=api.element(CASE['tag'],m);assert obj['class']==CASE['class']and obj['predefined_type']==CASE['predefined_type']
    parts=api.service_geometry(CASE['tag'],m)['parts'];points=[v for p in parts for v in api.part_vertices(p)];bounds=[min(p[0]for p in points),min(p[1]for p in points),max(p[0]for p in points),max(p[1]for p in points)]
    assert all(abs(a-b)<1e-9 for a,b in zip(bounds,CASE['bounds'])),bounds
    assert sum(p['role']=='trough_leg'for p in parts)==CASE['legs']
    for row in CASE['bowls']:
        tap=api.element(row['tap'],m);assert sum(p.get('role')=='tap_stem'for p in tap['geometry']['parts'])==CASE['tap_segments'];assert sum(p.get('role')=='tap_cross_handle'for p in tap['geometry']['parts'])==4


def test_laundry_drains_tap_bores_and_concrete_deck_contacts_are_real():
    """🫧 Independently slice empty drains, falling-water paths and positively mounted hollow taps."""
    from shapely.geometry import Point
    cut=importlib.import_module(__package__+'.🚿️niches')._slice;m=api.load_model();g=api.service_geometry(CASE['tag'],m);concrete,_=cut(g['parts'],.94)
    for row in CASE['bowls']:
        x,y,_=row['mount'];tap=api.solid_geometry(row['tap'],m);metal,_=cut(tap['parts'],.94);air,_=cut(tap['parts'],1.0)
        assert concrete.intersection(metal).area<CASE['tolerance']
        assert concrete.boundary.intersection(metal.boundary.buffer(1e-8)).length>.05,row['tap']
        assert not air.covers(Point(x,y))and air.covers(Point(x+.011,y))
        floor,_=cut(g['parts'],.69);cavity,_=cut(g['parts'],.85);ox,oy,_=row['outlet'];dx,dy,_=row['drain']
        assert not cavity.covers(Point(ox,oy))and floor.covers(Point(ox,oy)),row['tap']
        assert not floor.covers(Point(dx,dy)),'filled drain'

def test_laundry_native_material_volume_and_independent_union_are_conserved():
    """⚖️ Match exact native quantities to independent SciPy components and nonoverlapping GEOS slices."""
    import numpy as np
    from scipy.spatial import ConvexHull
    cut=importlib.import_module(__package__+'.🚿️niches')._slice;m=api.load_model()
    for tag in [CASE['tag'],*[r['tap']for r in CASE['bowls']]]:
        g=api.service_geometry(tag,m)if tag==CASE['tag']else api.solid_geometry(tag,m);volume=sum(ConvexHull(np.asarray(api.part_vertices(p))).volume for p in g['parts'])
        assert math.isclose(volume,api.quantities(tag,m)['volume'],abs_tol=CASE['tolerance']),(tag,volume)
        for z in [.1,.4,.69,.8,.94,1.0,1.065,1.145]:
            shape,total=cut(g['parts'],z);assert abs(total-shape.area)<CASE['tolerance'],(tag,z,total-shape.area)
        for language in ('en','de'):assert api.step_excerpt(tag,m,language=language)['lines']
