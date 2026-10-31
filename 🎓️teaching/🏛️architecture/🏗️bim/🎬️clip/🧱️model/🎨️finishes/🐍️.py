"""🎨 Source-qualified face identities on existing material envelopes."""
from copy import deepcopy
import math
from fractions import Fraction

def wall_regions(wall,geometry,bands,material,station):
    """🧱 Resolve bounded source regions without changing the substrate envelope or declaring film volume."""
    result=[];tags=set()
    known={(layer['id'],layer['material']) for band in bands for layer in band['layers'] if not layer.get('is_void')}
    for raw in wall['geometry'].get('surface_regions',[]):
        if raw['tag'] in tags:raise ValueError('Surface region tags must be unique')
        tags.add(raw['tag'])
        if (raw['layer'],raw['substrate']) not in known:raise ValueError('Surface region requires an existing substrate layer')
        if raw['start']['unit']!='m' or raw['end']['unit']!='m':raise ValueError('Surface stations require metre units')
        start,end=raw['start']['value'],raw['end']['value']
        lower,upper=station(raw['from'],geometry),station(raw['to'],geometry)
        if not all(math.isfinite(x) for x in (start,end,lower,upper)):raise ValueError('Surface regions require finite bounds')
        if not 0<=start<end<=geometry['length']:raise ValueError('Surface stations must lie inside their host')
        if not geometry['base']<=lower<upper<=geometry['top']:raise ValueError('Surface heights must lie inside their host')
        definition=material(raw['material']);substrate=material(raw['substrate'])
        finish=definition.get('surface_finish',{})
        if definition['class']!='IfcMaterial' or finish.get('substrate')!=raw['substrate'] or finish.get('quantity_scope')!='existing_substrate_envelope' or finish.get('film_thickness')!='unmeasured':
            raise ValueError('Surface identity must retain an explicit unmeasured-film substrate scope')
        if any(definition['density'][key]!=substrate['density'][key] for key in ('value','unit')):
            raise ValueError('Surface identity must retain substrate unit weight; coating mass is unmeasured')
        if raw['face'] not in ('positive','negative'):raise ValueError('Surface region requires an explicit native face')
        region={**deepcopy(raw),'start':start,'end':end,'z0':lower,'z1':upper,'finish':deepcopy(finish)}
        for other in result:
            if region['layer']==other['layer'] and max(start,other['start'])<min(end,other['end']) and max(lower,other['z0'])<min(upper,other['z1']):
                raise ValueError('Surface identities cannot overlap on one substrate layer')
        result.append(region)
    return result

def cell_region(regions,layer,start,end,lower,upper):
    """🧩 Select the unique region of an already partitioned physical material cell."""
    station,height=(start+end)/2,(lower+upper)/2
    return next((region for region in regions if region['layer']==layer['id'] and region['substrate']==layer['material'] and region['start']<=station<region['end'] and region['z0']<=height<region['z1']),None)

def face_identity(region,geometry,offsets):
    """🎯 Bind the finish to one outward physical plane rather than colouring buried substrate faces."""
    dx=(geometry['end'][0]-geometry['start'][0])/geometry['length']
    dy=(geometry['end'][1]-geometry['start'][1])/geometry['length']
    sign=1 if region['face']=='positive' else -1
    normal=[-sign*dy,sign*dx,0.]
    value=offsets[1] if sign==1 else offsets[0]
    offset=sign*(-dy*geometry['start'][0]+dx*geometry['start'][1]+value)
    return {**deepcopy(region['finish']),'tag':region['tag'],'material':region['material'],'normal':normal,'offset':offset,'registration_basis':region['basis'],'registration_source':region['source']}

def face_area(points):
    """📐 Measure one actual planar face using anchored signed cross products."""
    origin=points[0];crosses=[[],[],[]]
    for index in range(1,len(points)-1):
        a=[points[index][k]-origin[k] for k in range(3)]
        b=[points[index+1][k]-origin[k] for k in range(3)]
        for axis,value in enumerate((a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])):crosses[axis].append(value)
    return math.sqrt(sum(math.fsum(values)**2 for values in crosses))/2

def face_matches(points,identity):
    """🧭 Select only the outward face lying on the source-bound treatment plane."""
    origin=points[0];normal=[0.,0.,0.]
    for index in range(1,len(points)-1):
        a=[points[index][k]-origin[k] for k in range(3)]
        b=[points[index+1][k]-origin[k] for k in range(3)]
        cross=(a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])
        normal=[normal[k]+cross[k] for k in range(3)]
    length=math.sqrt(sum(value*value for value in normal))
    return length>0 and math.fsum(normal[k]*identity['normal'][k] for k in range(3))/length>1-1e-10 and all(abs(math.fsum(point[k]*identity['normal'][k] for k in range(3))-identity['offset'])<1e-10 for point in points)

def summaries(regions,parts,vertices,faces):
    """⚖️ Keep treated face area separate from the unchanged substrate volume."""
    rows={region['tag']:{'tag':region['tag'],'material':region['material'],'substrate':region['substrate'],'binder':region['finish']['binder'],'colour_family':region['finish']['colour_family'],'quantity_scope':'existing_substrate_envelope','film_thickness':'unmeasured','area':0.,'substrate_volume':0.} for region in regions}
    for part in parts:
        identity=part.get('surface_finish')
        if identity is None:continue
        row=rows[identity['tag']];points=vertices(part)
        row['area']+=math.fsum(face_area([points[i] for i in face]) for face in faces(part) if face_matches([points[i] for i in face],identity))
        row['substrate_volume']+=face_area([[*p,0.] for p in part['outline']])*(part['z1']-part['z0'])
    if any(row['area']<=0 for row in rows.values()):raise ValueError('Surface region requires an actual occupied substrate face')
    return list(rows.values())


def face_appearance(part,points,fallback):
    """🖌️ Bind an inferred display colour only to its actual source-qualified outward face."""
    uniform=part.get("surface_appearance")
    identity=uniform or part.get("surface_finish",{})
    display=identity.get("display_rgb")
    if display and (uniform is not None or face_matches(points,identity)):
        return {"family":identity["colour_family"],"rgb":deepcopy(display["value"]),"basis":display["basis"]}
    return fallback

def appearance_colour(appearance,fallback):
    """🎨 Convert a declared illustrative RGB without claiming an original measured colour."""
    if not isinstance(appearance,dict):return fallback
    rgb=appearance.get("rgb",[])
    if len(rgb)!=3 or any(not isinstance(v,(int,float)) or not math.isfinite(v) or not 0<=v<=1 for v in rgb):raise ValueError("Display RGB must contain three finite unit components")
    return "#"+"".join(f"{round(v*255):02X}" for v in rgb)


def skirting_stock(start,end,normal,section,material,*,extensions=(0.,0.),back_clearance=0.):
    """🪵 Extrude an explicitly rounded timber stock section without scaling its thickness on an incline."""
    t,h,r,n=[section[k] for k in ("thickness","height","round_radius","round_segments")]
    if not all(isinstance(v,(int,float)) and math.isfinite(v) for p in (start,end,normal) for v in p) or any(len(p)!=3 for p in (start,end,normal)):raise ValueError("Stock axes require finite three-dimensional points")
    length=math.sqrt(sum((b-a)**2 for a,b in zip(start,end)))
    if not math.isfinite(back_clearance) or back_clearance<0 or len(extensions)!=2 or any(not math.isfinite(v) or v<0 for v in extensions):raise ValueError("Stock fitting clearance and overlength extensions must be finite and nonnegative")
    if length<=0 or not 0<r<min(t,h) or min(t,h)<=0 or not isinstance(n,int) or n<2:raise ValueError("Stock section and longitudinal length must be positive")
    d=[(b-a)/length for a,b in zip(start,end)]
    if abs(sum(v*v for v in normal)-1)>1e-12 or abs(sum(a*b for a,b in zip(d,normal)))>1e-12:raise ValueError("Stock face normal must be unit and orthogonal to its longitudinal axis")
    up=[d[1]*normal[2]-d[2]*normal[1],d[2]*normal[0]-d[0]*normal[2],d[0]*normal[1]-d[1]*normal[0]]
    if up[2]<0:up=[-v for v in up]
    if up[2]<=0:raise ValueError("Stock longitudinal axis must retain a horizontal component")
    profile=[[0.,0.],[t,0.]]+[[t-r+r*math.cos(i*math.pi/(2*n)),h-r+r*math.sin(i*math.pi/(2*n))] for i in range(n+1)]+[[0.,h]]
    count=len(profile)
    axis=[(Fraction(b)-Fraction(a))/Fraction(length) for a,b in zip(start,end)]
    exact_normal=[Fraction(v) for v in normal]
    if start[2]==end[2] and normal[2]==0:
        sign=1 if normal[0]*float(axis[1])-normal[1]*float(axis[0])>0 else -1
        exact_normal=[sign*axis[1],-sign*axis[0],Fraction(0)];up=[0,0,1]
    origins=[[Fraction(p[k])+axis[k]*Fraction(amount) for k in range(3)] for p,amount in ((start,-extensions[0]),(end,extensions[1]))]
    exact_vertices=[[origin[k]+exact_normal[k]*(Fraction(u)+Fraction(back_clearance))+Fraction(up[k])*Fraction(v) for k in range(3)] for origin in origins for u,v in profile]
    vertices=[[float(value) for value in p] for p in exact_vertices]
    faces=[list(range(count-1,-1,-1)),list(range(count,2*count))]+[[i,(i+1)%count,(i+1)%count+count,i+count] for i in range(count)]
    handed=math.fsum(d[k]*value for k,value in enumerate((normal[1]*up[2]-normal[2]*up[1],normal[2]*up[0]-normal[0]*up[2],normal[0]*up[1]-normal[1]*up[0])))
    if handed<0:faces=[face[::-1] for face in faces]
    result=_ClippedStock(_stock_mesh(vertices,faces,material,"skirting"))
    result._origin={**deepcopy(dict(result)),"mesh":{"vertices":exact_vertices,"faces":deepcopy(faces)}}
    result._planes=();result._exact_mesh=None
    return result


def _stock_mesh(vertices,faces,material,role):
    """📦 Record the exact convex mesh projection and elevations of one physical timber piece."""
    points=sorted(set((p[0],p[1]) for p in vertices))
    def half(points):
        """📏 Keep the exact convex projection of this convex stock without introducing a geometry dependency."""
        result=[]
        for p in points:
            while len(result)>1 and (result[-1][0]-result[-2][0])*(p[1]-result[-2][1])-(result[-1][1]-result[-2][1])*(p[0]-result[-2][0])<=0:result.pop()
            result.append(p)
        return result
    outline=[list(p) for p in half(points)[:-1]+half(points[::-1])[:-1]]
    def strength(face,index):
        """🔺 Choose a stable cyclic face origin without deleting collinear boundary vertices."""
        points=[vertices[face[(index+j)%len(face)]] for j in range(3)]
        return face_area(points)
    faces=[face[k:]+face[:k] for face in faces for k in [max(range(len(face)),key=lambda index:strength(face,index))]]
    return {"material":material,"role":role,"outline":outline,"z0":min(p[2] for p in vertices),"z1":max(p[2] for p in vertices),"mesh":{"vertices":vertices,"faces":faces}}


class _StockTopologyError(ValueError):
    """🧩 A computed clip boundary requires exact replay of its original operands."""


def _positive_face(points):
    """🔺 Preserve every nonzero anchored face normal without an arbitrary area threshold."""
    origin=points[0];axes=[[],[],[]]
    for i in range(1,len(points)-1):
        a=[points[i][k]-origin[k] for k in range(3)];b=[points[i+1][k]-origin[k] for k in range(3)]
        for k,v in enumerate((a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])):axes[k].append(v)
    accumulate=sum if isinstance(origin[0],Fraction) else math.fsum
    return any(accumulate(values)!=0 for values in axes)


class _ClippedStock(dict):
    """🧾 Own transient original operands outside the JSON shape of one authored stock."""
    __slots__=("_origin","_planes","_exact_mesh")


def clip_stock(part,planes,*,exact=False):
    """✂️ Keep convex material with owned exact-state continuation; public mesh values are read-only previews until publication."""
    origin=part._origin if isinstance(part,_ClippedStock) else deepcopy(dict(part))
    history=part._planes if isinstance(part,_ClippedStock) else ()
    state=part._exact_mesh if isinstance(part,_ClippedStock) else None
    ordered=history+tuple(deepcopy(planes))
    if exact or state is not None:
        working={**dict(part),"mesh":state} if state is not None else origin
        result=_clip_stock_kernel(working,planes if state is not None else ordered,True,True)
    else:
        try:result=_clip_stock_kernel(part,planes,False)
        except _StockTopologyError:result=_clip_stock_kernel(origin,ordered,True,True)
    if result is None:return None
    result,state=result if isinstance(result,tuple) else (result,None)
    result=_ClippedStock(result);result._origin=origin;result._planes=ordered;result._exact_mesh=state
    return result


def replay_stock(part):
    """🧮 Rebuild from the original primitive and every ordered cut without inherited publication rounding."""
    if not isinstance(part,_ClippedStock):raise ValueError("Exact stock replay requires owned construction operands")
    result=_clip_stock_kernel(part._origin,part._planes,True,True)
    if result is None:return None
    result,state=result;result=_ClippedStock(result);result._origin=part._origin;result._planes=part._planes;result._exact_mesh=state
    return result


def publish_stock(part):
    """📤 Emit an independently owned plain JSON mesh once construction and its exact predicates are complete."""
    return deepcopy(dict(part))



def _plane_values(plane,exact):
    """📐 Resolve original construction planes with their signed metric clearance."""
    normal,offset=plane["normal"],plane["offset"]
    if exact:
        normal=[Fraction(v) for v in normal]
        if "points" in plane:
            a,b,c=[[Fraction(v) for v in p] for p in plane["points"]]
            u,v=[[p[k]-a[k] for k in range(3)] for p in (b,c)]
            cross=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
            sign=1 if sum(n*x for n,x in zip(normal,cross))>0 else -1
            normal=[sign*x for x in cross];offset=sum(n*x for n,x in zip(normal,a))
        else:offset=sum(n*Fraction(v) for n,v in zip(normal,plane["point"])) if "point" in plane else Fraction(offset)
    if plane.get("clearance",0):
        distance=plane["clearance"]
        if not math.isfinite(distance):raise ValueError("Authored signed plane offset must be finite")
        offset+=(Fraction(distance)*Fraction(math.hypot(*normal))) if exact else distance*math.hypot(*normal)
    if len(normal)!=3 or not all(math.isfinite(v) for v in [*normal,offset]) or sum(v*v for v in normal)==0:raise ValueError("Ownership planes require a finite positive normal")
    return normal,offset


def stock_separated(part,planes):
    """🤝 Reject only a provably empty-volume convex intersection before material partitioning."""
    if isinstance(part,_ClippedStock):
        state=part._exact_mesh
        if state is None:
            rebuilt=replay_stock(part)
            if rebuilt is None:return True
            state=rebuilt._exact_mesh
    else:state=part["mesh"]
    vertices=state["vertices"]
    for plane in planes:
        normal,offset=_plane_values(plane,True)
        if all(sum(n*Fraction(v) for n,v in zip(normal,p))-offset>=0 for p in vertices):return True
    return False


def _clip_stock_kernel(part,planes,exact,keep_state=False):
    """✂️ Clip one convex timber stock against exact joint or ownership half-spaces, retaining a closed solid."""
    vertices=deepcopy(part["mesh"]["vertices"]);faces=deepcopy(part["mesh"]["faces"])
    accumulate=sum if exact else math.fsum
    if exact:
        vertices=[[Fraction(v) for v in p] for p in vertices]
    for plane in planes:
        normal,offset=_plane_values(plane,exact)
        signed=[accumulate(n*v for n,v in zip(normal,p))-offset for p in vertices]
        if all(v<=0 for v in signed):continue
        if all(v>=0 for v in signed):return None
        updated=[];crossings={};point_index={tuple(p):i for i,p in enumerate(vertices)}
        for face in faces:
            result=[]
            for a,b in zip(face,face[1:]+face[:1]):
                if signed[a]<=0:result.append(a)
                if (signed[a]<0<signed[b]) or (signed[b]<0<signed[a]):
                    key=tuple(sorted((a,b)))
                    if key not in crossings:
                        i,j=key;ratio=signed[i]/(signed[i]-signed[j])
                        point=[vertices[i][k]+ratio*(vertices[j][k]-vertices[i][k]) for k in range(3)]
                        axis=max(range(3),key=lambda k:abs(normal[k]))
                        point[axis]=(offset-accumulate(normal[k]*point[k] for k in range(3) if k!=axis))/normal[axis]
                        identity=tuple(point)
                        if identity not in point_index:point_index[identity]=len(vertices);vertices.append(point)
                        crossings[key]=point_index[identity]
                    result.append(crossings[key])
            result=list(dict.fromkeys(result))
            if len(result)>=3 and _positive_face([vertices[i] for i in result]):updated.append(result)
        incidence={};directed={}
        for face in updated:
            for a,b in zip(face,face[1:]+face[:1]):
                key=tuple(sorted((a,b)));incidence[key]=incidence.get(key,0)+1;directed[key]=(a,b)
        boundary=[directed[key][::-1] for key,count in incidence.items() if count==1]
        links={a:b for a,b in boundary}
        if len(links)!=len(boundary):raise _StockTopologyError("Clipped stock boundary must form disjoint simple cycles")
        while links:
            start=next(iter(links));loop=[];point=start
            while point in links:
                loop.append(point);point=links.pop(point)
                if point==start:break
            if point!=start:raise _StockTopologyError("Clipped stock boundary is open")
            if len(loop)>=3 and _positive_face([vertices[i] for i in loop]):updated.append(loop)
        if not updated:return None
        used=sorted({i for face in updated for i in face});mapping={old:new for new,old in enumerate(used)}
        vertices=[vertices[i] for i in used];faces=[[mapping[i] for i in face] for face in updated]
    state={"vertices":vertices,"faces":faces} if exact and keep_state else None
    if exact:vertices=[[float(v) for v in p] for p in vertices]
    result=_stock_mesh(vertices,faces,part["material"],part["role"])
    if len(result["outline"])<3 or result["z1"]<=result["z0"]:return None
    result={**deepcopy(dict(part)),**result}
    return (result,state) if keep_state else result


def apply_material_appearance(parts,material,default=None):
    """🧵 Attach a uniformly treated material surface without recolouring an unrelated layer or changing its volume."""
    for part in parts:
        tag=part.get("material",default)
        if tag is None:continue
        finish=material(tag).get("surface_finish")
        if finish and finish.get("display_rgb"):
            if finish.get("quantity_scope")!="existing_substrate_envelope" or finish.get("film_thickness")!="unmeasured":raise ValueError("Uniform appearance must retain substrate-only quantities and explicit unknown film")
            part["surface_appearance"]=deepcopy(finish)


def skirting_joint(previous,point,following,thickness):
    """📐 Construct one shared inferred mitre plane and overlength stocks, keeping cuts away from coincident source end caps."""
    directions=[]
    for a,b in ((previous,point),(point,following)):
        length=math.sqrt(math.fsum((v-u)**2 for u,v in zip(a,b)))
        if length<=0 or not math.isfinite(length):raise ValueError("A timber joint requires two positive finite stock axes")
        directions.append([(v-u)/length for u,v in zip(a,b)])
    normal=[math.fsum(v) for v in zip(*directions)];length=math.sqrt(math.fsum(v*v for v in normal))
    if length==0 or thickness<=0:raise ValueError("A timber joint cannot reverse its stock axis or use a nonpositive section")
    normal=[v/length for v in normal];cosine=max(-1.,min(1.,math.fsum(a*b for a,b in zip(*directions))))
    if cosine<=-1:raise ValueError("A timber joint cannot reverse its stock axis")
    return {"plane":{"normal":normal,"offset":math.fsum(a*b for a,b in zip(normal,point)),"point":deepcopy(point)},"extension":thickness*(1+math.sqrt((1-cosine)/(1+cosine)))}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   