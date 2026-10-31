"""🗺️ Exact finite native coordinate frames, independent of any IFC geometry engine."""
import math

def coordinate_frame(vertices,faces,*,minimum_basis=1e-7):
    """📐 Preserve physical binary64 points through a reversible local power-of-two coordinate map."""
    points=[[float(v) for v in p] for p in vertices]
    if not points or any(len(p)!=3 or not all(math.isfinite(v) for v in p) for p in points) or not math.isfinite(minimum_basis) or minimum_basis<=0:raise ValueError('Native coordinate frames require finite points and a positive basis policy')
    bases=[]
    for face in faces:
        if len(face)<3 or any(not isinstance(i,int) or not 0<=i<len(points) for i in face):raise ValueError('Native faces require valid original point indices')
        origin=points[face[0]];values=[]
        for i in range(1,len(face)-1):
            a,b=[[points[index][k]-origin[k] for k in range(3)] for index in (face[i],face[i+1])]
            values.append(math.hypot(a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]))
        basis=max(values)
        if not math.isfinite(basis) or basis<=0:raise ValueError('Native coordinate frames cannot replace a degenerate face')
        bases.append(basis)
    if not bases:raise ValueError('Native coordinate frames require original faces')
    basis=min(bases);conditioned=basis<minimum_basis
    if not conditioned:return {'conditioned':False,'origin':[0.,0.,0.],'scale':1.,'vertices':points,'minimum_basis':basis}
    exponent=max(0,math.ceil(-math.log2(basis)/2));scale=math.ldexp(1.,exponent);origin=points[0][:]
    for axis in range(3):
        if any((p[axis]-origin[axis])+origin[axis]!=p[axis] for p in points):origin[axis]=0.
    local=[[math.ldexp(value-origin[axis],exponent) for axis,value in enumerate(p)] for p in points]
    if any(not math.isfinite(v) for p in local for v in p):raise ValueError('Native conditioned coordinates must remain finite')
    if any(origin[i]+value/scale!=points[j][i] for j,p in enumerate(local) for i,value in enumerate(p)):raise ValueError('Native conditioning must preserve each original binary64 coordinate')
    return {'conditioned':True,'origin':origin,'scale':scale,'vertices':local,'minimum_basis':basis}
