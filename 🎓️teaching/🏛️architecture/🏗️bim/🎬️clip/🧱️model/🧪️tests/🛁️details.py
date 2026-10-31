"""🛁 Source-evidenced fixed fixtures checked against independent occupied geometry."""
import importlib,json,math
from pathlib import Path
model=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')

def _contract():
    """🧬 Read language-neutral current fixture identities and retained circulation witnesses."""
    return json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def _fixture(tag,source):
    """🚰 Resolve native furnishings and distribution terminals through their actual owned producer."""
    obj=model.element(tag,source)
    return model.service_geometry(tag,source) if obj['class'] in model._exchange.SERVICE_CLASSES else model.solid_geometry(tag,source)

def _complex(part):
    """🔷 Distinguish true three-dimensional components from vertical polygon prisms."""
    return any(key in part for key in ('mesh','section','top_outline'))

def _cut_points(part,z):
    """✂️ Independently intersect convex native vertices and SciPy hull edges with a horizontal plane."""
    from scipy.spatial import ConvexHull
    vertices=model.part_vertices(part);faces=ConvexHull(vertices).simplices;edges={tuple(sorted((int(a),int(b)))) for face in faces for a,b in zip(face,list(face[1:])+[face[0]])};points=[p[:2] for p in vertices if abs(p[2]-z)<1e-10]
    for i,j in edges:
        a,b=vertices[i],vertices[j]
        if (a[2]-z)*(b[2]-z)<0:
            t=(z-a[2])/(b[2]-a[2]);points.append([a[k]+t*(b[k]-a[k]) for k in (0,1)])
    return points

def _slice(parts,z):
    """🔬 Evaluate real mesh/loft cuts and nonconvex prism holes with independent SciPy/GEOS."""
    from shapely.geometry import Polygon,MultiPoint
    from shapely.ops import unary_union
    polygons=[]
    for p in parts:
        if not p['z0']<=z<=p['z1']:continue
        polygons.append(MultiPoint(_cut_points(p,z)).convex_hull if _complex(p) else Polygon(p['outline'],p.get('holes',[])))
    return unary_union(polygons)

def _band(part,z0,z1):
    """🚶 Project only occupied material in the requested body-height slab."""
    from shapely.geometry import Polygon,MultiPoint
    lo,hi=max(z0,part['z0']),min(z1,part['z1'])
    if hi-lo<=1e-10:return Polygon()
    if not _complex(part):return Polygon(part['outline'],part.get('holes',[]))
    points=[v[:2] for v in model.part_vertices(part) if lo<=v[2]<=hi]+_cut_points(part,lo)+_cut_points(part,hi)
    return MultiPoint(points).convex_hull

def test_details_have_actual_hollow_and_occupied_volumes():
    """💧 Keep retained cavity probes and require every reconstructed fixture's dedicated portable geometry contract."""
    from shapely.geometry import Point
    source=model.load_model();registry={e['tag']:e for e in source['elements']}
    levels={s['tag']:s['elevation']['value'] for s in source['storeys']}
    for case in _contract()['fixtures']:
        if 'geometry_contract' in case:
            path=Path(__file__).with_name(case['geometry_contract']);assert path.is_file() and case['tag'] in path.read_text(encoding='utf8'),case['tag']
        parts=_fixture(case['tag'],source)['parts'];base=levels[registry[case['tag']]['storey']]
        assert parts
        for x,y,z in case.get('cavities',[]):
            cut=_slice(parts,base+z);assert not cut.intersects(Point(x,y).buffer(.04)) and cut.area>.02,case['tag']
        for x,y,z in case.get('occupied',[]):assert _slice(parts,base+z).covers(Point(x,y).buffer(.02)),case['tag']

def test_detail_placement_and_independent_quantities():
    """📐 Verify current room containment, real support separation and independent three-dimensional material volume."""
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    source=model.load_model();geometry=model.building_geometry(source);rooms={e['tag']:Polygon(e['outline']) for e in geometry['spaces']}
    columns=[(column['tag'],cp,_band(cp,cp['z0'],cp['z1'])) for column in geometry['pilotis'] for cp in column['parts']]
    for case in _contract()['fixtures']:
        item=_fixture(case['tag'],source);volume=0.
        for part in item['parts']:
            footprint=_band(part,part['z0'],part['z1']);assert footprint.is_valid and rooms[case['room']].buffer(1e-8).covers(footprint),case['tag']
            volume+=ConvexHull(model.part_vertices(part)).volume if _complex(part) else Polygon(part['outline'],part.get('holes',[])).area*(part['z1']-part['z0'])
            for column_tag,cp,column_projection in columns:
                if column_tag in case.get('integrates',[]) or not footprint.intersects(column_projection):continue
                lo,hi=max(part['z0'],cp['z0']),min(part['z1'],cp['z1'])
                if hi-lo>1e-9:assert _band(part,lo,hi).intersection(_band(cp,lo,hi)).area<1e-8,(case['tag'],column_tag)
        assert math.isclose(volume,model.quantities(case['tag'],source)['volume'],abs_tol=1e-7),(case['tag'],volume,model.quantities(case['tag'],source)['volume'])

def test_details_leave_person_routes_and_door_approaches_clear():
    """🚶 Check exact fixture unions once per occupied band; dedicated room contracts retain structural/open-leaf access."""
    from shapely.geometry import LineString
    from shapely.ops import unary_union
    source=model.load_model();geometry=model.building_geometry(source);contract=_contract();items={row['tag']:_fixture(row['tag'],source) for row in contract['fixtures']};levels=geometry['storeys'];bands={}
    def occupied(tag,height):
        """🗂️ Reuse the unchanged independent occupied-material union for one fixture and body height."""
        key=(tag,height)
        if key not in bands:
            item=items[tag];base=levels[item['storey']];bands[key]=unary_union([_band(p,base,base+height) for p in item['parts']])
        return bands[key]
    def hits(passage,storey,height):
        """🔎 Return every positive fixture collision without recomputing component hulls for each portal."""
        return [(tag,passage.intersection(occupied(tag,height)).area) for tag,item in items.items() if item['storey']==storey and passage.intersection(occupied(tag,height)).area>1e-8]
    for route in contract['routes']:
        passage=LineString(route['points']).buffer(route['width']/2,cap_style=2);collisions=hits(passage,route['storey'],2.1)
        assert not collisions,collisions
    for wall in geometry['walls']+geometry['screens']:
        for opening in wall['openings']:
            if opening.get('fill_class')!='IfcDoor':continue
            tag=opening['fill_tag'];a,b=opening['p0'],opening['p1'];dx,dy=b[0]-a[0],b[1]-a[1];length=math.hypot(dx,dy);centre=[(a[i]+b[i])/2 for i in (0,1)];normal=[-dy/length*.45,dx/length*.45];witness=contract.get('door_approaches',{}).get(tag);diameter=contract['door_body_default']['width'];height=contract['door_body_default']['height']
            if witness:
                centre=witness.get('center',centre);diameter=witness['width'];height=witness.get('height',height);station=((centre[0]-a[0])*dx+(centre[1]-a[1])*dy)/length
                assert abs((centre[0]-a[0])*dy-(centre[1]-a[1])*dx)<1e-10 and diameter/2<station<length-diameter/2,tag
            points=witness.get('points') if witness else None
            path=LineString(points or [[centre[i]-normal[i] for i in (0,1)],[centre[i]+normal[i] for i in (0,1)]])
            assert path.intersects(LineString([a,b])),tag
            passage=path.buffer(diameter/2,cap_style=2);base=levels[wall['storey']];filling=model.filling_geometry(tag,source)
            assert filling['clear_width']+contract['body_interface_tolerance_m']>=diameter and filling['clear_height']+contract['body_interface_tolerance_m']>=height,(tag,filling['clear_width'],filling['clear_height'],diameter,height)
            assert sum(passage.intersection(_band(p,base,base+height)).area for p in filling['parts'] if p['role']=='frame')<1e-8,tag
            collisions=hits(passage,wall['storey'],height);assert not collisions,(tag,collisions)


def test_stair_guards_follow_flights_with_connected_tube_and_open_arrival():
    """🌀 Slice actual continuous U-guard meshes at each tread station using independent GEOS."""
    from shapely.geometry import Point,Polygon,MultiPoint
    from shapely.ops import unary_union
    geometry=model.building_geometry();registry={e["tag"]:e for e in geometry["railings"]}
    for case in _contract()["guards"]:
        stair=next(e for e in geometry["stairs"] if e["tag"]==case["stair"]);cx,cy=stair["centre"]
        assert stair["base"]==case["base"] and stair["top"]==case["top"]
        rise=(stair["top"]-stair["base"])/len(stair["steps"])
        for i,step in enumerate(stair["steps"]):
            if i<6:x,y=sum(p[0] for p in step["outline"])/len(step["outline"]),cy-stair["radius"]-.04
            elif i<12:
                angle=-math.pi/2+math.pi*(i-6+.5)/6;x,y=cx+(stair["radius"]+.04)*math.cos(angle),cy+(stair["radius"]+.04)*math.sin(angle)
            else:x,y=sum(p[0] for p in step["outline"])/len(step["outline"]),cy+stair["radius"]+.04
            z=stair["base"]+(i+.5)*rise+.5;slices=[]
            for part in registry[case["guard"]]["parts"]:
                if not part["z0"]<=z<=part["z1"]:continue
                vertices=part["mesh"]["vertices"];points=[]
                for face in part["mesh"]["faces"]:
                    for ia,ib in zip(face,face[1:]+face[:1]):
                        a,b=vertices[ia],vertices[ib]
                        if abs(a[2]-z)<1e-10:points.append(a[:2])
                        if (a[2]-z)*(b[2]-z)<0:
                            t=(z-a[2])/(b[2]-a[2]);points.append([a[k]+t*(b[k]-a[k]) for k in (0,1)])
                if len(points)>=3:slices.append(MultiPoint(points).convex_hull)
            assert unary_union(slices).buffer(1e-8).covers(Point(x,y)),(case["stair"],i)
        assert registry[case["tube"]]["parts"]


def test_original_guest_niche_and_registered_zenith_opening_preserve_their_distinct_programmes():
    """🌤️ Keep the measured roof aperture while placing both original fixture symbols in their own source-registered stations."""
    from shapely.geometry import Polygon,Point,box
    case=json.loads(Path(__file__).with_name('🧺washfixtures.json').read_text(encoding='utf8'))['guest'];source=model.load_model();clear=box(*case['clear_bounds']);slab=next(e for e in source['elements'] if e['tag']==case['slab']);aperture=next(v for v in slab['geometry']['voids'] if v['tag']==case['void'])
    assert Polygon(aperture['outline']).equals(clear)
    room=Polygon(next(s for s in source['spaces'] if s['tag']==case['room'])['outline'])
    for tag in (case['basin'],case['bidet']):assert room.covers(Point(*model.element(tag,source)['geometry']['axis']))
    assert clear.buffer(case['uncertainty_m']).covers(Point(*model.element(case['bidet'],source)['geometry']['axis']))
    assert model.element(case['basin'],source)['geometry']['axis'][1]<model.element(case['bidet'],source)['geometry']['axis'][1]


def test_registered_laundry_material_clearances_and_every_declared_tap_discharge():
    """🧺 Preserve actual laundry access and require every specified water outlet to project into its real bowl cavity."""
    from shapely.geometry import Polygon,Point,LineString,box
    from shapely.ops import unary_union
    case=json.loads(Path(__file__).with_name('🧺washfixtures.json').read_text(encoding='utf8'));source=model.load_model();record=case['laundry'];items={tag:_fixture(tag,source)['parts'] for tag in record['tags']};geometry=model.building_geometry(source)
    walls=[p for e in geometry['walls']+geometry['screens'] if e['storey']=='ground' for p in e['parts']];columns=[p for e in geometry['pilotis'] for p in e['parts']];fixtures=[p for row in _contract()['fixtures'] for item in [_fixture(row['tag'],source)] if item['storey']=='ground' for p in item['parts']];room=Polygon(next(s for s in source['spaces'] if s['tag']==record['room'])['outline'])
    actual=unary_union([_band(p,p['z0'],p['z1']) for p in items['FIX-LAUNDRY']]).bounds;assert all(math.isclose(a,b,abs_tol=1e-10) for a,b in zip(actual,record['bounds']))
    physical=[(q,_band(q,q['z0'],q['z1'])) for q in columns+walls]
    for tag,parts in items.items():
        for p in parts:
            projection=_band(p,p['z0'],p['z1']);assert room.covers(projection),tag
            for q,other in physical:
                if not projection.intersects(other):continue
                lo,hi=max(p['z0'],q['z0']),min(p['z1'],q['z1'])
                if hi-lo>1e-10:assert _band(p,lo,hi).intersection(_band(q,lo,hi)).area*(hi-lo)<case['tolerances']['volume'],tag
    framing=[p for p in model.filling_geometry('D-LAUNDRY',source)['parts'] if p['role']=='frame'];obstacles=columns+walls+framing+fixtures
    for domain in (LineString(record['route']).buffer(record['body_width']/2,cap_style=2),box(*record['standing_bounds'])):
        assert sum(domain.intersection(_band(p,0,record['body_height'])).area for p in obstacles)<case['tolerances']['area']
    levels=geometry['storeys']
    for tap,discharge in case['discharge_samples'].items():
        basin=_fixture(discharge['basin'],source);base=levels[basin['storey']];tap_parts=_fixture(tap,source)['parts'];projection=unary_union([_band(p,p['z0'],p['z1']) for p in tap_parts])
        for x,y,z in discharge['points']:
            assert not _slice(basin['parts'],base+z).intersects(Point(x,y).buffer(.02)),tap
            assert projection.buffer(1e-10).covers(Point(x,y)),tap


def test_fixture_cases_are_schema_valid_and_door_witnesses_name_actual_portals():
    """🧬 Validate portable geometry witnesses and reject unknown doors without changing historical dimensions."""
    from jsonschema import Draft202012Validator
    contract=_contract();validator=Draft202012Validator(json.loads(Path(__file__).with_name('🛁️details.schema.json').read_text(encoding='utf8')));validator.validate(contract);source=model.load_model();registry={e['tag']:e for e in source['elements']};rooms={s['tag']for s in source['spaces']}
    assert len({r['tag']for r in contract['fixtures']})==len(contract['fixtures'])
    for row in contract['fixtures']:assert row['tag']in registry and row['room']in rooms,row['tag']
    for tag in contract['door_approaches']:assert registry[tag]['class']=='IfcDoor',tag
