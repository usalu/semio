"""📏 Tool-independent convex occupied-solid distance with explicit closest witnesses."""
from itertools import combinations
from collections import OrderedDict

_CACHE = OrderedDict()
import math


def _dot(a,b):
    return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]


def _solve(matrix,rhs):
    n=len(rhs)
    rows=[list(row)+[value] for row,value in zip(matrix,rhs)]
    scale=max((abs(v) for row in matrix for v in row),default=1.)
    for i in range(n):
        pivot=max(range(i,n),key=lambda k:abs(rows[k][i]))
        if abs(rows[pivot][i])<=max(1e-25,scale*1e-15):
            return None
        rows[i],rows[pivot]=rows[pivot],rows[i]
        divisor=rows[i][i]
        rows[i]=[v/divisor for v in rows[i]]
        for j in range(n):
            if j!=i:
                factor=rows[j][i]
                rows[j]=[a-factor*b for a,b in zip(rows[j],rows[i])]
    return [row[-1] for row in rows]


def _closest(simplex):
    best=None
    for count in range(1,min(4,len(simplex))+1):
        for indices in combinations(range(len(simplex)),count):
            points=[simplex[i] for i in indices]
            origin=points[0][0]
            directions=[[p[0][k]-origin[k] for k in range(3)] for p in points[1:]]
            solution=_solve([[_dot(a,b) for b in directions] for a in directions],[-_dot(v,origin) for v in directions]) if directions else []
            if solution is None:
                continue
            weights=[1-sum(solution),*solution]
            if min(weights)<-1e-10:
                continue
            weights=[max(0.,w) for w in weights]
            total=sum(weights)
            weights=[w/total for w in weights]
            point=[sum(w*p[0][k] for w,p in zip(weights,points)) for k in range(3)]
            squared=_dot(point,point)
            if best is None or squared<best[0]:
                a=[sum(w*p[1][k] for w,p in zip(weights,points)) for k in range(3)]
                b=[sum(w*p[2][k] for w,p in zip(weights,points)) for k in range(3)]
                best=(squared,[p for w,p in zip(weights,points) if w>1e-12],point,a,b)
    return best


def convex_distance(a,b,*,tolerance=1e-9,max_iterations=96,cancelled=None):
    """🧲 Resolve Euclidean distance between two convex hulls, including points and segments."""
    if not a or not b:
        raise ValueError("Convex distance requires nonempty vertex sets")
    def support(direction):
        pa=max(a,key=lambda p:_dot(p,direction))
        pb=min(b,key=lambda p:_dot(p,direction))
        return ([pa[i]-pb[i] for i in range(3)],pa,pb)
    direction=[sum(p[i] for p in a)/len(a)-sum(p[i] for p in b)/len(b) for i in range(3)]
    if _dot(direction,direction)<1e-24:
        direction=[1.,0.,0.]
    best=_closest([support(direction)])
    for iteration in range(max_iterations):
        if cancelled is not None and cancelled():
            raise InterruptedError("Material distance cancelled")
        squared,simplex,point,pa,pb=best
        distance=math.sqrt(max(0.,squared))
        if distance<=tolerance:
            return {"distance":0.,"intersects":True,"a_point":list(pa),"b_point":list(pb)}
        candidate=support([-v for v in point])
        gap=squared-_dot(point,candidate[0])
        if gap<=tolerance*distance or any(math.dist(candidate[0],p[0])<=tolerance*1e-3 for p in simplex):
            return {"distance":distance,"intersects":False,"a_point":list(pa),"b_point":list(pb)}
        best=_closest([*simplex,candidate])
    raise ArithmeticError("Convex material distance did not converge")


def bounds(vertices):
    """📦 Axis-aligned bounds used only as an exact distance lower bound."""
    return tuple(min(p[i] for p in vertices) for i in range(3))+tuple(max(p[i] for p in vertices) for i in range(3))


def bounds_distance(a,b):
    """↔️ Euclidean separation of axis-aligned bounds, never a filled-solid distance."""
    return math.sqrt(sum(max(0.,a[i]-b[i+3],b[i]-a[i+3])**2 for i in range(3)))


def material_distance(a,b,*,cancelled=None):
    """🧱 Minimize over occupied convex components with exact broad-phase pruning."""
    if cancelled is not None and cancelled():
        raise InterruptedError("Material distance cancelled")
    key=(tuple(tuple(tuple(v) for v in p["vertices"]) for p in a),tuple(tuple(tuple(v) for v in p["vertices"]) for p in b))
    if key in _CACHE:
        value=_CACHE.pop(key);_CACHE[key]=value
        return {"distance":value[0],"intersects":value[1],"a_point":list(value[2]),"b_point":list(value[3])}
    pairs=sorted((bounds_distance(pa["bounds"],pb["bounds"]),i,j) for i,pa in enumerate(a) for j,pb in enumerate(b))
    best=None
    for lower,i,j in pairs:
        if best is not None and lower>best["distance"]+1e-10:
            break
        if cancelled is not None and cancelled():
            raise InterruptedError("Material distance cancelled")
        candidate=convex_distance(a[i]["vertices"],b[j]["vertices"],cancelled=cancelled)
        if best is None or candidate["distance"]<best["distance"]:
            best=candidate
        if best["intersects"]:
            break
    if best is None:
        raise ValueError("Material distance requires occupied components")
    _CACHE[key]=(best["distance"],best["intersects"],tuple(best["a_point"]),tuple(best["b_point"]))
    if len(_CACHE)>64:
        _CACHE.popitem(last=False)
    return best
