"""🚧 Occupied material distance and explicit hosted conduit clearance contracts."""
from copy import deepcopy
import importlib
import json
import math
from pathlib import Path
from functools import lru_cache
import unittest

import numpy as np
from scipy.optimize import minimize,LinearConstraint
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,MultiPoint,Point
from shapely.ops import nearest_points,triangulate,unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _convex_oracle(a,b):
    """🧪 Minimize distance through independent SciPy hull halfspaces, returning plain coordinates."""
    a,b=np.asarray(a,float),np.asarray(b,float)
    origin=(a.mean(axis=0)+b.mean(axis=0))/2
    a,b=a-origin,b-origin
    ah,bh=ConvexHull(a),ConvexHull(b)
    matrix=np.zeros((len(ah.equations)+len(bh.equations),6))
    matrix[:len(ah.equations),:3]=ah.equations[:,:3]
    matrix[len(ah.equations):,3:]=bh.equations[:,:3]
    upper=-np.r_[ah.equations[:,3],bh.equations[:,3]]
    def objective(x):
        return np.dot(x[:3]-x[3:],x[:3]-x[3:])
    def jacobian(x):
        return np.r_[2*(x[:3]-x[3:]),2*(x[3:]-x[:3])]
    result=minimize(objective,np.r_[a.mean(axis=0),b.mean(axis=0)],jac=jacobian,constraints=[LinearConstraint(matrix,-np.inf,upper)],method="SLSQP",options={"ftol":1e-14,"maxiter":200})
    if not result.success or np.max(matrix@result.x-upper)>1e-7:
        raise AssertionError("Independent convex distance failed: "+result.message)
    return {"distance":float(np.linalg.norm(result.x[:3]-result.x[3:])),"a_point":(result.x[:3]+origin).tolist(),"b_point":(result.x[3:]+origin).tolist()}


def _oracle_vertices(part):
    if "mesh" in part:
        return np.asarray(part["mesh"]["vertices"],float)
    if "section" in part:
        s=part["section"]
        return np.asarray([[h,t,z] if s["plane"]=="XZ" else [t,h,z] for t in s["sweep"] for h,z in s["profile"]],float)
    return np.asarray([[*p,part["z0"]] for p in part["outline"]]+[[*p,part["z1"]] for p in part.get("top_outline",part["outline"])],float)


@lru_cache(maxsize=1024)
def _oracle_record(encoded):
    part=json.loads(encoded)
    prism=not any(key in part for key in ("mesh","section","top_outline"))
    if prism:
        polygon=Polygon(part["outline"],part.get("holes",[]))
        vertices=None
        equations=None
        interval=(part["z0"],part["z1"])
    else:
        vertices=_oracle_vertices(part)
        polygon=MultiPoint(vertices[:,:2]).convex_hull
        equations=ConvexHull(vertices).equations
        groups={}
        for x,y,z in vertices:
            groups.setdefault((round(float(x),12),round(float(y),12)),[]).append(float(z))
        spans=[(min(groups[(round(x,12),round(y,12))]),max(groups[(round(x,12),round(y,12))])) for x,y in list(polygon.exterior.coords)[:-1]]
        interval=(max(a for a,b in spans),min(b for a,b in spans))
    bounds=(polygon.bounds[0],polygon.bounds[1],part["z0"],polygon.bounds[2],polygon.bounds[3],part["z1"])
    return {"part":part,"prism":prism,"polygon":polygon,"vertices":vertices,"equations":equations,"core":interval,"bounds":bounds}


def _oracle_height(record,point):
    if record["prism"]:
        return record["bounds"][2],record["bounds"][5]
    lo,hi=-math.inf,math.inf
    for nx,ny,nz,offset in record["equations"]:
        remaining=-(nx*point.x+ny*point.y+offset)
        if nz>1e-12:
            hi=min(hi,remaining/nz)
        elif nz< -1e-12:
            lo=max(lo,remaining/nz)
        elif remaining< -1e-8:
            raise AssertionError("Independent projected witness is outside its solid")
    if lo>hi+1e-8:
        raise AssertionError("Independent projected witness has no occupied height")
    return lo,hi


def _oracle_projection_distance(a,b):
    pa,pb=nearest_points(a["polygon"],b["polygon"])
    ia,ib=a["core"],b["core"]
    same_core=max(ia[0],ib[0])<=min(ia[1],ib[1])+1e-12
    contained=(a["prism"] and a["bounds"][2]<=b["bounds"][2] and a["bounds"][5]>=b["bounds"][5]) or (b["prism"] and b["bounds"][2]<=a["bounds"][2] and b["bounds"][5]>=a["bounds"][5])
    if not (a["prism"] and b["prism"]) and not same_core and not contained:
        return None
    alo,ahi=_oracle_height(a,pa)
    blo,bhi=_oracle_height(b,pb)
    if max(alo,blo)<=min(ahi,bhi)+1e-10:
        za=zb=(max(alo,blo)+min(ahi,bhi))/2
    elif ahi<blo:
        za,zb=ahi,blo
    else:
        za,zb=alo,bhi
    ap,bp=[pa.x,pa.y,za],[pb.x,pb.y,zb]
    return {"distance":math.dist(ap,bp),"a_point":ap,"b_point":bp}


@lru_cache(maxsize=256)
def _oracle_convex_pieces(encoded):
    record=_oracle_record(encoded)
    if not record["prism"]:
        return (record["vertices"],)
    polygon=record["polygon"]
    pieces=[]
    for triangle in triangulate(polygon):
        cut=polygon.intersection(triangle)
        for piece in ([cut] if cut.geom_type=="Polygon" else getattr(cut,"geoms",())):
            if piece.geom_type!="Polygon" or piece.area<=1e-14:
                continue
            if piece.symmetric_difference(piece.convex_hull).area>1e-11:
                raise AssertionError("Independent triangulation produced a concave component")
            ring=list(piece.convex_hull.exterior.coords)[:-1]
            pieces.append(np.asarray([[x,y,z] for z in (record["bounds"][2],record["bounds"][5]) for x,y in ring],float))
    union=unary_union([Polygon(p[:len(p)//2,:2]) for p in pieces])
    if union.symmetric_difference(polygon).area>1e-9:
        raise AssertionError("Independent GEOS components do not cover occupied material")
    return tuple(pieces)


def _oracle_bbox_distance(a,b):
    return math.sqrt(sum(max(0.,a[i]-b[i+3],b[i]-a[i+3])**2 for i in range(3)))


@lru_cache(maxsize=128)
def _independent_distance(encoded_a,encoded_b):
    parts_a,parts_b=json.loads(encoded_a),json.loads(encoded_b)
    encoded_parts_a=[json.dumps(p,sort_keys=True) for p in parts_a]
    encoded_parts_b=[json.dumps(p,sort_keys=True) for p in parts_b]
    ra,rb=[_oracle_record(p) for p in encoded_parts_a],[_oracle_record(p) for p in encoded_parts_b]
    candidates=sorted((_oracle_bbox_distance(a["bounds"],b["bounds"]),i,j) for i,a in enumerate(ra) for j,b in enumerate(rb))
    best=None
    for lower,i,j in candidates:
        if best is not None and lower>best["distance"]+1e-9:
            break
        result=_oracle_projection_distance(ra[i],rb[j])
        if result is None:
            ca,cb=_oracle_convex_pieces(encoded_parts_a[i]),_oracle_convex_pieces(encoded_parts_b[j])
            pairs=sorted((_oracle_bbox_distance(tuple(np.min(a,axis=0))+tuple(np.max(a,axis=0)),tuple(np.min(b,axis=0))+tuple(np.max(b,axis=0))),x,y) for x,a in enumerate(ca) for y,b in enumerate(cb))
            result=None
            for bound,x,y in pairs:
                if result is not None and bound>result["distance"]+1e-9 or best is not None and bound>best["distance"]+1e-9:
                    break
                candidate=_convex_oracle(ca[x],cb[y])
                if result is None or candidate["distance"]<result["distance"]:
                    result=candidate
                if result["distance"]<1e-9:
                    break
        if result is not None and (best is None or result["distance"]<best["distance"]):
            best=result
        if best is not None and best["distance"]<1e-9:
            break
    if best is None:
        raise AssertionError("Independent material distance needs occupied parts")
    return best["distance"],tuple(best["a_point"]),tuple(best["b_point"])


def independent_distance(pipe_geometry,column_geometry):
    """🧪 GEOS exact prism/projection distances plus independent SciPy hull QP for genuine3D candidates."""
    value=_independent_distance(json.dumps(pipe_geometry["parts"],sort_keys=True),json.dumps(column_geometry["parts"],sort_keys=True))
    return {"distance":value[0],"intersects":value[0]<=1e-8,"a_point":list(value[1]),"b_point":list(value[2])}


class OccupiedClashContract(unittest.TestCase):
    def setUp(self):
        self.case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))

    def test_convex_distance_matches_independent_scipy_halfspaces(self):
        distance=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.📏️distance.🐍️")
        for case in self.case["convex_cases"]:
            actual=distance.convex_distance(case["a"],case["b"])
            oracle=_convex_oracle(case["a"],case["b"])
            self.assertAlmostEqual(actual["distance"],case["distance"],places=7)
            self.assertAlmostEqual(actual["distance"],oracle["distance"],places=7)
            self.assertEqual(actual["intersects"],case["intersects"])
            self.assertAlmostEqual(math.dist(actual["a_point"],actual["b_point"]),actual["distance"],places=8)

    def test_source_has_explicit_native_host_relations_and_policy(self):
        m=api.load_model()
        hosts={(r["relating"],r["related"][0]) for r in m["relations"] if r.get("connection",{}).get("kind")=="hosted_conduit"}
        self.assertEqual(hosts,{tuple(pair) for pair in self.case["source_hosts"]})
        rule=m["rules"]["clearance"][0]
        self.assertEqual(rule["hosted"]["policy"],"reserved_void")
        self.assertEqual(rule["hosted"]["required"]["value"],self.case["hosted_required"])
        self.assertEqual(rule["required"]["value"],self.case["unhosted_required"])

    def test_source_hosted_conduits_do_not_create_false_clashes(self):
        rows=api.clash_check(include_passed=True)
        self.assertEqual({r["a_tag"] for r in rows},{"DP-1","DP-2","DP-1-OUTLET","DP-2-OUTLET"})
        for row in rows:
            self.assertEqual(row["status"],"ok")
            self.assertEqual(row["policy"],"reserved_void")
            self.assertEqual(row["required"],0)
            self.assertGreater(row["distance"],.009)
            self.assertLess(row["distance"],.011)
            self.assertFalse(row["intersects"])
            self.assertNotIn("clearance",row)
            self.assertAlmostEqual(math.dist(row["a_point"],row["b_point"]),row["distance"],places=7)

    def test_host_relations_do_not_hide_filled_bore_interference(self):
        m=api.load_model()
        next(t for t in m["types"] if t["tag"]=="TY-CIRC33-DRAIN")["profile"].pop("voids")
        for e in m["elements"]:
            if e["tag"] in ("2","4"):
                for section in e["geometry"]["sections"]:
                    section["profile"].pop("voids",None)
        rows=api.clash_check(m)
        actual={(r["a_tag"],r["b_tag"]) for r in rows if r["intersects"]}
        self.assertTrue({("DP-1","2"),("DP-1","2-FIRST"),("DP-2","4"),("DP-2","4-FIRST")}<=actual)

    def test_stale_host_relation_is_invalid_and_work_is_cancellable(self):
        m=api.load_model()
        pipe=next(e for e in m["elements"] if e["tag"]=="DP-1")
        pipe["geometry"].pop("penetrations",None)
        for p in pipe["geometry"]["points"]:
            p[0]+=1
        pipe["geometry"]["tube"]["clip_end"]["point"][0]+=1
        with self.assertRaisesRegex(ValueError,"host"):
            api.clash_check(m)
        with self.assertRaises(InterruptedError):
            api.clash_check(cancelled=lambda:True)

    def test_unhosted_nearby_pipe_still_violates_the_external_clearance(self):
        m=api.load_model()
        moved={"DP-1","DP-1-OUTLET"}
        m["relations"]=[r for r in m["relations"] if not (r.get("connection",{}).get("kind")=="hosted_conduit" and moved.intersection(r["related"]))]
        for e in m["elements"]:
            if e["tag"] not in moved:
                continue
            e["geometry"].pop("penetrations",None)
            for p in e["geometry"]["points"]:
                p[0]+=.3
            for end in ("clip_start","clip_end"):
                if end in e["geometry"]["tube"]:
                    e["geometry"]["tube"][end]["point"][0]+=.3
        row=next(r for r in api.clash_check(m) if r["a_tag"]=="DP-1" and r["b_tag"]=="2")
        self.assertEqual(row["status"],"violation")
        self.assertEqual(row["policy"],"separation")
        self.assertFalse(row["intersects"])
        self.assertEqual(row["required"],.25)
        self.assertAlmostEqual(row["distance"],.095,places=7)

    def test_empty_pipe_lumen_is_not_reported_as_solid_pipe_material(self):
        m=api.load_model()
        column=next(e for e in m["elements"] if e["tag"]=="19")
        column["geometry"]={"axis":[14.25,1.25],"base_level":0,"top_level":2}
        next(t for t in m["types"] if t["tag"]==column["type"])["profile"]={"shape":"circle","diameter":{"value":.02,"unit":"m","basis":"assumed"}}
        row=next(r for r in api.clash_check(m) if r["a_tag"]=="DP-1" and r["b_tag"]=="19")
        self.assertFalse(row["intersects"])
        self.assertGreater(row["distance"],.026)
        pipe=api.service_geometry("DP-1",m)
        structural=api.column_geometry(m,column)
        oracle=independent_distance(pipe,structural)
        self.assertAlmostEqual(row["distance"],oracle["distance"],places=7)

    def test_independent_material_oracle_exercises_genuine_3d_halfspaces(self):
        case=self.case["convex_cases"][1]
        parts=[]
        for vertices in (case["a"],case["b"]):
            parts.append({"parts":[{"mesh":{"vertices":vertices,"faces":[[0,2,1],[0,1,3],[0,3,2],[1,2,3]]},"z0":min(p[2] for p in vertices),"z1":max(p[2] for p in vertices)}]})
        result=independent_distance(*parts)
        self.assertAlmostEqual(result["distance"],case["distance"],places=7)

    def test_progress_result_and_source_schemas_are_explicit(self):
        import jsonschema
        progress=[]
        m=api.load_model()
        rows=api.clash_check(m,include_passed=True,progress=progress.append)
        self.assertEqual(progress[0]["completed"],0)
        self.assertEqual(progress[-1]["completed"],progress[-1]["total"])
        model_schema=json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())
        jsonschema.Draft202012Validator(model_schema).validate(m)
        result_schema=json.loads((Path(__file__).parents[1]/"🧬️schema/🚧️clash.schema.json").read_text())
        for row in rows:
            jsonschema.Draft202012Validator(result_schema).validate(row)
