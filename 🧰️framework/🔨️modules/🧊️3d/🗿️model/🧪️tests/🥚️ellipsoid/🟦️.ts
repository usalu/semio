/** 🥚️ Ellipsoid: volume against three's SphereGeometry scaled by the radii. */
import { SphereGeometry } from "three";
import { expect } from "vitest";
import { signedVolume } from "../🧰️support/🟦️.ts";
import { solidSuite } from "../🧰️support/🧊️solids/🟦️.ts";

solidSuite("🥚️ellipsoid", (c, ours) => {
  const r = (c.document.solids[0] as any).radii as number[];
  const sphere = new SphereGeometry(1, 192, 96).scale(r[0]!, r[1]!, r[2]!);
  expect(Math.abs(Math.abs(signedVolume(sphere)) - signedVolume(ours)) / signedVolume(ours)).toBeLessThan(0.01);
});
