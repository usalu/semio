/** 📦️ Box and rounded box: exact volume of the flat box, rounded-box volume against three's RoundedBoxGeometry, mirrored and rotated placement. */
import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
import { BoxGeometry } from "three";
import { expect } from "vitest";
import { signedVolume } from "../🧰️support/🟦️.ts";
import { solidSuite } from "../🧰️support/🧊️solids/🟦️.ts";

solidSuite("📦️box", (c, ours) => {
  const s = (c.document.solids[0] as any).size as number[];
  if (c.name === "box") expect(Math.abs(signedVolume(new BoxGeometry(s[0], s[1], s[2])) - signedVolume(ours))).toBeLessThan(1e-6);
  if (c.name === "rounded-box") {
    const reference = new RoundedBoxGeometry(s[0]!, s[1]!, s[2]!, 16, (c.document.solids[0] as any).cornerRadius);
    reference.deleteAttribute("normal");
    reference.deleteAttribute("uv");
    expect(Math.abs(Math.abs(signedVolume(reference)) - signedVolume(ours)) / signedVolume(ours)).toBeLessThan(0.01);
  }
});
