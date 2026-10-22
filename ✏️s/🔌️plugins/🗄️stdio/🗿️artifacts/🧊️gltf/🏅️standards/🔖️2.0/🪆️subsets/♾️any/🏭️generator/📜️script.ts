#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Fixture generator for `s.stdio.gltf@2.0/♾️any`, reader-pattern retrofit.
//
// ONE real glTF document is built by `three` 0.182.0's own `GLTFExporter` (real scene graph:
// materials, a skinned+morphed mesh, a camera, two scenes, an animation clip) — that call does the
// actual byte-level work this generator never re-implements: accessor/bufferView layout, binary
// packing, hierarchy flattening, material serialization. Every recipe's `before`/`after` pair is then
// derived from that ONE base document by a small, GENERIC, mechanically-uniform structural edit —
// `applyOp` below — never a per-recipe hand-authored document. The edits are uniform because the
// mutation surface itself is: every `🧬️schema/🔣️.json`'s own `x-semio.touchedPaths` describes the
// same handful of shapes repeated across 13 entity families (`create-X{position}`, `delete-X{index}`,
// `move-X{index,position}`, `reorder-X{order[]}`, `bind/unbind-X-Y{parent,child,position}`), read
// directly out of those schemas, never guessed.
//
// Usage:
//   bun 📜️script.ts generate --only <fixture-id>   # writes the manifest's exact pair coordinates
//   SEMIO_FIXTURE_OUT stages <id>/<declared-file-basename> for repository fixture reproduction.
//   bun 📜️script.ts list                           # prints every recipe id this generator knows
//
// The artifact-root case's own rows (`🌱️Real-input rows`) are the exception to the ONE synthetic base: they edit the
// case's committed REAL GLB export the same generic way and write only each row's `➡️after.glb`.
//
// @see ../🔬️probes/📜️script.ts — the reader half; this file only WRITES, it never reads back semantics
// @see ../🔣️oracle.json — mutationManifests / testEvidence this generator's output is registered under
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️pilot-playbook.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { leadingEmojiIdentity } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🪪️identity/🧩️grapheme/🟦️.ts";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import { buildBaseDoc } from "./🏗️base-document/🟦️.ts";
import { clone, RECIPES } from "./🧬️recipe-registry/🟦️.ts";
import { readGlb, REAL_INPUT, REAL_ROWS, realRowOutputPath, writeGlb } from "./🌱️real-input-rows/🟦️.ts";
import "./👪️recipes-nodes-and-scenes/🟦️.ts";
import "./🕸️recipes-geometry-and-materials/🟦️.ts";
import "./🧥️recipes-skin-animation-and-resources/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🚀️Entry
/** 🧭️ Resolves exact fixture roles from the current catalog, preserving authored physical names. */
export function gltfFixtureOutputPaths(manifests: readonly { id: string; files: readonly { role: string; path: string }[] }[], id: string, catalogDir: string, contextualOperations: readonly string[], outRoot?: string): { before: string; after: string } {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) throw new Error("Invalid glTF fixture identity.");
  const records = manifests.filter((record) => record.id === id);
  if (records.length !== 1 || records[0]!.files.length !== 2) throw new Error(`Expected one exact glTF fixture pair: ${id}`);
  if (!contextualOperations.length || contextualOperations.some((operation) => !/^[a-z]+$/u.test(operation))) throw new Error("Invalid declared mutation-operation vocabulary.");
  const subsetsRoot = resolve(catalogDir, "../..");
  const select = (role: string): string => {
    const files = records[0]!.files.filter((file) => file.role === role);
    if (files.length !== 1) throw new Error(`Expected one ${role} file for ${id}`);
    const path = files[0]!.path;
    if (!path || path !== path.normalize("NFC") || isAbsolute(path) || /^[a-z]:/iu.test(path) || path.includes("\\") || /[\x00-\x1f]/u.test(path)) throw new Error(`Invalid fixture path: ${path}`);
    const target = resolve(catalogDir, path), owner = relative(subsetsRoot, target).split(/[\\/]/u);
    if (relative(catalogDir, target).replaceAll("\\", "/") !== path || (owner.length !== 5 && owner.length !== 6) || owner.some((part) => !part || part === "." || part === "..") || owner[1] !== "🧪️tests" || owner[2] !== "🧫️fixtures" || !owner.at(-1)!.endsWith(".gltf")) throw new Error(`Fixture path escapes its declared collection: ${path}`);
    if (owner.length === 5 && contextualOperations.includes(leadingEmojiIdentity(owner.at(-2)!).rest)) throw new Error(`Contextual mutation operation requires its physical domain: ${path}`);
    return target;
  };
  const before = select("expected-before-gltf"), after = select("expected-after-gltf");
  if (before === after || dirname(before) !== dirname(after)) throw new Error(`Fixture pair must have distinct sibling files: ${id}`);
  return outRoot === undefined ? { before, after } : { before: join(outRoot, id, basename(before)), after: join(outRoot, id, basename(after)) };
}

async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  if (command === "list") {
    if (rest.includes("--json")) {
      console.log(JSON.stringify([...RECIPES.map((r) => ({ id: `${r.mutationId}-applied`, mutationId: r.mutationId, outcome: r.outcome, family: r.family, notes: r.notes })), ...REAL_ROWS.map((row) => ({ id: row.id, mutationId: row.mutationId, outcome: "applied", family: row.family, notes: row.notes }))], null, 2));
      return 0;
    }
    for (const r of RECIPES) console.log(r.id);
    for (const row of REAL_ROWS) console.log(row.id);
    return 0;
  }
  if (command !== "generate") {
    console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | list`);
    return 1;
  }
  const onlyIndex = rest.indexOf("--only");
  const only = onlyIndex >= 0 ? rest[onlyIndex + 1] : undefined;
  const outRoot = process.env.SEMIO_FIXTURE_OUT;
  const catalogDir = join(import.meta.dir, "..", "🔮️oracles");
  const catalog = JSON.parse(readFileSync(join(catalogDir, "🔣️.json"), "utf8"));
  const selected = only ? RECIPES.filter((r) => `${r.mutationId}-applied` === only) : RECIPES;
  const realSelected = only ? REAL_ROWS.filter((row) => row.id === only) : REAL_ROWS;
  if (selected.length + realSelected.length === 0) {
    console.error(`[generator] no recipe matches --only ${JSON.stringify(only)} — known: ${[...RECIPES.map((r) => `${r.mutationId}-applied`), ...REAL_ROWS.map((row) => row.id)].join(", ")}`);
    return 1;
  }
  const contextualOperations = [...new Set(RECIPES.map((row) => row.mutationId.split("-")[0]!))];
  const destinations = selected.map((recipe) => {
    const id = `${recipe.mutationId}-applied`;
    return gltfFixtureOutputPaths(catalog.testEvidence, id, catalogDir, contextualOperations, outRoot);
  });
  const realDestinations = realSelected.map((row) => realRowOutputPath(catalog.testEvidence, row.id, catalogDir, outRoot));
  let count = 0;
  if (selected.length > 0) {
    const base = await buildBaseDoc();
    for (const [index, r] of selected.entries()) {
      const { before, after } = r.build(base);
      const paths = destinations[index]!;
      mkdirSync(dirname(paths.before), { recursive: true });
      writeFileSync(paths.before, `${JSON.stringify(before, null, 2)}\n`);
      writeFileSync(paths.after, `${JSON.stringify(after, null, 2)}\n`);
      count += 1;
    }
  }
  if (realSelected.length > 0) {
    const input = readGlb(readFileSync(resolve(catalogDir, REAL_INPUT)));
    for (const [index, row] of realSelected.entries()) {
      const after = clone(input.doc);
      row.edit(after);
      mkdirSync(dirname(realDestinations[index]!), { recursive: true });
      writeFileSync(realDestinations[index]!, writeGlb(after, input.bin));
      count += 1;
    }
  }
  console.log(`[generator] wrote ${count} fixture bundle(s) into ${outRoot ?? "declared catalog paths"}`);
  return 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
