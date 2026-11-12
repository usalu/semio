/** 📦️ Box with flat hard faces, or, with a corner radius, six cube-sphere face grids that share their edge points so rounded edges are smooth and the surface is watertight.
 * @see ../🧱️builder/🟦️.ts */
import type { BoxSolid, Operation, Quality } from "../../🧬️schema/🟦️.ts";
import { normalize, type V3 } from "../../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { MeshBuilder, gridSurface, type Geometry } from "../🧱️builder/🟦️.ts";
import { circleSegments } from "../📏️segments/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

type Station = { c: number; t: number };

function stations(half: number, radius: number, steps: number): Station[] {
  if (radius <= 0) return [{ c: -half, t: 0 }, { c: half, t: 0 }];
  const inner = half - radius;
  const out: Station[] = [];
  for (let i = 0; i <= steps; i++) out.push({ c: -inner, t: portable.tan((Math.PI / 4) * (-1 + i / steps)) });
  for (let i = inner < 1e-12 ? 1 : 0; i <= steps; i++) out.push({ c: inner, t: portable.tan((Math.PI / 4) * (i / steps)) });
  return out;
}

export function tessellateBox(solid: BoxSolid, quality: Quality, operation: Operation = silentOperation): Geometry {
  const half: V3 = [solid.size[0] / 2, solid.size[1] / 2, solid.size[2] / 2];
  const radius = Math.min(solid.cornerRadius ?? 0, ...half);
  const steps = Math.max(2, Math.ceil(circleSegments(radius, quality) / 4));
  const builder = new MeshBuilder(new Pace(operation));
  for (let k = 0; k < 3; k++) for (const s of [1, -1]) {
    const i = s > 0 ? (k + 1) % 3 : (k + 2) % 3;
    const j = s > 0 ? (k + 2) % 3 : (k + 1) % 3;
    const su = stations(half[i]!, radius, steps), sv = stations(half[j]!, radius, steps);
    gridSurface(builder, sv.length, su.length - 1, false, (row, column) => {
      const u = su[column]!, v = sv[row]!;
      const n: V3 = [0, 0, 0], p: V3 = [0, 0, 0];
      if (radius <= 0) {
        n[k] = s;
        p[i] = u.c;
        p[j] = v.c;
        p[k] = s * half[k]!;
      } else {
        const local = normalize([u.t, v.t, 1]);
        n[i] = local[0];
        n[j] = local[1];
        n[k] = s * local[2];
        p[i] = u.c + radius * n[i];
        p[j] = v.c + radius * n[j];
        p[k] = s * (half[k]! - radius) + radius * n[k];
      }
      return { p, n };
    });
  }
  return builder.build();
}
