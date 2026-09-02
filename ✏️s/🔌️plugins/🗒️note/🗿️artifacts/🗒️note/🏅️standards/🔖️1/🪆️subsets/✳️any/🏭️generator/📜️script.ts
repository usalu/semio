#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.note.note@1/✳️any`.
//
// Every BEFORE/AFTER byte pair is built by `../🦀️note-oracle-codec` — a standalone Rust binary that
// links `dxf` 0.6, `quick-xml` 0.42 and `lopdf` 0.44 DIRECTLY (the SAME crates already registered as
// this subset's oracle in `../🔣️oracle.json`), never note's own (currently non-building)
// production serializers. This file only marshals: it shells out to `cargo run`, computes digests
// over what the crate wrote, and emits/merges the `fixtureManifests` index — exactly the split
// `…✳️mesh/🏭️generator/📜️script.ts` and `…✳️cad/🏭️generator/📜️script.ts` already use.
//
// Generation and execution are SEPARATE operations: a normal test run must never be able to rewrite
// the expectation it is measured against.
//
//   bun 📜️script.ts generate [--only <recipe-id>]...   — writes 🧫️fixtures/<recipe>/{before,after}.<ext>
//   bun 📜️script.ts manifests [--only <recipe-id>]...  — emits the fixtureManifests block to stdout
//
// `SEMIO_FIXTURE_OUT` (set by `test fixture generate|reproduce`) is a FIXTURES ROOT; every recipe
// writes `<root>/<recipe>/<file>`. Absent it, the committed 🧫️fixtures directory is the root.
//
// @see ../🦀️note-oracle-codec/src/recipes.rs — the 16 recipes (one per witnessable mutation)
// @see ../🔬️probes/📜️script.ts — the sibling that reads/compares what this file writes
// @see ../🔣️oracle.json — the fixtureManifests this file's `manifests` output is pasted into
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️note-1-any-fixture-corpus.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
type FixtureFile = { role: string; path: string; mediaType: string; sha256: string; bytes: number };

/** 🎯️ id, mutation kind, and which of dxf/svg/pdf carriers this recipe covers — MIRRORS
 * `🦀️note-oracle-codec/src/recipes.rs::recipes()`. Duplicated here (never imported — TS cannot import
 * Rust) so this file can compute file roles/paths/media-types without re-invoking the crate to ask;
 * kept in the SAME order and content as the Rust source, which is the single source of truth for what
 * bytes actually get written. A recipe added on one side and not the other is caught immediately by
 * `generate` (the crate refuses an unknown `--only` id) or by `fixture reproduce` (a missing file).
 */
const RECIPES: readonly { id: string; mutation: string; carriers: readonly ("dxf" | "svg" | "pdf")[] }[] = [
  { id: "retitles-the-document", mutation: "rename-note", carriers: ["pdf"] },
  { id: "adds-the-diagram-asset", mutation: "create-asset", carriers: ["svg"] },
  { id: "swaps-the-logo-payload", mutation: "replace-asset-payload", carriers: ["svg"] },
  { id: "removes-the-logo-asset", mutation: "delete-asset", carriers: ["svg"] },
  { id: "creates-an-ink-block", mutation: "create-block", carriers: ["dxf", "svg"] },
  { id: "deletes-the-intro-text-block", mutation: "delete-block", carriers: ["svg", "pdf"] },
  { id: "deletes-the-ink-and-text-blocks", mutation: "delete-blocks", carriers: ["dxf", "svg", "pdf"] },
  { id: "duplicates-the-ink-block", mutation: "duplicate-block", carriers: ["dxf", "svg"] },
  { id: "duplicates-the-ink-and-text-blocks", mutation: "duplicate-blocks", carriers: ["dxf", "svg", "pdf"] },
  { id: "drags-the-callout-group-subtree", mutation: "drag-blocks", carriers: ["svg"] },
  { id: "moves-the-math-block", mutation: "move-block", carriers: ["svg"] },
  { id: "resizes-the-image-block", mutation: "resize-block", carriers: ["svg"] },
  { id: "hides-the-intro-text-block", mutation: "change-block-visible", carriers: ["svg"] },
  { id: "edits-the-intro-paragraph", mutation: "edit-block-text", carriers: ["pdf", "svg"] },
  { id: "thickens-the-sketch-stroke", mutation: "change-block-ink-width", carriers: ["svg"] },
  { id: "redraws-the-sketch-polyline", mutation: "edit-block-ink-stroke", carriers: ["dxf", "svg"] },
] as const;

const MEDIA_TYPE: Record<string, string> = { dxf: "image/vnd.dxf", svg: "image/svg+xml", pdf: "application/pdf" };
const CRATE_DIR = join(import.meta.dir, "🦀️note-oracle-codec");
const COMMITTED_FIXTURES = join(import.meta.dir, "..", "🧫️fixtures");
const FIXTURE_PATH_PREFIX = "../🧫️fixtures/";
const ORACLE_BY_CARRIER: Record<string, string> = { dxf: "dxf-crate-note-ink-reader", svg: "quick-xml-note-drawing-reader", pdf: "lopdf-note-text-reader" };
const ENGINE_BY_CARRIER: Record<string, { family: string; version: string; packageVersion: string }> = {
  dxf: { family: "dxf-rs", version: "0.6", packageVersion: "0.6" },
  svg: { family: "quick-xml", version: "0.42", packageVersion: "0.42" },
  pdf: { family: "lopdf", version: "0.44", packageVersion: "0.44" },
};
//#endregion 🧬️Contract

//#region 🏭️Generate
async function sha256(bytes: Uint8Array): Promise<string> {
  const hash = await crypto.subtle.digest("SHA-256", bytes as BufferSource);
  return `sha256:${[...new Uint8Array(hash)].map((b) => b.toString(16).padStart(2, "0")).join("")}`;
}

/** 🦀️ Shells `cargo run` in the crate directory — cargo needs to run FROM there, so every path this
 * file passes it (`--out`) is resolved to an absolute path first. */
function runCodec(args: readonly string[]): { status: number | null; stderr: string } {
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(CRATE_DIR, "target"), "generator");
  const run = spawnSync("cargo", ["run", "--quiet", "--bin", "note-oracle-codec", "--", ...args], { cwd: CRATE_DIR, encoding: "utf8", env: { ...process.env, CARGO_TARGET_DIR: target } });
  return { status: run.status, stderr: (run.stderr ?? "").trim() };
}

async function fixtureManifestFor(recipe: (typeof RECIPES)[number], outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  const files: FixtureFile[] = [];
  for (const carrier of recipe.carriers) {
    for (const label of ["before", "after"] as const) {
      const filename = `${label}.${carrier}`;
      const path = join(dir, filename);
      const bytes = readFileSync(path);
      files.push({ role: `${label}-${carrier}`, path: `${FIXTURE_PATH_PREFIX}${recipe.id}/${filename}`, mediaType: MEDIA_TYPE[carrier]!, sha256: await sha256(bytes), bytes: bytes.length });
    }
  }
  const primaryCarrier = recipe.carriers[0]!;
  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.note.note", standard: "1", subset: "any" },
    mutation: recipe.mutation,
    outcome: "applied",
    units: { length: "unitless", angle: "degree", handedness: "right", up: "y" },
    files,
    generator: {
      oracle: ORACLE_BY_CARRIER[primaryCarrier]!,
      packageVersion: ENGINE_BY_CARRIER[primaryCarrier]!.packageVersion,
      engineFamily: ENGINE_BY_CARRIER[primaryCarrier]!.family,
      engineVersion: ENGINE_BY_CARRIER[primaryCarrier]!.version,
      command: `bun ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      seed: 0,
      platform: `${process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux"}-${process.arch === "arm64" ? "arm64" : "x64"}`,
    },
    provenance: {
      source: "generated",
      license: "MIT",
      attribution: `Built and written by the ${recipe.carriers.map((c) => ({ dxf: "dxf 0.6", svg: "quick-xml 0.42", pdf: "lopdf 0.44" })[c]).join(" + ")} crate(s) — no byte of ${recipe.carriers.join("/").toUpperCase()} here originates in this repository.`,
      security: "scanned-clean",
      privacy: "no-personal-data",
    },
    comparisonProfile: { dxf: "semantic-note-dxf-ink-v1", svg: "semantic-note-svg-drawing-v1", pdf: "semantic-note-pdf-text-v1" }[primaryCarrier],
    reproducible: true,
    family: recipe.carriers.join("+"),
    notes: `Witnesses ${recipe.mutation} via the ${recipe.carriers.join("+")} carrier(s) — see 📓️note-1-any-fixture-corpus.md for why exactly these carriers and not the others declared for this mutation kind.`,
  };
}

async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate"] = argv;
  if (command !== "generate" && command !== "manifests") {
    console.error(`[note generator] unknown command ${JSON.stringify(command)} — expected generate | manifests [--only <recipe-id>]...`);
    return 2;
  }
  const only: string[] = [];
  for (let i = 0; i < argv.length; i += 1) if (argv[i] === "--only" && argv[i + 1]) only.push(argv[i + 1]!);
  const known = new Set(RECIPES.map((r) => r.id));
  const unknown = only.filter((id) => !known.has(id));
  if (unknown.length > 0) {
    console.error(`[note generator] unknown recipe(s) ${unknown.join(", ")} — known: ${RECIPES.map((r) => r.id).join(", ")}`);
    return 2;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? COMMITTED_FIXTURES;
  mkdirSync(outDir, { recursive: true });

  const codecArgs = ["generate", "--out", outDir, ...only.flatMap((id) => ["--only", id])];
  const result = runCodec(codecArgs);
  if (result.status !== 0) {
    console.error(`[note generator] note-oracle-codec exited ${result.status}: ${result.stderr.split("\n").slice(-12).join("\n")}`);
    return 1;
  }

  const selected = only.length === 0 ? RECIPES : RECIPES.filter((r) => only.includes(r.id));
  const manifests: Record<string, unknown>[] = [];
  for (const recipe of selected) {
    manifests.push(await fixtureManifestFor(recipe, outDir));
    console.error(`[note generator] ${recipe.id} (${recipe.carriers.join("+")})`);
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
    return 0;
  }

  // 🧬️A NARROWED run MERGES into the manifest index; it does not replace it — same guard
  // `…✳️mesh/🏭️generator/📜️script.ts` uses, for the same incident (a sequence of `--only` runs
  // silently destroying every other fixture's manifest record while leaving its files on disk).
  const indexPath = join(outDir, "🧫️manifests.json");
  const previous = (() => {
    if (only.length === 0 || !existsSync(indexPath)) return [];
    try {
      return JSON.parse(readFileSync(indexPath, "utf8")) as Record<string, unknown>[];
    } catch {
      return [];
    }
  })();
  const produced = new Set(manifests.map((m) => m.id as string));
  const merged = [...previous.filter((m) => !produced.has(m.id as string)), ...manifests].sort((a, b) => String(a.id).localeCompare(String(b.id)));
  mkdirSync(dirname(indexPath), { recursive: true });
  writeFileSync(indexPath, `${JSON.stringify(merged, null, 2)}\n`);
  console.error(`[note generator] ${manifests.length}/${selected.length} bundle(s) generated into ${outDir}${only.length > 0 ? ` (merged into ${merged.length} total)` : ""}`);
  return 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
export { RECIPES };
//#endregion 🏭️Generate
