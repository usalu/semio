"""🕳️ Tool-independent closed cap triangulation of planar polygons with voids."""
from functools import lru_cache


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
