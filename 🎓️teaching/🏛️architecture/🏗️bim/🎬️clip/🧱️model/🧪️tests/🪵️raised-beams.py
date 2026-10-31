"""🪵 Real floor material must respect exclusive raised beam ownership."""
import importlib,json
from pathlib import Path
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_real_floor_layers_do_not_occupy_exclusive_raised_beam_material():
    """🧩 Integrate independent GEOS occupied intersections in every actual height stratum."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    c=json.loads(Path(__file__).with_suffix(".json").read_text());m=a.load_model();beams=[p for t in c["beam_tags"] for p in a._solid_geometry(m,a._find(m,t))["parts"] if p["z0"]<c["finished"] and p["z1"]>3.08];overlap=0.
    for tag in c["floor_tags"]:
        for p in a.solid_geometry(tag,m)["parts"]:
            active=[q for q in beams if q["z0"]<p["z1"] and q["z1"]>p["z0"]];zs=sorted({p["z0"],p["z1"],*(max(p["z0"],min(p["z1"],q[k])) for q in active for k in ("z0","z1"))})
            for lo,hi in zip(zs,zs[1:]):
                z=(lo+hi)/2;owned=unary_union([Polygon(r) for q in active if q["z0"]<z<q["z1"] for r in a.part_regions(q,z)]);overlap+=Polygon(p["outline"]).intersection(owned).area*(hi-lo)
    assert overlap<c["tolerance"],overlap
