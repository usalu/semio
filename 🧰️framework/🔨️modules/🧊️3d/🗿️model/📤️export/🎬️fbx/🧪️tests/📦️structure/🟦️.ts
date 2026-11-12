import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { tessellate } from "../../../../🔺️tessellation/🟦️.ts";
import { encodeFbxDetailed } from "../../🟦️.ts";
import { connectionList, objectCounts, outline, readFbx } from "../../📖️reader/🟦️.ts";
import { DOCUMENT_FIXTURES, NO_OPERATION, fixture, sha256 } from "../🧰️support/🟦️.ts";
import type { FbxFixture } from "../🧰️support/🟦️.ts";

/** 📦️ Language-agnostic structure vectors: every implementation must reproduce these digests byte for byte. */
describe("FBX encoder structure vectors", () => {
  for (const name of DOCUMENT_FIXTURES) {
    const { document, cases } = fixture<FbxFixture>(name);
    for (const item of cases)
      it(`${name} / ${item.name}: bytes, outline, object counts, connections and notices match the vector`, () => {
        const first = encodeFbxDetailed(document, item.options, NO_OPERATION);
        const again = encodeFbxDetailed(document, item.options, NO_OPERATION);
        expect(sha256(again.bytes)).toBe(sha256(first.bytes));
        expect(first.bytes.length).toBe(item.expected.byteLength);
        expect(sha256(first.bytes)).toBe(item.expected.sha256);
        const file = readFbx(first.bytes);
        expect(file.version).toBe(7400);
        expect(objectCounts(file)).toEqual(item.expected.objectCounts);
        expect(connectionList(file)).toEqual(item.expected.connections);
        expect(outline(file)).toEqual(item.expected.outline);
        expect(first.notices.map((notice) => `${notice.code}:${notice.subject}`)).toEqual(item.expected.notices);
      });
  }

  it("writes the 7.4 header, 13-byte sentinels, a 16-aligned footer and compresses large arrays with zlib", () => {
    const { document, cases } = fixture<FbxFixture>("rigged-arm");
    const { bytes } = encodeFbxDetailed(document, cases[1].options, NO_OPERATION);
    expect(new TextDecoder().decode(bytes.subarray(0, 18))).toBe("Kaydara FBX Binary");
    expect(new DataView(bytes.buffer, bytes.byteOffset).getUint32(23, true)).toBe(7400);
    const file = readFbx(bytes);
    expect(bytes.subarray(file.footerStart - 13, file.footerStart).every((byte) => byte === 0)).toBe(true);
    expect(bytes.length - file.footerStart).toBeGreaterThanOrEqual(161);
    expect((bytes.length - 140) % 16).toBe(0);
    const tail = bytes.subarray(bytes.length - 140);
    expect(new DataView(tail.buffer, tail.byteOffset).getUint32(0, true)).toBe(7400);
    expect(tail.subarray(4, 124).every((byte) => byte === 0)).toBe(true);
    let compressed = 0;
    const scan = (position: number): void => {
      for (let index = position; index < bytes.length - 20; index++) if (bytes[index] === 0x78 && bytes[index + 1] === 0x9c) compressed++;
    };
    scan(0);
    expect(compressed).toBeGreaterThan(0);
  });

  it("expresses the mechanism as limit properties and the Euler rotation order XYZ", () => {
    const { document, cases } = fixture<FbxFixture>("rotation-continuity");
    const file = readFbx(encodeFbxDetailed(document, cases[0].options, NO_OPERATION).bytes);
    const objects = file.nodes.find((record) => record.name === "Objects")!.children;
    const props = (id: string): Record<string, readonly unknown[]> => {
      const model = objects.find((record) => record.name === "Model" && String(record.properties[1].value).startsWith(`${id}\u0000`))!;
      const entries = model.children.find((record) => record.name === "Properties70")!.children;
      return Object.fromEntries(entries.map((entry) => [entry.properties[0].value as string, entry.properties.slice(4).map((value) => value.value)]));
    };
    expect(props("slider")["TranslationMin"]).toEqual([0, 0, -0.1]);
    expect(props("slider")["TranslationMax"]).toEqual([0, 0, 0.5]);
    expect(props("slider")["TranslationMinX"]).toEqual([1]);
    expect(props("slider")["RotationMin"]).toEqual([0, 0, 0]);
    expect(props("diagonal")["RotationActive"]).toBeUndefined();
    expect(props("flipped")["RotationMin"]![1]).toBeCloseTo(-2 * (180 / Math.PI), 9);
    expect(props("flipped")["RotationMax"]![1]).toBeCloseTo(0.5 * (180 / Math.PI), 9);
    expect(props("flipped")["RotationMin"]![0]).toBe(0);
    expect(props("ball")["RotationMinX"]).toEqual([1]);
    expect(props("ball")["RotationMax"]![0]).toBeCloseTo(0.5 * (180 / Math.PI), 9);
    expect(props("ball")["RotationMin"]![1]).toBeCloseTo(-1 * (180 / Math.PI), 9);
    expect(props("ball")["PreRotation"]![1]).toBeCloseTo(0.4 * (180 / Math.PI), 9);
    expect(props("ball")["Lcl Rotation"]).toEqual([0, 0, 0]);
    expect(props("rigid")["RotationMin"]).toEqual([0, 0, 0]);
    expect(props("spinner")["Lcl Rotation"]![1]).toBeCloseTo(0.2 * (180 / Math.PI), 9);
    expect(props("spinner")["InheritType"]).toEqual([1]);
    expect(props("spinner")["RotationOrder"]).toEqual([0]);
  });

  it("keeps the solid provenance of a mesh as the SolidRanges user property of its Model and says so in a notice", () => {
    const { document, cases } = fixture<FbxFixture>("rigged-arm");
    const result = encodeFbxDetailed(document, cases[0].options, NO_OPERATION);
    const model = readFbx(result.bytes).nodes.find((record) => record.name === "Objects")!.children.find((record) => record.name === "Model" && String(record.properties[1].value).startsWith("body\u0000"))!;
    const entry = model.children.find((record) => record.name === "Properties70")!.children.find((property) => property.properties[0].value === "SolidRanges")!;
    expect(entry.properties.map((value) => value.value)).toEqual(["SolidRanges", "KString", "", "U", "upper-arm:0:4;lower-arm:4:4;upper-arm:8:2;lower-arm:10:2"]);
    expect(result.notices.filter((notice) => notice.code === "solids-as-property").map((notice) => notice.subject)).toEqual(["arm-mesh"]);
    const cube = fixture<FbxFixture>("static-cube");
    const plain = encodeFbxDetailed({ ...cube.document, meshes: cube.document.meshes.map((mesh) => ({ ...mesh, primitives: mesh.primitives.map(({ solids: _, ...rest }) => rest) })) }, cube.cases[0].options, NO_OPERATION);
    expect(plain.notices.some((notice) => notice.code === "solids-as-property")).toBe(false);
    expect(readFbx(plain.bytes).nodes.find((record) => record.name === "Objects")!.children.some((record) => JSON.stringify(record).includes("SolidRanges"))).toBe(false);
  });

  it("encodes a tessellated procedural document and lists the solid ranges of its polygons", async () => {
    const data = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🧬️provenance/🔣️.json", import.meta.url), "utf8"));
    const result = encodeFbxDetailed(await tessellate(data.document, data.quality), { format: "binary" }, NO_OPERATION);
    const objects = readFbx(result.bytes).nodes.find((record) => record.name === "Objects")!.children;
    const ranges = (id: string): string => String(objects.find((record) => record.name === "Model" && (String(record.properties[1].value).startsWith(`${id}\u0000`) && record.properties[2].value !== "LimbNode") || String(record.properties[1].value).startsWith(`${id}-geometry\u0000`))!.children.find((record) => record.name === "Properties70")!.children.find((property) => property.properties[0].value === "SolidRanges")!.properties[4].value);
    expect(ranges("root")).toBe("b-one:0:12;b-many:12:36;cloth-box:48:12");
    expect(ranges("arm")).toBe("arm-box:0:12");
    expect(ranges("skinned")).toBe("skinned-a:0:12;skinned-b:12:12;skinned-cloth:24:12");
  });

  it("unwraps Euler keys continuously and subdivides long arcs", () => {
    const { document, cases } = fixture<FbxFixture>("rotation-continuity");
    const file = readFbx(encodeFbxDetailed(document, cases[1].options, NO_OPERATION).bytes);
    const objects = file.nodes.find((record) => record.name === "Objects")!.children;
    const curves = objects.filter((record) => record.name === "AnimationCurve");
    const connections = connectionList(file);
    expect(connections.some((line) => line.includes("-> Model:spinner[LimbNode]") || line.includes("-> Model:spinner[Null]"))).toBe(true);
    const yCurve = curves.find((curve) => {
      const values = curve.children.find((child) => child.name === "KeyValueFloat")!.properties[0].value as number[];
      return values.length > 5 && Math.max(...values.map(Math.abs)) > 400;
    });
    expect(yCurve).toBeDefined();
    const values = yCurve!.children.find((child) => child.name === "KeyValueFloat")!.properties[0].value as number[];
    for (let index = 1; index < values.length; index++) expect(Math.abs(values[index] - values[index - 1])).toBeLessThanOrEqual(15.0001);
    expect(values[values.length - 1]).toBeCloseTo(8.4 * (180 / Math.PI), 2);
  });
});
