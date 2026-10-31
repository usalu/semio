"""🚿 Source-registered servant niches, hollow cupboards and operable mounted taps."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_original_stepped_niches_and_missing_cupboards_are_registered():
    """📜 Preserve original divider topology and both source-attested cupboard identities."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,Point
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASE)
    m=api.load_model();assert api.element(CASE['wall'],m)['geometry']['path']==CASE['path']
    for row in CASE['basins']:
        obj=api.element(row['tag'],m);assert obj['geometry']['axis']==row['axis']
        assert Polygon(api._expand(m,api.element(row['space'],m)['outline'])).covers(Point(row['axis']))
        parts=api.element(row['tap'],m)['geometry']['parts'];assert sum(p.get('role')=='tap_stem' for p in parts)==2*CASE['segments']
        assert sum(p.get('role')=='tap_cross_handle' for p in parts)==8
    for row in CASE['cabinets']:
        obj=api.element(row['tag'],m);assert obj['class']=='IfcFurnishingElement'
        assert obj['historical_evidence']['status']=='inferred_reconstruction'


def _slice(parts,z,material=None):
    """🔬 Cut each actual convex component independently with SciPy half-space edges and GEOS."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import MultiPoint
    from shapely.ops import unary_union
    shapes=[]
    for p in parts:
        if not p['z0']<z<p['z1'] or material is not None and p['material']!=material:continue
        vertices=np.asarray(api.part_vertices(p));hull=ConvexHull(vertices);edges={tuple(sorted((int(a),int(b))))for tri in hull.simplices for a,b in zip(tri,[tri[1],tri[2],tri[0]])};points=[]
        for a,b in edges:
            lower,upper=vertices[a],vertices[b]
            if (lower[2]-z)*(upper[2]-z)<0:
                q=lower+(upper-lower)*(z-lower[2])/(upper[2]-lower[2]);points.append(q[:2])
            if abs(lower[2]-z)<1e-12:points.append(lower[:2])
            if abs(upper[2]-z)<1e-12:points.append(upper[:2])
        if len(points)>=3:shapes.append(MultiPoint(points).convex_hull)
    return unary_union(shapes),sum(s.area for s in shapes)

def test_niche_taps_have_empty_airways_positive_walls_and_actual_ceramic_mounts():
    """🫧 Prove actual bores, deck contact, empty falling-water paths and disjoint metal/ceramic."""
    from shapely.geometry import Point
    m=api.load_model()
    for row in CASE['basins']:
        basin=api.service_geometry(row['tag'],m);tap=api.solid_geometry(row['tap'],m);ceramic,_=_slice(basin['parts'],.89,'MAT-SANITARY-CERAMIC');metal,_=_slice(tap['parts'],.89,'MAT-METAL');air,_=_slice(tap['parts'],.96);cavity,_=_slice(basin['parts'],.80);floor,_=_slice(basin['parts'],.70)
        assert ceramic.intersection(metal).area<CASE['tolerance'],row['tag']
        assert ceramic.boundary.intersection(metal.boundary.buffer(1e-8)).length>.10,(row['tag'],'unsupported stems')
        for x,y in row['mounts']:
            assert not air.covers(Point(x,y)),row['tap']
            assert air.covers(Point(x+.011,y)),row['tap']
            outlet=[x-.16*row['back'][0],y-.16*row['back'][1]]
            assert not cavity.covers(Point(outlet)),(row['tag'],'water hits material',outlet)
            assert floor.covers(Point(outlet)),(row['tag'],'water misses bowl floor',outlet)
        assert not floor.covers(Point(row['axis'])),(row['tag'],'blocked drain')

def test_niche_components_and_hollow_cupboards_conserve_independent_material_volume():
    """⚖️ Compare independent SciPy volumes and GEOS unions, retaining every cabinet cavity."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Point
    m=api.load_model();rows=[(r['tag'],True)for r in CASE['basins']]+[(r['tap'],False)for r in CASE['basins']]+[(r['tag'],False)for r in CASE['cabinets']]
    for tag,service in rows:
        g=api.service_geometry(tag,m)if service else api.solid_geometry(tag,m);volume=sum(ConvexHull(np.asarray(api.part_vertices(p))).volume for p in g['parts'])
        assert math.isclose(volume,api.quantities(tag,m)['volume'],abs_tol=CASE['tolerance']),(tag,volume)
        for z in [.035,.10,.25,.50,.68,.695,.75,.885,.96,1.015,1.1]:
            shape,total=_slice(g['parts'],z);assert abs(total-shape.area)<CASE['tolerance'],(tag,z,'overlapping material',total-shape.area)
        for language in ('en','de'):assert api.step_excerpt(tag,m,language=language)['lines']
    for r in CASE['cabinets']:
        g=api.solid_geometry(r['tag'],m);x,y,z=r['void'];shape,_=_slice(g['parts'],z);assert not shape.covers(Point(x,y)),r['tag']
        assert any(p['role']=='cabinet_shelf' for p in g['parts'])and any(p['role']=='cabinet_pull' for p in g['parts'])


def test_niche_assemblies_clear_actual_host_material_and_brackets_touch_it():
    """🏗️ Keep all cupboard/basin material outside actual wall layers and supported at real interfaces."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=api.load_model();walls=[api._wall_geometry(m,api.element(t,m))for t in ['W-G-W','W-G-E',CASE['wall'],'W-G-SERVICE']];assets=[api.service_geometry(r['tag'],m)for r in CASE['basins']]+[api.solid_geometry(r['tap'],m)for r in CASE['basins']]+[api.solid_geometry(r['tag'],m)for r in CASE['cabinets']]
    for z in [.10,.50,.68,.70,.80,.89,.96,1.015]:
        wall=unary_union([Polygon(r)for w in walls for p in w['parts']if p['z0']<z<p['z1']for r in api.part_regions(p,z)])
        for asset in assets:
            shape,_=_slice(asset['parts'],z);assert shape.intersection(wall).area<CASE['tolerance'],(asset['tag'],z,shape.intersection(wall).area)
    chauffeur=assets[2];bracket,_=_slice([p for p in chauffeur['parts']if p['role']=='basin_bracket'],.60);east=unary_union([Polygon(r)for p in walls[1]['parts']if p['z0']<.60<p['z1']for r in api.part_regions(p,.60)])
    assert bracket.boundary.intersection(east.boundary.buffer(1e-8)).length>.035,'floating chauffeur brackets'
