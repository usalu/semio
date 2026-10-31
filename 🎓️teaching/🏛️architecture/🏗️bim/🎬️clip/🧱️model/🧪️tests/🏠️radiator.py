"""🏠️ The source-attested opposite-wall radiator must fit below the actual ramp and bear on the hall floor."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def _independent_intersection(a,b):
    """🔬 Intersect independently normalized SciPy convex halfspaces around an LP interior point."""
    import numpy as np
    from scipy.optimize import linprog
    from scipy.spatial import ConvexHull,HalfspaceIntersection
    points=[np.asarray(p['mesh']['vertices'],float) for p in (a,b)];origin=np.vstack(points).min(axis=0);scale=np.vstack(points).max(axis=0)-origin;planes=np.vstack([ConvexHull((v-origin)/scale).equations for v in points]);norm=np.linalg.norm(planes[:,:3],axis=1);lp=linprog([0,0,0,-1],A_ub=np.column_stack((planes[:,:3],norm)),b_ub=-planes[:,3],bounds=[(None,None)]*3+[(0,None)],method='highs');assert lp.success and lp.x[3]>0
    return ConvexHull(HalfspaceIntersection(planes,lp.x[:3]).intersections).volume*float(np.prod(scale))

def test_original_radiator_ramp_overlap_matches_independent_scipy_and_native_difference():
    """🧬 Reproduce the retained original positive component intersection before fitting any source geometry."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    row=CASE['previous'];expected=_independent_intersection(row['radiator'],row['ramp']);remaining,reported=api._subtract_native_material([row['radiator']],[row['ramp']]);actual=api._polyhedron_volume(row['radiator'])-sum(api._polyhedron_volume(p) for p in remaining)
    assert expected>0 and math.isclose(expected,row['volume'],abs_tol=CASE['comparison']['volume'])
    assert math.isclose(actual,expected,abs_tol=CASE['comparison']['volume'])
    assert math.isclose(reported,expected,abs_tol=CASE['comparison']['volume'])

def test_actual_opposite_wall_radiator_has_the_nearest_fitted_station_below_the_source_ramp():
    """📐 Preserve the source-qualified assembly while proving all material lies below actual soffit controls."""
    independent=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔻️volume').independent_volume
    m=api.load_model();obj=api._find(m,CASE['radiator']);parts=api._service(m,obj)['parts'];vertices=[v for p in parts for v in api.part_vertices(p)];stock=api._ramp_reference_stock(m,api._find(m,CASE['flight']));bottom=stock['mesh']['vertices'][:4];y0,y1=min(v[1] for v in bottom),max(v[1] for v in bottom);z0,z1=min(v[2] for v in bottom if v[1]==y0),min(v[2] for v in bottom if v[1]==y1);slope=(z1-z0)/(y1-y0);clearances=[float(z0+slope*(api._source_fraction(v[1])-y0)-api._source_fraction(v[2])) for v in vertices]
    assert min(clearances)>=CASE['minimum_soffit_clearance']-CASE['comparison']['coordinate'],min(clearances)
    assert obj['geometry']['axis']==CASE['axis'];relative_y=min(v[1] for v in vertices)-obj['geometry']['axis'][1];required_y=float(y0+(api._source_fraction(max(v[2] for v in vertices))+api._source_fraction(CASE['minimum_soffit_clearance'])-z0)/slope)-relative_y
    assert required_y-CASE['comparison']['coordinate']<=CASE['axis'][1]<required_y+CASE['station_increment']+CASE['comparison']['coordinate']
    assert len(parts)==CASE['part_count'] and sum(p.get('role')=='radiator_foot' for p in parts)==CASE['foot_count']
    volume=sum(independent(api.part_vertices(p)) for p in parts);assert math.isclose(volume,CASE['body_volume'],abs_tol=CASE['comparison']['volume'])
    assert math.isclose(volume,sum(api._polyhedron_volume(p) for p in parts),abs_tol=CASE['comparison']['volume'])

def test_actual_radiator_feet_bear_on_retained_hall_finish_inside_the_source_room():
    """👣 Check every true foot face against occupied floor caps and the actual hall programme."""
    from shapely import union_all
    from shapely.geometry import Polygon
    m=api.load_model();body=api._service(m,api._find(m,CASE['radiator']));floor=api._solid_geometry(m,api._find(m,CASE['floor']));caps=[]
    for p in floor['parts']:
        vertices=api.part_vertices(p)
        for f in api.part_faces(p):
            if all(vertices[i][2]==0 for i in f):
                ring=Polygon([vertices[i][:2] for i in f])
                if ring.area>0:caps.append(ring)
    support=union_all(caps);room=Polygon(api._expand(m,api._find(m,'SP-HALL')['outline']))
    for p in (p for p in body['parts'] if p.get('role')=='radiator_foot'):
        vertices=api.part_vertices(p);foot=Polygon([v[:2] for v in vertices if v[2]==0]).convex_hull
        assert foot.area>0 and foot.difference(support).area<=CASE['comparison']['volume'],foot.bounds
        assert room.covers(foot)
