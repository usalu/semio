/** 🧵️ Loft through superelliptic sections `|x/rx|^n + |y/ry|^n = 1` with smooth normals from the surface grid and flat fan caps.
 * A section with `rotation` lies in the x, y plane of that rotation (absolute); a section without `rotation` lies perpendicular to the path tangent at its `at` (central difference of the neighbouring sections, one-sided at the ends),
 * its x axis being the node X axis projected perpendicular to the tangent (fallback Z) and y = tangent x x, the frame of {@link revolveFrame} that also orients `lateralScale`. Rings run counter-clockwise about the direction of travel.
 * @see ../🌀️revolved/🟦️.ts */
import type { LoftSolid, Operation, Quality } from "../../🧬️schema/🟦️.ts";
import { TAU, add, cross, dot, length, mul, normalize, qRotate, sub, type V3 } from "../../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { MeshBuilder, gridSurface, type Geometry } from "../🧱️builder/🟦️.ts";
import { revolveFrame } from "../🌀️revolved/🟦️.ts";
import { circleSegments } from "../📏️segments/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

const signedPower = (x: number, e: number): number => Math.sign(x) * portable.pow(Math.abs(x), e);

export function tessellateLoft(solid: LoftSolid, quality: Quality, operation: Operation = silentOperation): Geometry {
  const pace = new Pace(operation);
  const sections = solid.sections;
  const segments = Math.max(4, circleSegments(Math.max(...sections.flatMap((s) => [s.rx, s.ry])), quality));
  const travel = sub(sections[sections.length - 1]!.at, sections[0]!.at);
  const tangentAt = (i: number): V3 => {
    const d = sub(sections[Math.min(sections.length - 1, i + 1)]!.at, sections[Math.max(0, i - 1)]!.at);
    return length(d) > 1e-12 ? normalize(d) : length(travel) > 1e-12 ? normalize(travel) : [0, 0, 1];
  };
  const frames = sections.map((s, i) => {
    if (!s.rotation) {
      const { a, e1, e2 } = revolveFrame(tangentAt(i));
      return { x: e1, y: e2, z: a };
    }
    return { x: qRotate(s.rotation, [1, 0, 0]), y: qRotate(s.rotation, [0, 1, 0]), z: qRotate(s.rotation, [0, 0, 1]) };
  });
  const direction = dot(travel, frames[0]!.z) < 0 ? -1 : 1;
  const rings: V3[][] = sections.map((s, si) => {
    const f = frames[si]!;
    const e = 2 / (s.exponent ?? 2);
    const ring: V3[] = [];
    for (let j = 0; j < segments; j++) {
      pace.tick();
      const theta = (direction * TAU * j) / segments;
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
  const endNormal = (i: number, sign: number): V3 => mul(normalize(travel.some((v) => v !== 0) ? travel : frames[i]!.z), sign);
  if (solid.capStart) cap(rings[0]!, [...sections[0]!.at], endNormal(0, -1), true);
  if (solid.capEnd) cap(rings[rowCount - 1]!, [...sections[rowCount - 1]!.at], endNormal(rowCount - 1, 1), false);
  return builder.build();
}
