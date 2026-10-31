"""➖️ Verify semantic curve differences independently of display tessellation and owner labels."""
import importlib,json,math
from pathlib import Path
from jsonschema import Draft202012Validator
B=Path(__file__).parent
CASE=json.loads((B/"🔣️.json").read_text(encoding="utf-8"))


def test_analytic_difference_schema_retains_native_curve_operands_and_separate_air_roles():
    """🧬 Require a single semantic geometry authority with explicit axes, profile voids and recursive finite stock."""
    validator=Draft202012Validator(json.loads((B.parent/"🧬️.schema.json").read_text(encoding="utf-8")))
    for row in CASE["cases"]:
        validator.validate(row["part"])
        invalid={**row["part"],"mesh":{"vertices":[],"faces":[]}}
        assert not validator.is_valid(invalid),row["id"]


def test_analytic_difference_measures_exact_curve_sections_and_retains_two_nanometre_material():
    """⚖️ Match independently specified analytic quantities and positive material without polygon buffers."""
    backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️")
    for row in CASE["cases"]:
        measured=backend.measure(row["part"]);expected=row["expected"]
        if "owned_volume"in expected:assert math.isclose(measured["volume"],expected["owned_volume"],rel_tol=1e-10,abs_tol=1e-11),(row["id"],measured)
        if expected.get("owned_volume_positive"):assert measured["volume"]>0,row["id"]
        assert measured["absolute_error"]>=0 and measured["method"]=="analytic_sections"


def test_analytic_difference_progress_cancellation_and_operand_isolation():
    """🛑 Keep expensive analytic measures cancellable without mutating source operands or using display geometry."""
    backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️");raw=CASE["cases"][0]["part"];before=json.dumps(raw,sort_keys=True);events=[]
    try:backend.measure(raw,progress=events.append,cancel=lambda:True)
    except InterruptedError:pass
    else:raise AssertionError("Analytic stock difference ignored cancellation")
    assert backend.measure(raw,progress=events.append)["volume"]>0 and events
    assert json.dumps(raw,sort_keys=True)==before


def test_inclined_analytic_stock_cancels_after_first_progress_without_partial_publication():
    """🛑 Cancel an actual sloping-stock analytic calculation after progress has begun."""
    backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️");row=next(r for r in CASE["cases"]if r.get("cancel_after_progress"));events=[]
    try:backend.measure(row["part"],progress=events.append,cancel=lambda:len(events)>=row["cancel_after_progress"])
    except InterruptedError:pass
    else:raise AssertionError("Inclined analytic solid ignored cancellation after progress")
    assert events


def test_semantic_construction_translation_bounds_and_actual_owner_intersection():
    """🏗️ Preserve real curve operands through local/world transport and receiving-material Boolean operations."""
    backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️");row=CASE["cases"][0];part=row["part"]
    moved=backend.translate(part,[3,-2,5]);assert backend.bounds(moved)==((3,5),(-3,-1),(5,6))
    assert math.isclose(backend.measure(moved)["volume"],row["expected"]["owned_volume"],abs_tol=1e-11)
    stock={"material":"MAT-TEST","role":"stock","outline":part["csg"]["stock"]["outline"],"holes":[],"z0":0,"z1":1}
    nested=backend.reference_material_parts(stock,[part]);assert len(nested)==1
    assert math.isclose(backend.measure(nested[0])["volume"],4-row["expected"]["owned_volume"],abs_tol=1e-11)
    owner={"material":"MAT-OTHER","role":"column","profile":{"shape":"circle","diameter":{"value":1,"unit":"m","basis":"assumed"},"voids":[]},"axis":[0,0],"z0":0,"z1":1}
    common=backend.reference_material_parts(stock,[],bounds=[owner]);assert len(common)==1
    assert math.isclose(backend.measure(common[0])["volume"],4-row["expected"]["owned_volume"],abs_tol=1e-11)
    assert backend.operand(stock)==part["csg"]["stock"]


def test_mixed_semantic_material_union_counts_actual_occupation_once():
    """⚖️ Integrate the true union of overlapping semantic stocks without additive or owner-label shortcuts."""
    backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️");first=CASE["cases"][0]["part"];second=backend.translate(first,[1,0,0]);actual=backend.measure_union([first,second])
    assert math.isclose(actual["volume"],6-math.pi/8,abs_tol=1e-11)


def test_core_semantic_stock_dispatch_uses_actual_operators_and_source_local_transport():
    """🏗️ Keep native construction, source transport and volume consumers on the semantic authority."""
    api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️");part=CASE["cases"][0]["part"]
    assert api._material_bounds(part)==((0,2),(-1,1),(0,1))
    stock={"material":"MAT-TEST","role":"stock","outline":part["csg"]["stock"]["outline"],"holes":[],"z0":0,"z1":1}
    remainder=api._reference_material_parts(stock,[part]);assert len(remainder)==1 and "csg"in remainder[0]
    assert math.isclose(api.part_volume(remainder[0]),4-CASE["cases"][0]["expected"]["owned_volume"],abs_tol=1e-11)
    assert math.isclose(api._occupied_volume([part,backend.translate(part,[1,0,0])]),6-math.pi/8,abs_tol=1e-11)
    assert math.isclose(api._partition_volume([part]),CASE["cases"][0]["expected"]["owned_volume"],abs_tol=1e-11)
    neutral=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📦️disjoint");model=neutral.model({"id":"csg-local-transport"},[]);obj=model["elements"][0];obj["geometry"]["base_level"]=5
    actual=api._solid_stock_parts(model,obj,[part],exact=True);assert backend.bounds(actual[0])==((0,2),(-1,1),(5,6))
    try:api._native_material_cells(part)
    except ValueError:pass
    else:raise AssertionError("An authoritative semantic part cannot become physical display cells")


def test_building_public_wall_list_publishes_semantic_scalars_after_construction():
    """📤 Keep exact source operands private until the complete public building query boundary."""
    from unittest.mock import patch
    api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");backend=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️");neutral=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📦️disjoint")
    part=backend.translate(CASE["cases"][0]["part"],[0,0,0]);model=neutral.model({"id":"semantic-building-publication"},[])
    with patch.object(api,"_building_geometry_uncached",return_value={"walls":[{"parts":[part]}]}):actual=api.building_geometry(model)
    serialized=json.dumps(actual,allow_nan=False);assert serialized and actual["walls"][0]["parts"][0]["csg"]
