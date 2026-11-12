/** 🧵️ Loft: cylinder and frustum volume against three's CylinderGeometry, superellipse area analytic, reversed section order, open tube boundary; sections without rotation follow the path tangent (frames recomputed with three's Vector3 and Quaternion), mixed with rotated sections. */
import { CylinderGeometry, Quaternion, Vector3 } from "three";
import { expect, it } from "vitest";
import { signedVolume } from "../🧰️support/🟦️.ts";
import { solidSuite } from "../🧰️support/🧊️solids/🟦️.ts";

type Section = { at: number[]; rx: number; ry: number; exponent?: number; rotation?: number[] };

const framesOf = (sections: Section[]) => sections.map((s, i) => {
  if (s.rotation) {
    const q = new Quaternion(...(s.rotation as [number, number, number, number]));
    return { x: new Vector3(1, 0, 0).applyQuaternion(q), y: new Vector3(0, 1, 0).applyQuaternion(q) };
  }
  const t = new Vector3(...(sections[Math.min(sections.length - 1, i + 1)]!.at as [number, number, number])).sub(new Vector3(...(sections[Math.max(0, i - 1)]!.at as [number, number, number]))).normalize();
  const x = new Vector3(1, 0, 0).addScaledVector(t, -t.x);
  if (x.length() < 1e-6) x.set(0, 0, 1).addScaledVector(t, -t.z);
  x.normalize();
  return { x, y: new Vector3().crossVectors(t, x) };
});

solidSuite("🧵️loft", (c, ours, resolved) => {
  const sections = (c.document.solids[0] as any).sections as Section[];
  if (c.expected.oracle === "CylinderGeometry") {
    const length = new Vector3(...(sections[sections.length - 1]!.at as [number, number, number])).distanceTo(new Vector3(...(sections[0]!.at as [number, number, number])));
    const cylinder = new CylinderGeometry(sections[sections.length - 1]!.rx, sections[0]!.rx, length, 192, 1);
    cylinder.deleteAttribute("normal");
    cylinder.deleteAttribute("uv");
    expect(Math.abs(Math.abs(signedVolume(cylinder)) - signedVolume(ours)) / signedVolume(ours)).toBeLessThan(0.01);
  }
  if (!c.expected.frames) return;
  const frames = framesOf(sections);
  const positions = resolved.meshes.flatMap((m) => m.primitives).flatMap((p) => Array.from({ length: p.positions.length / 3 }, (_, v) => new Vector3(p.positions[v * 3]!, p.positions[v * 3 + 1]!, p.positions[v * 3 + 2]!)));
  sections.forEach((s, i) => {
    expect(frames[i]!.x.distanceTo(new Vector3(...(c.expected.frames[i].x as [number, number, number])))).toBeLessThan(1e-9);
    expect(frames[i]!.y.distanceTo(new Vector3(...(c.expected.frames[i].y as [number, number, number])))).toBeLessThan(1e-9);
    const at = new Vector3(...(s.at as [number, number, number])), z = new Vector3().crossVectors(frames[i]!.x, frames[i]!.y);
    const exponent = s.exponent ?? 2;
    const onRing = positions.filter((p) => {
      const d = p.clone().sub(at), u = d.dot(frames[i]!.x) / s.rx, v = d.dot(frames[i]!.y) / s.ry;
      return Math.abs(d.dot(z)) < 1e-9 && Math.abs(Math.abs(u) ** exponent + Math.abs(v) ** exponent - 1) < 1e-6;
    });
    expect(onRing.length).toBeGreaterThanOrEqual(24);
    const first = at.clone().addScaledVector(frames[i]!.x, s.rx);
    expect(onRing.some((p) => p.distanceTo(first) < 1e-9)).toBe(true);
  });
});

it("the loft frames of sections without rotation match the capsule's lateral axis (same helper)", async () => {
  const { tessellateSolid } = await import("../../🔺️tessellation/🟦️.ts");
  const quality = { chordM: 0.0005, minSegments: 24, maxSegments: 128 };
  const loft = tessellateSolid({ id: "l", kind: "loft", node: "n", material: "m", sections: [{ at: [0, 0, 0], rx: 0.3, ry: 0.1 }, { at: [0.6, 0.8, 0], rx: 0.3, ry: 0.1 }], capStart: true, capEnd: true } as never, quality);
  const capsule = tessellateSolid({ id: "c", kind: "capsule", node: "n", material: "m", p0: [0, 0, 0], p1: [0.6, 0.8, 0], r0: 0.1, r1: 0.1, lateralScale: 3 } as never, quality);
  const extent = (g: { positions: number[] }, axis: Vector3) => { let m = 0; for (let i = 0; i < g.positions.length; i += 3) m = Math.max(m, Math.abs(new Vector3(g.positions[i], g.positions[i + 1], g.positions[i + 2]).dot(axis))); return m; };
  const lateral = new Vector3(0.8, -0.6, 0);
  expect(extent(loft, lateral)).toBeCloseTo(0.3, 3);
  expect(extent(capsule, lateral)).toBeGreaterThan(0.29);
});
