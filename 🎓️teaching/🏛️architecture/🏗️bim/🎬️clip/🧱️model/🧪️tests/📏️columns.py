"""🧪 Portable column profiles checked against independent Shapely and NumPy geometry."""
import importlib.util
import json
import math
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon


def run(root):
    """📏 Verify physical contours, quantities, mesh and semantic STEP profiles."""
    path = Path(root) / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model"
    spec = importlib.util.spec_from_file_location("columns_core", path / "🐍️.py")
    core = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(core)
    spec = importlib.util.spec_from_file_location("columns_exchange", path / "🔌️exchange/🐍️.py")
    exchange = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(exchange)
    exchange._CORE = core
    cases = json.loads((path / "🧪️tests/📏️columns.json").read_text(encoding="utf-8"))
    original=core.load_model()
    for case in cases["source_profiles"]:
        column=core._find(original,case["tag"]);profile=core._find(original,column["type"])["profile"]
        assert profile["shape"]==case["profile"]["shape"],case["tag"]
        assert math.isclose(profile["diameter"]["value"],case["profile"]["diameter"],abs_tol=1e-12)
        assert column["geometry"]["axis"]==case["axis"]
        assert column["geometry"]["base_level"]==case["base"] and column["geometry"]["top_level"]==case["top"]
        assert core._find(original,case["upper_tag"])["type"]==case["upper_type"]
        area=math.pi*(case["profile"]["diameter"]/2)**2
        assert math.isclose(core.quantities(case["tag"],original)["volume"],area*(case["top"]-case["base"]),abs_tol=1e-12)
        column_mesh=exchange.mesh(case["tag"],256,original)
        vertices,faces=np.asarray(column_mesh["vertices"]),np.asarray(column_mesh["faces"])
        measured=abs(np.einsum('ij,ij->i',vertices[faces[:,0]],np.cross(vertices[faces[:,1]],vertices[faces[:,2]])).sum()/6)
        assert math.isclose(measured,area*(case["top"]-case["base"]),rel_tol=.0002)
        assert 'IFCCIRCLEPROFILEDEF' in '\n'.join(exchange.step_excerpt(case["tag"],original)["lines"])
    checked = []
    for case in cases["cases"]:
        m = core.load_model()
        obj = next(e for e in m["elements"] if e["class"] == "IfcColumn")
        typ = core._find(m, obj["type"])
        typ["profile"] = case["profile"]
        obj["geometry"] = {"axis": [2., 3.], "base_level": 0., "top_level": cases["height"]}
        g = core.column_geometry(m, obj, 256)
        q = core.quantities(obj["tag"], m)
        assert math.isclose(q["cross_section_area"], case["area"], abs_tol=1e-12)
        assert math.isclose(q["volume"], case["area"] * cases["height"], abs_tol=1e-12)
        assert math.isclose(Polygon(g["parts"][0]["outline"]).area, case["area"], rel_tol=.0002)
        mesh = exchange.mesh(obj["tag"], 256, m)
        v = np.asarray(mesh["vertices"])
        f = np.asarray(mesh["faces"])
        volume = abs(np.einsum('ij,ij->i', v[f[:,0]], np.cross(v[f[:,1]],v[f[:,2]])).sum()/6)
        assert math.isclose(volume,q["volume"],rel_tol=.0002), (case["name"],volume,q)
        assert ('diameter' in q) == (case["profile"]["shape"] == 'circle')
        step = '\n'.join(exchange.step_excerpt(obj["tag"],m)["lines"])
        cls = {'circle':'IFCCIRCLEPROFILEDEF','rectangle':'IFCRECTANGLEPROFILEDEF','ellipse':'IFCELLIPSEPROFILEDEF','polygon':'IFCARBITRARYCLOSEDPROFILEDEF'}[case["profile"]["shape"]]
        assert cls in step
        import re
        definitions = set(re.findall(r'^#(\d+)=',step,re.M))
        references = set(re.findall(r'#(\d+)',step))
        assert definitions == references
        edges = {}
        for triangle in f:
            for a,b in zip(triangle,np.roll(triangle,-1)):
                edge = tuple(sorted((int(a),int(b))))
                edges[edge] = edges.get(edge,0)+1
        assert set(edges.values()) == {2}
        for view in ('plan','footprint','section','elevation','3d','body','axis'):
            assert core.represent(obj["tag"], view, m)
        checked.append(case["name"])
    m = core.load_model()
    obj = next(e for e in m["elements"] if e["class"] == "IfcColumn")
    obj["geometry"] = {"axis":[2.,3.],"base_level":0.,"top_level":2.91,"sections":[
        {"base_level":0.,"top_level":1.2,"profile":cases["variable"]["lower"]},
        {"base_level":1.2,"top_level":2.91,"profile":cases["variable"]["upper"]}]}
    q = core.quantities(obj["tag"],m)
    expected = .18*.22*1.2 + math.pi*.14**2*1.71
    assert math.isclose(q['volume'],expected,abs_tol=1e-12)
    assert len(core.column_geometry(m,obj)['parts']) == 2
    step = '\n'.join(exchange.step_excerpt(obj["tag"],m)["lines"])
    assert 'IFCCIRCLEPROFILEDEF' in step and 'IFCRECTANGLEPROFILEDEF' in step
    mesh = exchange.mesh(obj['tag'], 256, m)
    v, f = np.asarray(mesh['vertices']), np.asarray(mesh['faces'])
    volume = abs(np.einsum('ij,ij->i', v[f[:,0]],np.cross(v[f[:,1]],v[f[:,2]])).sum()/6)
    assert math.isclose(volume,expected,rel_tol=.0002)
    typ = core._find(m,obj['type'])
    assert math.isclose(core.quantities(typ['tag'],m)['volume'],sum(core.quantities(tag,m)['volume'] for tag in core.type_occurrences(typ['tag'],m)),abs_tol=1e-12)
    checked.append('vertical sections')
    return checked
  