"""🌿️ The bounded exterior gravel follows a sourced terrain anchor and true analytic shaft clearances."""
import ast,importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def test_inscribed_circle_reservations_have_independently_proven_positive_material_intrusion():
    """🧬 Reproduce the old inscribed-proxy defect with analytic circle area and independent GEOS."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,Point
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    row=CASE['previous_circle'];r,n=row['radius'],row['segments'];ring=[[r*math.cos(2*math.pi*i/n),r*math.sin(2*math.pi*i/n)] for i in range(n)];polygon=Polygon(ring);gap=(math.pi*r*r-polygon.area)*row['depth'];assert gap>0 and math.isclose(gap,row['gap_volume'],abs_tol=CASE['comparison']['volume']);assert math.isclose(api._area(ring),polygon.area,abs_tol=CASE['comparison']['volume'])
    angle=math.pi/n;rho=r*(1+math.cos(angle))/2;point=[rho*math.cos(angle),rho*math.sin(angle)];assert math.hypot(*point)<r and not polygon.contains(Point(point)) and not api._contains_point(ring,point)

def test_external_gravel_uses_qualified_terrain_datum_and_visible_wearing_role():
    """🪨 Keep the local sourced anchor separate from inferred grading and real vertical wearing depth."""
    from shapely.geometry import Polygon
    from jsonschema import Draft202012Validator
    schema=json.loads((Path(api.__file__).parent/'🧬️schema/🔣️.schema.json').read_text(encoding='utf8'))
    for row in CASE['elevation_cases']:
        value={'reference_surface_elevation':{'value':row['value'],'unit':row['unit'],'basis':'sourced','source':'Neutral signed datum contract'}}
        oracle=Draft202012Validator({'$defs':schema['$defs'],'$ref':'#/$defs/attributes'}).is_valid(value)
        assert oracle==row['valid'],row
    m=api.load_model();obj=api._find(m,CASE['product']);g=api._solid_geometry(m,obj);stock=Polygon(CASE['stock']);assert g['parts']
    for p in g['parts']:
        assert CASE['reference_elevation']-CASE['depth']-CASE['comparison']['coordinate']<=p['z0']<p['z1']<=CASE['comparison']['coordinate']
        pairs={}
        for x,y,z in api.part_vertices(p):pairs.setdefault((x,y),[]).append(z)
        assert all(math.isclose(max(values)-min(values),CASE['depth'],abs_tol=CASE['comparison']['coordinate']) for values in pairs.values())
        assert p['role']=='wearing' and p['material']=='MAT-GRAVEL'
        assert stock.covers(Polygon(p['outline']))
    path=Path(api.__file__).parents[1]/'🎨️visuals/🏛️savoye/📋️plan/🐍️.py';tree=ast.parse(path.read_text(encoding='utf8'));module=ast.Module(body=[node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in ('plan_part_rank','plan_part_visible')],type_ignores=[]);namespace={};exec(compile(module,str(path),'exec'),namespace)
    assert all(namespace['plan_part_visible'](g,p,'ground',(0,3.17),True) for p in g['parts'])
    for key in ('reference_surface_elevation','nominal_depth','maximum_column_reservation','UniformExtension'):
        for language in ('en','de'):assert m['property_names'][key]['name'][language]
    assert obj['attributes']['reference_surface_elevation']['basis']=='sourced'
    assert obj['attributes']['reference_surface_elevation']['value']==CASE['reference_elevation']
    assert obj['psets']['Pset_TerrainReconstruction']['UniformExtension']['basis']=='inferred'

def test_gravel_has_no_positive_analytic_circle_overlap_at_its_actual_vertical_band():
    """🔭 Independently measure polygon distance to original circle centres, without a polygonal circle oracle."""
    from shapely.geometry import Polygon,Point,box
    from shapely.strtree import STRtree
    m=api.load_model();parts=api._solid_geometry(m,api._find(m,CASE['product']))['parts'];polygons=[Polygon(p['outline']) for p in parts];tree=STRtree(polygons);checks=0
    for obj in api._by_class(m,'IfcColumn'):
        for p in api.column_geometry(m,obj)['parts']:
            if p.get('profile',{}).get('shape')!='circle':continue
            radius=p['profile']['diameter']['value']/2;center=[(min(v[k] for v in p['outline'])+max(v[k] for v in p['outline']))/2 for k in range(2)];point=Point(center)
            for index in tree.query(box(center[0]-radius,center[1]-radius,center[0]+radius,center[1]+radius)):
                actual=parts[int(index)]
                if min(actual['z1'],p['z1'])<=max(actual['z0'],p['z0']):continue
                checks+=1;assert polygons[int(index)].distance(point)>=radius-CASE['comparison']['coordinate'],(obj['tag'],int(index),polygons[int(index)].distance(point),radius)
    assert checks>0


def _height_at(parts,point):
    """📍 Independently intersect actual upward native faces at one registered XY station."""
    import numpy as np
    from shapely.geometry import Polygon,Point
    heights=[]
    for part in parts:
        if not Polygon(part['outline']).covers(Point(point)):continue
        vertices=np.asarray(api.part_vertices(part),float)
        for face in api.part_faces(part):
            ring=vertices[face];normal=None
            for i in range(1,len(ring)-1):
                candidate=np.cross(ring[i]-ring[0],ring[i+1]-ring[0])
                if np.any(candidate):normal=candidate;break
            if normal is None or normal[2]<=0 or not Polygon(ring[:,:2]).covers(Point(point)):continue
            heights.append(float(ring[0,2]-(normal[0]*(point[0]-ring[0,0])+normal[1]*(point[1]-ring[0,1]))/normal[2]))
    if not heights:raise AssertionError(('No authored gravel surface at required station',point))
    return max(heights)


def test_gravel_has_continuous_explicitly_inferred_garage_approach_at_actual_portal():
    """🚗 Reject the old15cm step while retaining the sourced reference and honest inference metadata."""
    m=api.load_model();obj=api._find(m,CASE['product']);parts=api._solid_geometry(m,obj)['parts'];spec=CASE['approach']
    for sample in spec['samples']:
        actual=_height_at(parts,sample['point'])
        assert math.isclose(actual,sample['height'],abs_tol=spec['maximum_vertical_facet_error']),('approach height',sample,actual)
    assert obj['attributes']['reference_surface_elevation']['value']==CASE['reference_elevation']
    terrain=obj['psets']['Pset_TerrainReconstruction'];assert terrain['UniformExtension']['value'] is False
    assert terrain['GarageApproachInferred']['value'] is True and terrain['GarageApproachInferred']['basis']=='inferred'
    assert terrain['GarageApproachRun']['value']==spec['run'] and terrain['GarageApproachRise']['value']==spec['rise']
    for key in ('GarageApproachInferred','GarageApproachRun','GarageApproachRise','GarageApproachFanSegments','GarageApproachMaximumHeightError'):
        assert terrain[key]['basis']=='inferred'
        for language in ('en','de'):assert m['property_names'][key]['name'][language]
