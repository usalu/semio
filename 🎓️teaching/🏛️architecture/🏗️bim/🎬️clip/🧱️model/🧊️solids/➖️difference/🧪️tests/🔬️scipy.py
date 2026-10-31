"""🔬 Independently integrate semantic CSG with true analytic chords, GEOS slices and SciPy quadrature."""
import json,math
from collections import Counter
from decimal import Decimal,localcontext
from fractions import Fraction
from functools import lru_cache
from scipy.integrate import quad
from shapely.geometry import LineString,Polygon
from shapely.ops import polygonize,unary_union

def _merge(intervals):
    """🧵 Merge only actual overlapping intervals, preserving arbitrarily narrow positive spans."""
    out=[]
    for lo,hi in sorted(intervals):
        if hi<=lo:continue
        if out and lo<=out[-1][1]:out[-1]=(out[-1][0],max(hi,out[-1][1]))
        else:out.append((lo,hi))
    return out

def _subtract(stock,cutters):
    """➖ Apply regularized line-set difference without a geometric tolerance."""
    out=[];cutters=_merge(cutters)
    for lo,hi in stock:
        cursor=lo
        for left,right in cutters:
            if right<=cursor:continue
            if left>=hi:break
            if left>cursor:out.append((cursor,min(left,hi)))
            cursor=max(cursor,right)
            if cursor>=hi:break
        if cursor<hi:out.append((cursor,hi))
    return out

def _intersect(a,b):
    """🤝 Retain all positive common line spans using ordered endpoint comparisons."""
    return _merge((max(lo,left),min(hi,right))for lo,hi in a for left,right in b if min(hi,right)>max(lo,left))

@lru_cache(maxsize=512)
def _polygon(key):
    """🗺️ Prepare the independent polygon oracle with its original finite holes intact."""
    value=json.loads(key);poly=Polygon(value['outline'],value['holes'])
    if not poly.is_valid:raise ValueError('Independent CSG oracle requires a valid polygon, without healing')
    return poly

def _rational(value):
    """🔢 Keep exact integration abscissae instead of rounding them back to world binary64."""
    return value if isinstance(value,Fraction)else Fraction.from_float(float(value))

def _decimal(value):
    """🧮 Preserve exact dyadic source scalars at the high-precision chord boundary."""
    value=_rational(value);return Decimal(value.numerator)/Decimal(value.denominator)

def _circle_interval(axis,radius,x):
    """⭕ Evaluate a true circular chord without cancellation between neighboring analytic boundaries."""
    radicand=_rational(radius)**2-(_rational(x)-_rational(axis[0]))**2
    if radicand<=0:return[]
    with localcontext()as context:
        context.prec=60;half=_decimal(radicand).sqrt();center=_decimal(axis[1]);return[(center-half,center+half)]

def _bores(node):
    """🕳️ Transform each declared eccentric bore around the original profile axis once."""
    angle=math.radians(node['rotation']);cs,sn=math.cos(angle),math.sin(angle);cx,cy=node['axis']
    return[([cx+cs*h['offset'][0]-sn*h['offset'][1],cy+sn*h['offset'][0]+cs*h['offset'][1]],h['radius'])for h in node['profile']['voids']]

def _cross3(a,b):
    """✖️ Preserve finite spatial boundary products in rational arithmetic."""
    return(a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])

@lru_cache(maxsize=128)
def _polyhedron(key):
    """🔺 Validate actual closed oriented planar faces without a convex-hull replacement."""
    node=json.loads(key);points=tuple(tuple(_rational(v)for v in p)for p in node['vertices']);faces=node['faces'];edges=Counter();volume=Fraction(0)
    for face in faces:
        if len(face)<3 or any(type(i)is not int or not 0<=i<len(points)for i in face):raise ValueError('Invalid independent polyhedron face')
        origin=points[face[0]];normal=None
        for i in range(1,len(face)-1):
            a=tuple(points[face[i]][k]-origin[k]for k in range(3));b=tuple(points[face[i+1]][k]-origin[k]for k in range(3));candidate=_cross3(a,b)
            if any(candidate):normal=candidate
            volume+=sum(origin[k]*_cross3(points[face[i]],points[face[i+1]])[k]for k in range(3))/6
        if normal is None or any(sum(normal[k]*(points[i][k]-origin[k])for k in range(3))for i in face):raise ValueError('Nonplanar independent polyhedron face')
        for a,b in zip(face,face[1:]+face[:1]):
            if a==b:raise ValueError('Degenerate independent polyhedron edge')
            edges[(a,b)]+=1
    if any(count!=1 or edges.get((b,a))!=1 for(a,b),count in edges.items())or volume<=0:raise ValueError('Independent polyhedron must be closed, consistently oriented and positive')
    return points,faces,volume

def _z_extent(node):
    """↕️ Read actual finite height authority from either vertices or original extrusion levels."""
    if node['kind']=='polyhedron':return min(v[2]for v in node['vertices']),max(v[2]for v in node['vertices'])
    return node['z0'],node['z1']

@lru_cache(maxsize=512)
def _poly_section(key,z):
    """🧩 Independently polygonize exact spatial face sections, preserving concavity and finite cavities."""
    points,faces,_=_polyhedron(key)
    if not min(v[2]for v in points)<z<max(v[2]for v in points):return Polygon()
    lines=[]
    for face in faces:
        hits=set()
        for i,j in zip(face,face[1:]+face[:1]):
            a,b=points[i],points[j]
            if min(a[2],b[2])<z<max(a[2],b[2]):
                t=(z-a[2])/(b[2]-a[2]);hits.add(tuple(a[k]+t*(b[k]-a[k])for k in range(2)))
        if not hits:continue
        if len(hits)%2:raise ValueError('Odd independent polyhedron face section')
        axis=max(range(2),key=lambda k:max(p[k]for p in hits)-min(p[k]for p in hits));ordered=sorted(hits,key=lambda p:p[axis]);lines.extend(LineString([[float(v)for v in a],[float(v)for v in b]])for a,b in zip(ordered[::2],ordered[1::2])if a!=b)
    cells=list(polygonize(lines))
    if lines and not cells:raise ValueError('Open independent polyhedron section')
    rings=[Polygon(p.exterior)for p in cells];selected=[p for p in cells if sum(r.contains(p.representative_point())for r in rings)%2];result=unary_union(selected)if selected else Polygon()
    if not result.is_valid:raise ValueError('Invalid independent polyhedron section')
    return result

def _rings(geometry):
    """🪆 Expose every actual polygon boundary for independent integration events."""
    if geometry.is_empty:return
    if geometry.geom_type=='Polygon':
        yield list(geometry.exterior.coords)[:-1]
        for ring in geometry.interiors:yield list(ring.coords)[:-1]
    elif hasattr(geometry,'geoms'):
        for child in geometry.geoms:yield from _rings(child)

def _local_profile(node):
    """📏 Read original local profile vertices without registering rounded world corners as authority."""
    p=node['profile']
    if p['shape']=='rectangle':
        w,d=p['width']/2,p['depth']/2;return[[-w,-d],[w,-d],[w,d],[-w,d]]
    return p['outline']

@lru_cache(maxsize=256)
def _profile_polygon(key):
    """🧿 Prepare only the original local rectangle or polygon for the independent GEOS oracle."""
    node=json.loads(key);poly=Polygon(_local_profile(node))
    if not poly.is_valid:raise ValueError('Invalid independent local profile')
    return poly

def _profile_world_x(node):
    """🪁 Propose integration events from the unchanged source frame without replacing its operand."""
    angle=math.radians(node['rotation']);cs,sn=math.cos(angle),math.sin(angle);cx=node['axis'][0]
    return[cx+cs*x-sn*y for x,y in _local_profile(node)]

def _profile_polygon_intervals(node,x):
    """🧭 Intersect a world scanline in the original local frame, preserving eccentric bores separately."""
    poly=_profile_polygon(json.dumps(node,sort_keys=True));angle=math.radians(node['rotation']);cs,sn=math.cos(angle),math.sin(angle);cx,cy=node['axis'];dx=_rational(x)-_rational(cx);radius=max(math.hypot(*v)for v in _local_profile(node))+1.;endpoints=[(float(_rational(cs)*dx+_rational(sn)*_rational(y)),float(-_rational(sn)*dx+_rational(cs)*_rational(y)))for y in(-radius,radius)];cut=poly.intersection(LineString(endpoints))
    def lines(value):
        if value.geom_type in('LineString','LinearRing'):
            ys=[]
            for u,v in value.coords:
                offset=(_rational(u)-_rational(cs)*dx)/_rational(sn)if abs(sn)>abs(cs)else(_rational(v)+_rational(sn)*dx)/_rational(cs);ys.append(_decimal(_rational(cy)+offset))
            if ys:yield min(ys),max(ys)
        elif hasattr(value,'geoms'):
            for child in value.geoms:yield from lines(child)
    return _merge(lines(cut))

def _leaf_bounds(node):
    """📦 Bound actual analytic primitives without treating a display ring as material."""
    if node['kind']=='prism':return _polygon(json.dumps(node,sort_keys=True)).bounds
    if node['kind']=='polyhedron':return(min(v[0]for v in node['vertices']),min(v[1]for v in node['vertices']),max(v[0]for v in node['vertices']),max(v[1]for v in node['vertices']))
    p=node['profile'];cx,cy=node['axis']
    if p['shape']in('rectangle','polygon'):
        radius=math.nextafter(max(math.hypot(*v)for v in _local_profile(node)),math.inf);return(math.nextafter(cx-radius,-math.inf),math.nextafter(cy-radius,-math.inf),math.nextafter(cx+radius,math.inf),math.nextafter(cy+radius,math.inf))
    if p['shape']=='circle':rx=ry=p['radius']
    else:
        a,b=p['radii'];angle=math.radians(node['rotation']);cs,sn=math.cos(angle),math.sin(angle);rx,ry=math.hypot(a*cs,b*sn),math.hypot(a*sn,b*cs)
    return(cx-rx,cy-ry,cx+rx,cy+ry)

def _stock(node):
    """🪵 Resolve the original carrier without erasing any nested semantic cuts."""
    while node.get('operator')=='difference':node=node['stock']
    return node

def _leaves(node):
    """🌳 Preserve every nested primitive used by the exact semantic expression."""
    if node.get('operator')=='difference':
        yield from _leaves(node['stock'])
        for cutter in node['cutters']:yield from _leaves(cutter['operand'])
    else:yield node

def intervals(node,x,z):
    """📍 Slice authoritative material into vertical line intervals at an actual source height."""
    if node.get('operator')=='difference':
        result=intervals(node['stock'],x,z)
        for cutter in node['cutters']:
            if not result:break
            result=_subtract(result,intervals(cutter['operand'],x,z))
        return result
    z0,z1=_z_extent(node)
    if not z0<z<z1:return[]
    if node['kind']in('prism','polyhedron'):
        key=json.dumps(node,sort_keys=True);poly=_polygon(key)if node['kind']=='prism'else _poly_section(key,z)
        if poly.is_empty:return[]
        x0,y0,x1,y1=poly.bounds
        if x<x0 or x>x1:return[]
        cut=poly.intersection(LineString([(x,y0-1),(x,y1+1)]))
        def lines(value):
            if value.geom_type in('LineString','LinearRing'):
                coordinates=list(value.coords)
                if coordinates:yield _decimal(min(p[1]for p in coordinates)),_decimal(max(p[1]for p in coordinates))
            elif hasattr(value,'geoms'):
                for child in value.geoms:yield from lines(child)
        return _merge(lines(cut))
    p=node['profile'];axis=node['axis']
    if p['shape']in('rectangle','polygon'):result=_profile_polygon_intervals(node,x)
    elif p['shape']=='circle':result=_circle_interval(axis,p['radius'],x)
    else:
        rx,ry=p['radii'];angle=math.radians(node['rotation']);cs,sn=math.cos(angle),math.sin(angle);q=_rational;sxx=q(rx)**2*q(cs)**2+q(ry)**2*q(sn)**2;d=q(x)-q(axis[0]);radicand=sxx-d*d
        if radicand<=0:return[]
        with localcontext()as context:
            context.prec=60;center=_decimal(axis[1])+_decimal((q(rx)**2-q(ry)**2)*q(cs)*q(sn)*d/sxx);half=_decimal(rx)*_decimal(ry)*_decimal(radicand).sqrt()/_decimal(sxx);result=[(center-half,center+half)]
    return _subtract(result,[span for center,radius in _bores(node)for span in _circle_interval(center,radius,x)])

def _events(nodes,z=None):
    """🪢 Split integration at primitive extrema and real circle/circle crossings."""
    leaves=[leaf for node in nodes for leaf in _leaves(node)];events=set();circles=[]
    for leaf in leaves:
        bounds=_leaf_bounds(leaf);events.update((bounds[0],bounds[2]))
        if leaf['kind']=='prism':events.update(v[0]for ring in(leaf['outline'],*leaf['holes'])for v in ring)
        elif leaf['kind']=='polyhedron':
            if z is None:events.update(v[0]for v in leaf['vertices'])
            else:events.update(v[0]for ring in _rings(_poly_section(json.dumps(leaf,sort_keys=True),z))for v in ring)
        else:
            if leaf['profile']['shape']=='circle':circles.append((leaf['axis'],leaf['profile']['radius']))
            if leaf['profile']['shape']in('rectangle','polygon'):events.update(_profile_world_x(leaf))
            circles.extend(_bores(leaf))
    for center,radius in circles:events.update((center[0]-radius,center[0]+radius))
    for i,(a,r)in enumerate(circles):
        for b,s in circles[i+1:]:
            dx,dy=b[0]-a[0],b[1]-a[1];distance=math.hypot(dx,dy);feature=distance+abs(r-s)
            if feature>0:
                step=feature
                while step<2*max(r,s):
                    for center,radius in((a,r),(b,s)):
                        for edge in(center[0]-radius,center[0]+radius):events.update((edge-step,edge+step))
                    step*=2
            if distance==0 or distance>r+s or distance<abs(r-s):continue
            station=((r-s)*(r+s)+distance*distance)/(2*distance);radicand=(r-station)*(r+station)
            if radicand<0:continue
            half=math.sqrt(radicand);base=a[0]+station*dx/distance;events.update((base-half*dy/distance,base+half*dy/distance))
    return sorted(events)

def _area(nodes,z,cancel=None,union=False):
    """📈 Independently integrate one exact semantic horizontal section, including inclined finite stocks."""
    bounds=[];active=[]
    for node in nodes:
        stock=_stock(node);low,high=_z_extent(stock)
        if not low<z<high:
            if union:continue
            return 0.,0.
        if stock['kind']=='polyhedron':
            section=_poly_section(json.dumps(stock,sort_keys=True),z)
            if section.is_empty:
                if union:continue
                return 0.,0.
            bounds.append(section.bounds)
        else:bounds.append(_leaf_bounds(stock))
        active.append(node)
    if not active:return 0.,0.
    nodes=active;lo=(min if union else max)(b[0]for b in bounds);hi=(max if union else min)(b[2]for b in bounds)
    if hi<=lo:return 0.,0.
    xs=sorted({lo,hi}|{x for x in _events(nodes,z)if lo<x<hi});values=[];errors=[]
    def length(x):
        if cancel and cancel():raise InterruptedError('Independent analytic CSG integration cancelled')
        result=intervals(nodes[0],x,z)
        for node in nodes[1:]:result=_merge(result+intervals(node,x,z)) if union else _intersect(result,intervals(node,x,z))
        return math.fsum(hi-lo for lo,hi in result)
    for left,right in zip(xs,xs[1:]):
        if right<=left:continue
        origin=_rational(left);width=_rational(right)-origin
        value,error=quad(lambda t:float(width)*length(origin+width*_rational(t)),0.,1.,epsabs=1e-18,epsrel=1e-12,limit=200);values.append(value);errors.append(error)
    return math.fsum(values),math.fsum(errors)

def _measure(nodes,cancel=None,progress=None,union=False):
    """📊 Integrate real Boolean sections through stationary bands or genuinely varying spatial sections."""
    leaves=[leaf for node in nodes for leaf in _leaves(node)];levels=sorted({z for leaf in leaves for z in([v[2]for v in leaf['vertices']]if leaf['kind']=='polyhedron'else _z_extent(leaf))});moving=any(leaf['kind']=='polyhedron'for leaf in leaves);values=[];errors=[]
    for index,(z0,z1)in enumerate(zip(levels,levels[1:])):
        if cancel and cancel():raise InterruptedError('Independent analytic CSG integration cancelled')
        origin=_rational(z0);span=_rational(z1)-origin;height=float(span)
        if progress:progress({'phase':'independent_csg_height','index':index,'total':len(levels)-1})
        if cancel and cancel():raise InterruptedError('Independent analytic CSG integration cancelled')
        if moving:
            inner=[]
            def area(t):
                if cancel and cancel():raise InterruptedError('Independent analytic CSG integration cancelled')
                value,error=_area(nodes,origin+span*_rational(t),cancel,union);inner.append(error);return height*value
            value,error=quad(area,0.,1.,epsabs=1e-13,epsrel=1e-12,limit=200);values.append(value);errors.append(error+height*max(inner,default=0.))
        else:
            value,error=_area(nodes,origin+span/2,cancel,union);values.append(value*height);errors.append(error*height)
    return math.fsum(values),math.fsum(errors)

def volume(operand,cancel=None,progress=None):
    """⚖️ Return independent owned volume and an explicit absolute numerical error estimate."""
    if cancel and cancel():raise InterruptedError('Independent analytic CSG integration cancelled')
    if operand.get('kind')=='polyhedron':
        if progress:progress({'phase':'independent_csg_height','index':0,'total':1})
        if cancel and cancel():raise InterruptedError('Independent analytic CSG integration cancelled')
        exact=_polyhedron(json.dumps(operand,sort_keys=True))[2];value=float(exact);return value,float(abs(Fraction.from_float(value)-exact))
    return _measure([operand],cancel,progress)

def intersection_volume(left,right,cancel=None,progress=None):
    """🔍 Measure actual common semantic material without trusting owner names or GUIDs."""
    return _measure([left,right],cancel,progress)


def union_volume(operands,cancel=None,progress=None):
    """🫧 Integrate actual occupied interval unions across all original finite-height analytic stocks."""
    if cancel and cancel():raise InterruptedError('Independent analytic CSG union cancelled')
    return _measure(operands,cancel,progress,union=True) if operands else (0.,0.)
