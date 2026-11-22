/** 🧱️ The posed kernel triangles of the sample figure for the oracle of the flow obstacles: the body solids of the fixture's scale as the axonometric depiction poses them (`poseSoup`), in the staffage world (ahead x, up y, left z, metres, rounded to micrometres), one index range per solid, and their digest.
 * Run as a script (`bun 🧱️mesh.ts <output.json>`) it writes them for `🐍️.py --write --mesh <output.json>`; the test imports {@link figureMesh} to prove that the oracle was derived from the triangles the kernel gives today.
 *
 * @see ./🐍️.py — the oracle
 * @see ../../🔨️modules/🎨️depiction/🧊️axonometric/🧱️mesh/🟦️.ts — `poseSoup` */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import type { AssetDocument } from "../../🧬️schema/🟦️.ts";
import { poseSoup } from "../../🔨️modules/🎨️depiction/🧊️axonometric/🧱️mesh/🟦️.ts";
import { lineLook } from "../../🔨️modules/🧊️volume/🎨️looks/🟦️.ts";
import { Slots, frameDataOf } from "../../🔨️modules/🧊️volume/🧷️frames/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

/** 🧱️ The mesh of the figure a fixture names at its pose and time: vertices, triangles, the triangle range of every solid and the SHA-256 of the rounded vertices and the triangles. */
export function figureMesh(figure: { readonly asset: string; readonly pose: string; readonly scale: number; readonly timeS: number }): { readonly vertices: number[]; readonly triangles: number[]; readonly parts: { readonly id: string; readonly first: number; readonly count: number }[]; readonly digest: string } {
  const asset = (JSON.parse(readFileSync(new URL(`../../🧫️fixtures/${figure.asset}/🔣️.json`, import.meta.url), "utf8")) as AssetDocument).asset;
  const state = evaluate(asset, { kind: "pose", pose: figure.pose, timeS: figure.timeS });
  const frame = frameDataOf({ asset, views: ["light-line"], theme: "light", mm: figure.scale / 1000, longestMm: 10, looks: { thermal: undefined, forces: undefined, moments: undefined, fixed: { "light-line": lineLook("light", "light-line") } }, slots: new Slots() }, state, {});
  const soup = poseSoup(asset, frame, new Map(), figure.scale);
  const vertices: number[] = [];
  for (let i = 0; i < soup.vertices.length; i += 3) vertices.push(Math.round(soup.vertices[i + 2]! * 1e6) / 1e6, Math.round(soup.vertices[i + 1]! * 1e6) / 1e6, Math.round(soup.vertices[i]! * 1e6) / 1e6);
  const triangles: number[] = [];
  for (let i = 0; i < soup.indices.length; i += 3) triangles.push(soup.indices[i]!, soup.indices[i + 2]!, soup.indices[i + 1]!);
  const parts = soup.parts.map((part) => ({ id: part.id, first: part.first, count: part.count }));
  return { vertices, triangles, parts, digest: createHash("sha256").update(JSON.stringify([vertices.map((v) => Math.round(v * 1e6)), triangles])).digest("hex") };
}

if (import.meta.main) {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🌊️flow-obstacles/🔣️.json", import.meta.url), "utf8")) as { figure: { asset: string; pose: string; scale: number; timeS: number } };
  writeFileSync(process.argv[2]!, JSON.stringify(figureMesh(fixture.figure)));
}
