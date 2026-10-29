"""🚪 Continuous garage portal and frameless sliding-train contracts."""
import importlib
import json
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def _case():
    return json.loads(Path(__file__).with_suffix(".json").read_text())


def test_garage_portal_has_no_false_intermediate_masonry_piers():
    """🚗 One registered continuous portal remains physically free of false intermediate masonry."""
    from shapely.geometry import Polygon,box
    from shapely.ops import unary_union
    source,case=model.load_model(),_case()
    host=model._find(source,case["host"])
    opening=model._find(source,case["opening"])
    assert opening["geometry"]["head_level"]==case["head_level"]
    geometry=model._wall_geometry(source,host)
    portal=box(15.926,case["world_y"][0],16.074,case["world_y"][1])
    masonry=unary_union([Polygon(p["outline"]) for p in geometry["parts"] if p["z0"]<case["section_z"]<p["z1"]])
    assert masonry.intersection(portal).area<1e-9
    assert len([e for e in source["elements"] if e["class"]=="IfcOpeningElement" and e["geometry"].get("wall")==case["host"]])==1


def test_garage_two_train_panels_are_frameless_and_material_unverified():
    """🚂 Keep the attested two trains separate from unknown panel build-up and train proportions."""
    source,case=model.load_model(),_case()
    door=model._find(source,case["door"])
    geometry=model._filling_geometry(source,door)
    assert len(geometry["parts"])==case["trains"]
    assert all(p["role"]=="panel" and p["material"]=="MAT-GARAGE-PANEL" for p in geometry["parts"])
    assert {p["panel_index"] for p in geometry["parts"]}=={0,1}
    q=model.quantities(case["door"],source)
    assert q["frame_volume"]==0
    assert abs(q["clear_width"]-(case["end"]-case["start"]))<1e-9
    assert abs(q["clear_height"]-model._top(source,case["head_level"]))<1e-9
    assert model._find(source,door["type"])["filling"]["panel_thickness"]["basis"]=="inferred"
    from shapely.geometry import Polygon
    from shapely import intersection
    for column in [e for e in source["elements"] if e["class"]=="IfcColumn"]:
        for part in model.column_geometry(source,column)["parts"]:
            for panel in geometry["parts"]:
                height=min(part["z1"],panel["z1"])-max(part["z0"],panel["z0"])
                if height>1e-9:
                    common=intersection(Polygon(part["outline"]),Polygon(panel["outline"]),grid_size=1e-10).area*height
                    assert common<=case["positive_column_overlap_volume_max"],column["tag"]


def test_garage_head_tracks_its_actual_slab_soffit():
    """⬆️ A slab-level event updates the continuous garage head without a copied numeric lintel."""
    source,case=model.load_model(),_case()
    before=model.quantities(case["door"],source)["clear_height"]
    first=next(s for s in source["storeys"] if s["id"]=="first")
    first["elevation"]["value"]+=.1
    after=model.quantities(case["door"],source)["clear_height"]
    assert abs(after-before-.1)<1e-9
