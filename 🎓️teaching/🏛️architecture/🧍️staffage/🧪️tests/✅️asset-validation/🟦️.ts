/** ✅️ Semantic validation of assets and manifests against the broken variants of the fixture.
 *
 * @see ./🥒️.feature — the scenarios
 * THE REFERENCE of the lateral ambiguity is numpy (the bone chain with its rest placement and the paper map of the flat drawing) in 🐍️.py.
 *
 * @see ../../🧫️fixtures/✅️asset-validation/🔣️.json — the defects
 */
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, Manifest } from "../../🧬️schema/🟦️.ts";
import { plateLean, validateAsset, validateManifest } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { samplePose } from "../../🔨️modules/🎞️animation/🟦️.ts";
import { restFrames, solveChain } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { STAFFAGE_ROOT, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";
requireTool("uv");

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

describe("✅️ a flat elevation shape is ambiguous only where neither the rest placement nor a solid places it", () => {
  type Finding = { bone: string; bands: string[]; thetaMaxDeg: number; errorMm?: number; reachLimited?: number };
  type Case = { name: string; patch: Operation[]; thetaDeg: number; lean: number; findings: Finding[] };
  const vectors = fixture.lateralAmbiguous as { bone: string; extentM: number; reachM: number; beside: Operation[]; cases: Case[] };
  const built = (c: Case): Asset => patch(base, [...vectors.beside, ...c.patch]);
  const leanOf = (asset: Asset): number => {
    const rest = restFrames(asset.rig)[vectors.bone]!.frame;
    return Math.max(...asset.poses.map((pose) => {
      const sample = samplePose(asset, pose);
      return plateLean(rest, solveChain(asset.rig, sample.offsets, sample.root)[vectors.bone]!.frame);
    }));
  };

  for (const c of vectors.cases)
    it(c.name, { timeout: 300_000 }, () => {
      const asset = built(c);
      expect(leanOf(asset)).toBeCloseTo(c.lean, 6);
      const got = validateAsset(asset).filter((issue) => issue.code === "lateral-ambiguous").map((issue) => issue.data as { bone: string; bands: string[]; thetaMaxDeg: number; errorMm: number });
      expect(got.map((data) => ({ bone: data.bone, bands: data.bands.filter((band) => band.endsWith("elevation")).sort(), thetaMaxDeg: data.thetaMaxDeg }))).toEqual(c.findings.map(({ bone, bands, thetaMaxDeg }) => ({ bone, bands: [...bands].sort(), thetaMaxDeg })));
      c.findings.forEach((expected, index) => {
        if (expected.errorMm !== undefined) expect(got[index]!.errorMm).toBeCloseTo(expected.errorMm, 3);
        if (expected.errorMm !== undefined) expect((vectors.extentM * c.lean * 1000) / 50).toBeCloseTo(expected.errorMm, 3);
        if (expected.reachLimited !== undefined) {
          expect((vectors.reachM * c.lean * 1000) / 50).toBeCloseTo(expected.reachLimited, 6);
          expect(got[index]!.errorMm).toBeLessThanOrEqual(expected.reachLimited + 5e-4);
          expect(got[index]!.errorMm).toBeGreaterThan(expected.reachLimited * 0.98);
        }
      });
    });

  it("numpy solves the chain with its rest placement and finds the same tilt and the same lean", { timeout: 300_000 }, () => {
    const directory = mkdtempSync(join(tmpdir(), "staffage-ambiguous-"));
    try {
      const input = join(directory, "cases.json");
      const assets = vectors.cases.map((c) => [c, built(c)] as const);
      writeFileSync(input, JSON.stringify({ cases: Object.fromEntries(assets.map(([c, asset]) => [c.name, { bone: vectors.bone, bones: asset.rig.bones.map(({ id, parent, angleDeg, abductionDeg, twistDeg }) => ({ id, parent, angleDeg, abductionDeg: abductionDeg ?? 0, twistDeg: twistDeg ?? 0 })), poses: asset.poses.map((pose) => pose.bones ?? {}) }])) }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/✅️asset-validation/🐍️.py"), "geometry", [input])) as Record<string, { thetaDeg: number; lean: number }>;
      for (const [c, asset] of assets) {
        expect(reference[c.name]!.lean, c.name).toBeCloseTo(leanOf(asset), 9);
        expect(reference[c.name]!.lean, c.name).toBeCloseTo(c.lean, 6);
        expect(reference[c.name]!.thetaDeg, c.name).toBeCloseTo(c.thetaDeg, 6);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});
