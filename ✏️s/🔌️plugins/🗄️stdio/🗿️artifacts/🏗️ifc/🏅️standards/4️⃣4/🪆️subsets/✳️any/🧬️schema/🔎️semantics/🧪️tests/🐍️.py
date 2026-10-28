"""🔎️ Independent IfcOpenShell attribute extraction and semantic edit re-read oracle."""
import json
from pathlib import Path
import ifcopenshell

def verify():
    """🧪️ Compare IFC4 descriptors and neutral edit vectors with the independent schema-bound engine."""
    owner = Path(__file__).resolve().parent
    contract = json.loads((owner.parent / "🔣️.json").read_text(encoding="utf-8"))
    fixture = json.loads((owner / "🔣️.json").read_text(encoding="utf-8"))
    source = "ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;" + fixture["data"] + "ENDSEC;END-ISO-10303-21;"
    document = ifcopenshell.file.from_string(source)
    for row in fixture["cases"]:
        entity = document.by_id(int(row["id"]))
        declaration = ifcopenshell.schema_by_name("IFC4").declaration_by_name(entity.is_a()).as_entity()
        ancestors = []
        while declaration is not None:
            ancestors.append(declaration.name().upper())
            declaration = declaration.supertype()
        families = [family for family, bases in contract["families"].items() if any(base in ancestors for base in bases)]
        result = {}
        for field in contract["attributes"]:
            if field["family"] not in families:
                continue
            name = field["id"][0].upper() + field["id"][1:]
            assert entity.attribute_name(field["index"]) == name
            value = getattr(entity, name)
            result[field["id"]] = "#" + str(value.id()) if isinstance(value, ifcopenshell.entity_instance) else value
        assert result == row["fields"], row["id"]
    for row in fixture["edits"]:
        if "error" in row:
            continue
        current = ifcopenshell.file.from_string(source)
        entity = current.by_id(int(row["id"]))
        name = entity.attribute_name(row["index"])
        setattr(entity, name, row["value"])
        reread = ifcopenshell.file.from_string(current.to_string()).by_id(int(row["id"]))
        assert getattr(reread, name) == row["value"]
    return {"cases": len(fixture["cases"]), "edits": sum("error" not in row for row in fixture["edits"])}

if __name__ == "__main__":
    print(json.dumps(verify(), ensure_ascii=False))
