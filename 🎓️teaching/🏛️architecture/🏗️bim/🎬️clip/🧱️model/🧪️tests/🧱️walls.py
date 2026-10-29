"""🧱 Portable cavity-wall geometry checked independently with Shapely."""
import importlib
import json
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _case():
    return json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))


def _fixture():
    source,case=model.load_model(),_case()
    wall=next(e for e in source["elements"] if e["tag"]=="W-S")
    wall["geometry"].update(path=[[0,0],[4,0]],base_level=0,top_level=3,exterior_side=-1)
    wall["attributes"]["thickness"]["value"]=case["wall"]["thickness"]
    wall["geometry"].pop("layer_regions",None)
    layers=next(m for m in source["materials"] if m["tag"]=="LS-FACADE-29")
    layers["layers"]=[{"id":l["id"],"material":"MAT-BRICK","name":{"en":l["id"],"de":l["id"]},"thickness":{"value":l["thickness"],"unit":"m","basis":"assumed"},"is_void":l.get("is_void",False)} for l in case["wall"]["layers"]]
    source["elements"]=[e for e in source["elements"] if e["class"]!="IfcOpeningElement" or e["geometry"].get("wall")!="W-S" or e["tag"]=="OP-S"]
    opening=next(e for e in source["elements"] if e["tag"]=="OP-S")
    opening["attributes"].update({k:{"value":v,"unit":"m","basis":"assumed"} for k,v in case["openings"][0].items()})
    source["elements"]=[e for e in source["elements"] if e["class"]!="IfcBeam"]
    return source,wall,case


def test_cavity_wall_parts_preserve_opening_and_exclude_air():
    """🕳️ A wall aperture removes every solid leaf and its cavity never becomes masonry."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source,wall,case=_fixture()
    geometry=model._wall_geometry(source,wall)
    levels=sorted({p[k] for p in geometry["parts"] for k in ("z0","z1")})
    volume=sum((hi-lo)*unary_union([Polygon(p["outline"]) for p in geometry["parts"] if p["z0"]<(lo+hi)/2<p["z1"]]).area for lo,hi in zip(levels,levels[1:]))
    assert abs(volume-case["expected"]["volume"])<1e-9
    assert abs(model.quantities("W-S",source)["volume"]-volume)<1e-9
    assert len({p["layer"] for p in geometry["parts"]})==case["expected"]["layer_count"]
    assert all(p["layer"]!="cavity" for p in geometry["parts"])
    assert abs(model.quantities("W-S",source)["cavity_volume"]-case["expected"]["cavity_volume"])<1e-9


def test_wall_axis_reversal_preserves_solid_ownership():
    """🔄 Reversing axis and exterior side leaves every physical material layer unchanged."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source,wall,_=_fixture()
    first=model._wall_geometry(source,wall)
    wall["geometry"]["path"].reverse()
    wall["geometry"]["exterior_side"]=1
    second=model._wall_geometry(source,wall)
    for layer in {p["layer"] for p in first["parts"]}:
        a=unary_union([Polygon(p["outline"]) for p in first["parts"] if p["layer"]==layer])
        b=unary_union([Polygon(p["outline"]) for p in second["parts"] if p["layer"]==layer])
        assert a.symmetric_difference(b).area<1e-10


def test_wall_body_uses_occupied_parts():
    """🧊 IFC body prisms contain the same occupied volume as the material takeoff."""
    source,wall,_=_fixture()
    geometry=model._wall_geometry(source,wall)
    body=model.represent("W-S","3d",source)
    assert len(body["primitives"])==len(geometry["parts"])


def test_step_wall_extrusions_use_occupied_leaves():
    """📦 Every exported wall solid comes from one shared occupied material part."""
    source,wall,_=_fixture()
    exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
    geometry=model.building_geometry(source)
    body=exchange._wall(model.element("W-S",source),geometry)
    expected=model._wall_geometry(source,wall)["parts"]
    assert len(body["parts"])==len(expected)
    assert body["point"]==[0.,0.,0.]
    assert all(a["outline"]==b["outline"] and a["z0"]==b["z0"] and a["z1"]==b["z1"] for a,b in zip(body["parts"],expected))


def test_source_facade_families_keep_solid_and_cavity_dimensions_distinct():
    """🏛️ Source-attested finished family totals retain the inferred air-width provenance."""
    source=model.load_model()
    for case in _case()["facades"]:
        wall=model._find(source,case["tag"])
        assert wall["geometry"]["exterior_side"]==case["side"]
        assert model._wall_thickness(source,wall)==case["thickness"]
        layers=model._material(source,wall)["layers"]
        cavity=next(l for l in layers if l.get("is_void"))
        assert cavity["thickness"]["basis"]=="inferred"
        assert abs(sum(l["thickness"]["value"] for l in layers if not l.get("is_void"))-.175)<1e-10
        geometry=model._wall_geometry(source,wall)
        quantities=model.quantities(case["tag"],source)
        assert {r["tag"] for r in wall["geometry"]["layer_regions"]}==set(_case()["facade_regions"])
        assert abs(quantities["gross_volume"]-sum(l["gross_side_area"]*(l["offsets"][1]-l["offsets"][0]) for l in geometry["layers"] if not l["is_void"]))<1e-8
        assert quantities["bounding_volume"]>=quantities["envelope_volume"]>quantities["gross_volume"]>quantities["volume"]


def test_vertical_material_regions_follow_opening_sill_and_keep_outer_plane():
    """📏 A thinner parapet and deep upper screen share one exterior face and follow sill events."""
    from copy import deepcopy
    source,wall,case=_fixture()
    layers=deepcopy(model._material(source,wall))
    layers["tag"]="TEST-PARAPET"
    next(l for l in layers["layers"] if l["id"]=="cavity")["thickness"]["value"]=.025
    source["materials"].append(layers)
    def station(datum):
        return {"datum":datum,"offset":{"value":0,"unit":"m","basis":"assumed"},**({"opening":"OP-S"} if datum=="sill" else {})}
    wall["geometry"]["layer_regions"]=[{"tag":"lower","from":station("base"),"to":station("sill"),"material":"TEST-PARAPET"},{"tag":"upper","from":station("sill"),"to":station("top"),"material":"LS-FACADE-29"}]
    geometry=model._wall_geometry(source,wall)
    q=model.quantities("W-S",source)
    assert abs(q["envelope_volume"]-case["regional"]["expected_envelope_volume"])<1e-9
    assert abs(q["cavity_volume"]-case["regional"]["expected_cavity_volume"])<1e-9
    for region in ("lower","upper"):
        exterior=[p for p in geometry["parts"] if p["region"]==region and p["layer"]=="render"]
        assert all(abs(min(v[1] for v in p["outline"])+.145)<1e-9 for p in exterior)
    opening=model._find(source,"OP-S")
    opening["attributes"]["sill"]["value"]=1.2
    q=model.quantities("W-S",source)
    assert abs(q["cavity_volume"]-case["regional"]["expected_cavity_after_sill_resize"])<1e-9
