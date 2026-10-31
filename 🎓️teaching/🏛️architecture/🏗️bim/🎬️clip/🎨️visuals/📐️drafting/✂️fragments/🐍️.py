"""✂️ Order actual polygon material with iterative source-plane BSP fragmentation and no runtime libraries."""
from dataclasses import dataclass
import math
from pathlib import Path

@dataclass(frozen=True)
class Face:
    """🧩 One original material boundary or its fragment, retaining source and polygon identity."""
    source: int
    polygon: int
    vertices: tuple[tuple[float,float,float],...]
    normal: tuple[float,float,float]

@dataclass(frozen=True)
class _Fragment:
    """🧬 Retain one validated original input plane through every ear and descendant split."""
    face: Face
    plane: tuple[tuple[float,float,float],tuple[float,float,float]]

class FragmentCancelled(Exception):
    """🛑 An explicitly cancelled preparation, partition or traversal operation."""

def source_files():
    """🪪 Return exact implementation and language-independent schema dependencies."""
    return (Path(__file__).resolve(),Path(__file__).with_name('🔣️.schema.json').resolve())

def _dot(a,b):
    """📏 Evaluate a three-dimensional scalar product."""
    return math.fsum(x*y for x,y in zip(a,b))

def _sub(a,b):
    """↔️ Subtract finite spatial coordinates."""
    return tuple(x-y for x,y in zip(a,b))

def _epsilon(points):
    """🔬 Bound local classification roundoff by 64 ulps at the actual coordinate scale."""
    return 64*math.ulp(max((abs(value) for point in points for value in point),default=1.0) or 1.0)

def _turn(a,b,c):
    """📐 Evaluate oriented projected triangle area twice."""
    return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])

def _clean(points,epsilon):
    """🧹 Remove only consecutive numerically identical boundary vertices."""
    result=[]
    for point in points:
        if not result or max(abs(value) for value in _sub(point,result[-1]))>epsilon:
            result.append(point)
    if len(result)>1 and max(abs(value) for value in _sub(result[0],result[-1]))<=epsilon:
        result.pop()
    return tuple(result)

def _cross(a,b):
    """🧭 Construct a spatial cross product from local coordinate differences."""
    return (a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])

def _plane(vertices,supplied,check):
    """📐 Recover a maximum-area local source plane while preserving supplied-normal orientation."""
    origin=vertices[0]
    vectors=tuple(_sub(point,origin) for point in vertices[1:])
    best,size=None,0.0
    for index,first in enumerate(vectors):
        check()
        for second in vectors[index+1:]:
            check()
            candidate=_cross(first,second)
            squared=_dot(candidate,candidate)
            if squared>size:
                best,size=candidate,squared
    if best is None:
        raise ValueError('A source polygon must have positive material area')
    length=math.sqrt(size)
    normal=tuple(value/length for value in best)
    orientation=_dot(normal,supplied)
    if orientation==0:
        raise ValueError('A source normal must orient its material plane')
    if orientation<0:
        normal=tuple(-value for value in normal)
    return origin,normal

def _project(face,normal):
    """📽️ Project to the dominant source-normal coordinate plane without altering winding."""
    axis=max(range(3),key=lambda index:abs(normal[index]))
    return tuple(tuple(value for index,value in enumerate(point) if index!=axis) for point in face.vertices)

def _simple(points,tolerance,epsilon,check):
    """🧷 Reject nonadjacent boundary crossings without inventing material connections."""
    def on(a,b,p):
        """📍 Recognize only roundoff-close actual segment contact."""
        return abs(_turn(a,b,p))<=tolerance and all(min(a[i],b[i])-epsilon<=p[i]<=max(a[i],b[i])+epsilon for i in (0,1))
    for i,a in enumerate(points):
        check()
        b=points[(i+1)%len(points)]
        for j in range(i+1,len(points)):
            check()
            if j==i+1 or (i==0 and j==len(points)-1):
                continue
            c,d=points[j],points[(j+1)%len(points)]
            if (_turn(a,b,c)*_turn(a,b,d)<-tolerance*tolerance and _turn(c,d,a)*_turn(c,d,b)<-tolerance*tolerance) or any((on(a,b,c),on(a,b,d),on(c,d,a),on(c,d,b))):
                return False
    return True

def _prepare(face,check):
    """🔺 Keep convex source material whole and ear-clip concave boundaries without joining holes."""
    vertices=tuple(tuple(float(value) for value in point) for point in face.vertices)
    normal=tuple(float(value) for value in face.normal)
    if len(vertices)<3 or len(normal)!=3 or any(len(point)!=3 for point in vertices) or not all(math.isfinite(value) for point in (*vertices,normal) for value in point) or _dot(normal,normal)==0:
        raise ValueError('Faces require finite planar polygons and a nonzero normal')
    epsilon=_epsilon(vertices)
    vertices=_clean(vertices,epsilon)
    face=Face(face.source,face.polygon,vertices,normal)
    origin,geometric=_plane(vertices,normal,check)
    supplied_length=math.sqrt(_dot(normal,normal))
    if any(abs(_dot(_sub(point,origin),normal))/supplied_length>epsilon for point in vertices):
        raise ValueError('A supplied normal must define the actual source plane')
    if any(abs(_dot(_sub(point,origin),geometric))>epsilon for point in vertices):
        raise ValueError('Source vertices must define an actual planar boundary')
    points=_project(face,geometric)
    span=max(max(point[axis] for point in points)-min(point[axis] for point in points) for axis in (0,1))
    tolerance=epsilon*max(span,epsilon)
    if not _simple(points,tolerance,epsilon,check):
        raise ValueError('A source boundary must be a simple planar polygon')
    signed=math.fsum(_turn(points[0],a,b) for a,b in zip(points,points[1:]+points[:1]))
    if abs(signed)<=tolerance:
        raise ValueError('A source polygon must have positive material area')
    orientation=1 if signed>0 else -1
    if all(orientation*_turn(points[i-1],points[i],points[(i+1)%len(points)])>=-tolerance for i in range(len(points))):
        return [face]
    remaining=list(range(len(points)))
    output=[]
    while len(remaining)>3:
        check()
        found=False
        for offset,index in enumerate(remaining):
            check()
            before,after=remaining[offset-1],remaining[(offset+1)%len(remaining)]
            a,b,c=points[before],points[index],points[after]
            turn=orientation*_turn(a,b,c)
            if abs(turn)<=tolerance:
                remaining.pop(offset)
                found=True
                break
            if turn<0:
                continue
            if any(all(orientation*_turn(u,v,points[other])>=-tolerance for u,v in ((a,b),(b,c),(c,a))) for other in remaining if other not in (before,index,after)):
                continue
            output.append(Face(face.source,face.polygon,(vertices[before],vertices[index],vertices[after]),normal))
            remaining.pop(offset)
            found=True
            break
        if not found:
            raise ValueError('A source boundary must be a simple planar polygon')
    output.append(Face(face.source,face.polygon,tuple(vertices[index] for index in remaining),normal))
    return output

def _split(face,origin,normal,epsilon):
    """⚔️ Split one convex material polygon solely by an actual source face plane."""
    distances=[_dot(_sub(point,origin),normal) for point in face.vertices]
    sides=[1 if value>epsilon else -1 if value<-epsilon else 0 for value in distances]
    if not any(sides):
        return 0,face,None
    if min(sides)>=0:
        return 1,face,None
    if max(sides)<=0:
        return -1,face,None
    front,back=[],[]
    for index,point in enumerate(face.vertices):
        following=(index+1)%len(face.vertices)
        if sides[index]>=0:
            front.append(point)
        if sides[index]<=0:
            back.append(point)
        if sides[index]*sides[following]<0:
            fraction=distances[index]/(distances[index]-distances[following])
            crossing=tuple(a+fraction*(b-a) for a,b in zip(point,face.vertices[following]))
            front.append(crossing)
            back.append(crossing)
    return 2,Face(face.source,face.polygon,_clean(front,epsilon),face.normal),Face(face.source,face.polygon,_clean(back,epsilon),face.normal)

def ordered_fragments(faces,towards,progress=None,cancelled=None):
    """👁️ Preserve polygon material and return deterministic far-to-near orthographic BSP fragments."""
    def check():
        """🛑 Observe explicit cancellation at bounded units of geometric work."""
        if cancelled and cancelled():
            raise FragmentCancelled('Fragment ordering cancelled')
    def notify(phase,completed,total):
        """📣 Publish phase-local progress without hiding total changes caused by splits."""
        if progress:
            progress({'phase':phase,'completed':completed,'total':total})
        check()
    check()
    towards=tuple(float(value) for value in towards)
    if len(towards)!=3 or not all(math.isfinite(value) for value in towards) or _dot(towards,towards)==0:
        raise ValueError('An orthographic camera requires a finite nonzero direction')
    faces=tuple(faces)
    prepared=[]
    for index,face in enumerate(faces):
        check()
        material=_prepare(face,check)
        length=math.hypot(*face.normal)
        plane=(tuple(float(value) for value in face.vertices[0]),tuple(value/length for value in face.normal))
        prepared.extend(_Fragment(row,plane) for row in material)
        notify('prepare',index+1,len(faces))
    nodes=[]
    work=[(None,None,prepared)] if prepared else []
    while work:
        check()
        parent,side,group=work.pop()
        first=group[0]
        origin,normal=first.plane
        epsilon=_epsilon(point for fragment in group for point in fragment.face.vertices)
        index=len(nodes)
        node={'origin':origin,'normal':normal,'same':[],'front':None,'back':None}
        nodes.append(node)
        if parent is not None:
            nodes[parent][side]=index
        front,back=[],[]
        for fragment in group:
            check()
            relation,one,two=_split(fragment.face,origin,normal,epsilon)
            if fragment is first and relation!=0:
                raise ArithmeticError('A splitter fragment must remain on its validated original plane')
            if relation==0:
                node['same'].append(one)
            elif relation==1:
                front.append(fragment)
            elif relation==-1:
                back.append(fragment)
            else:
                front.append(_Fragment(one,fragment.plane))
                back.append(_Fragment(two,fragment.plane))
        node['same'].sort(key=lambda face:(face.source,face.polygon,face.vertices,face.normal))
        if front:
            work.append((index,'front',front))
        if back:
            work.append((index,'back',back))
        notify('partition',len(nodes),len(nodes)+len(work))
    ordered=[]
    work=[(0,False)] if nodes else []
    while work:
        check()
        index,emit=work.pop()
        node=nodes[index]
        if emit:
            ordered.extend(node['same'])
            notify('order',len(ordered),len(ordered)+len(work))
            continue
        far,near=('back','front') if _dot(node['normal'],towards)>=0 else ('front','back')
        if node[near] is not None:
            work.append((node[near],False))
        work.append((index,True))
        if node[far] is not None:
            work.append((node[far],False))
    notify('complete',len(ordered),len(ordered))
    return tuple(ordered)
