"""🧩 Exact exposed material regions of oriented closed-cell facets without runtime dependencies."""
from collections import Counter,defaultdict
from copy import deepcopy
from fractions import Fraction
import importlib,json,math
_r=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️regions.🐍️")

class SurfaceCancelled(Exception):
    """🛑 The caller cancelled material-surface preparation."""

def plane_chart(facet):
    """📐 Return the exact supporting plane and cyclic planar coordinates of an authored facet."""
    rings=[tuple(tuple(Fraction(v)for v in p)for p in ring)for ring in facet["rings"]]
    a=rings[0][0];normal=None
    for b,c in zip(rings[0][1:],rings[0][2:]):
        u=tuple(b[i]-a[i]for i in range(3));v=tuple(c[i]-a[i]for i in range(3))
        n=(u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0])
        if any(n):normal=n;break
    if normal is None:raise ValueError("Surface facets require nonzero area")
    drop=max(range(3),key=lambda i:abs(normal[i]));n=tuple(v/normal[drop]for v in normal);offset=sum(n[i]*a[i]for i in range(3))
    if any(sum(n[i]*p[i]for i in range(3))!=offset for ring in rings for p in ring):
        return None,None,None,None
    axes=((drop+1)%3,(drop+2)%3);sign=1 if facet["normal"][drop]>0 else -1
    return (n,offset),axes,(drop,n,offset),sign

def _trace(edges,check):
    """🧵 Trace exact directed material boundaries with disconnected regions and holes."""
    outgoing=defaultdict(set)
    for a,b in edges:outgoing[a].add(b)
    loops=[]
    while edges:
        check();a,b=min(edges);start=a;ring=[a]
        while True:
            edges.remove((a,b));outgoing[a].remove(b);ring.append(b)
            if b==start:break
            candidates=outgoing[b]
            if not candidates:raise ValueError("Open exposed material boundary")
            a,b=b,_r._successor(_r._sub(b,a),b,candidates)
        ring.pop();changed=True
        while changed and len(ring)>3:
            changed=False
            for i,p in enumerate(ring):
                if _r._cross(_r._sub(p,ring[i-1]),_r._sub(ring[(i+1)%len(ring)],p))==0:
                    ring.pop(i);changed=True;break
        loops.append(ring)
    regions=[{"outer":ring,"holes":[]}for ring in loops if _r._area(ring)>0]
    for hole in (ring for ring in loops if _r._area(ring)<0):
        owners=[row for row in regions if _r._contains_ring(hole,row["outer"],check)]
        if not owners:raise ValueError("An exposed material hole has no outer boundary")
        min(owners,key=lambda row:_r._area(row["outer"]))["holes"].append(hole)
    return regions

def _regions(operands,check,notify,others=()):
    """🪢 Cancel tiled edges before exact noding and signed material-side classification."""
    rings=[];weights=Counter();owned_weights=Counter()
    for ring,sign,owned in [*( (ring,sign,True)for ring,sign in operands),*( (ring,sign,False)for ring,sign in others)]:
        check()
        if _r._area(ring)<0:ring=tuple(reversed(ring))
        rings.append((ring,sign,(min(p[0]for p in ring),max(p[0]for p in ring),min(p[1]for p in ring),max(p[1]for p in ring)),owned))
        for a,b in zip(ring,ring[1:]+ring[:1]):
            if a!=b:
                edge=tuple(sorted((a,b)));weight=sign*(1 if a<b else -1);weights[edge]+=weight
                if owned:owned_weights[edge]+=weight
    edges=sorted(edge for edge,weight in weights.items()if weight or owned_weights[edge])
    bounds=[(min(a[0],b[0]),max(a[0],b[0]),min(a[1],b[1]),max(a[1],b[1]))for a,b in edges];nodes=[{a,b}for a,b in edges]
    order=sorted(range(len(edges)),key=lambda i:bounds[i][0])
    for offset,i in enumerate(order):
        check();a,b=edges[i];direction=_r._sub(b,a);box=bounds[i]
        for j in order[offset+1:]:
            other=bounds[j]
            if other[0]>box[1]:break
            if other[2]>box[3]or other[3]<box[2]:continue
            c,d=edges[j];second=_r._sub(d,c);delta=_r._sub(c,a);denominator=_r._cross(direction,second)
            if denominator:
                first=_r._cross(delta,second)/denominator;following=_r._cross(delta,direction)/denominator
                if 0<=first<=1 and 0<=following<=1:
                    p=(a[0]+first*direction[0],a[1]+first*direction[1]);nodes[i].add(p);nodes[j].add(p)
            elif _r._cross(delta,direction)==0:
                for p in (a,b,c,d):
                    if all(max(box[2*k],other[2*k])<=p[k]<=min(box[2*k+1],other[2*k+1])for k in (0,1)):nodes[i].add(p);nodes[j].add(p)
        if offset%128==0:notify("node",offset,len(edges))
    atoms=set()
    for points in nodes:
        points=sorted(points);atoms.update(zip(points,points[1:]))
    boundaries={1:set(),-1:set()}
    for index,(a,b)in enumerate(sorted(atoms)):
        check();mid=((a[0]+b[0])/2,(a[1]+b[1])/2);left=right=total_left=total_right=0
        for ring,sign,box,owned in rings:
            if not(box[0]<=mid[0]<=box[1]and box[2]<=mid[1]<=box[3]):continue
            location=_r._location(mid,ring)
            if location>0:
                total_left+=sign;total_right+=sign
                if owned:left+=sign;right+=sign
            elif location==0:
                for c,d in zip(ring,ring[1:]+ring[:1]):
                    if _r._cross(_r._sub(d,c),_r._sub(mid,c))==0 and all(min(c[k],d[k])<=mid[k]<=max(c[k],d[k])for k in (0,1)):
                        if sum(x*y for x,y in zip(_r._sub(b,a),_r._sub(d,c)))>0:
                            total_left+=sign
                            if owned:left+=sign
                        else:
                            total_right+=sign
                            if owned:right+=sign
                        break
        for sign in (1,-1):
            inside_left=left*sign>0 and total_left*sign>0;inside_right=right*sign>0 and total_right*sign>0
            if inside_left!=inside_right:boundaries[sign].add((a,b)if inside_left else(b,a))
        if index%128==0:notify("classify",index,len(atoms))
    return {sign:_trace(edges,check)for sign,edges in boundaries.items()}

def material_skin(facets,progress=None,cancelled=None):
    """🧱 Return isolated exact coplanar exposed regions while preserving nonplanar source facets."""
    def check():
        """🛑 Observe cancellation during each bounded surface operation."""
        if cancelled and cancelled():raise SurfaceCancelled("Material surface preparation cancelled")
    def notify(phase,completed,total):
        """📣 Publish bounded progress without retaining callers or render objects."""
        if progress:progress({"phase":phase,"completed":completed,"total":total})
        check()
    check();groups={};output=[];facets=tuple(facets)
    for index,facet in enumerate(facets):
        check()
        if any(len(p)!=3 or any(not math.isfinite(v)for v in p)for ring in facet["rings"]for p in ring):raise ValueError("Surface coordinates must be finite spatial points")
        key,axes,plane,sign=plane_chart(facet)
        if key is None:
            output.append(deepcopy(facet));continue
        material=json.dumps(facet["material"],sort_keys=True,separators=(",",":"))
        group=groups.setdefault((material,key),{"axes":axes,"plane":plane,"material":facet["material"],"rings":[]})
        for ri,ring in enumerate(facet["rings"]):
            projected=tuple(tuple(Fraction(p[i])for i in axes)for p in ring)
            group["rings"].append((projected,sign if ri==0 else -sign))
        if index%256==0:notify("facets",index,len(facets))
    planes=defaultdict(list)
    for (material,key),group in groups.items():planes[key].append(group)
    for index,((material,key),group) in enumerate(groups.items()):
        check();drop,n,offset=group["plane"];axes=group["axes"]
        others=[operand for other in planes[key]if other is not group for operand in other["rings"]]
        regions=_regions(group["rings"],check,notify,others)
        def lift(p):
            """📍 Lift an exact planar boundary once into the original supporting plane."""
            q=[Fraction(0)]*3;q[axes[0]],q[axes[1]]=p;q[drop]=(offset-sum(n[i]*q[i]for i in axes))/n[drop]
            return [float(v)for v in q]
        length=math.sqrt(sum(float(v)**2 for v in n))
        for sign,rows in regions.items():
            for row in rows:
                output.append({"rings":[[lift(p)for p in ring]for ring in [row["outer"],*row["holes"]]],"normal":[sign*float(v)/length for v in n],"material":deepcopy(group["material"])})
        notify("planes",index+1,len(groups))
    notify("complete",len(facets),len(facets));return output


def projected_material_regions(facets,matrix,progress=None,cancelled=None):
    """🖋️ Union actual visible facets in an exact view chart, retaining only final material silhouettes."""
    def check():
        """🛑 Observe caller cancellation during view-boundary preparation."""
        if cancelled and cancelled():raise SurfaceCancelled("Projected material boundary cancelled")
    def notify(phase,completed,total):
        """📣 Report bounded exact projected-boundary work."""
        if progress:progress({"phase":phase,"completed":completed,"total":total})
        check()
    check();matrix=tuple(tuple(Fraction(v)for v in row)for row in matrix);groups={}
    if len(matrix)!=3 or any(len(row)!=2 for row in matrix):raise ValueError("Material projection requires a finite three-by-two matrix")
    for index,facet in enumerate(facets):
        check();key=json.dumps(facet["material"],sort_keys=True,separators=(",",":"));group=groups.setdefault(key,{"material":facet["material"],"rings":[],"facet":index})
        for ri,ring in enumerate(facet["rings"]):
            projected=tuple(tuple(sum(Fraction(p[i])*matrix[i][j]for i in range(3))for j in range(2))for p in ring)
            if _r._area(projected):group["rings"].append((projected,1 if ri==0 else -1))
        if index%256==0:notify("project",index,len(facets))
    output=[]
    for group in groups.values():
        for region in _regions(group["rings"],check,notify)[1]:
            output.append({"rings":[[[float(v)for v in p]for p in ring]for ring in [region["outer"],*region["holes"]]],"material":deepcopy(group["material"]),"facet":group["facet"]})
    notify("complete",len(facets),len(facets));return output

def producer_token():
    """🪪 Bind renderer snapshot reuse to every exposed-surface implementation closure."""
    return (producer_token,material_skin,projected_material_regions,plane_chart,_regions,_trace,_r.producer_token())
