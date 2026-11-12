/** 🧊️ Member: a prismatic bar between two bone-local points drawn in its true foreshortened outline in the side, front and plan view.
 *
 * THE REFERENCE is numpy (section frame and corners of both end sections in 3D) and shapely (convex hull of the projection) in 🐍️.py.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, Shape, PlanarViewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { CAMERAS } from "../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { MemberInPaperUnitsError, evaluate, shapeDepth } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";
import { catalog, extentOf } from "../🧊️three-views/🧰️.ts";
requireTool("uv");

const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const VIEWS: readonly PlanarViewpoint[] = ["elevation", "front", "plan"];
const select = { kind: "scale", scale: { denominator: 50 } } as const;

type Section = { kind: "rect"; w: number; h: number } | { kind: "circle"; d: number };
type End = { bone?: string; x: number; y: number; z: number };

const withMember = (bone: string, from: End, to: End, section: Section, extra: readonly Shape[] = []): Asset => {
  const authored = sample.representations["50"] as AuthoredRepresentation;
  const member = { id: "diag", kind: "member", bone, role: "body", z: 5, filled: true, tone: 0.5, from, to, section } as unknown as Shape;
  const geometry = Object.fromEntries(VIEWS.map((view) => [view, [...authored.geometry[view]!, member, ...extra]])) as unknown as AuthoredRepresentation["geometry"];
  return { ...sample, representations: { ...sample.representations, "50": { ...authored, geometry } } };
};

const outline = (state: EvaluatedState, view: PlanarViewpoint) => extentOf(state.shapes[view].filter((s) => s.shape.id === "diag"), true)!;

function oracle(state: EvaluatedState, bone: string, member: { from: End; to: End; section: Section }): Record<PlanarViewpoint, { minX: number; maxX: number; minY: number; maxY: number }> {
  const frames = Object.fromEntries(Object.entries(state.bones).map(([id, b]) => [id, { o: [b.frame.o.x, b.frame.o.y, b.frame.o.z], u: [b.frame.u.x, b.frame.u.y, b.frame.u.z], v: [b.frame.v.x, b.frame.v.y, b.frame.v.z], w: [b.frame.w.x, b.frame.w.y, b.frame.w.z] }]));
  const cameras = Object.fromEntries(VIEWS.map((view) => [view, [[CAMERAS[view].paperX.axis, CAMERAS[view].paperX.sign], [CAMERAS[view].paperUp.axis, CAMERAS[view].paperUp.sign]]]));
  const directory = mkdtempSync(join(tmpdir(), "staffage-member-"));
  try {
    const input = join(directory, "input.json");
    writeFileSync(input, JSON.stringify({ frames, bone, member, cameras }));
    return JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️member-solid/🐍️.py"), "geometry", [input]));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

const close = (actual: { minX: number; maxX: number; minY: number; maxY: number }, expected: { minX: number; maxX: number; minY: number; maxY: number }, label: string): void => {
  for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(actual[edge] - expected[edge]), `${label} ${edge}`).toBeLessThanOrEqual(1e-6);
};

describe("🧊️ a member is a bar in its true foreshortened outline", () => {
  it("draws a diagonal rectangular bar in the three views like numpy and shapely", () => {
    const from = { x: 0, y: 0, z: 0 };
    const to = { x: 0.4, y: 0.6, z: 0.3 };
    const section = { kind: "rect", w: 0.06, h: 0.04 } as const;
    const asset = withMember("torso", from, to, section);
    for (const pose of ["stand", "stride-left-forward", "carry-bag"]) {
      const state = evaluate(asset, { kind: "pose", pose }, select);
      const reference = oracle(state, "torso", { from, to, section });
      for (const view of VIEWS) close(outline(state, view), reference[view], `${pose} ${view}`);
    }
  });

  it("is not a bounding rectangle: the outline of a diagonal has less area than its box", () => {
    const asset = withMember("torso", { x: 0, y: 0, z: 0 }, { x: 0.4, y: 0.6, z: 0.3 }, { kind: "rect", w: 0.02, h: 0.02 });
    const state = evaluate(asset, { kind: "pose", pose: "stand" }, select);
    const ring = state.shapes.elevation.find((s) => s.shape.id === "diag")!.rings[0]!;
    const area = Math.abs(ring.reduce((sum, p, i) => sum + (p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y), 0)) / 2;
    const box = outline(state, "elevation");
    expect(area).toBeLessThan(0.5 * (box.maxX - box.minX) * (box.maxY - box.minY));
  });

  it("follows its bone when the bone is turned and spread", () => {
    const from = { x: 0, y: 0, z: 0 };
    const to = { x: 0.4, y: 0.6, z: 0.3 };
    const section = { kind: "rect", w: 0.06, h: 0.04 } as const;
    const asset = withMember("arm-l", from, to, section);
    const turned = { ...asset, poses: asset.poses.map((p) => (p.id === "stand" ? { ...p, bones: { ...p.bones, "arm-l": { angleDeg: -140, abductionDeg: 20 } } } : p)) };
    const state = evaluate(turned, { kind: "pose", pose: "stand" }, select);
    const reference = oracle(state, "arm-l", { from, to, section });
    for (const view of VIEWS) close(outline(state, view), reference[view], view);
  });

  it("follows both bones of a bar between the arm and the shank", () => {
    const from = { x: 0.3, y: 0, z: 0 };
    const to = { bone: "shank-l", x: 0.2, y: 0.05, z: 0.1 };
    const section = { kind: "circle", d: 0.05 } as const;
    const asset = withMember("arm-l", from, to, section);
    for (const pose of ["stand", "stride-left-forward"]) {
      const state = evaluate(asset, { kind: "pose", pose }, select);
      const reference = oracle(state, "arm-l", { from, to, section });
      for (const view of VIEWS) close(outline(state, view), reference[view], `${pose} ${view}`);
    }
  });

  it("is ranked by the nearest point of its end sections", () => {
    const from = { x: 0, y: 0, z: 0 };
    const to = { x: 0.4, y: 0.6, z: 0.3 };
    const asset = withMember("torso", from, to, { kind: "rect", w: 0.04, h: 0.04 });
    const state = evaluate(asset, { kind: "pose", pose: "stand" }, select);
    const member = state.shapes.plan.find((s) => s.shape.id === "diag")!.shape;
    const top = Math.max(state.bones.torso!.frame.o.y, state.bones.torso!.frame.o.y + 0.6 * state.bones.torso!.frame.v.y + 0.4 * state.bones.torso!.frame.u.y + 0.3 * state.bones.torso!.frame.w.y);
    expect(shapeDepth(state, member, "plan")).toBeGreaterThanOrEqual(top - 1e-9);
    expect(shapeDepth(state, member, "plan")).toBeLessThanOrEqual(top + 0.03);
  });
});

describe("🧊️ a member is never silently dropped from a nominal symbol", () => {
  it("throws a named error when a member is drawn in paper units", () => {
    const member = { id: "diag", kind: "member", bone: "torso", role: "body", z: 5, filled: true, tone: 0.5, from: { x: 0, y: 0, z: 0 }, to: { x: 0.4, y: 0.6, z: 0.3 }, section: { kind: "circle", d: 0.05 } } as unknown as Shape;
    const asset = { ...sample, symbols: [{ id: "bar-symbol", units: "paper-mm", geometry: { elevation: [member], front: [member], plan: [member] } }] } as unknown as Asset;
    expect(() => evaluate(asset, { kind: "pose", pose: "stand" }, { kind: "symbol", symbol: "bar-symbol" })).toThrow(MemberInPaperUnitsError);
  });
});

describe("🧊️ a member is drawn through the depiction in every view and scale", () => {
  const asset = withMember("torso", { x: 0, y: 0, z: 0 }, { x: 0.4, y: 0.6, z: 0.3 }, { kind: "rect", w: 0.06, h: 0.04 });

  it.each(VIEWS)("draws the member in the %s view as a path whose extent is the evaluated outline, in line and solid modes", (view) => {
    const state = evaluate(asset, { kind: "pose", pose: "stand" }, select);
    const expected = outline(state, view);
    for (const mode of ["dark-line", "light-solid"] as const) {
      const scene = depict(state, defaultDepictionOptions(mode, "en", 50, view));
      const items = scene.layers.flatMap((layer) => layer.items).filter((item) => item.kind === "path" && item.id.includes("diag"));
      expect(items.length, `${view} ${mode}`).toBeGreaterThan(0);
      const points = items.flatMap((item) => (item.kind === "path" ? item.commands.flatMap((c) => ("x" in c && "y" in c ? [{ x: c.x, y: c.y }] : [])) : []));
      const k = 1000 / 50;
      const xs = points.map((p) => p.x);
      const ys = points.map((p) => -p.y);
      expect(Math.abs((Math.max(...xs) - Math.min(...xs)) / k - (expected.maxX - expected.minX)), `${view} ${mode} width`).toBeLessThanOrEqual(1e-3);
      expect(Math.abs((Math.max(...ys) - Math.min(...ys)) / k - (expected.maxY - expected.minY)), `${view} ${mode} height`).toBeLessThanOrEqual(1e-3);
    }
  });
});

describe("🧊️ a member on the head of the labrador is drawn in the front and the plan", () => {
  const labrador = ((): Asset => {
    const asset = structuredClone(catalog().find((candidate) => candidate.id === "labrador-dog")!);
    const authored = asset.representations["50"] as AuthoredRepresentation;
    const part = (id: string, x0: number, x1: number, d: number) => ({ id, kind: "member", bone: "head", role: "body", z: 4, filled: true, tone: 0.72, region: "coat-head", from: { x: x0, y: 0.018, z: 0 }, to: { x: x1, y: 0.018, z: 0 }, section: { kind: "circle", d } }) as unknown as Shape;
    for (const view of ["front", "plan"] as const) {
      const shapes = authored.geometry[view]!.filter((shape) => !(shape.bone.startsWith("head") && (shape.id === "head" || shape.id === "nose")));
      (authored.geometry as unknown as Record<string, readonly Shape[]>)[view] = [...shapes, part("head-cranium", 0, 0.13, 0.115), part("head-muzzle", 0.13, 0.25, 0.06)];
    }
    return asset;
  })();

  it.each(["standing-alert", "sniffing-head-down"])("evaluates and depicts both parts with a positive area in %s", (pose) => {
    const state = evaluate(labrador, { kind: "pose", pose }, select);
    for (const view of ["front", "plan"] as const)
      for (const id of ["head-cranium", "head-muzzle"]) {
        const evaluated = state.shapes[view].find((shape) => shape.shape.id === id)!;
        expect(evaluated.rings[0]!.length, `${pose} ${view} ${id}`).toBeGreaterThan(8);
        const box = extentOf([evaluated])!;
        expect(box.maxX - box.minX, `${pose} ${view} ${id} width`).toBeGreaterThan(0.01);
        expect(box.maxY - box.minY, `${pose} ${view} ${id} height`).toBeGreaterThan(0.01);
        const scene = depict(state, defaultDepictionOptions("dark-solid", "en", 50, view));
        expect(scene.layers.flatMap((layer) => layer.items).some((item) => item.kind === "path" && item.id === `shape:${id}`), `${pose} ${view} ${id} painted`).toBe(true);
      }
  });
});
