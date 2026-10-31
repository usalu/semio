/** ✅️ Semantic validation of assets and manifests against the broken variants of the fixture.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/✅️asset-validation/🔣️.json — the defects
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Asset, Manifest } from "../../🧬️schema/🟦️.ts";
import { validateAsset, validateManifest } from "../../🔨️modules/🗂️catalog/🟦️.ts";

type Operation = { op: "replace" | "remove" | "add"; path: string; value?: unknown };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/✅️asset-validation/🔣️.json", import.meta.url), "utf8"));
const walker = readFileSync(new URL("../../🧫️fixtures/👤️sample-figure/🔣️.json", import.meta.url), "utf8");

function step(node: any, segment: string): string | number {
  if (Array.isArray(node)) {
    if (segment === "-") return node.length;
    if (segment.startsWith("id=")) {
      const index = node.findIndex((item) => item.id === segment.slice(3));
      if (index < 0) throw new Error(`no element ${segment}`);
      return index;
    }
    return Number(segment);
  }
  return segment;
}

function patch<T>(document: T, operations: readonly Operation[]): T {
  const root: any = structuredClone(document);
  for (const { op, path, value } of operations) {
    const segments = path.split("/").slice(1);
    let node = root;
    for (const segment of segments.slice(0, -1)) node = node[step(node, segment)];
    const key = step(node, segments[segments.length - 1]!);
    if (op === "remove") Array.isArray(node) ? node.splice(key as number, 1) : delete node[key];
    else if (op === "add" && Array.isArray(node)) node.splice(key as number, 0, structuredClone(value));
    else node[key] = structuredClone(value);
  }
  return root;
}

const base = (JSON.parse(walker) as { asset: Asset }).asset;

describe("✅️ asset validation", () => {
  it("accepts the sample figure", () => {
    expect(validateAsset(base)).toEqual([]);
  });

  for (const c of fixture.cases) {
    it(`reports ${c.expectCodes.join(" + ")} for: ${c.name}`, () => {
      const issues = validateAsset(patch(base, c.patch));
      const codes = issues.map((issue) => issue.code);
      for (const code of c.expectCodes) expect(codes, JSON.stringify(issues.map((i) => [i.path, i.code]))).toContain(code);
      for (const code of c.expectAbsent ?? []) expect(codes).not.toContain(code);
      for (const issue of issues) {
        expect(issue.path.length).toBeGreaterThan(0);
        expect(issue.message.en.length).toBeGreaterThan(0);
        expect(issue.message.de.length).toBeGreaterThan(0);
      }
    });
  }

  for (const c of fixture.manifestCases) {
    it(`validates the manifest: ${c.name}`, () => {
      const manifest = patch(fixture.manifest as Manifest, c.patch);
      const codes = validateManifest(manifest, [base]).map((issue) => issue.code);
      if (c.expectCodes.length === 0) expect(codes).toEqual([]);
      for (const code of c.expectCodes) expect(codes).toContain(code);
    });
  }
});
