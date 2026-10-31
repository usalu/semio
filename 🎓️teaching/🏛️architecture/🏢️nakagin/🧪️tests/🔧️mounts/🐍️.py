"""🔧 Physical reference mounts and usable-cavity checks with NumPy and independent mesh oracles."""
import importlib
import importlib.util
import json
from pathlib import Path
import sys
import unittest
from collections import Counter
import numpy as np
import rhino3dm as rh

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
MODEL=ROOT/"🎓️teaching/🏛️architecture/🏢️nakagin"
M=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
C=json.loads((MODEL/"🧪️cases/🔧️mounts/🔣️.json").read_text(encoding="utf8"))
spec=importlib.util.spec_from_file_location("reference_mount_oracles",MODEL/"🧪️tests/🚶️circulation/🐍️.py")
O=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=O
spec.loader.exec_module(O)

def vertex_distance(points,triangles):
    """📐 NumPy point/triangle distances, including face interiors and edge projections."""
    p=np.asarray(points)[:,None,:]
    tri=np.asarray(triangles)
    a,b,c=tri[:,0],tri[:,1],tri[:,2]
    ab,ac=b-a,c-a
    n=np.cross(ab,ac)
    nn=np.sum(n*n,axis=1)
    delta=p-a
    signed=np.sum(delta*n,axis=2)
    projected=delta-signed[:,:,None]*n/np.maximum(nn,1e-30)[None,:,None]
    d00,d01,d11=np.sum(ab*ab,axis=1),np.sum(ab*ac,axis=1),np.sum(ac*ac,axis=1)
    d20,d21=np.sum(projected*ab,axis=2),np.sum(projected*ac,axis=2)
    denom=d00*d11-d01*d01
    u=(d11*d20-d01*d21)/np.maximum(denom,1e-30)
    v=(d00*d21-d01*d20)/np.maximum(denom,1e-30)
    face=np.where((u>=-1e-9)&(v>=-1e-9)&(u+v<=1+1e-9),np.abs(signed)/np.sqrt(np.maximum(nn,1e-30)),np.inf)
    best=float(face.min())
    for x,y in ((a,b),(b,c),(c,a)):
        edge=y-x
        offset=p-x
        factor=np.clip(np.sum(offset*edge,axis=2)/np.maximum(np.sum(edge*edge,axis=1),1e-30),0,1)
        best=min(best,float(np.linalg.norm(offset-factor[:,:,None]*edge,axis=2).min()))
    return best

def edges_cross(edges,triangles):
    """📍 Vectorized segment/triangle intersection independently detects real mounting penetration."""
    edges=np.asarray(edges)
    triangles=np.asarray(triangles)
    origin=edges[:,0,None,:]
    direction=(edges[:,1]-edges[:,0])[:,None,:]
    a,b,c=triangles[:,0],triangles[:,1],triangles[:,2]
    e1,e2=b-a,c-a
    h=np.cross(direction,e2)
    determinant=np.sum(e1*h,axis=2)
    inverse=np.divide(1,determinant,out=np.zeros_like(determinant),where=np.abs(determinant)>1e-12)
    delta=origin-a
    u=np.sum(delta*h,axis=2)*inverse
    q=np.cross(delta,e1)
    v=np.sum(direction*q,axis=2)*inverse
    t=np.sum(e2*q,axis=2)*inverse
    return bool(np.any((np.abs(determinant)>1e-12)&(u>=-1e-8)&(v>=-1e-8)&(u+v<=1+1e-8)&(t>=-1e-8)&(t<=1+1e-8)))

def contact(first,second):
    """🧩 Test actual surface contact/intersection; overlapping bounds alone are insufficient."""
    a,b=np.asarray(first.vertices),np.asarray(second.vertices)
    if np.any(a.max(axis=0)<b.min(axis=0)-C["contact_tolerance"]) or np.any(b.max(axis=0)<a.min(axis=0)-C["contact_tolerance"]):
        return False
    ta,tb=O.triangles_of(first),O.triangles_of(second)
    ea=[(first.vertices[i],first.vertices[j]) for i,j in first.edges]
    eb=[(second.vertices[i],second.vertices[j]) for i,j in second.edges]
    return vertex_distance(a,tb)<=C["contact_tolerance"] or vertex_distance(b,ta)<=C["contact_tolerance"] or edges_cross(ea,tb) or edges_cross(eb,ta)

def rhino_closed(solid):
    """🦏 Independently check the same triangulated material topology with OpenNURBS."""
    mesh=rh.Mesh()
    for point in solid.vertices:
        mesh.Vertices.Add(*point)
    indices={tuple(point):i for i,point in enumerate(solid.vertices)}
    for triangle in O.triangles_of(solid):
        mesh.Faces.AddFace(*(indices[tuple(point)] for point in triangle))
    return mesh.IsClosed

class ReferenceMounts(unittest.TestCase):
    def test_all_named_mounts_join_real_material_after_hand_and_usable_mapping(self):
        for controls in C["controls"]:
            for layout in C["layouts"]:
                with self.subTest(controls=controls,layout=layout):
                    parts={p.key:p for p in M._reference_interior({"key":"A",**controls},layout)}
                    missing=[row["mount"] for row in C["links"] if row["mount"] not in parts]
                    self.assertFalse(missing)
                    for row in C["links"]:
                        for target in row["contacts"]:
                            self.assertTrue(contact(parts[row["mount"]].solid,parts[target].solid),(layout,row["mount"],target))

    def test_mounts_are_closed_positive_and_explicitly_inferred(self):
        parts=[p for p in M._reference_interior(M.capsule_type("A"),"Backslash") if p.key.startswith("fixture-mount-")]
        self.assertGreaterEqual(len(parts),len(C["links"]))
        for part in parts:
            self.assertEqual(part.solid.meta.get("basis"),C["basis"])
            self.assertEqual(part.solid.meta.get("source"),C["source"])
            edges=Counter(tuple(sorted((a,b))) for face in part.solid.faces for a,b in zip(face,face[1:]+face[:1]))
            self.assertTrue(all(n==2 for n in edges.values()),part.key)
            self.assertEqual(rhino_closed(part.solid),all(n==2 for n in edges.values()),part.key)
            triangles=np.asarray(O.triangles_of(part.solid))
            volume=float(np.sum(triangles[:,0]*np.cross(triangles[:,1],triangles[:,2]))/6)
            self.assertGreater(volume,C["minimum_volume"],part.key)

    def test_all_reference_vertices_fit_the_real_cavity_and_clips_remain_closed(self):
        for controls in C["controls"]:
            for layout in C["layouts"]:
                t={"key":"A",**controls}
                profile=M.capsule_interior_profile(t)
                for part in M._reference_interior(t,layout):
                    for (ax,az),(bx,bz) in zip(profile,profile[1:]+profile[:1]):
                        normal=np.asarray((bz-az,ax-bx))
                        points=np.asarray(part.solid.vertices)[:,[0,2]]-np.asarray((ax,az))
                        self.assertLessEqual(float(np.max(points@normal)),C["contact_tolerance"],(layout,part.key))
                    if "cavity_fit" in part.solid.meta:
                        edges=Counter(tuple(sorted((a,b))) for face in part.solid.faces for a,b in zip(face,face[1:]+face[:1]))
                        self.assertTrue(all(n==2 for n in edges.values()),(layout,part.key))
                        self.assertEqual(rhino_closed(part.solid),all(n==2 for n in edges.values()),(layout,part.key))
                        triangles=np.asarray(O.triangles_of(part.solid))
                        volume=float(np.sum(triangles[:,0]*np.cross(triangles[:,1],triangles[:,2]))/6)
                        self.assertGreater(volume,C["minimum_volume"],(layout,part.key))

    def test_reference_material_does_not_fill_actual_lining_corner_space(self):
        parts={p.key:p for p in M._reference_interior(M.capsule_type("A"),"Backslash")}
        for row in C["corner_probes"]:
            self.assertFalse(O.inside_mesh(tuple(row["point"]),O.triangles_of(parts[row["part"]].solid)),row["part"])

if __name__=="__main__":
    unittest.main()
