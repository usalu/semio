"""🏠 Independent actual roof reserve, wearing and physical ownership contracts."""
import importlib,json
from pathlib import Path
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_roof_wearing_products_partition_actual_reserve_without_plugging_voids():
    """🌧️ Compare physical roof finish partitions with independent GEOS area and volume."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    from jsonschema import Draft202012Validator
    c=json.loads(Path(__file__).with_suffix(".json").read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(c);m=a.load_model();parts=[];volume=0.
    for row in c["floors"]:
        e=a._find(m,row["tag"]);assert e["class"]=="IfcCovering" and e["predefined_type"]=="FLOORING" and e["metadata"]["domains"]==["architecture"]
        g=a.solid_geometry(e["tag"],m);wear=[p for p in g["parts"] if p["role"]=="wearing"];polys=[Polygon(p["outline"]) for p in wear];union=unary_union(polys)
        assert union.area>row["minimum_area"] and abs(sum(p.area for p in polys)-union.area)<1e-7
        assert all(abs(p["z1"]-c["finished"])<1e-8 and p["material"] in row["wearing_materials"] for p in wear)
        assert all(abs(p["z1"]-p["z0"]-.04)<1e-8 for p in wear if p["material"]=="MAT-PAVING")
        independent=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in g["parts"]);assert abs(a.quantities(e["tag"],m)["volume"]-independent)<1e-7;volume+=independent;parts.extend(g["parts"])
    slabs=[a._slab_geometry(m,a._find(m,t)) for t in ("SL-R","SL-KIOSK")];support=unary_union([Polygon(p["outline"]) for g in slabs for p in g["parts"]+g["shared_parts"]])
    wearing=[Polygon(p["outline"]) for p in parts if p["role"]=="wearing"];assert abs(sum(p.area for p in wearing)-unary_union(wearing).area)<1e-7;assert unary_union(wearing).difference(support).area<1e-7
    beam_parts=[p for tag in c["raised_beams"] for p in a._solid_geometry(m,a._find(m,tag),resolve_embedded=False)["parts"]]
    structural=[p for g in slabs for p in g["parts"]+g["shared_parts"]]+beam_parts
    contact_by_cap={z:unary_union([Polygon(q["outline"]) for q in structural+parts if abs(q["z1"]-z)<1e-8]) for z in {p["z0"] for p in parts}}
    for p in parts:
        assert Polygon(p["outline"]).difference(contact_by_cap[p["z0"]]).area<1e-7,(p["role"],p["z0"])
    from shapely.strtree import STRtree
    residual=[p for g in slabs for p in g["finish_parts"]];polygons=[Polygon(q["outline"]) for q in residual]
    invalid=[{**p,"validity":str(poly)} for p,poly in zip(residual,polygons) if not poly.is_valid]
    if invalid:raise AssertionError(("Invalid native residual finish polygons",invalid[:2]))
    tree=STRtree(polygons);double=0.
    for p in parts:
        polygon=Polygon(p["outline"])
        for i in tree.query(polygon,predicate="intersects"):
            q=residual[i];double+=polygon.intersection(polygons[i]).area*max(0,min(p["z1"],q["z1"])-max(p["z0"],q["z0"]))
    assert double<1e-7,double


def test_finish_reserve_adjacent_precision_matches_independent_geos():
    """🧩 Preserve strict owner tolerance when default decimal clipping would manufacture overlap."""
    from shapely.geometry import Polygon
    from copy import deepcopy
    c=json.loads(Path(__file__).with_suffix(".json").read_text())["adjacent_precision"];assert Polygon(c["a"]).intersection(Polygon(c["b"])).area<c["tolerance"]
    m=a.load_model();slab=deepcopy(a._find(m,"SL-R"));slab["geometry"]["finish_voids"]=[]
    for i,ring in enumerate((c["a"],c["b"])):
        e=deepcopy(a._find(m,"FIN-CELLAR-CEMENT"));e.update(tag="PRECISION-"+str(i));e["geometry"]={"kind":"flooring","base_level":0.,"parts":[{"outline":ring,"z0":0.,"z1":.09,"material":"MAT-CEMENT-SAND"}]};m["elements"].append(e);slab["geometry"]["finish_voids"].append({"tag":e["tag"],"owner":e["tag"],"outline":ring})
    remaining,shared=a._finish_reservations(m,slab,[{"outline":[[4.6,.1],[4.9,.1],[4.9,1.3],[4.6,1.3]],"z0":0.,"z1":.09}]);expected=sum(Polygon(c[k]).area*.09 for k in ("a","b"));actual=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in shared);assert abs(expected-actual)<1e-10


def test_finish_owner_is_resolved_once_for_multiple_reference_cells():
    """⚡ Compare cached owner material with independent GEOS while counting producer invocations."""
    from copy import deepcopy
    from unittest.mock import patch
    from shapely.geometry import Polygon
    m=a.load_model();e=deepcopy(a._find(m,"FIN-CELLAR-CEMENT"));e.update(tag="NEUTRAL-ROOF-OWNER");e["geometry"]={"kind":"flooring","base_level":0.,"parts":[{"outline":[[0,0],[1,0],[1,1],[0,1]],"z0":0.,"z1":.09,"material":"MAT-CEMENT-SAND"},{"outline":[[1,0],[2,0],[2,1],[1,1]],"z0":0.,"z1":.09,"material":"MAT-CEMENT-SAND"}]};m["elements"].append(e);slab=deepcopy(a._find(m,"SL-R"));slab["geometry"]["finish_voids"]=[{"tag":"RES-"+str(i),"owner":e["tag"],"outline":p["outline"]} for i,p in enumerate(e["geometry"]["parts"])];parts=[{"outline":[[0,0],[2,0],[2,1],[0,1]],"z0":0.,"z1":.09}]
    with patch.object(a,"_solid_geometry",wraps=a._solid_geometry) as resolve:
        residual,shared=a._finish_reservations(m,slab,parts);assert resolve.call_count==1,resolve.call_count
    assert not residual;assert abs(sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in shared)-.18)<1e-10


def test_finish_reserve_structural_joint_preserves_strict_complete_ownership_without_host_cycle():
    """🏗️ Compare exact concrete/finish joint volumes with GEOS without resolving embedded host references."""
    from copy import deepcopy
    from unittest.mock import patch
    from shapely.geometry import Polygon
    c=json.loads(Path(__file__).with_suffix(".json").read_text())["structural_joint"];m=a.load_model();floor=deepcopy(a._find(m,"FIN-CELLAR-CEMENT"));floor.update(tag=c["reserve"]["owner"],guid=a.global_id(c["reserve"]["owner"]));floor["geometry"]={"kind":"flooring","base_level":0.,"parts":[c["floor_part"]]};beam=deepcopy(a._find(m,"BEAM-PRIMARY-R-34-10"));beam.update(tag=c["reserve"]["joint_owners"][0],guid=a.global_id(c["reserve"]["joint_owners"][0]));beam["geometry"]={"kind":"beam","base_level":0.,"parts":[c["beam_part"]],"embedded_slabs":["SL-R"]};m["elements"].extend([floor,beam]);slab=deepcopy(a._find(m,"SL-R"));slab["geometry"]["finish_voids"]=[c["reserve"]];host=[{"outline":c["reserve"]["outline"],"z0":0.,"z1":.09}]
    with patch.object(a,"_slab_geometry_entry",side_effect=AssertionError("Structural joint must not resolve its embedded host")):
        remaining,shared=a._finish_reservations(m,slab,host)
    assert not remaining;volumes={owner:sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in shared if p["owner"]==owner) for owner in c["expected"]};assert all(abs(volumes[k]-v)<1e-10 for k,v in c["expected"].items())
    floor["geometry"]["parts"][0]=dict(c["floor_part"],z0=.05)
    try:a._finish_reservations(m,slab,host)
    except ValueError:pass
    else:raise AssertionError("Overlapping concrete and finish owners were accepted")


def test_roof_coverings_exclude_actual_exclusive_raised_beam_material():
    """🪵 Integrate true roof layer intersections against independent exclusive structural beam unions."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    c=json.loads(Path(__file__).with_suffix(".json").read_text());m=a.load_model();beams=[p for tag in c["raised_beams"] for p in a._solid_geometry(m,a._find(m,tag),resolve_embedded=False)["parts"]];overlap=0.
    for row in c["floors"]:
        for p in a.solid_geometry(row["tag"],m)["parts"]:
            active=[q for q in beams if q["z0"]<p["z1"] and q["z1"]>p["z0"]];levels=sorted({p["z0"],p["z1"],*(max(p["z0"],min(p["z1"],q[k])) for q in active for k in ("z0","z1"))})
            for lo,hi in zip(levels,levels[1:]):
                z=(lo+hi)/2;body=unary_union([Polygon(r) for q in active if q["z0"]<z<q["z1"] for r in a.part_regions(q,z)]);overlap+=Polygon(p["outline"]).intersection(body).area*(hi-lo)
    assert overlap<1e-8,overlap


def test_sourced_concave_weather_cap_replaces_inferred_rectangular_middle_parapet():
    """🏛️ Preserve the source cap silhouette, its air spaces and actual toe bearing with independent GEOS."""
    from shapely.geometry import Polygon,Point,box
    from shapely.ops import unary_union
    c=json.loads(Path(__file__).with_suffix(".json").read_text())["weather_cap"];m=a.load_model();cap=a._solid_geometry(m,a._find(m,c["tag"]));parapets=[e for e in m["elements"] if e["tag"].startswith(c["parapet_prefix"])];overlap=0.
    for e in parapets:
        x=e["geometry"]["path"][0][0];ys=[p[1] for p in e["geometry"]["path"]];width=e["attributes"]["thickness"]["value"];base=a._z(m,e["geometry"]["base_level"]);top=a._top(m,e["geometry"]["top_level"]);wall=box(x-width/2,base,x+width/2,top)
        for p in cap["parts"]:
            section=p["section"];depth=max(0,min(max(ys),section["sweep"][1])-max(min(ys),section["sweep"][0]));overlap+=wall.intersection(Polygon(section["profile"])).area*depth
        for q in c["air"]:assert not(min(ys)<q[1]<max(ys) and wall.contains(Point(q[0],q[2]))),(e["tag"],"Source concavity/underside filled",q)
    assert overlap<1e-8,overlap;assert abs(max(v[2] for p in cap["parts"] for v in a.part_vertices(p))-c["top"])<1e-8
    q=c["positive"];assert any(p["section"]["sweep"][0]<q[1]<p["section"]["sweep"][1] and Polygon(p["section"]["profile"]).contains(Point(q[0],q[2])) for p in cap["parts"])
    intervals=sorted([sorted(p[1] for p in e["geometry"]["path"]) for e in parapets]);assert intervals==[[c["perimeter_y"][0],c["authority_y"][0]],[c["authority_y"][1],c["perimeter_y"][1]]]
    toe=unary_union([Polygon(r) for p in cap["parts"] for r in a.part_regions(p,c["support_level"]+1e-8)]);slab=a._slab_geometry_entry(m,a._find(m,"SL-R"));support=unary_union([Polygon(r) for p in slab["parts"]+slab["shared_parts"] for r in a.part_regions(p,c["support_level"]-1e-8)]);assert toe.intersection(support).area>c["minimum_support_area"]
