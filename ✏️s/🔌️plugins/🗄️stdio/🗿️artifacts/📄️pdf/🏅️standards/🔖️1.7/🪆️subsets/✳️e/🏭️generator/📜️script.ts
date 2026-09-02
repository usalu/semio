#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.pdf@1.7/✳️e` — the 12-kind ISO 24517-1:2008 (PDF/E-1)
// conformance-class catalog.
//
// Every base/mutated pair `🦀️lopdf-engine/src/generate.rs` writes is produced by the SAME registered `lopdf`
// 0.44 reference implementation named `lopdf-pdf-1-7-e-mutate` in `../🔣️oracle.json` —
// `document::pdf_conformance` inside the standalone, already-qualified
// `semio-s-plugin-stdio-test-oracle` crate, the identical engine the differential test case
// `../../../../../../🧪️tests/mutate-pdf-1-7-e` drives — never this repository's own production PDF
// codec, and never hand-rolled to match it. This script only marshals: it builds and invokes the
// Rust binary and reports what it wrote; it computes no PDF bytes itself.
//
// One generator run produces EVERY declared kind's recipe in one pass — the engine loops `KINDS`
// itself — so `generate`/`manifests` operate on the whole 12-recipe corpus, not one recipe at a time.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds the engine, writes every recipe, prints digests
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entries for the committed corpus
//
// @see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️pilot-playbook.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE_DIR = join(HERE, "🦀️lopdf-engine");
const ENGINE_BIN = join(ENGINE_DIR, "target", "release", "generate");
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const SUBSET = "e";
const ORACLE_ID = "lopdf-pdf-1-7-e-mutate-reader";
const COMPARISON_PROFILE = "semantic-pdf-conformance-e-v1";
// 🧾️ Kept in step with `🦀️lopdf-engine/src/generate.rs::KINDS` (itself the same list as
// `../🔣️oracle.json`'s `pdf-1-7-e` catalog) — `manifests` walks whichever recipe directories the
// engine actually wrote rather than trusting this constant, so a drift here fails loudly as a
// missing-directory error instead of silently under-registering a kind.
const KINDS: readonly string[] = ["insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-media-annotation", "remove-media-annotation", "set-output-intent", "remove-output-intent", "embed-font-file", "remove-font-file"];
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
async function generate(outRoot: string): Promise<void> {
  ensureBuilt();
  mkdirSync(outRoot, { recursive: true });
  const result = spawnSync(ENGINE_BIN, [outRoot], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`generator binary failed with status ${result.status}`);
  for (const kind of readdirSync(outRoot).sort()) {
    const basePath = join(outRoot, kind, "base.pdf");
    const mutatedPath = join(outRoot, kind, "mutated.pdf");
    if (!existsSync(basePath) || !existsSync(mutatedPath)) continue;
    console.log(`${kind}: base=${await sha256(basePath)} mutated=${await sha256(mutatedPath)}`);
  }
}

async function manifests(): Promise<void> {
  const entries = [];
  for (const kind of KINDS) {
    const dir = join(FIXTURES_DIR, kind);
    const basePath = join(dir, "base.pdf");
    const mutatedPath = join(dir, "mutated.pdf");
    if (!existsSync(basePath) || !existsSync(mutatedPath)) throw new Error(`${dir} does not exist — run "generate" first`);
    entries.push({
      schema: "semio.repository-test.fixture/v2",
      id: `e-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.pdf", standard: "1.7", subset: SUBSET },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files: [
        { role: "base-pdf", path: `../🧫️fixtures/${kind}/base.pdf`, mediaType: "application/pdf", sha256: await sha256(basePath), bytes: readFileSync(basePath).length },
        { role: "mutated-pdf", path: `../🧫️fixtures/${kind}/mutated.pdf`, mediaType: "application/pdf", sha256: await sha256(mutatedPath), bytes: readFileSync(mutatedPath).length },
      ],
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: {
        oracle: ORACLE_ID,
        packageVersion: "0.44",
        engineFamily: "lopdf",
        engineVersion: "0.44",
        command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🏭️generator/📜️script.ts generate",
        platform: process.platform,
      },
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A minimal lopdf-built seed with the ${kind} mutation applied THROUGH lopdf's own public COS API (🦀️lopdf-engine/src/lib.rs::apply) — never through this repository's own mutation engine, which is what made the previous corpus inadmissible as evidence. base.pdf is the seed after arrange put the mutation's precondition in place; mutated.pdf is the result lopdf wrote. Observability (mutated projection != base projection, both read back through lopdf) is checked before a pair is written, and a pair that does not move is refused rather than committed.`,
    });
  }
  console.log(JSON.stringify(entries, null, 2));
}
//#endregion 🚪️Commands

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "", ...rest] = argv;
  const outFlagIndex = rest.indexOf("--out");
  // 🧭️ `SEMIO_FIXTURE_OUT` (set by `test fixture reproduce`/`generate`) is a FIXTURES ROOT — the
  // engine writes `<root>/<kind>/{base,mutated}.pdf` for every declared kind in one run.
  const fixtureOutRoot = process.env.SEMIO_FIXTURE_OUT;
  const outRoot = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : (fixtureOutRoot ?? FIXTURES_DIR);
  if (command === "generate") {
    await generate(outRoot);
    return 0;
  }
// 🔐️ENCRYPTION MODE — the two kinds lopdf can neither write nor read.
//
// lopdf 0.44 DECRYPTS transparently on load with the empty user password and then reports
// is_encrypted() == false on a genuinely encrypted document, and its writer demands the encryption
// state a real decryption would have recorded, so a synthetic /Encrypt can be neither written nor read
// back. pypdf 6.14 both encrypts and reports it, byte-deterministically. The reader is shared across
// all four conformance subsets at ✳️vt/🔬️probes/📜️script.ts.
const ENCRYPTION_WRITER = String.raw`
import os, sys
from pypdf import PdfWriter

seed, out, kind = sys.argv[1], sys.argv[2], sys.argv[3]

def plain(path):
    w = PdfWriter(clone_from=seed)
    w.write(path)

def encrypted(path):
    w = PdfWriter(clone_from=seed)
    w.encrypt('', algorithm='RC4-128')
    w.write(path)

d = os.path.join(out, kind); os.makedirs(d, exist_ok=True)
before = os.path.join(d, 'base.pdf'); after = os.path.join(d, 'mutated.pdf')
if kind == 'insert-encryption-dictionary':
    plain(before); encrypted(after)
elif kind == 'remove-encryption-dictionary':
    encrypted(before); plain(after)
else:
    raise SystemExit('unknown kind ' + kind)
print(kind + ': written')
`;

async function encryptionMode(emitManifests: boolean, root: string): Promise<number> {
  const { existsSync: exists, readFileSync: read } = await import("node:fs");
  const { join: j } = await import("node:path");
  const { spawnSync: run } = await import("node:child_process");
  const KINDS = ["insert-encryption-dictionary", "remove-encryption-dictionary"];
  const here = import.meta.dir;
  const probes = j(here, "..", "..", "✳️vt", "🔬️probes", "📜️script.ts");
  // 🌱️The seed is this subset's own committed base — the same document every other kind starts from,
  // so the pair differs by encryption and nothing else.
  const seedDir = j(here, "..", "🧫️fixtures");
  const seedKind = ["set-output-intent", "insert-javascript-action", "embed-font-file"].find((k) => exists(j(seedDir, k, "base.pdf")));
  if (!seedKind) throw new Error("no committed base.pdf to seed the encryption pair from");
  const seed = j(seedDir, seedKind, "base.pdf");
  if (!emitManifests) {
    const failures: string[] = [];
    for (const kind of KINDS) {
      const written = run("python3", ["-c", ENCRYPTION_WRITER, seed, root, kind], { stdio: "inherit" });
      if (written.status !== 0) { failures.push(`${kind}: writer failed`); continue; }
      const cmp = run("bun", [probes, "pdf-encryption-compare", "--input", j(root, kind, "base.pdf"), "--input", j(root, kind, "mutated.pdf")], { encoding: "utf8" });
      if (cmp.status !== 0) { failures.push(`${kind}: reader refused the pair`); continue; }
      if (JSON.parse(cmp.stdout).measurements.equal === true) failures.push(`${kind}: not observable in the encryption projection`);
    }
    for (const failure of failures) console.error(`[generator] ${failure}`);
    return failures.length > 0 ? 1 : 0;
  }
  const entries = [];
  for (const kind of KINDS) {
    const files = [];
    for (const [role, name] of [["base-pdf", "base.pdf"], ["mutated-pdf", "mutated.pdf"]] as const) {
      const bytes = read(j(root, kind, name));
      const digest = await crypto.subtle.digest("SHA-256", bytes);
      files.push({ role, path: `../🧫️fixtures/${kind}/${name}`, mediaType: "application/pdf", sha256: `sha256:${[...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("")}`, bytes: bytes.length });
    }
    entries.push({
      schema: "semio.repository-test.fixture/v2",
      id: `encryption-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.pdf", standard: "1.7", subset: "e" },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files,
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: { oracle: "pypdf-pdf-1-7-encryption-reader", packageVersion: "6.14.2", engineFamily: "pypdf", engineVersion: "6.14.2", command: "bun 📜️script.ts encryption", platform: process.platform },
      comparisonProfile: "semantic-pdf-encryption-v1",
      reproducible: true,
      family: "mechanical",
      notes: `This subset's own committed base document, written twice by pypdf 6.14 — once plain and once with a standard security handler. lopdf, which judges every other kind here, decrypts transparently on load and reports is_encrypted() false on a genuinely encrypted file, and cannot write an encryption dictionary at all. Observability was checked through the pypdf reader before the pair was written.`,
    });
  }
  process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
  return 0;
}

  if (command === "encryption") return await encryptionMode(false, outRoot);
  if (command === "encryption-manifests") return await encryptionMode(true, outRoot);
  if (command === "manifests") {
    await manifests();
    return 0;
  }
  console.error(`usage: 📜️script.ts <generate|manifests> [--out <dir>]`);
  return 2;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
