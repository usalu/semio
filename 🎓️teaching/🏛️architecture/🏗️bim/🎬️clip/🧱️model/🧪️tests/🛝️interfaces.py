"""🛝️ Actual floor and sloping ramp interfaces without duplicate occupied material."""
import importlib,json
from pathlib import Path
import numpy as np
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
def pieces(part):
    """🧩 Decompose only native prismatic profiles, retaining actual convex meshes."""
    if "mesh" in part:return [part]
    return [{**part,"outline":ring} for ring in a._analysis._convex_parts(part["outline"])]
def occupied(model,tag):
    """🏷️ Resolve the actual exclusive physical owner through its existing provider."""
    obj=a._find(model,tag)
    if obj["class"] in ("IfcRamp","IfcRampFlight"):return a._ramp_geometry(model,obj)["parts"]
    if obj["class"]=="IfcWall":return a.path_wall_geometry(model,obj)["parts"]
    return a._solid_geometry(model,obj)["parts"]
def common_union_volume(p,cutters,extra_levels=()):
    """🔬 Integrate independent NumPy edge sections against a GEOS material union with SciPy error bounds."""
    from shapely.geometry import MultiPoint,Polygon
    from shapely.ops import unary_union
    from scipy.integrate import quad
    pv=np.asarray(a.part_vertices(p),dtype=float);active=[]
    for q in cutters:
        qv=np.asarray(a.part_vertices(q),dtype=float)
        if np.all(np.minimum(pv.max(0),qv.max(0))>np.maximum(pv.min(0),qv.min(0))):active.append((qv,a.part_faces(q),q))
    if not active:return 0.
    def slicer(part,vertices,faces):
        if not any(k in part for k in ("mesh","section","top_outline")):
            shape=Polygon(part["outline"],part.get("holes",()))
            return lambda z:shape if part["z0"]<=z<=part["z1"] else Polygon()
        edges=np.asarray(list({tuple(sorted((i,j))) for face in faces for i,j in zip(face,face[1:]+face[:1])}),dtype=int);u,v=vertices[edges[:,0]],vertices[edges[:,1]]
        def at(z):
            crossing=((u[:,2]<z)&(z<v[:,2]))|((v[:,2]<z)&(z<u[:,2]));on=vertices[vertices[:,2]==z,:2];points=np.concatenate([on,(u[crossing]+(v[crossing]-u[crossing])*((z-u[crossing,2])/(v[crossing,2]-u[crossing,2]))[:,None])[:,:2]])
            return MultiPoint(points).convex_hull if len(points) else Polygon()
        return at
    from math import nextafter
    pfaces=a.part_faces(p);source=slicer(p,pv,pfaces);targets=[]
    for vertices,faces,part in active:targets.append(slicer(part,vertices,faces))
    lo,hi=pv[:,2].min(),pv[:,2].max();levels=sorted({lo,hi,*[z for vertices,_,_ in active for z in vertices[:,2] if lo<z<hi],*[z for z in pv[:,2] if lo<z<hi],*[z for z in extra_levels if lo<z<hi]});total=0.;area_bound=float(np.prod(pv.max(0)[:2]-pv.min(0)[:2]))
    for bottom,top in zip(levels,levels[1:]):
        if nextafter(float(bottom),float(top),steps=32)>=top:
            total+=area_bound*(top-bottom)
            continue
        value,error=quad(lambda z:source(z).intersection(unary_union([section(z) for section in targets])).area,bottom,top,epsabs=1e-14,epsrel=1e-10,limit=200);total+=value+abs(error)
    return total

def common_volume(p,q):
    """🧮 Compare two actual native material owners through the independent union integrator."""
    return common_union_volume(p,[q])

def circle_clearance(part,column):
    """⭕ Compare the true analytic disk with the exact XY projection of a convex solid inside its height interval."""
    from shapely.geometry import MultiPoint,Point
    vertices=np.asarray(a.part_vertices(part));lo=max(vertices[:,2].min(),column["z0"]);hi=min(vertices[:,2].max(),column["z1"])
    if hi<=lo:return float("inf")
    points=[p[:2] for p in vertices if lo<=p[2]<=hi];faces=a.part_faces(part);edges={tuple(sorted((i,j))) for face in faces for i,j in zip(face,face[1:]+face[:1])}
    for z in (lo,hi):
        for i,j in edges:
            u,v=vertices[i],vertices[j]
            if (u[2]<z<v[2]) or (v[2]<z<u[2]):points.append((u+(v-u)*(z-u[2])/(v[2]-u[2]))[:2])
    if not points:return float("inf")
    return MultiPoint(points).convex_hull.distance(Point(column["centroid"]))-column["profile"]["diameter"]["value"]/2

def test_floor_and_ramp_interfaces_have_no_duplicate_material():
    """📐 Check both source interfaces against exact native profiles and an independent 3D oracle."""
    from jsonschema import Draft202012Validator
    c=json.loads(Path(__file__).with_suffix(".json").read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(c);m=a.load_model();errors=[]
    def box(bounds):
        lo,hi=bounds;return a._polyhedron_part([[x,y,z] for z in (lo[2],hi[2]) for x,y in ((lo[0],lo[1]),(hi[0],lo[1]),(hi[0],hi[1]),(lo[0],hi[1]))])
    control=c["positive_control"];assert abs(common_volume(box(control["stock"]),box(control["cutter"]))-control["expected"])<control["tolerance"]
    for source,target in c["pairs"]:
        ps=[r for p in occupied(m,source) for r in pieces(p)];qs=[r for p in occupied(m,target) for r in pieces(p)];overlap=sum(common_volume(p,q) for p in ps for q in qs)
        if overlap>c["tolerance"]:errors.append((source,target,overlap))
    assert not errors,errors
                                                                                              

def test_ramp_structural_interfaces_are_exclusive_with_analytic_round_owners():
    """🏛️ Compare real owned RC with unchanged analytic columns and physical beam material."""
    c=json.loads(Path(__file__).with_suffix(".json").read_text());m=a.load_model();errors=[]
    for tag,owners in c["structural_owners"].items():
        obj=a._find(m,tag);assert obj["geometry"]["interface_owners"]==owners
        parts=occupied(m,tag)
        for owner in owners:
            for q in a._interface_owner_parts(m,owner):
                for p in parts:
                    if q.get("profile",{}).get("shape")=="circle":
                        margin=circle_clearance(p,q)
                        if margin<0:errors.append((tag,owner,"analytic circle penetration",margin))
                    else:
                        overlap=common_volume(p,q)
                        if overlap>c["tolerance"]:errors.append((tag,owner,overlap))
    assert not errors,errors


def test_ramp_retains_positive_planar_bearing_after_structural_partition():
    """🧭 Prove native face bearing against original exact owner planes and rigorous publication bounds."""
    import math,itertools
    from fractions import Fraction as Q
    from shapely.geometry import Polygon
    case=json.loads(Path(__file__).with_suffix(".json").read_text());m=a.load_model();rows=[]
    def cross(u,v):return [u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
    for tag,required in case['bearing_owners'].items():
     parts=a.ramp_geometry(tag,m)['parts'];hits=[]
     for owner in a._find(m,tag)['geometry']['interface_owners']:
      obj=a._find(m,owner)
      if obj['class']=='IfcColumn' and a.column_geometry(m,obj)['parts'][0].get('profile',{}).get('shape')=='circle':continue
      for stock in a._interface_reference_parts(m,owner):
       vs=a.part_vertices(stock)
       for face in a.part_faces(stock):
        anchor=vs[face[0]];normal=None
        for i,j in itertools.combinations(face[1:],2):
         n=cross([v-u for u,v in zip(anchor,vs[i])],[v-u for u,v in zip(anchor,vs[j])])
         if any(n):normal=n;break
        if normal is None:raise ValueError('Original owner has a degenerate face')
        offset=sum(n*v for n,v in zip(normal,anchor));axis=max(range(3),key=lambda k:abs(normal[k]));drop=lambda p:[float(p[k]) for k in range(3) if k!=axis];host=Polygon([drop(vs[i]) for i in face])
        for part in parts:
         pv=a.part_vertices(part)
         for pf in a.part_faces(part):
          coords=[pv[i] for i in pf]
          if not all(abs(sum(n*Q(v) for n,v in zip(normal,p))-offset)<=sum(abs(n)*Q(math.ulp(float(v)))/2 for n,v in zip(normal,p)) for p in coords):continue
          shape=Polygon([drop(p) for p in coords])
          if not shape.is_valid:raise ValueError('Published material face must be valid')
          area=host.intersection(shape).area
          if area>0:hits.append({'owner':owner,'projected_area':area,'projection_axis':axis})
     assert any(hit['owner']==required and hit['projected_area']>0 for hit in hits),(tag,required,hits)
     rows.append({'tag':tag,'contacts':hits,'projected_area_sum':sum(h['projected_area'] for h in hits)})
    
    

def test_ramp_wearing_excludes_actual_structural_material_and_analytic_columns():
    """🪜 Preserve actual wearing and bedding around unchanged source structural owners."""
    c=json.loads(Path(__file__).with_suffix(".json").read_text());m=a.load_model();errors=[]
    for tag,owners in c["wearing_owners"].items():
        obj=a._find(m,tag);assert obj["psets"]["Pset_ExactConstruction"]["Owners"]["value"].split(", ")==owners
        parts=occupied(m,tag)
        for owner in owners:
            for q in a._interface_owner_parts(m,owner):
                for p in parts:
                    if q.get("profile",{}).get("shape")=="circle":
                        margin=circle_clearance(p,q)
                        if margin<0:errors.append((tag,owner,"analytic circle penetration",margin))
                    else:
                        overlap=common_volume(p,q)
                        if overlap>c["tolerance"]:errors.append((tag,owner,overlap))
    assert not errors,errors
