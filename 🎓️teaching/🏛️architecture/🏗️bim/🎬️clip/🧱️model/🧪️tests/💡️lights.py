"""💡 Native original light fixture bodies, source qualifications and actual mounting contracts."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_native_lighting_class_and_mounting_protocol_are_explicit():
    """📜 Require a real IFC fixture and closed source-qualified host mounting rather than scene lights."""
    schema=json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())
    exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
    assert "IfcLightFixture" in schema["$defs"]["ifc_class"]["enum"]
    assert "lighting" in schema["$defs"]["element"]["properties"]["system"]["enum"]
    assert schema["$defs"]["geometry"]["properties"]["mounting"]["additionalProperties"] is False
    assert "IfcLightFixture" in exchange.SERVICE_CLASSES
    assert exchange.BODIES["IfcLightFixture"] is exchange._service

def test_registered_original_lights_have_physical_bodies_and_qualified_native_identity():
    """🕯️ Check only photo/text-attested executions, explicit inferred dimensions and native EN/DE records."""
    import jsonschema
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    schema=json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())
    jsonschema.Draft202012Validator(schema).validate(source)
    assert source["classes"][case["class"]]=={"supertype":"IfcFlowTerminal","adds":["PredefinedType"]}
    for row in case["fixtures"]:
        obj=api._find(source,row["tag"])
        assert obj["class"]==case["class"] and obj["predefined_type"]==row["predefined_type"]
        assert obj["system"]==case["system"] and obj["historical_evidence"]["state"]==case["state"]
        mounting=obj["geometry"]["mounting"]
        assert mounting["host"]==row["host"] and mounting["kind"]==row["kind"]
        assert mounting["basis"]==row["basis"]
        assert abs(sum(value*value for value in mounting["normal"])-1)<1e-10
        assert all(quantity["basis"]=="inferred" for quantity in obj["attributes"].values())
        assert api.service_geometry(row["tag"],source)["parts"]
        for lang in ("en","de"):
            assert any("=IFCLIGHTFIXTURE(" in line for line in api.step_excerpt(row["tag"],source,language=lang)["lines"])


def test_physical_lights_volumes_and_actual_mount_points_match_independent_hulls():
    """📐 Compare material volumes with SciPy hulls and prove both sides touch each declared host point."""
    from scipy.spatial import ConvexHull
    from scipy.optimize import linprog
    source=api.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text())
    def hull(part):
        return ConvexHull(api.part_vertices(part))
    def contains(part,point):
        return all(sum(eq[k]*point[k] for k in range(3))+eq[3]<2e-7 for eq in hull(part).equations)
    for row in case["fixtures"]:
        obj=api._find(source,row["tag"]);geometry=api.service_geometry(row["tag"],source)
        total=sum(hull(part).volume for part in geometry["parts"])
        assert abs(total-api.quantities(row["tag"],source)["volume"])<1e-7,row["tag"]
        point=obj["geometry"]["mounting"]["point"];host=api._find(source,row["host"])
        if host["class"]=="IfcColumn":host_geometry=api.column_geometry(source,host)
        elif host["class"]=="IfcWall":host_geometry=api._wall_geometry(source,host)
        elif host["class"]=="IfcSlab":host_geometry=api._slab_geometry(source,host)
        else:host_geometry=api.service_geometry(host["tag"],source)
        assert any(contains(part,point) for part in host_geometry["parts"]),(row["tag"],"host attachment")
        assert any(contains(part,point) for part in geometry["parts"]),(row["tag"],"fixture attachment")
        for part in geometry["parts"]:
            ph=hull(part);pv=ph.points
            for solid in host_geometry["parts"]:
                vertices=api.part_vertices(solid)
                if any(min(max(v[k] for v in vertices),max(v[k] for v in pv))-max(min(v[k] for v in vertices),min(v[k] for v in pv))<1e-8 for k in range(3)):continue
                equations=list(ph.equations)+list(hull(solid).equations)
                fit=linprog([0,0,0,-1],A_ub=[list(eq[:3])+[1] for eq in equations],b_ub=[-eq[3] for eq in equations],bounds=[(None,None)]*3+[(0,None)],method="highs")
                assert not fit.success or fit.x[3]<1e-8,(row["tag"],"host material penetration",fit.x[3])
      