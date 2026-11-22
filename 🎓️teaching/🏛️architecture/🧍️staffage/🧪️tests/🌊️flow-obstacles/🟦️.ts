/** 🪨️ Flow obstacles: uniform wind past one sphere is the analytic potential flow (velocity, no normal velocity on the surface, stagnation points, streamlines against scipy `solve_ivp`), no air crosses any wall or the ground where several obstacles meet, a porous obstacle lets its share through, an eye sees past superellipsoids what trimesh ray casts see,
 * and for a figure with a volume no stream and no parcel runs through the body (analytic obstacles, and the kernel's triangles through trimesh), while the parcels of an asset without a volume are bit for bit what they were before obstacles existed.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ./🐍️.py — the oracle */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🌊️flow-obstacles/🔣️.json" with { type: "json" };
import type { AssetDocument, Environment } from "../../🧬️schema/🟦️.ts";
import type { FlowEye, FlowObstacle, Vec3 } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { ambientAt } from "../../🔨️modules/💨️transport/🌬️ambient/🟦️.ts";
import { flowMarks, flowDetailOf, flowSampler, streamFrom } from "../../🔨️modules/💨️transport/🌊️flow/🟦️.ts";
import { OBSTACLE_CAP, flowVisibility, insideWall, obstacleLevel, obstaclesOf, openingsOf, packWalls } from "../../🔨️modules/💨️transport/🌊️flow/🪨️obstacles/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { figureMesh } from "./🧱️mesh.ts";

const expected = fixture.expected;
const tolerance = fixture.tolerances;
const vec = (p: readonly number[]): Vec3 => ({ x: p[0]!, y: p[1]!, z: p[2]! });
const dot = (a: Vec3, b: Vec3): number => a.x * b.x + a.y * b.y + a.z * b.z;
const ambient = ambientAt(fixture.ambient as unknown as Environment, 0, null, fixture.turbulence);
const EVERYWHERE = { min: { x: -50, y: 0, z: -50 }, max: { x: 50, y: 50, z: 50 } };
const sphere = fixture.sphere.obstacle as unknown as FlowObstacle;
const shapes = fixture.shapes.map((shape) => shape.obstacle as unknown as FlowObstacle);
const probes: Vec3[] = [];
const span = fixture.probes;
const along = (k: number, i: number): number => span.min[k]! + ((span.max[k]! - span.min[k]!) * i) / (span.counts[k]! - 1);
for (let i = 0; i < span.counts[0]!; i++) for (let j = 0; j < span.counts[1]!; j++) for (let k = 0; k < span.counts[2]!; k++) probes.push({ x: along(0, i), y: along(1, j), z: along(2, k) });
const DIRECTIONS: Vec3[] = [];
for (let i = 0; i < 40; i++) {
  const y = 1 - (2 * (i + 0.5)) / 40;
  const r = Math.sqrt(1 - y * y);
  DIRECTIONS.push({ x: r * Math.cos(2.399963 * i), y, z: r * Math.sin(2.399963 * i) });
}
const onSurface = (obstacle: FlowObstacle, direction: Vec3, skin: number): Vec3 => {
  const t = (1 + skin) / obstacleLevel(obstacle, { x: obstacle.centre.x + direction.x, y: obstacle.centre.y + direction.y, z: obstacle.centre.z + direction.z }).s;
  return { x: obstacle.centre.x + direction.x * t, y: obstacle.centre.y + direction.y * t, z: obstacle.centre.z + direction.z * t };
};
const asset = (JSON.parse(readFileSync(new URL(`../../🧫️fixtures/${fixture.figure.asset}/🔣️.json`, import.meta.url), "utf8")) as AssetDocument).asset;
const windy: Environment = { ...asset.thermal.environment, windMS: fixture.figure.windMS, windDirectionDeg: fixture.figure.windDirectionDeg };
const query = { kind: "pose" as const, pose: fixture.figure.pose, timeS: fixture.figure.timeS };

describe("🪨️ flow obstacles", () => {
  it("gives the potential flow past a sphere at every sample point", () => {
    const sample = flowSampler([], [], ambient, [], packWalls([sphere]));
    for (const row of expected.sphere.samples) {
      const u = sample(vec(row.at)).u;
      expect(Math.hypot(u.x - row.u[0]!, u.y - row.u[1]!, u.z - row.u[2]!)).toBeLessThan(tolerance.velocityMS);
    }
  });

  it("lets no air through the surface of the sphere, slides along it as the analytic flow does, 1.5 U at the equator, and stagnates up- and downwind", () => {
    const sample = flowSampler([], [], ambient, [], packWalls([sphere]));
    let fastest = 0;
    for (const row of expected.sphere.surface) {
      const u = sample(vec(row.at)).u;
      expect(Math.abs(dot(u, vec(row.normal)))).toBeLessThan(tolerance.normalMS);
      expect(Math.hypot(u.x - row.u[0]!, u.y - row.u[1]!, u.z - row.u[2]!)).toBeLessThan(tolerance.velocityMS);
      fastest = Math.max(fastest, Math.hypot(u.x, u.y, u.z));
    }
    expect(Math.abs(fastest - expected.sphere.equatorSpeedMS)).toBeLessThan(1e-6);
    for (const at of expected.sphere.stagnation) {
      const u = sample(vec(at)).u;
      expect(Math.hypot(u.x, u.y, u.z)).toBeLessThan(tolerance.normalMS);
    }
    expect(sample(sphere.centre)).toEqual({ u: { x: 0, y: 0, z: 0 }, inducedMS: 0, excessK: 0, vapourExcessKgPerKg: 0, condensateKgPerKg: 0 });
  });

  it("traces the streams around the sphere like solve_ivp of the analytic field and never enters it", () => {
    const walls = packWalls([sphere]);
    const sample = flowSampler([], [], ambient, [], walls);
    for (const row of expected.sphere.streams) {
      const stream = streamFrom(sample, { at: vec(row.seed), role: "ambient", emitter: null }, fixture.sphere.stepM, fixture.sphere.streamPoints / 3, EVERYWHERE, [], walls)!;
      expect(stream.points.length).toBe(fixture.sphere.streamPoints + 1);
      for (const p of stream.points) expect(obstacleLevel(sphere, p).s).toBeGreaterThanOrEqual(1);
      for (const i of [49, 99, 149, 199, 249, 299]) {
        const p = stream.points[i + 1]!;
        expect(Math.hypot(p.x - row.points[i]![0]!, p.y - row.points[i]![1]!, p.z - row.points[i]![2]!)).toBeLessThan(tolerance.streamM);
        expect(Math.abs(stream.travelS[i + 1]! / row.travelS[i]! - 1)).toBeLessThan(tolerance.travelRelative);
      }
    }
  });

  it("lets no air through any wall or the ground where several obstacles and their images meet", () => {
    const sample = flowSampler([], [], ambient, [], packWalls(shapes));
    let checked = 0;
    for (const obstacle of shapes)
      for (const direction of DIRECTIONS) {
        const at = onSurface(obstacle, direction, 1e-9);
        if (at.y <= 0 || shapes.some((other) => other !== obstacle && obstacleLevel(other, at).s < 1)) continue;
        const u = sample(at).u;
        expect(Math.abs(dot(u, obstacleLevel(obstacle, at).normal))).toBeLessThan(1e-6);
        checked++;
      }
    expect(checked).toBeGreaterThan(60);
    for (const x of [-2, -0.7, 0.05, 0.6, 1.7]) for (const z of [-0.9, 0.3, 1.2]) expect(Math.abs(sample({ x, y: 0, z }).u.y)).toBeLessThan(1e-12);
  });

  it("lets its share of the air through a porous obstacle and keeps the normal flow continuous across its surface", () => {
    const crown: FlowObstacle = { ...sphere, id: "crown", kind: "foliage", blockage: 0.6 };
    const sample = flowSampler([], [], ambient, [], packWalls([crown]));
    const inside = sample({ x: 0.1, y: 20.2, z: -0.1 }).u;
    expect(inside.x).toBeCloseTo(0.4 * fixture.ambient.windMS, 12);
    expect(Math.hypot(inside.y, inside.z)).toBeLessThan(1e-12);
    for (const direction of DIRECTIONS) {
      const outer = sample(onSurface(crown, direction, 1e-9)).u;
      const inner = sample(onSurface(crown, direction, -1e-9)).u;
      expect(Math.abs(dot(outer, direction) - dot(inner, direction))).toBeLessThan(1e-7);
      expect(Math.abs(dot(inner, direction) - 0.4 * fixture.ambient.windMS * direction.x)).toBeLessThan(1e-9);
    }
  });

  it.each(fixture.shapes.map((shape, index) => [shape.id, index] as const))("hides behind the %s what a trimesh ray cast hides, for every eye", (id, index) => {
    const rows = (expected.shapes as Record<string, string[]>)[id]!;
    let clear = 0;
    fixture.eyes.forEach((eye, e) => {
      const seen = flowVisibility([shapes[index]!], eye as FlowEye);
      probes.forEach((probe, i) => {
        const oracle = rows[e]![i]!;
        if (oracle === "-") return;
        expect(seen(probe), `${id} eye ${e} probe ${i}`).toBe(oracle === "1" ? 0 : 1);
        clear++;
      });
    });
    expect(clear).toBeGreaterThan(0.9 * probes.length * fixture.eyes.length);
  });

  it("veils what lies behind or inside a porous obstacle and carries the visibility into the marks", () => {
    const crown: FlowObstacle = { ...sphere, id: "crown", kind: "foliage", blockage: 0.6 };
    const seen = flowVisibility([crown], { toward: { x: 0, y: 0, z: -1 } });
    expect(seen({ x: 0, y: 20, z: 3 })).toBeCloseTo(1 - 0.75 * 0.6, 12);
    expect(seen({ x: 0, y: 20, z: 0 })).toBeCloseTo(1 - 0.75 * 0.6, 12);
    expect(seen({ x: 0, y: 20, z: -3 })).toBe(1);
    expect(seen({ x: 0, y: 21, z: 3 })).toBe(1);
  });

  describe("a figure with a volume in wind", () => {
    const state = evaluate(asset, query);
    const obstacles = obstaclesOf(state);
    const field = solveTransport(asset, query, windy, { maxAgeS: fixture.figure.maxAgeS, flow: true, pathlines: false });
    const flow = field.flow!;
    const voxels = expected.figure.voxels;
    const deep = new Set(voxels.deep);
    const deepAt = (p: Vec3): boolean => {
      const cell = [p.x, p.y, p.z].map((v, k) => Math.round((v - voxels.originM[k]!) / voxels.sizeM));
      return cell.every((c, k) => c >= 0 && c < voxels.counts[k]!) && deep.has((cell[0]! * voxels.counts[1]! + cell[1]!) * voxels.counts[2]! + cell[2]!);
    };

    it("derives a small set of walls from the body and the oracle from the kernel's triangles of today", () => {
      expect(figureMesh(fixture.figure).digest).toBe(expected.figure.meshDigest);
      expect(expected.figure.watertightSolids).toBe(expected.figure.solids);
      expect(obstacles.length).toBeGreaterThan(5);
      expect(obstacles.length).toBeLessThanOrEqual(OBSTACLE_CAP);
      expect(flow.obstacles).toEqual(obstacles);
      expect(obstacles.every((o) => o.kind === "solid" && o.blockage === 1)).toBe(true);
      expect(obstaclesOf(evaluate({ ...asset, volumes: {} } as typeof asset, query))).toEqual([]);
    });

    it("holds every vertex of the posed body inside its obstacles' reach: no vertex lies farther outside all obstacles than a twentieth of a radius", () => {
      const mesh = figureMesh(fixture.figure);
      let worst = 0;
      for (let i = 0; i < mesh.vertices.length; i += 3) worst = Math.max(worst, Math.min(...obstacles.map((o) => obstacleLevel(o, { x: mesh.vertices[i]!, y: mesh.vertices[i + 1]!, z: mesh.vertices[i + 2]! }).s)));
      expect(worst).toBeLessThan(1.05);
    });

    it("runs no stream through the body: no point inside a wall, none deep inside the kernel's triangles", () => {
      const walls = packWalls(obstacles, openingsOf(field.emitters, new Map(asset.thermal.emitters.map((e) => [e.id, e.diameterM]))));
      const points = flow.streams.flatMap((stream) => stream.points);
      expect(flow.streams.filter((stream) => stream.role === "ambient").length).toBe(6);
      expect(points.length).toBeGreaterThan(1000);
      expect(points.filter((p) => insideWall(walls, p.x, p.y, p.z)).length).toBe(0);
      expect(points.filter(deepAt).length).toBe(0);
      const beside = flow.streams.filter((stream) => stream.role === "ambient").flatMap((stream) => stream.points).filter((p) => obstacles.some((o) => obstacleLevel(o, p).s < 1.5));
      expect(beside.length).toBeGreaterThan(5);
    });

    it("runs no parcel through the body once it has left its nozzle", () => {
      const walls = packWalls(obstacles);
      const grown = field.parcels.filter((p) => p.ageS >= fixture.figure.parcelAgeS);
      expect(grown.length).toBeGreaterThan(100);
      expect(grown.filter((p) => insideWall(walls, p.at.x, p.at.y, p.at.z)).length).toBe(0);
      expect(grown.filter((p) => deepAt(p.at)).length).toBe(0);
    });

    it("hides behind the body what trimesh ray casts against the kernel's triangles hide, within the fit of the obstacles", () => {
      let same = 0;
      fixture.eyes.forEach((eye, e) => {
        const seen = flowVisibility(obstacles, eye as FlowEye);
        probes.forEach((probe, i) => {
          if ((seen(probe) === 0) === (expected.figure.hidden[e]![i] === "1")) same++;
        });
      });
      expect(same / (probes.length * fixture.eyes.length)).toBeGreaterThan(tolerance.figureOcclusionAgreement);
      const marks = flowMarks(flow, flow.timeS, { ...flowDetailOf(50), eye: { toward: { x: 0, y: 0, z: -1 } } });
      const visible = marks.ribbons.flatMap((ribbon) => ribbon.visible);
      expect(visible.some((v) => v === 0)).toBe(true);
      expect(visible.filter((v) => v === 1).length).toBeGreaterThan(visible.length / 2);
      expect(flowMarks(flow, flow.timeS, flowDetailOf(50)).ribbons.every((ribbon) => ribbon.visible.every((v) => v === 1))).toBe(true);
    });

    it("leaves the parcels of the same figure without its volume bit for bit as they were before obstacles existed", () => {
      const bare = solveTransport({ ...asset, volumes: {} } as typeof asset, query, windy, { maxAgeS: fixture.figure.maxAgeS, pathlines: false });
      expect(createHash("sha256").update(JSON.stringify(bare.parcels)).digest("hex")).toBe(expected.figure.unobstructedParcelsDigest);
      expect(JSON.stringify(bare.parcels)).not.toBe(JSON.stringify(field.parcels));
    });
  });
});
