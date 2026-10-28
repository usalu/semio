/** 🔬 Shared language-neutral structural cases, verified against official declarations. */
import { expect, test } from "bun:test";
import fixtures from "./schema-cases.json";
import { ifc4x3Catalog, ifcAttributeDeclarations, ifcIsSubtype, validateIfc4x3Snapshot } from "../🧬️schema/📚️catalog/🟦️.ts";
import type { IfcSnapshot } from "../🧬️schema/📸️snapshot/🟦️.ts";
import {IFC4X3_SCHEMA_IDENTIFIER,IFC4X3_GRAPH_SCHEMA,IFC4X3_DOCUMENT_SCHEMA} from "../🧬️schema/📸️snapshot/🟦️.ts";
import details from "../✏️editor/tests.json";

test("source, shared graph and host envelope retain their declared identities",()=>{
 expect(String(IFC4X3_SCHEMA_IDENTIFIER)).toBe(details.identities.source);
 expect(String(IFC4X3_GRAPH_SCHEMA)).toBe(details.identities.graph);
 expect(String(IFC4X3_DOCUMENT_SCHEMA)).toBe(details.identities.document);
});

for (const fixture of fixtures) {
  test(`IFC4X3_ADD2 structural case: ${fixture.name}`, () => {
    const entities = JSON.parse(JSON.stringify(fixture.entities), (key, value) => key === "id" || key === "bits" ? BigInt(value) : value?.kind === "integer" || value?.kind === "reference" ? { ...value, value: BigInt(value.value) } : value);
    const snapshot: IfcSnapshot = { schema: "stdio.ifc", header: { fileDescription: [], fileName: [], fileSchema: [{ kind: "aggregate", value: [{ kind: "string", value: "IFC4X3_ADD2" }] }] }, entities };
    expect(validateIfc4x3Snapshot(snapshot).map(diagnostic => diagnostic.code)).toEqual(fixture.codes);
  });
}

test("official catalog covers every entity and named type", () => {
  expect(Object.keys(ifc4x3Catalog.entities)).toHaveLength(876);
  expect(Object.keys(ifc4x3Catalog.types)).toHaveLength(436);
  expect(ifc4x3Catalog.source.sha256).toBe("f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860");
  expect(ifc4x3Catalog.validation.whereRules).toBe(false);
  expect(ifcIsSubtype("IFCWALL", "IFCROOT")).toBe(true);
  expect(ifcAttributeDeclarations("IFCSIUNIT")?.[0]).toMatchObject({ name: "Dimensions", derived: true, declaredBy: "IfcNamedUnit" });
  expect(ifcAttributeDeclarations("IFCWALL")?.map(attribute => attribute.name)).toEqual(["GlobalId", "OwnerHistory", "Name", "Description", "ObjectType", "ObjectPlacement", "Representation", "Tag", "PredefinedType"]);
});

test("enumerations, selects, derived markers and aggregate uniqueness are checked", () => {
  const snapshot: IfcSnapshot = { schema: "stdio.ifc", header: { fileDescription: [], fileName: [], fileSchema: [{ kind: "aggregate", value: [{ kind: "string", value: "IFC4X3_ADD2" }] }] }, entities: [{ id: 1n, name: "IFCSIUNIT", args: [{ kind: "derived" }, { kind: "enum", value: "LENGTHUNIT" }, { kind: "unset" }, { kind: "enum", value: "METRE" }], complex: [] }] };
  expect(validateIfc4x3Snapshot(snapshot)).toEqual([]);
  snapshot.entities[0]!.args[3] = { kind: "enum", value: "METER" };
  expect(validateIfc4x3Snapshot(snapshot).map(item => item.code)).toEqual(["attribute-type"]);
  snapshot.entities[0]!.args[0] = { kind: "unset" };
  expect(validateIfc4x3Snapshot(snapshot).map(item => item.code)).toEqual(["derived-attribute", "attribute-type"]);
  snapshot.entities = [{ id: 1n, name: "IFCPROPERTYSINGLEVALUE", args: [{ kind: "string", value: "Height" }, { kind: "unset" }, { kind: "typedValue", value: { name: "IFCLENGTHMEASURE", items: [{ kind: "real", value: { bits: 4607182418800017408n } }] } }, { kind: "unset" }], complex: [] }];
  expect(validateIfc4x3Snapshot(snapshot)).toEqual([]);
  snapshot.entities[0]!.args[2] = { kind: "typedValue", value: { name: "NOTATYPE", items: [{ kind: "integer", value: 1n }] } };
  expect(validateIfc4x3Snapshot(snapshot).map(item => item.code)).toEqual(["attribute-type"]);
  snapshot.entities = [{ id: 1n, name: "IFCUNITASSIGNMENT", args: [{ kind: "aggregate", value: [] }], complex: [] }];
  expect(validateIfc4x3Snapshot(snapshot).map(item => item.code)).toEqual(["aggregate-bounds"]);
});
