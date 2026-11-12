/** 💊️ Capsule: volume against three's CapsuleGeometry for the straight capsule; tapered, lateral-scaled and cone-tip analytic hulls of two spheres. */
import { CapsuleGeometry } from "three";
import { expect } from "vitest";
import { signedVolume } from "../🧰️support/🟦️.ts";
import { solidSuite } from "../🧰️support/🧊️solids/🟦️.ts";

solidSuite("💊️capsule", (c, ours) => {
  if (c.name !== "capsule") return;
  const reference = new CapsuleGeometry(0.2, 1, 48, 192);
  expect(Math.abs(Math.abs(signedVolume(reference)) - signedVolume(ours)) / signedVolume(ours)).toBeLessThan(0.01);
});
