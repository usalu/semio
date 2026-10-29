"""🪟 Portable physical opening filling checks with independent solid unions."""
import importlib
import json
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def test_filling_perimeter_solids_and_clear_aperture():
    """🚪 Keep physical frame ownership separate from rough aperture and free passage."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    source=model.load_model()
    filling=next(e for e in source["elements"] if e["tag"]=="D-SON")
    kind=model._find(source,filling["type"])
    opening=model._find(source,filling["geometry"]["opening"])
    opening["attributes"].update({k:{"value":v,"unit":"m","basis":"assumed"} for k,v in {"start":0,"end":case["rough_width"],"sill":0,"head":case["rough_height"]}.items()})
    g=model._filling_geometry(source,filling)
    assert abs(g["clear_width"]-case["expected_clear_width"])<1e-9
    assert abs(g["clear_height"]-case["expected_clear_height"])<1e-9
    assert len([p for p in g["parts"] if p["role"]=="frame"])==3
    for part in g["parts"]:
        assert part["material"]!="MAT-RENDERED"
    volume=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in g["parts"])
    assert abs(model.quantities("D-SON",source)["volume"]-volume)<1e-9


def test_registered_curtain_has_real_planar_members_and_native_portals():
    """🌐 Compare authored metal rail paths with independent GEOS buffers and finite native apertures."""
    from shapely.geometry import LineString,Polygon
    from shapely import set_precision
    source=model.load_model()
    curtain=model._find(source,"CW-1")
    spec=model._find(source,curtain["type"])["filling"]
    geometry=model.filling_geometry("CW-1",source)
    assert any(p.get("member_kind")=="mullion" for p in geometry["parts"])
    assert any(p.get("member_kind")=="transom" for p in geometry["parts"])
    for record in geometry["paths"]:
        expected=set_precision(LineString(record["path"]).buffer(spec["frame_depth"]["value"]/2,cap_style=2,join_style=2),1e-10)
        actual=set_precision(Polygon(model._path_band(record["path"],spec["frame_depth"]["value"])),1e-10)
        assert actual.symmetric_difference(expected).area<1e-9
    for part in geometry["parts"]:
        if part["role"]=="glass":
            polygon=Polygon(part["outline"])
            assert set_precision(polygon,1e-10).symmetric_difference(set_precision(polygon.minimum_rotated_rectangle,1e-10)).area<1e-9
    for kind in ("axis","footprint","plan","section","elevation","3d","body"):
        assert model.represent("CW-1",kind,source)["kind"]==kind

def test_native_filling_materials_and_clear_widths_are_explicit():
    """🪵 Source native fillings distinguish original frame and panel material ownership."""
    source=model.load_model()
    fillings=[model.filling_geometry(e["tag"],source) for e in source["elements"] if e["class"] in ("IfcDoor","IfcWindow","IfcCurtainWall")]
    assert model.quantities("D-SON",source)["clear_width"]>=.8
    for filling in fillings:
        assert all(p["material"] not in ("MAT-RENDERED","MAT-AIR") for p in filling["parts"])
    assert model._find(source,"TY-RIBBON")["filling"]["frame_material"]=="MAT-WOOD"
    assert model._find(source,"TY-SALON-GLAZED")["filling"]["panel_material"]=="MAT-GLASS"


def test_material_relations_do_not_reassign_typed_fillings_to_concrete():
    """🔗 Material association targets match their actual authored material ownership."""
    source=model.load_model()
    registry=model._registry(source)
    for relation in source["relations"]:
        if relation["class"]=="IfcRelAssociatesMaterial":
            for tag in relation["related"]:
                assert registry[tag].get("material")==relation["relating"]


def test_glass_face_area_excludes_frames_and_includes_glazed_doors():
    """💎 Glass area comes from actual panel faces, including transparent door leaves."""
    from shapely.geometry import Polygon
    source=model.load_model()
    window=model.quantities("RW-S",source)
    spec=model._find(source,"TY-RIBBON")["filling"]
    sash=spec["sash_width"]["value"]
    assert abs(window["glazing_area"]-(window["clear_width"]-2*sash)*(window["clear_height"]-2*sash))<1e-8
    assert window["glazing_area"]<window["area"]
    door=model._find(source,"D-SALON")
    physical=model._filling_geometry(source,door)
    expected=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"])/physical["panel_thickness"] for p in physical["parts"] if p["role"]=="glass")
    wall=model.quantities(physical["host"],source)
    assert expected>0
    glass_total=0.
    for filling in source["elements"]:
        if filling["class"] in ("IfcWindow","IfcDoor"):
            opening=model._find(source,filling["geometry"]["opening"])
            if opening["geometry"].get("wall")==physical["host"]:
                g=model.filling_geometry(filling["tag"],source)
                glass_total+=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"])/g["panel_thickness"] for p in g["parts"] if p["role"]=="glass")
    assert abs(wall["glazing_area"]-glass_total)<1e-8
    assert abs(wall["glazing_share"]-glass_total/wall["gross_side_area"])<1e-8
    assert model.quantities("D-ENTRY",source)["glazing_area"]==0
