"""🚶 Actual hall body passage, honest nominal width limits and retained operated door material."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_hall_body_route_retains_all_native_parts_and_actual_laundry_leaf():
    """🚪 Prove accepted body-band sweeps, actual floor support and rejection of the wider cylinder."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon,LineString,box
    from shapely.ops import unary_union
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASE)
    helpers=importlib.import_module(__package__+'.🛁️details');m=api.load_model();g=api.building_geometry(m);inventory=[]
    for family in ['pilotis','slabs','walls','fillings','screens','stairs','details','furnishings','beams','foundations','railings','services']:inventory+=g[family]
    inventory+=[g['curtain_wall'],*g['ramp']['ramps']];obstacles=[];floors=[];leaf=[];unoperated=[];complete_leaf=[];complete_fixed=[];fixed=[];fixed_tags=[];domain=box(*CASE['domain']);pose=CASE['operated'][0];hx,hy=pose['hinge'];lo,hi=CASE['band']
    for item in inventory:
        for p in item['parts']+item.get('child_parts',[]):
            (complete_leaf if item['tag']==pose['tag']and 'frame'not in p.get('role','').lower() else complete_fixed).append((item['tag'],p))
            if abs(p['z1'])<1e-9:floors.append(helpers._slice([p],p['z1']))
            if p['z1']<=lo or p['z0']>=hi:continue
            shape=helpers._band(p,lo,hi)
            if item['tag']==pose['tag']and 'frame'not in p.get('role','').lower():
                from shapely.affinity import rotate
                unoperated.append(shape);shape=rotate(shape,pose['angle_degrees'],origin=(hx,hy));leaf.append(shape)
            else:fixed.append(shape);fixed_tags.append((item['tag'],shape))
            if shape.intersects(domain):obstacles.append((item['tag'],shape))
    assert len(complete_leaf)==pose['parts'] and len(unoperated)==pose['body_parts'] and unary_union(leaf).intersection(unary_union(fixed)).area<CASE['area_tolerance'],'operated leaf hits fixed material'
    for _,part in complete_leaf:
        swept=rotate(helpers._band(part,part['z0'],part['z1']),pose['angle_degrees'],origin=(hx,hy));bounds=swept.bounds
        for tag,other in complete_fixed:
            lower,upper=max(part['z0'],other['z0']),min(part['z1'],other['z1'])
            if upper<=lower:continue
            footprint=api.part_footprint(other)
            if max(p[0]for p in footprint)<bounds[0]or min(p[0]for p in footprint)>bounds[2]or max(p[1]for p in footprint)<bounds[1]or min(p[1]for p in footprint)>bounds[3]:continue
            actual=rotate(helpers._band(part,lower,upper),pose['angle_degrees'],origin=(hx,hy)).intersection(helpers._band(other,lower,upper)).area
            assert actual==0,(tag,'complete operated leaf intersects fixed material',actual)
    control=CASE['interference_control'];blocked=unary_union([rotate(shape,control['angle_degrees'],origin=(hx,hy)) for shape in unoperated]);overlap=blocked.intersection(unary_union([shape for tag,shape in fixed_tags if tag==control['host']])).area
    assert overlap>0 and math.isclose(overlap,control['expected_overlap_area'],rel_tol=1e-7,abs_tol=1e-12),'original90degree interference must remain a positive control'
    support=unary_union(floors);occupied=unary_union([p for _,p in obstacles])
    for row in CASE['cases']:
        route=LineString(row['points']);body=route.buffer(row['diameter']/2+CASE['radial_margin'],quad_segs=128)
        hits=[(tag,p.intersection(body).area)for tag,p in obstacles if p.intersection(body).area>1e-10]
        assert not hits,(row['id'],hits)
        assert body.difference(support).area<CASE['area_tolerance'],row['id']
    for diameter,expected in ((CASE['connected_diameter'],True),(CASE['rejected_diameter'],False)):
        free=domain.difference(occupied.buffer(diameter/2,quad_segs=128));regions=list(free.geoms)if hasattr(free,'geoms')else[free];start=LineString([[7,12.2],[8.15,12.2]]);end=LineString([[7,CASE['connectivity_end_y']],[8.15,CASE['connectivity_end_y']]])
        assert any(part.intersects(start)and part.intersects(end)for part in regions)==expected,(diameter,'declared configuration-space connectivity')
