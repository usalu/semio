/** 🧬️ Conformance of the fixture asset and the TypeScript twin to `🧬️schema/🔣️.json`, validated by ajv.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧬️schema/🔣️.json — the normative schema
 */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import * as twin from "../../🧬️schema/🟦️.ts";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const schema = read("../../🧬️schema/🔣️.json");
const fixture = read("../../🧫️fixtures/🧬️schema-conformance/🔣️.json");
const document = read("../../🧫️fixtures/👤️sample-figure/🔣️.json");
const walker = document.asset;
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const validator = (name: string) => ajv.compile({ $ref: `${schema.$id}#/$defs/${name}` });

function patch(document: any, operations: { op: string; path: string; value?: unknown }[]): any {
  const root = structuredClone(document);
  for (const { op, path, value } of operations) {
    const segments = path.split("/").slice(1);
    let node = root;
    const at = (n: any, s: string) => (Array.isArray(n) ? (s.startsWith("id=") ? n.findIndex((i: any) => i.id === s.slice(3)) : Number(s)) : s);
    for (const segment of segments.slice(0, -1)) node = node[at(node, segment)];
    const key = at(node, segments[segments.length - 1]!);
    if (op === "remove") Array.isArray(node) ? node.splice(key as number, 1) : delete node[key];
    else if (op === "add" && Array.isArray(node)) node.push(structuredClone(value));
    else node[key] = structuredClone(value);
  }
  return root;
}

describe("🧬️ schema conformance", () => {
  const asset = validator("Asset");

  it("declares draft-07 and the semio.staffage/v2 id", () => {
    expect(schema.$schema).toBe("http://json-schema.org/draft-07/schema#");
    expect(schema.$id).toBe(twin.SCHEMA_VERSION);
  });

  it("validates the sample figure with ajv", () => {
    expect(asset(walker), JSON.stringify(asset.errors?.slice(0, 3))).toBe(true);
  });

  it("validates an asset document, a manifest, the print profiles and, through the root, each of them", () => {
    const root = ajv.compile(schema);
    expect(validator("AssetDocument")(document), JSON.stringify(validator("AssetDocument").errors?.slice(0, 3))).toBe(true);
    expect(root(document)).toBe(true);
    const manifest = read("../../🧫️fixtures/✅️asset-validation/🔣️.json").manifest;
    const check = validator("Manifest");
    expect(check(manifest), JSON.stringify(check.errors?.slice(0, 3))).toBe(true);
    expect(root(manifest)).toBe(true);
    const profiles = read("../../🗂️catalog/🖨️print-profiles/🔣️.json");
    expect(validator("PrintProfileDocument")(profiles), JSON.stringify(validator("PrintProfileDocument").errors?.slice(0, 3))).toBe(true);
    expect(root(profiles)).toBe(true);
  });

  for (const c of fixture.invalid) {
    it(`rejects: ${c.name}`, () => {
      expect(asset(patch(walker, c.patch))).toBe(false);
    });
  }

  it("has a $defs entry for every type of the twin and forbids additional properties on objects", () => {
    for (const name of fixture.defs) expect(schema.$defs[name], name).toBeDefined();
    for (const [name, def] of Object.entries(schema.$defs) as [string, any][]) if (def.type === "object" && def.properties) expect(def.additionalProperties, name).toBe(false);
  });

  it("restates the enums of the twin exactly", () => {
    const pairs: [string, readonly unknown[]][] = [
      ["CategoryId", twin.CATEGORIES], ["ScaleDenominator", twin.SCALE_DENOMINATORS], ["ParameterSource", twin.PARAMETER_SOURCES], ["Viewpoint", twin.VIEWPOINTS], ["ExportFormat", twin.EXPORT_FORMATS], ["Completion", twin.COMPLETION],
      ["JointType", twin.JOINT_TYPES], ["AnchorKind", twin.ANCHOR_KINDS], ["SupportType", twin.SUPPORT_TYPES], ["ShapeRole", twin.SHAPE_ROLES], ["Ease", twin.EASES], ["ThermalModelKind", twin.THERMAL_MODELS],
    ];
    for (const [name, values] of pairs) expect(schema.$defs[name].enum, name).toEqual([...values]);
    expect(schema.$defs.ClipEvent.properties.kind.enum).toEqual([...twin.EVENT_KINDS]);
    expect(schema.$defs.Text.required).toEqual([...twin.LANGUAGES]);
    expect(schema.$defs.AssetDocument.properties.version.const).toBe(twin.SCHEMA_VERSION);
  });

  it("builds Shape from exactly the five geometries", () => {
    const kinds = schema.$defs.Shape.oneOf.map((branch: any) => branch.properties.kind.const);
    expect(kinds).toEqual(["capsule", "ellipse", "rect", "poly", "path"]);
    for (const branch of schema.$defs.Shape.oneOf) for (const common of ["id", "bone", "role", "z", "filled", "tone"]) expect(branch.required).toContain(common);
  });
});
