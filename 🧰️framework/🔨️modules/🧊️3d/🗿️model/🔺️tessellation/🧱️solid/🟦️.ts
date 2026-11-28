/** 🧱️ One solid as a triangle soup in its node frame, and the rest world matrices of the nodes: the part of tessellation that needs no validation, so a bundle that only draws posed solids does not carry the document tessellator or the validator.
 * @see ../🟦️.ts — `tessellate`, the document pass that validates and merges these soups */
import type { Node, Operation, Quality, Slug, Solid } from "../../🧬️schema/🟦️.ts";
import { compose, determinant3, identity4, multiply4, normalMatrix, qAxisAngle, transformNormal, transformPoint, worldMatrices, type M4, type V3 } from "../../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { type Geometry } from "../🧱️builder/🟦️.ts";
import { tessellateBox } from "../📦️box/🟦️.ts";
import { tessellateExtrude } from "../🏗️extrude/🟦️.ts";
import { tessellatePolyhedron } from "../🪨️polyhedron/🟦️.ts";
import { tessellateLoft } from "../🧵️loft/🟦️.ts";
import { tessellateCapsule, tessellateEllipsoid, tessellateRevolve } from "../🌀️revolved/🟦️.ts";

/** 🌍️ World matrices of the nodes at their rest transforms, keyed by node id. */
export function restWorldMatrices(nodes: readonly Node[]): Map<Slug, M4> {
  return worldMatrices(nodes, (n) => compose(n.translation, n.rotation, n.scale));
}

/** 🧊️ Geometry of one solid in its node frame after `transform` and `repeat`, then carried by `extra` (the rest world matrix of the node for a weighted solid), so the repeat step is always a step in the node frame; triangles stay CCW outward. */
export function tessellateSolid(solid: Solid, quality: Quality, extra: M4 = identity4(), operation: Operation = silentOperation): Geometry {
  const pace = new Pace(operation);
  const raw = ((): Geometry => {
    switch (solid.kind) {
      case "box": return tessellateBox(solid, quality, operation);
      case "ellipsoid": return tessellateEllipsoid(solid, quality, operation);
      case "capsule": return tessellateCapsule(solid, quality, operation);
      case "revolve": return tessellateRevolve(solid, quality, operation);
      case "extrude": return tessellateExtrude(solid, quality, operation);
      case "polyhedron": return tessellatePolyhedron(solid, operation);
      case "loft": return tessellateLoft(solid, quality, operation);
    }
  })();
  const t = solid.transform;
  const base = compose(t?.translation ?? [0, 0, 0], t?.rotation ?? [0, 0, 0, 1], t?.scale ?? [1, 1, 1]);
  const repeat = solid.repeat;
  const count = repeat?.count ?? 1;
  const origin: V3 = (repeat?.origin as V3 | undefined) ?? [0, 0, 0];
  const turn = compose(origin, qAxisAngle(repeat?.rotationAxis ?? [0, 1, 0], repeat?.rotationRad ?? 0), [1, 1, 1]);
  const unturn = compose([-origin[0], -origin[1], -origin[2]], [0, 0, 0, 1], [1, 1, 1]);
  const step = multiply4(compose(repeat?.translation ?? [0, 0, 0], [0, 0, 0, 1], [1, 1, 1]), multiply4(turn, unturn));
  const out: Geometry = { positions: [], normals: [], indices: [] };
  let copy = identity4();
  for (let k = 0; k < count; k++) {
    const m = multiply4(extra, multiply4(copy, base));
    const nm = normalMatrix(m);
    const flip = determinant3(m) < 0;
    const offset = out.positions.length / 3;
    for (let i = 0; i < raw.positions.length; i += 3) {
      pace.tick();
      out.positions.push(...transformPoint(m, [raw.positions[i]!, raw.positions[i + 1]!, raw.positions[i + 2]!]));
      out.normals.push(...transformNormal(nm, [raw.normals[i]!, raw.normals[i + 1]!, raw.normals[i + 2]!]));
    }
    for (let i = 0; i < raw.indices.length; i += 3) {
      pace.tick();
      const [a, b, c] = [raw.indices[i]! + offset, raw.indices[i + 1]! + offset, raw.indices[i + 2]! + offset];
      out.indices.push(a, flip ? c : b, flip ? b : c);
    }
    copy = multiply4(step, copy);
  }
  return out;
}
