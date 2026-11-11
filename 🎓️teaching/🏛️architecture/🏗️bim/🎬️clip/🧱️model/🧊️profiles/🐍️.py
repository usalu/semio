"""🕳️ Tool-independent closed cap triangulation of planar polygons with voids."""
from functools import lru_cache
import math


def _area(ring):
    return sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(ring,ring[1:]+ring[:1]))/2


def triangulate(rings):
    """🔺 Return a conforming cap mesh and subdivided physical boundary rings without internal walls."""
    vertices,triangles,boundaries=_triangulate(tuple(tuple(tuple(float(v) for v in p) for p in ring) for ring in rings))
    return {"vertices":[list(p) for p in vertices],"triangles":[list(t) for t in triangles],"boundaries":[list(r) for r in boundaries]}


@lru_cache(maxsize=64)
def _triangulate(raw):
    raw=tuple(tuple(tuple(round(v,12) for v in p) for p in ring) for ring in raw)
    rings=[list(r) if (_area(list(r))>0)==(i==0) else list(reversed(r)) for i,r in enumerate(raw)]
    stations=sorted({round(p[0],12) for ring in rings for p in ring})
    edges=[(a,b) for ring in rings for a,b in zip(ring,ring[1:]+ring[:1])]
    vertices,lookup,triangles=[],{},[]
    def index(p):
        key=tuple(round(v,12) for v in p)
        if key not in lookup:
            lookup[key]=len(vertices);vertices.append(key)
        return lookup[key]
    def y_at(edge,x):
        a,b=edge
        return a[1]+(b[1]-a[1])*(x-a[0])/(b[0]-a[0])
    @lru_cache(maxsize=256)
    def station_points(x):
        values=[]
        for a,b in edges:
            if min(a[0],b[0])-1e-11<=x<=max(a[0],b[0])+1e-11:
                values += [a[1],b[1]] if abs(a[0]-b[0])<1e-13 else [y_at((a,b),x)]
        return sorted({round(y,12) for y in values})
    for a,b in zip(stations,stations[1:]):
        if b-a<1e-12:
            continue
        mid=(a+b)/2
        active=sorted((edge for edge in edges if min(edge[0][0],edge[1][0])<mid<max(edge[0][0],edge[1][0])),key=lambda edge:y_at(edge,mid))
        for lower,upper in zip(active[::2],active[1::2]):
            ya,yb,za,zb=y_at(lower,a),y_at(lower,b),y_at(upper,a),y_at(upper,b)
            polygon=[(a,ya),(b,yb)]+[(b,y) for y in station_points(b) if yb+1e-11<y<zb-1e-11]+[(b,zb),(a,za)]+[(a,y) for y in reversed(station_points(a)) if ya+1e-11<y<za-1e-11]
            ids=[]
            for p in polygon:
                i=index(p)
                if not ids or ids[-1]!=i:
                    ids.append(i)
            if ids[-1]==ids[0]:
                ids.pop()
            centre=index((mid,(y_at(lower,mid)+y_at(upper,mid))/2))
            for i,j in zip(ids,ids[1:]+ids[:1]):
                pa,pb,pc=vertices[centre],vertices[i],vertices[j]
                if abs((pb[0]-pa[0])*(pc[1]-pa[1])-(pb[1]-pa[1])*(pc[0]-pa[0]))>1e-16:
                    triangles.append((centre,i,j))
    boundaries=[]
    for ring in rings:
        ids=[]
        for a,b in zip(ring,ring[1:]+ring[:1]):
            stops=[0.]+[(x-a[0])/(b[0]-a[0]) for x in stations if abs(b[0]-a[0])>1e-13 and min(a[0],b[0])+1e-11<x<max(a[0],b[0])-1e-11]
            for t in sorted(stops):
                ids.append(index((a[0]+t*(b[0]-a[0]),a[1]+t*(b[1]-a[1]))))
        boundaries.append(tuple(ids))
    return tuple(vertices),tuple(triangles),tuple(boundaries)


def circle_envelope(radius: float, maximum_outward: float, safety: float, minimum_sides: int) -> dict:
    """🧿 Bound a solid circular interface conservatively without moving its analytic owner."""
    if not all(math.isfinite(v) for v in (radius,maximum_outward,safety)) or radius<=0 or maximum_outward<=0 or not 0<=safety<maximum_outward or isinstance(minimum_sides,bool) or not isinstance(minimum_sides,int) or minimum_sides<3:
        raise ValueError("A circular interface requires a positive representable precision bound")
    sides = minimum_sides
    while sides<=65536:
        outer = (radius+safety)/math.cos(math.pi/sides)
        if outer-radius<=maximum_outward:break
        sides *= 2
    else:raise ValueError("The circular interface exceeds its finite representation budget")
    points = []
    for i in range(sides):
        c,s = ((1,0),(0,1),(-1,0),(0,-1))[4*i//sides] if (4*i)%sides==0 else (math.cos(2*math.pi*i/sides),math.sin(2*math.pi*i/sides))
        points.append([outer*c,outer*s])
    return {"outline":points,"sides":sides,"radius":radius,"outer_radius":outer,"maximum_deviation":outer-radius,"minimum_radial_clearance":safety}

def hollow_polyline(points: list, diameter: float, wall_thickness: float, segments: int = 24, exact: bool = False) -> dict:
    """🪈️ Construct hollow faceted tube stocks with shared bisector miters and transported circle generators."""
    from fractions import Fraction
    def q(value):
        return value if isinstance(value,Fraction) else Fraction(str(value))
    if len(points)<2 or segments<8 or segments%4 or not 0<wall_thickness<diameter/2:
        raise ValueError('Hollow tube construction requires a finite path, positive gauge and fourfold angular sampling')
    centers = [[q(v) for v in point] for point in points]
    if any(len(point)!=3 for point in centers):raise ValueError('Tube stations require three coordinates')
    def dot(a,b):return sum(x*y for x,y in zip(a,b))
    def cross(a,b):return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
    def unit(vector):
        length = math.sqrt(float(dot(vector,vector)))
        if not math.isfinite(length) or length==0:raise ValueError('Tube axes and miter normals must be finite and nonzero')
        return [v/q(length) for v in vector]
    axes = [[b[k]-a[k] for k in range(3)] for a,b in zip(centers,centers[1:])]
    directions = [unit(axis) for axis in axes]
    normals = [directions[0],*[unit([a[k]+b[k] for k in range(3)]) for a,b in zip(directions,directions[1:])],directions[-1]]
    basis = [Fraction(int(k==min(range(3),key=lambda j:abs(directions[0][j])))) for k in range(3)]
    axis = directions[0];projection = dot(basis,axis)/dot(axis,axis)
    first = unit([basis[k]-axis[k]*projection for k in range(3)]);second = unit(cross(axis,first));radius = q(diameter)/2
    def angle(index):
        quarter = segments//4;quadrant,step = divmod(index,quarter);swap = 2*step>quarter
        reduced = quarter-step if swap else step
        if reduced==0:c,s = Fraction(1),Fraction(0)
        elif 2*reduced==quarter:c = s = q(math.sqrt(.5))
        else:c,s = q(math.cos(2*math.pi*reduced/segments)),q(math.sin(2*math.pi*reduced/segments))
        if swap:c,s = s,c
        return ((c,s),(-s,c),(-c,-s),(s,-c))[quadrant]
    ring = [[centers[0][k]+radius*(first[k]*c+second[k]*s) for k in range(3)] for c,s in map(angle,range(segments))]
    rings = [ring]
    for axis,center,normal in zip(axes,centers[1:],normals[1:]):
        denominator = dot(axis,normal)
        if denominator<=0:raise ValueError('A tube miter must advance along its segment')
        ring = [[p[k]+axis[k]*dot(normal,[center[j]-p[j] for j in range(3)])/denominator for k in range(3)] for p in ring]
        rings.append(ring)
    ratio = (radius-q(wall_thickness))/radius
    inner = [[[c[k]+ratio*(p[k]-c[k]) for k in range(3)] for p in ring] for c,ring in zip(centers,rings)]
    def mesh(a,b):
        n = len(a)
        return {'mesh':{'vertices':a+b,'faces':[list(reversed(range(n))),list(range(n,2*n))]+[[i,(i+1)%n,n+(i+1)%n,n+i] for i in range(n)]}}
    outer = [mesh(a,b) for a,b in zip(rings,rings[1:])];core = [mesh(a,b) for a,b in zip(inner,inner[1:])];metal = []
    for a,b,c,d in zip(rings,rings[1:],inner,inner[1:]):
        for i in range(segments):
            j = (i+1)%segments
            metal.append(mesh([a[i],a[j],c[j],c[i]],[b[i],b[j],d[j],d[i]]))
    caps = [[{'normal':[-v for v in normals[i]],'offset':-dot(normals[i],centers[i]),'point':centers[i]},{'normal':normals[i+1],'offset':dot(normals[i+1],centers[i+1]),'point':centers[i+1]}]for i in range(len(axes))]
    result = {'outer':outer,'core':core,'metal':metal,'rings':rings,'cap_planes':caps}
    def publish(value):
        if isinstance(value,Fraction):return float(value)
        if isinstance(value,list):return [publish(v) for v in value]
        if isinstance(value,dict):return {k:publish(v) for k,v in value.items()}
        return value
    return result if exact else publish(result)

def tube_routes(routes: list, exact: bool = False) -> list:
    """🧭 Resolve declared terminal anchors on native route axes without rounding affine intersections."""
    from fractions import Fraction
    from copy import deepcopy
    records = {r['id']:r for r in routes};resolved = {};visiting = set()
    if len(records)!=len(routes):raise ValueError('Tube route identifiers must be unique')
    def resolve(key):
        if key in resolved:return resolved[key]
        if key in visiting or key not in records:raise ValueError('Tube anchors require an acyclic existing host route')
        visiting.add(key);record = records[key];points = [[v if isinstance(v,Fraction)else Fraction(str(v))for v in p]for p in record['points']]
        if len(points)<2 or any(len(p)!=3 for p in points):raise ValueError('Tube route stations require three coordinates')
        if 'end_anchor'in record:
            anchor = record['end_anchor'];host = resolve(anchor['route']);axis = 'XYZ'.index(anchor['axis'])
            if len(host)!=2:raise ValueError('A tube terminal anchor requires one straight host segment')
            delta = [host[1][k]-host[0][k]for k in range(3)];fixed = [k for k in range(3)if k!=axis]
            station = max(fixed,key=lambda k:abs(delta[k]))
            if delta[station]==0:raise ValueError('A tube terminal cannot locate its host along the constrained axis')
            parameter = (points[-1][station]-host[0][station])/delta[station]
            if not 0<=parameter<=1 or any(points[-1][k]!=host[0][k]+parameter*delta[k]for k in fixed):raise ValueError('A tube terminal must lie on its finite host route')
            points[-1][axis] = host[0][axis]+parameter*delta[axis]
        visiting.remove(key);resolved[key] = points
        return points
    return [{**deepcopy(record),'points':resolve(record['id']) if exact else [[float(v)for v in p]for p in resolve(record['id'])]}for record in routes]
_BIM_LOADED_MODULE_CODE = __import__("sys")._getframe().f_code
