"""➖️ Measure authoritative analytic stock differences without tessellation or external runtime geometry."""
import math
from functools import lru_cache
from decimal import Decimal,localcontext
from copy import deepcopy
from fractions import Fraction as Q


def _number(value):
    """🔢 Preserve finite source scalars without treating booleans as geometry."""
    if isinstance(value,bool)or not isinstance(value,(int,float,Q))or not math.isfinite(value):raise ValueError("Analytic solid coordinates must be finite numbers")
    return Q(value)


def _point(value):
    """📍 Validate an original two-dimensional profile placement."""
    if not isinstance(value,list)or len(value)!=2:raise ValueError("Analytic solid points require two source coordinates")
    return tuple(_number(v)for v in value)


def _ring(value):
    """📐 Retain real source profile edges with exact nonzero signed area."""
    if not isinstance(value,list)or len(value)<3:raise ValueError("Analytic solid profiles require occupied rings")
    result=[_point(p)for p in value]
    if not sum(a[0]*b[1]-a[1]*b[0]for a,b in zip(result,result[1:]+result[:1])):raise ValueError("Analytic solid profiles require positive occupied area")
    return result


def _validate(node):
    """🧬 Validate one recursive finite analytic solid without accepting a geometry proxy."""
    if not isinstance(node,dict):raise ValueError("Analytic solid operands require a semantic object")
    if node.get("operator")=="difference":
        if set(node)!={"operator","stock","cutters"}or not node["cutters"]:raise ValueError("A difference requires original stock and explicit cutters")
        _validate(node["stock"])
        for cutter in node["cutters"]:
            if cutter.get("role")=="construction_boundary":
                if set(cutter)!={"operand","role"}:raise ValueError("A geometric boundary cannot invent a physical owner")
            elif set(cutter)!={"operand","owner","owner_guid","role"}or cutter["role"]not in ("material","void_air")or not all(isinstance(cutter[k],str)and cutter[k]for k in ("owner","owner_guid")):raise ValueError("Analytic cutters require physical owner identity and material or reserved-air role")
            _validate(cutter["operand"])
        return
    if node.get("kind")=="polyhedron":
        if set(node)!={"kind","vertices","faces"}:raise ValueError("Finite polyhedra require their original vertices and closed faces")
        _planes(node)
        return
    if node.get("kind")not in ("prism","profile_extrusion")or _number(node["z1"])<=_number(node["z0"]):raise ValueError("An analytic primitive requires positive finite extrusion")
    if node["kind"]=="prism":
        if set(node)!={"kind","outline","holes","z0","z1"}:raise ValueError("An analytic prism requires its original profile and explicit holes")
        _ring(node["outline"])
        for ring in node["holes"]:_ring(ring)
        return
    if set(node)!={"kind","axis","rotation","profile","z0","z1"}:raise ValueError("An analytic profile requires explicit source placement")
    _point(node["axis"]);_number(node["rotation"]);profile=node["profile"]
    if profile["shape"]=="circle":
        if set(profile)!={"shape","radius","voids"}or _number(profile["radius"])<=0:raise ValueError("An analytic circle requires a positive source radius")
    elif profile["shape"]=="ellipse":
        if set(profile)!={"shape","radii","voids"}or len(profile["radii"])!=2 or any(_number(v)<=0 for v in profile["radii"]):raise ValueError("An analytic ellipse requires positive source radii")
    elif profile["shape"]=="rectangle":
        if set(profile)!={"shape","width","depth","voids"}or any(_number(profile[k])<=0 for k in ("width","depth")):raise ValueError("An analytic rectangle requires positive original dimensions")
    elif profile["shape"]=="polygon":
        if set(profile)!={"shape","outline","voids"}:raise ValueError("An analytic polygon requires its original local profile")
        _ring(profile["outline"])
    else:raise ValueError("An analytic profile requires an implemented source curve")
    for bore in profile["voids"]:
        if set(bore)!={"offset","radius"}or _number(bore["radius"])<=0:raise ValueError("An analytic bore requires its original offset and positive radius")
        _point(bore["offset"])


@lru_cache(maxsize=128)
def _plane_kernel(vertices,faces):
    """🔷 Validate exact closed convex finite stock and retain its true source face halfspaces."""
    if len(vertices)<4 or any(len(p)!=3 for p in vertices):raise ValueError("A finite polyhedron requires original three-dimensional vertices")
    edges={};planes=[]
    for face in faces:
        if len(face)<3 or any(isinstance(i,bool)or not isinstance(i,int)or not 0<=i<len(vertices)for i in face):raise ValueError("A finite polyhedron requires valid closed face indices")
        for i,j in zip(face,face[1:]+face[:1]):
            if i==j:raise ValueError("A finite polyhedron face requires real edges")
            key=tuple(sorted((i,j)));edges[key]=edges.get(key,0)+1
        a=vertices[face[0]]
        for index in range(1,len(face)-1):
            u,v=[tuple(vertices[j][k]-a[k]for k in range(3))for j in (face[index],face[index+1])]
            normal=(u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0])
            if any(normal):break
        else:raise ValueError("Finite polyhedron faces require positive source area")
        offset=sum(normal[k]*a[k]for k in range(3));values=[sum(normal[k]*p[k]for k in range(3))-offset for p in vertices]
        if any(values[i]!=0 for i in face):raise ValueError("Finite polyhedron faces must be exactly planar")
        if max(values)>0:
            if min(values)<0:raise ValueError("Finite polyhedron source faces must bound a convex solid")
            normal=tuple(-v for v in normal);offset=-offset
        if not any(values):raise ValueError("Finite polyhedron stock requires positive volume")
        planes.append((normal,offset))
    if len(planes)<4 or any(count!=2 for count in edges.values()):raise ValueError("Finite polyhedron stock must be closed")
    return tuple(planes)


def _planes(node):
    """🗃️ Reuse immutable validated finite-stock planes without retaining mutable input operands."""
    return _plane_kernel(tuple(tuple(_number(v)for v in p)for p in node["vertices"]),tuple(tuple(face)for face in node["faces"]))


def _polyhedron(node,x,z):
    """📏 Intersect true finite-stock face inequalities without replacing a slope by a midpoint prism."""
    low=None;high=None;x,z=Q(x),Q(z)
    for normal,offset in _planes(node):
        value=offset-normal[0]*x-normal[2]*z
        if normal[1]>0:high=value/normal[1]if high is None else min(high,value/normal[1])
        elif normal[1]<0:low=value/normal[1]if low is None else max(low,value/normal[1])
        elif value<0:return []
    if low is None or high is None:raise ValueError("Finite stock requires bounded transverse sections")
    return [(low,high)]if low<high else[]


def validate(part):
    """🛡️ Enforce a single authoritative semantic geometry channel."""
    if any(k in part for k in ("outline","mesh","section","top_outline","holes","z0","z1","radius","profile","display_mesh")):raise ValueError("Display geometry cannot replace or accompany authoritative analytic stock")
    if not all(isinstance(part.get(k),str)and part[k]for k in ("material","role")):raise ValueError("Analytic material requires explicit material and role")
    _validate(part["csg"])


def _merge(intervals):
    """🔗 Form actual occupied intervals without a gap or minimum-width tolerance."""
    result=[]
    for lo,hi in sorted((a,b)for a,b in intervals if b>a):
        if result and lo<=result[-1][1]:result[-1]=(result[-1][0],max(result[-1][1],hi))
        else:result.append((lo,hi))
    return result


def _subtract(stock,cutters):
    """✂️ Retain every strictly positive interval after exact ordered set subtraction."""
    result=[];cutters=_merge(cutters)
    for lo,hi in stock:
        cursor=lo
        for start,end in cutters:
            if end<=cursor or start>=hi:continue
            if start>cursor:result.append((cursor,min(start,hi)))
            cursor=max(cursor,min(end,hi))
        if cursor<hi:result.append((cursor,hi))
    return result


def _polygon(ring,x):
    """📏 Intersect original straight profile edges using exact binary source coordinates."""
    points=_ring(ring);x=Q(x);crossings=[]
    for a,b in zip(points,points[1:]+points[:1]):
        if (a[0]<=x<b[0])or(b[0]<=x<a[0]):crossings.append(a[1]+(b[1]-a[1])*(x-a[0])/(b[0]-a[0]))
    crossings.sort()
    if len(crossings)%2:raise ValueError("Original profile edges require closed occupied intervals")
    return [(a,b)for a,b in zip(crossings[::2],crossings[1::2])if a<b]


def _sqrt(value):
    """🔬 Resolve real dyadic radicands before subtracting narrow analytic chords."""
    with localcontext()as context:
        context.prec=70
        return Q((Decimal(value.numerator)/Decimal(value.denominator)).sqrt())


def _circle(axis,radius,x):
    """⭕ Intersect the real circle equation without a buffered or sampled outline."""
    cx,cy=axis;delta=Q(x)-Q(cx);r=Q(radius)
    if abs(delta)>=r:return []
    half=_sqrt((r-delta)*(r+delta))
    return [(Q(cy)-half,Q(cy)+half)]


def _frame(node):
    """🧭 Retain original analytic source axes and rotate local bore offsets about that axis."""
    angle=math.radians(node["rotation"]);return _point(node["axis"]),Q(math.cos(angle)),Q(math.sin(angle))


def _profile_outline(node):
    """📐 Retain original local rectangle or polygon geometry and its exact source placement frame."""
    axis,c,s=_frame(node);profile=node["profile"]
    if profile["shape"]=="rectangle":
        w,d=Q(profile["width"])/2,Q(profile["depth"])/2;local=[(-w,-d),(w,-d),(w,d),(-w,d)]
    else:local=_ring(profile["outline"])
    return [[axis[0]+c*u-s*v,axis[1]+s*u+c*v]for u,v in local]


def _profile(node,x):
    """🧿 Evaluate circle or ellipse material and its real rotated circular bores."""
    axis,c,s=_frame(node);profile=node["profile"]
    if profile["shape"]in ("rectangle","polygon"):outer=_polygon(_profile_outline(node),x)
    elif profile["shape"]=="circle":outer=_circle(axis,profile["radius"],x)
    else:
        a,b=map(Q,profile["radii"]);dx=Q(x)-axis[0];sx=Q(math.hypot(float(a*c),float(b*s)))
        if abs(dx)>=sx:return []
        middle=dx*c*s*(a*a-b*b)/(sx*sx);half=_sqrt((sx-dx)*(sx+dx))*a*b/(sx*sx)
        outer=[(axis[1]+middle-half,axis[1]+middle+half)]
    holes=[]
    for bore in profile["voids"]:
        u,v=_point(bore["offset"]);holes.extend(_circle((axis[0]+c*u-s*v,axis[1]+s*u+c*v),bore["radius"],x))
    return _subtract(outer,holes)


def intervals(operand,x,z):
    """📊 Expose actual analytic section intervals for semantic differences without geometry flattening."""
    if operand.get("operator")=="difference":return _subtract(intervals(operand["stock"],x,z),[interval for cutter in operand["cutters"]for interval in intervals(cutter["operand"],x,z)])
    if operand.get("kind")=="polyhedron":return _polyhedron(operand,x,z)
    if not Q(operand["z0"])<Q(z)<Q(operand["z1"]):return []
    if operand["kind"]=="profile_extrusion":return _profile(operand,x)
    return _subtract(_polygon(operand["outline"],x),[interval for hole in operand["holes"]for interval in _polygon(hole,x)])


def _leaves(node):
    """🌳 Visit original analytic operands while retaining finite z bands and nested differences."""
    if node.get("operator")=="union":
        for child in node["operands"]:yield from _leaves(child)
    elif node.get("operator")=="difference":
        yield from _leaves(node["stock"])
        for cutter in node["cutters"]:yield from _leaves(cutter["operand"])
    else:yield node


def _support_radius(value):
    """📦 Round a true dyadic support radius outward by an exact comparison, never by a geometric tolerance."""
    root=_sqrt(value)
    if root*root<value:
        with localcontext()as context:
            context.prec=70;decimal=(Decimal(value.numerator)/Decimal(value.denominator)).sqrt();root+=Q(Decimal(1).scaleb(decimal.as_tuple().exponent))
    if root*root<value:raise ArithmeticError("Analytic support radius must conservatively enclose its exact radicand")
    return root


def _xs(node):
    """📐 Retain source vertices and outward true-curve supports as integration datums."""
    if node["kind"]=="polyhedron":return [Q(p[0])for p in node["vertices"]]
    if node["kind"]=="prism":return [Q(p[0])for ring in [node["outline"],*node["holes"]]for p in ring]
    axis,c,s=_frame(node);profile=node["profile"]
    if profile["shape"]in ("rectangle","polygon"):result=[Q(p[0])for p in _profile_outline(node)]
    else:
        radius=Q(profile["radius"])if profile["shape"]=="circle"else _support_radius((Q(profile["radii"][0])*c)**2+(Q(profile["radii"][1])*s)**2)
        result=[axis[0]-radius,axis[0]+radius]
    for bore in profile["voids"]:
        u,v=_point(bore["offset"]);centre=axis[0]+c*u-s*v;r=Q(bore["radius"]);result.extend((centre-r,centre+r))
    return result


def _stock(node):
    """📦 Bound only original occupied stock, never enlarging it to a cutter envelope."""
    return _stock(node["stock"])if node.get("operator")=="difference"else node


def _zs(node):
    """📐 Preserve every original finite-height event for vertical and inclined stock."""
    return [Q(p[2])for p in node["vertices"]]if node["kind"]=="polyhedron"else[Q(node["z0"]),Q(node["z1"])]


def _section_xs(node,z):
    """📍 Retain actual inclined-edge section events before analytic transverse integration."""
    if node["kind"]!="polyhedron":return _xs(node)
    points=[tuple(Q(v)for v in p)for p in node["vertices"]];result=[]
    for i,j in {tuple(sorted((i,j)))for face in node["faces"]for i,j in zip(face,face[1:]+face[:1])}:
        a,b=points[i],points[j]
        if a[2]==z:result.append(a[0])
        if (a[2]<z<b[2])or(b[2]<z<a[2]):result.append(a[0]+(z-a[2])*(b[0]-a[0])/(b[2]-a[2]))
    return result


def _ys(node):
    """📦 Retain conservative original-stock transverse limits, not display tessellation bounds."""
    if node["kind"]=="polyhedron":return [Q(p[1])for p in node["vertices"]]
    if node["kind"]=="prism":return [Q(p[1])for p in node["outline"]]
    axis,c,s=_frame(node);profile=node["profile"]
    if profile["shape"]in ("rectangle","polygon"):return [Q(p[1])for p in _profile_outline(node)]
    radius=Q(profile["radius"])if profile["shape"]=="circle"else _support_radius((Q(profile["radii"][0])*s)**2+(Q(profile["radii"][1])*c)**2)
    return [axis[1]-radius,axis[1]+radius]


def _zlines(leaf,x):
    """📉 Retain all affine vertical endpoints before deciding any occupied interval order."""
    if leaf["kind"]!="polyhedron":return [(Q(0),Q(leaf[k]))for k in ("z0","z1")]
    return [(-n[1]/n[2],(offset-n[0]*x)/n[2])for n,offset in _planes(leaf)if n[2]]


def _zintervals(node,x,y,profiles):
    """📊 Evaluate the real semantic vertical material intervals at a fixed source planar point."""
    if node.get("operator")=="union":return _merge([interval for child in node["operands"]for interval in _zintervals(child,x,y,profiles)])
    if node.get("operator")=="difference":return _subtract(_zintervals(node["stock"],x,y,profiles),[interval for cutter in node["cutters"]for interval in _zintervals(cutter["operand"],x,y,profiles)])
    if node["kind"]!="polyhedron":return [(Q(node["z0"]),Q(node["z1"]))]if any(a<y<b for a,b in profiles[id(node)])else[]
    low=None;high=None
    for normal,offset in _planes(node):
        value=offset-normal[0]*x-normal[1]*y
        if normal[2]>0:high=value/normal[2]if high is None else min(high,value/normal[2])
        elif normal[2]<0:low=value/normal[2]if low is None else max(low,value/normal[2])
        elif value<0:return []
    if low is None or high is None:raise ValueError("Finite analytic stock requires bounded vertical intervals")
    return [(low,high)]if low<high else[]


def _slice(node,leaves,x,low,high):
    """⚖️ Integrate every piecewise-affine source height exactly between real curve and face events."""
    profiles={id(leaf):(_profile(leaf,x)if leaf["kind"]=="profile_extrusion"else _subtract(_polygon(leaf["outline"],x),[interval for hole in leaf["holes"]for interval in _polygon(hole,x)]))for leaf in leaves if leaf["kind"]!="polyhedron"}
    events={low,high};lines=set()
    for leaf in leaves:
        lines.update(_zlines(leaf,x))
        if leaf["kind"]!="polyhedron":events.update(value for interval in profiles[id(leaf)]for value in interval if low<value<high)
        else:
            for n,offset in _planes(leaf):
                if n[1]and not n[2]:
                    value=(offset-n[0]*x)/n[1]
                    if low<value<high:events.add(value)
    ordered=list(lines)
    for index,(slope,intercept)in enumerate(ordered):
        for other,origin in ordered[:index]:
            if slope!=other:
                value=(origin-intercept)/(slope-other)
                if low<value<high:events.add(value)
    events=sorted(events);total=Q(0)
    for a,b in zip(events,events[1:]):
        left=a+(b-a)/3;right=a+2*(b-a)/3
        lengths=[sum(hi-lo for lo,hi in _zintervals(node,x,y,profiles))for y in (left,right)]
        total+=(b-a)*sum(lengths)/2
    return float(total)


def _stocks(node):
    """📦 Retain each original stock when independently measuring a material union."""
    return [stock for child in node["operands"]for stock in _stocks(child)]if node.get("operator")=="union"else[_stock(node)]


def _measure(node,progress=None,cancel=None):
    """⚖️ Measure real curve stock with exact affine height sections and one bounded numerical dimension."""
    node=deepcopy(node);leaves=list(_leaves(node));stocks=_stocks(node);xbound=[x for stock in stocks for x in _xs(stock)];low,high=min(xbound),max(xbound);ybound=[y for stock in stocks for y in _ys(stock)];ylow,yhigh=min(ybound),max(ybound);xs=sorted({low,high,*[v for leaf in leaves for v in _xs(leaf)if low<v<high]});done=0;root=Q(math.sqrt(3/5));nodes=(-root,Q(0),root);weights=(5/9,8/9,5/9)
    def check():
        """🛑 Retain cooperative progress and explicit refinement bounds without altering physical geometry."""
        nonlocal done
        if cancel and cancel():raise InterruptedError("Analytic stock difference measurement cancelled")
        done+=1
        if done>200000:raise ArithmeticError("Analytic solid measure exceeded its explicit refinement budget")
        if progress and(done==1 or done%128==0):progress({"phase":"analytic_sections","completed":done})
    def integrate(function,lo,hi,tolerance,depth=0):
        """🧮 Refine exact-height section integrals while retaining every positive material interval."""
        check();middle=(lo+hi)/2
        def gauss(a,b):
            centre=(a+b)/2;scale=(b-a)/2
            return float(scale)*math.fsum(w*function(centre+scale*t)for t,w in zip(nodes,weights))
        coarse=gauss(lo,hi);left=gauss(lo,middle);right=gauss(middle,hi);fine=math.fsum((left,right));error=abs(fine-coarse)
        if error<=tolerance:return fine,error
        if depth==0:
            half=(hi-lo)/2
            def transformed(t):
                """⭕ Remove support square-root singularities through an exact-curve parameterization."""
                angle=math.pi*float(t)
                return function(middle-half*Q(math.cos(angle)))*float(half)*math.pi*math.sin(angle)
            return integrate(transformed,Q(0),Q(1),tolerance,depth+1)
        if depth>=32:raise ArithmeticError("Analytic section measure did not meet its explicit error estimate tolerance")
        a,ae=integrate(function,lo,middle,tolerance/2,depth+1);b,be=integrate(function,middle,hi,tolerance/2,depth+1)
        return math.fsum((a,b)),math.fsum((ae,be))
    values=[];errors=[];function=lambda x:_slice(node,leaves,x,ylow,yhigh);tolerance=1e-14*max(1.,float(high-low))/max(1,len(xs))
    for lo,hi in zip(xs,xs[1:]):
        value,error=integrate(function,lo,hi,tolerance);values.append(value);errors.append(error)
    if progress:progress({"phase":"analytic_sections","completed":done,"done":True})
    return {"volume":math.fsum(values),"absolute_error":math.fsum(errors),"method":"analytic_sections","integration":"exact_vertical_sections","error_is_estimate":True}


def measure(part,progress=None,cancel=None):
    """⚖️ Measure an authoritative semantic part without importing any external geometry implementation."""
    validate(part);return _measure(part["csg"],progress,cancel)


def measure_operand(node,progress=None,cancel=None):
    """📐 Measure a real source operand independently of material labels or geometry proxies."""
    _validate(node);return _measure(node,progress,cancel)


def measure_union(parts,progress=None,cancel=None):
    """🤝 Integrate actual shared material once across semantic and original native stocks."""
    if not parts:return {"volume":0.,"absolute_error":0.,"method":"analytic_sections","integration":"exact_vertical_sections","error_is_estimate":True}
    nodes=[operand(part)for part in parts]
    for node in nodes:_validate(node)
    return _measure(nodes[0]if len(nodes)==1 else{"operator":"union","operands":nodes},progress,cancel)


def operand(part,vertices=None,faces=None):
    """🧱 Convert an original native part while retaining analytic source profiles and explicit placement axes."""
    if "csg"in part:return deepcopy(part["csg"])
    if part.get("kind")in ("prism","profile_extrusion","polyhedron")and "material"not in part:return deepcopy(part)
    if "mesh"in part:return {"kind":"polyhedron",**deepcopy(part["mesh"])}
    profile=part.get("profile",{})
    if profile.get("shape")in ("circle","ellipse","rectangle","polygon"):
        if "axis"not in part:raise ValueError("An analytic original profile requires its explicit source axis")
        value=lambda q:Q(q["value"]if isinstance(q,dict)else q)
        shape=profile["shape"];native={"shape":shape,"voids":[{"offset":deepcopy(hole["offset"]),"radius":value(hole["diameter"])/2}for hole in profile.get("voids",())]}
        if shape=="circle":native["radius"]=value(profile["diameter"])/2
        elif shape=="ellipse":native["radii"]=[value(profile["width"])/2,value(profile["depth"])/2]
        elif shape=="rectangle":native.update(width=value(profile["width"]),depth=value(profile["depth"]))
        else:native["outline"]=deepcopy(profile["outline"])
        return {"kind":"profile_extrusion","axis":deepcopy(part["axis"]),"rotation":profile.get("rotation",0),"profile":native,"z0":part["z0"],"z1":part["z1"]}
    if "section"in part or "top_outline"in part:
        if vertices is None or faces is None:raise ValueError("An affine native stock requires its original finite topology")
        return {"kind":"polyhedron","vertices":deepcopy(vertices),"faces":deepcopy(faces)}
    return {"kind":"prism","outline":deepcopy(part["outline"]),"holes":deepcopy(part.get("holes",[])),"z0":part["z0"],"z1":part["z1"]}


def bounds(part):
    """📦 Return conservative original-stock bounds without treating a geometric envelope as occupied material."""
    stock=_stock(operand(part));_validate(stock);xs=_xs(stock);ys=_ys(stock);zs=_zs(stock)
    return ((min(xs),max(xs)),(min(ys),max(ys)),(min(zs),max(zs)))


def translate(part,delta):
    """🧭 Transport every original Boolean operand exactly between local and world coordinates."""
    if len(delta)!=3:raise ValueError("Analytic stock translation requires three source offsets")
    dx,dy,dz=map(_number,delta);result=deepcopy(part)
    def move(node):
        """📍 Retain axes, profile frames and physical owner identity through the same coordinate transport."""
        if node.get("operator")=="difference":
            move(node["stock"])
            for cutter in node["cutters"]:move(cutter["operand"])
        elif node["kind"]=="polyhedron":node["vertices"]=[[Q(x)+dx,Q(y)+dy,Q(z)+dz]for x,y,z in node["vertices"]]
        else:
            node["z0"],node["z1"]=Q(node["z0"])+dz,Q(node["z1"])+dz
            if node["kind"]=="profile_extrusion":node["axis"]=[Q(node["axis"][0])+dx,Q(node["axis"][1])+dy]
            else:
                node["outline"]=[[Q(x)+dx,Q(y)+dy]for x,y in node["outline"]];node["holes"]=[[[Q(x)+dx,Q(y)+dy]for x,y in ring]for ring in node["holes"]]
    move(result["csg"]);return result


def _metadata(part):
    """🪪 Retain material and source identity while enforcing one authoritative geometric representation."""
    geometry={"csg","kind","outline","holes","mesh","section","top_outline","z0","z1","axis","profile","radius","area","perimeter","centroid","outer_area","void_area","outer_perimeter","second_moments","curve_error"}
    return {key:deepcopy(value)for key,value in part.items()if key not in geometry}


def _cutter(part):
    """🔗 Keep true physical provenance when provided, without inventing product identity for geometric bounds."""
    if "operand"in part:return deepcopy(part)
    result={"operand":operand(part)}
    if part.get("owner")and part.get("owner_guid"):result.update(owner=part["owner"],owner_guid=part["owner_guid"],role="void_air"if part.get("role")=="void_air"else"material")
    else:result["role"]="construction_boundary"
    return result


def difference(part,cutters):
    """➖ Return original source stock minus real operands as one authoritative semantic part."""
    if not cutters:return deepcopy(part)
    result={**_metadata(part),"csg":{"operator":"difference","stock":operand(part),"cutters":[_cutter(cutter)for cutter in cutters]}};validate(result);return result


def reference_material_parts(part,cutters,bounds=(),progress=None,cancel=None):
    """🤝 Retain exact semantic stock differences and bounded intersections through physical construction."""
    result=[];stock=operand(part)
    for index,boundary in enumerate(bounds or (None,)):
        if cancel and cancel():raise InterruptedError("Analytic reference construction cancelled")
        if progress:progress({"phase":"analytic_reference","completed":index,"total":len(bounds)or 1})
        current=deepcopy(part)
        if boundary is not None:
            outside={"operator":"difference","stock":deepcopy(stock),"cutters":[{"operand":operand(boundary),"role":"construction_boundary"}]}
            current={**_metadata(part),"csg":{"operator":"difference","stock":deepcopy(stock),"cutters":[{"operand":outside,"role":"construction_boundary"}]}}
        result.append(difference(current,cutters))
    return result


def producers():
    """🧬 Bind semantic validation, real curve sections and analytic quantity integration to native revisions."""
    return (validate,measure,measure_operand,measure_union,_measure,_stocks,intervals,operand,bounds,translate,difference,reference_material_parts,_metadata,_cutter,_profile_outline,_support_radius,_number,_point,_ring,_validate,_sqrt,_ys,_zlines,_zintervals,_slice,_planes,_plane_kernel,_polyhedron,_zs,_section_xs,_merge,_subtract,_polygon,_circle,_frame,_profile,_leaves,_xs,_stock,producers)
_BIM_LOADED_MODULE_CODE = __import__("sys")._getframe().f_code
