"""🪟 Neutral occurrence placement and separate rough versus outer frame envelope."""
import importlib,json,copy
from pathlib import Path
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_occurrence_normal_offset_and_head_recess_preserve_rough_aperture():
    """📏 Compare all native solid vertices against independent NumPy rigid translation."""
    import numpy as np
    from jsonschema import Draft202012Validator
    c=json.loads(Path(__file__).with_suffix(".json").read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(c);m=a.load_model();obj=a._find(m,c["tag"]);base=a._filling_geometry(m,obj);axis=np.asarray(base["axis"]);tangent=(axis[1]-axis[0])/np.linalg.norm(axis[1]-axis[0]);domain=a._opening_domain(m,a._find(m,obj["geometry"]["opening"]));normal=np.array([-tangent[1],tangent[0],0])*domain["side"]
    for offset in c["offsets"]:
        changed=copy.deepcopy(obj);changed["geometry"]["normal_offset"]={"value":offset,"unit":"m","basis":"inferred","source":"Neutral fixture"};g=a._filling_geometry(m,changed);assert len(g["parts"])==len(base["parts"])
        for old,new in zip(base["parts"],g["parts"]):assert np.allclose(np.asarray(a.part_vertices(new))-np.asarray(a.part_vertices(old)),normal*offset,atol=c["tolerance"],rtol=0)
        assert abs(g["rough_height"]-base["rough_height"])<c["tolerance"]
    changed=copy.deepcopy(obj);changed["geometry"]["frame_head_extension"]={"value":c["head_extension"],"unit":"m","basis":"inferred","source":"Neutral head recess"};g=a._filling_geometry(m,changed);assert abs(g["rough_height"]-base["rough_height"])<c["tolerance"];assert abs(g["overall_height"]-base["rough_height"]-c["head_extension"])<c["tolerance"];assert abs(max(p["z1"] for p in g["parts"])-max(p["z1"] for p in base["parts"])-c["head_extension"])<c["tolerance"]
