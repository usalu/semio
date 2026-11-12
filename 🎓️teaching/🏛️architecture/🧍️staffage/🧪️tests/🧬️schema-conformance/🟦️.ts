/** 🧬️ Conformance of the fixture asset and the TypeScript twin to `🧬️schema/🔣️.json`, validated by ajv.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧬️schema/🔣️.json — the normative schema
 */
import { existsSync, readdirSync, readFileSync } from "node:fs";
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

  it("accepts every extruded sweep axis and rejects a missing one in a catalog-shaped front shape", () => {
    const solid = (value: unknown) => asset(patch(walker, [{ op: "replace", path: "/representations/50/geometry/front/id=torso/solid", value }]));
    expect(solid({ kind: "extruded", sweep: "w", z0: -0.1, z1: 0.1 })).toBe(true);
    expect(solid({ kind: "extruded", sweep: "v", z0: -0.1, z1: 0.1 })).toBe(true);
    expect(solid({ kind: "extruded", z0: -0.1, z1: 0.1 })).toBe(false);
  });

  it("validates every asset of the catalog and every asset fixture with ajv, so no catalog file can lack a required member", () => {
    const root = new URL("../../", import.meta.url);
    const document = validator("AssetDocument");
    const failures: string[] = [];
    let checked = 0;
    for (const category of readdirSync(new URL("🗂️catalog/", root))) {
      const folder = new URL(`🗂️catalog/${category}/`, root);
      let slugs: string[] = [];
      try { slugs = readdirSync(folder); } catch { continue; }
      for (const slug of slugs) {
        const file = new URL(`${slug}/🔣️.json`, folder);
        if (!existsSync(file) || category === "🌤️environments" || category === "🖨️print-profiles") continue;
        checked++;
        if (!document(JSON.parse(readFileSync(file, "utf8")))) failures.push(`${category}/${slug}: ${JSON.stringify(document.errors?.slice(0, 2))}`);
      }
    }
    for (const name of readdirSync(new URL("🧫️fixtures/", root))) {
      const file = new URL(`🧫️fixtures/${name}/🔣️.json`, root);
      if (!existsSync(file)) continue;
      const json = JSON.parse(readFileSync(file, "utf8"));
      const candidate = json.asset?.rig !== undefined ? json.asset : json.rig !== undefined && json.representations !== undefined ? json : undefined;
      if (candidate === undefined) continue;
      checked++;
      if (!asset(candidate)) failures.push(`fixture ${name}: ${JSON.stringify(asset.errors?.slice(0, 2))}`);
    }
    expect(checked).toBeGreaterThanOrEqual(152);
    expect(failures).toEqual([]);
  });

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

  it("builds Shape from exactly the six geometries (the five planar ones and the member)", () => {
    const kinds = schema.$defs.Shape.oneOf.map((branch: any) => branch.properties.kind.const);
    expect(kinds).toEqual(["capsule", "ellipse", "rect", "poly", "path", "member"]);
    for (const branch of schema.$defs.Shape.oneOf) for (const common of ["id", "bone", "role", "z", "filled", "tone"]) expect(branch.required).toContain(common);
  });
});
