"""🕳️ Independent occupied-material proof for fixed rainwater passages through all beam products."""
import importlib
import json
from copy import deepcopy
from pathlib import Path
import unittest
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,MultiPoint,GeometryCollection
from shapely import set_precision
from shapely.strtree import STRtree

clash=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🕳️clashes")
api=clash.api


def horizontal_section(part,z):
    """📐 Slice independent hull edges with a horizontal plane; retain native prism holes."""
    if z<part["z0"]-1e-12 or z>part["z1"]+1e-12:
        return GeometryCollection()
    if not any(k in part for k in ("mesh","section","top_outline")):
        return Polygon(part["outline"],part.get("holes",[]))
    vertices=clash._oracle_vertices(part)
    hull=ConvexHull(vertices)
    points=[]
    edges={tuple(sorted((int(face[i]),int(face[(i+1)%3])))) for face in hull.simplices for i in range(3)}
    for i,j in edges:
        a,b=vertices[i],vertices[j]
        if abs(a[2]-z)<1e-11:
            points.append(a[:2])
        if abs(b[2]-z)<1e-11:
            points.append(b[:2])
        if (a[2]-z)*(b[2]-z)<0:
            points.append((a+(b-a)*(z-a[2])/(b[2]-a[2]))[:2])
    return MultiPoint(points).convex_hull if points else GeometryCollection()



def intersection_volume(a,b):
    """🧪 Integrate independent pairwise GEOS intersections without combining unrelated beam domains."""
    records_a=[clash._oracle_record(json.dumps(p,sort_keys=True)) for p in a]
    records_b=[clash._oracle_record(json.dumps(p,sort_keys=True)) for p in b]
    nodes,weights=np.polynomial.legendre.leggauss(5)
    total=0.
    maximum=0.
    for ra in records_a:
        for rb in records_b:
            if any(ra["bounds"][i+3]<=rb["bounds"][i] or rb["bounds"][i+3]<=ra["bounds"][i] for i in range(3)):
                continue
            lo=max(ra["bounds"][2],rb["bounds"][2])
            hi=min(ra["bounds"][5],rb["bounds"][5])
            projected=ra["polygon"].intersection(rb["polygon"])
            if projected.area<=1e-14:
                continue
            if ra["core"][0]<=lo and ra["core"][1]>=hi and rb["core"][0]<=lo and rb["core"][1]>=hi:
                area=projected.area
                total+=area*(hi-lo)
                maximum=max(maximum,area)
                continue
            levels=sorted({lo,hi}|{float(z) for r in (ra,rb) for z in clash._oracle_vertices(r["part"])[:,2] if lo<z<hi})
            for low,high in zip(levels,levels[1:]):
                for node,weight in zip(nodes,weights):
                    z=(low+high)/2+float(node)*(high-low)/2
                    area=horizontal_section(ra["part"],z).intersection(horizontal_section(rb["part"],z)).area
                    maximum=max(maximum,area)
                    total+=float(weight)*area*(high-low)/2
    return total,maximum


def bounds(parts):
    vertices=np.concatenate([clash._oracle_vertices(p) for p in parts])
    return np.min(vertices,axis=0),np.max(vertices,axis=0)


def candidate_parts(a,b):
    lo,hi=bounds(a)
    return [p for p in b if np.all(bounds([p])[1]>=lo-1e-12) and np.all(bounds([p])[0]<=hi+1e-12)]


class FixedBeamPassageContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        cls.model=api.load_model()
        cls.pipes={tag:api.service_geometry(tag,cls.model)["parts"] for tag in cls.case["pipes"]}
        cls.beams={e["tag"]:api.solid_geometry(e["tag"],cls.model) for e in cls.model["elements"] if e["class"]=="IfcBeam"}

    def test_fixed_profiles_are_exact_native_openings_and_only_four_beams(self):
        elements={e["tag"]:e for e in self.model["elements"]}
        actual={(e["tag"],v["tag"]) for e in self.model["elements"] if e["class"]=="IfcBeam" for v in e["geometry"].get("voids",[]) if v["tag"].startswith("V-DP-")}
        self.assertEqual(actual,{(b,v) for b,v,o in self.case["beam_voids"]})
        for beam,void,opening in self.case["beam_voids"]:
            profile=next(v for v in elements[beam]["geometry"]["voids"] if v["tag"]==void)["outline"]
            self.assertEqual(profile,elements[opening]["geometry"]["outline"])
            points=np.asarray(profile)
            centre=points.mean(axis=0)
            self.assertTrue(np.allclose(np.linalg.norm(points-centre,axis=1),self.case["inferred_reserved_diameter"]/2,atol=1e-12))

    def test_all_beams_including_cellar_are_disjoint_from_actual_pipe_material(self):
        self.assertIn("RIB-CELLAR",self.beams)
        self.assertIn("BEAM-CELLAR-STAIR",self.beams)
        checked=[]
        for pipe,parts in self.pipes.items():
            for tag,geometry in self.beams.items():
                for domain in ("parts","shared_parts","full_parts"):
                    candidates=candidate_parts(parts,geometry.get(domain,[])) if geometry.get(domain) else []
                    if not candidates:
                        continue
                    volume,area=intersection_volume(parts,candidates)
                    self.assertLessEqual(volume,self.case["maximum_material_intersection_volume"],(pipe,tag,domain,volume,area))
                    self.assertLessEqual(area,1e-10,(pipe,tag,domain,area))
                    checked.append((pipe,tag,domain))
        self.assertTrue(checked)

    def test_passages_have_nominal_ten_mm_radial_gap_and_are_fixed_under_pipe_edits(self):
        moved=deepcopy(self.model)
        for e in moved["elements"]:
            if e["tag"] in self.case["pipes"]:
                for p in e["geometry"]["points"]:
                    p[0]+=.4
                e["geometry"].pop("penetrations",None)
                for end in ("clip_start","clip_end"):
                    if end in e["geometry"]["tube"]:
                        e["geometry"]["tube"][end]["point"][0]+=.4
        for beam,void,opening in self.case["beam_voids"]:
            geometry=self.beams[beam]
            self.assertEqual(api.solid_geometry(beam,moved),geometry)
            pipe="DP-1" if "DP-1" in void else "DP-2"
            oracle=clash.independent_distance({"parts":self.pipes[pipe]},{"parts":geometry["full_parts"]})
            self.assertGreaterEqual(oracle["distance"],self.case["minimum_faceted_gap"])
            self.assertLess(oracle["distance"],.0101)

    def test_portable_slice_and_material_volume_oracle_detects_a_plug(self):
        tube=[{"outline":[[-.04,-.04],[.04,-.04],[.04,.04],[-.04,.04]],"holes":[[[-.03,-.03],[.03,-.03],[.03,.03],[-.03,.03]]],"z0":0.,"z1":1.}]
        plug=[{"outline":[[-.1,-.1],[.1,-.1],[.1,.1],[-.1,.1]],"z0":.4,"z1":.6}]
        volume,area=intersection_volume(tube,plug)
        self.assertAlmostEqual(volume,(.08**2-.06**2)*.2,places=12)
        self.assertAlmostEqual(area,.08**2-.06**2,places=12)
        tetra={"mesh":{"vertices":[[0,0,0],[1,0,0],[0,1,0],[0,0,1]],"faces":[[0,2,1],[0,1,3],[0,3,2],[1,2,3]]},"z0":0.,"z1":1.}
        self.assertAlmostEqual(horizontal_section(tetra,.5).area,.125,places=12)
        self.assertAlmostEqual(ConvexHull(np.asarray(tetra["mesh"]["vertices"])).volume,1/6,places=12)

    def test_each_fixed_cut_is_exact_and_bounded_by_its_raw_beam(self):
        raw=deepcopy(self.model)
        for element in raw["elements"]:
            if any(element["tag"]==b for b,v,o in self.case["beam_voids"]):
                element["geometry"].pop("voids")
        original={e["tag"]:e for e in self.model["elements"]}
        for beam,void,opening in self.case["beam_voids"]:
            raw_parts=api.solid_geometry(beam,raw)["full_parts"]
            actual=self.beams[beam]["full_parts"]
            hole=Polygon(original[opening]["geometry"]["outline"])
            levels=sorted({p[k] for p in raw_parts+actual for k in ("z0","z1")})
            for lo,hi in zip(levels,levels[1:]):
                z=(lo+hi)/2
                expected=[set_precision(horizontal_section(p,z),1e-10).difference(set_precision(hole,1e-10)) for p in raw_parts if p["z0"]<z<p["z1"]]
                occupied=[set_precision(horizontal_section(p,z),1e-10) for p in actual if p["z0"]<z<p["z1"]]
                expected=[p for p in expected if not p.is_empty]
                occupied=[p for p in occupied if not p.is_empty]
                for partition in (expected,occupied):
                    tree=STRtree(partition)
                    overlap=sum(p.intersection(partition[int(j)]).area for i,p in enumerate(partition) for j in tree.query(p) if int(j)>i)
                    self.assertLess(overlap,1e-8,(beam,z,"partition overlap"))
                tree=STRtree(expected)
                missing=sum(max(0.,p.area-sum(p.intersection(expected[int(j)]).area for j in tree.query(p))) for p in occupied)
                self.assertLess(missing,1e-8,(beam,z,"outside fixed cut"))
                self.assertAlmostEqual(sum(p.area for p in expected),sum(p.area for p in occupied),delta=1e-8,msg=str((beam,z)))
            self.assertGreaterEqual(min(p["z0"] for p in actual),min(p["z0"] for p in raw_parts)-1e-12)
            self.assertLessEqual(max(p["z1"] for p in actual),max(p["z1"] for p in raw_parts)+1e-12)
                                                                                                                                                          