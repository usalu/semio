"""🕳️ Source-backed cellar topology and independent Shapely regression."""
import importlib
import json
import math
from pathlib import Path

api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_cellar_contract():
    """🏚️ Verify measured construction values, a partial basement and genuine stair access."""
    source = api.load_model()
    case = json.loads(Path(__file__).with_name("🕳️cellar.json").read_text(encoding="utf-8"))
    expected = case["expected"]
    elements = {e["tag"]: e for e in source["elements"]}
    assert next(s for s in source["storeys"] if s["id"] == "cellar")["elevation"]["value"] == expected["floor"]
    assert elements["SL-CELLAR"]["geometry"]["regions"][0]["thickness"]["value"] == expected["floor_slab_thickness"]
    assert elements["SL-GROUND-CELLAR"]["geometry"]["regions"][0]["thickness"]["value"] == expected["slab_thickness"]
    stair = elements["ST-CG"]["geometry"]
    assert stair["step_count"] == expected["stair_step_count"] != elements["ST-GF"]["geometry"]["step_count"]
    assert stair["from_level"] == "cellar" and stair["to_level"] == "ground"
    assert all(math.isclose(a,b,abs_tol=1e-6) for a,b in zip(stair["centre"],expected["stair_centre"]))
    assert len(stair["step_parts"]) == stair["step_count"]
    assert math.isclose(expected["floor"]+stair["step_parts"][-1]["z1"],0,abs_tol=1e-9)
    assert math.isclose(min(p["z0"] for p in api.solid_geometry("FT-CELLAR",source)["parts"]),expected["footing_bottom"],abs_tol=1e-9)
    assert elements["FT-CELLAR"]["class"] == "IfcFooting"
    assert elements["FT-CELLAR-C14"]["class"] == "IfcFooting"
    assert all(api.represent(tag,"3d",source)["primitives"] for tag in ["FT-CELLAR","RIB-CELLAR","ST-CG"])
    assert {e["tag"] for e in source["elements"] if e["class"] == "IfcOpeningElement" and e["geometry"].get("wall") in {"W-CELLAR-W","W-CELLAR-E","W-CELLAR-N","W-CELLAR-S"}} == {"OP-COAL-CELLAR","OP-HOIST-CELLAR"}

def test_cellar_shapely():
    """📏 Validate registered bounds, voids and union takeoffs against Shapely."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source = api.load_model()
    elements = {e["tag"]: e for e in source["elements"]}
    floor = Polygon(elements["SL-CELLAR"]["geometry"]["outline"])
    slab = elements["SL-GROUND-CELLAR"]["geometry"]
    void = Polygon(slab["voids"][0]["outline"])
    assert floor.is_valid and floor.area < 85 and floor.contains(void)
    assert all(void.covers(Polygon(step["outline"])) for step in elements["ST-CG"]["geometry"]["step_parts"])
    assert math.isclose(api.quantities("SL-CELLAR",source)["area"],floor.area,abs_tol=1e-8)
    central_pad = Polygon(elements["FT-CELLAR-C14"]["geometry"]["parts"][0]["outline"])
    strip = unary_union([Polygon(p["outline"]) for p in elements["FT-CELLAR"]["geometry"]["parts"]])
    assert strip.covers(central_pad)
    for tag in ["FT-CELLAR","RIB-CELLAR","FT-STAIR-PA","FT-CELLAR-C14"]:
        parts = elements[tag]["geometry"]["parts"]
        levels = sorted({p[k] for p in parts for k in ["z0","z1"]})
        volume = sum((hi-lo)*unary_union([Polygon(p["outline"]) for p in parts if p["z0"] < (lo+hi)/2 < p["z1"]]).area for lo,hi in zip(levels,levels[1:]))
        assert all(Polygon(p["outline"]).is_valid for p in parts)
        assert math.isclose(api.quantities(tag,source)["volume"],volume,abs_tol=1e-7)
