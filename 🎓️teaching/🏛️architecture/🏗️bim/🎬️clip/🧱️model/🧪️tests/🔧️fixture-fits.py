"""🔧 Coordinate visible fixture stock with its real drain, raised rim and structural envelope."""
import importlib,json,math
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')

def case():
    """📖 Validate the language-neutral source-qualified fitting controls."""
    from jsonschema import Draft202012Validator
    path=Path(__file__);data=json.loads(path.with_suffix('.json').read_text(encoding='utf8'));Draft202012Validator(json.loads(path.with_suffix('.schema.json').read_text(encoding='utf8'))).validate(data);return data

def shifted(part,delta):
    """🧭 Move actual native vertices rigidly without changing section or part ownership."""
    out=deepcopy(part)
    if 'mesh'in out:out['mesh']['vertices']=[[v[k]+delta[k]for k in range(3)]for v in out['mesh']['vertices']]
    else:
        for key in('outline','top_outline'):
            if key in out:out[key]=[[v[k]+delta[k]for k in range(2)]for v in out[key]]
        if 'holes'in out:out['holes']=[[[v[k]+delta[k]for k in range(2)]for v in ring]for ring in out['holes']]
        out['z0']+=delta[2];out['z1']+=delta[2]
    return out

def parts(source,tag):
    """🧱 Read published stocks in world coordinates or the actual complete native structural reference."""
    obj=api._find(source,tag);g=obj['geometry']
    if 'parts'not in g:
        body=api._ramp_geometry(source,obj)if obj['class']=='IfcRampFlight'else api.solid_geometry(tag,source);return deepcopy(body.get('full_parts',body['parts']))
    base=g.get('base_level',obj['storey']);z=base if isinstance(base,(int,float))else next(s['elevation']['value']for s in source['storeys']if s['tag']==base);return[shifted(p,[0,0,z])for p in g['parts']]

def separated(a,b):
    """↔️ Prove material separation with exact binary64 support intervals on finite arbitrary axes."""
    import numpy as np
    if any(p.get('profile',{}).get('shape')=='circle'or 'radius'in p for p in(a,b)):return False
    va,vb=[np.asarray(api.part_vertices(p),float)for p in(a,b)]
    if any(va[:,k].max()<=vb[:,k].min()or vb[:,k].max()<=va[:,k].min()for k in range(3)):return True
    qa=[[Fraction.from_float(float(x))for x in v]for v in va];qb=[[Fraction.from_float(float(x))for x in v]for v in vb];axes=[]
    for p,v in((a,qa),(b,qb)):
        for f in api.part_faces(p):
            for i in range(1,len(f)-1):
                u=[v[f[i]][k]-v[f[0]][k]for k in range(3)];w=[v[f[i+1]][k]-v[f[0]][k]for k in range(3)];n=[u[1]*w[2]-u[2]*w[1],u[2]*w[0]-u[0]*w[2],u[0]*w[1]-u[1]*w[0]]
                if any(n):axes.append(n);break
    for q in axes:
        aa=[sum(x*y for x,y in zip(v,q))for v in qa];bb=[sum(x*y for x,y in zip(v,q))for v in qb]
        if max(aa)<=min(bb)or max(bb)<=min(aa):return True
    return False

def contains(part,point):
    """🔬 Independently locate a strictly interior positive witness using conditioned SciPy halfspaces."""
    import numpy as np
    from scipy.spatial import ConvexHull
    v=np.asarray(api.part_vertices(part),float);origin=v.mean(axis=0);scale=np.ptp(v,axis=0).max();hull=ConvexHull((v-origin)/scale);return bool(np.all(hull.equations[:,:3]@((np.asarray(point)-origin)/scale)+hull.equations[:,3]<-1e-10))

def test_patio_pipe_spine_matches_real_collector_and_retains_hollow_closed_stock():
    """🌧️ Retain the actual lower station and gauges while aligning the formerly obsolete upper inlet."""
    import numpy as np
    from scipy.spatial import ConvexHull
    c=case()['rain'];source=api.load_model();obj=api._find(source,c['tag']);stock=obj['geometry']['parts'];assert len(stock)==3*c['sectors'];groups=[[p for p in stock if p['role']=='pipe_wall_'+str(i)]for i in range(3)]
    for i,group in enumerate(groups):
        for offset,expected in((0,c['spine'][i]),(4,c['spine'][i+1])):np.testing.assert_allclose(np.mean([v for p in group for v in p['mesh']['vertices'][offset:offset+4]],axis=0),expected,atol=1e-12,rtol=0)
    assert api._find(source,c['collector'])['geometry']['axis']==c['spine'][0][:2];assert obj['geometry']['axis']==c['spine'][-1][:2]
    volumes=[]
    for p in stock:
        vertices=np.asarray(p['mesh']['vertices'],float);local=vertices-vertices.mean(axis=0);edges=[(a,b)for f in p['mesh']['faces']for a,b in zip(f,f[1:]+f[:1])];assert all(edges.count((b,a))==1 for a,b in edges);volumes.append(ConvexHull(local).volume);assert volumes[-1]>0
    area=c['sectors']/2*math.sin(2*math.pi/c['sectors'])*((c['diameter']/2)**2-(c['diameter']/2-c['gauge'])**2);expected=area*sum(math.dist(a,b)for a,b in zip(c['spine'],c['spine'][1:]));assert math.isclose(math.fsum(volumes),expected,rel_tol=1e-9,abs_tol=1e-12)
    for point in c['spine']:assert not any(contains(p,point)for p in stock)

def test_tap_flange_keeps_deck_bearing_and_clears_actual_raised_ceramic_rim():
    """🚰 Compare real flange/rim volumes and three distributed rear bearing controls with independent GEOS."""
    from shapely.geometry import Polygon,Point
    from shapely.ops import unary_union
    c=case();source=api.load_model()
    for row in c['taps']:
        assert api._find(source,row['tag'])['geometry']['axis']==row['axis'];tap=parts(source,row['tag']);flange=[p for p in tap if p['role']=='deck_flange'];rim=[p for p in parts(source,row['sink'])if p['role']=='sink_rim'];deck=parts(source,row['counter']);assert len(flange)==32;volume=math.fsum(Polygon(a['outline']).intersection(Polygon(b['outline'])).area*max(0,min(a['z1'],b['z1'])-max(a['z0'],b['z0']))for a in flange for b in rim);assert volume==0,(row['tag'],volume)
        bottom=min(p['z0']for p in flange);top=unary_union([Polygon(p['outline'],p.get('holes',[]))for p in deck if p.get('z1')==bottom]);foot=unary_union([Polygon(p['outline'])for p in flange]);assert foot.intersection(top).area>0
        for angle in(-45,-90,-135):
            theta=math.radians(angle);point=Point(row['axis'][0]+.020*math.cos(theta),row['axis'][1]+.020*math.sin(theta));assert foot.covers(point)and top.covers(point),(row['tag'],angle)

def test_lamp_keeps_host_family_and_human_height_while_clearing_full_ramp_guard_reference():
    """💡 Preserve fixed column19 and all lamp material, with exact separation and a retained old-pose positive witness."""
    c=case()['lamp'];source=api.load_model();obj=api._find(source,c['tag']);assert obj['geometry']['mounting']['host']==c['host'];assert obj['geometry']['mounting']['point'][2]==c['mount_height'];lamp=parts(source,c['tag']);assert len(lamp)==99;assert min(v[2]for p in lamp for v in api.part_vertices(p))>c['minimum_human_clearance'];targets=[p for tag in c['obstacles']for p in parts(source,tag)]
    for a in lamp:
        for b in targets:assert separated(a,b),'Lamp material separation from the actual structural reference is unproven'
    old=[shifted(p,[0,0,c['old_mount_height']-c['mount_height']])for p in lamp];point=c['positive_witness'];assert any(contains(p,point)for p in old)and any(contains(p,point)for p in targets),'Old occupied collision control was lost'

def test_exact_support_separation_preserves_positive_two_nanometre_volume():
    """🧮 Validate the conservative support proof against independent GEOS on contact and true2nm penetration."""
    from shapely.geometry import box
    a={'outline':[[0,0],[1,0],[1,1],[0,1]],'z0':0,'z1':1};touch=shifted(a,[1,0,0]);overlap=shifted(a,[1-2e-9,0,0]);assert separated(a,touch);assert box(0,0,1,1).intersection(box(1,0,2,1)).area==0;assert not separated(a,overlap);assert box(0,0,1,1).intersection(box(1-2e-9,0,2-2e-9,1)).area>1e-9


def test_patio_pipe_material_clears_real_owned_slab_finish_and_collector_sleeve():
    """🕳️ Certify every positive pipe cell against actual bore material without a nominal masking cylinder."""
    c=case()['rain'];source=api.load_model();pipe=parts(source,c['tag'])
    for tag in c['obstacles']:
        obj=api._find(source,tag);targets=api._slab_geometry_entry(source,obj)['parts']if obj['class']=='IfcSlab'else parts(source,tag)
        for a in pipe:
            for b in targets:assert separated(a,b),(c['tag'],tag,'Actual material separation is unproven')


def test_pantry_storage_rear_plane_clears_the_complete_source_wall_envelope():
    """🗄️ Preserve all cabinet parts and prove exact contact with the declared full wall strip using independent GEOS."""
    from shapely.geometry import Polygon,box
    c=case()['storage'];source=api.load_model();obj=api._find(source,c['tag']);wall=api._find(source,c['wall']);assert wall['geometry']['path']==c['wall_path'];assert wall['attributes']['thickness']['value']==c['wall_thickness'];stock=obj['geometry']['parts'];assert len(stock)==c['parts'];q=lambda x:Fraction(str(x));assert q(c['rear_x'])==q(c['wall_path'][0][0])+q(c['wall_thickness'])/2;rear=min(v[0]for p in stock for v in p['outline']);assert rear==c['rear_x'],(rear,c['rear_x']);envelope=box(c['rear_x']-c['wall_thickness'],c['wall_path'][0][1],c['rear_x'],c['wall_path'][1][1]);assert all(Polygon(p['outline']).intersection(envelope).area==0 for p in stock);assert any(Polygon(p['outline']).boundary.intersection(envelope.boundary).length>0 for p in stock)
