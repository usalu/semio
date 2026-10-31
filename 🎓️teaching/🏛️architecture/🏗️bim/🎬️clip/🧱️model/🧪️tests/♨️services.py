"""♨️ Original service products and source-qualified logical topology acceptance."""
import importlib,json
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_original_services_source_topology_matches_independent_sparse_graph():
    """🕸️ Compare owned reachability with SciPy without inventing unregistered metric pipe routing."""
    from scipy.sparse import csr_matrix
    from scipy.sparse.csgraph import breadth_first_order
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());graph=api.service_topology(source)
    ids=[n["id"] for n in graph["nodes"]];index={tag:i for i,tag in enumerate(ids)}
    rows=[index[e["from"]] for e in graph["edges"]];cols=[index[e["to"]] for e in graph["edges"]]
    matrix=csr_matrix(([1]*len(rows),(rows,cols)),shape=(len(ids),len(ids)))
    for row in case["reachability"]:
        actual=api.service_reachable(row["start"],source)
        oracle=sorted(ids[i] for i in breadth_first_order(matrix,index[row["start"]],directed=True,return_predecessors=False))
        assert actual==oracle and set(row["includes"])<=set(actual)
    assert all(e["metric_status"]=="unknown" for e in graph["edges"])

def test_original_services_native_identities_and_attested_radiator_section_feet():
    """📜 Keep original executed identities distinct from inferred fabrication and radiator counts."""
    from jsonschema import Draft202012Validator
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    Draft202012Validator(json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())).validate(source)
    for row in case["products"]:
        obj=api._find(source,row["tag"]);assert obj["guid"]==api.global_id(row["tag"])
        assert obj["class"]==row["class"] and obj["system"]==row["system"]
        assert obj["historical_evidence"]["status"]=="inferred_reconstruction"
        if "sections" in row:
            assert obj["attributes"]["section_count"]=={**obj["attributes"]["section_count"],"value":row["sections"],"basis":row["sections_basis"]}
            assert obj["attributes"]["foot_count"]["value"]==row["feet"]
            assert obj["attributes"]["foot_count"]["basis"]==row["feet_basis"]
            assert len({p["role"] for p in obj["geometry"]["parts"] if p["role"].startswith("radiator_section_")})==row["sections"]
            assert sum(p["role"]=="radiator_foot" for p in obj["geometry"]["parts"])==row["feet"]


def test_original_services_phase_one_source_schema_and_ownership():
    """🕳️ Require real source void owners and canonical identities for the bounded original pipe batch."""
    from jsonschema import Draft202012Validator
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    Draft202012Validator(json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())).validate(source)
    for row in case["products"][:5]:
        obj=api._find(source,row["tag"]);assert obj["guid"]==api.global_id(row["tag"])
        assert obj["class"]==row["class"] and obj["system"]==row["system"]
    for row in case["openings"]:
        opening=api._find(source,row["tag"]);assert opening["geometry"]["slab"]==row["host"]
        owners=[r for r in source["relations"] if r["class"]=="IfcRelVoidsElement" and row["tag"] in r["related"]]
        assert len(owners)==1 and owners[0]["relating"]==row["host"]


def test_native_pipe_geometry_variants_reject_hybrid_with_same_schema_outcome():
    """🧬 Keep straight hollow tubes and owned composite pipe/casing bodies mutually exclusive."""
    from copy import deepcopy
    from jsonschema import Draft202012Validator
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());schema=json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text());validator=Draft202012Validator({"$ref":"#/$defs/element","$defs":schema["$defs"]})
    for row in case["geometry_variants"]:
        sample=deepcopy(source);obj=api._find(sample,row["tag"]);obj["geometry"].update(row.get("add_geometry",{}))
        assert validator.is_valid(obj)==row["valid"]
        if not row["valid"]:
            try:api.service_geometry(row["tag"],sample)
            except ValueError:pass
            else:raise AssertionError("Native composite service accepted a hybrid source envelope")


def test_original_pipe_batch_material_volumes_cavities_and_actual_steel_contact():
    """📐 Independently check convex material, real pipe airways and positive-contact nonpenetrating clips."""
    from scipy.spatial import ConvexHull
    from scipy.optimize import linprog
    from shapely.geometry import Polygon,Point
    from shapely import constrained_delaunay_triangles
    source=api.load_model();tags=["RAIN-KITCHEN-PATIO-9","RAIN-KITCHEN-PATIO-9-COVER","COLLECTOR-KITCHEN-PATIO-9","HEAT-RISER-VESTIBULE-5-SUPPLY","HEAT-RISER-VESTIBULE-5-RETURN","HEAT-RISER-VESTIBULE-5-MOUNTS"];bodies={}
    for tag in tags:
        obj=api._find(source,tag);g=api.solid_geometry(tag,source) if obj["class"]=="IfcBuildingElementProxy" else api.service_geometry(tag,source);bodies[tag]=g
        expected=sum(ConvexHull(api.part_vertices(p)).volume for p in g["parts"])
        assert abs(expected-api.quantities(tag,source)["volume"])<1e-7,(tag,expected,api.quantities(tag,source)["volume"])
    def contains(part,point):
        return part["z0"]-1e-8<=point[2]<=part["z1"]+1e-8 and Polygon(part["outline"]).buffer(1e-9).covers(Point(point[:2])) if "mesh" not in part and "section" not in part and "top_outline" not in part else all(sum(eq[k]*point[k] for k in range(3))+eq[3]<1e-8 for eq in ConvexHull(api.part_vertices(part)).equations)
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    for point in case["rain_airway"]:
        assert not any(contains(p,point) for p in bodies["RAIN-KITCHEN-PATIO-9"]["parts"]),"Real continuous rain9 airway"
    curtain=api.filling_geometry("CW-1",source);host=curtain["parts"]
    convex_host=[{"outline":[list(xy) for xy in t.exterior.coords[:-1]],"z0":p["z0"],"z1":p["z1"]} for p in host for t in constrained_delaunay_triangles(Polygon(p["outline"])).geoms]
    for tag in ("RAIN-KITCHEN-PATIO-9-COVER","HEAT-RISER-VESTIBULE-5-MOUNTS"):
        point=api._find(source,tag)["geometry"]["mounting"]["point"]
        assert any(contains(p,point) for p in host) and any(contains(p,point) for p in bodies[tag]["parts"]),(tag,"native steel attachment")
    for tag,g in bodies.items():
        if tag=="COLLECTOR-KITCHEN-PATIO-9":continue
        for part in g["parts"]:
            hull=ConvexHull(api.part_vertices(part));pv=hull.points
            for solid in convex_host:
                vertices=api.part_vertices(solid)
                if any(min(max(v[k] for v in vertices),max(v[k] for v in pv))-max(min(v[k] for v in vertices),min(v[k] for v in pv))<1e-8 for k in range(3)):continue
                eqs=list(hull.equations)+list(ConvexHull(vertices).equations)
                fit=linprog([0,0,0,-1],A_ub=[list(e[:3])+[1] for e in eqs],b_ub=[-e[3] for e in eqs],bounds=[(None,None)]*3+[(0,None)],method="highs")
                assert not fit.success or fit.x[3]<1e-8,(tag,"curtain material penetration",fit.x[3])
                               

def test_actual_service_bores_remove_only_supported_slab_and_patio_finish_material():
    """🧮 Compare actual net bore ownership with independent GEOS clipped material integration."""
    from copy import deepcopy
    from shapely.geometry import Polygon,Point
    from shapely.ops import unary_union
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());prior=deepcopy(source);tags={r["tag"] for r in case["openings"]}
    bore=unary_union([Polygon(api._expand(source,api._find(source,t)["geometry"]["outline"])) for t in tags])
    prior["elements"]=[e for e in prior["elements"] if e["tag"] not in tags]
    before=api._slab_geometry(prior,api._find(prior,"SL-1"));after=api._slab_geometry(source,api._find(source,"SL-1"));levels=sorted({p[k] for p in before["parts"] for k in ("z0","z1")});expected=0.
    for lo,hi in zip(levels,levels[1:]):
        mid=(lo+hi)/2;section=unary_union([Polygon(r) for p in before["parts"] if p["z0"]<mid<p["z1"] for r in api.part_regions(p,mid)])
        expected+=section.intersection(bore).area*(hi-lo)
    removed=api._occupied_volume(before["parts"])-api._occupied_volume(after["parts"])
    assert expected>0 and abs(removed-expected)<1e-7,(removed,expected)
    for tag in tags:
        ring=Polygon(api._expand(source,api._find(source,tag)["geometry"]["outline"]));point=ring.representative_point()
        assert not any(Polygon(r).contains(point) for p in after["parts"] if p["z0"]<3.<p["z1"] for r in api.part_regions(p,3.)),tag
    fin=api.solid_geometry("FIN-KITCHEN-PATIO",source);xy=case["rain_airway"][0][:2]
    assert Polygon(api._find(source,"OP-RAIN-9-SL1")["geometry"]["outline"]).contains(Point(xy)),"Slab bore follows actual collector inlet"
    assert not any(Polygon(p["outline"]).contains(Point(xy)) for p in fin["parts"]),"Actual wearing finish inlet bore"


def test_servant_radiators_have_open_section_gaps_and_actual_floor_feet():
    """♨️ Compare native cast sections with independent hull volume and real supporting floor contact."""
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=api.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text())
    for row in case['servant_radiators']:
        e=api._find(m,row['tag']);g=api.service_geometry(e['tag'],m);parts=g['parts'];oracle=sum(ConvexHull(api.part_vertices(p)).volume for p in parts)
        assert abs(oracle-api.quantities(e['tag'],m)['volume'])<1e-8
        def contains(p,q):return all(sum(v[k]*q[k] for k in range(3))+v[3]<1e-8 for v in ConvexHull(api.part_vertices(p)).equations)
        cx,cy=row['axis'];assert not any(contains(p,[cx,cy,.4]) for p in parts),'Real central inter-section air gap'
        assert any(contains(p,[cx+.06,cy-.1875,.4]) for p in parts),'Actual metal section remains'
        wearing=unary_union([Polygon(p['outline']) for p in api.solid_geometry(row['floor'],m)['parts'] if p['role']=='wearing'])
        for p in parts:
            if p['role']=='radiator_foot':
                assert p['z0']==0
                ring=Polygon(api.part_footprint(p));assert ring.difference(wearing).area<1e-7
