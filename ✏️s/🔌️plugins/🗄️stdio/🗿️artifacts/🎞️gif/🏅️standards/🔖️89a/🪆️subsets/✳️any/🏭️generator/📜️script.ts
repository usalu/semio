#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.gif@89a/✳️any`.
//
// The bytes this file produces are written entirely by the real `gif` 0.13 encoder
// (`🦀️engine/src/main.rs`) — the SAME crate registered as `gif-89a-any-mutate` in
// `../🔣️oracle.json` — never by this repository's own `encode_gif`. This script only marshals:
// it builds and invokes the Rust binary and reports what it wrote; it computes no GIF bytes itself.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds the engine, writes the fixture, prints its sha256
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entry for the committed fixture
//
// TWO generators live in this one file, each backing a DIFFERENT oracle, sharing nothing but the
// `gif` 0.13 dependency:
//   generate/manifests           — pattern-strip.gif, UNTOUCHED, backs `gif-89a-any-mutate`
//                                   (cross-semio-implementation).
//   build/build-manifests        — the per-kind before/after recipe corpus, backs the NEW
//                                   `gif-89a-any-mutate-reader` (third-party-library), via
//                                   `🦀️engine/src/reader_main.rs` (its own independent codec, never
//                                   sharing code with `main.rs` or `🦀️oracle.rs`).
//
//   bun 📜️script.ts build          [--only <recipe-id>] [--out <dir>]   # writes <out>/<id>/{before,after}.gif
//   bun 📜️script.ts build-manifests [--only <recipe-id>]                 # prints the fixtureManifests block (JSON)
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️gif-las-pdf17-findings.md
// @see ./🦀️engine/src/reader_main.rs — the `build`/`project` codec this file's `build`/`build-manifests` commands shell out to

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { readdirSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE_DIR = join(HERE, "🦀️engine");
const ENGINE_MANIFEST = join(ENGINE_DIR, "Cargo.toml");
const ENGINE_BIN = join(ENGINE_DIR, "target", "release", "generate");
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const RECIPE = "pattern-strip";
const FIXTURE_FILE = "pattern-strip.gif";

const READER_ORACLE_ID = "gif-89a-any-mutate-reader";
const READER_ENGINE_VERSION = "0.13.3";

type ReaderOutcome = "applied" | "no-op";
type ReaderRecipe = Readonly<{ id: string; mutation: string; outcome: ReaderOutcome; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🦀️engine/src/reader_main.rs` verbatim — one entry per
 *  WITNESSABLE `GifMutation` (89a) kind (16 of 21 — see `../🔣️oracle.json`'s `-uncarried`
 *  entries for the other 5, and `reader.rs`'s own header docstring for why). Every kind here
 *  applies `["applied"]` per the real dispatch (`../🧬️schema/🧬️mutations/🦀️.rs:288`,
 *  `MutationOutcome::new(...)` uniform for all 21 kinds, no per-kind rejection branch) EXCEPT
 *  `set-snapshot`, which also reaches a documented `no-op` warn path on an identical replacement
 *  (`../🧬️schema/🧬️mutations/📄set-snapshot/🦀️.rs:19`) — exercised by its own `-no-op` recipe,
 *  same convention as `no-mutation-no-op`. */
const READER_RECIPES: readonly ReaderRecipe[] = [
  { id: "no-mutation-no-op", mutation: "no-mutation", outcome: "no-op", notes: "Identity — before and after bytes are the same document; no-mutation's diff is unconditionally a no-op." },
  { id: "set-snapshot-applied", mutation: "set-snapshot", outcome: "applied", notes: "Whole-document replace: screen size, palette, background index, loop count and frames all change together." },
  { id: "set-snapshot-no-op", mutation: "set-snapshot", outcome: "no-op", notes: "Replacement snapshot is byte-identical to the current one — the dispatch's own documented no-op/warn branch." },
  { id: "set-screen-size-applied", mutation: "set-screen-size", outcome: "applied", notes: "Only the logical screen width/height change." },
  { id: "set-global-color-table-applied", mutation: "set-global-color-table", outcome: "applied", notes: "The global colour table is replaced with a different palette." },
  { id: "set-background-color-index-applied", mutation: "set-background-color-index", outcome: "applied", notes: "Only the background colour index scalar changes — readable via `Decoder::bg_color`, a real public getter, even though the encoder has no setter for it." },
  { id: "set-loop-count-applied", mutation: "set-loop-count", outcome: "applied", notes: "Only the NETSCAPE2.0 loop count changes." },
  { id: "insert-frame-applied", mutation: "insert-frame", outcome: "applied", notes: "A fourth frame is appended." },
  { id: "remove-frame-applied", mutation: "remove-frame", outcome: "applied", notes: "The middle (index 1) frame is removed." },
  { id: "move-frame-applied", mutation: "move-frame", outcome: "applied", notes: "The first frame is moved to the end." },
  { id: "set-frame-geometry-applied", mutation: "set-frame-geometry", outcome: "applied", notes: "Frame 0's left/top offset changes; width/height/pixels untouched." },
  { id: "set-frame-pixels-applied", mutation: "set-frame-pixels", outcome: "applied", notes: "Frame 0's palette-index buffer is replaced, same geometry." },
  { id: "set-frame-interlace-applied", mutation: "set-frame-interlace", outcome: "applied", notes: "Frame 0's interlace flag flips; rows are re-stored in GIF's 4-pass order. The reader recovers the flag via `Decoder::next_frame_info` (before pixel decode) — see reader.rs's own header docstring for why this is readable at all." },
  { id: "set-frame-delay-applied", mutation: "set-frame-delay", outcome: "applied", notes: "Frame 0's delay changes." },
  { id: "set-frame-disposal-applied", mutation: "set-frame-disposal", outcome: "applied", notes: "Frame 0's disposal method changes." },
  { id: "set-frame-transparency-applied", mutation: "set-frame-transparency", outcome: "applied", notes: "Frame 0 gains a transparent index." },
  { id: "set-frame-user-input-applied", mutation: "set-frame-user-input", outcome: "applied", notes: "Frame 0's needs-user-input flag flips." },
];
//#endregion 🧬️Contract

//#region 🔨️Build
/** ⚙️ Builds the reference-crate-backed generator binary if it is missing or stale. */
function ensureBuilt(): void {
  const result = spawnSync("cargo", ["build", "--release", "--manifest-path", join(ENGINE_DIR, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build failed with status ${result.status}`);
}

async function sha256(path: string): Promise<string> {
  const bytes = readFileSync(path);
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion 🔨️Build

//#region 🚪️Commands
async function generate(outDir: string): Promise<void> {
  ensureBuilt();
  mkdirSync(outDir, { recursive: true });
  const outPath = join(outDir, FIXTURE_FILE);
  const result = spawnSync(ENGINE_BIN, [outPath], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`generator binary failed with status ${result.status}`);
  const digest = await sha256(outPath);
  console.log(`${outPath}\n${digest}`);
}

async function manifests(): Promise<void> {
  const outPath = join(FIXTURES_DIR, RECIPE, FIXTURE_FILE);
  if (!existsSync(outPath)) throw new Error(`${outPath} does not exist — run "generate" first`);
  const digest = await sha256(outPath);
  const entry = {
    schema: "semio.repository-test.fixture/v2",
    id: RECIPE,
    class: "third-party-generated",
    family: "mechanical",
    files: [{ path: `../🧫️fixtures/${RECIPE}/${FIXTURE_FILE}`, mediaType: "image/gif", sha256: digest }],
    provenance: { license: "public-domain (synthetic, no third-party content embedded)" },
    generator: {
      oracle: "gif-89a-any-mutate",
      packageVersion: "0.13",
      engineFamily: "gif",
      engineVersion: "0.13",
      command: "bun 🏭️generator/📜️script.ts generate",
      platform: process.platform,
    },
    reproducible: true,
  };
  console.log(JSON.stringify(entry, null, 2));
}

function platformId(): string {
  const os = process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux";
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  return `${os}-${arch}`;
}

function contentDigest(bytes: Buffer): string {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

/** 🦀️ Shells out to the standalone `reader` binary — the ONLY place this file touches it. */
function readerBuild(id: string, outDir: string): void {
  const result = spawnSync("cargo", ["run", "--quiet", "--manifest-path", ENGINE_MANIFEST, "--bin", "reader", "--", "build", id, outDir], { encoding: "utf8" });
  if (result.status !== 0) throw new Error(`reader build ${id} failed (exit ${result.status}): ${result.stderr}`);
}

function readerFileEntry(role: string, dir: string, filename: string, id: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `../🧫️fixtures/${id}/${filename}`, mediaType: "image/gif", sha256: contentDigest(bytes), bytes: bytes.length };
}

/** 📋️ Builds the manifest entry from files ALREADY on disk — never rebuilds, never rewrites what
 *  it just hashed (playbook step 5.4). `build` (below) is the only command that writes bytes. */
function readerManifestEntry(recipe: ReaderRecipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.id);
  const files = [readerFileEntry("expected-before-gif", dir, "before.gif", recipe.id), readerFileEntry("expected-after-gif", dir, "after.gif", recipe.id)];
  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.gif", standard: "89a", subset: "any" },
    mutation: recipe.mutation,
    outcome: recipe.outcome,
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: READER_ORACLE_ID,
      packageVersion: READER_ENGINE_VERSION,
      engineFamily: "gif",
      engineVersion: READER_ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🏭️generator/📜️script.ts build --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (gif)", attribution: "Generated with gif (MIT OR Apache-2.0) via the standalone reader binary in 🦀️engine/src/reader_main.rs", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-gif-89a-reader-v1",
    reproducible: true,
    family: "structural",
    notes: recipe.notes,
  };
}

async function build(outDir: string, only: string | null): Promise<number> {
  const recipes = only === null ? READER_RECIPES : READER_RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[gif reader generator] no recipe matches ${JSON.stringify(only)} — known: ${READER_RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  mkdirSync(outDir, { recursive: true });
  let failed = 0;
  for (const recipe of recipes) {
    try {
      readerBuild(recipe.id, outDir);
      console.error(`[gif reader generator] ${recipe.id} (${recipe.mutation}/${recipe.outcome})`);
    } catch (error) {
      // 🧭️A recipe the codec refuses is REPORTED, never dropped — same rationale as the avi/bcf/mesh generators.
      failed += 1;
      console.error(`[gif reader generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }
  console.error(`[gif reader generator] ${recipes.length - failed}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}

async function buildManifests(only: string | null): Promise<number> {
  const recipes = only === null ? READER_RECIPES : READER_RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[gif reader generator] no recipe matches ${JSON.stringify(only)} — known: ${READER_RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const manifests = recipes.map((recipe) => {
    const dir = join(FIXTURES_DIR, recipe.id);
    if (!existsSync(join(dir, "before.gif")) || !existsSync(join(dir, "after.gif"))) throw new Error(`${dir} is missing before.gif/after.gif — run "build --only ${recipe.id}" first`);
    return readerManifestEntry(recipe, FIXTURES_DIR);
  });
  console.log(JSON.stringify(manifests, null, 2));
  return 0;
}
//#endregion 🚪️Commands

//#region 🚪️Entry
function flagValue(rest: readonly string[], flag: string): string | null {
  const index = rest.indexOf(flag);
  return index === -1 ? null : (rest[index + 1] ?? null);
}

async function main(argv: readonly string[]): Promise<number> {
  const [command = "", ...rest] = argv;
  // 🧭️ `SEMIO_FIXTURE_OUT` (set by `test fixture reproduce`/`generate`) is a FIXTURES ROOT, not a
  // per-recipe directory — every generator in the repository writes `<root>/<recipe>/<file>`.
  const fixtureOutRoot = process.env.SEMIO_FIXTURE_OUT;

  if (command === "generate") {
    const outFlag = flagValue(rest, "--out");
    const outDir = outFlag ?? (fixtureOutRoot !== undefined ? join(fixtureOutRoot, RECIPE) : join(FIXTURES_DIR, RECIPE));
    await generate(outDir);
    return 0;
  }
  if (command === "manifests") {
    await manifests();
    return 0;
  }
  if (command === "build") {
    const outFlag = flagValue(rest, "--out");
    const outDir = outFlag ?? fixtureOutRoot ?? FIXTURES_DIR;
    return await build(outDir, flagValue(rest, "--only"));
  }
  if (command === "build-manifests") {
    return await buildManifests(flagValue(rest, "--only"));
  }
  // 🏷️EXTENSION MODE — the four kinds this subset's high-level reader cannot see.
  //
  // `gif`'s `Decoder` models a decoded ANIMATION: comment (0xFE) and application (0xFF) extension
  // blocks are consumed on the way to frames and never surfaced, which is why those kinds were
  // `-uncarried` against it. Its documented low-level `StreamingDecoder` DOES surface them, via
  // `Decoded::SubBlockFinished` and `last_ext()`.
  //
  // The crate cannot WRITE them — `ExtensionData` has only `Control` and `Repetitions` — so Pillow
  // writes: `comment=` emits the 0xFE block, and `loop=` on a multi-frame save emits the 0xFF
  // NETSCAPE2.0 application block. Writer and reader are two different third-party implementations.
  if (command === "extensions" || command === "extensions-manifests") {
    const WRITER = String.raw`
import os, sys
from PIL import Image

PALETTE = [255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0] + [0] * (256 * 3 - 12)

def frame(data):
    im = Image.new('P', (4, 3))
    im.putpalette(PALETTE)
    im.putdata(data)
    return im

A = [0, 1, 2, 3] * 3
B = [3, 2, 1, 0] * 3
COMMENT = b'semio 89a fixture comment'

def plain(path):
    frame(A).save(path, format='GIF')

def commented(path):
    frame(A).save(path, format='GIF', comment=COMMENT)

def looped(path):
    frame(A).save(path, format='GIF', save_all=True, append_images=[frame(B)], loop=0)

def unlooped(path):
    frame(A).save(path, format='GIF', save_all=True, append_images=[frame(B)])

out, kind = sys.argv[1], sys.argv[2]
d = os.path.join(out, kind); os.makedirs(d, exist_ok=True)
before = os.path.join(d, 'before.gif'); after = os.path.join(d, 'after.gif')
if kind == 'insert-comment':
    plain(before); commented(after)
elif kind == 'remove-comment':
    commented(before); plain(after)
elif kind == 'add-app-extension':
    unlooped(before); looped(after)
elif kind == 'remove-app-extension':
    looped(before); unlooped(after)
else:
    raise SystemExit('unknown kind ' + kind)
print(kind + ': written')
`;
    const KINDS = ["insert-comment", "remove-comment", "add-app-extension", "remove-app-extension"];
    const readerDir = join(HERE, "..", "🔬️probes", "🦀️extension-reader");
    const built = spawnSync("cargo", ["build", "--release", "--offline", "--manifest-path", join(readerDir, "Cargo.toml")], { stdio: "inherit" });
    if (built.status !== 0) throw new Error(`cargo build failed with status ${built.status}`);
    const readerBin = join(readerDir, "target", "release", "reader");
    const root = flagValue(rest, "--out") ?? fixtureOutRoot ?? FIXTURES_DIR;
    if (command === "extensions") {
      const failures: string[] = [];
      for (const kind of KINDS) {
        const written = spawnSync("python3", ["-c", WRITER, root, kind], { stdio: "inherit" });
        if (written.status !== 0) { failures.push(`${kind}: writer failed`); continue; }
        const cmp = spawnSync(readerBin, ["compare", join(root, kind, "before.gif"), join(root, kind, "after.gif")], { encoding: "utf8" });
        if (cmp.status !== 0) { failures.push(`${kind}: reader refused the pair`); continue; }
        if (JSON.parse(cmp.stdout).measurements.equal === true) failures.push(`${kind}: not observable in the extension projection`);
      }
      for (const failure of failures) console.error(`[generator] ${failure}`);
      return failures.length > 0 ? 1 : 0;
    }
    const entries = [];
    for (const kind of KINDS) {
      const files = [];
      for (const [role, name] of [["expected-before-gif", "before.gif"], ["expected-after-gif", "after.gif"]] as const) {
        const path = join(FIXTURES_DIR, kind, name);
        files.push({ role, path: `../🧫️fixtures/${kind}/${name}`, mediaType: "image/gif", sha256: await sha256(path), bytes: readFileSync(path).length });
      }
      entries.push({
        schema: "semio.repository-test.fixture/v2",
        id: `extension-${kind}`,
        class: "third-party-generated",
        target: { artifact: "s.stdio.gif", standard: "89a", subset: "any" },
        mutation: kind,
        outcome: "applied",
        units: { length: "unitless", angle: "degree" },
        files,
        provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
        generator: { oracle: "gif-89a-extension-reader", packageVersion: "11.3.0", engineFamily: "pillow", engineVersion: "11.3.0", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🏭️generator/📜️script.ts extensions", platform: process.platform },
        comparisonProfile: "semantic-gif-89a-extension-v1",
        reproducible: true,
        family: "mechanical",
        notes: `A Pillow-written GIF pair differing only in the extension block this kind touches. gif 0.13's high-level Decoder, this subset's other reader, consumes extension blocks on the way to frames and never surfaces them; its documented low-level StreamingDecoder does. Observability was checked through that reader before the pair was written.`,
      });
    }
    process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
    return 0;
  }
  const ASPECT_OUT = process.env.SEMIO_FIXTURE_OUT ?? join(import.meta.dir, "..", "🧫️fixtures");
  // 🖥️ASPECT MODE — the artifact's last kind, closed by a third-party CLI.
  //
  // `gif` 0.13.3 (`encoder.rs:345`) and 0.14.2 (`encoder.rs:401`) each write a hardcoded `0u8` for the
  // aspect byte and NEITHER has a parse path for it; Pillow surfaces only `background` and `version`.
  // Every LIBRARY was checked and the negative was right — but the inventory behind it was scoped to
  // libraries and never to installed CLIs, and Protocol v2 lists `third-party-cli` as qualifying.
  //
  // giflib does both halves: `gifbuild -d` dumps a text description carrying `pixel aspect byte N`,
  // `gifbuild` writes a GIF back from it (byte-deterministic across runs, checked), and `giftext`
  // reports `Aspect = N`. The only authored step is one line of that text description — fixture
  // authoring, which the goal statement admits, with giflib doing every byte of the encoding.
  if (command === "aspect" || command === "aspect-manifests") {
    const seedDir = join(import.meta.dir, "..", "🧫️fixtures");
    const seedKind = readdirSync(seedDir).find((name) => existsSync(join(seedDir, name, "before.gif")));
    if (!seedKind) throw new Error("no committed before.gif to seed the aspect pair from");
    const seed = join(seedDir, seedKind, "before.gif");
    const kind = "set-pixel-aspect-ratio";
    const dir = join(ASPECT_OUT, kind);
    const probes = join(import.meta.dir, "..", "🔬️probes", "📜️script.ts");
    if (command === "aspect") {
      mkdirSync(dir, { recursive: true });
      const dump = spawnSync("gifbuild", ["-d", seed], { encoding: "utf8" });
      if (dump.status !== 0) throw new Error(`gifbuild -d failed: ${dump.stderr}`);
      const before = dump.stdout;
      if (!before.includes("pixel aspect byte 0")) throw new Error("the seed does not carry a zero aspect byte to change");
      const after = before.replace("pixel aspect byte 0", "pixel aspect byte 49");
      for (const [name, text] of [["before.gif", before], ["after.gif", after]] as const) {
        const built = spawnSync("gifbuild", [], { input: text, maxBuffer: 64 * 1024 * 1024 });
        if (built.status !== 0) throw new Error(`gifbuild failed for ${name}`);
        writeFileSync(join(dir, name), built.stdout);
      }
      const cmp = spawnSync("bun", [probes, "gif-screen-compare", "--input", join(dir, "before.gif"), "--input", join(dir, "after.gif")], { encoding: "utf8" });
      if (cmp.status !== 0) throw new Error(`reader refused the pair: ${cmp.stdout}${cmp.stderr}`);
      const m = JSON.parse(cmp.stdout).measurements;
      if (m.equal === true) { console.error(`[generator] ${kind}: not observable in the screen projection`); return 1; }
      // 🔍️The pair must move the ASPECT and nothing else in the descriptor.
      for (const field of ["screenWidth", "screenHeight", "colorResolution", "bitsPerPixel", "background", "imageCount"]) {
        if (m.expected[field] !== m.actual[field]) { console.error(`[generator] ${kind}: the pair also changed ${field}`); return 1; }
      }
      console.error(`[generator] ${kind}: observable, aspect ${m.expected.aspect} -> ${m.actual.aspect}`);
      return 0;
    }
    const files = [];
    for (const [role, name] of [["expected-before-gif", "before.gif"], ["expected-after-gif", "after.gif"]] as const) {
      const bytes = readFileSync(join(dir, name));
      const digest = await crypto.subtle.digest("SHA-256", bytes);
      files.push({ role, path: `../🧫️fixtures/${kind}/${name}`, mediaType: "image/gif", sha256: `sha256:${[...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("")}`, bytes: bytes.length });
    }
    process.stdout.write(`${JSON.stringify([{
      schema: "semio.repository-test.fixture/v2",
      id: `aspect-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.gif", standard: "89a", subset: "any" },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files,
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: { oracle: "giflib-gif-screen-cli", packageVersion: "6.1", engineFamily: "giflib", engineVersion: "6.1", command: "bun 📜️script.ts aspect", platform: process.platform },
      comparisonProfile: "semantic-gif-screen-v1",
      reproducible: true,
      family: "mechanical",
      notes: "This subset's own committed seed, dumped by gifbuild -d and rebuilt twice by gifbuild — once with the pixel aspect byte at 0 and once at 49. Every other descriptor field is asserted identical at generation time. No library reader reaches this byte: both vendored gif crate versions write a hardcoded zero and neither parses it, and Pillow surfaces only background and version. giftext reports Aspect = N.",
    }], null, 2)}\n`);
    return 0;
  }
  console.error(`usage: 📜️script.ts <generate|manifests|build|build-manifests|extensions|extensions-manifests|aspect|aspect-manifests> [--out <dir>] [--only <recipe-id>]`);
  return 2;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
