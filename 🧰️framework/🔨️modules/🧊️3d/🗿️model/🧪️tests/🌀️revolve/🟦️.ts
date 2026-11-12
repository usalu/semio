/** 🌀️ Revolve: frustum-sum volume, Pappus torus, lateral area against three's LatheGeometry. */
import { LatheGeometry, Vector2 } from "three";
import { expect } from "vitest";
import { surfaceArea } from "../🧰️support/🟦️.ts";
import { solidSuite } from "../🧰️support/🧊️solids/🟦️.ts";

solidSuite("🌀️revolve", (c, ours) => {
  if (!c.expected.lateralArea) return;
  const profile = (c.document.solids[0] as any).profile as number[][];
  const lathe = new LatheGeometry(profile.map(([a, r]) => new Vector2(r, a)), 256);
  lathe.deleteAttribute("normal");
  lathe.deleteAttribute("uv");
  expect(Math.abs(surfaceArea(lathe) - surfaceArea(ours)) / surfaceArea(lathe)).toBeLessThan(0.01);
});
