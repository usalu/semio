"""🚪️ Ground-bearing concrete continues through the actual garage-door reveal without moving the registered opening."""
import importlib,json,math
from copy import deepcopy
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def test_garage_threshold_source_programme_has_independent_positive_reveal_coverage():
    """🧾 Validate a language-neutral reveal domain and exact source programmes with independent GEOS."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,box
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    m=api.load_model();slab=api._find(m,CASE['slab']);r=next(r for r in slab['geometry']['regions']if r['tag']==CASE['region']);e=CASE['expected'];required=box(e['x0'],e['y0'],e['x1'],e['y1']);assert math.isclose(required.area,e['area'],abs_tol=CASE['comparison']['coordinate'])
    for current,old in ((slab['geometry']['outline'],CASE['previous']['outline']),(r['outline'],CASE['previous']['region'])):
        poly=Polygon(current);prior=Polygon(old);assert poly.covers(required);assert math.isclose(poly.difference(prior).area,e['area'],abs_tol=CASE['comparison']['coordinate']);assert prior.difference(poly).is_empty;assert math.isclose(api._area(current),poly.area,abs_tol=CASE['comparison']['coordinate'])
    assert r['material']==e['material'] and r['thickness']['value']==e['thickness'] and r['top_offset']['value']==e['top'];assert slab['attributes']['garage_threshold_width']['basis']=='inferred'

def test_native_garage_threshold_material_reaches_both_sides_of_the_unchanged_leaf():
    """🔬 Prove actual owned concrete at both former void strips and compare added volume with GEOS."""
    from shapely.geometry import Polygon,Point
    m=api.load_model();slab=api._find(m,CASE['slab']);g=api._slab_geometry_entry(m,slab);parts=g['parts'];e=CASE['expected'];band=(-.02,-.01)
    for point in CASE['points']:
        assert any(Polygon(ring).contains(Point(point)) for p in parts for ring in api.part_band_footprints(p,*band)),point
    old=deepcopy(m);previous=api._find(old,CASE['slab']);previous['geometry']['outline']=CASE['previous']['outline'];next(r for r in previous['geometry']['regions']if r['tag']==CASE['region'])['outline']=CASE['previous']['region'];prior=api._slab_geometry_entry(old,previous)['parts'];volume=lambda cells:math.fsum(api._polyhedron_volume(p)for p in cells);assert math.isclose(volume(parts)-volume(prior),e['volume'],abs_tol=CASE['comparison']['volume'])
    for point in CASE['points']:assert not any(Polygon(ring).contains(Point(point)) for p in prior for ring in api.part_band_footprints(p,*band))
    opening=api._find(m,CASE['opening']);host=api._find(m,CASE['host']);assert opening['geometry']['wall']==host['tag'] and opening['attributes']['sill']['value']==0;path=api._wall_path(m,host);assert math.isclose(path[0][1]+opening['attributes']['start']['value'],e['y0'],abs_tol=1e-12) and math.isclose(path[0][1]+opening['attributes']['end']['value'],e['y1'],abs_tol=1e-12);assert math.isclose(path[0][0]+api._wall_thickness(m,host)/2,e['x1'],abs_tol=1e-12)


def test_proven_disjoint_surface_cut_preserves_original_triangle_without_skipping_positive_two_nanometre_material():
    """🧬 Reproduce the actual bounding-box-only overpartition with independent GEOS and a positive2nm control."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    case=CASE['separation'];piece=Polygon(case['pieces'][0]);void=Polygon(case['voids'][0]);assert piece.is_valid and void.is_valid and piece.intersection(void).area==0
    actual=api._surface_difference(deepcopy(case['pieces']),case['voids']);assert actual==case['pieces'];assert all(Polygon(p).is_valid for p in actual)
    stock=Polygon(case['positive_stock']);cut=Polygon(case['positive_cut']);reference=stock.difference(cut);expected=stock.intersection(cut).area;assert math.isclose(expected,case['positive_removed_area'],abs_tol=1e-15)
    parts=api._surface_difference([case['positive_stock']],[case['positive_cut']]);union=unary_union([Polygon(p)for p in parts]);assert union.symmetric_difference(reference).area<1e-15;assert math.isclose(api._area(case['positive_stock'])-math.fsum(api._area(p)for p in parts),expected,abs_tol=1e-15)
