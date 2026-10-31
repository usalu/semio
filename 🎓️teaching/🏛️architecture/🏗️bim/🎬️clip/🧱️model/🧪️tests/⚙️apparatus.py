"""⚙️ Native unknown apparatus interface with qualified user-defined purpose."""
import importlib,json,copy
from pathlib import Path
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_native_unknown_apparatus_preserves_owned_volume_and_required_user_type():
    """🔎 Verify schema, independent GEOS quantity and real nine-attribute native STEP entity."""
    from shapely.geometry import Polygon
    from jsonschema import Draft202012Validator
    c=json.loads(Path(__file__).with_suffix(".json").read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(c);m=a.load_model();e=copy.deepcopy(c["fixture"]);e["guid"]=a.global_id(e["tag"]);m["elements"].append(e);Draft202012Validator(json.loads((Path(a.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())).validate(m)
    g=a._solid_geometry(m,e);v=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in g["parts"]);assert abs(v-c["volume"])<1e-10;assert abs(a.quantities(e["tag"],m)["volume"]-v)<1e-10
    ex=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
    for language in ("en","de"):
        native=ex.step_excerpt(e["tag"],m,language=language);line=next(l for l in native["lines"] if "=IFCBUILDINGELEMENTPROXY(" in l);values=line.split("(",1)[1].rsplit(")",1)[0].split(",");assert len(values)==9 and values[-1]==".USERDEFINED.";assert values[4]!="$","USERDEFINED requires actual IfcObject.ObjectType"


def test_original_kitchen_appliances_and_visible_pipes_have_real_hollow_supported_material():
    """🍳 Compare source-qualified hollow bodies, exact support and quantities with SciPy and GEOS."""
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    c=json.loads(Path(__file__).with_suffix(".json").read_text())["physical"];m=a.load_model();assert all(row["tag"] in a._registry(m) for row in c["products"]);bodies={}
    for row in c["products"]:
        e=a._find(m,row["tag"]);assert e["class"]==row["class"];g=a._solid_geometry(m,e);bodies[row["tag"]]=g;hulls=[ConvexHull(a.part_vertices(p)) for p in g["parts"]];volume=sum(h.volume for h in hulls);assert abs(a.quantities(e["tag"],m)["volume"]-volume)<1e-7,(e["tag"],"materialquantity")
        for key,expected in (("empty",False),("metal",True)):
            for point in row[key]:
                q=[*point[:2],point[2]+c["base"]];inside=any(all(sum(plane[k]*q[k] for k in range(3))+plane[3]<-1e-9 for plane in h.equations) for h in hulls);assert inside==expected,(e["tag"],key,point)
    stand=bodies[c["stand"]];top=c["base"]+c["top"];support=unary_union([Polygon(r) for p in stand["parts"] for r in a.part_regions(p,top-1e-8)])
    for tag in ("KITCHEN-APPLIANCE-CYLINDER","KITCHEN-APPLIANCE-PLATE"):
        bottom=unary_union([Polygon(r) for p in bodies[tag]["parts"] for r in a.part_regions(p,top+1e-8)]);assert bottom.intersection(support).area>.001,(tag,"standcontact")
    floor=unary_union([Polygon(p["outline"]) for p in a.solid_geometry("FIN-KITCHEN",m)["parts"] if p["role"]=="wearing"])
    feet=[p for p in stand["parts"] if p["role"]=="inferred_leg_shell"];assert feet and all(abs(p["z0"]-c["base"])<1e-8 and Polygon(p["outline"]).difference(floor).area<1e-8 for p in feet)
