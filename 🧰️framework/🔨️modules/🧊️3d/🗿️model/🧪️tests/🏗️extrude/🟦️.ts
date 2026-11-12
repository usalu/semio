/** 🏗️ Extrude: holes and tapers against analytic volumes; the L-shape against three's ExtrudeGeometry; cap triangulation area against three's ShapeUtils. */
import { ExtrudeGeometry, Path, Shape, ShapeUtils, Vector2 } from "three";
import { expect } from "vitest";
import { signedVolume } from "../🧰️support/🟦️.ts";
import { solidSuite } from "../🧰️support/🧊️solids/🟦️.ts";

solidSuite("🏗️extrude", (c, ours) => {
  const s = c.document.solids[0] as any;
  const outer = s.profile.map(([a, b]: number[]) => new Vector2(a, b));
  const shape = new Shape(outer);
  for (const h of s.holes ?? []) shape.holes.push(new Path(h.map(([a, b]: number[]) => new Vector2(a, b))));
  const holeArea = (s.holes ?? []).reduce((sum: number, h: number[][]) => sum + Math.abs(ShapeUtils.area(h.map(([a, b]) => new Vector2(a, b)))), 0);
  if (!s.taperTo) {
    const geometry = new ExtrudeGeometry(shape, { depth: Math.abs(s.to - s.from), bevelEnabled: false });
    geometry.deleteAttribute("normal");
    geometry.deleteAttribute("uv");
    expect(Math.abs(Math.abs(signedVolume(geometry)) - signedVolume(ours))).toBeLessThan(1e-6);
    expect(Math.abs(ShapeUtils.area(outer)) - holeArea).toBeCloseTo(signedVolume(ours) / Math.abs(s.to - s.from), 6);
  }
});
