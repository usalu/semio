/** 🧵️ Loft through superelliptic sections `|x/rx|^n + |y/ry|^n = 1` with smooth normals from the surface grid and flat fan caps.
 * A section with `rotation` lies in the x, y plane of that rotation (absolute); a section without `rotation` lies perpendicular to the path tangent at its `at` (central difference of the neighbouring sections, one-sided at the ends),
 * its x axis being the node X axis projected perpendicular to the tangent (fallback Z) and y = tangent x x, the frame of {@link revolveFrame} that also orients `lateralScale`.
 * Orientation: the rows advance along the path in section order and every ring runs counter-clockwise about the local tangent, whatever the order of the sections, the bends of the path and the handedness of a section `rotation` (a rotation whose z axis opposes the tangent mirrors only the ring parameterisation), so the triangles are always counter-clockwise seen from outside and the signed volume of a closed loft is positive; an end cap faces away along the end tangent. Caps off give an open tube whose normals still face outward.
 * @see ../🌀️revolved/🟦️.ts */
import type { LoftSolid, Operation, Quality } from "../../🧬️schema/🟦️.ts";
import { TAU, add, cross, dot, length, mul, normalize, qRotate, sub, type V3 } from "../../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { MeshBuilder, gridSurface, type Geometry } from "../🧱️builder/🟦️.ts";
import { revolveFrame } from "../🌀️revolved/🟦️.ts";
import { circleSegments } from "../📏️segments/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

const signedPower = (x: number, e: number): number => Math.sign(x) * portable.pow(Math.abs(x), e);

/** 🧭️ The frames of the loft sections: `tangents[i]` is the path direction at section i (central difference of the neighbours, one-sided at the ends, the overall travel or +Z when the path stands still), `frames[i]` the right-handed section frame (x, y in the section plane, z their cross product) and `turn[i]` the sign (+1 or -1) by which the ring angle must be multiplied so that every ring runs counter-clockwise about its own tangent, whatever the handedness of the section's `rotation` and the direction of the path. */
export function loftFrames(sections: LoftSolid["sections"]): { tangents: V3[]; frames: { x: V3; y: V3; z: V3 }[]; turn: number[] } {
  const travel = sub(sections[sections.length - 1]!.at, sections[0]!.at);
  const tangents: V3[] = sections.map((_, i) => {
    const d = sub(sections[Math.min(sections.length - 1, i + 1)]!.at, sections[Math.max(0, i - 1)]!.at);
    return length(d) > 1e-12 ? normalize(d) : length(travel) > 1e-12 ? normalize(travel) : [0, 0, 1];
  });
  const frames = sections.map((s, i) => {
    if (!s.rotation) {
      const { a, e1, e2 } = revolveFrame(tangents[i]!);
      return { x: e1, y: e2, z: a };
    }
    return { x: qRotate(s.rotation, [1, 0, 0]), y: qRotate(s.rotation, [0, 1, 0]), z: qRotate(s.rotation, [0, 0, 1]) };
  });
  const turn = frames.map((f, i) => (dot(f.z, tangents[i]!) < 0 ? -1 : 1));
  return { tangents, frames, turn };
}

/** 🪤️ The first fold of a loft, or `null`: the pair of consecutive sections `[i, i + 1]` whose rings are not entirely behind the plane of the next section and in front of the plane of the previous one (planes oriented along the path), i.e. the path turns back or bends tighter than the section is wide, so the rings overlap and the skin crosses itself (negative or meaningless volume, a pinched elbow). Sampled on 32 points per ring. */
export function loftFault(sections: LoftSolid["sections"]): [number, number] | null {
  const { frames, turn } = loftFrames(sections);
  const samples = 32;
  const rings = sections.map((s, si) => {
    const f = frames[si]!, e = 2 / (s.exponent ?? 2);
    return Array.from({ length: samples }, (_, j) => {
      const theta = (TAU * j) / samples;
      return add(s.at, add(mul(f.x, s.rx * signedPower(portable.cos(theta), e)), mul(f.y, s.ry * signedPower(portable.sin(theta), e))));
    });
  });
  const normals = frames.map((f, i) => mul(f.z, turn[i]!));
  for (let i = 0; i + 1 < sections.length; i++) {
    const [a, b] = [sections[i]!, sections[i + 1]!];
    const eps = 1e-9 * Math.max(1, length(sub(b.at, a.at)));
    if (rings[i]!.some((p) => dot(sub(p, b.at), normals[i + 1]!) >= -eps) || rings[i + 1]!.some((p) => dot(sub(p, a.at), normals[i]!) <= eps)) return [i, i + 1];
  }
  return null;
}

export function tessellateLoft(solid: LoftSolid, quality: Quality, operation: Operation = silentOperation): Geometry {
  const pace = new Pace(operation);
  const sections = solid.sections;
  const segments = solid.segments ?? Math.max(4, circleSegments(Math.max(...sections.flatMap((s) => [s.rx, s.ry])), quality));
  const { tangents, frames, turn } = loftFrames(sections);
  const rings: V3[][] = sections.map((s, si) => {
    const f = frames[si]!;
    const e = 2 / (s.exponent ?? 2);
    const ring: V3[] = [];
    for (let j = 0; j < segments; j++) {
      pace.tick();
      const theta = (turn[si]! * TAU * j) / segments;
      const x = s.rx * signedPower(portable.cos(theta), e), y = s.ry * signedPower(portable.sin(theta), e);
      ring.push(add(s.at, add(mul(f.x, x), mul(f.y, y))));
    }
    return ring;
  });
  const builder = new MeshBuilder(pace);
  const rowCount = sections.length;
  const normalAt = (i: number, j: number): V3 => {
    const ring = (row: number) => rings[row]!;
    const around = sub(ring(i)[(j + 1) % segments]!, ring(i)[(j + segments - 1) % segments]!);
    const along = sub(ring(Math.min(rowCount - 1, i + 1))[j]!, ring(Math.max(0, i - 1))[j]!);
    const n = cross(around, along);
    return normalize(n, [0, 1, 0]);
  };
  gridSurface(builder, rowCount, segments, true, (i, j) => ({ p: rings[i]![j]!, n: normalAt(i, j) }));
  const cap = (ring: V3[], center: V3, outward: V3, flip: boolean) => {
    const c = builder.vertex(center, outward);
    const ids = ring.map((p) => builder.vertex(p, outward));
    for (let j = 0; j < segments; j++) {
      const a = ids[j]!, b = ids[(j + 1) % segments]!;
      if (flip) builder.triangle(c, b, a);
      else builder.triangle(c, a, b);
    }
  };
  if (solid.capStart) cap(rings[0]!, [...sections[0]!.at], mul(tangents[0]!, -1), true);
  if (solid.capEnd) cap(rings[rowCount - 1]!, [...sections[rowCount - 1]!.at], tangents[rowCount - 1]!, false);
  return builder.build();
}
